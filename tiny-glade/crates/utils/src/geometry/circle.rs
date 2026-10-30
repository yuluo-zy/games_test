//! Circle geometry and perimeter sampling.
use super::{Aabb2, Ray2};
use crate::circular_f32::CircularF32;
use crate::curve::{Curve2, Curve3XZ, CurveError, CurveU};
use glam::{Vec2, Vec3};
use lerp::Lerp;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle2d {
    pub center: Vec2,
    pub radius: f32,
    pub rotation: f32,
}
impl Circle2d {
    /// B 0x140c98170 interpolates center/radius and uses shortest circular rotation.
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        let complement = 1.0 - factor;
        Self {
            center: self.center * complement + other.center * factor,
            radius: self.radius * complement + other.radius * factor,
            rotation: CircularF32::new(self.rotation)
                .lerp(other.rotation, factor)
                .as_f32(),
        }
    }
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            rotation: 0.0,
        }
    }
    pub fn new_rotated(center: Vec2, radius: f32, rotation: f32) -> Self {
        Self {
            center,
            radius,
            rotation,
        }
    }
    pub fn area(self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
    pub fn perimeter(self) -> f32 {
        self.radius * std::f32::consts::TAU
    }
    fn angle(self, u: f32) -> f32 {
        let r = self.rotation % std::f32::consts::TAU;
        let r = if r < 0.0 {
            std::f32::consts::TAU + r
        } else {
            r
        };
        r + (1.0 - u) * std::f32::consts::TAU
    }
    pub fn get_pos_at_u(self, u: f32) -> Vec2 {
        let a = self.angle(u);
        Vec2::new(
            a.cos() * self.radius + self.center.x,
            a.sin() * self.radius + self.center.y,
        )
    }
    pub fn get_normal_at_u(self, u: f32) -> Vec2 {
        let a = self.angle(u);
        Vec2::new(a.cos(), a.sin())
    }
    // Original tangent is counterclockwise although increasing u samples clockwise.
    pub fn get_tangent_at_u(self, u: f32) -> Vec2 {
        let a = self.angle(u);
        Vec2::new(-a.sin(), a.cos())
    }
    pub fn get_closest_u_from_pos(self, p: Vec2) -> f32 {
        let d = p - self.center;
        // B 0x140c976f0 returns 0.0 for an unnormalizable displacement.
        let Some(n) = d.try_normalize() else {
            return 0.0;
        };
        let angle = n.y.atan2(n.x);
        let rotation = self.rotation.rem_euclid(std::f32::consts::TAU);
        (1.0 - (angle - rotation) / std::f32::consts::TAU).rem_euclid(1.0)
    }
    pub fn signed_distance(self, p: Vec2) -> f32 {
        let d = p - self.center;
        (d.y * d.y + d.x * d.x).sqrt() - self.radius
    }
    pub fn is_point_inside(self, p: Vec2) -> bool {
        let d = p - self.center;
        self.radius > (d.y * d.y + d.x * d.x).sqrt()
    }
    pub fn contains_point(self, p: Vec2) -> bool {
        let d = p - self.center;
        d.y * d.y + d.x * d.x < self.radius * self.radius
    }
    /// B140c97b40/84B：保留先归一化、再乘半径的运算顺序。
    /// 零、极小和非有限输入按原浮点结果传播，不替换为默认方向。
    pub fn closest_point_on_perimeter(self, p: Vec2) -> Vec2 {
        let d = p - self.center;
        let reciprocal = 1.0 / (d.y * d.y + d.x * d.x).sqrt();
        (d * reciprocal) * self.radius + self.center
    }
    pub fn with_expanded_diameter(self, delta: f32) -> Self {
        Self {
            radius: delta * 0.5 + self.radius,
            ..self
        }
    }
    pub fn aabb(self) -> Aabb2 {
        Aabb2::from_center_radius(self.center, self.radius)
    }
    pub fn intersect_ray(self, ray: Ray2) -> Option<(f32, f32)> {
        let a = ray.direction.y * ray.direction.y + ray.direction.x * ray.direction.x;
        if a == 0.0 {
            return None;
        }
        let relative = ray.origin - self.center;
        let dot = ray.direction.y * relative.y + ray.direction.x * relative.x;
        let b = dot + dot;
        let discriminant = (-4.0 * a)
            * ((relative.y * relative.y + relative.x * relative.x) - self.radius * self.radius)
            + b * b;
        // 原指令只在有序且严格小于零时拒绝；NaN 继续传播到调用者。
        if discriminant < 0.0 {
            None
        } else {
            let root = discriminant.sqrt();
            Some(((-b - root) / (a + a), (root - b) / (a + a)))
        }
    }
    /// B 140c97a40：只有两次有序严格大于返回None；退化/NaN载荷仍可为Some。
    pub fn intersect_circle(self, other: Self) -> Option<(Vec2, Vec2)> {
        let d = self.center - other.center;
        let distance = (d.y * d.y + d.x * d.x).sqrt();
        if distance > self.radius + other.radius {
            return None;
        }
        if (self.radius - other.radius).abs() > distance {
            return None;
        }
        // 原调用者读取Option的tag；不能把零距离或NaN载荷改成None。
        let a = (distance * distance + (self.radius * self.radius - other.radius * other.radius))
            / (distance + distance);
        let delta = other.center - self.center;
        let factor = (self.radius * self.radius - a * a).sqrt() / distance;
        let mid = delta * (a / distance) + self.center;
        let offset = Vec2::new(-delta.y * factor, delta.x * factor);
        Some((mid + offset, mid - offset))
    }
    pub fn as_points2_ptcount(&self, count: usize) -> CirclePoints2<'_> {
        CirclePoints2 {
            circle: self,
            count,
            front: 0,
            back: count,
        }
    }
    fn sample_count(self, max_step: f32, span: f32) -> Result<usize, CurveError> {
        if !max_step.is_finite() || max_step <= 0.0 {
            return Err(CurveError::InvalidSpacing);
        }
        let count = (span * (self.perimeter() / max_step)).ceil().max(3.0);
        if !count.is_finite() || count >= isize::MAX as f32 || !self.radius.is_finite() {
            return Err(CurveError::InvalidSampleCount);
        }
        Ok(count as usize)
    }
    /// Samples both endpoints. Panics if the step is not finite and positive,
    /// or if the requested sample count cannot fit in a collection.
    pub fn as_points2(&self, max_step: f32) -> CirclePoints2<'_> {
        let count = self
            .sample_count(max_step, 1.0)
            .expect("invalid circle sampling parameters");
        self.as_points2_ptcount(count)
    }
    pub fn as_points3(
        &self,
        max_step: f32,
        height: f32,
    ) -> impl DoubleEndedIterator<Item = Vec3> + '_ {
        self.as_points2(max_step)
            .map(move |p| Vec3::new(p.x, height, p.y))
    }
    pub fn try_as_curve2(self, max_step: f32) -> Result<Curve2, CurveError> {
        let count = self.sample_count(max_step, 1.0)?;
        let mut points = Vec::new();
        points
            .try_reserve_exact(count)
            .map_err(|_| CurveError::AllocationFailed)?;
        points.extend(self.as_points2_ptcount(count));
        Curve2::try_new(points, false)
    }
    /// The B sampler includes both endpoints. It does not copy the first vertex
    /// to close the curve: sin(TAU) and sin(0) may leave slightly different points.
    pub fn as_curve2(self, max_step: f32) -> Curve2 {
        self.try_as_curve2(max_step).unwrap()
    }
    pub fn as_curve3(self, max_step: f32, height: f32) -> Curve3XZ {
        Curve3XZ::try_new(self.as_points3(max_step, height).collect(), false).unwrap()
    }
    /// B 0x140c97f50 uses the directed numeric range, without circular unwrapping.
    pub fn as_curve2_range(self, max_step: f32, start: f32, end: f32) -> Curve2 {
        let count = self
            .sample_count(max_step, end - start)
            .expect("invalid circle sampling parameters");
        let points = (0..count)
            .map(|index| {
                let t = index as f32 / (count - 1) as f32;
                let u = CurveU::<crate::curve::CurveCoordSemantic2D>::new(start.lerp(end, t));
                self.get_pos_at_u(u.get())
            })
            .collect();
        Curve2::try_new(points, false).unwrap()
    }
}
pub struct CirclePoints2<'a> {
    pub circle: &'a Circle2d,
    pub count: usize,
    pub front: usize,
    pub back: usize,
}
impl CirclePoints2<'_> {
    fn point(&self, index: usize) -> Vec2 {
        let denominator = self.count.saturating_sub(1).max(1) as f32;
        self.circle.get_pos_at_u(index as f32 / denominator)
    }
}
impl Iterator for CirclePoints2<'_> {
    type Item = Vec2;
    fn next(&mut self) -> Option<Vec2> {
        if self.front >= self.back {
            return None;
        }
        let i = self.front;
        self.front += 1;
        Some(self.point(i))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back.saturating_sub(self.front);
        (n, Some(n))
    }
}
impl DoubleEndedIterator for CirclePoints2<'_> {
    fn next_back(&mut self) -> Option<Vec2> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(self.point(self.back))
    }
}
impl ExactSizeIterator for CirclePoints2<'_> {}
impl std::iter::FusedIterator for CirclePoints2<'_> {}

