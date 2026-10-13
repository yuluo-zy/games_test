//! `ui_clutter/debug_spawn_clutter.rs` — debug clutter spawning.
//!
//! PDB: `debug_spawn_all_clutter @0x1409a1f50` (832 B). Named entities
//! `DebugSpawnClutterCmd`, `DebugSpawnAllClutterCmd`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `DebugSpawnClutterCmd` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct DebugSpawnClutterCmd {
    pub _gap: Gap,
}

/// `DebugSpawnAllClutterCmd` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct DebugSpawnAllClutterCmd {
    pub _gap: Gap,
}

/// `debug_spawn_all_clutter` @0x1409a1f50 (O).
pub fn debug_spawn_all_clutter(
    mut _spawns: EventReader<DebugSpawnClutterCmd>,
    mut _all: EventReader<DebugSpawnAllClutterCmd>,
    _commands: Commands,
) {
    todo!("recover debug_spawn_all_clutter (batch clutter spawn)")
}
