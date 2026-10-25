use glam::{Vec2, Vec3};
use serde::Deserialize;
use utils::{
    curve::{
        Curve2, Curve3XZ, CurveCoordSemantic2D, CurveError, CurveInternalCoord, CurveU,
        curve_slice::CurveSlice, curve_u_circular::CurveUCircular,
    },
    natural_cubic::{Spline2D, Spline3D, SplineError},
    rdp::Rdp,
};

#[test]
fn failed_curve_edits_preserve_points_and_parameter_cache() {
    let mut curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::X], false).unwrap();
    for suffix in [vec![Vec2::X], vec![Vec2::splat(f32::NAN)]] {
        let before = curve.clone();
        assert!(curve.try_extend(&suffix).is_err());
        assert_eq!(curve.points, before.points);
        assert_eq!(curve.points_u, before.points_u);
        assert_eq!(curve.length, before.length);
        assert_eq!(curve.get_pos_at_u(0.5), Vec2::X * 0.5);
    }
    curve.try_extend(&[Vec2::new(2.0, 0.0)]).unwrap();
    assert_eq!(curve.points_u, [0.0, 0.5, 1.0]);

    let mut closed = Curve2::try_new(vec![Vec2::ZERO, Vec2::X, Vec2::ZERO], false).unwrap();
    let before = closed.clone();
    assert_eq!(
        closed.smooth_in_place(1, true),
        Err(CurveError::NonFiniteLength)
    );
    assert_eq!(closed.points, before.points);
    assert_eq!(closed.points_u, before.points_u);
    assert_eq!(closed.length, before.length);
}

#[test]
fn resampling_rejects_invalid_or_unrepresentable_requests() {
    let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::X], false).unwrap();
    for spacing in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            curve.try_resample(spacing, false).unwrap_err(),
            CurveError::InvalidSpacing
        );
        assert_eq!(
            curve.try_resample_per_segment(spacing).unwrap_err(),
            CurveError::InvalidSpacing
        );
    }
    assert_eq!(
        curve.try_resample(f32::MIN_POSITIVE, false).unwrap_err(),
        CurveError::InvalidSampleCount
    );
    for count in [0, 1, (1 << f32::MANTISSA_DIGITS) + 1, usize::MAX] {
        assert_eq!(
            curve
                .try_resample_approx_pt_count(count, false)
                .unwrap_err(),
            CurveError::InvalidSampleCount
        );
    }
}

#[test]
fn resampling_preserves_endpoints_and_remaps_segment_midpoints() {
    let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::X, Vec2::new(1.0, 1.0)], false).unwrap();
    for result in [
        curve.try_resample(0.2, true),
        curve.try_resample_per_segment(0.2),
        curve.try_resample_approx_pt_count(7, true),
    ] {
        let sampled = result.unwrap();
        assert_eq!(sampled.points.first(), curve.points.first());
        assert_eq!(sampled.points.last(), curve.points.last());
        assert!(sampled.points_u.windows(2).all(|pair| pair[0] < pair[1]));
    }
    let sampled = curve
        .try_resample_remap_per_segment_2::<Vec3, CurveCoordSemantic2D>(
            10.0,
            true,
            |segment, t, point| Vec3::new(point.x, segment as f32 + t, point.y),
        )
        .unwrap();
    assert_eq!(sampled.points.len(), 5);
    assert_eq!(sampled.points[1], Vec3::new(0.5, 0.5, 0.0));
    assert_eq!(sampled.points[3], Vec3::new(1.0, 1.5, 0.5));
}

#[test]
fn zero_length_slice_edges_follow_original_right_and_unchanged_left() {
    let curve = Curve2::try_new(vec![Vec2::ZERO, Vec2::ZERO, Vec2::X, Vec2::X], false).unwrap();
    assert_eq!(curve.get_tangent_at_u(0.0), Vec2::ZERO);
    assert_eq!(curve.get_tangent_at_u(1.0), Vec2::ZERO);
    let mut slice = CurveSlice::from_curve_range(&curve, 0..=0);
    let start = slice.start();
    slice.extend_right(0.0);
    assert_eq!(slice.start(), start);
    assert_eq!(slice.end().segment, 0);
    assert!(slice.end().t.is_nan());
    // Original right output through the unchanged current host measurement.
    assert!(slice.measure_length().is_nan());
    slice.extend_right(0.5);
    assert_eq!(slice.end(), CurveInternalCoord { segment: 2, t: 1.0 });
    assert_eq!(slice.measure_length(), 1.0);
    // Additional fresh-state coverage preserves the original positive-distance
    // input without replacing the now-observable NaN continuation above.
    let mut slice = CurveSlice::from_curve_range(&curve, 0..=0);
    slice.extend_right(0.5);
    assert_eq!(slice.end(), CurveInternalCoord { segment: 1, t: 0.5 });
    let mut slice = CurveSlice::from_curve_range(&curve, 3..=3);
    slice.extend_left(0.5);
    assert_eq!(slice.start(), CurveInternalCoord { segment: 1, t: 0.5 });
}

