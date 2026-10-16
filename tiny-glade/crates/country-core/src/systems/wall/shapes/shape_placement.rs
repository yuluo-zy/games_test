//! B `systems/wall/shapes/shape_placement.rs`: shape placement parameters.
//!
//! Evidence: `chapters/systems/wall/shapes/shape_placement.rs.md` (O,
//! 0x14097f970 / 0x140980db0). The cursor terrain position becomes an
//! unsnapped placement or, when a wall lies within 1.5 units, a snapped one
//! carrying the wall normal/target. `snap_to_roof_ridge` (0x140980db0) is the
//! roof-ridge branch; its full Roof::pivot_3d / RoofRidgeDims::calc derivation
//! is 待取证 (roof ridge geometry, host wiring).

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::{ShapePlacement, SelectedWallShape};
use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
    walls::public_walls::PublicWalls,
};
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// Snap threshold (0x3fc00000 @0x1429258d0).
const SNAP_DISTANCE: f32 = 1.5;
/// Spatial-query half extent for shape placement probing.
const QUERY_HALF_EXTENT: f32 = 8.0;

impl ShapePlacement {
    /// Inline B line 70: unsnapped placement at the cursor terrain position.
    /// The original also seeds a 0.3 scalar whose role is 待取证.
    pub fn new_unsnapped(position: Vec2, height: f32) -> Self {
        Self {
            mode: 0,
            snapped: false,
            position,
            target: glam::Vec3::new(position.x, height, position.y),
            normal: glam::Vec3::Y,
        }
    }
}

/// B 0x14097f970 (14-param system). Cursor terrain position drives a snapped
/// placement when a wall is within `SNAP_DISTANCE`; otherwise unsnapped.
pub fn shape_placement_params(
    _selected: Res<SelectedWallShape>,
    cursor: Res<CursorTerrainRaycast>,
    public: Res<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut placement: ResMut<ShapePlacement>,
) {
    if cursor.tag == 0 {
        return;
    }
    let position = Vec2::new(cursor.pos.x, cursor.pos.z);
    let height = terrain.sample_at_world_xz(position.x, position.y);
    let candidates = spatial.query_with_aabb_clamped(
        [position.x - QUERY_HALF_EXTENT, position.y - QUERY_HALF_EXTENT],
        [position.x + QUERY_HALF_EXTENT, position.y + QUERY_HALF_EXTENT],
    );
    let mut snap_wall = None;
    let mut best = SNAP_DISTANCE * SNAP_DISTANCE;
    for (candidate, wall) in candidates {
        let distance_sq =
            (candidate[0] - position.x).powi(2) + (candidate[1] - position.y).powi(2);
        if distance_sq < best {
            best = distance_sq;
            snap_wall = Some(wall);
        }
    }
    let Some(wall) = snap_wall else {
        *placement = ShapePlacement::new_unsnapped(position, height);
        return;
    };
    let Some(state) = public.get(wall) else {
        *placement = ShapePlacement::new_unsnapped(position, height);
        return;
    };
    // Normal at the snapped point: premade shapes use their shape normal,
    // freehand uses the curve tangent normal.
    let normal = crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(
        state.shape(),
    )
    .get_shape()
    .map(|shape| {
        let u = shape.get_closest_u_from_pos(position);
        let n = shape.get_normal_at_u(u);
        glam::Vec3::new(n.x, 0.0, n.y)
    })
    .unwrap_or_else(|| {
        let curve = state.simplified_curve_2d();
        let u = curve_u_at(&curve.points, position);
        let tangent = curve_tangent(&curve.points, u);
        glam::Vec3::new(-tangent[1], 0.0, tangent[0])
    });
    *placement = ShapePlacement {
        mode: 2,
        snapped: true,
        position,
        target: glam::Vec3::new(position.x, height, position.y),
        normal,
    };
}

/// B 0x140980db0: roof-ridge snap target. The original walks the roof's
/// `Roof::pivot_3d` / `RoofRidgeDims::calc` ridge geometry; the full ridge
/// derivation is roof-domain 待取证. This documented approximation returns the
/// snapped 2D position (or None when no roof ridge applies).
pub fn snap_to_roof_ridge(
    _wall: crate::resources::walls::WallId,
    position: Vec2,
    height: f32,
) -> Option<glam::Vec3> {
    // 待取证: Roof::pivot_3d / RoofRidgeDims::calc ridge walk.
    Some(glam::Vec3::new(position.x, height, position.y))
}

/// Approximate arc-length parameter of the closest point (CurveU projection).
fn curve_u_at(points: &[[f32; 2]], position: Vec2) -> f32 {
    if points.len() < 2 {
        return 0.0;
    }
    let mut best = 0.0f32;
    let mut best_d = f32::MAX;
    let mut length = 0.0f32;
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let seg = glam::Vec2::new(b[0] - a[0], b[1] - a[1]);
        let len2 = seg.length_squared().max(1e-20);
        let t = ((position - glam::Vec2::new(a[0], a[1])).dot(seg) / len2).clamp(0.0, 1.0);
        let closest = glam::Vec2::new(a[0], a[1]) + seg * t;
        let d = (position - closest).length_squared();
        if d < best_d {
            best_d = d;
            best = (length + seg.length() * t) / total_length(points);
        }
        length += seg.length();
    }
    best
}

fn total_length(points: &[[f32; 2]]) -> f32 {
    let mut length = 0.0;
    for pair in points.windows(2) {
        length += (glam::Vec2::new(pair[1][0] - pair[0][0], pair[1][1] - pair[0][1])).length();
    }
    if length > 0.0 { length } else { 1.0 }
}

fn curve_tangent(points: &[[f32; 2]], u: f32) -> [f32; 2] {
    let segment = ((points.len() - 1) as f32 * u).floor().min((points.len() - 2) as f32) as usize;
    let a = points[segment];
    let b = points[segment + 1];
    let dx = b[0] - a[0];
    let dz = b[1] - a[1];
    let inv = 1.0 / (dx * dx + dz * dz).sqrt();
    [dx * inv, dz * inv]
}
