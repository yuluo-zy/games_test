//! draw_list.rs — DrawListItem。
//!
//! 本文件无独立编译过程（`DrawListItem::to_draw_packet` inline 22 宿主）。
//! 类型关系证据：`core::slice::sort::stable::driftsort_main<
//!   tuple<DrawListItem, fn(DrawBuilder, &EntityDrawParams, &mut World)> >`（module 590）。

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::draw_stream::DrawPacket;
use crate::render::pass_builder::pass_resources::PassResources;
use bevy_ecs::world::World;

/// CPU draw-list 项：携带实体绘制回调。
/// 排序键/字段布局 U（typed gap）。
#[derive(Clone, Copy, Debug)]
pub struct DrawListItem {
    pub sort_key: u64,
    /// 绘制回调：`fn(DrawBuilder, &EntityDrawParams, &mut World)`。
    pub render_fn: fn(DrawBuilder, &EntityDrawParams, &mut World) -> DrawPacket,
}

/// 实体绘制参数（归属 WP4 渲染列表，引用不定义）。
pub struct EntityDrawParams {
    pub _private: (),
}

impl DrawListItem {
    /// `to_draw_packet`（inline 22）：把 draw-list 项转成 DrawPacket。
    pub fn to_draw_packet(
        &self,
        _resources: &PassResources,
        _world: &mut World,
        _params: &EntityDrawParams,
    ) -> DrawPacket {
        DrawPacket::new()
    }
}