#[cfg(test)]
mod native_circle_intersection_tests {
    use super::*;

    #[test]
    fn original_ray_nan_payload_cases() {
        let data: serde_json::Value = serde_json::from_str(include_str!("../../../../reports/module-recovery-2026-10-23/m06-circle-ray-nan-oracle.json")).unwrap();
        for row in data["rows"].as_array().unwrap() {
            let values = row["input_bits"].as_array().unwrap();
            let f = |i: usize| f32::from_bits(values[i].as_u64().unwrap() as u32);
            let result = Circle2d::new(Vec2::new(f(0), f(1)), f(2)).intersect_ray(crate::geometry::ray::Ray2 {
                origin: Vec2::new(f(3), f(4)), direction: Vec2::new(f(5), f(6)),
            });
            let words = row["rosetta_words"].as_array().unwrap();
            assert_eq!(result.is_some(), words[0].as_u64().unwrap() != 0, "{row}");
            if let Some((a,b)) = result {
                assert_eq!([a.to_bits(),b.to_bits()], [words[1].as_u64().unwrap() as u32,words[2].as_u64().unwrap() as u32], "{row}");
            }
        }
    }

    #[test]
    fn ray_discriminant_preserves_nan_and_zero_direction_contract() {
        let circle = Circle2d::new(Vec2::ZERO, 2.0);
        for x in [f32::from_bits(0x7fc12345), f32::from_bits(0x7f812345), f32::INFINITY] {
            let (a, b) = circle.intersect_ray(crate::geometry::ray::Ray2 { origin: Vec2::new(x, 0.0), direction: Vec2::X }).unwrap();
            assert!(a.is_nan() && b.is_nan());
        }
        // 原机器码结果记录于 m06-circle-ray-oracle；含已有 NaN 负载。
        for bits in [0x7fc12345, 0x7f812345] {
            let (a, b) = circle.intersect_ray(crate::geometry::ray::Ray2 { origin: Vec2::new(f32::from_bits(bits), 0.0), direction: Vec2::X }).unwrap();
            assert_eq!([a.to_bits(), b.to_bits()], [0xffc12345, 0x7fc12345]);
        }
        for x in [0.0, -0.0] {
            assert_eq!(circle.intersect_ray(crate::geometry::ray::Ray2 { origin: Vec2::ZERO, direction: Vec2::new(x, 0.0) }), None);
        }
        assert!(circle.intersect_ray(crate::geometry::ray::Ray2 { origin: Vec2::new(-3.0, 2.0), direction: Vec2::X }).is_some());
    }

