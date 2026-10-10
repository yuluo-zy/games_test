//! 线性结构的基础运算与空间候选查询；不理解道路、窗户或引擎对象。
use std::collections::{BTreeMap, BTreeSet};
pub type Point2 = [f64; 2];
pub fn distance(a: Point2, b: Point2) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}
pub fn mix(a: Point2, b: Point2, t: f64) -> Point2 {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
}
pub fn projection(p: Point2, a: Point2, b: Point2) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1]];
    let l = d[0] * d[0] + d[1] * d[1];
    if l < 1e-12 {
        0.
    } else {
        ((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / l
    }
}
fn cross(a: Point2, b: Point2) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}
/// 返回双方的区间参数；共线重叠返回重叠端点，零长度线段不建立交点。
pub fn intersections(a: Point2, b: Point2, c: Point2, d: Point2) -> Vec<(f64, f64)> {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let cd = [d[0] - c[0], d[1] - c[1]];
    let ca = [c[0] - a[0], c[1] - a[1]];
    let denominator = cross(ab, cd);
    if denominator.abs() > 1e-10 {
        let t = cross(ca, cd) / denominator;
        let u = cross(ca, ab) / denominator;
        if (-1e-9..=1. + 1e-9).contains(&t) && (-1e-9..=1. + 1e-9).contains(&u) {
            vec![(t.clamp(0., 1.), u.clamp(0., 1.))]
        } else {
            Vec::new()
        }
    } else if cross(ca, ab).abs() < 1e-9 && distance(a, b) > 1e-9 && distance(c, d) > 1e-9 {
        let mut out: Vec<(f64, f64)> = Vec::new();
        for p in [a, b, c, d] {
            let t = projection(p, a, b);
            let u = projection(p, c, d);
            if (0.0..=1.).contains(&t)
                && (0.0..=1.).contains(&u)
                && !out.iter().any(|&(old, _)| (old - t).abs() < 1e-9)
            {
                out.push((t, u));
            }
        }
        out
    } else {
        Vec::new()
    }
}
/// 以逆时针凸多边形裁剪，供地基最高点查询共用。输出边界与输入均为有限平面点。
pub fn clip_convex(mut polygon: Vec<Point2>, clip: &[Point2]) -> Vec<Point2> {
    for i in 0..clip.len() {
        let a = clip[i];
        let b = clip[(i + 1) % clip.len()];
        let side = |p: Point2| cross([b[0] - a[0], b[1] - a[1]], [p[0] - a[0], p[1] - a[1]]);
        let mut next = Vec::new();
        if polygon.is_empty() {
            break;
        }
        let mut previous = *polygon.last().unwrap();
        let mut ps = side(previous);
        for &p in &polygon {
            let s = side(p);
            if (s >= -1e-9) != (ps >= -1e-9) {
                next.push(mix(previous, p, ps / (ps - s)));
            }
            if s >= -1e-9 {
                next.push(p);
            }
            previous = p;
            ps = s;
        }
        polygon = next;
    }
    polygon
}
/// 网格桶只负责候选；精确交点和高度规则由调用方执行。
#[derive(Default)]
pub struct SpatialGrid<K> {
    buckets: BTreeMap<(i32, i32), BTreeSet<K>>,
}
impl<K: Copy + Ord> SpatialGrid<K> {
    pub fn cells(bounds: [f64; 4]) -> Vec<(i32, i32)> {
        let mut out: Vec<(i32, i32)> = Vec::new();
        for z in (bounds[1] / 4.).floor() as i32..=(bounds[3] / 4.).floor() as i32 {
            for x in (bounds[0] / 4.).floor() as i32..=(bounds[2] / 4.).floor() as i32 {
                out.push((x, z));
            }
        }
        out
    }
    pub fn insert(&mut self, key: K, bounds: [f64; 4]) {
        for cell in Self::cells(bounds) {
            self.buckets.entry(cell).or_default().insert(key);
        }
    }
    pub fn query(&self, bounds: [f64; 4]) -> BTreeSet<K> {
        let mut out = BTreeSet::new();
        for cell in Self::cells(bounds) {
            if let Some(keys) = self.buckets.get(&cell) {
                out.extend(keys);
            }
        }
        out
    }
}

/// 精确三角形射线参数，方向无需归一化；调用方选择自己的数值容差。
pub fn ray_triangle(
    origin: [f64; 3],
    direction: [f64; 3],
    vertices: [[f64; 3]; 3],
    epsilon: f64,
) -> Option<f64> {
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross3 = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let u = sub(vertices[1], vertices[0]);
    let v = sub(vertices[2], vertices[0]);
    let h = cross3(direction, v);
    let det = dot(u, h);
    if det.abs() < epsilon {
        return None;
    }
    let s = sub(origin, vertices[0]);
    let a = dot(s, h) / det;
    if !(0.0..=1.).contains(&a) {
        return None;
    }
    let q = cross3(s, u);
    let b = dot(direction, q) / det;
    if b < 0. || a + b > 1. {
        return None;
    }
    let t = dot(v, q) / det;
    (t >= 0. && t.is_finite()).then_some(t)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crossing_and_collinear_intervals_are_symmetric() {
        assert_eq!(
            intersections([0., 0.], [2., 0.], [1., -1.], [1., 1.]),
            vec![(0.5, 0.5)]
        );
        assert_eq!(
            intersections([0., 0.], [2., 0.], [1., 0.], [3., 0.]).len(),
            2
        );
        assert!(intersections([0., 0.], [1., 0.], [0., 1.], [1., 1.]).is_empty());
    }
}
