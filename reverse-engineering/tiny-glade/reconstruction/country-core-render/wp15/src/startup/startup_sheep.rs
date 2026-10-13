//! `startup/startup_sheep.rs` — sheep critter mesh startup.
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_sheep.rs`.
//! O evidence: `startup_sheep` @0x140a093e0 (2 callers) — folds a `Mesh`
//! over `RangeInclusive<usize>` to build sheep variants (sub-mesh folds), then
//! spawns them (critters GPU caps: Sheep shared in `resources::critters`).

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn startup_sheep(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover startup_sheep (sheep mesh variants)")
}
