//! `startup/startup_time_of_day.rs` — default time-of-day boot.
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_time_of_day.rs`.
//! O evidence (Ghidra, B patched EXE): `startup_time_of_day` @0x14089d6f0 —
//! `load_default_tod` (inline) initializes the `TimeOfDay` resource
//! (`resources::time_of_day::TimeOfDay`).

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn startup_time_of_day(_time_of_day: &mut Gap) {
    load_default_tod();
}

/// `load_default_tod` (O inline host).
pub fn load_default_tod() {
    todo!("recover load_default_tod (TimeOfDay default values)")
}
