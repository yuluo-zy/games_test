//! 原装饰网格库使用的描边载荷；此处仅恢复真实拥有类型。
//!
//! 原140b33b00与两条四表插入链分别证明String、Vec<Vec3>和半径载荷。
//! 碰撞描边按原476B真实变换与debug请求显示；下游图元工厂需另行完整恢复。

use glam::Vec3;

/// 原主描边的四种载荷；显式None与库中没有exact key不同。
#[derive(Clone, Debug, PartialEq)]
pub enum DecoratorOutline {
    Mesh(String),
    Lines(Vec<Vec3>),
    Sphere(f32),
    None,
}

/// 原碰撞描边只保存线点、球半径或显式None。
#[derive(Clone, Debug, PartialEq)]
pub enum DecoratorCollisionOutline {
    Lines(Vec<Vec3>),
    Sphere(f32),
    None,
}

impl DecoratorCollisionOutline {
    /// B140b33920/476B：线点使用完整仿射变换，球仅使用平移且半径不缩放。
    /// 显式None没有显示副作用；空线点仍须提交原调试线请求。
    pub fn display(
        &self,
        transform: &glam::Affine3A,
        stream: &mut crate::systems::ui::signifiers::signifier_create::UiSignifierStream,
        dashed: bool,
    ) {
        use crate::utils::debug_viz::{DebugColor, DebugLine, DebugSphere};
        match self {
            Self::Lines(points) => {
                let points = collision_outline_points(points, transform);
                DebugLine { points, colors: None, color: DebugColor::White,
                    line_width: 2.0, ui_pass: true, dashed }.show(stream);
            }
            Self::Sphere(radius) => {
                DebugSphere { color: DebugColor::White, radius: *radius,
                    center: transform.translation.into(), alpha: 1.0, solid: true }.show(stream);
            }
            Self::None => {}
        }
    }
}

fn collision_outline_points(points: &[Vec3], transform: &glam::Affine3A) -> Vec<Vec3> {
    let x = transform.matrix3.x_axis;
    let y = transform.matrix3.y_axis;
    let z = transform.matrix3.z_axis;
    points.iter().map(|point| {
        // 原b33a4f..5f先y+x，再z加该和，最后平移；不借机滤除非有限输入。
        (z * point.z + (y * point.y + x * point.x) + transform.translation).into()
    }).collect()
}

#[cfg(test)]
mod collision_display_tests {
    use super::*;
    use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
    use glam::{Affine3A, Vec3A};

    #[test]
    fn full_affine_basis_preserves_point_order_and_nonuniform_scale() {
        let transform = Affine3A::from_cols(Vec3A::new(2.0, 1.0, 0.0),
            Vec3A::new(0.0, 3.0, 1.0), Vec3A::new(1.0, 0.0, 4.0), Vec3A::new(5.0, 6.0, 7.0));
        assert_eq!(collision_outline_points(&[Vec3::new(1.0, 2.0, 3.0), Vec3::ZERO], &transform),
            [Vec3::new(10.0, 13.0, 21.0), Vec3::new(5.0, 6.0, 7.0)]);
    }

    #[test]
    fn explicit_none_has_no_factory_and_nonfinite_points_are_not_dropped() {
        let mut stream = UiSignifierStream::default();
        DecoratorCollisionOutline::None.display(&Affine3A::IDENTITY, &mut stream, true);
        assert!(stream.take_entries().is_empty());
        let points = collision_outline_points(&[Vec3::new(f32::NAN, 0.0, 0.0)], &Affine3A::IDENTITY);
        assert_eq!(points.len(), 1);
        assert!(points[0].is_nan());
    }

    #[test]
    fn empty_lines_still_submit_original_factory_request() {
        let mut stream = UiSignifierStream::default();
        DecoratorCollisionOutline::Lines(Vec::new()).display(&Affine3A::IDENTITY, &mut stream, false);
        // 当前旧DebugLine的warn+skip不能满足原调用；依赖其完整工厂恢复后实跑。
        assert_eq!(stream.take_entries().len(), 1);
    }
}
