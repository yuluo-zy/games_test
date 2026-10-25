//! 原B Aabb定义/边界策略，0x14123ffc0..0x141240b10。
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
    /// Reuse the original segment entrance policy, including strict inside starts.
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
    /// 原函数只返回进入参数，不过滤负t、不检查离开参数，也不是bool射线命中。
    pub fn intersect_ray(self, ray: Ray2) -> f32 {
        let a = (self.min - ray.origin) / ray.direction;
        let b = (self.max - ray.origin) / ray.direction;
        a.x.min(b.x).max(a.y.min(b.y))
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
    /// 原缩放直接乘两个端点，负scale不会重新排序。
    pub fn scaled(self, scale: Vec3) -> Self {
        Self {
            min: self.min * scale,
            max: self.max * scale,
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
    fn ray_interval(self, ray: Ray) -> (f32, f32) {
        let a = (self.min - ray.origin) / ray.direction;
        let b = (self.max - ray.origin) / ray.direction;
        let near = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let far = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
        (near.z.max(near.y).max(near.x), far.z.min(far.y).min(far.x))
    }
    pub fn intersect_segment(self, start: Vec3, end: Vec3) -> Option<Vec3> {
        let ray = Ray {
            origin: start,
            direction: end - start,
        };
        let (near, far) = self.ray_interval(ray);
        if near <= far && near <= 1.0 {
            if near >= 0.0 {
                return Some(start + ray.direction * near);
            }
            if self.strict_contains(start) {
                return Some(start);
            }
        }
        None
    }
    pub fn classify_ray_intersection(self, ray: Ray) -> RayBoxIntersection {
        let (near, far) = self.ray_interval(ray);
        if near <= far {
            if near >= 0.0 {
                return RayBoxIntersection::Hit {
                    enter: near,
                    exit: far,
                };
            }
            if self.strict_contains(ray.origin) {
                return RayBoxIntersection::Inside {
                    enter: near,
                    exit: far,
                };
            }
        }
        RayBoxIntersection::NoHit
    }
}
pub fn is_inside_bbx(point: Vec2, min: Vec2, max: Vec2) -> bool {
    Aabb2 { min, max }.contains(point)
}
/// B 0x141240b30. No invented far>=0 test; a strictly interior start is returned
/// unchanged. Starting exactly on a boundary uses the original entrance rule.
pub fn intersection_aabb_segment2(min: Vec2, max: Vec2, start: Vec2, end: Vec2) -> Option<Vec2> {
    let direction = end - start;
    let a = (max - start) / direction;
    let b = (min - start) / direction;
    let enter = a.min(b).max_element();
    let exit = a.max(b).min_element();
    if enter <= exit && enter <= 1.0 {
        if enter >= 0.0 {
            return Some(start + direction * enter);
        }
        if (start.cmpgt(min) & start.cmplt(max)).all() {
            return Some(start);
        }
    }
    None
}