    #[test]
    fn original_point_order_and_both_tangency_boundaries_are_preserved() {
        let a = Circle2d::new(Vec2::ZERO, 5.0);
        let b = Circle2d::new(Vec2::new(6.0, 0.0), 5.0);
        // 原乘加序的3-4-5交点，两个返回点的顺序同机器码。
        assert_eq!(
            a.intersect_circle(b),
            Some((Vec2::new(3.0, 4.0), Vec2::new(3.0, -4.0)))
        );
        let a = Circle2d::new(Vec2::ZERO, 2.0);
        for center in [Vec2::X, Vec2::new(3.0, 0.0)] {
            assert_eq!(
                a.intersect_circle(Circle2d::new(center, 1.0)),
                Some((Vec2::new(2.0, 0.0), Vec2::new(2.0, 0.0)))
            );
        }
        assert!(
            a.intersect_circle(Circle2d::new(Vec2::new(3.01, 0.0), 1.0))
                .is_none()
        );
        assert!(
            a.intersect_circle(Circle2d::new(Vec2::new(0.99, 0.0), 1.0))
                .is_none()
        );
    }

    #[test]
    fn original_unordered_and_zero_distance_paths_keep_some_nan_payload() {
        let circle = Circle2d::new(Vec2::ZERO, 1.0);
        for (a, b) in [
            (circle, circle),
            (
                Circle2d::new(Vec2::ZERO, 0.0),
                Circle2d::new(Vec2::ZERO, 0.0),
            ),
            (Circle2d::new(Vec2::new(f32::NAN, 0.0), 1.0), circle),
            (Circle2d::new(Vec2::ZERO, f32::NAN), circle),
            (
                Circle2d::new(Vec2::new(f32::INFINITY, 0.0), f32::INFINITY),
                Circle2d::new(Vec2::ZERO, f32::INFINITY),
            ),
        ] {
            let (first, second) = a.intersect_circle(b).expect("原两次JA不会拒绝unordered");
            assert!(
                [first.x, first.y, second.x, second.y]
                    .into_iter()
                    .all(f32::is_nan)
            );
        }
        // 半径和有限时，无限距离由第一条有序严格大于拒绝。
        assert!(
            Circle2d::new(Vec2::new(f32::INFINITY, 0.0), 1.0)
                .intersect_circle(circle)
                .is_none()
        );
        // 同中心不同半径由第二条严格大于拒绝，不能泛化成所有同心圆Some。
        assert!(
            Circle2d::new(Vec2::ZERO, 2.0)
                .intersect_circle(circle)
                .is_none()
        );
    }

