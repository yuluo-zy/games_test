//! `decoclutter_show_pins.rs` — 显示/隐藏已固定 deco-clutter 图钉（B 0x1412357c0，O）。
//!
//! 证据：`chapters/systems/ui_clutter/decoclutter_show_pins.rs.md`、`ui-input/DIGEST.md`：
//! - 遍历 pinned clutter 集合，按 `ShowPinnedDecoClutter` 状态切换图钉实体可见性；
//! - 局部 helper `FUN_141233f80`（图钉槽查找，I）；
//! - `register_required_components` @0x141237200（bevy 组件注册）；
//! - `clone_behavior` @0x140002370（ICF 折叠单行，O）。
use bevy_ecs::prelude::*;
use std::collections::HashSet;

/// 已固定 deco-clutter 集合（I：pinned clutter；宿主维护）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PinnedDecoClutter(pub HashSet<u64>);

/// 是否显示图钉（O：`ShowPinnedDecoClutter` 状态资源）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ShowPinnedDecoClutter(pub bool);

/// `decoclutter_show_pins` @0x1412357c0（O 结构）。
///
/// 图钉实体可见性切换依赖实体组件机制（宿主），本实现以事件登记结构。
pub fn decoclutter_show_pins(
    show: Res<ShowPinnedDecoClutter>,
    pinned: Res<PinnedDecoClutter>,
    mut toggled: EventWriter<PinVisibilityToggle>,
) {
    for &entity in pinned.0.iter() {
        toggled.write(PinVisibilityToggle { entity, visible: show.0 });
    }
}

/// 图钉可见性切换事件（I：宿主按实体切换组件）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinVisibilityToggle {
    pub entity: u64,
    pub visible: bool,
}

/// `register_required_components` @0x141237200（O：bevy 组件注册胶水；组件类型属宿主）。
pub fn register_required_components(_world: &mut World) {}

/// `clone_behavior` @0x140002370（O：ICF 折叠单行；行为标记，无独立逻辑）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CloneBehavior(pub u8);
