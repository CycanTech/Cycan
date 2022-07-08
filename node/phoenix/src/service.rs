// Copyright 2020-2022 Cycan.
// This file is part of Cycan.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// Cycan is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cycan is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cycan. If not, see <http://www.gnu.org/licenses/>.

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use fc_consensus::FrontierBlockImport;
use fc_mapping_sync::{MappingSyncWorker, SyncStrategy};
use fc_rpc::EthTask;
use fc_rpc_core::types::{FilterPool, PendingTransactions};
use phoenix_runtime::{self, opaque::Block, RuntimeApi};
use futures::StreamExt;
use sc_cli::SubstrateCli;
use sc_client_api::{BlockchainEvents, ExecutorProvider};
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_finality_grandpa::SharedVoterState;
use sc_service::{error::Error as ServiceError, BasePath, Configuration, TaskManager};
use sp_core::U256;
use std::{
	collections::{BTreeMap, HashMap},
	sync::{Arc, Mutex},
	time::Duration,
};

use crate::cli::Cli;

// Our native executor instance.
native_executor_instance!(
	pub Executor,
	phoenix_runtime::api::dispatch,
	phoenix_runtime::native_version,
	frame_benchmarking::benchmarking::HostFunctions,
);

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub type ConsensusResult = (
	sc_consensus_babe::BabeBlockImport<
		Block,
		FullClient,
		FrontierBlockImport<
			Block,
			sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
			FullClient
		>
	>,
	sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
	sc_consensus_babe::BabeLink<Block>,
);


pub fn frontier_database_dir(config: &Configuration) -> std::path::PathBuf {
	let config_dir = config
		.base_path
		.as_ref()
		.map(|base_path| base_path.config_dir(config.chain_spec.id()))
		.unwrap_or_else(|| {
			BasePath::from_project("", "", &crate::cli::Cli::executable_name())
				.config_dir(config.chain_spec.id())
		});
	config_dir.join("frontier").join("db")
}

pub fn open_frontier_backend(config: &Configuration) -> Result<Arc<fc_db::Backend<Block>>, String> {
	Ok(Arc::new(fc_db::Backend::<Block>::new(
		&fc_db::DatabaseSettings {
			source: fc_db::DatabaseSettingsSrc::RocksDb {
				path: frontier_database_dir(&config),
				cache_size: 0,
			},
		},
	)?))
}

pub fn new_partial(
	config: &Configuration,
	#[allow(unused_variables)] cli: &Cli,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sp_consensus::import_queue::BasicQueue<Block, sp_api::TransactionFor<FullClient, Block>>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			ConsensusResult,
			PendingTransactions,
			Option<FilterPool>,
			Arc<fc_db::Backend<Block>>,
		),
	>,
	ServiceError,
> {
	let inherent_data_providers = sp_inherents::InherentDataProviders::new();

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(
			&config
		)?;
	let client = Arc::new(client);

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
	);

	let pending_transactions: PendingTransactions = Some(Arc::new(Mutex::new(HashMap::new())));

	let filter_pool: Option<FilterPool> = Some(Arc::new(Mutex::new(BTreeMap::new())));

	let frontier_backend = open_frontier_backend(config)?;

	{
		inherent_data_providers
			.register_provider(pallet_dynamic_fee::InherentDataProvider(U256::from(
				cli.run.target_gas_price,
			)))
			.map_err(Into::into)
			.map_err(sp_consensus::Error::InherentData)?;

		let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
			client.clone(),
			&(client.clone() as Arc<_>),
			select_chain.clone(),
		)?;

		let justification_import = grandpa_block_import.clone();

		let frontier_block_import = FrontierBlockImport::new(
			grandpa_block_import.clone(),
			client.clone(),
			frontier_backend.clone(),
		);

		let (block_import, babe_link) = sc_consensus_babe::block_import(
			sc_consensus_babe::Config::get_or_compute(&*client)?,
			frontier_block_import,
			client.clone(),
		)?;

		let import_queue = sc_consensus_babe::import_queue(
			babe_link.clone(),
			block_import.clone(),
			Some(Box::new(justification_import)),
			client.clone(),
			select_chain.clone(),
			inherent_data_providers.clone(),
			&task_manager.spawn_handle(),
			config.prometheus_registry(),
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
		)?;

		Ok(sc_service::PartialComponents {
			client,
			backend,
			task_manager,
			import_queue,
			keystore_container,
			select_chain,
			transaction_pool,
			inherent_data_providers,
			other: (
				(block_import, grandpa_link, babe_link),
				pending_transactions,
				filter_pool,
				frontier_backend,
			),
		})
	}
}

