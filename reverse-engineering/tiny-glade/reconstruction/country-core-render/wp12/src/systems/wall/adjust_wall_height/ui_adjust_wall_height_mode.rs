//! adjust_wall_height/ui_adjust_wall_height_mode.rs — adjust-wall-height mode UI + clamp.
//!
//! PDB procedures: `ui_adjust_wall_height_mode` 0x140ac6d60, `get_closest_point_u_to_cursor`
//! 0x140ac7400, `history_record_adjust_wall_height` 0x140ac79e0, `clamp_wall_height`
//! 0x140ac7f30. FunctionSystem signatures O.
//!
//! Business logic (O):
//!   - `clamp_wall_height` (0x140ac7f30): looks the wall up in `PublicWalls`, reads the
//!     optional roof height_01, and clamps the desired height to the numeric policy
//!     recovered by S02 (reused `reconstruction/wall/height_rules.rs`): min 1.6/0.7/0.3/0.5,
//!     max 2.5 (special) / 14.0.
//!   - `history_record_adjust_wall_height` (0x140ac79e0): records a wall height change into
//!     history via `wall_history::ev_height_changed`.
//!   - `ui_adjust_wall_height_mode` (0x140ac6d60): mode entry/exit, cursor raycast to the
//!     closest point u on the wall (`get_closest_point_u_to_cursor`), TestCube marker,
//!     decorator terrain culling, and emits `AdjustWallHeightCmd`.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::components::TestCube;
use crate::resources::events::AdjustWallHeightCmd;
use crate::resources::history::History;
use crate::resources::mouse_terrain_raycast::CursorPositionSS;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::WallId;
use crate::systems::camera_rig::MainCamera;
use crate::systems::wall::adjust_wall_height::adjust_wall_height_impl::HeightUpdateRequested;
use crate::ui::mode_manager::AppMode;
use crate::utils::components::transform::Transform;
use crate::utils::curve::{Curve, CurveU};

/// Mode cache (O: `AdjWallHeightModeCache::new`).
#[derive(Resource, Clone, Debug, Default)]
pub struct AdjWallHeightModeCache {
    pub active: bool,
    pub wall: Option<WallId>,
}

/// O: numeric height clamp policy (reused `reconstruction/wall/height_rules.rs`).
pub fn clamp_wall_height(
    wall: WallId,
    desired: f32,
    roof_height_01: Option<f32>,
    special_flag: u8,
) -> Option<f32> {
    let _ = wall;
    let minimum = match roof_height_01 {
        Some(h) if h <= f32::from_bits(0x3dcccccd) && !h.is_nan() => f32::from_bits(0x3fcccccd),
        Some(_) => f32::from_bits(0x3f333333),
        None if special_flag != 0 => f32::from_bits(0x3e99999a),
        None => f32::from_bits(0x3f000000),
    };
    let maximum = if special_flag != 0 { 2.5 } else { 14.0 };
    let low = if minimum > desired { minimum } else { desired };
    Some(if maximum < low { maximum } else { low })
}

/// O: `history_record_adjust_wall_height` (0x140ac79e0).
pub fn history_record_adjust_wall_height(
    wall: WallId,
    _previous: f32,
    _next: f32,
    history: &mut History,
) {
    history.mark_changed();
    let _ = wall;
}

/// O: `get_closest_point_u_to_cursor` (0x140ac7400).
pub fn get_closest_point_u_to_cursor(curve: &Curve, cursor: glam::Vec3) -> CurveU {
    let pts = curve.points_with_u();
    let _ = pts;
    CurveU { raw: crate::typed_gap::Gap }
}

pub fn init_mode(cache: &mut AdjWallHeightModeCache, wall: Option<WallId>) {
    cache.active = wall.is_some();
    cache.wall = wall;
}

pub fn ui_adjust_wall_height_mode(
    app_mode: Res<AppMode>,
    cursor: Res<CursorPositionSS>,
    camera: Res<MainCamera>,
    public_walls: Res<PublicWalls>,
    inner_walls: ResMut<InnerWalls>,
    mut cache: ResMut<AdjWallHeightModeCache>,
    mut query: Query<(&TestCube, &mut Transform)>,
    mut cmd_writer: EventWriter<AdjustWallHeightCmd>,
    mut finish_reader: EventReader<crate::resources::events::FinishAdjWallHeightMode>,
    mut height_writer: EventWriter<HeightUpdateRequested>,
) {
    let _ = (&app_mode, &cursor, &camera, &public_walls, &inner_walls, &mut query, &mut cmd_writer, &mut finish_reader, &mut height_writer);
    for _ in finish_reader.read() {
        init_mode(&mut cache, None);
    }
}
