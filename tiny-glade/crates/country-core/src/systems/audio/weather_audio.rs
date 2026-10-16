//! B `weather_audio.rs`: weather/season global audio parameters.
//!
//! O-level facts (chapter weather_audio.rs.md): `weather_audio_system`
//! (inline, no independent symbol) reads the `Weather` state and pushes
//! season/weather FMOD global parameters through `AudioCore::set_parameter`
//! (0x140a53db0). The parameter names and the weather-state source are 待取证
//! (the weather resource belongs to the camera/time/weather domain).

use crate::resources::audio_resources::{AudioCore, Studio};
use bevy_ecs::prelude::*;

/// Weather state (native `resources::weather` 待接线). The season index drives
/// the global parameter below.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct Weather {
    pub season: u8,
    pub time_of_day: f32,
}

/// Global season parameter — 待取证 (name not resolved; documented placeholder
/// the host reconciles against the FMOD project).
pub const WEATHER_SEASON_PARAM: &str = "WeatherSeason";

/// `weather_audio_system` (native inline): push the weather/season global
/// parameters.
pub fn weather_audio_system<S: Studio + Send + Sync + 'static>(
    weather: Res<Weather>,
    core: Res<AudioCore<S>>,
) {
    let _ = core.set_parameter(WEATHER_SEASON_PARAM, f32::from(weather.season));
    // O: the native also drives time-of-day / weather strength params; those
    // names are 待取证.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn season_param_is_the_documented_placeholder() {
        assert_eq!(WEATHER_SEASON_PARAM, "WeatherSeason");
    }
}
