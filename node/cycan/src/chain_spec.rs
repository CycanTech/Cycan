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

use cycan_runtime::{
	AccountId, BabeConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	Signature, SudoConfig, SystemConfig,StakerStatus,{opaque::SessionKeys as SessionKeys}, DOLLARS, SessionConfig, StakingConfig,
	ContractsConfig,
	//ImOnlineConfig,
	wasm_binary_unwrap,IndicesConfig,CouncilConfig,DemocracyConfig,ElectionsConfig,TechnicalCommitteeConfig,RGrandpaConfig,
	AuthorityDiscoveryConfig
};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};
use sp_runtime::{Perbill};
use pallet_rgrandpa::AuthorityId as RGrandpaId;
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};

use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
const BETA_PROPERTIES: &str = r#"
{
    "ss58Format": 42,
    "tokenDecimals": 18,
    "tokenSymbol": "CYN"
}"#;
const BETA_PROTOCOL_ID: &str = "cyn";

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate a crypto pair from seed.
pub fn my_get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate an account ID from seed.
pub fn my_get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
	where
		AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(my_get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, BabeId, GrandpaId,
												AuthorityDiscoveryId,
												RGrandpaId,
												//ImOnlineId,
												) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
		get_from_seed::<RGrandpaId>(seed),
		//get_from_seed::<ImOnlineId>(seed),

	)
}

