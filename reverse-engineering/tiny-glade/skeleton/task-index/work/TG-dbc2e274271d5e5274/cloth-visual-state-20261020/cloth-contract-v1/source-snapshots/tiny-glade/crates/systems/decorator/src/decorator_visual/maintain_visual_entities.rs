//! B `decorator_visual/maintain_visual_entities.rs` — 装饰视觉实体维护（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f9260（clear_out_visual_entities，11KB）+
//! 0x1407f98c0 原字节手解（clear_out_wall_holes = 尾跳
//! `WallHoles::clear_for_next_frame` 0x140f867c0）+ 骨架函数体注解
//! （decorator_visual_state::drain 内联宿主 + EntityCommands::despawn）。
//!
//! 2026-10-17 reviewer-state 复核：clear_out_visual_entities **无持久表清除、
//! 无墙洞帧推进**（后两者已从本函数删除）；墙洞清帧归 clear_out_wall_holes。

use bevy_ecs::system::Commands;
use bevy_ecs::system::ResMut;
use country_core::resources::walls::wall_holes::{PrevWallHoles, WallHoles};

use crate::decorator_visual_state::DecoratorVisualState;

/// 0x1407f98c0（O，原字节手解）：两帧槽位写回 + 尾跳
/// `WallHoles::clear_for_next_frame(a,b)`。
///
/// 参数面（O）：param_1 = ResMut<WallHoles>（+0x1c→+0x10 tick 回写 + 首字段
/// 指针），param_2 = ResMut<PrevWallHoles>（同形）；尾跳清帧推进。
pub fn clear_out_wall_holes(
    mut wall_holes: ResMut<WallHoles>,
    mut previous: ResMut<PrevWallHoles>,
) {
    // 原实现 = 尾跳（jmp 0x140f867c0），无自有业务。
    wall_holes.clear_for_next_frame(&mut previous);
}

/// 0x1407f9260（O，11KB fresh）：装饰视觉实体帧清系统。
///
/// 参数面（O）：param_1 = ResMut 复合（tick 回写 + 槽位写）+ param_2 四 qword
/// 复合（查询/捕获面，U）。业务（O，reviewer-state 复核 2026-10-17）：puffin
/// 作用域 → 瞬态表 drain（内联宿主 impl$3::drain，闭包类型
/// `Drain<DecoratorId, Vec<Entity>>` flat_map）逐实体 despawn
/// （EntityCommands::despawn 边）→ drop `FlatMap<Drain, Map<IntoIter<Entity>>>`。
/// **无持久表清除、无墙洞帧推进**（墙洞清帧 = 独立系统 clear_out_wall_holes
/// 0x1407f98c0；持久表清除若在他处，单独落地并登记）。
///
/// U 登记：param_2 复合（四 qword 捕获）的精确类型待取证；此处以
/// Commands + DecoratorVisualState 承载可观察业务。
pub fn clear_out_visual_entities(
    mut commands: Commands,
    mut visual_state: ResMut<DecoratorVisualState>,
) {
    // O：瞬态表 drain + 逐实体 despawn（原内联 closure 语义）。
    for (_id, entities) in visual_state.transient.drain() {
        for entity in entities {
            commands.entity(entity).despawn();
        }
    }
}