/// Builds a new service for a full client.
pub fn new_full(mut config: Configuration, cli: &Cli) -> Result<TaskManager, ServiceError> {
	let enable_dev_signer = cli.run.enable_dev_signer;

	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		inherent_data_providers,
		other:
			(consensus_result, pending_transactions, filter_pool, frontier_backend),
	} = new_partial(&config, cli)?;

	config
		.network
		.extra_sets
		.push(sc_finality_grandpa::grandpa_peers_set_config());

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: None,
			block_announce_validator_builder: None,
		})?;

	// Channel for the rpc handler to communicate with the authorship task.
	let (command_sink, _commands_stream) = futures::channel::mpsc::channel(1000);

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			backend.clone(),
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
	let is_authority = role.is_authority();
	let subscription_task_executor =
		sc_rpc::SubscriptionTaskExecutor::new(task_manager.spawn_handle());

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let network = network.clone();
		let pending = pending_transactions.clone();
		let filter_pool = filter_pool.clone();
		let frontier_backend = frontier_backend.clone();
		let max_past_logs = cli.run.max_past_logs;

		Box::new(move |deny_unsafe, _| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
				is_authority,
				enable_dev_signer,
				network: network.clone(),
				pending_transactions: pending.clone(),
				filter_pool: filter_pool.clone(),
				backend: frontier_backend.clone(),
				max_past_logs,
				command_sink: Some(command_sink.clone()),
			};
			crate::rpc::create_full(deps, subscription_task_executor.clone())
		})
	};

	task_manager.spawn_essential_handle().spawn(
		"frontier-mapping-sync-worker",
		MappingSyncWorker::new(
			client.import_notification_stream(),
			Duration::new(6, 0),
			client.clone(),
			backend.clone(),
			frontier_backend.clone(),
			SyncStrategy::Normal,
		)
		.for_each(|()| futures::future::ready(())),
	);

	let (_rpc_handlers, telemetry_connection_notifier) = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		rpc_extensions_builder,
		on_demand: None,
		remote_blockchain: None,
		backend,
		network_status_sinks,
		system_rpc_tx,
		config,
	})?;

	// Spawn Frontier EthFilterApi maintenance task.
	if let Some(filter_pool) = filter_pool {
		// Each filter is allowed to stay in the pool for 100 blocks.
		const FILTER_RETAIN_THRESHOLD: u64 = 100;
		task_manager.spawn_essential_handle().spawn(
			"frontier-filter-pool",
			EthTask::filter_pool_task(Arc::clone(&client), filter_pool, FILTER_RETAIN_THRESHOLD),
		);
	}

	// Spawn Frontier pending transactions maintenance task (as essential, otherwise we leak).
	if let Some(pending_transactions) = pending_transactions {
		const TRANSACTION_RETAIN_THRESHOLD: u64 = 5;
		task_manager.spawn_essential_handle().spawn(
			"frontier-pending-transactions",
			EthTask::pending_transaction_task(
				Arc::clone(&client),
				pending_transactions,
				TRANSACTION_RETAIN_THRESHOLD,
			),
		);
	}

	task_manager.spawn_essential_handle().spawn(
		"frontier-schema-cache-task",
		EthTask::ethereum_schema_cache_task(Arc::clone(&client), Arc::clone(&frontier_backend)),
	);

	{
		let (block_import, grandpa_link,babe_link) = consensus_result;

		if role.is_authority() {
			let proposer = sc_basic_authorship::ProposerFactory::new(
				task_manager.spawn_handle(),
				client.clone(),
				transaction_pool.clone(),
				prometheus_registry.as_ref(),
			);

			let can_author_with =
				sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());
			let babe_config = sc_consensus_babe::BabeParams {
				keystore: keystore_container.sync_keystore(),
				client: client.clone(),
				select_chain,
				env: proposer,
				block_import,
				sync_oracle: network.clone(),
				inherent_data_providers: inherent_data_providers.clone(),
				force_authoring,
				backoff_authoring_blocks,
				babe_link,
				can_author_with,
			};

			// the AURA authoring task is considered essential, i.e. if it
			// fails we take down the service with it.
			let babe = sc_consensus_babe::start_babe(babe_config)?;
			task_manager.spawn_essential_handle().spawn_blocking("babe", babe);

			// if the node isn't actively participating in consensus then it doesn't
			// need a keystore, regardless of which protocol we use below.
			let keystore = if role.is_authority() {
				Some(keystore_container.sync_keystore())
			} else {
				None
			};

			let grandpa_config = sc_finality_grandpa::Config {
				// FIXME #1578 make this available through chainspec
				gossip_duration: Duration::from_millis(333),
				justification_period: 512,
				name: Some(name),
				observer_enabled: false,
				keystore,
				is_authority: role.is_authority(),
			};

			if enable_grandpa {
				// start the full GRANDPA voter
				// NOTE: non-authorities could run the GRANDPA observer protocol, but at
				// this point the full voter should provide better guarantees of block
				// and vote data availability than the observer. The observer has not
				// been tested extensively yet and having most nodes in a network run it
				// could lead to finality stalls.
				let grandpa_config = sc_finality_grandpa::GrandpaParams {
					config: grandpa_config,
					link: grandpa_link,
					network,
					telemetry_on_connect: telemetry_connection_notifier.map(|x| x.on_connect_stream()),
					voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
					prometheus_registry,
					shared_voter_state: SharedVoterState::empty(),
				};

				// the GRANDPA voter task is considered infallible, i.e.
				// if it fails we take down the service with it.
				task_manager.spawn_essential_handle().spawn_blocking(
					"grandpa-voter",
					sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
				);
			}
		}
	}

	network_starter.start_network();
	Ok(task_manager)
}


