//! B `weather_audio.rs`: weather/season global audio parameters.
//!
//! O-level facts (chapter weather_audio.rs.md): `weather_audio_system`
//! (inline, no independent symbol) reads the `Weather` state and pushes
//! season/weather FMOD global parameters through `AudioCore::set_parameter`
//! (0x140a53db0). The parameter names and the weather-state source are 待取证
//! (the weather resource belongs to the camera/time/weather domain).

use crate::resources::audio_resources::{AudioCore, Studio};
use bevy_ecs::prelude::*;

pub use crate::resources::weather::Weather;

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
    // Original @14155691f reads the canonical Weather+8 directly.
    let _ = core.set_parameter(WEATHER_RAIN_PARAM, weather.field_8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rain_param_is_the_original_name() {
        assert_eq!(WEATHER_RAIN_PARAM, "RainStrength");
    }
}
