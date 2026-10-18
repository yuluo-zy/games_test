//! 三维编辑滑块的坐标转换与透视校正。矩阵运算复用 glam。

use glam::{Mat4, Vec2, Vec3};
use utils::geometry::Ray;

const MIN_PROJECTED_LENGTH_SQUARED: f32 = 1e-20;
const MIN_CLIP_WEIGHT: f32 = 1e-10;

/// 比例屏幕空间：X 乘宽高比，Y 保持单位尺度。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalScreenSpace {
    pub size: Vec2,
    pub aspect: f32,
    pub unit: f32,
}

impl ProportionalScreenSpace {
    /// 最小化窗口的零尺寸按 1 像素处理，避免产生无穷宽高比。
    pub fn from_window_size(size: [u32; 2]) -> Self {
        let size = Vec2::new(size[0].max(1) as f32, size[1].max(1) as f32);
        Self {
            size,
            aspect: size.x / size.y,
            unit: 1.0,
        }
    }

    /// 像素原点在左上；屏幕中心映射到零点，上方为正 Y。
    pub fn transform_screen_space(&self, screen_x: f32, screen_y: f32) -> Vec2 {
        let normalized = Vec2::new(screen_x, screen_y) / self.size;
        Vec2::new(
            (normalized.x * 2.0 - 1.0) * self.aspect,
            (1.0 - normalized.y * 2.0) * self.unit,
        )
    }
}

/// 已转换的比例屏幕坐标。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalScreenSpaceCoord(pub Vec2);

impl ProportionalScreenSpaceCoord {
    /// 投影失败时保持射线起点，避免无效窗口或相机参数污染编辑状态。
    /// 新调用方可使用 try_project_onto_ray 区分投影失败。
    pub fn project_onto_ray(
        self,
        screen: &ProportionalScreenSpace,
        world_to_view: &Mat4,
        view_to_clip: &Mat4,
        ray: Ray,
        range: [f32; 2],
    ) -> Vec3 {
        assert!(range[0] <= range[1], "投影范围逆序或包含 NaN：{range:?}");
        self.try_project_onto_ray(screen, world_to_view, view_to_clip, ray, range)
            .unwrap_or(ray.origin)
    }

