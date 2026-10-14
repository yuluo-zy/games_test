use glam::{Vec2, Vec3};
use serde::Deserialize;
use utils::{
    natural_cubic::{Spline2D, Spline3D},
    swizzlers::project_vec3_slice_xz,
};

#[derive(Deserialize)]
struct Source {
    points: Vec<[u32; 2]>,
    periodic: bool,
    start: Option<[u32; 2]>,
    end: Option<[u32; 2]>,
    segments: Vec<Vec<u32>>,
    min_t: u32,
    max_t: u32,
}
#[derive(Deserialize)]
struct Dense {
    source: Source,
    index: usize,
    panic: bool,
    #[serde(default)]
    points: Vec<[u32; 2]>,
    #[serde(default)]
    start_tangent: [u32; 2],
    #[serde(default)]
    end_tangent: [u32; 2],
}
#[derive(Deserialize)]
struct Projection {
    input: Vec<[u32; 3]>,
    output: Vec<[u32; 2]>,
}
#[derive(Deserialize)]
struct Fixtures {
    dense: Vec<Dense>,
    projection: Vec<Projection>,
}
fn fixture() -> Fixtures {
    serde_json::from_str(include_str!("fixtures/deep-consumers.json")).unwrap()
}
#[test]
fn original_constructor_to_dense_segment_consumer() {
    for row in fixture().dense {
        let points: Vec<_> = row
            .source
            .points
            .into_iter()
            .map(|v| Vec2::from_array(v.map(f32::from_bits)))
            .collect();
        let spline = if row.source.periodic {
            Spline2D::from_periodic_loop(&points)
        } else {
            Spline2D::from_control_points_with_tangents(
                &points,
                row.source
                    .start
                    .map(|v| Vec2::from_array(v.map(f32::from_bits))),
                row.source
                    .end
                    .map(|v| Vec2::from_array(v.map(f32::from_bits))),
            )
        };
        assert_eq!(spline.min_t.to_bits(), row.source.min_t);
        assert_eq!(spline.max_t.to_bits(), row.source.max_t);
        assert_eq!(spline.segments.len(), row.source.segments.len());
        for (s, expected) in spline.segments.iter().zip(&row.source.segments) {
            let c = s.coefficients.map(|v| v.to_array().map(f32::to_bits));
            let start = s.start_t.to_bits();
            assert_eq!(
                [
                    start, c[0][0], c[1][0], c[2][0], c[3][0], start, c[0][1], c[1][1], c[2][1],
                    c[3][1], start
                ]
                .as_slice(),
                expected,
                "constructor coefficient mismatch before dense consumer"
            );
        }
        if row.panic {
            assert!(std::panic::catch_unwind(|| spline.densely_sample_segment(row.index)).is_err());
            continue;
        }
        let result = spline.densely_sample_segment(row.index);
        assert_eq!(
            result
                .points
                .into_iter()
                .map(|v| v.to_array().map(f32::to_bits))
                .collect::<Vec<_>>(),
            row.points
        );
        assert_eq!(
            result.start_tangent.to_array().map(f32::to_bits),
            row.start_tangent
        );
        assert_eq!(
            result.end_tangent.to_array().map(f32::to_bits),
            row.end_tangent
        );
    }
}
#[test]
fn original_owned_projection_keeps_xz_payload_and_order() {
    for row in fixture().projection {
        let source: Vec<_> = row
            .input
            .into_iter()
            .map(|v| Vec3::from_array(v.map(f32::from_bits)))
            .collect();
        let result = project_vec3_slice_xz(&source);
        assert_eq!(
            result
                .into_iter()
                .map(|v| v.to_array().map(f32::to_bits))
                .collect::<Vec<_>>(),
            row.output
        );
    }
}

#[derive(Deserialize)]
struct Source3D {
    points: Vec<[u32; 3]>,
    start: Option<[u32; 3]>,
    end: Option<[u32; 3]>,
    segments: Vec<Vec<u32>>,
    min_t: u32,
    max_t: u32,
    samples: Vec<Sample3D>,
}
#[derive(Deserialize)]
struct Sample3D {
    t: u32,
    position: [u32; 3],
    tangent: [u32; 3],
}
#[derive(Deserialize)]
struct Fixtures3D {
    rows: Vec<Source3D>,
}
#[test]
fn shared_scalar_fix_matches_original_3d_constructor_and_queries() {
    let f: Fixtures3D = serde_json::from_str(include_str!("fixtures/deep-spline3d.json")).unwrap();
    for row in f.rows {
        let points: Vec<_> = row
            .points
            .into_iter()
            .map(|v| Vec3::from_array(v.map(f32::from_bits)))
            .collect();
        let spline = Spline3D::from_control_points_with_tangents(
            &points,
            row.start.map(|v| Vec3::from_array(v.map(f32::from_bits))),
            row.end.map(|v| Vec3::from_array(v.map(f32::from_bits))),
        );
        assert_eq!(spline.min_t.to_bits(), row.min_t);
        assert_eq!(spline.max_t.to_bits(), row.max_t);
        assert_eq!(spline.segments.len(), row.segments.len());
        for (s, expected) in spline.segments.iter().zip(&row.segments) {
            let c = s.coefficients.map(|v| v.to_array().map(f32::to_bits));
            let start = s.start_t.to_bits();
            assert_eq!(
                [
                    start, c[0][0], c[1][0], c[2][0], c[3][0], start, c[0][1], c[1][1], c[2][1],
                    c[3][1], start, c[0][2], c[1][2], c[2][2], c[3][2], start
                ]
                .as_slice(),
                expected
            );
        }
        for s in row.samples {
            assert_eq!(
                spline
                    .position_at_t(f32::from_bits(s.t))
                    .to_array()
                    .map(f32::to_bits),
                s.position
            );
            assert_eq!(
                spline
                    .tangent_at_t(f32::from_bits(s.t))
                    .to_array()
                    .map(f32::to_bits),
                s.tangent
            );
        }
    }
}
