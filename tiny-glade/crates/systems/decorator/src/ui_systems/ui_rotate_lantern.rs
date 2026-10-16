//! B `ui_systems/ui_rotate_lantern.rs` — 灯/装饰旋转交互（0x1407a8580，O）。
//!
//! 门控（O）：`dispatcher.state==3 && payload==0`（128 位 `state-2==1` 检查）；
//! `UiWidgetContext::is_active`（0x14092e680）激活检查；拖动相位
//! （`dragged` 0x14092df10）：相位<2 → 复位（`*[+0x10]←[+0x1c]`、
//! `[内层+0x48]=1`）；随后 `lookup_storage`（0x140a1c3e0）+ `decorator_info`
//! （0x140a1c5f0）→ 按目的 tag 跳表 UNK_142a82be8 分派
//! （wall→0x1407a91bb / roof→0x1407a8766 / terrain→0x1407a87a6 /
//! stair→0x1407a8814）；未命中 panic（ui_rotate_lantern.rs 行 40）。
//! 旋转语义（I）：按相机 xz 投影角与光标相对位置计算旋转；旋转角写入
//! `DecoratorRotationXZ`（from_xz_forward_vector 消费）并推 0x28B 移动类操作
//! （kind 0x21 族；精确载荷 U——处理器为函数尾部内联段，逐位登记）。
use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::systems::ui::resources::ui_state::{UiState, WidgetId};

use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use crate::ui_systems::MoveDecorationInteractionState;

/// `Local<EditingState>`（ui_rotate_lantern 域；I：旋转预览状态）。
#[derive(Default)]
pub struct EditingState {
    /// 当前旋转角（I）。
    pub yaw: f32,
    /// 被编辑装饰 id（I）。
    pub id: Option<DecoratorId>,
}

/// B 0x1407a8580（O 结构）：旋转交互主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_rotate_decorator(
    ui: Res<UiState>,
    _stream: ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    storage: ResMut<DecoratorStorage>,
    mut interaction: ResMut<MoveDecorationInteractionState>,
    mut local: Local<EditingState>,
) {
    if !dispatcher.gated(3) {
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
    let info = lookup_decorator_info(&storage, DecoratorId(dispatcher.payload as u32));
    let Some(info) = info else {
        panic!("ui_rotate_decorator: no decorator info for active id (ui_rotate_lantern.rs:40)");
    };
    // O：按目的 tag 跳表分派（处理器 = 本函数尾部内联段）。
    match info.dst_tag() {
        0 | 1 => rotate_wall_handler(&mut local, &storage, &info, dragged),
        2 => rotate_roof_handler(&mut local, &storage, &info, dragged),
        3 => rotate_terrain_handler(&mut local, &storage, &info, dragged),
        4 => rotate_stair_handler(&mut local, &storage, &info, dragged),
        _ => unreachable!(),
    }
}

fn lookup_decorator_info(
    storage: &DecoratorStorage,
    id: DecoratorId,
) -> Option<&DecoratorInfo> {
    let addr = storage.lookup_storage(id)?;
    storage.get(addr)?.get(id)
}

fn rotate_wall_handler(
    _local: &mut EditingState,
    _storage: &DecoratorStorage,
    _info: &DecoratorInfo,
    _dragged: u8,
) {
    // I：墙旋转处理器（0x1407a91bb 尾部段；墙空间角度计算 U）。
}

fn rotate_roof_handler(
    _local: &mut EditingState,
    storage: &DecoratorStorage,
    _info: &DecoratorInfo,
    _dragged: u8,
) {
    // I：屋顶旋转处理器（0x1407a8766：0x140a1aa20 存储族调用 + 载荷拷贝）。
    let _ = storage;
}

fn rotate_terrain_handler(
    _local: &mut EditingState,
    _storage: &DecoratorStorage,
    _info: &DecoratorInfo,
    _dragged: u8,
) {
    // I：地形旋转处理器（0x1407a87a6：位置写 + FUN_140257ba0 墙空间转换）。
}

fn rotate_stair_handler(
    _local: &mut EditingState,
    _storage: &DecoratorStorage,
    _info: &DecoratorInfo,
    _dragged: u8,
) {
    // I：楼梯旋转处理器（0x1407a8814 尾部段；楼梯拓扑旋转 U）。
}

/// 目的存储地址辅助（I：DecoratorStorageAddr 16B 判别面）。
#[allow(dead_code)]
fn dst_addr(_info: &DecoratorInfo) -> Option<DecoratorStorageAddr> {
    None
}