    /// 将光标投影到射线的屏幕方向，再使用裁剪 w 校正为世界空间插值参数。
    /// range 限制的是屏幕参数；零投影长度保持起点，视点后方或奇异透视返回 None。
    pub fn try_project_onto_ray(
        self,
        screen: &ProportionalScreenSpace,
        world_to_view: &Mat4,
        view_to_clip: &Mat4,
        ray: Ray,
        range: [f32; 2],
    ) -> Option<Vec3> {
        let [minimum, maximum] = range;
        if !(minimum.is_finite() && maximum.is_finite() && minimum <= maximum)
            || !self.0.is_finite()
            || !ray.origin.is_finite()
            || !ray.direction.is_finite()
            || !screen.aspect.is_finite()
            || !screen.unit.is_finite()
        {
            return None;
        }
        let near_clip = *view_to_clip * (*world_to_view * ray.origin.extend(1.0));
        let far_clip = *view_to_clip * (*world_to_view * (ray.origin + ray.direction).extend(1.0));
        if !near_clip.is_finite()
            || !far_clip.is_finite()
            || near_clip.w <= MIN_CLIP_WEIGHT
            || far_clip.w <= MIN_CLIP_WEIGHT
        {
            return None;
        }
        let screen_scale = Vec2::new(screen.aspect, screen.unit);
        let near_screen = near_clip.truncate().truncate() / near_clip.w * screen_scale;
        let far_screen = far_clip.truncate().truncate() / far_clip.w * screen_scale;
        let projected_direction = far_screen - near_screen;
        let length_squared = projected_direction.length_squared();
        if !length_squared.is_finite() {
            return None;
        }
        if length_squared <= MIN_PROJECTED_LENGTH_SQUARED {
            return Some(ray.origin);
        }
        let screen_parameter = ((self.0 - near_screen).dot(projected_direction) / length_squared)
            .clamp(minimum, maximum);
        let near_weight = (1.0 - screen_parameter) / near_clip.w;
        let far_weight = screen_parameter / far_clip.w;
        let total_weight = near_weight + far_weight;
        // 外推可能跨过透视奇点；负权重对应相机背后的另一条分支。
        if !total_weight.is_finite() || total_weight <= MIN_CLIP_WEIGHT {
            return None;
        }
        let world_parameter = far_weight / total_weight;
        let position = ray.origin + ray.direction * world_parameter;
        if !position.is_finite() {
            return None;
        }
        let clip = *view_to_clip * (*world_to_view * position.extend(1.0));
        (clip.is_finite() && clip.w > MIN_CLIP_WEIGHT).then_some(position)
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
    fn screen_coordinates_have_a_centered_origin_and_upward_y() {
        let pss = ProportionalScreenSpace::from_window_size([200, 100]);
        // 屏幕四分之一高度映射到 y=0.5，屏幕顶部映射到 y=1。
        let v = pss.transform_screen_space(100.0, 25.0);
        assert_eq!(v.x, 0.0);
        assert!((v.y - 0.5).abs() < 1e-6, "{v:?}");
        let v = pss.transform_screen_space(0.0, 0.0);
        assert_eq!(v.x, -2.0);
        assert!((v.y - 1.0).abs() < 1e-6);
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
    #[should_panic(expected = "投影范围逆序")]
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

    #[test]
    fn perspective_extrapolation_rejects_the_horizon_and_the_branch_behind_camera() {
        use crate::systems::ui::resources::slider3d::{Slider3dMatrices, Slider3dState};

        let projection = Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 100.0);
        let ray = ray(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.2, 0.0, -1.0));
        // 两个采样点均在相机前方；屏幕 x=0.2 是世界直线的透视渐近线。
        for screen_x in [0.2, 0.25] {
            let coord = ProportionalScreenSpaceCoord(Vec2::new(screen_x, 0.0));
            assert_eq!(
                coord.try_project_onto_ray(
                    &unit_pss(),
                    &Mat4::IDENTITY,
                    &projection,
                    ray,
                    [-1e10, 1e10]
                ),
                None
            );
        }
        // 奇点之前的正常外推仍有效，避免修复时禁用全部屏幕区间外推。
        let position = ProportionalScreenSpaceCoord(Vec2::new(0.15, 0.0))
            .try_project_onto_ray(
                &unit_pss(),
                &Mat4::IDENTITY,
                &projection,
                ray,
                [-1e10, 1e10],
            )
            .unwrap();
        assert!(position.is_finite() && position.z < -2.0);
        let clip = projection * position.extend(1.0);
        assert!(clip.w > 0.0);
        assert!((clip.x / clip.w - 0.15).abs() < 1e-6);

        let matrices = Slider3dMatrices {
            second: projection,
            ..Default::default()
        };
        let slider = Slider3dState {
            origin: ray.origin,
            direction: ray.direction,
        };
        assert_eq!(
            slider.project_cursor_onto_line(Vec2::new(62.5, 50.0), [100, 100], &matrices),
            ray.origin
        );
    }

    #[test]
    fn viewport_center_and_invalid_perspective_are_safe() {
        let screen = ProportionalScreenSpace::from_window_size([0, 0]);
        assert_eq!(screen.transform_screen_space(0.5, 0.5), Vec2::ZERO);
        let ray = ray(Vec3::ONE, Vec3::X);
        assert_eq!(
            ProportionalScreenSpaceCoord(Vec2::ZERO).try_project_onto_ray(
                &screen,
                &Mat4::IDENTITY,
                &Mat4::ZERO,
                ray,
                [0.0, 1.0]
            ),
            None
        );
    }
}
