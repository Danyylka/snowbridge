
//! Autogenerated weights for pallet_template
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Alexs-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../../target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_template
// --extrinsic
// *
// --steps=50
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --output
// pallets/template/src/weights.rs
// --template
// ../../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
	fn upgrade(data_size: u32) -> Weight;
	fn create_agent() -> Weight;
	fn create_channel() -> Weight;
	fn update_channel() -> Weight;
	fn set_operating_mode() -> Weight;
	fn transfer_native_from_agent() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn upgrade(data_size: u32) -> Weight {
		Weight::from_parts(30_740_411, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(Weight::from_parts(8_805, 0).saturating_mul(data_size.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	fn create_agent() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Todo: update with real benchmark
	fn create_channel() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	fn update_channel() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	fn set_operating_mode() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	fn transfer_native_from_agent() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
}