    #[test]
    fn original_tag_distinguishes_trap_roof_border_intersection_from_no_intersection() {
        let probe_radius = f32::from_bits(0x3f6e147a);
        let roof = Circle2d::new(Vec2::ZERO, 3.0);
        let inside = Circle2d::new(Vec2::ZERO, probe_radius);
        let touching_border = Circle2d::new(Vec2::new(2.5, 0.0), probe_radius);
        assert!(roof.intersect_circle(inside).is_none());
        assert!(roof.intersect_circle(touching_border).is_some());
        // 原trap的bef30只按tag拒绝；同圆的Some(NaN)也必须拒绝。
        assert!(inside.intersect_circle(inside).is_some());
        // 原函数没有“必须在大圆内”额外条件，分离的圆同样是None。
        assert!(
            roof.intersect_circle(Circle2d::new(Vec2::new(5.0, 0.0), probe_radius))
                .is_none()
        );
    }
}

#[cfg(test)]
mod native_closest_perimeter_tests {
    use super::*;

    #[test]
    fn finite_radius_five_preserves_original_scaled_direction() {
        let circle = Circle2d::new(Vec2::new(1.0, 2.0), 5.0);
        assert_eq!(circle.closest_point_on_perimeter(Vec2::new(4.0, 6.0)), Vec2::new(4.0, 6.0));
    }

    #[test]
    fn center_and_zero_radius_keep_original_unordered_result() {
        for radius in [0.0, 1.0] {
            let circle = Circle2d::new(Vec2::new(3.0, 4.0), radius);
            assert!(circle.closest_point_on_perimeter(circle.center).is_nan());
        }
    }

    #[test]
    fn overflowing_length_returns_original_center_instead_of_robust_direction() {
        let circle = Circle2d::new(Vec2::new(2.0, 3.0), 4.0);
        assert_eq!(circle.closest_point_on_perimeter(Vec2::splat(1.0e30)), circle.center);
    }

    #[test]
    fn underflow_and_nan_are_not_replaced_with_x_axis() {
        let circle = Circle2d::new(Vec2::ZERO, 1.0);
        let underflow = circle.closest_point_on_perimeter(Vec2::new(1.0e-30, 0.0));
        assert!(underflow.x.is_infinite() && underflow.y.is_nan());
        assert!(circle.closest_point_on_perimeter(Vec2::new(f32::NAN, 1.0)).is_nan());
    }
}
