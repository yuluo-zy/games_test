//! utils/swizzlers.rs — zero-cost 2D->3D swizzles observed in wall geometry paths.

/// Swizzle helpers on `Vec2` (game-owned; math reuse from glam).
pub trait Swizzle2 {
    fn x0y(self) -> glam::Vec3;
    fn xvy(self) -> glam::Vec3;
}
impl Swizzle2 for glam::Vec2 {
    fn x0y(self) -> glam::Vec3 {
        glam::Vec3::new(self.x, 0.0, self.y)
    }
    fn xvy(self) -> glam::Vec3 {
        glam::Vec3::new(self.x, self.y, 0.0)
    }
}
