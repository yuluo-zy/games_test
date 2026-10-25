//! `startup/load_glade_tod_and_weather.rs` — glade time-of-day + weather, snow
//! particles.
//!
//! Source: B skeleton `crates/country-core/src/startup/load_glade_tod_and_weather.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `load_glade_tod_and_weather` @0x140ae7810 (2 callers).
//! - `load_snow_particles` @0x140ae7d90 (2 callers) — winter snow particle
//!   system spawn.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn load_glade_tod_and_weather(_session: &Gap, _commands: &mut Gap) {
    todo!("recover load_glade_tod_and_weather")
}

pub fn load_snow_particles(_commands: &mut Gap) {
    todo!("recover load_snow_particles")
}
