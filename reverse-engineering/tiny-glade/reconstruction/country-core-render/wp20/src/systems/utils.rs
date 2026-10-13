//! `systems/utils.rs` — systems-layer helpers.
//!
//! PDB: `add_stages @0x140945ed0` (128 B).
//!
//! O: `add_stages` configures every `GameSystemSet` variant in the `Update`
//! schedule: `enum_iterator::All<GameSystemSet>` → `configure_sets(Update)`.

#![allow(dead_code, unused_imports)]

use crate::reused::bevy_app::App;
use crate::systems::GameSystemSet;

/// `add_stages` @0x140945ed0 (O).
pub fn add_stages(_app: &mut App) {
    todo!("recover add_stages (enum_iterator::All<GameSystemSet> → configure_sets(Update))")
}

/// `add_stages` iterator over the game system sets (O surface; the standalone
/// gate enumerates an empty set — the real enum_iterator::Sequence wiring is
/// integration-owned).
pub fn all_game_system_sets() -> impl Iterator<Item = GameSystemSet> {
    core::iter::empty()
}
