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
//! Autogenerated weights for `pallet_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_staking::WeightInfo for WeightInfo<T> {
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn bond() -> Weight {
		// Minimum execution time: 43_104 nanoseconds.
		Weight::from_ref_time(44_132_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: VoterList ListBags (r:2 w:2)
	fn bond_extra() -> Weight {
		// Minimum execution time: 79_519 nanoseconds.
		Weight::from_ref_time(80_713_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	fn unbond() -> Weight {
		// Minimum execution time: 86_781 nanoseconds.
		Weight::from_ref_time(87_699_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		// Minimum execution time: 38_383 nanoseconds.
		Weight::from_ref_time(40_391_884 as u64)
			// Standard Error: 1_926
			.saturating_add(Weight::from_ref_time(13_869 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		// Minimum execution time: 72_726 nanoseconds.
		Weight::from_ref_time(74_449_922 as u64)
			// Standard Error: 1_212
			.saturating_add(Weight::from_ref_time(2_788 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(11 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	fn validate() -> Weight {
		// Minimum execution time: 58_309 nanoseconds.
		Weight::from_ref_time(59_475_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	/// The range of component `k` is `[1, 128]`.
	fn kick(k: u32, ) -> Weight {
		// Minimum execution time: 33_254 nanoseconds.
		Weight::from_ref_time(29_388_895 as u64)
			// Standard Error: 11_549
			.saturating_add(Weight::from_ref_time(6_414_781 as u64).saturating_mul(k as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	/// The range of component `n` is `[1, 24]`.
	fn nominate(n: u32, ) -> Weight {
		// Minimum execution time: 60_454 nanoseconds.
		Weight::from_ref_time(59_639_286 as u64)
			// Standard Error: 7_738
			.saturating_add(Weight::from_ref_time(2_394_179 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill() -> Weight {
		// Minimum execution time: 56_822 nanoseconds.
		Weight::from_ref_time(57_376_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	fn set_payee() -> Weight {
		// Minimum execution time: 14_896 nanoseconds.
		Weight::from_ref_time(15_201_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn set_controller() -> Weight {
		// Minimum execution time: 22_255 nanoseconds.
		Weight::from_ref_time(22_833_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Staking ValidatorCount (r:0 w:1)
	fn set_validator_count() -> Weight {
		// Minimum execution time: 3_940 nanoseconds.
		Weight::from_ref_time(4_374_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_no_eras() -> Weight {
		// Minimum execution time: 3_897 nanoseconds.
		Weight::from_ref_time(4_113_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		// Minimum execution time: 3_954 nanoseconds.
		Weight::from_ref_time(4_130_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era_always() -> Weight {
		// Minimum execution time: 3_944 nanoseconds.
		Weight::from_ref_time(4_107_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Invulnerables (r:0 w:1)
	/// The range of component `v` is `[0, 1000]`.
	fn set_invulnerables(v: u32, ) -> Weight {
		// Minimum execution time: 4_262 nanoseconds.
		Weight::from_ref_time(4_644_196 as u64)
			// Standard Error: 29
			.saturating_add(Weight::from_ref_time(10_165 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn force_unstake(s: u32, ) -> Weight {
		// Minimum execution time: 69_465 nanoseconds.
		Weight::from_ref_time(72_255_081 as u64)
			// Standard Error: 2_127
			.saturating_add(Weight::from_ref_time(868_173 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// The range of component `s` is `[1, 1000]`.
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		// Minimum execution time: 119_457 nanoseconds.
		Weight::from_ref_time(1_336_120_543 as u64)
			// Standard Error: 87_691
			.saturating_add(Weight::from_ref_time(7_429_607 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[0, 256]`.
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		// Minimum execution time: 92_948 nanoseconds.
		Weight::from_ref_time(130_397_907 as u64)
			// Standard Error: 14_923
			.saturating_add(Weight::from_ref_time(19_907_210 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `n` is `[0, 256]`.
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		// Minimum execution time: 108_497 nanoseconds.
		Weight::from_ref_time(162_756_589 as u64)
			// Standard Error: 30_079
			.saturating_add(Weight::from_ref_time(27_543_828 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: VoterList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: VoterList ListBags (r:2 w:2)
	/// The range of component `l` is `[1, 32]`.
	fn rebond(l: u32, ) -> Weight {
		// Minimum execution time: 78_279 nanoseconds.
		Weight::from_ref_time(80_070_464 as u64)
			// Standard Error: 4_979
			.saturating_add(Weight::from_ref_time(49_821 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn reap_stash(s: u32, ) -> Weight {
		// Minimum execution time: 77_611 nanoseconds.
		Weight::from_ref_time(79_448_315 as u64)
			// Standard Error: 2_582
			.saturating_add(Weight::from_ref_time(862_567 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: VoterList ListBags (r:183 w:0)
	// Storage: VoterList ListNodes (r:101 w:0)
	// Storage: Staking Nominators (r:101 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking Bonded (r:101 w:0)
	// Storage: Staking Ledger (r:101 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:0 w:1)
	// Storage: Staking ErasValidatorPrefs (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:1)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	/// The range of component `v` is `[1, 10]`.
	/// The range of component `n` is `[0, 100]`.
	fn new_era(v: u32, n: u32, ) -> Weight {
		// Minimum execution time: 441_741 nanoseconds.
		Weight::from_ref_time(444_325_000 as u64)
			// Standard Error: 1_735_478
			.saturating_add(Weight::from_ref_time(58_700_922 as u64).saturating_mul(v as u64))
			// Standard Error: 172_930
			.saturating_add(Weight::from_ref_time(12_724_841 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(191 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: VoterList CounterForListNodes (r:1 w:0)
	// Storage: Staking SlashingSpans (r:21 w:0)
	// Storage: VoterList ListBags (r:183 w:0)
	// Storage: VoterList ListNodes (r:1500 w:0)
	// Storage: Staking Nominators (r:1500 w:0)
	// Storage: Staking Validators (r:500 w:0)
	// Storage: Staking Bonded (r:1500 w:0)
	// Storage: Staking Ledger (r:1500 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	/// The range of component `v` is `[500, 1000]`.
	/// The range of component `n` is `[500, 1000]`.
	/// The range of component `s` is `[1, 20]`.
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 25_700_092 nanoseconds.
		Weight::from_ref_time(25_792_272_000 as u64)
			// Standard Error: 477_913
			.saturating_add(Weight::from_ref_time(11_545_647 as u64).saturating_mul(v as u64))
			// Standard Error: 477_913
			.saturating_add(Weight::from_ref_time(11_116_420 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(186 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking Validators (r:501 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	/// The range of component `v` is `[500, 1000]`.
	fn get_npos_targets(v: u32, ) -> Weight {
		// Minimum execution time: 3_338_127 nanoseconds.
		Weight::from_ref_time(3_386_919_000 as u64)
			// Standard Error: 39_447
			.saturating_add(Weight::from_ref_time(2_627_920 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_set() -> Weight {
		// Minimum execution time: 7_335 nanoseconds.
		Weight::from_ref_time(7_692_000 as u64)
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_configs_all_remove() -> Weight {
		// Minimum execution time: 6_297 nanoseconds.
		Weight::from_ref_time(6_555_000 as u64)
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking ChillThreshold (r:1 w:0)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	fn chill_other() -> Weight {
		// Minimum execution time: 65_691 nanoseconds.
		Weight::from_ref_time(66_805_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Staking MinCommission (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	fn force_apply_min_commission() -> Weight {
		// Minimum execution time: 14_328 nanoseconds.
		Weight::from_ref_time(14_698_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
