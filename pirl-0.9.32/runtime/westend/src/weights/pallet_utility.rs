// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 11_344 nanoseconds.
		Weight::from_ref_time(15_084_816 as u64)
			// Standard Error: 2_719
			.saturating_add(Weight::from_ref_time(3_318_885 as u64).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 5_807 nanoseconds.
		Weight::from_ref_time(6_033_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 11_444 nanoseconds.
		Weight::from_ref_time(18_634_415 as u64)
			// Standard Error: 3_166
			.saturating_add(Weight::from_ref_time(3_479_338 as u64).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 13_410 nanoseconds.
		Weight::from_ref_time(13_821_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 11_023 nanoseconds.
		Weight::from_ref_time(14_615_133 as u64)
			// Standard Error: 2_404
			.saturating_add(Weight::from_ref_time(3_327_844 as u64).saturating_mul(c as u64))
	}
}
