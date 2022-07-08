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

//! Benchmarking setup for pallet-rgrandpa

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use sp_std::{vec, vec::Vec, boxed::Box};
use frame_system::Origin;

benchmarks! {
	set_parameter {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, 100,T::BlockNumber::from(20 as u32))
	verify {
		assert_eq!(CyclePercent::<T>::get(), 100);
	}
}

impl_benchmark_test_suite!(
  	RGrandpa,
  	crate::mock::ExtBuilder::default().build(),
	crate::mock::Test,
);
