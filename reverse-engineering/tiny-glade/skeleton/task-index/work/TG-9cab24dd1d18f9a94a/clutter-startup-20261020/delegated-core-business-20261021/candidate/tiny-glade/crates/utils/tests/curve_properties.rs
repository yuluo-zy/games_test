//! Deterministic differential and metamorphic tests for polyline operations.
use glam::{Vec2, Vec3};
use utils::{
    curve::{Curve2, CurveCoordSemantic2D, CurveInternalCoord, curve_slice::CurveSlice},
    rdp::{Rdp, RdpPoint},
    wall_space::{CurveSampling, WallSpaceCoord},
};

fn coordinates(point: Vec3) -> [f64; 3] {
    [f64::from(point.x), f64::from(point.y), f64::from(point.z)]
}

// Scalar f64 reference, independent of glam's vector implementation.
fn segment_distance(point: Vec3, start: Vec3, end: Vec3) -> f64 {
    let (point, start, end) = (coordinates(point), coordinates(start), coordinates(end));
    let vector = std::array::from_fn::<_, 3, _>(|i| end[i] - start[i]);
    let squared: f64 = vector.iter().map(|v| v * v).sum();
    let projection = if squared == 0.0 {
        0.0
    } else {
        ((0..3)
            .map(|i| (point[i] - start[i]) * vector[i])
            .sum::<f64>()
            / squared)
            .clamp(0.0, 1.0)
    };
    (0..3)
        .map(|i| (point[i] - start[i] - projection * vector[i]).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn reference_indices(points: &[Vec3], epsilon: f64) -> Vec<usize> {
    fn simplify(points: &[Vec3], first: usize, last: usize, epsilon: f64, out: &mut Vec<usize>) {
        let mut greatest = 0.0;
        let mut split = first;
        for index in first + 1..last {
            let distance = segment_distance(points[index], points[first], points[last]);
            if distance > greatest {
                greatest = distance;
                split = index;
            }
        }
        if greatest > epsilon {
            simplify(points, first, split, epsilon, out);
            simplify(points, split, last, epsilon, out);
        } else {
            out.push(last);
        }
    }
    match points.len() {
        0 => vec![],
        1 => vec![0],
        count => {
            let mut out = vec![0];
            simplify(points, 0, count - 1, epsilon, &mut out);
            out
        }
    }
}

fn check_tolerance<T: RdpPoint>(points: &[T], result: &Rdp<T>, epsilon: f32) {
    assert_eq!(result.points.len(), result.indices.len());
    assert_eq!(
        result.indices.first().copied(),
        (!points.is_empty()).then_some(0)
    );
    assert_eq!(result.indices.last().copied(), points.len().checked_sub(1));
    for (&index, point) in result.indices.iter().zip(&result.points) {
        assert_eq!(points[index].as_vec3(), point.as_vec3());
    }
    for pair in result.indices.windows(2) {
        assert!(pair[0] < pair[1]);
        for point in &points[pair[0]..=pair[1]] {
            let distance = segment_distance(
                point.as_vec3(),
                points[pair[0]].as_vec3(),
                points[pair[1]].as_vec3(),
            );
            assert!(
                distance <= f64::from(epsilon) + 1e-10,
                "distance={distance}, epsilon={epsilon}, pair={pair:?}"
            );
        }
    }
}

struct Generator(u64);
impl Generator {
    fn sample(&mut self) -> f32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 40) as f32 / (1u32 << 24) as f32 - 0.5) * 20.0
    }
}

#[test]
fn rdp_random_chains_match_recursive_reference_and_bound_every_omitted_point() {
    let mut random = Generator(0xe16b_5a39_1124_9f01);
    for case in 0..96 {
        let points3: Vec<_> = (0..case % 37)
            .map(|_| Vec3::new(random.sample(), random.sample(), random.sample()))
            .collect();
        let points2: Vec<_> = points3.iter().map(|point| point.truncate()).collect();
        for epsilon in [0.0, 0.125, 1.0, 6.0, 30.0] {
            for spacing in [0.0, 0.5, 4.0, 100.0] {
                let result3 = Rdp::new_with_opts(&points3, epsilon, spacing);
                let result2 = Rdp::new_with_opts(&points2, epsilon, spacing);
                check_tolerance(&points3, &result3, epsilon);
                check_tolerance(&points2, &result2, epsilon);
                if spacing == 0.0 {
                    assert_eq!(
                        result3.indices,
                        reference_indices(&points3, f64::from(epsilon))
                    );
                    let projected: Vec<_> = points2.iter().map(|p| p.extend(0.0)).collect();
                    assert_eq!(
                        result2.indices,
                        reference_indices(&projected, f64::from(epsilon))
                    );
                }
            }
        }
    }
}

