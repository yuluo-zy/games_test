use glam::{Vec2, Vec3};
use utils::{
    curve::{Curve2, Curve3, CurveInternalCoord},
    easing_functions::EasingFunction,
    geometry::Circle2d,
};

#[test]
fn repeated_and_subnormal_point_tangents_remain_finite() {
    let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::ZERO, Vec2::X], false).unwrap();
    for u in [-2.0, 0.0] {
        assert_eq!(
            curve.get_tangent_at_u(u).to_array().map(f32::to_bits),
            [0; 2]
        );
    }
    assert_eq!(curve.get_tangent_at_u(0.5), Vec2::X);
    let curve = Curve3::try_new(vec![Vec3::ZERO, Vec3::ZERO, Vec3::X], false).unwrap();
    assert_eq!(
        curve
            .get_tangent_at_coord(CurveInternalCoord { segment: 0, t: 0.5 })
            .to_array()
            .map(f32::to_bits),
        [0; 3]
    );
    // The zero check must not classify a nonzero subnormal displacement as zero.
    let curve = Curve2::try_new(
        vec![Vec2::ZERO, Vec2::new(f32::from_bits(1), 0.0), Vec2::X],
        false,
    )
    .unwrap();
    assert_eq!(curve.get_tangent_at_u(0.0), Vec2::X);
    let curve = Curve3::try_new(
        vec![Vec3::ZERO, Vec3::new(0.0, f32::from_bits(1), 0.0), Vec3::X],
        false,
    )
    .unwrap();
    assert_eq!(curve.get_tangent_at_u(0.0), Vec3::Y);
}

#[test]
fn circle_parameter_tracks_independent_double_precision_reference() {
    for rotation in [-9.0, -2.7, 0.0, 0.7, 8.0] {
        let circle = Circle2d::new_rotated(Vec2::new(-4.0, 3.0), 2.0, rotation);
        assert_eq!(circle.get_closest_u_from_pos(circle.center), 0.0);
        for index in 0..256 {
            let angle = index as f64 * std::f64::consts::TAU / 256.0;
            let point = circle.center + Vec2::new(angle.cos() as f32, angle.sin() as f32) * 11.0;
            let delta = point - circle.center;
            let expected = (1.0
                - ((delta.y as f64).atan2(delta.x as f64)
                    - rotation.rem_euclid(std::f32::consts::TAU) as f64)
                    / std::f32::consts::TAU as f64)
                .rem_euclid(1.0);
            let actual = circle.get_closest_u_from_pos(point) as f64;
            let error = (actual - expected).abs();
            let circular_error = error.min((1.0 - error).abs());
            assert!(
                circular_error <= 2.0 * f32::EPSILON as f64,
                "rotation={rotation}, index={index}, error={circular_error}"
            );
        }
    }
}

#[test]
fn elastic_rounding_boundary_has_independent_math_reference() {
    let t = f32::from_bits(1044372184);
    let scaled = 20.0 * t;
    let phase = (scaled - 11.125) * f32::from_bits(0x3fb2_b8c3);
    // Independently checked at 80 decimal digits, rounded once for each f32 call.
    let wave = (phase as f64).sin() as f32;
    let power = ((scaled - 10.0) as f64).exp2() as f32;
    assert_eq!(wave.to_bits(), 0x3f44_c1da);
    assert_eq!(power.to_bits(), 0x3c56_d8cf);
    let correctly_rounded = (wave * power) * -0.5;
    assert_eq!(correctly_rounded.to_bits(), 3148161199);
    let actual = EasingFunction::ElasticInOut.apply(t);
    assert!(actual.to_bits().abs_diff(correctly_rounded.to_bits()) <= 1);
    assert_eq!(EasingFunction::ElasticInOut.apply(0.0), 0.0);
    assert_eq!(EasingFunction::ElasticInOut.apply(1.0), 1.0);
}
