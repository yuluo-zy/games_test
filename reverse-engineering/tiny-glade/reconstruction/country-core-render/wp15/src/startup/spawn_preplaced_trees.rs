//! `startup/spawn_preplaced_trees.rs` — pre-placed forest trees.
//!
//! Source: B skeleton `crates/country-core/src/startup/spawn_preplaced_trees.rs`.
//! O evidence (Ghidra, B patched EXE; `spawn_preplaced_trees` @0x140862270,
//! 2 callers): builds `ForestTree` entries from `PlacementDataSsbo` rows into
//! `resources::tree_storage::TreeStorage` (hashbrown + next_index, wp10).

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn spawn_preplaced_trees(_storage: &mut Gap, _placement: &Gap) {
    todo!("recover spawn_preplaced_trees (PlacementDataSsbo -> TreeStorage ForestTree)")
}
