//! 原 0x14175ada0：线段在圆形或矩形内的参数区间。
use glam::Vec2;
use utils::curve::{CurveCoordSemantic2D, CurveU};
use utils::geometry::ray::Ray2;
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};

/// 圆周参数跨度位于原线段两端，padding 用于将超出线段端点的交点向内收缩。
pub fn line_x_shape_overlap(
    line: [Vec2; 2],
    start: CurveU<CurveCoordSemantic2D>,
    end: CurveU<CurveCoordSemantic2D>,
    shape: &ShapeParameters,
    padding: f32,
) -> Option<[CurveU<CurveCoordSemantic2D>; 2]> {
    let direction = line[1] - line[0];
    let (near, far) = match shape.get_shape().expect("预制形状") {
        Shape::Circle(circle) => circle.intersect_ray(Ray2 { origin: line[0], direction })?,
        Shape::Rectangle(rect) => {
            let inverse = rect.orientation_mat2().inverse();
            let origin = inverse * (line[0] - rect.center);
            let ray = inverse * direction;
            let half = rect.size * 0.5;
            let t0 = (-half - origin) / ray;
            let t1 = (half - origin) / ray;
            let min = |a: f32, b: f32| if b.is_nan() { a } else if a.is_nan() || b <= a { b } else { a };
            let max = |a: f32, b: f32| if b.is_nan() { a } else if a.is_nan() || b >= a { b } else { a };
            let near = max(min(t0.x, t1.x), min(t0.y, t1.y));
            let far = min(max(t0.x, t1.x), max(t0.y, t1.y));
            if !(near <= far && 0.0 <= far) { return None; }
            (near, far)
        }
    };
    if near > 1.0 || far < 0.0 { return None; }
    let start_value = start.get();
    let end_value = end.get();
    let a = if near < 0.0 {
        let wrapped = (start_value + padding) % 1.0;
        if wrapped < 0.0 { wrapped + 1.0 } else { wrapped }
    } else {
        assert!((0.0..=1.0).contains(&near));
        start_value + (end_value - start_value) * near
    };
    let b = if far > 1.0 {
        let wrapped = (end_value - padding) % 1.0;
        if wrapped < 0.0 { wrapped + 1.0 } else { wrapped }
    } else {
        assert!((0.0..=1.0).contains(&far));
        start_value + (end_value - start_value) * far
    };
    if a > b { None } else { Some([CurveU::from_raw(a), CurveU::from_raw(b)]) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::wall::valid_enclosure::Shape;
    use utils::geometry::{Circle2d, Rectangle2d};
    #[test]
    fn original_rne_cases() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../../../reports/module-recovery-2026-10-23/m06-line-overlap-oracle.json")).unwrap();
        for row in fixtures["rows"].as_array().unwrap() {
            let value = |index: usize| row["shape"][index].as_f64().unwrap() as f32;
            let shape = if row["tag"].as_u64().unwrap() == 0 {
                Shape::Circle(Circle2d { center: Vec2::new(value(0), value(1)), radius: value(2), rotation: value(3) })
            } else {
                Shape::Rectangle(Rectangle2d { axis: Vec2::new(value(0), value(1)), center: Vec2::new(value(2), value(3)), size: Vec2::new(value(4),value(5)) })
            };
            let point = |index:usize| Vec2::new(f32::from_bits(row["line_bits"][index][0].as_u64().unwrap() as u32), f32::from_bits(row["line_bits"][index][1].as_u64().unwrap() as u32));
            let result = std::panic::catch_unwind(|| line_x_shape_overlap([point(0),point(1)], CurveU::new(row["start"].as_f64().unwrap() as f32), CurveU::new(row["end"].as_f64().unwrap() as f32), &ShapeParameters::from_shape(shape), f32::from_bits(row["padding_bits"].as_u64().unwrap() as u32)));
            if row["status"] == "panic" {
                assert!(result.is_err(), "{row}");
                continue;
            }
            assert_eq!(row["status"], "ok", "{row}");
            let result = result.unwrap_or_else(|_| panic!("候选意外 panic: {row}"));
            let words = row["words"].as_array().unwrap();
            let expected = words[0].as_u64().unwrap() != 0;
            assert_eq!(result.is_some(),expected,"{row}");
            if let Some([a,b]) = result {
                for (actual, expected) in [a.get(), b.get()].into_iter().zip(&words[1..3]) {
                    let expected = expected.as_u64().unwrap() as u32;
                    if f32::from_bits(expected).is_nan() {
                        assert!(actual.is_nan(), "{row}");
                    } else {
                        assert_eq!(actual.to_bits(), expected, "{row}");
                    }
                }
            }
        }
    }
}
