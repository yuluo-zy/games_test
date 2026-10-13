//! Private B intersections; no replacement for an external collision library.
use super::{Aabb3, Ray};
use glam::Vec3;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub points: [Vec3; 3],
}
impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { points: [a, b, c] }
    }
    pub fn points_mut(&mut self) -> &mut [Vec3; 3] {
        &mut self.points
    }
    pub fn normal(self) -> Vec3 {
        (self.points[1] - self.points[0])
            .cross(self.points[2] - self.points[1])
            .normalize()
    }
    pub fn aabb(self) -> Aabb3 {
        Aabb3 {
            min: self.points[0].min(self.points[1]).min(self.points[2]),
            max: self.points[0].max(self.points[1]).max(self.points[2]),
        }
    }
    pub fn intersect_triangle(
        self,
        other: Self,
    ) -> super::triangle_triangle::TriangleTriangleIntersection {
        super::triangle_triangle::triangles_intersect(self.points, other.points)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}
impl Sphere {
    pub fn as_f32x4(self) -> [f32; 4] {
        [self.center.x, self.center.y, self.center.z, self.radius]
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphereIntersection {
    pub near: f32,
    pub far: f32,
}
impl SphereIntersection {
    /// Original returns minimum even when both intersections are behind origin.
    pub fn closest_intersection_distance(self) -> f32 {
        if self.near.is_nan() {
            self.far
        } else if self.near <= self.far {
            self.near
        } else {
            self.far
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TriangleIntersection {
    pub distance: f32,
    pub u: f32,
    pub v: f32,
}
/// B 0x14123afa0: two-sided triangle; inclusive barycentric boundaries;
/// determinant abs >= epsilon but distance strictly > epsilon.
pub fn ray_triangle_intersection(ray: Ray, triangle: Triangle) -> Option<TriangleIntersection> {
    let [a, b, c] = triangle.points;
    let e1 = b - a;
    let e2 = c - a;
    let p = ray.direction.cross(e2);
    let determinant = e1.z * p.z + e1.y * p.y + e1.x * p.x;
    const EPS: f32 = 1e-5;
    if determinant.abs() < EPS || determinant.is_nan() {
        return None;
    }
    let inv = 1.0 / determinant;
    let t = ray.origin - a;
    let u = (p.z * t.z + p.y * t.y + p.x * t.x) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = t.cross(e1);
    let v = (ray.direction.x * q.x + ray.direction.y * q.y + ray.direction.z * q.z) * inv;
    if !(v >= 0.0 && u + v <= 1.0) {
        return None;
    }
    let distance = inv * (e2.x * q.x + e2.y * q.y + e2.z * q.z);
    if distance > EPS {
        Some(TriangleIntersection { distance, u, v })
    } else {
        None
    }
}
/// B 0x14123b230: unnormalized rays, tangency accepted, no positive-root guard.
pub fn ray_sphere_intersection(ray: Ray, sphere: Sphere) -> Option<SphereIntersection> {
    let d = ray.direction;
    let a = d.x * d.x + d.y * d.y + d.z * d.z;
    if a == 0.0 {
        return None;
    }
    let o = ray.origin - sphere.center;
    let dot = d.x * o.x + d.y * o.y + d.z * o.z;
    let b = dot + dot;
    let discriminant =
        (-4.0 * a) * ((o.x * o.x + o.y * o.y + o.z * o.z) - sphere.radius * sphere.radius) + b * b;
    if discriminant >= 0.0 {
        let root = discriminant.sqrt();
        Some(SphereIntersection {
            near: (-b - root) / (a + a),
            far: (root - b) / (a + a),
        })
    } else {
        None
    }
}
