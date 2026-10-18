//! Ray intersection with an infinite plane.
use super::Ray;
use glam::Vec3;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}
impl Plane {
    pub fn intersect_ray(self, ray: Ray, min_t: f32) -> Option<(f32, Vec3)> {
        let denominator = self.normal.dot(ray.direction);
        // Parallel and coplanar rays have no unique intersection.
        if denominator == 0.0 || !denominator.is_finite() {
            return None;
        }
        let t = self.normal.dot(self.point - ray.origin) / denominator;
        if t.is_finite() && min_t <= t {
            Some((t, ray.origin + ray.direction * t))
        } else {
            None
        }
    }
}
