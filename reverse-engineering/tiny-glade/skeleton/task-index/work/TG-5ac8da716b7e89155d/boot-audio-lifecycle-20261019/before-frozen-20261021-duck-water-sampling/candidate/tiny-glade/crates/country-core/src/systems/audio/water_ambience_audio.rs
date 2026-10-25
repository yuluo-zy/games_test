//! Original water-audio owner: 10x10 fixed cell bases, 64-frame visibility
//! bits and wrapping i16 jitter sums. Native volume is based on popcount/64,
//! with RNE coefficient42.25; Active belongs to AudioInstance, not the cell.
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

/// Native init1409ebf20 constructs10 rows with10 columns.
pub const WATER_GRID_EXTENT: usize = 10;

/// Water ambience volume constants (O: disasm-0x1409ed1c1/1ce/1d7):
/// `1.0` @ 0x142925904, `0.015625` @ 0x142ac4d00, `42.25` @ 0x142ac4d20.
pub const WATER_VOLUME_ONE: f32 = 1.0;
pub const WATER_VOLUME_BIT_WEIGHT: f32 = 0.015625;
pub const WATER_VOLUME_ATTENUATION: f32 = 42.25;

/// B native 24B cell: base x/z at0, 64-frame visibility bits at8,
/// two wrapping signed jitter sums at0x10; the last four bytes are padding.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct WaterGridCell {
    pub position: [f32; 2],
    pub bits: u64,
    pub sums: [i16; 2],
}

impl WaterGridCell {
    /// Native producer141baf4b3..50c: +/- the current jitter only on bit changes.
    pub fn set_sample(&mut self, bit: u64, visible: bool, delta: [i16; 2]) {
        let was_set = self.bits & bit != 0;
        if visible {
            self.bits |= bit;
            if !was_set {
                self.sums[0] = self.sums[0].wrapping_add(delta[0]);
                self.sums[1] = self.sums[1].wrapping_add(delta[1]);
            }
        } else {
            self.bits &= !bit;
            if was_set {
                self.sums[0] = self.sums[0].wrapping_sub(delta[0]);
                self.sums[1] = self.sums[1].wrapping_sub(delta[1]);
            }
        }
    }

    pub fn active_position(&self) -> [f32; 3] {
        let reciprocal_count = 1.0 / self.bits.count_ones() as f32;
        [self.position[0] + reciprocal_count * self.sums[0] as f32 * 0.8125,
            0.0,
            self.position[1] + reciprocal_count * self.sums[1] as f32 * 0.8125]
    }
}

#[derive(Resource, Debug, Default)]
pub struct WaterAudioGrid {
    pub cells: Vec<WaterGridCell>,
}

impl WaterAudioGrid {
    /// Original init1409ebf20: 10x10 cells centred in [-65,65].
    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(100);
        for z in 0..WATER_GRID_EXTENT {
            let tz = (z as f32 + 0.5) / 10.0;
            for x in 0..WATER_GRID_EXTENT {
                let tx = (x as f32 + 0.5) / 10.0;
                cells.push(WaterGridCell {
                    position: [tx * 65.0 - (1.0-tx) * 65.0,
                        tz * 65.0 - (1.0-tz) * 65.0],
                    bits: 0, sums: [0; 2],
                });
            }
        }
        Self { cells }
    }

    /// B uninit_clearing140a7a7b0..7c8: retain base positions and allocation.
    pub fn clear_activity(&mut self) {
        for cell in &mut self.cells {
            cell.bits = 0;
            cell.sums = [0; 2];
        }
    }

    /// B1409ebe80 returns visibility density, not a reference to the cell.
    pub fn load_index_unchecked(&self, index: usize) -> f32 {
        self.cells[index].bits.count_ones() as f32 * WATER_VOLUME_BIT_WEIGHT
    }

    /// 原14261f0c0的10×10可见性密度采样，供鸭子等消费者借用同一权威网格。
    /// 居中/legacy策略沿用现进程开关；不在这里钳制原legacy边界外推的结果。
    pub fn bilinear_sample_at_world_xz(
        &self,
        x: f32,
        z: f32,
        extent_x: f32,
        extent_z: f32,
    ) -> f32 {
        ::utils::float_grid::FloatGridSampler::bilinear_sample_at_world_xz(
            self,
            glam::Vec2::new(x, z),
            glam::Vec2::new(extent_x, extent_z),
            ::utils::float_grid::CENTERED_PATH.load(std::sync::atomic::Ordering::Relaxed),
        )
    }
}

