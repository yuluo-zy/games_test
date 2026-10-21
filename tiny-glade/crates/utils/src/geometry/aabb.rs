//! Axis-aligned bounds with inclusive intersection boundaries.
use super::ray::{Ray, Ray2};
use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb2 {
    pub min: Vec2,
    pub max: Vec2,
}
impl Aabb2 {
    /// B inline from_points folds from finite extrema, including empty input.
    pub fn from_points(points: impl IntoIterator<Item = Vec2>) -> Self {
        let mut bounds = Self {
            min: Vec2::splat(f32::MAX),
            max: Vec2::splat(f32::MIN),
        };
        for point in points {
            bounds.min = bounds.min.min(point);
            bounds.max = bounds.max.max(point);
        }
        bounds
    }
    /// Returns the first segment point in the closed bounds.
    pub fn intersect_segment(self, start: Vec2, end: Vec2) -> Option<Vec2> {
        intersection_aabb_segment2(self.min, self.max, start, end)
    }
    pub fn overlaps_segment(self, start: Vec2, end: Vec2) -> bool {
        self.intersect_segment(start, end).is_some()
    }
    pub fn from_center_dims(center: Vec2, dimensions: Vec2) -> Self {
        let h = dimensions * 0.5;
        Self {
            min: center - h,
            max: center + h,
        }
    }
    pub fn from_center_radius(center: Vec2, radius: f32) -> Self {
        Self {
            min: center - Vec2::splat(radius),
            max: center + Vec2::splat(radius),
        }
    }
    pub fn center(self) -> Vec2 {
        (self.min + self.max) * 0.5
    }
    pub fn half_dims(self) -> Vec2 {
        (self.max - self.min) * 0.5
    }
    pub fn area(self) -> f32 {
        (self.max.y - self.min.y) * (self.max.x - self.min.x)
    }
    pub fn contains(self, p: Vec2) -> bool {
        self.min.x <= p.x && p.x <= self.max.x && self.min.y <= p.y && p.y <= self.max.y
    }
    pub fn intersects(self, b: Self) -> bool {
        // 原x轴是两个排除判断，而y轴是正向比较；NaN时不能简化为同一Vec比较。
        !(self.max.x < b.min.x || b.max.x < self.min.x)
            && self.min.y <= b.max.y
            && b.min.y <= self.max.y
    }
    /// Returns the signed slab entrance parameter, without testing the exit.
    /// A parallel ray outside a slab returns positive infinity.
    pub fn intersect_ray(self, ray: Ray2) -> f32 {
        let Some((x, _)) = slab_interval(self.min.x, self.max.x, ray.origin.x, ray.direction.x)
        else {
            return f32::INFINITY;
        };
        let Some((y, _)) = slab_interval(self.min.y, self.max.y, ray.origin.y, ray.direction.y)
        else {
            return f32::INFINITY;
        };
        x.max(y)
    }
    pub fn max_extent(self) -> f32 {
        let d = self.max - self.min;
        d.x.max(d.y)
    }
    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }
    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }
    /// 输入是尺寸增量，每侧扩一半；不排序、不钳制负值。
    pub fn expanded(self, dimensions: Vec2) -> Self {
        let h = dimensions * 0.5;
        Self {
            min: self.min - h,
            max: self.max + h,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb3 {
    pub min: Vec3,
    pub max: Vec3,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RayBoxIntersection {
    NoHit,
    Inside { enter: f32, exit: f32 },
    Hit { enter: f32, exit: f32 },
}
impl RayBoxIntersection {
    pub fn is_hit(self) -> bool {
        !matches!(self, Self::NoHit)
    }
}
impl Aabb3 {
    /// B Mesh::aabb_bounds inlines the first-point initialization. This differs
    /// from Aabb2 for infinite first points. Empty input uses finite extrema.
    pub fn from_points(points: impl IntoIterator<Item = Vec3>) -> Self {
        let mut points = points.into_iter();
        let Some(first) = points.next() else {
            return Self {
                min: Vec3::splat(f32::MAX),
                max: Vec3::splat(f32::MIN),
            };
        };
        let mut bounds = Self {
            min: first,
            max: first,
        };
        for point in points {
            bounds.expand_to_contain_point(point);
        }
        bounds
    }
    pub fn expand_to_contain_point(&mut self, p: Vec3) {
        self.min = self.min.min(p);
        self.max = self.max.max(p)
    }
    pub fn expand(&mut self, dimensions: Vec3) -> Self {
        let h = dimensions * 0.5;
        self.min -= h;
        self.max += h;
        *self
    }
    pub fn center(self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
    pub fn extent(self) -> Vec3 {
        self.max - self.min
    }
    /// Scales both endpoints and sorts them to support reflected axes.
    pub fn scaled(self, scale: Vec3) -> Self {
        let a = self.min * scale;
        let b = self.max * scale;
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }
    pub fn translated(self, delta: Vec3) -> Self {
        Self {
            min: self.min + delta,
            max: self.max + delta,
        }
    }
    pub fn inclusive_contains_point(self, p: Vec3) -> bool {
        (self.min.cmple(p) & p.cmple(self.max)).all()
    }
    fn strict_contains(self, p: Vec3) -> bool {
        (self.min.cmplt(p) & p.cmplt(self.max)).all()
    }
    fn ray_interval(self, ray: Ray) -> Option<(f32, f32)> {
        ray_interval(
            self.min.to_array(),
            self.max.to_array(),
            ray.origin.to_array(),
            ray.direction.to_array(),
        )
    }
    /// Returns the first segment point inside the closed box.
    pub fn intersect_segment(self, start: Vec3, end: Vec3) -> Option<Vec3> {
        if self.inclusive_contains_point(start) {
            return Some(start);
        }
        let ray = Ray {
            origin: start,
            direction: end - start,
        };
        let (enter, exit) = self.ray_interval(ray)?;
        if enter.is_finite() && enter <= exit && (0.0..=1.0).contains(&enter) {
            Some(start + ray.direction * enter)
        } else {
            None
        }
    }
    /// Classifies a nonzero ray; starts on a face count as a hit at zero.
    pub fn classify_ray_intersection(self, ray: Ray) -> RayBoxIntersection {
        if ray.direction == Vec3::ZERO {
            return RayBoxIntersection::NoHit;
        }
        let Some((enter, exit)) = self.ray_interval(ray) else {
            return RayBoxIntersection::NoHit;
        };
        if exit < 0.0 {
            return RayBoxIntersection::NoHit;
        }
        if self.strict_contains(ray.origin) {
            RayBoxIntersection::Inside { enter, exit }
        } else {
            RayBoxIntersection::Hit {
                enter: enter.max(0.0),
                exit,
            }
        }
    }
}
/// B 0x14123ffc0: all four comparisons are strict; this private picker
/// predicate intentionally differs from the public closed Aabb2::contains.
pub fn is_inside_bbx(point: Vec2, min: Vec2, max: Vec2) -> bool {
    min.x < point.x && point.x < max.x && min.y < point.y && point.y < max.y
}
/// Returns the first segment point inside the closed 2D box.
pub fn intersection_aabb_segment2(min: Vec2, max: Vec2, start: Vec2, end: Vec2) -> Option<Vec2> {
    if (Aabb2 { min, max }).contains(start) {
        return Some(start);
    }
    let direction = end - start;
    let (enter, exit) = ray_interval(
        min.to_array(),
        max.to_array(),
        start.to_array(),
        direction.to_array(),
    )?;
    if enter.is_finite() && enter <= exit && (0.0..=1.0).contains(&enter) {
        Some(start + direction * enter)
    } else {
        None
    }
}

// A parallel ray contributes the whole interval when it lies in the slab.
// Handling it explicitly avoids the NaN produced by 0 / 0 on a box face.
fn slab_interval(min: f32, max: f32, origin: f32, direction: f32) -> Option<(f32, f32)> {
    if !(min.is_finite() && max.is_finite() && origin.is_finite() && direction.is_finite())
        || min > max
    {
        return None;
    }
    if direction == 0.0 {
        return (min <= origin && origin <= max).then_some((f32::NEG_INFINITY, f32::INFINITY));
    }
    let a = (min - origin) / direction;
    let b = (max - origin) / direction;
    Some((a.min(b), a.max(b)))
}

fn ray_interval<const N: usize>(
    min: [f32; N],
    max: [f32; N],
    origin: [f32; N],
    direction: [f32; N],
) -> Option<(f32, f32)> {
    let mut enter = f32::NEG_INFINITY;
    let mut exit = f32::INFINITY;
    for axis in 0..N {
        let (near, far) = slab_interval(min[axis], max[axis], origin[axis], direction[axis])?;
        enter = enter.max(near);
        exit = exit.min(far);
        if enter > exit {
            return None;
        }
    }
    Some((enter, exit))
}
