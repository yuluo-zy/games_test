//! path_stroke.rs — `country_core::resources::path_stroke` 路径笔画事件（组 A
//! restorer 接线；`system_tree::ui_delete_tree` 0x141bd2510 的
//! `EventReader<PathDrawEvent>` 面——删除树 UI 以路径笔画为擦除输入）。
//!
//! 载荷 = {位置, 半径}（O 面：擦除圈输入；与 water/garden 绘制事件同形状，
//! 字段名 I）。

use bevy_ecs::event::Event;
use glam::Vec3;

/// 路径笔画事件（O：`resources::path_stroke::PathDrawEvent`；生产者 = 路径
/// 绘制域（draw_path 族），消费者 = ui_delete_tree 等擦除式 UI）。
#[derive(Event, Clone, Copy, Debug)]
pub struct PathDrawEvent {
    pub position: Vec3,
    pub radius: f32,
}
