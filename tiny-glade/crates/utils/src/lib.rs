//! Shared geometry, curves, sampling, compact storage and filesystem utilities.
//!
//! Numerical policies and input requirements are documented at their API boundaries.
//! Historical provenance and validation scope are recorded in the package README.

pub mod angle_range;
pub mod bytes;
pub mod camera;
pub mod circular_f32;
pub mod comparative_space;
pub mod components;
pub mod curve;
pub mod easing_functions;
pub mod file;
pub mod float_grid;
pub mod geometry;
pub mod halver;
pub mod hash;
pub mod index_condenser;
pub mod iso_transform;
pub mod local_time;
pub mod loop_range;
pub mod name_hash;
pub mod natural_cubic;
pub mod pack;
pub mod rdp;
pub mod rgb9e5;
pub mod rng;
pub mod sampling;
pub mod scalar;
pub mod selective_deserialize;
pub mod subtractive_1d_boolean;
pub mod swizzlers;
pub mod threshold_range;
pub mod timer;
pub mod wall_space;
pub use geometry::rotation_from_normal;
pub use iso_transform::IsoTransform;
pub use hash::calculate_hash;

/// Jittered normalized boundaries, including exact endpoints 0 and 1.
/// Original B 0x140c90350; `count` counts boundaries and must be at least two.
/// The caller's one pinned fastrand stream is advanced only for internal points.
#[track_caller]
pub fn random_splits(count: usize, jitter: f32, rng: &fastrand::Rng) -> Vec<f32> {
    assert!(count >= 2, "assertion failed: splits >= 2");
    let mut out = Vec::with_capacity(count);
    random_splits_into(count, jitter, rng, &mut out);
    out
}

/// Append the original normalized boundary sequence without clearing `out`.
/// B 0x140c90050 appends [0, 1] for every count below three; this differs from
/// the allocating wrapper's count0/1 panic. Endpoints do not consume the RNG.
#[track_caller]
pub fn random_splits_into(
    count: usize,
    jitter: f32,
    rng: &fastrand::Rng,
    out: &mut Vec<f32>,
) {
    if count < 3 {
        out.extend_from_slice(&[0.0, 1.0]);
        return;
    }
    // Keep both original conversion orders: the cap casts count-1, while
    // the step subtracts 1 after casting count. Do not algebraically merge.
    let cap = (1.0 / ((count - 1) as f32)) * f32::from_bits(0x3efd_70a4);
    let effective_jitter = if jitter <= cap { jitter } else { cap };
    let step = 1.0 / ((count as f32) - 1.0);
    out.push(0.0);
    // Original reserves all internal points before consuming the RNG. A
    // checked capacity failure must not advance the caller's stream.
    out.reserve(count - 2);
    for index in 1..count - 1 {
        out.push(((rng.f32() + (-0.5)) * effective_jitter) + (index as f32) * step);
    }
    out.push(1.0);
}

/// Original B 0x140c90fd0: face the x/z direction with a +X fallback.
/// Reuse pinned glam's normalization and basis-to-quaternion conversion;
/// atan2/from_rotation_y can choose another quaternion sign branch.
pub fn xz_rotation_bad(direction: glam::Vec2) -> glam::Quat {
    let direction = direction.try_normalize().unwrap_or(glam::Vec2::X);
    let forward = glam::Vec3::new(direction.x, 0.0, direction.y);
    let up = glam::Vec3::Y;
    let right = up.cross(forward);
    // 原140c91087..910a4逐项平方后DIVPS；normalize的倒长度乘法会差1ULP。
    let length = (right.y * right.y + (right.z * right.z + right.x * right.x)).sqrt();
    let right = right / length;
    glam::Quat::from_mat3(&glam::Mat3::from_cols(right, up, forward))
}

