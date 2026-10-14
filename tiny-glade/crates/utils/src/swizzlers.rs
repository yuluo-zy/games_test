//! Game coordinate adapters; construction and storage remain original glam.
use glam::{Vec2, Vec3, Vec3Swizzles};

/// Owned XZ projection used by B's Curve::reinterpret_as_curve2/try_to_curve2
/// consumers: 12-byte Vec3 inputs become packed 8-byte Vec2 outputs. The axes
/// are evidenced in the map/collect specialization 0x1402ba9e0; std allocation
/// and glam swizzling are reused. This is distinct from the folded as_vec2
/// identity entry, whose actual receiver and return type are still unresolved.
pub fn project_vec3_slice_xz(source: &[Vec3]) -> Vec<Vec2> {
    source.iter().map(|p| p.xz()).collect()
}

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

/// Safe borrowed transport chosen by the reconstruction. B's folded identity
/// entry only establishes RCX -> RAX; unchanged RDX cannot prove a fat-pointer
/// length or even a slice receiver. The author's as_vec2 type remains unknown.
/// Source storage remains Vec3 rather than an unsafe packed Vec2 slice cast.
#[derive(Clone, Copy, Debug)]
pub struct Vec3SliceView2<'a> {
    source: &'a [Vec3],
}
impl<'a> Vec3SliceView2<'a> {
    pub fn as_source(self) -> &'a [Vec3] {
        self.source
    }
    pub fn len(self) -> usize {
        self.source.len()
    }
    pub fn is_empty(self) -> bool {
        self.source.is_empty()
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
#[path = "oracle/swizzler_tests.rs"]
mod native_oracle;
