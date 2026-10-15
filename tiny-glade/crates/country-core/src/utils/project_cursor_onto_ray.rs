//! project_cursor_onto_ray.rs — 屏幕比例空间与“把光标投回射线”。
//!
//! | 函数 | 地址 | 级别 |
//! |---|---|---|
//! | `ProportionalScreenSpace::from_window_size` | 0x140ac8420 | O |
//! | `ProportionalScreenSpace::transform_screen_space` | 0x140ac8450（helper 0x140c906d0） | O |
//! | `ProportionalScreenSpaceCoord::project_onto_ray` | 0x140ac84a0 | O |
//!
//! 与 DIGEST 不同：`transform_screen_space` 的 helper 0x140c906d0 这次读全了
//! （DIGEST 记为“语义未吃透”），见 [`ProportionalScreenSpace::transform_screen_space`]。

use glam::{Mat4, Vec2, Vec3, Vec4};
use utils::geometry::Ray;

/// `maxss` 的下限常量，按 EXE 位模式（0x142a7baa0 / 0x142abaa00）。
const DOT_FLOOR: f32 = f32::from_bits(0x1e3c_e508);
const WEIGHT_FLOOR: f32 = f32::from_bits(0x2edb_e6ff);

/// 16 B：`{ size: Vec2 @0, aspect: f32 @8, 1.0 @0xc }`（O：0x140ac8420）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalScreenSpace {
    pub size: Vec2,
    pub aspect: f32,
    pub unit: f32,
}

impl ProportionalScreenSpace {
    /// O：0x140ac8420。`u32 → f32` 经 64 位 `cvtsi2ss`（零扩展），`aspect = w / h`。
    pub fn from_window_size(size: [u32; 2]) -> Self {
        let width = size[0] as f32;
        let height = size[1] as f32;
        Self {
            size: Vec2::new(width, height),
            aspect: crate::resources::window::aspect_ratio_observed(size[0], size[1]),
            unit: 1.0,
        }
    }

    /// O：0x140ac8450。helper 0x140c906d0 把 `(a, b)` 换成
    /// `x' = a / size.x * 2 + (-1.0)`、`y' = (1.0 - b) / size.y * 2 + 1.0`
    /// （乘 2 用的是自加；常量 -1.0 在 0x14295a0e4），随后 `x' * aspect`、`y' * 1.0`。
    /// `y'` 的形式照原样保留，没有按“看起来应该是”的公式改写。
    pub fn transform_screen_space(&self, a: f32, b: f32) -> Vec2 {
        let x = a / self.size.x;
        let x = x + x + -1.0;
        let y = (1.0 - b) / self.size.y;
        let y = y + y + 1.0;
        Vec2::new(x * self.aspect, y * self.unit)
    }
}