#[test]
fn coordinates_reject_nonfinite_values_without_deserializer_panics() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let result =
            CurveU::<CurveCoordSemantic2D>::deserialize(serde::de::value::F64Deserializer::<
                serde::de::value::Error,
            >::new(value));
        assert!(result.is_err());
    }
    for (value, expected) in [(-f64::MAX, 0.0), (f64::MAX, 1.0), (0.25, 0.25)] {
        let coordinate =
            CurveU::<CurveCoordSemantic2D>::deserialize(serde::de::value::F64Deserializer::<
                serde::de::value::Error,
            >::new(value))
            .unwrap();
        assert_eq!(coordinate.get(), expected);
    }
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(
            std::panic::catch_unwind(|| CurveUCircular::<CurveCoordSemantic2D>::new(value))
                .is_err()
        );
    }
    assert!(Curve3XZ::try_new(vec![Vec3::ZERO, Vec3::new(1.0, f32::NAN, 0.0)], false).is_err());
}

#[test]
fn spline_construction_rejects_degenerate_knots_and_tangents() {
    for points in [
        vec![Vec2::ZERO],
        vec![Vec2::ZERO, Vec2::ZERO],
        vec![Vec2::ZERO, Vec2::splat(f32::NAN)],
    ] {
        assert!(Spline2D::try_from_control_points_with_tangents(&points, None, None).is_err());
    }
    assert_eq!(
        Spline2D::try_from_control_points_with_tangents(
            &[Vec2::ZERO, Vec2::X],
            Some(Vec2::splat(f32::INFINITY)),
            None
        )
        .unwrap_err(),
        SplineError::NonFiniteTangent
    );
    assert_eq!(
        Spline2D::try_from_periodic_loop(&[Vec2::ZERO, Vec2::X, Vec2::ZERO]).unwrap_err(),
        SplineError::InvalidSegment(2)
    );
    assert!(
        Spline3D::try_from_control_points_with_tangents(&[Vec3::ZERO, Vec3::ZERO], None, None)
            .is_err()
    );
}

#[test]
fn splines_interpolate_knots_and_periodic_boundary_derivatives() {
    let points = [
        Vec3::ZERO,
        Vec3::X,
        Vec3::new(2.0, 1.0, 0.0),
        Vec3::new(3.0, 1.0, 1.0),
    ];
    let spline =
        Spline3D::try_from_control_points_with_tangents(&points, Some(Vec3::X), Some(Vec3::Z))
            .unwrap();
    for (segment, point) in spline.segments.iter().zip(points) {
        assert!((spline.position_at_t(segment.start_t) - point).length() < 1e-5);
    }
    assert!((spline.position_at_t(spline.max_t) - points[3]).length() < 1e-5);
    assert!((spline.tangent_at_t(spline.min_t) - Vec3::X).length() < 1e-5);
    assert!((spline.tangent_at_t(spline.max_t) - Vec3::Z).length() < 1e-5);
    let periodic = Spline3D::try_from_periodic_loop(&points).unwrap();
    assert!((periodic.position_at_t(periodic.max_t) - points[0]).length() < 1e-5);
    assert!((periodic.tangent_at_t(periodic.max_t) - periodic.tangent_at_t(0.0)).length() < 1e-5);
}

#[test]
fn rdp_rejects_invalid_options_and_keeps_ordered_source_indices() {
    let points = [Vec2::ZERO, Vec2::X, Vec2::X * 2.0];
    for epsilon in [-1.0, f32::NAN, f32::INFINITY] {
        assert!(std::panic::catch_unwind(|| Rdp::new_with_opts(&points, epsilon, 0.0)).is_err());
    }
    for spacing in [-1.0, f32::NAN, f32::INFINITY] {
        assert!(std::panic::catch_unwind(|| Rdp::new_with_opts(&points, 0.1, spacing)).is_err());
    }
    let result = Rdp::new_with_opts(&points, 0.0, 0.0);
    assert_eq!(result.indices, [0, 2]);
    let folded = [Vec2::ZERO, Vec2::X, Vec2::new(1.0, 1.0), Vec2::ZERO];
    let result = Rdp::new_with_opts(&folded, 0.0, 0.0);
    assert!(result.indices.windows(2).all(|pair| pair[0] < pair[1]));
    for (&index, point) in result.indices.iter().zip(&result.points) {
        assert_eq!(*point, folded[index]);
    }
}
