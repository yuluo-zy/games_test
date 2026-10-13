//! `systems/ui/signifiers/signifier_mark_for_delete.rs` — signifier deletion.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/signifiers/signifier_mark_for_delete.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `signfier_mark_for_delete` @0x140929160 (2 callers; note the PDB
//!   misspelling) — marks signifiers for deletion (consults
//!   `signifier_config_hash` and `UiState`).
//! - `despawn_marked` @0x140929a90 (3 callers) — despawns entities marked
//!   `SignifierMarkedForDeletion`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn signfier_mark_for_delete(_commands: &mut Gap, _ui_state: &Gap) {
    todo!("recover signfier_mark_for_delete")
}

pub fn despawn_marked(_commands: &mut Gap) {
    todo!("recover despawn_marked")
}