/// 原140c911d0：忽略输入Y，仅以XZ投影构造本地正Z朝向。
/// 两次规范化保留原除法与求和次序；倒长度无效时选正X。
pub fn xz_projective_rotation_bad(direction: glam::Vec3) -> glam::Quat {
    let reciprocal = 1.0 / (direction.z * direction.z + direction.x * direction.x).sqrt();
    let forward = if reciprocal > 0.0 && reciprocal.is_finite() {
        glam::Vec3::new(direction.x * reciprocal, 0.0, direction.z * reciprocal)
    } else {
        glam::Vec3::X
    };
    let up = glam::Vec3::Y;
    let right = up.cross(forward);
    let length = (right.z * right.z + (right.x * right.x + right.y * right.y)).sqrt();
    let right = right / length;
    glam::Quat::from_mat3(&glam::Mat3::from_cols(right, up, forward))
}

/// 原140c913e0：以本地负Z朝向输入，输入无有效倒长度时先选择正X。
/// 保留原右轴的第二次归一化和整个矩阵转四元数分支，不能先翻转原输入。
pub fn xz_rotation_from_forward(direction: glam::Vec2) -> glam::Quat {
    let direction = direction.try_normalize().unwrap_or(glam::Vec2::X);
    let forward = glam::Vec3::new(direction.x, 0.0, direction.y);
    let up = glam::Vec3::Y;
    let right = up.cross(forward).normalize();
    glam::Quat::from_mat3(&glam::Mat3::from_cols(-right, up, -forward))
}

#[cfg(test)]
mod cloth_shared_utils_tests {
    use super::*;
    use glam::{Vec2, Vec3};

    #[test]
    fn allocating_splits_rejects_zero_and_one_before_rng_draws() {
        for count in [0, 1] {
            let rng = fastrand::Rng::with_seed(0x1234_5678);
            let seed = rng.get_seed();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                random_splits(count, 0.2, &rng)
            }));
            assert!(result.is_err());
            assert_eq!(rng.get_seed(), seed);
        }
    }

    #[test]
    fn into_small_counts_append_to_existing_values_without_rng_draws() {
        for count in [0, 1, 2] {
            let rng = fastrand::Rng::with_seed(0x1234_5678);
            let seed = rng.get_seed();
            let mut out = vec![17.0, -3.0];
            random_splits_into(count, 0.2, &rng, &mut out);
            assert_eq!(out, [17.0, -3.0, 0.0, 1.0]);
            assert_eq!(rng.get_seed(), seed);
        }
        let rng = fastrand::Rng::with_seed(0x1234_5678);
        let seed = rng.get_seed();
        assert_eq!(random_splits(2, f32::NAN, &rng), [0.0, 1.0]);
        assert_eq!(rng.get_seed(), seed);
    }

    #[test]
    fn splits_keep_endpoints_and_exact_internal_draw_count() {
        let rng = fastrand::Rng::with_seed(0x1234_5678);
        let expected_stream = fastrand::Rng::with_seed(0x1234_5678);
        assert_eq!(random_splits(5, 0.0, &rng), [0.0, 0.25, 0.5, 0.75, 1.0]);
        for _ in 0..3 {
            expected_stream.f32();
        }
        assert_eq!(rng.get_seed(), expected_stream.get_seed());
    }

    #[test]
    fn into_capacity_overflow_happens_before_internal_rng_draws() {
        let rng = fastrand::Rng::with_seed(0x1234_5678);
        let seed = rng.get_seed();
        let mut out = vec![17.0];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            random_splits_into(usize::MAX, 0.2, &rng, &mut out);
        }));
        assert!(result.is_err());
        assert_eq!(rng.get_seed(), seed);
        assert_eq!(out, [17.0, 0.0]);
    }

    #[test]
    fn nan_and_positive_infinite_jitter_select_the_original_cap() {
        let count = 7;
        let cap = (1.0 / ((count - 1) as f32)) * f32::from_bits(0x3efd_70a4);
        let capped_rng = fastrand::Rng::with_seed(0x1234_5678);
        let expected = random_splits(count, cap, &capped_rng);
        for jitter in [f32::NAN, f32::INFINITY] {
            let rng = fastrand::Rng::with_seed(0x1234_5678);
            assert_eq!(random_splits(count, jitter, &rng), expected);
            assert_eq!(rng.get_seed(), capped_rng.get_seed());
        }
        let rng = fastrand::Rng::with_seed(0x1234_5678);
        let negative = random_splits(count, -cap, &rng);
        assert_ne!(negative, expected);
        assert_eq!(negative[0], 0.0);
        assert_eq!(negative[count - 1], 1.0);
        assert_eq!(rng.get_seed(), capped_rng.get_seed());
    }

    #[test]
    fn xz_rotation_faces_real_cardinal_and_noncardinal_directions() {
        for direction in [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y, Vec2::new(3.0, -4.0)] {
            let rotation = xz_rotation_bad(direction);
            let normalized = direction.normalize();
            assert!((rotation * Vec3::Z).abs_diff_eq(
                Vec3::new(normalized.x, 0.0, normalized.y), 1e-6));
            assert!((rotation * Vec3::Y).abs_diff_eq(Vec3::Y, 1e-6));
        }
        // Original matrix conversion's Y-major branch for the -X direction.
        let negative_x = xz_rotation_bad(-Vec2::X);
        assert!(negative_x.y > 0.0 && negative_x.w < 0.0);
    }

    #[test]
    fn xz_invalid_length_falls_back_to_positive_x_not_identity() {
        let expected = xz_rotation_bad(Vec2::X);
        for direction in [Vec2::ZERO, Vec2::splat(f32::NAN),
            Vec2::new(f32::INFINITY, 1.0), Vec2::splat(f32::MAX),
            Vec2::splat(f32::MIN_POSITIVE)] {
            assert_eq!(xz_rotation_bad(direction), expected);
        }
        assert!((expected * Vec3::Z).abs_diff_eq(Vec3::X, 1e-6));
    }
}