pub fn new_light(_config: Configuration) -> Result<TaskManager, ServiceError> {

	/*let (client, backend, keystore_container, mut task_manager, on_demand) =
		sc_service::new_light_parts::<Block, RuntimeApi, Executor>(
			&config,
		)?;

	let mut telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
		config.transaction_pool.clone(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
		on_demand.clone(),
	));

	let (grandpa_block_import, _) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let import_queue =
		sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _>(ImportQueueParams {
			slot_duration: sc_consensus_aura::slot_duration(&*client)?,
			block_import: grandpa_block_import.clone(),
			justification_import: Some(Box::new(grandpa_block_import)),
			client: client.clone(),
			inherent_data_providers: InherentDataProviders::new(),
			spawner: &task_manager.spawn_essential_handle(),
			registry: config.prometheus_registry(),
			can_author_with: sp_consensus::NeverCanAuthor,
			check_for_equivocation: Default::default(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		})?;

	let light_deps = crate::rpc::LightDeps {
		remote_blockchain: backend.remote_blockchain(),
		fetcher: on_demand.clone(),
		client: client.clone(),
		pool: transaction_pool.clone(),
	};

	let rpc_extensions = crate::rpc::create_light(light_deps);

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: Some(on_demand.clone()),
			block_announce_validator_builder: None,
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		remote_blockchain: Some(backend.remote_blockchain()),
		transaction_pool,
		task_manager: &mut task_manager,
		on_demand: Some(on_demand),
		rpc_extensions_builder: Box::new(sc_service::NoopRpcExtensionBuilder(rpc_extensions)),
		config,
		client,
		keystore: keystore_container.sync_keystore(),
		backend,
		network,
		network_status_sinks,
		system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;

	network_starter.start_network();

	Ok(task_manager)*/
	Err(sc_service::Error::Other("not impl!".to_string()))
}
