//! 三维编辑滑块的比例屏幕坐标与透视射线投影（B 原指令恢复）。
use glam::{Mat4, Vec2, Vec3, Vec4};
use utils::geometry::Ray;

const MIN_PROJECTED_LENGTH_SQUARED: f32 = 1e-20;
const MIN_CLIP_WEIGHT: f32 = 1e-10;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalScreenSpace {
    pub size: Vec2,
    pub aspect: f32,
    pub unit: f32,
}
impl ProportionalScreenSpace {
    /// 0x140ac8420：直接转换 u32，零尺寸也保留；原作没有一像素兜底。
    pub fn from_window_size(size: [u32; 2]) -> Self {
        let size = Vec2::new(size[0] as f32, size[1] as f32);
        Self {
            size,
            aspect: size.x / size.y,
            unit: 1.0,
        }
    }
    /// 0x140ac8450 调用原 mouse_ss_to_cs；Y 必须先做 (1-y)/height。
    pub fn transform_screen_space(&self, screen_x: f32, screen_y: f32) -> Vec2 {
        let x = screen_x / self.size.x;
        let y = (1.0 - screen_y) / self.size.y;
        Vec2::new((x + x + -1.0) * self.aspect, (y + y + 1.0) * self.unit)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalScreenSpaceCoord(pub Vec2);

/// 原两级变换均把输入看作三维点，第二级不传播第一级生成的 w。
/// 保持 y+x、再加 z、再加平移的浮点运算顺序，不能合成矩阵重排。
#[inline]
fn transform_point(matrix: &Mat4, point: Vec3) -> Vec4 {
    ((matrix.y_axis * point.y + matrix.x_axis * point.x) + matrix.z_axis * point.z) + matrix.w_axis
}
impl ProportionalScreenSpaceCoord {
    /// 0x140ac84a0：屏幕参数夹紧、逆 w 插值，再进行端点加权。
    /// 负 w、零 w 和退化投影依照原浮点运算与两个分母下限处理。
    pub fn project_onto_ray(
        self,
        screen: &ProportionalScreenSpace,
        world_to_view: &Mat4,
        view_to_clip: &Mat4,
        ray: Ray,
        range: [f32; 2],
    ) -> Vec3 {
        let [minimum, maximum] = range;
        assert!(minimum <= maximum, "投影范围逆序或包含 NaN：{range:?}");
        let endpoint = ray.origin + ray.direction;
        let near = transform_point(
            view_to_clip,
            transform_point(world_to_view, ray.origin).truncate(),
        );
        let far = transform_point(
            view_to_clip,
            transform_point(world_to_view, endpoint).truncate(),
        );
        let scale = Vec2::new(screen.aspect, screen.unit);
        let near_screen = near.truncate().truncate() / near.w * scale;
        let far_screen = far.truncate().truncate() / far.w * scale;
        let direction = far_screen - near_screen;
        let offset = self.0 - near_screen;
        // 对应原 addss 的 y+x 次序；max 的右操作数是固定 epsilon。
        let numerator = offset.y * direction.y + offset.x * direction.x;
        let denominator = (direction.y * direction.y + direction.x * direction.x)
            .max(MIN_PROJECTED_LENGTH_SQUARED);
        let t = numerator / denominator;
        // 两次有序比较保留 NaN 参数与相等时的原符号零，不替换非法坐标。
        let t = if minimum > t { minimum } else { t };
        let t = if maximum < t { maximum } else { t };
        let far_weight = 1.0 / far.w;
        let numerator = utils::scalar::lerp(0.0 / near.w, far_weight, t);
        let denominator = utils::scalar::lerp(1.0 / near.w, far_weight, t).max(MIN_CLIP_WEIGHT);
        let weight = numerator / denominator;
        endpoint * weight + ray.origin * (1.0 - weight)
    }

    /// 保留当前调用接口的显式检查适配；仅非法范围返回 None。
    /// 几何结果（含非有限值）由同一个原算法产生，不再退回射线起点。
    pub fn try_project_onto_ray(
        self,
        screen: &ProportionalScreenSpace,
        world_to_view: &Mat4,
        view_to_clip: &Mat4,
        ray: Ray,
        range: [f32; 2],
    ) -> Option<Vec3> {
        (range[0] <= range[1])
            .then(|| self.project_onto_ray(screen, world_to_view, view_to_clip, ray, range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_bits(actual: f32, expected: u32, context: &str) {
        if f32::from_bits(expected).is_nan() {
            // x86 的无效除法与 ARM 的默认 NaN 符号/载荷不同，不当作业务数值。
            assert!(
                actual.is_nan(),
                "{context}: {actual:?} vs NaN({expected:08x})"
            );
        } else {
            assert_eq!(
                actual.to_bits(),
                expected,
                "{context}: {actual:?} vs {:?}",
                f32::from_bits(expected)
            );
        }
    }
    #[test]
    fn native_window_and_transform_bit_fixtures() {
        let windows: Vec<([u32; 2], [u32; 4])> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-raise-projection-window.ron"
        ))
        .unwrap();
        assert_eq!(std::mem::size_of::<ProportionalScreenSpace>(), 16);
        assert_eq!(windows.len(), 8);
        for (window, expected) in windows {
            let actual = ProportionalScreenSpace::from_window_size(window);
            for (value, bits) in [actual.size.x, actual.size.y, actual.aspect, actual.unit]
                .into_iter()
                .zip(expected)
            {
                compare_bits(value, bits, &format!("window {window:?}"));
            }
        }
        let transforms: Vec<([u32; 4], [u32; 2], [u32; 2])> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-raise-projection-transform.ron"
        ))
        .unwrap();
        assert_eq!(transforms.len(), 56);
        for (screen, cursor, expected) in transforms {
            let [width, height, aspect, unit] = screen.map(f32::from_bits);
            let screen = ProportionalScreenSpace {
                size: Vec2::new(width, height),
                aspect,
                unit,
            };
            let actual =
                screen.transform_screen_space(f32::from_bits(cursor[0]), f32::from_bits(cursor[1]));
            for (value, bits) in actual.to_array().into_iter().zip(expected) {
                compare_bits(value, bits, &format!("transform {cursor:?}"));
            }
        }
    }
    #[test]
    fn native_full_projection_bit_fixture_and_panic_branches() {
        type Row = (
            [u32; 4],
            [u32; 2],
            [u32; 16],
            [u32; 16],
            [u32; 3],
            [u32; 3],
            [u32; 2],
            bool,
            [u32; 3],
        );
        let rows: Vec<Row> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-raise-projection-ray.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 538);
        for (
            index,
            (screen, cursor, view, projection, origin, direction, limits, native_panic, expected),
        ) in rows.into_iter().enumerate()
        {
            let [width, height, aspect, unit] = screen.map(f32::from_bits);
            let screen = ProportionalScreenSpace {
                size: Vec2::new(width, height),
                aspect,
                unit,
            };
            let coordinate =
                ProportionalScreenSpaceCoord(Vec2::from_array(cursor.map(f32::from_bits)));
            let view = Mat4::from_cols_array(&view.map(f32::from_bits));
            let projection = Mat4::from_cols_array(&projection.map(f32::from_bits));
            let ray = Ray {
                origin: Vec3::from_array(origin.map(f32::from_bits)),
                direction: Vec3::from_array(direction.map(f32::from_bits)),
            };
            let result = std::panic::catch_unwind(|| {
                coordinate.project_onto_ray(
                    &screen,
                    &view,
                    &projection,
                    ray,
                    limits.map(f32::from_bits),
                )
            });
            assert_eq!(result.is_err(), native_panic, "case {index} panic branch");
            if let Ok(actual) = result {
                for (axis, (value, bits)) in actual.to_array().into_iter().zip(expected).enumerate()
                {
                    compare_bits(value, bits, &format!("case {index} axis {axis}"));
                }
            }
        }
    }
}
