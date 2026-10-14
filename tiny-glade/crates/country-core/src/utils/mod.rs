pub mod dirty_tiles;
pub mod float_grid;
pub mod index_condenser;
pub mod project_cursor_onto_ray;
pub mod components {
    pub mod transform;
}

use glam::{Mat4, Vec2, Vec3};

/// O：0x140ac8780 `utils::world_space_to_screen_space`。
///
/// 世界点先过 `first`（参数结构 +0x80 处的矩阵，`transform_point3` 形式），再过 `second`
/// （+0x00 处的矩阵，补 w = 1 的 4 分量乘法，**保留 w**），`xy / w` 后
/// `x * 0.5 + 0.5`、`y * 0.5 + 0.5`，返回 `(x * width, (1.0 - y) * height)`。
/// 两个矩阵各自是相机结构里的哪一个字段（I）：+0x80 / +0x00。
pub fn world_space_to_screen_space(
    world: Vec3,
    width: f32,
    height: f32,
    first: &Mat4,
    second: &Mat4,
) -> Vec2 {
    let affine = |m: &Mat4, p: Vec3| m.y_axis * p.y + m.x_axis * p.x + m.z_axis * p.z + m.w_axis;
    let view = affine(first, world);
    let clip = affine(second, view.truncate());
    let x = clip.x / clip.w * 0.5 + 0.5;
    let y = clip.y / clip.w * 0.5 + 0.5;
    Vec2::new(x * width, (1.0 - y) * height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_origin_lands_in_the_screen_centre_with_identity_matrices() {
        let id = Mat4::IDENTITY;
        assert_eq!(
            world_space_to_screen_space(Vec3::ZERO, 200.0, 100.0, &id, &id),
            Vec2::new(100.0, 50.0)
        );
    }

    #[test]
    fn y_is_flipped_and_w_divides() {
        let id = Mat4::IDENTITY;
        // ndc = (1, 1) → 屏幕右上角 (width, 0)。
        assert_eq!(
            world_space_to_screen_space(Vec3::new(1.0, 1.0, 0.0), 200.0, 100.0, &id, &id),
            Vec2::new(200.0, 0.0)
        );
        // 第二个矩阵令 w = 2：ndc 减半。
        let mut second = Mat4::IDENTITY;
        second.w_axis.w = 2.0;
        assert_eq!(
            world_space_to_screen_space(Vec3::new(1.0, 1.0, 0.0), 200.0, 100.0, &id, &second),
            Vec2::new(150.0, 25.0)
        );
    }
}
