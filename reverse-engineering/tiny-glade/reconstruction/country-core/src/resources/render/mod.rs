//! `resources/render/mod.rs` -- resources::render 族（A3 res_render）。
//!
//! 原版 `country_core::resources::render::*` 的直属过程只有 `Affine3Packed` 的 4 个小函数
//! （0x14089d4d0 / 0x14089d500 / 0x14089d550 / 0x14089d560），其余在子模块里。
//! `lantern` 在原版里只有泛型单态化实例（`LanternSsbo` 的 ssbo/pass 访问器），没有自有过程，
//! 因而没有对应的源文件可核对。

use glam::{Affine3A, Mat3, Vec3};

pub mod mesh_atlas_library;
pub mod spatial_resolve_offsets;

/// 紧凑仿射变换（O：48 B = 12 个 f32；列主序 3 个旋转列 + 平移列，无 SIMD 填充）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Affine3Packed {
    /// `[x_axis, y_axis, z_axis, translation]`。
    pub cols: [Vec3; 4],
}

impl Affine3Packed {
    /// O 0x14089d4d0：`Mat3`（9 个连续 f32）+ 平移。
    pub fn from_mat3_translation(m: &Mat3, t: &Vec3) -> Self {
        Self { cols: [m.x_axis, m.y_axis, m.z_axis, *t] }
    }

    /// O 0x14089d500：四列直接存入。
    pub fn from_cols(x: Vec3, y: Vec3, z: Vec3, w: Vec3) -> Self {
        Self { cols: [x, y, z, w] }
    }

    /// O 0x14089d550：`translation.y += y`（偏移 0x28）。
    pub fn add_y(&mut self, y: f32) {
        self.cols[3].y += y;
    }
}

impl From<Affine3A> for Affine3Packed {
    /// O impl$1::from 0x14089d560：丢掉 Vec3A 的填充。
    fn from(a: Affine3A) -> Self {
        Self { cols: [a.matrix3.x_axis.into(), a.matrix3.y_axis.into(), a.matrix3.z_axis.into(), a.translation.into()] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats(a: &Affine3Packed) -> [f32; 12] {
        unsafe { core::mem::transmute::<Affine3Packed, [f32; 12]>(*a) }
    }

    #[test]
    fn layout_is_twelve_floats() {
        assert_eq!(core::mem::size_of::<Affine3Packed>(), 48);
    }

    #[test]
    fn from_mat3_translation_layout() {
        let m = Mat3::from_cols(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.), Vec3::new(7., 8., 9.));
        let a = Affine3Packed::from_mat3_translation(&m, &Vec3::new(10., 11., 12.));
        assert_eq!(floats(&a), [1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
    }

    #[test]
    fn from_cols_and_add_y() {
        let mut a = Affine3Packed::from_cols(Vec3::X, Vec3::Y, Vec3::Z, Vec3::new(1., 2., 3.));
        a.add_y(0.5);
        assert_eq!(floats(&a)[9..], [1., 2.5, 3.]);
        assert_eq!(floats(&a)[..9], [1., 0., 0., 0., 1., 0., 0., 0., 1.]);
    }

    #[test]
    fn from_affine3a_drops_padding() {
        let aff = Affine3A::from_mat3_translation(
            Mat3::from_cols(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.), Vec3::new(7., 8., 9.)),
            Vec3::new(10., 11., 12.),
        );
        assert_eq!(floats(&Affine3Packed::from(aff)), [1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
    }
}
