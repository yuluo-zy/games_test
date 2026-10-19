use super::*;
use glam::{Vec2, Vec3};
fn vector2(p: &serde_json::Value) -> Vec2 {
    let a = p.as_array().unwrap();
    Vec2::new(
        f32::from_bits(a[0].as_u64().unwrap() as u32),
        f32::from_bits(a[1].as_u64().unwrap() as u32),
    )
}
fn vector3(p: &serde_json::Value) -> Vec3 {
    let a = p.as_array().unwrap();
    Vec3::new(
        f32::from_bits(a[0].as_u64().unwrap() as u32),
        f32::from_bits(a[1].as_u64().unwrap() as u32),
        f32::from_bits(a[2].as_u64().unwrap() as u32),
    )
}
#[test]
fn original_completion_queries() {
    let d: serde_json::Value = serde_json::from_str(include_str!("query-fixtures.json")).unwrap();
    for (case, r) in d["cases"].as_array().unwrap().iter().enumerate() {
        match r["method"].as_str().unwrap() {
            "threshold_constructor" => {
                let c = Curve3Thresholded::try_new(
                    r["points"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(vector3)
                        .collect(),
                    false,
                )
                .unwrap();
                assert_eq!(
                    c.length.to_bits(),
                    r["expected_length"].as_u64().unwrap() as u32,
                    "case {case}"
                );
                assert_eq!(
                    c.points_u
                        .iter()
                        .map(|u| u.to_bits() as u64)
                        .collect::<Vec<_>>(),
                    r["expected_u"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_u64().unwrap())
                        .collect::<Vec<_>>()
                );
            }
            "right_side" => {
                let c = Curve2::try_new(
                    r["points"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(vector2)
                        .collect(),
                    false,
                )
                .unwrap();
                assert_eq!(
                    c.is_on_right_side(vector2(&r["point"])),
                    r["expected"].as_bool().unwrap(),
                    "case {case}"
                );
            }
            "extend" => {
                let mut c = Curve2::try_new(
                    r["points"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(vector2)
                        .collect(),
                    false,
                )
                .unwrap();
                let before = c.clone();
                let result = c.try_extend(
                    &r["suffix"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(vector2)
                        .collect::<Vec<_>>(),
                );
                let expected = match r["tag"].as_u64().unwrap() {
                    3 => Ok(()),
                    2 => Err(CurveError::DuplicateSegment(
                        r["detail"].as_u64().unwrap() as usize
                    )),
                    1 => Err(CurveError::NonFiniteLength),
                    _ => panic!("unexpected error"),
                };
                assert_eq!(result, expected, "case {case}");
                if result.is_err() {
                    // Failed edits now preserve valid points and parameter caches.
                    assert_eq!(c.points, before.points);
                    assert_eq!(c.points_u, before.points_u);
                    assert_eq!(c.length, before.length);
                    continue;
                }
                assert_eq!(
                    c.points
                        .iter()
                        .map(|p| p.to_array().map(f32::to_bits))
                        .collect::<Vec<_>>(),
                    r["expected_points"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| vector2(v).to_array().map(f32::to_bits))
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    c.points_u
                        .iter()
                        .map(|u| u.to_bits() as u64)
                        .collect::<Vec<_>>(),
                    r["expected_u"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_u64().unwrap())
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    c.length.to_bits(),
                    r["expected_length"].as_u64().unwrap() as u32
                );
            }
            "deserialize_scalar" => {
                use serde::Deserialize;
                let value = f64::from_bits(r["value_bits"].as_u64().unwrap());
                let got = std::panic::catch_unwind(|| {
                    CurveU::<CurveCoordSemantic2D>::deserialize(
                        serde::de::value::F64Deserializer::<serde::de::value::Error>::new(value),
                    )
                });
                if !value.is_finite() {
                    assert!(got.unwrap().is_err(), "case {case}");
                } else if r["panic"].as_bool().unwrap() {
                    // Finite f64 overflow is clamped before narrowing.
                    assert_eq!(got.unwrap().unwrap().get(), value.clamp(0.0, 1.0) as f32);
                } else {
                    assert_eq!(
                        got.unwrap().unwrap().get().to_bits(),
                        r["expected"].as_u64().unwrap() as u32,
                        "case {case}"
                    );
                }
            }
            "weighted_smooth" => {
                let c = Curve2::try_new(
                    r["points"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(vector2)
                        .collect(),
                    false,
                )
                .unwrap();
                let w = r["weights"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap() as usize)
                    .collect::<Vec<_>>();
                let c = c
                    .smooth_weighted(r["iterations"].as_u64().unwrap() as usize, &w)
                    .unwrap();
                for (a, b) in c
                    .points
                    .iter()
                    .zip(r["expected"].as_array().unwrap().iter().map(vector2))
                {
                    assert!((*a - b).length() <= 0.00003, "case {case}: {a:?} != {b:?}");
                }
            }
            _ => panic!("unknown fixture"),
        }
    }
}
