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
        // B 0x141280443..485: retain the two original SIMD reduction orders.
        let direction = ray.direction;
        let normal = self.normal;
        let offset = self.point - ray.origin;
        let denominator = normal.z * direction.z
            + (normal.y * direction.y + normal.x * direction.x);
        let numerator = normal.z * offset.z
            + (normal.x * offset.x + normal.y * offset.y);
        let t = numerator / denominator;
        // Native UCOMISS min_t,t; JA rejects only ordered min_t > t.
        // Unordered and positive-infinite results retain the original payload;
        // callers such as PlaneDisc apply their own geometric acceptance.
        if min_t > t {
            None
        } else {
            Some((t, ray.origin + ray.direction * t))
        }
    }
}