/// B140c904e0/81B 调用 UCRT expf，按真实时间步长平滑当前值。
/// 保留原 f32 运算顺序，不额外钳制或使用融合乘加。
pub fn time_continuous_lerp(a: f32, b: f32, delta: f32, rate: f32) -> f32 {
    (b - a) * (1.0 - (-delta * rate).exp()) + a
}

/// 原140c90540/40B：先对 delta 翻转符号，再乘 rate，最后从1减 expf。
/// 不钳制负时间步长，也不提前拒绝非有限值。
pub fn time_continuous_lerp_factor(delta: f32, rate: f32) -> f32 {
    1.0 - (-delta * rate).exp()
}

#[cfg(test)]
mod continuous_lerp_factor_tests {
    use super::time_continuous_lerp_factor;

    #[test]
    fn factor_keeps_decay_and_negative_time_without_clamping() {
        assert!((time_continuous_lerp_factor(1.0 / 60.0, 9.0) - 0.139_292_0).abs() < 0.000_001);
        assert!(time_continuous_lerp_factor(-1.0, 1.0) < 0.0);
        assert!(time_continuous_lerp_factor(1.0, -1.0) < 0.0);
        assert_eq!(time_continuous_lerp_factor(0.0, 9.0).to_bits(), 0);
        assert_eq!(time_continuous_lerp_factor(-0.0, 9.0).to_bits(), 0);
    }

    #[test]
    fn factor_keeps_overflow_and_unordered_domains() {
        assert_eq!(time_continuous_lerp_factor(f32::MAX, 9.0), 1.0);
        assert_eq!(time_continuous_lerp_factor(-f32::MAX, 9.0), f32::NEG_INFINITY);
        assert!(time_continuous_lerp_factor(f32::INFINITY, 0.0).is_nan());
        assert!(time_continuous_lerp_factor(f32::from_bits(0x7fc1_2345), 1.0).is_nan());
        assert!(time_continuous_lerp_factor(1.0, f32::NAN).is_nan());
    }
}

