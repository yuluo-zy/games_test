//! `ui_focus_decoclutter.rs` — deco-clutter 聚焦（B 0x1409a92c0，O 门 + 主读路径）。
//!
//! 证据：RE-DECORATOR-STORAGE §4（ui_clutter 消费端，O）+ `ui-input/DIGEST.md`。
//! 主读路径：DecoratorId → `decorator_to_storage` 索引（24B 条目）→
//! `DecoratorStorage::get`（0x140a1c2c0）→ Decorators 内嵌 map@+0x30 →
//! DecoratorInfo；随后经 `clutter_mesh_ty_data`（0x1408b8e90）+ `UiState::add`
//! （0x1409231f0）+ SignifierFactory + EventWriter×4 驱动 deco-clutter 移动/旋转。
//!
//! 实体机制（RE §2）：装饰实体由消费系统按 DecoratorId 读 DecoratorInfo 后生成；
//! 本系统完成存储读路径并以事件登记聚焦目标，实体/相机联动（clutter_mesh_ty_data/
//! UiState::add/SignifierFactory/EventWriter×4）属宿主接线（登记）。
use bevy_ecs::prelude::*;

use crate::resources::walls::decorator_storage::DecoratorStorage;
use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};

/// deco-clutter 聚焦状态资源（I：门 = 判别 `tag`；`decorator_id` = 聚焦目标，
/// RE §4 主读路径输入）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DecoClutterFocusState {
    /// 状态机门（O：首 u32 判别）。
    pub tag: u32,
    /// 聚焦的 DecoratorId（I：主读路径输入）。
    pub decorator_id: u32,
}

/// `ui_focus_decoclutter` @0x1409a92c0（O 门；主读路径 = DecoratorStorage）。
///
/// DecoratorId → d2s 索引 → DecoratorStorage::get → 内嵌 map → DecoratorInfo；
/// 未中（索引/桶/info 任一缺失）直接返回（O：null 或 [Decorators+0x48]==0 → return）。
pub fn ui_focus_decoclutter(
    state: Res<DecoClutterFocusState>,
    storage: Res<DecoratorStorage>,
    mut focused: EventWriter<DecoClutterFocused>,
) {
    if state.tag == 0 {
        return;
    }
    let id = DecoratorId(state.decorator_id);
    let Some(addr) = storage.lookup_storage(id) else {
        return;
    };
    let Some(info) = storage.decorator_info(id) else {
        return;
    };
    focused.write(DecoClutterFocused {
        decorator_id: id.0,
        addr,
        info: *info,
    });
}

/// deco-clutter 聚焦事件（I：载荷 = 聚焦 DecoratorId + 所在存储 + DecoratorInfo；
/// 宿主按 id 定位实体并驱动移动/旋转）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct DecoClutterFocused {
    pub decorator_id: u32,
    pub addr: crate::resources::walls::decorator_storage::DecoratorStorageAddr,
    pub info: DecoratorInfo,
}
