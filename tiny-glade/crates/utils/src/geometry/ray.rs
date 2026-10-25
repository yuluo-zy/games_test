//! World-space rays and screen-space unprojection.
use glam::{Mat4, Vec2, Vec3};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray2 {
    pub origin: Vec2,
    pub direction: Vec2,
}
impl Ray {
    /// Unprojects pixel coordinates from a top-left origin using reverse-Z.
    /// The ray starts on the near plane. Viewport dimensions must be positive.
    pub fn from_mouse_ss(
        mouse: Vec2,
        viewport: Vec2,
        inverse_projection: Mat4,
        inverse_view: Mat4,
    ) -> Self {
        assert!(
            viewport.is_finite() && viewport.cmpgt(Vec2::ZERO).all(),
            "viewport dimensions must be finite and positive"
        );
        let inverse = inverse_view * inverse_projection;
        let x = mouse.x / viewport.x;
        let y = mouse.y / viewport.y;
        let xy = inverse.y_axis * (1.0 - y - y) + inverse.x_axis * (x + x - 1.0);
        let near = inverse.z_axis + xy + inverse.w_axis;
        let far = inverse.z_axis * 0.01 + xy + inverse.w_axis;
        let origin = (near / near.w).truncate();
        let end = (far / far.w).truncate();
        Self {
            origin,
            direction: (end - origin).normalize(),
        }
    }
    pub fn xz(self) -> Ray2 {
        Ray2 {
            origin: Vec2::new(self.origin.x, self.origin.z),
            direction: Vec2::new(self.direction.x, self.direction.z),
        }
    }
}