#[cfg(test)]
mod continuous_lerp_tests {
    use super::time_continuous_lerp;

    #[test]
    fn actual_delta_exponential_preserves_current_value_and_zero_step() {
        assert_eq!(time_continuous_lerp(0.375, 1.0, 0.0, 3.0), 0.375);
        assert_eq!(time_continuous_lerp(0.375, 1.0, 0.2, 0.0), 0.375);
        let actual = time_continuous_lerp(0.25, 1.0, 0.5, 3.0);
        assert!((actual - 0.832_652_4).abs() < 2.0e-7, "{actual}");
        assert!((time_continuous_lerp(1.0, 0.0, 0.5, 3.0) - 0.223_130_17).abs() < 2.0e-7);
    }

    #[test]
    fn subdivision_and_target_direction_use_the_same_current_state() {
        let whole = time_continuous_lerp(-0.2, 0.9, 0.5, 3.0);
        let first = time_continuous_lerp(-0.2, 0.9, 0.125, 3.0);
        let split = time_continuous_lerp(first, 0.9, 0.375, 3.0);
        assert!((whole - split).abs() < 2.0e-7);
        assert!(time_continuous_lerp(0.5, 1.0, -0.25, 3.0) < 0.5);
    }

    #[test]
    fn invalid_scalar_inputs_are_not_silently_replaced() {
        assert!(time_continuous_lerp(0.0, 1.0, f32::NAN, 3.0).is_nan());
        assert!(time_continuous_lerp(f32::NAN, 1.0, 0.1, 3.0).is_nan());
        assert!(time_continuous_lerp(0.0, 1.0, 0.1, f32::NAN).is_nan());
        assert_eq!(time_continuous_lerp(0.25, 1.0, f32::INFINITY, 3.0), 1.0);
    }
}

#[cfg(test)]
mod forward_rotation_tests {
    use super::xz_rotation_from_forward;
    use glam::{Vec2, Vec3};

    #[test]
    fn original_negative_z_basis_tracks_all_planar_quadrants_and_keeps_up() {
        for direction in [
            Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y,
            Vec2::new(3.0, 4.0), Vec2::new(-3.0, 4.0),
            Vec2::new(3.0, -4.0), Vec2::new(-3.0, -4.0),
            Vec2::new(0.000_01, -0.000_02),
        ] {
            let rotation = xz_rotation_from_forward(direction);
            let expected = direction.normalize();
            assert!((rotation * Vec3::NEG_Z).abs_diff_eq(
                Vec3::new(expected.x, 0.0, expected.y), 0.000_001,
            ));
            assert!((rotation * Vec3::Y).abs_diff_eq(Vec3::Y, 0.000_001));
            assert!((rotation.length_squared() - 1.0).abs() < 0.000_001);
        }
    }

    #[test]
    fn original_fallback_precedes_axis_negation_for_zero_and_nonfinite_inputs() {
        let expected = xz_rotation_from_forward(Vec2::X).to_array().map(f32::to_bits);
        for direction in [
            Vec2::ZERO, Vec2::new(-0.0, 0.0), Vec2::new(0.0, -0.0),
            Vec2::new(f32::NAN, 1.0), Vec2::new(1.0, f32::NAN),
            Vec2::new(f32::INFINITY, 1.0), Vec2::new(1.0, f32::NEG_INFINITY),
            Vec2::splat(f32::MAX), Vec2::splat(f32::MIN_POSITIVE),
        ] {
            assert_eq!(xz_rotation_from_forward(direction).to_array().map(f32::to_bits), expected);
        }
        assert!((xz_rotation_from_forward(Vec2::ZERO) * Vec3::NEG_Z)
            .abs_diff_eq(Vec3::X, 0.000_001));
    }

