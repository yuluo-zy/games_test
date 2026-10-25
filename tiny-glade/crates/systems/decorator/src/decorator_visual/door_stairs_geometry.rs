//! Complete B check_for_door_stairs geometry, 1407f3340..1407f3873.
//!
//! Sampling callbacks are an adapter over the actual terrain/raster owners in
//! gothic_balcony_door. They preserve lazy evaluation and world coordinates;
//! this module does not implement either shared sampling algorithm.
use glam::{Quat, Vec2, Vec3};

/// Original eight-float payload for DoorStairsAssemblyParams, in native order.
pub fn sample_door_stairs(
    rotation: Quat,
    position: Vec3,
    width: f32,
    bottom: f32,
    top: f32,
    mut terrain_height: impl FnMut(Vec2) -> f32,
    mut path_coverage: impl FnMut(Vec2) -> f32,
) -> Option<[f32; 8]> {
    let origin = Vec2::new(position.x, position.z);
    let base = terrain_height(origin).max(0.0);
    let coverage = path_coverage(origin);
    let sill = (top - bottom) * -0.5 + position.y;
    // Original ucomiss/jbe also takes the non-positive branch for NaN.
    let low_sill = if coverage > 0.0 {
        let low_sill = 0.1 >= sill;
        if !low_sill && !(base > sill + -0.2) {
            return None;
        }
        low_sill
    } else {
        if !(base > sill + -0.2) {
            return None;
        }
        false
    };

    let forward = rotation * Vec3::new(0.0, 0.0, 1.06);
    let center = Vec2::new(position.x + forward.x, position.z + forward.z);
    let center_height = terrain_height(center);
    if !(base - center_height > 0.1) {
        return None;
    }
    // Preserve the original zero-times-width lanes for non-finite widths.
    let zero = (0.0 * width) * 0.5;
    let side = rotation * Vec3::new(width * 0.5, zero, zero);
    let side = Vec2::new(side.x, side.z);
    let left_height = terrain_height(center - side);
    let right_height = terrain_height(center + side);
    let best = center_height.min(right_height.min(left_height));
    // The path-covered, low-sill branch deliberately bypasses this rejection.
    // Unordered differences are rejected in the ordinary branch (setbe).
    if !low_sill && !(1.5 > base - best) {
        return None;
    }
    let forward = rotation * Vec3::Z;
    let direction = Vec2::new(forward.x, forward.z).normalize();
    Some([
        origin.x,
        origin.y,
        direction.x,
        direction.y,
        base,
        if low_sill { -0.8 } else { best },
        (-0.8 - best).max(0.0),
        width,
    ])
}
