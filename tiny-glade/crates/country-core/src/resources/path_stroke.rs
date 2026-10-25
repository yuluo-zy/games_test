//! path_stroke.rs — `country_core::resources::path_stroke` 路径笔画事件（组 A
//! restorer 接线；`system_tree::ui_delete_tree` 0x141bd2510 的
//! `EventReader<PathDrawEvent>` 面——删除树 UI 以路径笔画为擦除输入）。
//!
//! 载荷 = {位置, 半径, observed 标志}（O 面：擦除圈输入；与 garden/water 绘制
//! 事件同形状，即 `draw_raster` 的 `on_draw`/`on_erase` 回调实参）。

use bevy_ecs::event::Event;
use glam::Vec3;

/// 路径笔画事件（O：`resources::path_stroke::PathDrawEvent`；生产者 = 路径
/// 绘制域（draw_path 族）的 `draw_raster::on_draw` 闭包，消费者 = ui_delete_tree
/// 等擦除式 UI）。
///
/// 第三字段即 `draw_raster` 回调的 `observed` 参数（与 `GardenDrawEvent` 同形）。
#[derive(Event, Clone, Copy, Debug)]
pub struct PathDrawEvent {
    pub position: Vec3,
    pub radius: f32,
    pub observed_flag: bool,
}

/// 路径擦除事件（O：`resources::path_stroke::PathEraseEvent`；生产者 = draw_path
/// 的 `draw_raster::on_erase` 闭包，与 [`PathDrawEvent`] 同形状）。
#[derive(Event, Clone, Copy, Debug)]
pub struct PathEraseEvent {
    pub position: Vec3,
    pub radius: f32,
    pub observed_flag: bool,
}
