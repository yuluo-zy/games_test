//! Original B startup_time_of_day14089d6f0/448B. The complete native
//! wrapper reads the required JSON string and deserializes TodSequence, then
//! replaces that existing owner. File and serde failures both unwrap-panic.
//! Registered in original Startup batch4, not per-session OnEnter(Game).
use crate::resources::time_of_day::TodSequence;
use bevy_ecs::prelude::ResMut;
use std::path::Path;

pub const DEFAULT_TOD_PATH: &str = "assets/tod/default.json";

pub fn load_default_tod_from_str(text: &str) -> anyhow::Result<TodSequence> {
    Ok(serde_json::from_str(text)?)
}

pub fn load_default_tod(path: impl AsRef<Path>) -> anyhow::Result<TodSequence> {
    let text = std::fs::read_to_string(path)?;
    load_default_tod_from_str(&text)
}

pub fn startup_time_of_day(mut sequence: ResMut<TodSequence>) {
    // Host relocation only: B reads the same literal relative to installation;
    // a Cargo-built host keeps installed assets separate from its working dir.
    let loaded = load_default_tod(paths::installed().join(DEFAULT_TOD_PATH)).unwrap();
    *sequence = loaded;
}
