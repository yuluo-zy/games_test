//! country-core-wall — WP12 reconstruction of the Tiny Glade `country_core::systems::wall`
//! ECS systems (version B; target = B patched EXE `cf048e27...`, PDB `2d9686b8...`).
//!
//! Recovered per the project workflow: (1) business logic from B decompiles + PDB skeleton
//! evidence (skeleton/B/crates/country-core/src/systems/wall/, 57 source units) + call chains;
//! (2) organized contracts under `contracts/` (O/I/U + typed gaps); (3) skeleton-aligned Rust.
//! The ECS systems call the real `bevy_ecs` 0.16 API. Wall **state** types belong to WP11 and
//! are referenced as typed-gap facades (`crate::resources::walls::*`), never fabricated.
//!
//! This crate is a standalone offline `cargo check` gate; the integrator owns merging into
//! `reconstruction/country-core`. All `impl*` modules carry `implemented_unverified`.

extern crate alloc;

pub mod audio;
pub mod components;
pub mod render;
pub mod resources;
pub mod systems;
pub mod ui;
pub mod utils;

// ---------------------------------------------------------------------------
// Typed-gap marker + facade for the offline-cache gap `puffin` (never fabricated).
// ---------------------------------------------------------------------------

/// Typed-gap marker: a field whose layout/semantics were not recovered from the
/// original machine code.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Gap;

pub mod typed_gap {
    pub use crate::Gap;
}

pub mod puffin_contract {
    /// Reused crate `puffin` is absent from the offline registry cache; systems only
    /// bracket their bodies in scopes. The scope boundary (begin/end) is behaviour-neutral
    /// here; profiling is not part of gameplay business.
    pub struct ProfilerScope;
    impl ProfilerScope {
        #[inline]
        pub fn new(_name: &'static str) -> Self {
            ProfilerScope
        }
    }
    #[inline]
    pub fn are_scopes_on() -> bool {
        false
    }
}