    #[test]
    fn original_matrix_branch_keeps_the_y_and_w_component_sign_choice() {
        let positive_x = xz_rotation_from_forward(Vec2::X);
        assert!(positive_x.y > 0.0 && positive_x.w < 0.0);
        let negative_x = xz_rotation_from_forward(-Vec2::X);
        assert!(negative_x.y > 0.0 && negative_x.w > 0.0);
        let positive_z = xz_rotation_from_forward(Vec2::Y);
        assert_eq!(positive_z.y, 1.0);
        let negative_z = xz_rotation_from_forward(-Vec2::Y);
        assert_eq!(negative_z.w, 1.0);
    }
}

#[cfg(test)]
mod projective_rotation_tests {
    use super::xz_projective_rotation_bad;
    use glam::Vec3;

    #[test]
    fn projective_rotation_uses_positive_z_and_all_planar_quadrants() {
        for direction in [
            Vec3::X, -Vec3::X, Vec3::Z, -Vec3::Z,
            Vec3::new(3.0, 9.0, 4.0), Vec3::new(-3.0, 9.0, 4.0),
            Vec3::new(3.0, 9.0, -4.0), Vec3::new(-3.0, 9.0, -4.0),
            Vec3::new(1e-18, 9.0, -2e-18), Vec3::new(1e18, 9.0, -2e18),
        ] {
            let rotation = xz_projective_rotation_bad(direction);
            let expected = Vec3::new(direction.x, 0.0, direction.z).normalize();
            assert!((rotation * Vec3::Z - expected).length() < 2e-6, "{direction:?}: {rotation:?}");
            assert!((rotation * Vec3::Y - Vec3::Y).length() < 2e-6);
            assert!((rotation.length_squared() - 1.0).abs() < 2e-6);
        }
    }

    #[test]
    fn projective_rotation_does_not_read_y_even_when_nonfinite() {
        for (x, z) in [(3.0, -4.0), (-3.0, 4.0), (0.0, 0.0), (f32::MAX, 1.0)] {
            let expected = xz_projective_rotation_bad(Vec3::new(x, 0.0, z)).to_array().map(f32::to_bits);
            for y in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MAX] {
                assert_eq!(xz_projective_rotation_bad(Vec3::new(x, y, z)).to_array().map(f32::to_bits), expected);
            }
        }
    }

    #[test]
    fn projective_rotation_falls_back_for_invalid_reciprocal_including_square_range() {
        let expected = xz_projective_rotation_bad(Vec3::X).to_array().map(f32::to_bits);
        for direction in [
            Vec3::ZERO, Vec3::new(-0.0, 0.0, -0.0),
            Vec3::new(f32::NAN, 0.0, 1.0), Vec3::new(1.0, 0.0, f32::NAN),
            Vec3::new(f32::INFINITY, 0.0, 1.0), Vec3::new(1.0, 0.0, f32::NEG_INFINITY),
            Vec3::new(f32::MAX, 0.0, 1.0), Vec3::new(1e-30, 0.0, -1e-30),
            Vec3::new(f32::MIN_POSITIVE, 0.0, 0.0),
        ] {
            assert_eq!(xz_projective_rotation_bad(direction).to_array().map(f32::to_bits), expected, "{direction:?}");
        }
    }

    #[test]
    fn projective_rotation_keeps_original_quaternion_sign_choices() {
        let positive_x = xz_projective_rotation_bad(Vec3::X);
        assert!(positive_x.y > 0.0 && positive_x.w > 0.0);
        let negative_x = xz_projective_rotation_bad(-Vec3::X);
        assert!(negative_x.y > 0.0 && negative_x.w < 0.0);
        assert_eq!(xz_projective_rotation_bad(Vec3::Z).w, 1.0);
        assert_eq!(xz_projective_rotation_bad(-Vec3::Z).y, 1.0);
    }
}
