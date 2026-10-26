//! 游戏侧工具入口；共享数学、采样和索引算法委托工作区 utils crate。

pub mod child_wall;
pub mod conservative_line_raster;
pub mod debug_viz;
pub mod dirty_tiles;
pub mod float_grid;
pub mod geo;
pub mod index_condenser;
pub mod load_json;
pub mod project_cursor_onto_ray;
pub mod components {
    pub mod transform;
}

/// 复用共享 `utils` crate 的标准哈希入口；用于进程内名字和资源身份。
pub use ::utils::calculate_hash;

use bevy_ecs::{
    event::{Event, Events},
    prelude::{Mut, Resource},
    world::World,
};
use glam::{Mat4, Vec2, Vec3};

/// 通过 Bevy 读取资源；缺失资源由 Bevy 报错。
pub fn get_res<T: Resource>(world: &World) -> &T {
    world.resource::<T>()
}

/// 返回 Bevy 的可变借用守卫，保留变化检测。
pub fn get_res_mut<T: Resource>(world: &mut World) -> Mut<'_, T> {
    world.resource_mut::<T>()
}

/// 消费并清空事件双缓冲；多个订阅者应改用 EventReader。
pub fn drain_events<E: Event>(world: &mut World) -> Vec<E> {
    world.resource_mut::<Events<E>>().drain().collect()
}

/// 清空整个事件流并返回最后一项，之前的事件也会被消费。
pub fn get_last_event<E: Event>(world: &mut World) -> Option<E> {
    world.resource_mut::<Events<E>>().drain().last()
}

/// B @140ac8780：先仿射view列乘得到xyz，再以w=1进入投影；直接齐次除法。
/// 原版不裁剪负/零w，也不校验窗口；调用者自己决定如何筛选投影结果。
pub fn world_space_to_screen_space(
    world_position: Vec3,
    width: f32,
    height: f32,
    world_to_view: &Mat4,
    view_to_clip: &Mat4,
) -> Vec2 {
    fn transform(matrix: &Mat4, point: Vec3) -> glam::Vec4 {
        // 原指令先y列加x列，再z列，最后加平移；不改变浮点求值顺序。
        let xy = matrix.y_axis * point.y + matrix.x_axis * point.x;
        (matrix.z_axis * point.z + xy) + matrix.w_axis
    }
    let viewed = transform(world_to_view, world_position);
    let clip = transform(view_to_clip, viewed.truncate());
    let x = clip.x / clip.w * 0.5 + 0.5;
    let y = clip.y / clip.w * 0.5 + 0.5;
    Vec2::new(x * width, (1.0 - y) * height)
}

/// 当前架构显式可见性检查接口；不是原版直接投影入口的替代实现。
pub fn try_world_space_to_screen_space(
    world_position: Vec3,
    width: f32,
    height: f32,
    world_to_view: &Mat4,
    view_to_clip: &Mat4,
) -> Option<Vec2> {
    if !(width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0) {
        return None;
    }
    let clip_position = *view_to_clip * (*world_to_view * world_position.extend(1.0));
    if !clip_position.is_finite() || clip_position.w <= 1e-10 {
        return None;
    }
    let normalized = clip_position.truncate().truncate() / clip_position.w;
    let screen_position = Vec2::new(
        (normalized.x + 1.0) * 0.5 * width,
        (1.0 - normalized.y) * 0.5 * height,
    );
    screen_position.is_finite().then_some(screen_position)
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

pub mod audio_utils;
