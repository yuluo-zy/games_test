//! Game coordinate adapters; construction and storage remain original glam.
use glam::{Vec2, Vec3};

pub trait SwizzlerX0Y {
    fn x0y(self) -> Vec3;
}
impl SwizzlerX0Y for Vec2 {
    fn x0y(self) -> Vec3 {
        Vec3::new(self.x, 0.0, self.y)
    }
}
pub trait SwizzlerXVY {
    fn xvy(self, y: f32) -> Vec3;
}
impl SwizzlerXVY for Vec2 {
    fn xvy(self, y: f32) -> Vec3 {
        Vec3::new(self.x, y, self.y)
    }
}
pub trait SwizzlerXVZ {
    fn xvz(self, y: f32) -> Vec3;
}
impl SwizzlerXVZ for Vec3 {
    fn xvz(self, y: f32) -> Vec3 {
        Vec3::new(self.x, y, self.z)
    }
}
pub trait SwizzlerX0Z {
    fn x0z(self) -> Vec3;
}
impl SwizzlerX0Z for Vec3 {
    fn x0z(self) -> Vec3 {
        Vec3::new(self.x, 0.0, self.z)
    }
}

pub fn x0y(xy: Vec2) -> Vec3 {
    xy.x0y()
}
pub fn xvy(xz: Vec2, y: f32) -> Vec3 {
    xz.xvy(y)
}
pub fn xvz(xyz: Vec3, y: f32) -> Vec3 {
    xyz.xvz(y)
}
pub fn x0z(xyz: Vec3) -> Vec3 {
    xyz.x0z()
}

/// Explicit borrowed transport for B's `[Vec3]::as_vec2` adapter. Its original
/// native entry preserves pointer and length without allocating or repacking.
/// The author's return type/projected consumer API remains unresolved; source
/// storage remains Vec3 rather than an unsafe tightly packed Vec2 slice cast.
#[derive(Clone, Copy, Debug)]
pub struct Vec3SliceView2<'a> {
    source: &'a [Vec3],
}
impl<'a> Vec3SliceView2<'a> {
    pub fn as_source(self) -> &'a [Vec3] {
        self.source
    }
}
pub trait Vec3SliceAs2 {
    fn as_vec2(&self) -> Vec3SliceView2<'_>;
}
impl Vec3SliceAs2 for [Vec3] {
    fn as_vec2(&self) -> Vec3SliceView2<'_> {
        Vec3SliceView2 { source: self }
    }
}

#[cfg(test)]
#[path = "../../../../../evidence/shared-foundation-2026-10-13/utils-base/interface-batch-02/swizzler_tests.rs"]
mod native_oracle;