#[test]
fn rdp_handles_closed_chains_duplicates_folds_and_extreme_finite_coordinates() {
    let cases = [
        vec![Vec3::ZERO; 12],
        vec![
            Vec3::ZERO,
            Vec3::X * 5.0,
            Vec3::X,
            Vec3::X * 9.0,
            Vec3::ZERO,
        ],
        vec![Vec3::ZERO, Vec3::X * 10.0, Vec3::X],
        vec![
            Vec3::ZERO,
            Vec3::Y,
            Vec3::X * 10.0,
            Vec3::X * 10.0 + Vec3::Y,
        ],
        (0..48)
            .map(|i| Vec3::new(i as f32, if i % 2 == 0 { 0.0 } else { 3.0 }, 0.0))
            .collect(),
        vec![
            Vec3::ZERO,
            Vec3::X,
            Vec3::ONE,
            Vec3::Y,
            Vec3::ZERO,
            Vec3::ZERO,
        ],
        vec![
            Vec3::new(-f32::MAX, 0.0, 0.0),
            Vec3::new(0.0, f32::MAX, 0.0),
            Vec3::new(f32::MAX, 0.0, 0.0),
        ],
        vec![Vec3::ZERO, Vec3::splat(f32::MIN_POSITIVE), Vec3::ZERO],
    ];
    for points in cases {
        for epsilon in [0.0, 0.01, 0.5, 4.0, f32::MAX] {
            for spacing in [0.0, 1.0, 100.0, f32::MAX] {
                let result = Rdp::new_with_opts(&points, epsilon, spacing);
                check_tolerance(&points, &result, epsilon);
                // Extreme coordinates may conservatively retain extra points
                // when the numerical bound straddles epsilon.
                if spacing == 0.0 && points.iter().all(|p| p.abs().max_element() < 1e10) {
                    assert_eq!(
                        result.indices,
                        reference_indices(&points, f64::from(epsilon))
                    );
                }
            }
        }
    }
    let fold = [Vec2::ZERO, Vec2::new(10.0, 0.0), Vec2::X];
    assert_eq!(Rdp::new_with_opts(&fold, 0.1, 100.0).indices, [0, 1, 2]);
}

fn arc_position(points: &[Vec2], coord: CurveInternalCoord) -> f64 {
    let before: f64 = points[..=coord.segment]
        .windows(2)
        .map(|pair| pair[0].as_dvec2().distance(pair[1].as_dvec2()))
        .sum();
    before
        + f64::from(coord.t)
            * points[coord.segment]
                .as_dvec2()
                .distance(points[coord.segment + 1].as_dvec2())
}

#[test]
fn slice_extensions_follow_original_right_f32_and_left_arc_arithmetic() {
    // B 1412898e0/921B scalar-instruction lift for the unchanged64 seeded
    // chains and5 t values. These are lifted expected bits, not a native run.
    // NaN payload bits are intentionally compared only with is_nan.
    const RIGHT_ZERO_EXPECTED_BITS: [[u32; 5]; 64] = [
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4cccc8, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4cccc8, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4cccc8, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4cccc8, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
        [0x0, 0x0, 0x3e4ccccc, 0x3f000000, 0x3f800000],
    ];
    let mut random = Generator(0x02e7_5a31_861f_4221);
    for case in 0..64 {
        let mut points = vec![Vec2::ZERO];
        for index in 0..12 {
            let previous = *points.last().unwrap();
            let next = if (case + index) % 5 == 0 {
                previous
            } else {
                previous + Vec2::new(random.sample(), random.sample())
            };
            points.push(next);
        }
        let total = arc_position(
            &points,
            CurveInternalCoord {
                segment: points.len() - 2,
                t: 1.0,
            },
        );
        for (t_index, t) in [0.0, 1e-8, 0.2, 0.5, 1.0].into_iter().enumerate() {
            let coord = CurveInternalCoord {
                segment: case % 12,
                t,
            };
            let origin = arc_position(&points, coord);
            for distance in [0.0, 1e-6, 0.1, 3.0, 18.0, 1e5] {
                let mut slice = CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_coord_range(
                    points.as_slice(),
                    coord,
                    coord,
                );
                slice.extend_left(distance);
                slice.extend_right(distance);
                if distance == 0.0 {
                    assert_eq!(slice.start(), coord);
                    let expected_t = f32::from_bits(RIGHT_ZERO_EXPECTED_BITS[case][t_index]);
                    assert_eq!(slice.end().segment, coord.segment);
                    if expected_t.is_nan() {
                        assert!(slice.end().t.is_nan());
                    } else {
                        assert_eq!(slice.end().t.to_bits(), expected_t.to_bits());
                    }
                }
                let left = arc_position(&points, slice.start());
                let right = arc_position(&points, slice.end());
                assert!((left - (origin - f64::from(distance)).max(0.0)).abs() < 3e-5);
                if right.is_nan() {
                    // Retain every repeated-edge/zero-distance input. The
                    // original right method writes0/0 instead of an identity.
                    assert_eq!(distance, 0.0);
                    assert_eq!(points[coord.segment], points[coord.segment + 1]);
                } else {
                    assert!((right - (origin + f64::from(distance)).min(total)).abs() < 3e-5);
                }
                let mut split = CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_coord_range(
                    points.as_slice(),
                    coord,
                    coord,
                );
                for _ in 0..2 {
                    split.extend_left(distance * 0.5);
                    split.extend_right(distance * 0.5);
                }
                assert!((arc_position(&points, split.start()) - left).abs() < 3e-5);
                if right.is_nan() {
                    // Keep the original two-half call chain: its second right
                    // call propagates the NaN residual to the last endpoint.
                    assert_eq!(split.end(), CurveInternalCoord { segment: points.len() - 2, t: 1.0 });
                    assert_eq!(arc_position(&points, split.end()), total);
                } else {
                    assert!((arc_position(&points, split.end()) - right).abs() < 3e-5);
                }
            }
        }
    }
    let points = [Vec2::ZERO, Vec2::new(1000.0, 0.0)];
    let start = CurveInternalCoord {
        segment: 0,
        t: 1e-8,
    };
    let mut slice = CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_coord_range(
        points.as_slice(),
        start,
        start,
    );
    slice.extend_left(5e-6);
    slice.extend_right(5e-6);
    assert!((slice.start().t - 5e-9).abs() < 1e-15);
    // Original f32 cancellation rounds both1-t and1000-5e-6 back.
    assert_eq!(slice.end().t.to_bits(), 0.0f32.to_bits());
}

