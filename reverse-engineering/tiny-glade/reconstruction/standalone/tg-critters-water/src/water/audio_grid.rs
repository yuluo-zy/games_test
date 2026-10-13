//! `update_water_audio_grid` (0x141baefe0, source lines 21-106). Evidence: water-extra-analysis 2.
//!
//! Signature: (Local<WaterAudioGridUpdateState>, Res<WaterRaster>, Raycasting, Res<MainCamera>,
//! ResMut<WaterAudioGrid>), run_if Option<Res<WaterAudioGrid>>.

use super::raster::{RasterBufferRef, WorldRaster, WORLD_SCALE};
use crate::env::{DemoMode, MainCamera, RaycastWorld};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

/// 80^2 (0x142eed900 = 6400.0).
pub const AUDIO_RADIUS_SQ: f32 = 6400.0;
/// Normalised sample must be > 0.5 to count as water (0x142925870).
pub const WATER_THRESHOLD: f32 = 0.5;

/// 24 B cell: xz (+0x0), u64 64-frame window (+0x8), 2 x u16 counters (+0x10).
/// UNKNOWN(evidence: water-extra-analysis 2): semantics of the two counters beyond "+1 on set,
/// -1 on clear" (both are changed by the same PADDW/PSUBW pair); the grid's row/column layout.
#[derive(Clone, Debug, PartialEq)]
pub struct WaterAudioGridCell {
    pub xz: Vec2,
    pub water_mask: u64,
    pub counters: [u16; 2],
}

impl WaterAudioGridCell {
    pub fn new(x: f32, z: f32) -> Self {
        Self { xz: Vec2::new(x, z), water_mask: 0, counters: [0; 2] }
    }

