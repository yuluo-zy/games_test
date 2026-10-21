#[cfg(test)]
mod native_domain_tests {
    use super::*;
    use crate::curve::{Curve2, CurveError};

    #[test]
    fn repeated_control_points_reach_the_real_curve_failure_boundary() {
        let points = [Vec2::new(3.0, -2.0); 2];
        assert_eq!(
            Spline2D::try_from_control_points_with_tangents(&points, None, None).unwrap_err(),
            SplineError::InvalidSegment(0)
        );
        let spline = Spline2D::from_control_points_with_tangents(&points, None, None);
        assert_eq!(spline.segments.len(), 1);
        assert_eq!(spline.max_t, 0.0);
        let sampled = spline.densely_sample_segment(0);
        assert_eq!(sampled.points.len(), 2);
        assert!(sampled.points.iter().all(|point| point.is_nan()));
        assert_eq!(
            Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );
    }

    #[test]
    fn repeated_closing_point_reaches_periodic_solver_and_curve_consumer() {
        let points = [Vec2::ZERO, Vec2::X, Vec2::ZERO];
        assert_eq!(
            Spline2D::try_from_periodic_loop(&points).unwrap_err(),
            SplineError::InvalidSegment(2)
        );
        let spline = Spline2D::from_periodic_loop(&points);
        assert!(spline.periodic);
        assert_eq!(spline.segments.len(), 3);
        assert_eq!(spline.max_t, 2.0);
        let sampled = spline.densely_sample_segment(2);
        assert_eq!(sampled.points.len(), 2);
        assert_eq!(
            Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );
    }

    #[test]
    fn native_entry_preserves_nonfinite_data_for_the_existing_consumer() {
        let points = [Vec2::ZERO, Vec2::splat(f32::NAN)];
        assert_eq!(
            Spline2D::try_from_control_points_with_tangents(&points, None, None).unwrap_err(),
            SplineError::NonFinitePoint(1)
        );
        let spline = Spline2D::from_control_points_with_tangents(&points, None, None);
        assert!(spline.max_t.is_nan());
        let sampled = spline.densely_sample_segment(0);
        assert_eq!(sampled.points.len(), 2);
        assert_eq!(
            Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );

        let points = [Vec2::ZERO, Vec2::X];
        for tangent in [f32::INFINITY, f32::MAX] {
            let start = Some(Vec2::splat(tangent));
            let expected_error = if tangent.is_finite() {
                SplineError::NonFiniteCoefficients
            } else {
                SplineError::NonFiniteTangent
            };
            assert_eq!(
                Spline2D::try_from_control_points_with_tangents(&points, start, None).unwrap_err(),
                expected_error
            );
            let spline = Spline2D::from_control_points_with_tangents(&points, start, None);
            let sampled = spline.densely_sample_segment(0);
            assert_eq!(sampled.points.len(), 2);
            assert_eq!(
                Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
                CurveError::NonFiniteLength
            );
        }
    }

    fn segment_with_span(span: f32) -> Spline2D {
        Spline2D {
            segments: vec![SplineSegment2D {
                start_t: 0.0,
                coefficients: [
                    Vec2::new(2.0, -3.0),
                    Vec2::new(1.0, 4.0),
                    Vec2::ZERO,
                    Vec2::ZERO,
                ],
            }],
            min_t: 0.0,
            max_t: span,
            periodic: false,
        }
    }

    #[test]
    fn dense_zero_negative_and_nan_spans_keep_two_samples_and_outgoing_tangents() {
        let zero = segment_with_span(0.0).densely_sample_segment(0);
        assert_eq!(zero.points, vec![Vec2::new(2.0, -3.0); 2]);
        assert_eq!(zero.start_tangent, Vec2::new(1.0, 4.0));
        assert_eq!(zero.end_tangent, Vec2::new(-1.0, -4.0));
        assert_eq!(
            Curve2::try_new_from_points(zero.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );

        let reversed = segment_with_span(-1.0).densely_sample_segment(0);
        assert_eq!(
            reversed.points,
            vec![Vec2::new(2.0, -3.0), Vec2::new(1.0, -7.0)]
        );
        assert_eq!(reversed.start_tangent, Vec2::new(1.0, 4.0));
        assert_eq!(reversed.end_tangent, Vec2::new(-1.0, -4.0));
        for span in [f32::NAN, f32::NEG_INFINITY] {
            let sampled = segment_with_span(span).densely_sample_segment(0);
            assert_eq!(sampled.points.len(), 2);
            assert!(sampled.points.iter().all(|point| point.is_nan()));
            assert_eq!(sampled.start_tangent, Vec2::new(1.0, 4.0));
            assert!(sampled.end_tangent.is_nan());
        }
    }

    #[test]
    fn infinite_dense_count_reaches_real_vec_capacity_failure_before_sampling() {
        // usize::MAX 个 Vec2 必定先溢出布局容量，不会向分配器申请巨量内存。
        let panic =
            std::panic::catch_unwind(|| segment_with_span(f32::INFINITY).densely_sample_segment(0))
                .unwrap_err();
        let message = panic
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
            .unwrap_or("");
        assert!(message.contains("capacity overflow"), "{message}");
    }

    #[test]
    #[should_panic(expected = "assertion failed: points.len() >= 2")]
    fn native_open_checks_original_minimum_before_point_data() {
        Spline2D::from_control_points_with_tangents(&[Vec2::splat(f32::NAN)], None, None);
    }

    #[test]
    #[should_panic(expected = "assertion failed: points.len() >= 3")]
    fn native_periodic_checks_original_minimum_before_point_data() {
        Spline2D::from_periodic_loop(&[Vec2::splat(f32::NAN), Vec2::X]);
    }
}
