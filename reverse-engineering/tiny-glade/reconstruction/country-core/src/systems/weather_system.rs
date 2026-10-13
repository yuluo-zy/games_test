//! `systems/weather_system.rs` — winter snow-intensity oscillator.
//!
//! PDB: `weather_system @0x1408e5b00` (187 B).
//!
//! O: when `glade_type @+0x58 == 4` (winter), a 10-minute sine wave
//! `sin(game_time/300 · π)` drives a snow-intensity value @+0xc:
//! `intensity += (sin > 0 ? +0.1·frac : −0.1·frac)` clamped to [0, 1]
//! (`frac` = the clock's fractional-seconds field).

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::resources::time::GameTime;
use crate::Gap;

/// `SnowIntensity` (O: intensity value @+0xc; layout U).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SnowIntensity {
    pub raw: Gap,
}

/// `weather_system` @0x1408e5b00 (O).
pub fn weather_system(
    _time: Res<GameTime>,
    _glade: Res<Gap>,
    mut _intensity: ResMut<SnowIntensity>,
) {
    todo!("recover weather_system (winter sine oscillator + 0.1 clamp)")
}
