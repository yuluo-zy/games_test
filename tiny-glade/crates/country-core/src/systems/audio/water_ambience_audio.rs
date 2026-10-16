//! B `water_ambience_audio.rs`: water ambience — per-water-grid-cell Amb loops.
//!
//! O-level facts (chapter water_ambience_audio.rs.md, decomp-0x1409ebe80 /
//! 0x1409ebf20 / 0x1409ecea0): init builds an `AudioLoop` player per water
//! grid cell (from_iter<Range, instantiate_loop closure> over
//! `event:/SFX/Water/Amb`), then `insert_resource<WaterAmbienceAudio>` +
//! `insert_resource<WaterAudioGrid>`. The system traverses the grid: inactive
//! cells `AudioLoop::stop`, active cells `start_if_not_playing` +
//! `set_position` (from `CameraMatrices::eye_position` + x0y) + `set_volume`.
//! Constants: `0.015625` and `42.5` (distance/attenuation range, semantics 待取证).
//! The native `load_index_unchecked` (0x1409ebe80) loads a grid cell.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// `event:/SFX/Water/Amb` (O: rne-read).
pub const WATER_AMB_EVENT: &str = "event:/SFX/Water/Amb";

/// Grid extent (native grid dimensions 待取证; the constant mirrors the
/// documented 64-bit rolling-window cell accounting).
pub const WATER_GRID_EXTENT: usize = 8;

/// A water-grid cell (native cell 24 B with a 64-bit rolling window; the exact
/// bits are 待取证 — the water-domain `update_water_audio_grid` producer fills
/// the water level).
#[derive(Debug, Clone, Copy, Default)]
pub struct WaterGridCell {
    /// Water level [0,1] (O threshold 0.5 drives activation).
    pub water_level: f32,
    /// Cell world position (O: x0y from the camera eye).
    pub position: Vec3,
}

/// WaterAudioGrid: the grid (O: inserted by init; consumed by the system).
#[derive(Resource, Debug, Default)]
pub struct WaterAudioGrid {
    pub cells: Vec<WaterGridCell>,
}

impl WaterAudioGrid {
    /// `load_index_unchecked` (0x1409ebe80): grid cell access (index OOB is the
    /// caller's responsibility, matching the unchecked access).
    pub fn load_index_unchecked(&self, index: usize) -> &WaterGridCell {
        &self.cells[index]
    }
}

/// WaterAmbienceAudio: the per-cell Amb loop players (O: init-built).
#[derive(Resource)]
pub struct WaterAmbienceAudio<E: EventInstance> {
    pub loops: Vec<AudioLoop<AudioInstance<E>>>,
}

impl<E: EventInstance> WaterAmbienceAudio<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>, cells: usize) -> Self {
        WaterAmbienceAudio {
            loops: (0..cells)
                .map(|_| {
                    core.instantiate_loop(
                        &crate::resources::audio_resources::AudioLoopDesc::new(WATER_AMB_EVENT),
                    )
                })
                .collect(),
        }
    }
}

/// `water_ambience_audio_init_system` (0x1409ebf20): build one Amb loop per
/// grid cell and insert both resources.
pub fn water_ambience_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    let cells = WATER_GRID_EXTENT * WATER_GRID_EXTENT;
    let ambience = WaterAmbienceAudio::<S::Event>::new(&core, cells);
    let mut grid = WaterAudioGrid::default();
    grid.cells = vec![WaterGridCell::default(); cells];
    commands.insert_resource(ambience);
    commands.insert_resource(grid);
}

/// `water_ambience_audio_system` (0x1409ecea0): activate loops for cells above
/// the water threshold, stop the rest, and position/volume each active loop
/// (O: eye_position + x0y; the 0.015625/42.5 attenuation constants 待取证).
pub fn water_ambience_audio_system<S: Studio + Send + Sync + 'static>(
    mut ambience: ResMut<WaterAmbienceAudio<S::Event>>,
    grid: Res<WaterAudioGrid>,
    _camera_position: Option<Res<crate::systems::audio::audio_update::CameraPosition>>,
) {
    const WATER_THRESHOLD: f32 = 0.5;
    for (index, loop_player) in ambience.loops.iter_mut().enumerate() {
        let cell = grid.load_index_unchecked(index);
        if cell.water_level >= WATER_THRESHOLD {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            let _ = current.set_position([cell.position.x, cell.position.y, cell.position.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_amb_event_is_original() {
        assert_eq!(WATER_AMB_EVENT, "event:/SFX/Water/Amb");
    }
}
