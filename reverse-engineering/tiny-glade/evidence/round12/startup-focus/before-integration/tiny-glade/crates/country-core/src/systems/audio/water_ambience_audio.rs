//! B `water_ambience_audio.rs`: water ambience — per-water-grid-cell Amb loops.
//!
//! O-level facts (chapter water_ambience_audio.rs.md, decomp-0x1409ebe80 /
//! 0x1409ebf20 / 0x1409ecea0, disasm-0x1409ed160): init builds an `AudioLoop`
//! player per water grid cell (from_iter<Range, instantiate_loop closure> over
//! `event:/SFX/Water/Amb`), then `insert_resource<WaterAmbienceAudio>` +
//! `insert_resource<WaterAudioGrid>`. The system traverses the grid: inactive
//! cells `AudioLoop::stop`, active cells `start_if_not_playing` +
//! `set_position` (the resolved cell world position) + `set_volume`.
//! Resolved (disasm-0x1409ed160/0x1409ed2f0): each cell carries a 64-bit
//! "exists" bits field at +8 whose `popcount` weights the volume
//!   `volume = sqrt( pop / (1 + dist² / (pop * 0.015625 * 42.5)) )`
//! (`0.015625` @ 0x142ac4d00, `42.5` @ 0x142ac4d20, `1.0` @ 0x142925904),
//! and `set_volume` is guarded by a per-cell state byte == 1. The cell's world
//! position is `cell.xz` + a grid-sample offset (vector @ 0x142ac4d10,
//! 待对账). The native `load_index_unchecked` (0x1409ebe80) loads a grid cell.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::{
        camera_rig::MainCamera,
        backend::input::translate_input::DemoModeEnabled,
    },
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

/// `event:/SFX/Water/Amb` (O: rne-read).
pub const WATER_AMB_EVENT: &str = "event:/SFX/Water/Amb";

/// Grid extent (native grid dimensions 待取证; the constant mirrors the
/// documented 64-bit rolling-window cell accounting).
pub const WATER_GRID_EXTENT: usize = 8;

/// Water ambience volume constants (O: disasm-0x1409ed1c1/1ce/1d7):
/// `1.0` @ 0x142925904, `0.015625` @ 0x142ac4d00, `42.5` @ 0x142ac4d20.
pub const WATER_VOLUME_ONE: f32 = 1.0;
pub const WATER_VOLUME_BIT_WEIGHT: f32 = 0.015625;
pub const WATER_VOLUME_ATTENUATION: f32 = 42.5;

/// A water-grid cell (native cell 24 B: world xz @+0, 64-bit "exists" bits @+8
/// (popcount = volume weight), 2×i16 grid samples @+0x10, state byte @+0x18;
/// the water-domain producer fills the bits).
#[derive(Debug, Clone, Copy, Default)]
pub struct WaterGridCell {
    /// Native `+0` world x, z of the cell.
    pub position: [f32; 2],
    /// Native `+8` 64-bit "exists" flag (O: a non-zero field selects active
    /// cells; its popcount weights the volume). The previous water_level>=0.5
    /// threshold was a self-approximation and is dropped.
    pub bits: u64,
    /// Native set_volume guard byte (`== 1` plays the volume; 待接线 producer).
    pub state: u8,
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

/// Resolved water ambience volume (O: disasm-0x1409ed2b8..36d):
/// `sqrt( pop / (1 + dist² / (pop * 0.015625 * 42.5)) )`.
fn water_ambience_volume(pop: f32, dist_sq: f32) -> f32 {
    let attenuation = pop * WATER_VOLUME_BIT_WEIGHT * WATER_VOLUME_ATTENUATION;
    (pop / (WATER_VOLUME_ONE + dist_sq / attenuation)).sqrt()
}

/// `water_ambience_audio_system` (0x1409ecea0): per active cell start the Amb
/// loop at the cell position, stop the rest.
///
/// O (decomp-0x1409ecea0 + disasm-0x1409ed160): the whole body is inside the
/// `_DEV_OVERRIDE_IS_DEMO_MODE == 0` gate; each existing cell (bits != 0) does
/// `start_if_not_playing → set_position(resolved cell xz) → set_volume` (the
/// volume formula above, guarded by the state byte == 1); non-existing cells
/// `AudioLoop::stop`. The cell-position grid-sample offset (0x142ac4d10) and
/// the y-coordinate semantics are 待对账; the horizontal xz distance is used.
pub fn water_ambience_audio_system<S: Studio + Send + Sync + 'static>(
    mut ambience: ResMut<WaterAmbienceAudio<S::Event>>,
    grid: Res<WaterAudioGrid>,
    camera: Res<MainCamera>,
    demo: Option<Res<DemoModeEnabled>>,
) {
    // O: `_DEV_OVERRIDE_IS_DEMO_MODE == 0` gate.
    if demo.is_some_and(|d| d.0) {
        return;
    }
    let eye: [f32; 3] = camera.eye_position().into();
    let count = ambience.loops.len().min(grid.cells.len());
    for index in 0..count {
        let loop_player = &mut ambience.loops[index];
        let cell = grid.load_index_unchecked(index);
        if cell.bits != 0 {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            // O: resolved cell world position (cell.xz + grid-sample offset).
            let _ = current.set_position([cell.position[0], 0.0, cell.position[1]]);
            let dx = cell.position[0] - eye[0];
            let dz = cell.position[1] - eye[2];
            let dist_sq = dx * dx + dz * dz;
            let volume = water_ambience_volume(cell.bits.count_ones() as f32, dist_sq);
            if cell.state == 1 {
                let _ = current.set_volume(volume);
            }
        } else {
            let _ = loop_player.stop_now();
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

    #[test]
    fn water_ambience_volume_matches_the_native_formula() {
        // pop=1, dist=0 → volume = sqrt(1/(1+0)) = 1.0.
        assert!((water_ambience_volume(1.0, 0.0) - 1.0).abs() < 1e-6);
        // The 0.015625/42.5 constants enter only through dist².
        assert!((water_ambience_volume(2.0, 0.0) - 2.0f32.sqrt()).abs() < 1e-6);
    }
}
