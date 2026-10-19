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
    pub fn closest_point_on_perimeter(self, p: Vec2) -> Vec2 {
        let d = p - self.center;
        d.normalize_or(Vec2::X) * self.radius + self.center
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
        if discriminant >= 0.0 {
            let root = discriminant.sqrt();
            Some(((-b - root) / (a + a), (root - b) / (a + a)))
        } else {
            None
        }
    }
    pub fn intersect_circle(self, other: Self) -> Option<(Vec2, Vec2)> {
        let d = self.center - other.center;
        let distance = (d.y * d.y + d.x * d.x).sqrt();
        if !(distance <= self.radius + other.radius
            && (self.radius - other.radius).abs() <= distance)
        {
            return None;
        }
        // Concentric circles have either zero or infinitely many intersections.
        if distance == 0.0 || !distance.is_finite() {
            return None;
        }
        let a = (distance * distance + (self.radius * self.radius - other.radius * other.radius))
            / (distance + distance);
        let delta = other.center - self.center;
        let factor = (self.radius * self.radius - a * a).max(0.0).sqrt() / distance;
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
