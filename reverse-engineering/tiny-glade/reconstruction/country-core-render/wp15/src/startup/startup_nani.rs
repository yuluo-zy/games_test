//! `startup/startup_nani.rs` — nani GPU-world startup (27 subsets, wp3/wp6).
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_nani.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `startup_nani` @0x140a38610 — jumptable dispatch over the world-subset
//!   descriptors (27 subset kinds, wp6), `runtime_check_limits` + `create_world_subset`
//!   (inline hosts), builds the `GpuWorld` + `GpuWorldSubset`s.
//! - `reload_nani` @0x140a398b0 (2 callers) — rebuild on shader/mesh reload.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn startup_nani(_gpu: &Gap, _assets: &Gap, _world: &mut Gap) {
    todo!("recover startup_nani (27 GpuWorldSubset creation)")
}

pub fn reload_nani(_gpu: &Gap, _assets: &Gap, _world: &mut Gap) {
    todo!("recover reload_nani")
}

/// `runtime_check_limits` (O inline in `startup_nani`).
pub fn runtime_check_limits() {}

/// `create_world_subset` (O inline in `startup_nani`).
pub fn create_world_subset(_world: &mut Gap, _desc: &Gap) {
    todo!("recover create_world_subset")
}
