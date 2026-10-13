//! WP15 — country-core UI / input / startup / app-shell domain (version B).
//!
//! Business-logic-driven reconstruction of the `country_core` game-owned
//! non-render domain: `input`, `startup`, `systems::ui`, app-shell root state
//! (`app_loading_state`, `game_config`, `game_profile`, `game_key_bindings`,
//! `meta_commander`), `geometry` and `utils` helpers.
//!
//! Evidence: B skeleton PDB bundles under
//! `skeleton/B/crates/country-core/src/` + Ghidra decompiles (see
//! `analysis/` and the per-module contracts in `contracts/`).
//! Reused dependencies (bevy_ecs, glam, serde/ron, etc.) are consumed through
//! their real APIs; only game-owned logic is reconstructed here.
//!
//! Layouts marked `Gap` are typed gaps: never fabricated from PDB `void()`
//! placeholders.

#![allow(dead_code, unused_imports, non_camel_case_types, non_snake_case)]

pub mod app_loading_state;
pub mod game_config;
pub mod game_key_bindings;
pub mod game_profile;
pub mod geometry;
pub mod input;
pub mod meta_commander;
pub mod startup;
pub mod systems;
pub mod utils;

/// Typed-gap marker: a field whose layout/semantics were not recovered from the
/// original machine code. Never fabricated.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Gap;

/// Alias module so modules can write `crate::Gap`.
pub mod typed_gap {
    pub use crate::Gap;
}
