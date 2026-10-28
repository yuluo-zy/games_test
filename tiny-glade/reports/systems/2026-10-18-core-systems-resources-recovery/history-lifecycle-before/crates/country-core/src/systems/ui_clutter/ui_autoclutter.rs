//! `ui_autoclutter.rs` — 右键把 auto-clutter 转 deco-clutter（B 0x14099ca60）。
//!
//! 证据：fresh 反编译（restorer-terrain-clutter/jobs/decomp/14099ca60，1400+ 行）。逐语句对照：
//! - 消费点击事件队列（stride 0x10）；对每个 id 查 0xa0B clutter 记录表，`+0x58 == 0`
//!   视为已删除跳过；
//! - 记录 `+0x94` → 0x48 步长类型表；记录 `+0x50` = 子项列表（stride 0x10）；
//! - 逐子项：`uVar37 = *(uint*)(children + 8 + i*0x10)`；父记录 `0x60` 步长（`uVar37*0x60`），
//!   变换记录 `0x30` 步长（`uVar37*0x30`）；
//! - **`parent = puVar19[4]`（父记录 +0x10）**：`HashMap<DecoratorId,
//!   Vec<DisplayChildAutoClutterInfo>>`；父类型 `*puVar19 < 3` 才处理；
//! - `ClutterMeshTy::from` + `clutter_mesh_ty_data` → DisplayChildAutoClutterInfo
//!   （0x60 B：mesh 4 qword @+0x00、transform 6 qword @+0x20、`+0x50` = 变换记录 +0xc）；
//!   已有键 → `Vec::push`（`grow_one<DisplayChildAutoClutterInfo>`）；无键 →
//!   `RawTable::reserve_rehash` + 新建 `Vec`（cap=1/len=1）；
//! - 第二事件队列：命中 d2s 位图 → 直接调用 `handle_rmb_autoclutter`（crate 以产出
//!   [`RmbAutoClutterHit`] 事件承载）；
//! - 尾部模式门：`disc ∈ [6,0x15) → disc-5`，其余 0；接受 `disc==0` 或
//!   `disc==9 ∧ payload 低字节==3`（Bulldoze），随后 `Rapier::raycast`（命中 tag != 2
//!   ∧ 标签字节 0x0c）→ `DecoratorStorage` 探测 → `UiState::add` → 0x60 记录类型
//!   3..5 子跳表 `&DAT_142abbcdc`（UNRECOVERED_JUMPTABLE）。
//!
//! **早期 Rust 的 `InstaCreateDeco` / `InstaCreateStairPoint` 事件系编造**（fresh 无这些
//! 事件/命令），已删。宿主实体表（0xa0B/0x60/0x30 记录）由 [`AutoClutterChildTable`]
//! 适配承载，尾部 raycast/UI 注册路径登记待接。
use bevy_ecs::prelude::*;
use hashbrown::HashMap;

use crate::systems::mode_manager::{AppMode, EraserLayer};

use super::rmb_autoclutter::EvRightClickedAutoClutter;

/// DisplayChildAutoClutterInfo（O：0x60 B；`grow_one<DisplayChildAutoClutterInfo>`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DisplayChildAutoClutterInfo {
    /// `+0x00..+0x1f`：`clutter_mesh_ty_data()` 的 4 个 qword。
    pub mesh: [u64; 4],
    /// `+0x20..+0x4f`：变换记录（0x30 stride）的 6 个 qword。
    pub transform: [u64; 6],
    /// `+0x50`：变换记录 `+0xc` 的 u32。
    pub field: u32,
}

/// 单个子项输入（I：宿主从 0xa0B 记录的 +0x50 子表、0x60 父表、0x30 变换表解析）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AutoClutterChild {
    /// 父记录 `+0x00` 类型（O：`*puVar19 < 3` 门）。
    pub parent_tag: u32,
    /// 父记录 `+0x10`（O：HashMap 键 `DecoratorId`）。
    pub parent_key: u32,
    /// 子项信息（0x60 B）。
    pub info: DisplayChildAutoClutterInfo,
}

/// clutter id → 子项列表（I：宿主维护；对应 fresh 的 0xa0B 记录表 + 父/变换表）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AutoClutterChildTable(pub HashMap<u64, Vec<AutoClutterChild>>);

/// clutter→parent 映射（O：`HashMap<DecoratorId, Vec<DisplayChildAutoClutterInfo>>`；
/// `maintain_autoclutter_parent_hashmap` 在函数内维护）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AutoClutterParentMap(
    pub HashMap<u32, Vec<DisplayChildAutoClutterInfo>>,
);

impl AutoClutterParentMap {
    /// 维护一条记录（已有键 push，新键新建 Vec）。
    pub fn maintain(&mut self, parent: u32, info: DisplayChildAutoClutterInfo) {
        self.0.entry(parent).or_default().push(info);
    }
    pub fn parent_of(&self, parent: u32) -> Option<&Vec<DisplayChildAutoClutterInfo>> {
        self.0.get(&parent)
    }
}

/// `ui_convert_autoclutter_to_decoclutter` @0x14099ca60。
pub fn ui_convert_autoclutter_to_decoclutter(
    mode: Res<AppMode>,
    mut right_clicks: EventReader<EvRightClickedAutoClutter>,
    children: Res<AutoClutterChildTable>,
    mut parents: ResMut<AutoClutterParentMap>,
) {
    // **主循环**：点击 → 查 clutter 记录 → 逐子项维护 parent 映射。
    for click in right_clicks.read() {
        let Some(list) = children.0.get(&click.entity) else {
            continue;
        };
        for child in list {
            // O：`if (*puVar19 < 3)` 才进入转换；否则跳过。
            if child.parent_tag < 3 {
                parents.maintain(child.parent_key, child.info);
            }
        }
    }

    // **第二事件队列**（O：d2s 索引 + 位图命中 → 调 `handle_rmb_autoclutter`）。
    // crate 以产出 RmbAutoClutterHit 事件承载（宿主实体表命中判定在宿主侧，I）。

    // **尾部模式门**（O：`disc-6 < 0xf → disc-5`，否则 0；接受 disc==0 或
    // disc==9 ∧ payload 低字节==3）。
    let disc = mode.native_tag();
    let accepted = disc == 0
        || matches!(*mode, AppMode::Eraser(EraserLayer::Bulldoze));
    if !accepted {
        return;
    }
    // 之后：`Rapier::raycast`（命中 tag != 2 ∧ 标签 0x0c）→ DecoratorStorage 探测 →
    // `UiState::add` → 0x60 记录类型 3..5 子跳表（&DAT_142abbcdc，UNRECOVERED_JUMPTABLE）。
    // 宿主射线/装饰存储/UI 名队列未接入，登记待接（不伪造实体/事件）。
}
