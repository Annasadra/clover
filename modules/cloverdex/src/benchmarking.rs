// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Balances pallet benchmarking.
#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, account};
use frame_support::traits::Box;

pub use primitives::{
	AccountId, AccountIndex, Amount, Balance,
	CurrencyId,
	EraIndex, Hash, Index, Moment,
	Rate, Share,
	Signature,
  };
use orml_traits::{MultiCurrency, MultiCurrencyExtended};

const SEED: u32 = 0;

benchmarks! {
	_ { }
	add_liquidity {
		let user = account("tester", 5, SEED);
		let _ = <T as Trait>::Currency::update_balance(CurrencyId::CLV, &user, 5000000.into());
		let _ = <T as Trait>::Currency::update_balance(CurrencyId::CETH, &user, 10000000.into());
	}: _(RawOrigin::Signed(user),
		CurrencyId::CLV,
		CurrencyId::CETH,
		5000,
		10000)

}

#[cfg(test)]
mod tests {
	use super::*;
	use mock::{ExtBuilder, TestRuntime};
	use frame_support::assert_ok;

	#[test]
	fn add_liquidity() {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(test_benchmark_add_liquidity::<TestRuntime>());
		});
	}
}
