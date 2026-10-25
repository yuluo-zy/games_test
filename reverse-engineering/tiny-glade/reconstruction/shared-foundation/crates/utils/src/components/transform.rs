//! B look_at/abs_diff_eq访问40B实例的r@0,t@16,s@28。
//! 恢复字段语义，不固定Rust布局；矩阵转四元数直接复用原glam。
use glam::{Affine3A, Mat3, Mat4, Quat, Vec3};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub t: Vec3,
    pub r: Quat,
    pub s: Vec3,
}
impl Transform {
    /// B PDB inlinee 0x6091 delegates the S/R/T composition to glam.
    pub fn compute_affine(&self) -> Affine3A {
        Affine3A::from_scale_rotation_translation(self.s, self.r, self.t)
    }
    /// B's matrix wrapper uses the same original glam S/R/T constructor.
    pub fn compute_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.s, self.r, self.t)
    }
    /// Local -Z rotated by the stored quaternion; scale is intentionally ignored.
    pub fn forward(&self) -> Vec3 {
        self.r * Vec3::NEG_Z
    }
    /// Local -Z faces target. The original does not replace degenerate inputs.
    /// B 0x140c9ce00; only rotation changes.
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let z = (self.t - target).normalize();
        let x = up.cross(z).normalize();
        let y = z.cross(x);
        self.r = Quat::from_mat3(&Mat3::from_cols(x, y, z));
    }
    /// B 0x140c9d0d0: every component difference must be <= tolerance.
    /// NaNs fail; the original SIMD cmpnle cannot be simplified to `epsilon<d`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.t.abs_diff_eq(other.t, max_abs_diff)
            && self.r.abs_diff_eq(other.r, max_abs_diff)
            && self.s.abs_diff_eq(other.s, max_abs_diff)
    }
}
