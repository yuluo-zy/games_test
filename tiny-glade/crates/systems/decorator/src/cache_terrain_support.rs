//! Terrain clutter support in B cache_decorator_transforms, 140816362..140816653.
//! Inputs are the real metadata bounds after orientation and world translation.

use glam::{Quat, Vec2, Vec3};

/// B 140816282..140816338 rotates +X and normalizes its XZ projection.
pub(super) fn terrain_clutter_axis(rotation: Quat) -> Vec2 {
    let right = rotation * Vec3::X;
    Vec2::new(right.x, right.z).normalize()
}

/// Preserve the native edge-center order (+X, -X, +Y, -Y), its second
/// normalization, smooth minimum/mean blend, and maximum-height constraint.
pub(super) fn terrain_clutter_support_height(
    axis: Vec2,
    center: Vec2,
    size: Vec2,
    bottom: f32,
    top: f32,
    mut sample: impl FnMut(Vec2) -> f32,
) -> f32 {
    let reciprocal = 1.0 / (axis.y * axis.y + axis.x * axis.x).sqrt();
    let unit_x = axis.x * reciprocal;
    let dx = (size.x * unit_x) * 0.5;
    let dz = ((axis.y * reciprocal) * size.x) * 0.5;
    let h0 = sample(Vec2::new(center.x + dx, center.y + dz));
    let h1 = sample(Vec2::new(center.x - dx, center.y - dz));
    let dx = ((reciprocal * -axis.y) * size.y) * 0.5;
    let dz = (unit_x * size.y) * 0.5;
    let h2 = sample(Vec2::new(center.x + dx, center.y + dz));
    let h3 = sample(Vec2::new(center.x - dx, center.y - dz));
    // The original min/max masks ignore one NaN and keep the left value on
    // equal inputs (including signed zero); the clamp uses raw SSE comparisons.
    fn minimum(left: f32, right: f32) -> f32 {
        if left.is_nan() || right < left {
            right
        } else {
            left
        }
    }
    fn maximum(left: f32, right: f32) -> f32 {
        if left.is_nan() || right > left {
            right
        } else {
            left
        }
    }
    let average = (((h0 + h1) + h2) + h3) * 0.25;
    let minimum = minimum(minimum(minimum(h0, h1), h2), h3);
    let twice = minimum + minimum;
    let nonnegative = if 0.0 > twice { 0.0 } else { twice };
    let bounded = if 1.0 < nonnegative { 1.0 } else { nonnegative };
    let square = bounded * bounded;
    let smooth = 1.0 - (3.0 - (bounded + bounded)) * square;
    let blend = average * smooth + (1.0 - smooth) * minimum;
    let highest = maximum(maximum(maximum(h0, h1), h2), h3);
    let constraint = (top - bottom) * -0.5 + highest;
    if constraint.is_nan() || blend > constraint {
        blend
    } else {
        constraint
    }
}