    /// Number of set bits in the rolling window (the quantity the counters track).
    pub fn window_hits(&self) -> u32 {
        self.water_mask.count_ones()
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub struct WaterAudioGrid {
    pub cells: Vec<WaterAudioGridCell>,
}

/// `Local<WaterAudioGridUpdateState>`: u64 frame counter.
#[derive(Clone, Copy, Debug, Default)]
pub struct WaterAudioGridUpdateState(pub u64);

/// `uVar12 = n + (n+1)*10 + 1` with n the pre-increment counter; tested bit = `uVar12 & 63`.
pub fn frame_bit(counter_before: u64) -> u32 {
    (counter_before.wrapping_add(counter_before.wrapping_add(1).wrapping_mul(10)).wrapping_add(1) & 63) as u32
}

/// State transition of one cell for one bit. `present` = within radius AND sample > 0.5 AND
/// not occluded. Counters change only on a real bit flip (new set / clear of a set bit).
pub fn apply_bit(cell: &mut WaterAudioGridCell, bit: u32, present: bool) {
    let m = 1u64 << bit;
    let was = cell.water_mask & m != 0;
    if present {
        cell.water_mask |= m;
        if !was {
            cell.counters = cell.counters.map(|c| c.wrapping_add(1));
        }
    } else {
        cell.water_mask &= !m;
        if was {
            cell.counters = cell.counters.map(|c| c.wrapping_sub(1));
        }
    }
}

/// Core of one frame. `occluded(eye, cell_xz)` stands for `Raycasting::intersects_any` (ray camera ->
/// cell centre, flags 0x32a3). Distance metric: UNKNOWN(evidence: water-extra-analysis 2) whether
/// the 6400 test is 2D or 3D; xz distance to the eye is used.
/// Returns `false` (and does nothing) when `demo_mode`.
pub fn update_grid(
    state: &mut WaterAudioGridUpdateState,
    raster: RasterBufferRef,
    eye: Vec3,
    grid: &mut WaterAudioGrid,
    demo_mode: bool,
    occluded: &dyn Fn(Vec3, Vec2) -> bool,
) -> bool {
    if demo_mode {
        return false;
    }
    let bit = frame_bit(state.0);
    state.0 = state.0.wrapping_add(1);
    let eye_xz = Vec2::new(eye.x, eye.z);
    for cell in grid.cells.iter_mut() {
        let present = if cell.xz.distance_squared(eye_xz) >= AUDIO_RADIUS_SQ {
            false
        } else {
            let s = raster.bilinear_sample_at_world_xz(cell.xz.x, cell.xz.y, WORLD_SCALE);
            s > WATER_THRESHOLD && !occluded(eye, cell.xz)
        };
        apply_bit(cell, bit, present);
    }
    true
}

/// `update_water_audio_grid` (0x141baefe0) as a system:
/// `(Local<WaterAudioGridUpdateState>, Res<WaterRaster>, Raycasting, Res<MainCamera>,
/// ResMut<WaterAudioGrid>)`, registered with `run_if Option<Res<WaterAudioGrid>>`
/// (see [`super::plugin::WaterPlugin`]). `Raycasting` is [`RaycastWorld`]; `demo` is
/// `_DEV_OVERRIDE_IS_DEMO_MODE`.
pub fn update_water_audio_grid(
    mut state: Local<WaterAudioGridUpdateState>,
    raster: Res<WorldRaster>,
    raycast: Res<RaycastWorld>,
    camera: Res<MainCamera>,
    mut grid: ResMut<WaterAudioGrid>,
    demo: Res<DemoMode>,
) {
    let occluded = |eye: Vec3, xz: Vec2| raycast.intersects_any(eye, Vec3::new(xz.x, 0.0, xz.y));
    update_grid(&mut state, raster.read_ref(), camera.eye, &mut grid, demo.0, &occluded);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::raster::WorldRaster;

    #[test]
    fn frame_bit_formula() {
        assert_eq!(frame_bit(0), 11);
        assert_eq!(frame_bit(1), 22);
        assert_eq!(frame_bit(2), 33);
        assert!((0..1000).all(|n| frame_bit(n) < 64));
        let _ = frame_bit(u64::MAX);
    }

    #[test]
    fn counters_only_change_on_flip() {
        let mut c = WaterAudioGridCell::new(0.0, 0.0);
        apply_bit(&mut c, 3, true);
        apply_bit(&mut c, 3, true);
        assert_eq!(c.counters, [1, 1]);
        assert_eq!(c.window_hits(), 1);
        apply_bit(&mut c, 3, false);
        apply_bit(&mut c, 3, false);
        assert_eq!(c.counters, [0, 0]);
        assert_eq!(c.water_mask, 0);
        apply_bit(&mut c, 0, false);
        assert_eq!(c.counters, [0, 0]);
    }

    #[test]
    fn counters_wrap_like_paddw() {
        let mut c = WaterAudioGridCell { xz: Vec2::ZERO, water_mask: 1, counters: [0, 0xffff] };
        apply_bit(&mut c, 0, false);
        assert_eq!(c.counters, [0xffff, 0xfffe]);
    }

    fn wet_raster() -> WorldRaster {
        let mut r = WorldRaster::new();
        r.composite.fill(255);
        r
    }

    #[test]
    fn update_marks_near_wet_visible_cells_only() {
        let r = wet_raster();
        let mut grid = WaterAudioGrid {
            cells: vec![
                WaterAudioGridCell::new(10.0, 0.0),  // near, wet
                WaterAudioGridCell::new(100.0, 0.0), // beyond 80
                WaterAudioGridCell::new(-10.0, 0.0), // occluded
            ],
        };
        let mut st = WaterAudioGridUpdateState::default();
        let occ = |_e: Vec3, xz: Vec2| xz.x < 0.0;
        assert!(update_grid(&mut st, r.read_ref(), Vec3::ZERO, &mut grid, false, &occ));
        assert_eq!(grid.cells[0].window_hits(), 1);
        assert_eq!(grid.cells[1].window_hits(), 0);
        assert_eq!(grid.cells[2].window_hits(), 0);
        assert_eq!(st.0, 1);
    }

    #[test]
    fn dry_cell_and_demo_mode() {
        let r = WorldRaster::new();
        let mut grid = WaterAudioGrid { cells: vec![WaterAudioGridCell::new(0.0, 0.0)] };
        let mut st = WaterAudioGridUpdateState::default();
        update_grid(&mut st, r.read_ref(), Vec3::ZERO, &mut grid, false, &|_, _| false);
        assert_eq!(grid.cells[0].window_hits(), 0);
        assert!(!update_grid(&mut st, r.read_ref(), Vec3::ZERO, &mut grid, true, &|_, _| false));
        assert_eq!(st.0, 1, "demo mode early-outs before the counter");
    }

    #[test]
    fn window_fills_over_64_frames_then_decays_when_dried() {
        let mut r = wet_raster();
        let mut grid = WaterAudioGrid { cells: vec![WaterAudioGridCell::new(0.0, 0.0)] };
        let mut st = WaterAudioGridUpdateState::default();
        for _ in 0..64 {
            update_grid(&mut st, r.read_ref(), Vec3::ZERO, &mut grid, false, &|_, _| false);
        }
        // frame_bit is a bijection mod 64 over 64 consecutive frames (10n + n..., step 11 coprime to 64)
        assert_eq!(grid.cells[0].window_hits(), 64);
        assert_eq!(grid.cells[0].counters, [64, 64]);
        r.composite.fill(0);
        for _ in 0..64 {
            update_grid(&mut st, r.read_ref(), Vec3::ZERO, &mut grid, false, &|_, _| false);
        }
        assert_eq!(grid.cells[0].window_hits(), 0);
        assert_eq!(grid.cells[0].counters, [0, 0]);
    }

    #[test]
    fn system_runs_only_when_the_grid_resource_exists_and_fills_over_64_frames() {
        use crate::testing::base_app;
        use bevy_app::Update;
        use bevy_ecs::schedule::common_conditions::resource_exists;
        let mut app = base_app(0.016);
        app.add_systems(Update, update_water_audio_grid.run_if(resource_exists::<WaterAudioGrid>));
        app.world_mut().resource_mut::<WorldRaster>().composite.fill(255);
        app.update(); // no grid resource: the run_if skips the system
        app.insert_resource(WaterAudioGrid {
            cells: vec![WaterAudioGridCell::new(10.0, 0.0), WaterAudioGridCell::new(100.0, 0.0), WaterAudioGridCell::new(-10.0, 0.0)],
        });
        app.world_mut().resource_mut::<RaycastWorld>().spheres.push((Vec3::new(-5.0, 0.0, 0.0), 1.0));
        for _ in 0..64 {
            app.update();
        }
        let g = app.world().resource::<WaterAudioGrid>();
        assert_eq!(g.cells[0].window_hits(), 64);
        assert_eq!(g.cells[0].counters, [64, 64]);
        assert_eq!(g.cells[1].window_hits(), 0, "beyond 80 units");
        assert_eq!(g.cells[2].window_hits(), 0, "occluded by the sphere between eye and cell");
    }

    #[test]
    fn system_demo_mode_early_out() {
        use crate::testing::base_app;
        use bevy_app::Update;
        let mut app = base_app(0.016);
        app.add_systems(Update, update_water_audio_grid);
        app.insert_resource(DemoMode(true)).insert_resource(WaterAudioGrid { cells: vec![WaterAudioGridCell::new(1.0, 0.0)] });
        app.world_mut().resource_mut::<WorldRaster>().composite.fill(255);
        app.update();
        assert_eq!(app.world().resource::<WaterAudioGrid>().cells[0].window_hits(), 0);
    }
}
