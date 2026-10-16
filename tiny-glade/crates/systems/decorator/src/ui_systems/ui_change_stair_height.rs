//! B `ui_systems/ui_change_stair_height.rs` — 楼梯高度调整（0x1407adea0，O）。
//!
//! 门控（O）：`dispatcher.state==4 && payload==0` 且 `dispatcher.ty=='?'`
//! （`[*gate+0x5c]=='?'` 观察）；`is_active` 检查；拖动相位<2 → 复位；
//! `lookup_storage` + `decorator_info`（0x140a1c3e0/0x140a1c5f0）→ 按目的 tag
//! 跳表 UNK_142a832b0 分派（wall/roof 共用 0x1407adba7、terrain 0x1407ad998、
//! stair 0x1407ad91a；stair 与 wall 同址）；未命中 panic
//! （ui_change_stair_height.rs 行 40/44）。
//! 语义（I）：楼梯端高度随光标调整 → `DecoratorExtraInfo::Stair` 载荷写入 +
//! 0x28B 高度操作（kind 0x12 族）。
use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::systems::ui::resources::ui_state::{UiState, WidgetId};

use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use crate::ui_systems::MoveDecorationInteractionState;

/// B 0x1407adea0（O 结构）：楼梯高度调整主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_change_stair_height(
    ui: Res<UiState>,
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    _transforms: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    _walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    storage: ResMut<DecoratorStorage>,
    mut interaction: ResMut<MoveDecorationInteractionState>,
) {
    if !(dispatcher.gated(4) && dispatcher.ty == b'?') {
        return;
    }
    let id = WidgetId(dispatcher.payload);
    if !ui.widget_context(id).is_active() {
        return;
    }
    let dragged = ui.widget_context(id).dragged();
    if dragged < 2 {
        interaction.reset_interaction();
    }
    let decorator_id = DecoratorId(dispatcher.payload as u32);
    let Some(info) = lookup_info(&storage, decorator_id) else {
        panic!("ui_change_stair_height: no decorator info (ui_change_stair_height.rs:40)");
    };
    // O：按目的 tag 跳表（UNK_142a832b0；wall/stair 同址 0x1407adba7）。
    match info.dst_tag() {
        0 | 1 | 4 => change_stair_height_wall(&storage, info),
        2 => change_stair_height_terrain(&storage, info),
        3 => change_stair_height_stair(&storage, info),
        _ => unreachable!(),
    }
    let _ = DecoratorStorageAddr::default();
}

fn lookup_info<'a>(
    storage: &'a DecoratorStorage,
    id: DecoratorId,
) -> Option<&'a DecoratorInfo> {
    let addr = storage.lookup_storage(id)?;
    storage.get(addr)?.get(id)
}

/// 墙/楼梯端高度处理器（I：0x1407adba7 尾部段；楼梯端载荷写入 U）。
fn change_stair_height_wall(_storage: &DecoratorStorage, _info: &DecoratorInfo) {}

/// 地形处理器（I：0x1407ad998 尾部段）。
fn change_stair_height_terrain(_storage: &DecoratorStorage, _info: &DecoratorInfo) {}

/// 楼梯处理器（I：0x1407ad91a 尾部段；与墙共用 0x1407adba7 语义）。
fn change_stair_height_stair(_storage: &DecoratorStorage, _info: &DecoratorInfo) {}
