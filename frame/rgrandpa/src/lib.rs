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

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(core_panic)]
extern crate core;

use frame_support::dispatch::DispatchResult;

pub use pallet::*;
use frame_support::traits::{OneSessionHandler};
use sp_std::vec::Vec;
use sp_core::crypto::KeyTypeId;
use pallet_staking;
use pallet_babe;
use sp_core::{U256};
use pallet_grandpa;
use pallet_grandpa::AuthorityList;
use sp_runtime::{Percent, traits::Zero};
pub mod weights;
pub use weights::WeightInfo;
use core::panicking::panic;
use log::log;
use sp_core::LogLevel;
use sp_runtime::traits::Printable;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"rgra");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
	};
	app_crypto!(sr25519, KEY_TYPE);
}

/// Identity of a rGrandpa authority.
pub type AuthorityId = crypto::Public;

#[frame_support::pallet]
pub mod pallet {
	use core::panicking::panic;
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use codec::{ EncodeLike};
	use frame_support::{
		traits::{EnsureOrigin},
	};
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_staking::Config + pallet_babe::Config + pallet_grandpa::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Required origin for update para (though can always be Root).
		type UpdateOrigin: EnsureOrigin<Self::Origin>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn cycle_percent)]
	// percent of grandpa consensus count
	pub type CyclePercent<T> = StorageValue<_, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn validator_number)]
	// percent of grandpa consensus count
	pub type ValidatorNumber<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cycle_block_num)]
	// how many block past then may change the validator count for grandpa
	pub type CycleBlockNum<T:Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_change_at)]
	// block number of next change
	pub type NextChangeAt<T:Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn r_next_authorities)]
	// all author we need store,it can be changed on new session,then we need refresh
	pub type RNextAuthorities<T:Config> = StorageValue<_, AuthorityList, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [ CyclePercent, CycleBlockNum]
		ParameterStored( u8, T::BlockNumber),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// new validator count is less then minimum_validator_count.
		LessThenMin,
		WrongNumber,
		WrongValidatorNumber,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn on_finalize(n: T::BlockNumber) {
			let mut t_next_change_at = <NextChangeAt<T>>::get();
			let t_cycle_block_num = <CycleBlockNum<T>>::get();
			if n == t_next_change_at && t_next_change_at != Zero::zero() {
				t_next_change_at += t_cycle_block_num;
				<NextChangeAt<T>>::put(t_next_change_at);
				//then set rgrandpa validator count
				Self::set_random_validator_count().unwrap_or_else(|e| panic!("rgrandpa error!,msg:{:?}",e));
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_parameter())]
		pub fn set_parameter(origin: OriginFor<T>, percent: u8,bnum:T::BlockNumber) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			T::UpdateOrigin::ensure_origin(origin)?;
			//parameter need valid
			let min_num  = <pallet_staking::Module<T>>::minimum_validator_count();
			let mut org_num	= <pallet_staking::Module<T>>::validator_count();
			org_num =  Percent::from_percent(percent as u8) * org_num;
			if org_num >= min_num {
				// Update storage.
				<CyclePercent<T>>::put(percent);
				<CycleBlockNum<T>>::put(bnum);
				let t_next_change_at = <frame_system::Module<T>>::block_number() + bnum;
				<NextChangeAt<T>>::put(t_next_change_at);
				Self::deposit_event(Event::ParameterStored( percent, bnum));
				// Return a successful DispatchResultWithPostInfo
				Ok(().into())
			} else {
				Err(Error::<T>::LessThenMin)?
			}
		}
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub percent: u8,
		pub block_num: T::BlockNumber,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> where
		T::BlockNumber: EncodeLike<u32>
	{
		fn default() -> Self {
			Self {
				percent: 0,
				block_num:T::BlockNumber::zero(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> where
		T::BlockNumber: EncodeLike<u32>
	{
		fn build(&self) {
			<CyclePercent<T>>::put(self.percent);
			<CycleBlockNum<T>>::put(self.block_num);
			<NextChangeAt<T>>::put(self.block_num);
		}
	}
}

impl<T: Config> Pallet<T>{

    pub fn set_random_validator_count() -> DispatchResult {
        let min_num  = <pallet_staking::Module<T>>::minimum_validator_count();

		let org_num	= <pallet_babe::Module<T>>::authorities().len() as u32;
		let ana_num = <ValidatorNumber<T>>::get();
		if org_num != ana_num {
			Err(Error::<T>::WrongValidatorNumber)?
		}
		let mut percent = <CyclePercent<T>>::get();

		if percent > 100 {
			percent = 100;
		}

        let mut confirmer_num = Percent::from_percent(percent as u8)* org_num;

		if confirmer_num < min_num {
			confirmer_num = min_num;
		}

        let randomness = <pallet_babe::Module<T>>::randomness();

        let rand = U256::from(randomness);

        let mut auth_list = <RNextAuthorities<T>>::get();

        let mut count = auth_list.iter().count();

		if count < confirmer_num as usize {
			Err(Error::<T>::WrongNumber)?
		}
        let mut rand_auth:AuthorityList = Vec::new();
        for _i in 0..confirmer_num {
            let j = (rand % U256::from(count)).as_u32() as usize;
            rand_auth.push(auth_list.get(j).unwrap().clone());
            auth_list.remove(j);
            count-=1;
        }
        <pallet_grandpa::Module<T>>::schedule_change(rand_auth, Zero::zero(), None)
    }
}

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Module<T> {
	type Public = AuthorityId;
}
impl<T> OneSessionHandler<T::AccountId> for Module<T>
 where T: Config {
    type Key = AuthorityId;

    fn on_genesis_session<'a, I: 'a>(_validators: I)
        where I: Iterator<Item=(&'a T::AccountId, AuthorityId)>
    {
		let keys = <RNextAuthorities<T>>::get();
		if keys.is_empty() {
			let auth_list = <pallet_grandpa::Module<T>>::grandpa_authorities();
			<RNextAuthorities<T>>::put(auth_list.clone());
			<ValidatorNumber<T>>::put(auth_list.len() as u32);
		}
    }

    fn on_new_session<'a, I: 'a>(changed: bool, validators: I, _queued_validators: I)
        where I: Iterator<Item=(&'a T::AccountId, AuthorityId)>
    {
        if changed {
			if let Some(pending_change) = <pallet_grandpa::Module<T>>::pending_change()  {
				if <NextChangeAt<T>>::get() != Zero::zero() {
					let next_authorities = validators.map(|(_, k)| (k, 1)).collect::<Vec<_>>();
					<ValidatorNumber<T>>::put(next_authorities.len() as u32);
					<RNextAuthorities<T>>::put(pending_change.next_authorities);
					let mut t_next_change_at = <frame_system::Module<T>>::block_number();
					t_next_change_at += T::BlockNumber::from(1 as u32);
					<NextChangeAt<T>>::put(t_next_change_at);
				}
			}
        }
    }

    fn on_disabled(_i: usize) {
    }
}