pub fn my_authority_keys_from_seed(index:&str,seed: &str) -> (AccountId, AccountId, BabeId, GrandpaId,
															  AuthorityDiscoveryId,
															  RGrandpaId,
															  //ImOnlineId,
															  ) {
	(
		my_get_account_id_from_seed::<sr25519::Public>(&format!("{}//{}//stash", seed, index)),
		my_get_account_id_from_seed::<sr25519::Public>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<BabeId>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<GrandpaId>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<AuthorityDiscoveryId>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<RGrandpaId>(&format!("{}//{}", seed, index)),
		//my_get_from_seed::<ImOnlineId>(&format!("{}//{}", seed, index)),
	)
}
pub fn my_stash_and_control_keys_from_seed(index:&str,seed: &str) -> (AccountId, AccountId) {
	(
		my_get_account_id_from_seed::<sr25519::Public>(&format!("{}//{}//stash", seed, index)),
		my_get_account_id_from_seed::<sr25519::Public>(&format!("{}//{}", seed, index)),
	)
}
pub fn development_config() -> Result<ChainSpec, String> {

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}
pub fn beta_config() -> Result<ChainSpec, String> {

	Ok(ChainSpec::from_genesis(
		// Name
		"beta",
		// ID
		"beta",
		ChainType::Live,
		move || {
			beta_genesis(
				// Initial PoA authorities
				vec![
					 my_authority_keys_from_seed("1","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("2","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("3","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("4","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
				],
				// Sudo account
				my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
				// Pre-funded accounts
				vec![
				],
				vec![
					my_stash_and_control_keys_from_seed("5","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_stash_and_control_keys_from_seed("6","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_stash_and_control_keys_from_seed("7","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_stash_and_control_keys_from_seed("8","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
				],
				vec![
					my_authority_keys_from_seed("9","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("10","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("11","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("12","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("13","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("14","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("15","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("16","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("17","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("18","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("19","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("20","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("21","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("22","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("23","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					my_authority_keys_from_seed("24","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
				],
				true,
			)
		},
		// Bootnodes
		vec![
			// "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
		],
		// Telemetry
		None,
		// Protocol ID
		Some(BETA_PROTOCOL_ID),
		// Properties
		serde_json::from_str(BETA_PROPERTIES).unwrap(),
		// Extensions
		None,
	))
}

fn beta_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId,
							  AuthorityDiscoveryId,
							  RGrandpaId,
							  //ImOnlineId,
							  )>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	nominator_accounts: Vec<(AccountId, AccountId)>,
	nominate_accounts: Vec<(AccountId, AccountId, BabeId, GrandpaId,
							AuthorityDiscoveryId,
							RGrandpaId,
							//ImOnlineId,
							)>,
	_enable_println: bool,
) -> GenesisConfig {
	const INITIAL_STAKING: u128 = 10_000 * DOLLARS;
	const ENDOWMENT: u128 = 10_000_000 * DOLLARS;
	const STASH: u128 = ENDOWMENT / 1000;
	let num_nominate_accounts = nominate_accounts.len();
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: initial_authorities
				.iter()
				.cloned()
				.map(|k| (k.0.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().cloned().map(|k| (k.1.clone(), ENDOWMENT)))
				.chain(endowed_accounts
						   	.iter()
						   	.cloned()
						   	.map(|k| (k, ENDOWMENT)))
				.chain(nominator_accounts
					.iter()
					.cloned()
					.map(|k| (k.0.clone(), ENDOWMENT)))
				.chain(nominator_accounts
					.iter()
					.cloned()
					.map(|k| (k.1.clone(), ENDOWMENT)))
				.chain(nominate_accounts
					.iter()
					.cloned()
					.map(|k| (k.0.clone(), ENDOWMENT)))
				.chain(nominate_accounts
					.iter()
					.cloned()
					.map(|k| (k.1.clone(), ENDOWMENT)))
				.chain(vec![(root_key.clone(), ENDOWMENT)])
				.collect(),
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		// pallet_im_online: Some(ImOnlineConfig {
		// 	keys: vec![],
		// }),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				// log::info!("========================================{},{},{},{},{},{}",x.0.clone(),x.1.clone(),x.2.clone(),x.3.clone(),x.4.clone(),x.5.clone());
				(x.0.clone(),
				 x.0.clone(),
				 SessionKeys { babe:  x.2.clone(), grandpa: x.3.clone(),
					 authority_discovery: x.4.clone(),
					 rgrandpa: x.5.clone(),
					 //im_online:x.6.clone(),
					 }
				)
			})
				// .chain(nominate_accounts
			   // .iter()
			   // .map(|x| {
				//    // log::info!("========================================{},{},{},{},{},{}",x.0.clone(),x.1.clone(),x.2.clone(),x.3.clone(),x.4.clone(),x.5.clone());
			   //
				//    (x.0.clone(),
				// 	x.0.clone(),
				// 	SessionKeys { babe:  x.2.clone(), grandpa: x.3.clone(), rgrandpa: x.4.clone(), im_online:x.5.clone(), authority_discovery: x.6.clone()}
				//    )
			   // }))
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.enumerate()
				.map(|(idx,x)| {
						(x.0.clone(), x.1.clone(), INITIAL_STAKING+ idx as u128 *100*DOLLARS, StakerStatus::Validator)
				})
				.chain(
					nominator_accounts
						.iter()
						.enumerate()
						.map(|(idx,c)| {
							(c.0.clone(), c.1.clone(), INITIAL_STAKING + idx as u128 *200*DOLLARS , StakerStatus::Nominator(nominate_accounts.clone().iter().map(|x| x.0.clone()).collect()))
						})
				)
				.chain(nominate_accounts
								.iter()
								.map(|x| {
									(x.0.clone(), x.1.clone(), INITIAL_STAKING, StakerStatus::Idle)
								}))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_evm: Some(EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// H160 address of CI test runner account
					//Secret Recovery Phrase:foam injury begin blade card involve render cat quick private atom check
					H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						// balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
						// 	.expect("internal U256 is valid; qed"),
						balance: U256::from(ENDOWMENT),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		}),
		pallet_ethereum: Some(EthereumConfig {}),
		pallet_dynamic_fee: Some(Default::default()),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				..Default::default()
			},
		}),
		pallet_treasury: Some(Default::default()),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_elections_phragmen: Some(ElectionsConfig::default()),
		pallet_collective_Instance1: Some(CouncilConfig::default()),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig::default()),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_rgrandpa:Some(
			RGrandpaConfig {
				..Default::default()
			}),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
	}
}



/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId,
							  AuthorityDiscoveryId,
							  RGrandpaId,
							  //ImOnlineId,
							  )>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool,
) -> GenesisConfig {
	const INITIAL_STAKING: u128 = 10 * DOLLARS;
	const ENDOWMENT: u128 = 10_000_000 * DOLLARS;

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ENDOWMENT))
				.collect(),
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		// pallet_im_online: Some(ImOnlineConfig {
		// 	keys: vec![],
		// }),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_indices: Some(IndicesConfig {
			indices: vec![],
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				// log::info!("========================================{},{},{},{},{}",x.0.clone(),x.1.clone(),x.2.clone(),x.3.clone(),x.4.clone());
				(x.0.clone(),
				 x.0.clone(),
				 SessionKeys { babe:  x.2.clone(), grandpa: x.3.clone(),
					authority_discovery: x.4.clone(),
					rgrandpa: x.5.clone(),
					//im_online: x.6.clone(),
					  }
				)
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 1,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), INITIAL_STAKING, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_sudo: Some(SudoConfig {
			key: root_key,
		}),
		pallet_evm: Some(EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// H160 address of CI test runner account
					//Secret Recovery Phrase:foam injury begin blade card involve render cat quick private atom check
					H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from(ENDOWMENT),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		}),
		pallet_ethereum: Some(EthereumConfig {}),
		pallet_dynamic_fee: Some(Default::default()),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_elections_phragmen: Some(ElectionsConfig::default()),
		pallet_collective_Instance1: Some(CouncilConfig::default()),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig::default()),
		pallet_rgrandpa:Some(
			RGrandpaConfig {
				..Default::default()
			}),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
	}
}
