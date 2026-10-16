//! `decoclutter_show_pins.rs` — 显示/隐藏已固定 deco-clutter 图钉（B 0x1412357c0，O）。
//!
//! 证据：`chapters/systems/ui_clutter/decoclutter_show_pins.rs.md`、`ui-input/DIGEST.md`、
//! `ui-input/decomp/decomp-0x1412357c0.txt`（1128 行）：
//! - 遍历 pinned clutter 集合，按 `ShowPinnedDecoClutter` 状态切换图钉实体可见性；
//! - 局部 helper `FUN_141233f80`（图钉槽查找，I）；
//! - `register_required_components` @0x141237200（bevy 组件注册）；
//! - `clone_behavior` @0x140002370（ICF 折叠单行，O）。
//!
//! decomp 结构：`ShowPinnedDecoClutter` 含**两个 Vec<u64> 形态缓冲**（uStack_1d0/
//! lStack_1c8/uStack_1c0 与 uStack_220/lStack_218/uStack_210，cap=4/ptr=0/len=0）——
//! SPEC"56B union" = 2×24B Vec + 8B 载荷槽。实体/组件遍历（hashbrown + clutter_mesh_ty_data
//! + UiState::add + SignifierFactory + raycast FUN_1408ce180）依赖宿主 ECS 实体机制；
//! 本实现以 [`PinnedDecoClutter`] 集合 + [`PinVisibilityToggle`] 事件落地 CPU 侧渲染
//! 消费，union 缓冲语义按 O 双 Vec 登记（载荷槽语义待取证）。
use bevy_ecs::prelude::*;
use std::collections::HashSet;

/// 已固定 deco-clutter 集合（I：pinned clutter；宿主维护）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PinnedDecoClutter(pub HashSet<u64>);

/// 是否显示图钉（O：`ShowPinnedDecoClutter` 状态资源；56B union = 双 Vec<u64> + 载荷槽）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ShowPinnedDecoClutter {
    /// O 双 Vec 缓冲之一：pinned 实体列表（`uStack_1d0/lStack_1c8/uStack_1c0`）。
    pub pinned: Vec<u64>,
    /// O 双 Vec 缓冲之二：raycast 命中列表（`uStack_220/lStack_218/uStack_210`）。
    pub hits: Vec<u64>,
    /// 载荷槽（bool 判别；字段语义 U 待取证）。
    pub show: bool,
}

/// 图钉可见性切换事件（I：宿主按实体切换组件）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinVisibilityToggle {
    pub entity: u64,
    pub visible: bool,
}

/// 图钉槽组件（O：`register_required_components` 注册；宿主在 pinned 实体上添加）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DecoClutterLock;

/// `decoclutter_show_pins` @0x1412357c0（O 结构；CPU 侧渲染消费）。
///
/// 遍历 pinned clutter 集合，按 `ShowPinnedDecoClutter.show` 状态发射 `PinVisibilityToggle`
/// 并维护 O 双 Vec 缓冲（pinned 列表对齐当前集合）。实体组件切换/raycast（FUN_1408ce180）
/// 属宿主接线。
pub fn decoclutter_show_pins(
    show: Res<ShowPinnedDecoClutter>,
    pinned: Res<PinnedDecoClutter>,
    mut toggled: EventWriter<PinVisibilityToggle>,
    mut state: ResMut<ShowPinnedDecoClutter>,
) {
    state.pinned.clear();
    for &entity in pinned.0.iter() {
        state.pinned.push(entity);
        toggled.write(PinVisibilityToggle {
            entity,
            visible: show.show,
        });
    }
    // O 双 Vec 缓冲之二的 raycast 命中列表由宿主 raycast 消费填充；此处保持原样，
    // 不做假数据。
    let _ = &mut state.hits;
}

/// `register_required_components` @0x141237200（O：bevy 组件注册；`DecoClutterLock`）。
pub fn register_required_components(world: &mut World) {
    world.register_component::<DecoClutterLock>();
}

/// `clone_behavior` @0x140002370（O：ICF 折叠单行；行为标记，无独立逻辑）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CloneBehavior(pub u8);
