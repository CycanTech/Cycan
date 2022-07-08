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

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_core::crypto::AccountId32;
use sp_runtime::{
	assert_eq_error_rate, traits::BadOrigin,
};

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
#[test]
fn rgrandpa_set_param() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(RGrandpa::set_parameter(Origin::signed(ALICE), 20, 20), BadOrigin);
		assert_ok!(RGrandpa::set_parameter(Origin::root(), 20, 20));
		assert_eq!(RGrandpa::cycle_confirmer_num(), 20);
		assert_eq!(RGrandpa::cycle_block_num(), 20);
		assert_eq!(Staking::minimum_validator_count(), 2);
		assert_eq!(Staking::validator_count(), 100);
	});
}
