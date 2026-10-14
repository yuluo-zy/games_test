//! B Plane::intersect_ray，0x1412803f0：point@0、normal@12。
use super::Ray;
use glam::Vec3;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}
impl Plane {
    pub fn intersect_ray(self, ray: Ray, min_t: f32) -> Option<(f32, Vec3)> {
        let t = self.normal.dot(self.point - ray.origin) / self.normal.dot(ray.direction);
        // 不另加parallel epsilon或法线归一化；NaN自然不通过正向比较。
        if min_t <= t {
            Some((t, ray.origin + ray.direction * t))
        } else {
            None
        }
    }
}
