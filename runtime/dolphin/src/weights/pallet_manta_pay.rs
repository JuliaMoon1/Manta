// Copyright 2020-2022 Manta Network.
// This file is part of Manta.

// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_manta_pay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dolphin-dev"), DB CACHE: 1024

// Executed Command:
// manta
// benchmark
// --chain=dolphin-dev
// --pallet=pallet_manta_pay
// --extrinsic=*
// --execution=Wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --repeat=20
// --steps=50
// --template=.github/resources/frame-weight-template.hbs
// --output=pallet_manta_pay.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_manta_pay.
pub trait WeightInfo {
	fn to_private() -> Weight;
	fn to_public() -> Weight;
	fn private_transfer() -> Weight;
	fn public_transfer() -> Weight;
}

/// Weights for pallet_manta_pay using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_manta_pay::WeightInfo for SubstrateWeight<T> {
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay VoidNumberSetSize (r:1 w:0)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_private() -> Weight {
		(66_776_431_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
	// Storage: MantaPay VoidNumberSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay VoidNumberSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay VoidNumberSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_public() -> Weight {
		(82_758_559_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
	// Storage: MantaPay VoidNumberSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:2 w:2)
	// Storage: MantaPay VoidNumberSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:2 w:2)
	// Storage: MantaPay VoidNumberSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:2)
	fn private_transfer() -> Weight {
		(90_018_125_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(13 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	fn public_transfer() -> Weight {
		(41_396_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay VoidNumberSetSize (r:1 w:0)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_private() -> Weight {
		(66_776_431_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
	// Storage: MantaPay VoidNumberSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay VoidNumberSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay VoidNumberSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_public() -> Weight {
		(82_758_559_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
	// Storage: MantaPay VoidNumberSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:2 w:2)
	// Storage: MantaPay VoidNumberSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:2 w:2)
	// Storage: MantaPay VoidNumberSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:2)
	fn private_transfer() -> Weight {
		(90_018_125_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(13 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	fn public_transfer() -> Weight {
		(41_396_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
