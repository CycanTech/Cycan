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

//! Benchmarks for esbind Pallet
use crate::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{vec, vec::Vec, boxed::Box};
use sp_core::{U256, H256, H160};
use core::str::FromStr;
benchmarks!{
   // Individual benchmarks are placed here
  bind_account_else {
    let secrect = [0x46,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];
    let eth_addr = H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
    let sig = crate::Module::<T>::eth_sign(&secrect);
    let caller: T::AccountId = whitelisted_caller();
  }: bind_account(RawOrigin::Signed(caller.clone()), eth_addr, sig)
  verify {
        assert_eq!(S2EMap::<T>::get(&caller).unwrap(), eth_addr);
  }

  bind_account_if {
  let secrect = [0x46,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];
  let eth_addr = H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
  let eth_addr2 = H160::from_str("2B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
  let sig = crate::Module::<T>::eth_sign(&secrect);
  let caller: T::AccountId = whitelisted_caller();
  S2EMap::<T>::insert(&caller, &eth_addr2);
  E2SMap::<T>::insert(&eth_addr2, &caller);
  }: bind_account(RawOrigin::Signed(caller.clone()), eth_addr, sig)
  verify {
        assert_eq!(S2EMap::<T>::get(&caller).unwrap(), eth_addr);
  }
}

impl_benchmark_test_suite!(
  ESBind,
  crate::mock::ExtBuilder::default().build(),
  crate::mock::Test,
);