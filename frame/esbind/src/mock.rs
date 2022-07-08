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

use crate as pallet_esbind;
pub use sp_core::{U256, H256, H160};
use frame_support::{parameter_types};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
pub use pallet_evm::{FeeCalculator, GenesisAccount, };
pub use std::{collections::BTreeMap, str::FromStr};
use frame_support::sp_runtime::AccountId32;
use frame_support::traits::GenesisBuild;
use sp_core::sr25519::Public;
use sp_runtime::app_crypto::Ss58Codec;
use sp_runtime::{
	traits::{Verify, IdentifyAccount}, MultiSignature
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;


// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Module, Call, Storage},
		EVM: pallet_evm::{Module, Config, Call, Storage, Event<T>},
		Ethereum: pallet_ethereum::{Module, Call, Storage, Event, Config},
		ESBind: pallet_esbind::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

impl pallet_esbind::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type AddressMapping = pallet_esbind::ESAddressMapping<BlakeTwo256, Test>;
	type WeightInfo = ();
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u128;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}
pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> U256 {
		1.into()
	}
}

parameter_types! {
	pub const ChainId: u64 = 41;
	pub BlockGasLimit: U256 = U256::from(u32::max_value());
}
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl pallet_evm::Config for Test {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = ();
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping;
	type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
	type AddressMapping = pallet_esbind::ESAddressMapping<BlakeTwo256, Test>;
	type Currency = Balances;
	type Event = Event;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type Precompiles = ();
	type ChainId = ChainId;
	type BlockGasLimit = BlockGasLimit;
	type OnChargeTransaction = ();
	type FindAuthor = ();
}

impl pallet_ethereum::Config for Test {
	type Event = Event;
	type StateRoot = pallet_ethereum::IntermediateStateRoot;
}
pub struct ExtBuilder{
	accouts:std::collections::BTreeMap<H160, GenesisAccount>,
	e2s:Vec<(H160, AccountId32)>,
	s2e:Vec<(AccountId32, H160)>,
	balances:Vec<(AccountId32, u128)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			accouts:BTreeMap::new(),
			e2s:vec![],
			s2e:vec![],
			balances: Default::default(),
		}
	}
}
pub fn get_accountid_from_str(address:&str) -> AccountId32 {
	let pubkey = Public::from_ss58check(address).unwrap();
	AccountId32::from(pubkey)
}

impl ExtBuilder {

	pub fn set_accounts(mut self, f:impl Fn()-> BTreeMap<H160, GenesisAccount> ) -> Self {
		self.accouts = f();
		self
	}
	pub fn set_map(mut self, e2s:Vec<(H160, AccountId32)>,s2e:Vec<(AccountId32, H160)>) -> Self {
		self.e2s = e2s;
		self.s2e = s2e;
		self
	}
	pub fn set_balances(mut self, balances:Vec<(AccountId32, u128)>) -> Self {
		self.balances = balances;
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		// Build genesis storage according to the mock runtime.
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: self.balances,
		}
			.assimilate_storage(&mut t)
			.unwrap();
		pallet_evm::GenesisConfig { accounts:self.accouts }
			.assimilate_storage::<Test>(&mut t)
			.unwrap();
		pallet_esbind::GenesisConfig::<Test> {
			e2s:self.e2s,
			s2e:self.s2e,
		}.assimilate_storage(&mut t)
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}