#[test]
fn wall_extrapolation_uses_actual_endpoint_segments_and_original_negative_direction() {
    for repeated_start in 0..8 {
        for repeated_end in 0..8 {
            let mut points = vec![Vec2::ZERO; repeated_start + 1];
            points.extend([Vec2::new(3.0, 4.0), Vec2::new(6.0, 0.0)]);
            points.extend(std::iter::repeat_n(Vec2::new(6.0, 0.0), repeated_end));
            let curve = Curve2::try_new(points, false).unwrap();
            for distance in [0.001, 1.0, 50.0] {
                let left = WallSpaceCoord::new_unchecked(-distance, 2.0).to_world_space(&curve);
                let right = WallSpaceCoord::new_unchecked(curve.length + distance, 2.0)
                    .to_world_space(&curve);
                if repeated_start == 0 {
                    assert!(left.is_finite());
                    assert!(left.distance(Vec3::new(0.6 * distance, 2.0, 0.8 * distance)) < 1e-5);
                } else {
                    // Original actual zero first segment is not skipped.
                    assert!(left.x.is_nan() && left.y.is_nan() && left.z.is_nan());
                }
                if repeated_end == 0 {
                    assert!(right.is_finite());
                    assert!(
                        right.distance(Vec3::new(6.0 + 0.6 * distance, 2.0, -0.8 * distance)) < 1e-5
                    );
                } else {
                    assert!(right.x.is_nan() && right.y.is_nan() && right.z.is_nan());
                }
            }
        }
    }
    // Host custom-domain fixture, not an original shipped Curve2. Preserve its
    // original input; rawZERO now propagates through both native normalizations.
    struct Stationary;
    impl CurveSampling for Stationary {
        fn length(&self) -> f32 {
            1.0
        }
        fn point_u(&self, _: usize) -> f32 {
            0.0
        }
        fn position_at_u(&self, _: f32) -> Vec2 {
            Vec2::ZERO
        }
        fn tangent_at_u(&self, _: f32) -> Vec2 {
            Vec2::ZERO
        }
        fn approximate_u(&self, _: Vec2) -> f32 {
            0.0
        }
    }
    let stationary = WallSpaceCoord::new_unchecked(-1.0, 4.0).to_world_space(&Stationary);
    assert!(stationary.x.is_nan() && stationary.y.is_nan() && stationary.z.is_nan());
}

