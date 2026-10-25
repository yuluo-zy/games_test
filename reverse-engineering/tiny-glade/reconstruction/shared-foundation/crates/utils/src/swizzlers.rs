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
    pub fn len(self) -> usize {
        self.source.len()
    }
    pub fn is_empty(self) -> bool {
        self.source.is_empty()
    }
    /// Element access projecting the game's XZ plane. The original projected
    /// axis/stride was not recoverable (the view adapter has no compiled body
    /// in B); the x/z choice follows this module's XZ swizzle family.
    pub fn get(self, index: usize) -> Option<Vec2> {
        self.source
            .get(index)
            .map(|v| Vec2::new(v.x, v.z))
    }
    pub fn get_unchecked(self, index: usize) -> Vec2 {
        let v = self.source[index];
        Vec2::new(v.x, v.z)
    }
    pub fn iter(self) -> impl DoubleEndedIterator<Item = Vec2> + ExactSizeIterator + 'a {
        self.source.iter().map(|v| Vec2::new(v.x, v.z))
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

#[cfg(test)]
mod vec3_slice_view2_tests {
    use super::*;
    fn sample() -> Vec3SliceView2<'static> {
        static PTS: [Vec3; 4] = [
            Vec3::new(1.0, 9.0, 2.0),
            Vec3::new(3.0, 8.0, 4.0),
            Vec3::new(5.0, 7.0, 6.0),
            Vec3::new(7.0, 6.0, 8.0),
        ];
        PTS.as_vec2()
    }
    #[test]
    fn projected_elements_drop_y() {
        let view = sample();
        assert_eq!(view.len(), 4);
        assert!(!view.is_empty());
        assert_eq!(view.get(0), Some(Vec2::new(1.0, 2.0)));
        assert_eq!(view.get(3), Some(Vec2::new(7.0, 8.0)));
        assert_eq!(view.get(4), None);
        assert_eq!(view.get_unchecked(1), Vec2::new(3.0, 4.0));
    }
    #[test]
    fn iterators_yield_xz_projection() {
        let view = sample();
        let collected: Vec<_> = view.iter().collect();
        assert_eq!(
            collected,
            vec![
                Vec2::new(1.0, 2.0),
                Vec2::new(3.0, 4.0),
                Vec2::new(5.0, 6.0),
                Vec2::new(7.0, 8.0),
            ]
        );
        let rev: Vec<_> = view.iter().rev().collect();
        assert_eq!(rev[0], Vec2::new(7.0, 8.0));
        assert_eq!(view.iter().len(), 4);
    }
    #[test]
    fn source_borrow_is_preserved() {
        let view = sample();
        assert_eq!(view.as_source()[2], Vec3::new(5.0, 7.0, 6.0));
    }
}
