//! `startup/spawn_human_ref.rs` — human-scale reference box spawn.
//!
//! Source: B skeleton `crates/country-core/src/startup/spawn_human_ref.rs`.
//! O evidence: `spawn_human_ref` @0x14089d5a0 (2 callers) — uses
//! `geometry::cube::Box::new` (inline host) and spawns a reference box entity
//! at a fixed origin for human-scale grounding.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn spawn_human_ref(_commands: &mut Gap) {
    todo!("recover spawn_human_ref (cube Box + DrawableMeshBundle)")
}
