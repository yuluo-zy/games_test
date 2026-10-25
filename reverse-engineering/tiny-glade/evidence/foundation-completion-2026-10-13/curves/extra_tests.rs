use super::*;
use glam::{Vec2, Vec3};
use serde_json::Value;

fn scalar(v: &Value) -> f32 {
    f32::from_bits(v.as_u64().unwrap() as u32)
}
fn p2(v: &Value) -> Vec2 {
    Vec2::new(scalar(&v[0]), scalar(&v[1]))
}
fn p3(v: &Value) -> Vec3 {
    Vec3::new(scalar(&v[0]), scalar(&v[1]), scalar(&v[2]))
}
fn compare2(points: &[Vec2], expected: &Value, index: usize) {
    let expected = expected["points"].as_array().unwrap();
    assert_eq!(points.len(), expected.len(), "case {index}");
    for (a, b) in points.iter().zip(expected) {
        assert!(
            a.distance(p2(b)) < 3e-5,
            "case {index}: {a:?} != {:?}",
            p2(b)
        );
    }
}
fn compare3(points: &[Vec3], expected: &Value, index: usize) {
    let expected = expected["points"].as_array().unwrap();
    assert_eq!(points.len(), expected.len(), "case {index}");
    for (a, b) in points.iter().zip(expected) {
        assert!(
            a.distance(p3(b)) < 3e-5,
            "case {index}: {a:?} != {:?}",
            p3(b)
        );
    }
}
#[test]
fn original_completion_extras() {
    let fixtures: Value = serde_json::from_str(include_str!("extra-fixtures.json")).unwrap();
    for (index, row) in fixtures["cases"].as_array().unwrap().iter().enumerate() {
        let method = row["method"].as_str().unwrap();
        let input = row["points"].as_array().unwrap();
        let expected = &row["expected"];
        match method {
            "edge_global" => {
                let c = Curve3::try_new(input.iter().map(p3).collect(), false).unwrap();
                let actual =
                    std::panic::catch_unwind(|| c.try_resample(scalar(&row["argument"]), false));
                if expected["panic"].as_bool() == Some(true) {
                    assert!(actual.is_err());
                } else {
                    compare3(&actual.unwrap().unwrap().points, expected, index);
                }
            }
            "edge_per_segment" => {
                let c = Curve3XZ::try_new(input.iter().map(p3).collect(), false).unwrap();
                let actual = std::panic::catch_unwind(|| {
                    c.try_resample_per_segment(scalar(&row["argument"]))
                });
                if expected["panic"].as_bool() == Some(true) {
                    assert!(actual.is_err());
                } else {
                    compare3(&actual.unwrap().unwrap().points, expected, index);
                }
            }
            "approx_count" => {
                let c = Curve2::try_new(input.iter().map(p2).collect(), false).unwrap();
                compare2(
                    &c.try_resample_approx_pt_count(
                        row["argument"].as_u64().unwrap() as usize,
                        false,
                    )
                    .unwrap()
                    .points,
                    expected,
                    index,
                );
            }
            "flat_remap_per_segment" => {
                let c = Curve2::try_new(input.iter().map(p2).collect(), false).unwrap();
                let actual: Curve3 = c
                    .try_resample_remap_per_segment(scalar(&row["argument"]), |_, _, p| {
                        Vec3::new(p.x, 0.0, p.y)
                    })
                    .unwrap();
                compare3(&actual.points, expected, index);
            }
            "lerp" => {
                let a = Curve2::try_new(input.iter().map(p2).collect(), false).unwrap();
                let b = Curve2::try_new(
                    row["other"].as_array().unwrap().iter().map(p2).collect(),
                    false,
                )
                .unwrap();
                compare2(
                    &a.crashy_lerp(&b, scalar(&row["argument"])).points,
                    expected,
                    index,
                );
            }
            _ => panic!("unknown method {method}"),
        }
    }
}
