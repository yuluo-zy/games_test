//! `systems/time_of_day.rs` — the time-of-day system: interpolates the sun
//! angle + hour, computes the sun position (suncalc), drives the directional
//! light and reports the hour to audio.
//!
//! PDB: `time_of_day @0x140a73460` (4912 B), `remap_hour_to_fmod_hour
//! @0x140a74790` (1165 B).
//!
//! O: (see `contracts/time-weather.md`) — sun-angle interpolation toward
//! `(2π - target)` with `exp(-3·dt)` smoothing (converge < 0.01 rad), hour
//! interpolation toward the target hour (wrap ±0.5), play-state progress at
//! `dt/3` (fwd) / `dt/-5` (rev), `hour = fmodf(progress·24 + 8, 24)` (day
//! starts 08:00), `TIME_OF_DAY_SPEED` scaling, date→`OffsetDateTime`→
//! `suncalc::get_position`, then colatitude `π/2 − (azimuth + π)` feeds the
//! light.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::game_config::developer_hotkeys_enabled as DEVELOPER_HOTKEYS_ENABLED;
use crate::resources::audio_resources::AudioCoreInner;
use crate::resources::time::GameTime;
use crate::resources::time_of_day::TimeOfDay;
use crate::reused::bevy_input::{ButtonInput, KeyCode};
use crate::reused::{puffin, suncalc};
use crate::Gap;

/// `time_of_day` @0x140a73460 (O).
pub fn time_of_day(
    _time: Res<GameTime>,
    mut _state: ResMut<TimeOfDay>,
    mut _audio: ResMut<AudioCoreInner>,
    _keys: Res<ButtonInput<KeyCode>>,
    mut _light: ResMut<Gap>,
    mut _sun_sky: ResMut<Gap>,
    _suncalc_input: Res<Gap>,
) {
    let _scope = puffin::Scope::new("time_of_day");
    todo!("recover time_of_day (angle/hour interpolation + suncalc + light/audio)")
}

/// `remap_hour_to_fmod_hour` @0x140a74790 (O): hour → fmod-hour keyframe curve
/// (base `{0,3.5,9,17,20,24} → {48,17,11,24,22,...}`; winter inserts
/// 4.52/6.2/5.0/9.0) with a linear-interpolation lookup.
pub fn remap_hour_to_fmod_hour(_hour: f32, _winter: bool) -> f32 {
    // Keyframe table built at call time (O); interpolation search O.
    todo!("recover remap_hour_to_fmod_hour keyframe interpolation")
}