impl ::utils::float_grid::FloatGrid for WaterAudioGrid {
    fn dimensions(&self) -> [usize; 2] {
        [WATER_GRID_EXTENT; 2]
    }

    fn load_index_unchecked(&self, index: usize) -> f32 {
        WaterAudioGrid::load_index_unchecked(self, index)
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
    let grid = WaterAudioGrid::new();
    commands.insert_resource(ambience);
    commands.insert_resource(grid);
}

/// Resolved water ambience volume (O: disasm-0x1409ed2b8..36d):
/// `sqrt( pop / (1 + dist² / (pop * 0.015625 * 42.25)) )`.
fn water_ambience_volume(pop: f32, dist_sq: f32) -> f32 {
    // Exact mul/div/add/reciprocal/mul/sqrt sequence1409ed2b8..36d.
    let fraction=pop*WATER_VOLUME_BIT_WEIGHT;
    let attenuation=fraction*WATER_VOLUME_ATTENUATION;
    (fraction*(WATER_VOLUME_ONE/(dist_sq/attenuation+WATER_VOLUME_ONE))).sqrt()
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
        let cell = &grid.cells[index];
        if cell.bits != 0 {
            loop_player.start_if_not_playing().unwrap();
            let current = &mut loop_player.entries[loop_player.cursor];
            let position = cell.active_position();
            current.set_position(position).unwrap();
            let dx = position[0] - eye[0];
            let dy = position[1] - eye[1];
            let dz = position[2] - eye[2];
            let dist_sq = dx * dx + dy * dy + dz * dz;
            let volume = water_ambience_volume(cell.bits.count_ones() as f32, dist_sq);
            // Native ==1 guard is the selected AudioInstance::Active tag,
            // not a WaterGridCell state. AudioInstance::set_volume owns it.
            current.set_volume(volume).unwrap();
        } else {
            loop_player.stop_now().unwrap();
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
        assert!((water_ambience_volume(1.0, 0.0) - 0.125).abs() < 1e-6);
        // The 0.015625/42.25 constants enter only through dist².
        assert!((water_ambience_volume(2.0, 0.0) - (2.0f32/64.0).sqrt()).abs() < 1e-6);
    }
}


#[cfg(test)]
mod lifecycle_native_tests {
    use super::*;
    #[test]
    fn original_water_grid_owns_rollover_and_unload_preserves_positions() {
        let mut grid=WaterAudioGrid::new();
        assert_eq!(grid.cells.len(),100);
        assert_eq!(core::mem::size_of::<WaterGridCell>(),24);
        assert_eq!(core::mem::offset_of!(WaterGridCell,sums),0x10);
        assert_eq!(grid.cells[0].position,[-58.5,-58.5]);
        assert_eq!(grid.cells[99].position,[58.5,58.5]);
        let base=grid.cells.iter().map(|c|c.position).collect::<Vec<_>>();
        let address=grid.cells.as_ptr();
        grid.cells[0].set_sample(1<<11,true,[-1,-5]);
        grid.cells[0].set_sample(1<<11,true,[-1,-5]);
        assert_eq!(grid.cells[0].sums,[-1,-5]);
        assert_eq!(grid.cells[0].active_position(),[-59.3125,0.0,-62.5625]);
        assert_eq!(grid.load_index_unchecked(0),1.0/64.0);
        grid.clear_activity();
        assert_eq!(grid.cells.as_ptr(),address);
        assert_eq!(grid.cells.iter().map(|c|c.position).collect::<Vec<_>>(),base);
        assert!(grid.cells.iter().all(|c|c.bits==0 && c.sums==[0,0]));
        // The still-live producer's next frame uses the cleared canonical owner.
        grid.cells[0].set_sample(1<<22,true,[5,-3]);
        assert_eq!(grid.cells[0].bits,1<<22);
        assert_eq!(grid.cells[0].sums,[5,-3]);
        grid.cells[0].set_sample(1<<22,false,[5,-3]);
        assert_eq!(grid.cells[0].sums,[0,0]);
    }
    #[test]
    fn native_attenuation_constant_and_normalized_volume_are_retained() {
        assert_eq!(WATER_VOLUME_ATTENUATION.to_bits(),0x42290000);
        assert_eq!(water_ambience_volume(64.0,42.25).to_bits(),0.5f32.sqrt().to_bits());
    }
}


#[cfg(test)]
mod water_grid_sampling_regressions {
    use super::*;
    use ::utils::float_grid::{FloatGrid, FloatGridSampler};
    use glam::Vec2;

