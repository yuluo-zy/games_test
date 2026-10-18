use glam::{DVec2, Quat, Vec2, Vec3};
use utils::geometry::{
    Rectangle2d,
    triangle_triangle::{TriangleTriangleIntersection as Intersection, triangles_intersect},
};

mod reference {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/support/geometry_reference.rs"
    ));
}

const PERMUTATIONS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

fn assert_intersection(actual: Intersection, expected: Intersection, tolerance: f32) {
    let near = |a: Vec3, b: Vec3| a.as_dvec3().distance(b.as_dvec3()) <= tolerance as f64;
    let matches = match (actual, expected) {
        (Intersection::Disjoint, Intersection::Disjoint)
        | (Intersection::Coplanar, Intersection::Coplanar) => true,
        (Intersection::Point(a), Intersection::Point(b)) => near(a, b),
        (Intersection::Segment(a, b), Intersection::Segment(c, d)) => {
            (near(a, c) && near(b, d)) || (near(a, d) && near(b, c))
        }
        _ => false,
    };
    assert!(
        matches,
        "actual={actual:?}, expected={expected:?}, tolerance={tolerance}"
    );
}

#[test]
fn triangle_known_intersections_survive_all_windings_and_rigid_scale_transforms() {
    let a = [Vec3::ZERO, Vec3::X, Vec3::Y];
    let cases = [
        (
            [
                Vec3::ZERO,
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(-1.0, 0.0, -1.0),
            ],
            Intersection::Point(Vec3::ZERO),
        ),
        (
            [Vec3::ZERO, Vec3::X, Vec3::Z],
            Intersection::Segment(Vec3::ZERO, Vec3::X),
        ),
        (
            [
                Vec3::new(0.25, -0.5, -1.0),
                Vec3::new(0.25, -0.5, 1.0),
                Vec3::new(0.25, 1.5, 0.0),
            ],
            Intersection::Segment(Vec3::new(0.25, 0.0, 0.0), Vec3::new(0.25, 0.75, 0.0)),
        ),
        (
            [
                Vec3::new(1.25, -0.5, -1.0),
                Vec3::new(1.25, -0.5, 1.0),
                Vec3::new(1.25, 1.5, 0.0),
            ],
            Intersection::Disjoint,
        ),
        (a.map(|p| p + Vec3::Z), Intersection::Disjoint),
        (a.map(|p| p + Vec3::X * 3.0), Intersection::Coplanar),
    ];
    for scale in [1e-9, 1e-3, 1.0, 1e3, 1e9] {
        for rotation in [
            Quat::IDENTITY,
            Quat::from_rotation_y(0.7),
            Quat::from_euler(glam::EulerRot::XYZ, 0.3, -0.8, 1.1),
        ] {
            for translation in [Vec3::ZERO, Vec3::new(5.0, -3.0, 2.0) * scale] {
                let transform = |p| rotation * (p * scale) + translation;
                let transformed_a = a.map(transform);
                for (b, expected) in cases {
                    // Generic f32 rotations can round an originally coplanar pair
                    // onto distinct planes. Keep coplanarity's tolerance test exact.
                    if matches!(expected, Intersection::Coplanar) && rotation != Quat::IDENTITY {
                        continue;
                    }
                    let transformed_b = b.map(transform);
                    let expected = match expected {
                        Intersection::Point(p) => Intersection::Point(transform(p)),
                        Intersection::Segment(p, q) => {
                            Intersection::Segment(transform(p), transform(q))
                        }
                        other => other,
                    };
                    for permutation_a in PERMUTATIONS {
                        for permutation_b in PERMUTATIONS {
                            let first = permutation_a.map(|i| transformed_a[i]);
                            let second = permutation_b.map(|i| transformed_b[i]);
                            for (first, second) in [(first, second), (second, first)] {
                                assert_intersection(
                                    triangles_intersect(first, second),
                                    expected,
                                    2e-5 * scale,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn triangle_random_pairs_match_independent_segment_triangle_reference() {
    let random = fastrand::Rng::with_seed(0x6200_1234_7788_9900);
    let mut hits = 0;
    let mut misses = 0;
    for index in 0..5000 {
        let scale = [1e-8, 1e-3, 1.0, 1e4, 1e10][index % 5];
        let point = || {
            Vec3::new(
                random.f32() * 2.0 - 1.0,
                random.f32() * 2.0 - 1.0,
                random.f32() * 2.0 - 1.0,
            ) * scale
        };
        let a = std::array::from_fn(|_| point());
        let b = std::array::from_fn(|_| point());
        let actual = triangles_intersect(a, b);
        match reference::triangle_reference(a, b) {
            reference::ReferenceIntersection::Disjoint => {
                misses += 1;
                assert_eq!(actual, Intersection::Disjoint, "case {index}: {a:?} {b:?}");
            }
            reference::ReferenceIntersection::Coplanar => {
                assert_eq!(actual, Intersection::Coplanar)
            }
            reference::ReferenceIntersection::Points(points) => {
                hits += 1;
                let (p, q) = match actual {
                    Intersection::Point(p) => (p, p),
                    Intersection::Segment(p, q) => (p, q),
                    _ => panic!("case {index}: expected hits {points:?}, got {actual:?}"),
                };
                let tolerance = scale * 2e-5;
                for endpoint in [p, q] {
                    assert!(
                        points
                            .iter()
                            .any(|hit| endpoint.distance(*hit) <= tolerance),
                        "case {index}: endpoint {endpoint:?}, expected {points:?}"
                    );
                }
                let direction = q.as_dvec3() - p.as_dvec3();
                for hit in points {
                    let offset = hit.as_dvec3() - p.as_dvec3();
                    let t = if direction.length_squared() == 0.0 {
                        0.0
                    } else {
                        (offset.dot(direction) / direction.length_squared()).clamp(0.0, 1.0)
                    };
                    assert!(
                        (offset - direction * t).length() <= tolerance as f64,
                        "case {index}: truncated intersection"
                    );
                }
            }
        }
    }
    assert!(
        hits > 1000 && misses > 1000,
        "oracle must exercise both outcomes"
    );
}

fn rectangle_reference(a: Rectangle2d, b: Rectangle2d) -> bool {
    let corners = |r: Rectangle2d| {
        let center = r.center.as_dvec2();
        let x = r.axis.as_dvec2() * r.size.x as f64 * 0.5;
        let y = DVec2::new(-r.axis.y as f64, r.axis.x as f64) * r.size.y as f64 * 0.5;
        [
            center - x - y,
            center + x - y,
            center + x + y,
            center - x + y,
        ]
    };
    let a = corners(a);
    let b = corners(b);
    let cross = |u: DVec2, v: DVec2| u.x * v.y - u.y * v.x;
    for i in 0..4 {
        for j in 0..4 {
            let p = a[i];
            let q = b[j];
            let u = a[(i + 1) % 4] - p;
            let v = b[(j + 1) % 4] - q;
            let determinant = cross(u, v);
            if determinant != 0.0 {
                let t = cross(q - p, v) / determinant;
                let s = cross(q - p, u) / determinant;
                if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&s) {
                    return true;
                }
            } else if cross(q - p, u) == 0.0 {
                let dimension = if u.x.abs() >= u.y.abs() { 0 } else { 1 };
                let aa = [p[dimension], a[(i + 1) % 4][dimension]];
                let bb = [q[dimension], b[(j + 1) % 4][dimension]];
                if aa[0].min(aa[1]) <= bb[0].max(bb[1]) && bb[0].min(bb[1]) <= aa[0].max(aa[1]) {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn rectangle_boundaries_include_contacts_but_exclude_strict_containment() {
    let a = Rectangle2d::new(Vec2::ZERO, Vec2::splat(2.0), Vec2::X);
    for (center, size, expected) in [
        (Vec2::ZERO, 2.0, true),
        (Vec2::new(2.0, 0.0), 2.0, true),
        (Vec2::splat(2.0), 2.0, true),
        (Vec2::new(1.0, 0.0), 2.0, true),
        (Vec2::new(2.01, 0.0), 2.0, false),
        (Vec2::ZERO, 1.0, false),
        (Vec2::ZERO, 3.0, false),
        (Vec2::new(0.5, 0.0), 1.0, true),
    ] {
        let b = Rectangle2d::new(center, Vec2::splat(size), Vec2::X);
        assert_eq!(a.intersects(b), expected);
        assert_eq!(b.intersects(a), expected);
        assert_eq!(rectangle_reference(a, b), expected);
    }
}

#[test]
fn rectangle_random_boundaries_match_independent_edge_pairs_and_symmetry() {
    let random = fastrand::Rng::with_seed(0x9911_2233);
    let mut hits = 0;
    let mut misses = 0;
    for index in 0..5000 {
        let scale = [1e-8, 1.0, 1e10][index % 3];
        let rectangle = || {
            let angle = random.f32() * std::f32::consts::TAU;
            Rectangle2d {
                center: Vec2::new(random.f32() * 8.0 - 4.0, random.f32() * 8.0 - 4.0) * scale,
                size: Vec2::new(0.1 + random.f32() * 6.0, 0.1 + random.f32() * 6.0) * scale,
                // Public fields can be modified after construction, so exercise
                // nonunit axes as well as the constructor's unit-axis contract.
                axis: Vec2::new(angle.cos(), angle.sin()) * [0.5, 1.0, 2.0][index % 3],
            }
        };
        let a = rectangle();
        let b = rectangle();
        let expected = rectangle_reference(a, b);
        hits += usize::from(expected);
        misses += usize::from(!expected);
        assert_eq!(a.intersects(b), expected, "case {index}: {a:?}, {b:?}");
        assert_eq!(
            b.intersects(a),
            expected,
            "reversed case {index}: {a:?}, {b:?}"
        );
    }
    assert!(hits > 500 && misses > 500);
}

#[test]
fn triangle_extreme_scales_offsets_and_invalid_inputs_have_defined_results() {
    for scale in [1e-30_f32, 1e-20, 1e20, 1e30] {
        let a = [Vec3::ZERO, Vec3::X * scale, Vec3::Y * scale];
        let b = [Vec3::ZERO, Vec3::X * scale, Vec3::Z * scale];
        assert_intersection(
            triangles_intersect(a, b),
            Intersection::Segment(Vec3::ZERO, Vec3::X * scale),
            scale * 1e-5,
        );
    }
    // Use exact binary offsets so the f32 inputs retain the intended geometry.
    let offset = Vec3::new(1_048_576.0, -2_097_152.0, 4_194_304.0);
    let a = [Vec3::ZERO, Vec3::X * 8.0, Vec3::Y * 8.0].map(|p| p + offset);
    let b = [Vec3::ZERO, Vec3::X * 8.0, Vec3::Z * 8.0].map(|p| p + offset);
    assert_intersection(
        triangles_intersect(a, b),
        Intersection::Segment(offset, offset + Vec3::X * 8.0),
        1e-5,
    );
    let triangle = [Vec3::ZERO, Vec3::X, Vec3::Y];
    for invalid in [
        [Vec3::ZERO; 3],
        [Vec3::ZERO, Vec3::X, Vec3::X * 2.0],
        [Vec3::splat(f32::NAN), Vec3::X, Vec3::Y],
        [Vec3::ZERO, Vec3::splat(f32::INFINITY), Vec3::Y],
    ] {
        assert_eq!(
            triangles_intersect(triangle, invalid),
            Intersection::Disjoint
        );
        assert_eq!(
            triangles_intersect(invalid, triangle),
            Intersection::Disjoint
        );
    }
    for (offset, expected) in [
        (1e-11, Intersection::Coplanar),
        (1e-8, Intersection::Disjoint),
    ] {
        assert_eq!(
            triangles_intersect(triangle, triangle.map(|p| p + Vec3::Z * offset)),
            expected
        );
    }
}

#[test]
fn rectangle_invalid_and_degenerate_contours_do_not_produce_spurious_hits() {
    let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0));
    for point in [
        Vec2::ZERO,
        Vec2::new(1.0, 0.0),
        Vec2::splat(1.0),
        Vec2::splat(2.0),
    ] {
        let degenerate = Rectangle2d::new_axis_aligned(point, Vec2::ZERO);
        let expected = point.abs().max_element() == 1.0;
        assert_eq!(rectangle.intersects(degenerate), expected);
        assert_eq!(degenerate.intersects(rectangle), expected);
    }
    for invalid in [
        Rectangle2d {
            axis: Vec2::ZERO,
            ..rectangle
        },
        Rectangle2d {
            center: Vec2::splat(f32::NAN),
            ..rectangle
        },
        Rectangle2d {
            size: Vec2::splat(f32::INFINITY),
            ..rectangle
        },
        Rectangle2d {
            size: -Vec2::ONE,
            ..rectangle
        },
    ] {
        assert!(!rectangle.intersects(invalid));
        assert!(!invalid.intersects(rectangle));
    }
}

#[test]
fn rotated_rectangles_intersect_themselves_and_exact_parallel_edge_contacts() {
    let regression_axis = Vec2::new(0.999_999_94, 0.000_314_159_26);
    let regression = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0));
    let regression = Rectangle2d {
        axis: regression_axis,
        ..regression
    };
    assert!(regression.intersects(regression));
    let random = fastrand::Rng::with_seed(0x1000_7abc);
    for index in 0..2000 {
        let angle = random.f32() * std::f32::consts::TAU;
        let scale = [1e-12, 1.0, 1e12][index % 3];
        let axis = Vec2::new(angle.cos(), angle.sin());
        let a = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(2.0 * scale));
        let a = Rectangle2d { axis, ..a };
        assert!(a.intersects(a), "self-intersection {index}: {a:?}");
        let translated = Rectangle2d {
            center: Vec2::new(7.0, -3.0) * scale,
            ..a
        };
        assert!(
            translated.intersects(translated),
            "translated self-intersection {index}"
        );
        // Both axes and their twofold offsets are exactly represented when
        // scale=1, so this exercises real contact rather than rounded near contact.
        if scale == 1.0 {
            for direction in [axis, -axis, Vec2::new(-axis.y, axis.x)] {
                let touching = Rectangle2d {
                    center: direction * 2.0,
                    ..a
                };
                assert!(
                    a.intersects(touching),
                    "touching {index}: {a:?}, {touching:?}"
                );
                assert!(touching.intersects(a), "reversed touching {index}");
            }
        }
    }
}

#[test]
fn mixed_scale_triangle_sections_match_analytic_endpoints_and_input_order() {
    let a = [Vec3::ZERO, Vec3::X, Vec3::Y];
    let b = [
        Vec3::new(0.5, -1e12, -1e12),
        Vec3::new(0.5, -1e12, 1e12),
        Vec3::new(0.5, 1e12, 0.0),
    ];
    let expected = Intersection::Segment(Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.0));
    for permutation_a in PERMUTATIONS {
        for permutation_b in PERMUTATIONS {
            let a = permutation_a.map(|i| a[i]);
            let b = permutation_b.map(|i| b[i]);
            assert_intersection(triangles_intersect(a, b), expected, 1e-6);
            assert_intersection(triangles_intersect(b, a), expected, 1e-6);
        }
    }
    let random = fastrand::Rng::with_seed(0x7ef0_9901);
    for index in 0..1000 {
        let scale = 2.0_f32.powi(random.i32(-20..=20));
        let extent = scale * 2.0_f32.powi(random.i32(10..=80));
        let cut = [0.0, 0.125, 0.25, 0.5, 0.75, 0.875, 1.125][index % 7] * scale;
        let offset = Vec3::new(2.0, -3.0, 5.0) * scale;
        let a = [Vec3::ZERO, Vec3::X * scale, Vec3::Y * scale].map(|p| p + offset);
        let b = [
            Vec3::new(cut, -extent, -extent),
            Vec3::new(cut, -extent, extent),
            Vec3::new(cut, extent, 0.0),
        ]
        .map(|p| p + offset);
        let expected = if cut > scale {
            Intersection::Disjoint
        } else {
            Intersection::Segment(
                offset + Vec3::X * cut,
                offset + Vec3::new(cut, scale - cut, 0.0),
            )
        };
        for (first, second) in [(a, b), (b, a)] {
            assert_intersection(triangles_intersect(first, second), expected, scale * 1e-5);
        }
    }
}

#[test]
fn noncollinear_skinny_triangles_keep_shared_edges_and_short_cross_sections() {
    let skinny = [Vec3::ZERO, Vec3::X, Vec3::new(1.0, 1e-16, 0.0)];
    let perpendicular_skinny = [Vec3::ZERO, Vec3::X, Vec3::new(1.0, 0.0, 1e-16)];
    assert_intersection(
        triangles_intersect(skinny, perpendicular_skinny),
        Intersection::Segment(Vec3::ZERO, Vec3::X),
        1e-6,
    );
    assert_intersection(
        triangles_intersect(skinny, [Vec3::ZERO, Vec3::X, Vec3::Z]),
        Intersection::Segment(Vec3::ZERO, Vec3::X),
        1e-6,
    );
    let random = fastrand::Rng::with_seed(0x71aa_c900);
    for index in 0..1000 {
        let length = 2.0_f32.powi(random.i32(-40..=40));
        let altitude = length * 2.0_f32.powi(random.i32(-100..=-20));
        let a = [
            Vec3::ZERO,
            Vec3::X * length,
            Vec3::new(length, altitude, 0.0),
        ];
        let shared = [Vec3::ZERO, Vec3::X * length, Vec3::Z * length];
        assert_intersection(
            triangles_intersect(a, shared),
            Intersection::Segment(Vec3::ZERO, Vec3::X * length),
            length * 1e-6,
        );
        let cut = length * 0.5;
        let b = [
            Vec3::new(cut, -length, -length),
            Vec3::new(cut, -length, length),
            Vec3::new(cut, length, 0.0),
        ];
        let expected = Intersection::Segment(
            Vec3::new(cut, 0.0, 0.0),
            Vec3::new(cut, altitude * 0.5, 0.0),
        );
        // Compare relative to the altitude, not the long edge: a collapsed Point
        // or a misplaced endpoint must fail even at an extreme aspect ratio.
        let tolerance = (altitude * 1e-4).max(f32::from_bits(1));
        let actual = triangles_intersect(a, b);
        assert_intersection(actual, expected, tolerance);
        assert_intersection(triangles_intersect(b, a), expected, tolerance);
        if index % 10 == 0 {
            let outside = b.map(|p| p + Vec3::X * length);
            assert_eq!(triangles_intersect(a, outside), Intersection::Disjoint);
        }
    }
}