/// `x' = col0*x + col1*y + col2*z + col3`，加法顺序与 0x140ac84a0 的 `mulps/addps` 序列一致：
/// `((y*col1 + x*col0) + z*col2) + col3`。
fn affine(m: &Mat4, p: Vec3) -> Vec4 {
    m.y_axis * p.y + m.x_axis * p.x + m.z_axis * p.z + m.w_axis
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// 屏幕比例坐标（已经过 `transform_screen_space`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalScreenSpaceCoord(pub Vec2);

impl ProportionalScreenSpaceCoord {
    /// O：0x140ac84a0。
    ///
    /// 射线的近点 = 起点、远点 = 起点 + 方向，各自先过 `first`（原版结构 +0x80 处的矩阵）
    /// 再过 `second`（+0x00 处的矩阵，第二次**不重新取 w**，直接用第一次的 xyz 补 w = 1），
    /// 取 `xy / w` 再乘 `(aspect, 1.0)`。`t = ((c - near)·d) / max(d·d, 1e-20)`，
    /// 限制在 `range`（`min > max` 或含 NaN 时 `f32::clamp` panic）。最后按透视校正插值回世界点：
    /// `num = lerp(0/w_near, 1/w_far, t)`、`den = max(lerp(1/w_near, 1/w_far, t), 1e-10)`、
    /// `s = num / den`，结果 `near * (1 - s) + far * s`。
    pub fn project_onto_ray(
        self,
        pss: &ProportionalScreenSpace,
        first: &Mat4,
        second: &Mat4,
        ray: Ray,
        range: [f32; 2],
    ) -> Vec3 {
        let [min, max] = range;
        // 原版先检查 `min <= max`，不成立走 `f32::clamp` 的 panic。
        assert!(
            min <= max,
            "min > max, or either was NaN. min = {min:?}, max = {max:?}"
        );
        let near = ray.origin;
        let far = Vec3::new(
            ray.direction.x + ray.origin.x,
            ray.direction.y + ray.origin.y,
            ray.direction.z + ray.origin.z,
        );
        let near_clip = affine(second, affine(first, near).truncate());
        let far_clip = affine(second, affine(first, far).truncate());
        let pair = Vec2::new(pss.aspect, pss.unit);
        let near_ndc = Vec2::new(near_clip.x / near_clip.w, near_clip.y / near_clip.w) * pair;
        let far_ndc = Vec2::new(far_clip.x / far_clip.w, far_clip.y / far_clip.w) * pair;
        let to_cursor = self.0 - near_ndc;
        let along = far_ndc - near_ndc;
        let product = to_cursor * along;
        let numerator = product.y + product.x;
        let squared = along * along;
        let denominator = squared.y + squared.x;
        // `maxss denominator, 1e-20`：NaN 取常量。
        let denominator = if denominator > DOT_FLOOR {
            denominator
        } else {
            DOT_FLOOR
        };
        let t = (numerator / denominator).clamp(min, max);

        let inv_far = 1.0 / far_clip.w;
        let numerator = lerp(0.0 / near_clip.w, inv_far, t);
        let denominator = lerp(1.0 / near_clip.w, inv_far, t);
        let denominator = if denominator > WEIGHT_FLOOR {
            denominator
        } else {
            WEIGHT_FLOOR
        };
        let s = numerator / denominator;
        let one_minus_s = 1.0 - s;
        Vec3::new(
            s * far.x + one_minus_s * near.x,
            s * far.y + one_minus_s * near.y,
            s * far.z + one_minus_s * near.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_window_size_is_the_exe_16_byte_record() {
        let pss = ProportionalScreenSpace::from_window_size([1920, 1080]);
        assert_eq!(std::mem::size_of::<ProportionalScreenSpace>(), 16);
        assert_eq!(pss.size, Vec2::new(1920.0, 1080.0));
        assert_eq!(pss.aspect, 1920.0 / 1080.0);
        assert_eq!(pss.unit.to_bits(), 0x3f80_0000);
    }

    #[test]
    fn transform_screen_space_follows_the_helper_exactly() {
        let pss = ProportionalScreenSpace::from_window_size([200, 100]);
        // x' = 100/200*2 - 1 = 0, y' = (1 - 25)/100*2 + 1 = 0.52。
        let v = pss.transform_screen_space(100.0, 25.0);
        assert_eq!(v.x, 0.0);
        assert!((v.y - 0.52).abs() < 1e-6, "{v:?}");
        let v = pss.transform_screen_space(0.0, 0.0);
        assert_eq!(v.x, -2.0);
        assert!((v.y - 1.02).abs() < 1e-6);
    }

    fn ray(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    fn unit_pss() -> ProportionalScreenSpace {
        ProportionalScreenSpace::from_window_size([100, 100])
    }

    #[test]
    fn identity_matrices_project_to_the_cursor_point_along_the_ray() {
        let id = Mat4::IDENTITY;
        let r = ray(Vec3::ZERO, Vec3::X);
        let p = ProportionalScreenSpaceCoord(Vec2::new(0.5, 0.0)).project_onto_ray(
            &unit_pss(),
            &id,
            &id,
            r,
            [0.0, 1.0],
        );
        assert_eq!(p, Vec3::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn t_is_clamped_to_the_range() {
        let id = Mat4::IDENTITY;
        let r = ray(Vec3::ZERO, Vec3::X);
        let c = ProportionalScreenSpaceCoord(Vec2::new(5.0, 0.0));
        assert_eq!(
            c.project_onto_ray(&unit_pss(), &id, &id, r, [0.0, 1.0]),
            Vec3::X
        );
        assert_eq!(
            c.project_onto_ray(&unit_pss(), &id, &id, r, [0.0, 0.25]),
            Vec3::new(0.25, 0.0, 0.0)
        );
        let c = ProportionalScreenSpaceCoord(Vec2::new(-5.0, 0.0));
        assert_eq!(
            c.project_onto_ray(&unit_pss(), &id, &id, r, [0.0, 1.0]),
            Vec3::ZERO
        );
    }

    #[test]
    #[should_panic(expected = "min > max, or either was NaN")]
    fn reversed_range_panics_like_f32_clamp() {
        let id = Mat4::IDENTITY;
        ProportionalScreenSpaceCoord(Vec2::ZERO).project_onto_ray(
            &unit_pss(),
            &id,
            &id,
            ray(Vec3::ZERO, Vec3::X),
            [1.0, 0.0],
        );
    }

    #[test]
    fn perspective_w_changes_the_interpolation_weight() {
        // 第二个矩阵把 w 设为 z + 1：近点 w = 1，远点（z = 1）w = 2。
        let mut second = Mat4::IDENTITY;
        second.z_axis.w = 1.0;
        let id = Mat4::IDENTITY;
        let r = ray(Vec3::ZERO, Vec3::new(2.0, 0.0, 1.0));
        // 远点 ndc.x = 2 / 2 = 1；cursor.x = 0.5 → t = 0.5。
        let p = ProportionalScreenSpaceCoord(Vec2::new(0.5, 0.0)).project_onto_ray(
            &unit_pss(),
            &id,
            &second,
            r,
            [0.0, 1.0],
        );
        // num = lerp(0, 1/2, .5) = .25；den = lerp(1, .5, .5) = .75；s = 1/3。
        let s = 0.25_f32 / 0.75;
        assert!((p.x - s * 2.0).abs() < 1e-6, "{p:?}");
        assert!((p.z - s * 1.0).abs() < 1e-6);
    }

    #[test]
    fn degenerate_ray_hits_the_epsilon_floor_without_nan() {
        let id = Mat4::IDENTITY;
        let p = ProportionalScreenSpaceCoord(Vec2::new(1.0, 1.0)).project_onto_ray(
            &unit_pss(),
            &id,
            &id,
            ray(Vec3::new(1.0, 2.0, 3.0), Vec3::ZERO),
            [0.0, 1.0],
        );
        assert!(p.is_finite());
    }
}
