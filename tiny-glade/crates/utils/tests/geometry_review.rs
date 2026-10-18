use glam::{Mat4, Quat, Vec2, Vec3};
use utils::{
    IsoTransform,
    camera::{CameraBodyMatrices, CameraMatrices, CameraProjectionParams},
    components::transform::Transform,
    geometry::{
        Aabb2, Aabb3, Circle2d, Plane, Ray, Ray2, RayBoxIntersection, Rectangle2d,
        barycentric::{barycentric, barycentric2},
        intersect::{Sphere, Triangle, ray_sphere_intersection, ray_triangle_intersection},
        line_relation::{LineInterval, LineRelation},
        position_from_mesh_uvs::get_position_from_mesh_uv,
        split_heightmap::{ThresholdRegion, split_heightmap_regions},
        triangle_triangle::{TriangleTriangleIntersection, triangles_intersect},
    },
};

fn near3(actual: Vec3, expected: Vec3) {
    assert!(
        (actual - expected).length() < 2e-5,
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn rigid_composition_matches_matrix_multiplication() {
    let transforms = [
        IsoTransform::identity(),
        IsoTransform::from_translation(Vec3::new(4.0, -2.0, 8.0)),
        IsoTransform::from_translation_rotation(
            Vec3::new(-3.0, 5.0, 2.0),
            Quat::from_euler(glam::EulerRot::XYZ, 0.4, -0.7, 1.2),
        ),
    ];
    for parent in transforms {
        for child in transforms {
            let composed = child.nested_in(&parent);
            let expected = Mat4::from_rotation_translation(parent.rotation, parent.translation)
                * Mat4::from_rotation_translation(child.rotation, child.translation);
            let actual = Mat4::from_rotation_translation(composed.rotation, composed.translation);
            for point in [Vec3::ZERO, Vec3::X, Vec3::new(3.0, -5.0, 9.0)] {
                near3(
                    actual.transform_point3(point),
                    expected.transform_point3(point),
                );
            }
        }
    }
}

#[test]
fn camera_projection_and_pixel_unprojection_are_inverses() {
    let params = CameraProjectionParams {
        aspect: 16.0 / 9.0,
        near: 0.1,
        vertical_fov_degrees: 70.0,
    };
    let projection = params.calc_matrices();
    let body = CameraBodyMatrices::from_position_rotation(
        Vec3::new(4.0, 2.0, 3.0),
        Quat::from_rotation_y(0.6),
    );
    let camera = CameraMatrices {
        projection: projection.projection,
        inverse_view: body.inverse_view,
    };
    for p in [Vec3::new(0.0, 0.0, -0.1), Vec3::new(2.0, -3.0, -100.0)] {
        near3(
            projection
                .inverse_projection
                .project_point3(projection.projection.project_point3(p)),
            p,
        );
    }
    for invalid in [
        CameraProjectionParams {
            aspect: 0.0,
            ..params
        },
        CameraProjectionParams {
            near: -1.0,
            ..params
        },
        CameraProjectionParams {
            vertical_fov_degrees: 180.0,
            ..params
        },
        CameraProjectionParams {
            vertical_fov_degrees: f32::NAN,
            ..params
        },
    ] {
        assert!(std::panic::catch_unwind(|| invalid.calc_matrices()).is_err());
    }
    let viewport = Vec2::new(1920.0, 1080.0);
    for mouse in [Vec2::ZERO, viewport * 0.5, viewport] {
        let ray = Ray::from_mouse_ss(
            mouse,
            viewport,
            projection.inverse_projection,
            body.inverse_view,
        );
        let projected = projection
            .projection
            .project_point3(body.view.transform_point3(ray.origin));
        let pixel = Vec2::new(
            (projected.x + 1.0) * viewport.x * 0.5,
            (1.0 - projected.y) * viewport.y * 0.5,
        );
        assert!((pixel - mouse).length() < 0.005);
        assert!((ray.direction.length() - 1.0).abs() < 1e-6);
    }
    let center = Ray::from_mouse_ss(
        viewport * 0.5,
        viewport,
        projection.inverse_projection,
        body.inverse_view,
    );
    near3(center.direction, camera.eye_direction());
    near3(
        (body.view * body.inverse_view).transform_point3(Vec3::ONE),
        Vec3::ONE,
    );
    assert!((camera.aspect_ratio() - params.aspect).abs() < 1e-6);
}

#[test]
fn planes_reject_parallel_and_nonfinite_hits() {
    let plane = Plane {
        point: Vec3::ZERO,
        normal: Vec3::Y,
    };
    for origin in [Vec3::ZERO, Vec3::Y, -Vec3::Y] {
        assert_eq!(
            plane.intersect_ray(
                Ray {
                    origin,
                    direction: Vec3::X
                },
                0.0
            ),
            None
        );
    }
    let ray = Ray {
        origin: Vec3::Y * 3.0,
        direction: -Vec3::Y * 2.0,
    };
    assert_eq!(plane.intersect_ray(ray, 0.0), Some((1.5, Vec3::ZERO)));
    assert_eq!(plane.intersect_ray(ray, 2.0), None);
}

#[test]
fn bounds_handle_parallel_boundary_stationary_and_reflected_inputs() {
    let bounds = Aabb3 {
        min: Vec3::ZERO,
        max: Vec3::ONE,
    };
    for x in [0.0, 0.5, 1.0] {
        assert_eq!(
            bounds.classify_ray_intersection(Ray {
                origin: Vec3::new(x, 0.5, -1.0),
                direction: Vec3::Z
            }),
            RayBoxIntersection::Hit {
                enter: 1.0,
                exit: 2.0
            }
        );
    }
    for x in [-0.1, 1.1] {
        assert_eq!(
            bounds.classify_ray_intersection(Ray {
                origin: Vec3::new(x, 0.5, -1.0),
                direction: Vec3::Z
            }),
            RayBoxIntersection::NoHit
        );
    }
    for point in [Vec3::ZERO, Vec3::splat(0.5), Vec3::ONE] {
        assert_eq!(bounds.intersect_segment(point, point), Some(point));
        assert_eq!(
            bounds.classify_ray_intersection(Ray {
                origin: point,
                direction: Vec3::ZERO
            }),
            RayBoxIntersection::NoHit
        );
    }
    assert_eq!(bounds.intersect_segment(-Vec3::ONE, -Vec3::ONE), None);
    let flat = Aabb2 {
        min: Vec2::ZERO,
        max: Vec2::ONE,
    };
    assert_eq!(
        flat.intersect_segment(Vec2::new(0.0, -1.0), Vec2::new(0.0, 2.0)),
        Some(Vec2::ZERO)
    );
    assert_eq!(
        flat.intersect_ray(Ray2 {
            origin: Vec2::new(-1.0, 0.5),
            direction: Vec2::Y
        }),
        f32::INFINITY
    );
    let reflected = bounds.scaled(Vec3::new(-2.0, 3.0, -4.0));
    assert_eq!(reflected.min, Vec3::new(-2.0, 0.0, -4.0));
    assert_eq!(reflected.max, Vec3::new(0.0, 3.0, 0.0));
}

#[test]
fn rectangle_distance_matches_perimeter_on_every_side() {
    let rectangle = Rectangle2d::new(
        Vec2::new(5.0, -3.0),
        Vec2::new(4.0, 2.0),
        Vec2::new(0.6, 0.8),
    );
    for (point, distance) in [
        (Vec2::ZERO, -1.0),
        (Vec2::new(2.0, 0.0), 0.0),
        (Vec2::new(4.0, 0.0), 2.0),
        (Vec2::new(4.0, 4.0), 13.0_f32.sqrt()),
    ] {
        for sign in [
            Vec2::ONE,
            -Vec2::ONE,
            Vec2::new(-1.0, 1.0),
            Vec2::new(1.0, -1.0),
        ] {
            let world = rectangle.local_to_world(point * sign);
            assert!((rectangle.signed_distance_to_closest_edge(world) - distance).abs() < 1e-5);
        }
    }
    for index in 0..4 {
        let (a, b) = rectangle.get_edge_pts(index);
        assert_eq!(rectangle.get_closest_edge_id((a + b) * 0.5), index);
    }
}

#[test]
fn circle_sampling_handles_empty_singleton_and_mixed_ends() {
    let circle = Circle2d::new(Vec2::new(1.0, 2.0), 3.0);
    assert_eq!(circle.as_points2_ptcount(0).next(), None);
    assert_eq!(
        circle.as_points2_ptcount(1).next(),
        Some(circle.get_pos_at_u(0.0))
    );
    assert_eq!(
        circle.as_points2_ptcount(1).next_back(),
        Some(circle.get_pos_at_u(0.0))
    );
    let expected: Vec<_> = circle.as_points2_ptcount(9).collect();
    let mut points = circle.as_points2_ptcount(9);
    for index in 0..4 {
        assert_eq!(points.next(), Some(expected[index]));
        assert_eq!(points.next_back(), Some(expected[8 - index]));
        assert_eq!(points.len(), 7 - index * 2);
    }
    assert_eq!(points.next(), Some(expected[4]));
    assert_eq!(points.next_back(), None);
    assert_eq!(points.next(), None);
    assert_eq!(circle.intersect_circle(circle), None);
    assert_eq!(
        circle.closest_point_on_perimeter(circle.center),
        circle.center + Vec2::X * circle.radius
    );
    for step in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(std::panic::catch_unwind(|| circle.as_points2(step)).is_err());
        assert!(circle.try_as_curve2(step).is_err());
        assert!(std::panic::catch_unwind(|| circle.as_curve2_range(step, 0.0, 1.0)).is_err());
    }
}

#[test]
fn barycentric_uv_and_intersection_queries_cover_degenerate_inputs() {
    let weights = barycentric2(Vec2::new(0.2, 0.3), Vec2::ZERO, Vec2::X, Vec2::Y);
    near3(weights, Vec3::new(0.5, 0.2, 0.3));
    near3(
        barycentric(Vec3::new(0.2, 0.3, 7.0), Vec3::ZERO, Vec3::X, Vec3::Y),
        weights,
    );
    let positions = [[Vec3::ZERO, Vec3::X * 2.0, Vec3::Y * 3.0]];
    let uvs = [[Vec2::ZERO, Vec2::X, Vec2::Y]];
    near3(
        get_position_from_mesh_uv(&positions, &uvs, Vec2::new(0.2, 0.3)).unwrap(),
        Vec3::new(0.4, 0.9, 0.0),
    );
    assert_eq!(get_position_from_mesh_uv(&[], &uvs, Vec2::ZERO), None);
    assert_eq!(
        get_position_from_mesh_uv(&positions, &[[Vec2::ZERO; 3]], Vec2::ZERO),
        None
    );
    let ray = Ray {
        origin: Vec3::new(0.2, 0.3, 1.0),
        direction: -Vec3::Z * 2.0,
    };
    let hit = ray_triangle_intersection(ray, Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y)).unwrap();
    assert_eq!(hit.distance, 0.5);
    assert_eq!(
        ray_triangle_intersection(ray, Triangle::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO)),
        None
    );
    let sphere_hit = ray_sphere_intersection(
        Ray {
            origin: Vec3::new(0.0, 0.0, -3.0),
            direction: Vec3::Z * 2.0,
        },
        Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        },
    )
    .unwrap();
    assert_eq!((sphere_hit.near, sphere_hit.far), (1.0, 2.0));
    assert_eq!(
        LineInterval::line_segment(Vec2::ZERO, Vec2::ONE)
            .relate(LineInterval::line_segment(Vec2::X, Vec2::Y)),
        LineRelation::Intersection(Vec2::splat(0.5))
    );
    let segment = LineInterval::line_segment(Vec2::ZERO, Vec2::X);
    for (point, expected) in [
        (
            Vec2::new(0.5, 0.0),
            LineRelation::Intersection(Vec2::new(0.5, 0.0)),
        ),
        (Vec2::Y, LineRelation::Disjoint),
        (Vec2::X * 2.0, LineRelation::Disjoint),
    ] {
        let singleton = LineInterval::line_segment(point, point);
        assert_eq!(segment.relate(singleton), expected);
        assert_eq!(singleton.relate(segment), expected);
    }
    let a = [Vec3::ZERO, Vec3::X, Vec3::Y];
    assert_eq!(
        triangles_intersect(a, a),
        TriangleTriangleIntersection::Coplanar
    );
    assert_eq!(
        triangles_intersect(a, a.map(|p| p + Vec3::Z)),
        TriangleTriangleIntersection::Disjoint
    );
}

#[test]
fn look_at_and_heightmap_keep_finite_structured_results() {
    let mut transform = Transform {
        t: Vec3::ZERO,
        r: Quat::IDENTITY,
        s: Vec3::ONE,
    };
    transform.look_at(Vec3::ZERO, Vec3::Y);
    assert_eq!(transform.r, Quat::IDENTITY);
    for (target, up) in [
        (Vec3::Y, Vec3::Y),
        (Vec3::NEG_Z, Vec3::ZERO),
        (Vec3::X, Vec3::Y),
    ] {
        transform.look_at(target, up);
        assert!(transform.r.is_finite());
        near3(transform.forward(), target.normalize());
    }
    let regions = split_heightmap_regions(
        &[0.0, 2.0, 0.0, 2.0, 0.0],
        &[0.0, 0.25, 0.5, 0.75, 1.0],
        1.0,
    );
    assert_eq!(
        regions,
        vec![
            ThresholdRegion {
                start_u: 0.125,
                end_u: 0.375
            },
            ThresholdRegion {
                start_u: 0.625,
                end_u: 0.875
            }
        ]
    );
}