#[test]
fn wall_interpolation_preserves_phase_for_many_turns_and_linear_clamping() {
    use utils::{
        comparative_space::{CircularComparativeSpace, ComparativeSpace},
        wall_space::WallSpace,
    };
    let wall = WallSpace {
        is_loop: true,
        length: 11.0,
    };
    assert_eq!(wall.lerp_clamp(1e8, 0.0, 0.5), 0.5);
    for period in [0.25, 11.0, 1024.0] {
        let wall = WallSpace {
            is_loop: true,
            length: period,
        };
        let circular = CircularComparativeSpace { period };
        for a in [0.0, 1.0, -7.5, 1e8, -f32::MAX] {
            for b in [0.0, 2.0, 1e6, f32::MAX] {
                for t in [0.0, 0.125, 0.5, 1.0] {
                    let actual = wall.lerp_clamp(a, b, t);
                    assert!(actual.is_finite());
                    assert_eq!(actual, circular.lerp_clamp(a, b, t));
                    if t == 0.0 {
                        assert_eq!(actual, circular.project_clamp(a));
                    }
                }
            }
        }
    }
    let open = WallSpace {
        is_loop: false,
        length: 11.0,
    };
    for (a, b, t, expected) in [
        (2.0, 8.0, 0.5, 5.0),
        (-20.0, 0.0, 0.5, 0.0),
        (20.0, 30.0, 0.5, 11.0),
    ] {
        assert_eq!(open.lerp_clamp(a, b, t), expected);
    }
}

#[test]
fn rdp_retains_small_off_line_details_between_extreme_endpoints() {
    // The exact segment is y=x, so (1,0) has distance sqrt(1/2).
    // A translated f64 reference loses the 1 in 1 + 1e20 too; assert the
    // analytically known result instead of using the differential helper.
    for magnitude in [1e10_f32, 1e20, 1e30, f32::MAX] {
        for point in [Vec3::X, Vec3::Y, Vec3::new(3.0, -3.0, 0.0)] {
            let points = [
                Vec3::new(-magnitude, -magnitude, 0.0),
                point,
                Vec3::new(magnitude, magnitude, 0.0),
            ];
            for spacing in [0.0, f32::MAX] {
                assert_eq!(Rdp::new_with_opts(&points, 0.1, spacing).indices, [0, 1, 2]);
            }
        }
    }
    // Exact simple collinearity should still simplify with zero tolerance.
    assert_eq!(
        Rdp::new_with_opts(&[Vec3::ZERO, Vec3::ONE, Vec3::splat(2.0)], 0.0, 0.0).indices,
        [0, 2]
    );
}

#[test]
fn slice_lengths_and_extensions_handle_tiny_and_extreme_segments() {
    for length in [1e-30_f32, f32::MIN_POSITIVE, 1e20, f32::MAX] {
        let points = [Vec2::ZERO, Vec2::new(length, 0.0)];
        let mut slice =
            CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_point_range(points.as_slice(), 0..=0);
        slice.extend_right(length * 0.5);
        // Keep the original right→measure input chain. The right outcome is
        // nativef32; measure_length is the unchanged current host consumer,
        // whose extreme-input nativeB equivalence is not claimed here.
        if length < 1.0 {
            assert_eq!(slice.end(), CurveInternalCoord { segment: 0, t: 1.0 });
            assert_eq!(slice.measure_length() / length, 1.0);
        } else {
            assert!(slice.end().t.is_nan());
            assert!(slice.measure_length().is_nan());
        }
        // Supplement the combination case with the original direct length
        // expectation on an explicit half-segment coordinate range.
        let measured = CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_coord_range(
            points.as_slice(),
            CurveInternalCoord { segment: 0, t: 0.0 },
            CurveInternalCoord { segment: 0, t: 0.5 },
        );
        assert!((measured.measure_length() / length - 0.5).abs() < 1e-6);
        // 原构造器的f32平方在此下溢/上溢；现稳健slice测长断言继续独立保留。
        assert!(matches!(
            Curve2::try_new(points.to_vec(), false),
            Err(utils::curve::CurveError::NonFiniteLength)
        ));
        let mut slice =
            CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_point_range(points.as_slice(), 1..=1);
        slice.extend_left(length * 0.5);
        assert!((slice.start().t - 0.5).abs() < 1e-6);
    }
    let points = [Vec2::ZERO, Vec2::new(1e-30, 0.0), Vec2::new(1e-30, 1.0)];
    let curve = Curve2::try_new(points.to_vec(), false).unwrap();
    let mut slice = CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_curve_range(&curve, 0..=0);
    slice.extend_right(5e-31);
    assert_eq!(slice.end(), CurveInternalCoord { segment: 1, t: 0.0 });
    assert_eq!(slice.measure_length(), 1e-30);
    // Individual finite endpoints can overflow the original f32 delta/length.
    // Keep the original right→measure chain and the unchanged wider left test.
    let points = [Vec2::new(-f32::MAX, 0.0), Vec2::new(f32::MAX, 0.0)];
    let mut slice =
        CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_point_range(points.as_slice(), 0..=0);
    slice.extend_right(f32::MAX);
    assert!(slice.end().t.is_nan());
    assert!(slice.measure_length().is_nan());
    let mut slice =
        CurveSlice::<Vec2, _, CurveCoordSemantic2D>::from_point_range(points.as_slice(), 1..=1);
    slice.extend_left(f32::MAX);
    assert_eq!(slice.start().t, 0.5);
}
