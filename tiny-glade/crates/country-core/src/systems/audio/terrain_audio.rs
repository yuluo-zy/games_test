//! B `terrain_audio.rs`: terrain edit sounds — 4 loops + an edit-state machine.
//!
//! O-level facts (chapter terrain_audio.rs.md, decomp-0x1414613e0 /
//! 0x1414618c0, rne-read): init creates 4 loops — `event:/SFX/TerrainEditShape`,
//! `event:/SFX/TerrainEditHeight`, `event:/SFX/Terrain Draw`,
//! `event:/SFX/TerrainMove`. `terrain_audio_system` consumes `PlayTerrainSound`
//! (the terrain edit producers), maps each variant to loop should-play flags
//! and positions (via `EditStateAudio::new`/`update`), then frame-end
//! start_if_not_playing / stop per loop.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::terrain::PlayTerrainSound,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The four terrain edit loops (O: rne-read).
pub const TERRAIN_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/TerrainEditShape",
    "event:/SFX/TerrainEditHeight",
    "event:/SFX/Terrain Draw",
    "event:/SFX/TerrainMove",
];

/// `EditStateAudio` (native inline `new` line 18 / `update` line 25): tracks
/// whether a terrain stroke is in progress (drives the "Terrain Draw" loop).
#[derive(Default)]
pub struct EditStateAudio {
    pub editing: bool,
}

impl EditStateAudio {
    pub fn new() -> Self {
        Self { editing: false }
    }

    /// `update` (line 25): fold one terrain event into the editing flag.
    pub fn update(&mut self, sound: &PlayTerrainSound) {
        match sound {
            PlayTerrainSound::StrokeChanged => {}
            PlayTerrainSound::Falloff { .. } => self.editing = true,
            PlayTerrainSound::Height { .. } => self.editing = true,
            PlayTerrainSound::Points { .. } => self.editing = true,
            PlayTerrainSound::Removed { .. } => self.editing = false,
        }
    }
}

/// TerrainAudioState: the four loops + the edit state machine.
#[derive(Resource)]
pub struct TerrainAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 4],
    pub edit: EditStateAudio,
    /// should-play flags for each loop (native +0x38/+0x78/+0xb8/+0xf8).
    pub should_play: [bool; 4],
    /// last event position (native +0x40/+0x80/+0xc0/+0x100 per-loop position).
    pub positions: [Vec3; 4],
}

impl<E: EventInstance> TerrainAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TerrainAudioState {
            loops: TERRAIN_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
            edit: EditStateAudio::new(),
            should_play: [false; 4],
            positions: [Vec3::ZERO; 4],
        }
    }
}

/// `terrain_audio_init_system` (0x1414613e0).
pub fn terrain_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TerrainAudioState::<S::Event>::new(&core));
}

/// Map a `PlayTerrainSound` variant to (loop index, should-play) (O: the native
/// jump table 0x142c63274 dispatches variant → loop + flag).
fn variant_loop(sound: &PlayTerrainSound) -> (usize, bool, Vec3) {
    match sound {
        PlayTerrainSound::StrokeChanged => (2, true, Vec3::ZERO),
        PlayTerrainSound::Falloff { position, .. } => (0, true, *position),
        PlayTerrainSound::Height { position, .. } => (1, true, *position),
        PlayTerrainSound::Points { position, .. } => (1, true, *position),
        PlayTerrainSound::Removed { position, .. } => (3, true, *position),
    }
}

/// `terrain_audio_system` (0x1414618c0): fold terrain events into the loop
/// should-play flags + positions, then frame-end start/stop each loop.
pub fn terrain_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TerrainAudioState<S::Event>>,
    mut events: EventReader<PlayTerrainSound>,
    _core: Res<AudioCore<S>>,
) {
    for sound in events.read() {
        state.edit.update(sound);
        let (index, play, position) = variant_loop(sound);
        state.should_play[index] = play;
        state.positions[index] = position;
    }

    let should_play = std::mem::take(&mut state.should_play);
    let positions = std::mem::take(&mut state.positions);
    for (index, loop_player) in state.loops.iter_mut().enumerate() {
        let play = should_play[index];
        if play {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            let pos = positions[index];
            let _ = current.set_position([pos.x, pos.y, pos.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
    }
    state.should_play = should_play;
    state.positions = positions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_loop_names_are_original() {
        assert_eq!(TERRAIN_LOOP_EVENTS[0], "event:/SFX/TerrainEditShape");
        assert_eq!(TERRAIN_LOOP_EVENTS[1], "event:/SFX/TerrainEditHeight");
        assert_eq!(TERRAIN_LOOP_EVENTS[2], "event:/SFX/Terrain Draw");
        assert_eq!(TERRAIN_LOOP_EVENTS[3], "event:/SFX/TerrainMove");
    }

    #[test]
    fn edit_state_folds_variants() {
        let mut edit = EditStateAudio::new();
        edit.update(&PlayTerrainSound::Falloff {
            position: Vec3::ZERO,
            falloff: 0.0,
            magnitude: 0.0,
        });
        assert!(edit.editing);
        edit.update(&PlayTerrainSound::Removed {
            position: Vec3::ZERO,
            falloff: 0.0,
            magnitude: 0.0,
        });
        assert!(!edit.editing);
    }
}
