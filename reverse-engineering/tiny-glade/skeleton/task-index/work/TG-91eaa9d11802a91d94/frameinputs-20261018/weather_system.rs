//! Candidate original B weather_system@1408e5b00, complete187B business.
use crate::resources::{session::ActiveSession, time::GameTime, weather::Weather};
use bevy_ecs::prelude::*;

pub fn weather_system(
    mut weather: ResMut<Weather>,
    time: Res<GameTime>,
    session: Res<ActiveSession>,
) {
    if session.is_winter() {
        let wave = ((time.elapsed.as_secs_f32() / 300.0) * core::f32::consts::PI).sin();
        let delta = time.delta * 0.1;
        let next = weather.snow_amount + if wave > 0.0 { delta } else { -delta };
        // Native maxss/minss propagate a NaN source. f32::clamp does too.
        weather.snow_amount = next.clamp(0.0, 1.0);
    }
}