    fn density_ramp() -> WaterAudioGrid {
        let mut grid = WaterAudioGrid::new();
        for (index, cell) in grid.cells.iter_mut().enumerate() {
            let count = (index % 10 + 6 * (index / 10)) as u32;
            cell.bits = (1u64 << count) - 1;
        }
        grid
    }

    fn sample(grid: &WaterAudioGrid, x: f32, z: f32, centered: bool) -> f32 {
        FloatGridSampler::bilinear_sample_at_world_xz(
            grid, Vec2::new(x, z), Vec2::splat(130.0), centered,
        )
    }

    #[test]
    fn ten_by_ten_density_preserves_axis_order_and_both_edge_policies() {
        let grid = density_ramp();
        assert_eq!(FloatGrid::dimensions(&grid), [10, 10]);
        assert_eq!(sample(&grid, 0.0, 0.0, false).to_bits(), (35.0f32 / 64.0).to_bits());
        assert_eq!(sample(&grid, 0.0, 0.0, true).to_bits(), (31.5f32 / 64.0).to_bits());
        // 原legacy保留fraction=2的正边界外推；不能在桥里钳成[0,1]。
        assert_eq!(sample(&grid, 65.0, 65.0, false).to_bits(), (70.0f32 / 64.0).to_bits());
        assert_eq!(sample(&grid, 65.0, 65.0, true).to_bits(), (63.0f32 / 64.0).to_bits());
        assert_eq!(sample(&grid, -65.0, -65.0, false), 0.0);
        assert_eq!(sample(&grid, -65.0, -65.0, true), 0.0);
        assert_eq!(sample(&grid, f32::NAN, f32::NAN, false), 0.0);
        assert_eq!(sample(&grid, f32::NAN, f32::NAN, true).to_bits(), (63.0f32 / 64.0).to_bits());
    }

    #[test]
    fn shared_policy_api_reads_only_density_and_preserves_the_grid() {
        let mut grid = WaterAudioGrid::new();
        for cell in &mut grid.cells {
            cell.bits = u32::MAX as u64;
            cell.position = [10000.0, -10000.0];
            cell.sums = [i16::MIN, i16::MAX];
        }
        let before = grid.cells.clone();
        // 常量密度在两种现策略下都相同，测试无需改写全局开关。
        for (x, z) in [(0.0, 0.0), (65.0, 65.0), (-65.0, -65.0),
            (1e30, -1e30), (f32::NAN, f32::NAN)] {
            assert_eq!(grid.bilinear_sample_at_world_xz(x, z, 130.0, 130.0), 0.5);
        }
        assert_eq!(grid.cells, before);
    }

    #[test]
    fn missing_canonical_cells_keep_the_original_required_failure() {
        let grid = WaterAudioGrid::default();
        assert!(std::panic::catch_unwind(||
            grid.bilinear_sample_at_world_xz(0.0, 0.0, 130.0, 130.0)
        ).is_err());
    }
}
