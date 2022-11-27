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
//! Autogenerated weights for `frame_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32, ) -> Weight {
		// Minimum execution time: 2_963 nanoseconds.
		Weight::from_ref_time(3_019_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(409 as u64).saturating_mul(b as u64))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		// Minimum execution time: 11_944 nanoseconds.
		Weight::from_ref_time(12_024_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_718 as u64).saturating_mul(b as u64))
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Minimum execution time: 7_391 nanoseconds.
		Weight::from_ref_time(7_747_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		// Minimum execution time: 3_096 nanoseconds.
		Weight::from_ref_time(3_168_000 as u64)
			// Standard Error: 2_366
			.saturating_add(Weight::from_ref_time(607_856 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		// Minimum execution time: 3_088 nanoseconds.
		Weight::from_ref_time(3_190_000 as u64)
			// Standard Error: 893
			.saturating_add(Weight::from_ref_time(434_445 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		// Minimum execution time: 4_592 nanoseconds.
		Weight::from_ref_time(4_633_000 as u64)
			// Standard Error: 1_146
			.saturating_add(Weight::from_ref_time(928_996 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
}