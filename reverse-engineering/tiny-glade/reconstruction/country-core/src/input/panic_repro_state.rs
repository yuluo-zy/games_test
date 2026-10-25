//! `input/panic_repro_state.rs` — `PanicReproState` (panic-reproduction flag
//! state; 17 callers gate debug panic-repro input paths).
//!
//! Source: B skeleton `crates/country-core/src/input/panic_repro_state.rs`.
//! PDB named entity: `PanicReproState`; method `is_inactive` @0x140833230.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `PanicReproState` (O named entity; layout U — a small flag resource).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PanicReproState(pub Gap);

impl PanicReproState {
    /// `PanicReproState::is_inactive` @0x140833230 (O; 17 callers).
    pub fn is_inactive(self) -> bool {
        true
    }
}
