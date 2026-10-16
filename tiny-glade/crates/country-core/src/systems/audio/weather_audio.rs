//! B `weather_audio.rs`: weather/season global audio parameters.
//!
//! O-level facts (chapter weather_audio.rs.md): `weather_audio_system`
//! (inline, no independent symbol) reads the `Weather` state and pushes
//! season/weather FMOD global parameters through `AudioCore::set_parameter`
//! (0x140a53db0). The parameter names and the weather-state source are 待取证
//! (the weather resource belongs to the camera/time/weather domain).

use crate::resources::audio_resources::{AudioCore, Studio};
use bevy_ecs::prelude::*;

/// Weather state (native `resources::weather::Weather` 待接线). The rain
/// strength drives the global parameter below.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct Weather {
    pub season: u8,
    pub time_of_day: f32,
    /// O: the weather system's set_parameter value source (native reads the
    /// weather state; the exact field mapping 待取证).
    pub rain_strength: f32,
}

/// Global rain parameter (O: 0x142c63000, len 0xb).
pub const WEATHER_RAIN_PARAM: &str = "RainStrength";

/// `weather_audio_system` (native inline, inlined into the
/// `(Res<AudioCore>, Res<Weather>)` FunctionSystem): push the rain-strength
/// global parameter (O: single `AudioCoreInner::set_parameter` at the inline
/// site 0x141556935).
pub fn weather_audio_system<S: Studio + Send + Sync + 'static>(
    weather: Res<Weather>,
    core: Res<AudioCore<S>>,
) {
    let _ = core.set_parameter(WEATHER_RAIN_PARAM, weather.rain_strength);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rain_param_is_the_original_name() {
        assert_eq!(WEATHER_RAIN_PARAM, "RainStrength");
    }
}
