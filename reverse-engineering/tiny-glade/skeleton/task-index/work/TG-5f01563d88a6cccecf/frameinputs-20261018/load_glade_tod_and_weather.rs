//! Candidate complete original B load_glade_tod_and_weather@140ae7810.
//! Caller must preserve original asset working-directory context; no invented
//! empty sequence is installed if both real files fail.
use crate::resources::{
    glade_settings::GladeSettings, session::ActiveSession, time_of_day::TodSequence,
    weather::Weather,
};
use bevy_ecs::prelude::*;

pub fn load_glade_tod_and_weather(
    mut sequence: ResMut<TodSequence>,
    mut weather: ResMut<Weather>,
    settings: Res<GladeSettings>,
    session: Res<ActiveSession>,
) {
    let glade_path = session.assets_root().join("tod.json");
    let json = match std::fs::read_to_string(glade_path) {
        Ok(json) => json,
        Err(_) => std::fs::read_to_string("assets/tod/default.json").unwrap(),
    };
    // A malformed glade-specific JSON does not fall back in the original.
    *sequence = serde_json::from_str(&json).unwrap();
    *weather = Weather::default();
    weather.weather_fog = settings.weather_fog;
    weather.wind_strength = settings.wind_strength;
    weather.field_8 = 0.0;
    weather.snow_amount = if session.is_winter() { 1.0 } else { 0.0 };
}
