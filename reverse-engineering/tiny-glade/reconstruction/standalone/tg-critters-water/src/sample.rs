//! Sampling helpers named after the game's own `utils` (critters-behavior-analysis 时序/RNG):
//! `uniform_sample_disk` 0x140c90700, `uniform_sample_circle` 0x140c90770, `golden_spiral` 0x140c90940,
//! `uniform_sample_cone`, `uniform_sample_sphere`.
//!
//! UNKNOWN(evidence: critters-behavior-analysis 时序/RNG): the exact formulas were not decompiled
//! (callers feed them `perchance` / `fastrand` uniforms).
//! These are the textbook uniform samplers with the same names/arity; callers only depend on
//! the distribution shape (disk: area-uniform, circle: on the rim, ...).

use glam::{Vec2, Vec3};
use std::f32::consts::TAU;

/// Area-uniform point in the unit disk from two uniforms.
pub fn uniform_sample_disk(u1: f32, u2: f32) -> Vec2 {
    let r = u1.sqrt();
    let a = u2 * TAU;
    Vec2::new(r * a.cos(), r * a.sin())
}

/// Point on the unit circle from one uniform.
pub fn uniform_sample_circle(u: f32) -> Vec2 {
    let a = u * TAU;
    Vec2::new(a.cos(), a.sin())
}

/// Direction uniform on the unit sphere.
pub fn uniform_sample_sphere(u1: f32, u2: f32) -> Vec3 {
    let z = 1.0 - 2.0 * u1;
    let r = (1.0 - z * z).max(0.0).sqrt();
    let a = u2 * TAU;
    Vec3::new(r * a.cos(), z, r * a.sin())
}

/// Direction uniformly distributed in a cone around `axis` (half-angle in radians).
pub fn uniform_sample_cone(u1: f32, u2: f32, axis: Vec3, half_angle: f32) -> Vec3 {
    let axis = axis.normalize_or_zero();
    if axis == Vec3::ZERO {
        return Vec3::ZERO;
    }
    let cos_max = half_angle.cos();
    let cos_t = 1.0 - u1 * (1.0 - cos_max);
    let sin_t = (1.0 - cos_t * cos_t).max(0.0).sqrt();
    let a = u2 * TAU;
    let local = Vec3::new(sin_t * a.cos(), sin_t * a.sin(), cos_t);
    glam::Quat::from_rotation_arc(Vec3::Z, axis) * local
}

/// `index`-th of `count` points on a golden spiral inside the unit disk, rotated by `phase` turns.
/// UNKNOWN(evidence: critters-behavior-analysis 1.1): argument list of the original.
pub fn golden_spiral(index: u32, count: u32, phase: f32) -> Vec2 {
    const GOLDEN_ANGLE: f32 = 2.399_963_2;
    let r = ((index as f32 + 0.5) / count.max(1) as f32).sqrt();
    let a = index as f32 * GOLDEN_ANGLE + phase * TAU;
    Vec2::new(r * a.cos(), r * a.sin())
}

/// `x0y`: Vec2 -> Vec3(x, 0, y) (game swizzle names, 0x...; maps planar xy to world xz).
pub fn x0y(v: Vec2) -> Vec3 {
    Vec3::new(v.x, 0.0, v.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::PerchanceRes;

    #[test]
    fn disk_inside_unit() {
        let mut r = PerchanceRes::seeded(1);
        for _ in 0..500 {
            assert!(uniform_sample_disk(r.uniform_f32(), r.uniform_f32()).length() <= 1.0 + 1e-5);
        }
    }

    #[test]
    fn circle_on_rim_and_sphere_unit() {
        assert!((uniform_sample_circle(0.3).length() - 1.0).abs() < 1e-5);
        assert!((uniform_sample_sphere(0.3, 0.6).length() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn cone_within_angle() {
        let axis = Vec3::new(1.0, 2.0, -0.5).normalize();
        let mut r = PerchanceRes::seeded(9);
        for _ in 0..200 {
            let d = uniform_sample_cone(r.uniform_f32(), r.uniform_f32(), axis, 0.5);
            assert!((d.length() - 1.0).abs() < 1e-4);
            assert!(d.dot(axis) >= 0.5_f32.cos() - 1e-4);
        }
        assert_eq!(uniform_sample_cone(0.1, 0.1, Vec3::ZERO, 0.5), Vec3::ZERO);
    }

    #[test]
    fn golden_spiral_in_disk_and_x0y() {
        for i in 0..3 {
            assert!(golden_spiral(i, 3, 0.2).length() <= 1.0);
        }
        assert_eq!(x0y(Vec2::new(1.0, 2.0)), Vec3::new(1.0, 0.0, 2.0));
    }
}
