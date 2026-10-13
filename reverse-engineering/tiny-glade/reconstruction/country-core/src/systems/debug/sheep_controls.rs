//! sheep_controls.rs — 羊群调试控制（O）。
//!
//! 源 `systems/debug/sheep_controls.rs`；实体 `SheepControl` / `PickingMode`。
//! `sheep_controls`（0x1416f11a0）：鼠标拾取羊 → 拖动/快照；`apply_sheep_snapshot`
//! （0x1416f24f0）应用快照；`SheepControl::new`（0x1416f0ea0）。

#![allow(dead_code)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 拾取模式（O：`PickingMode`；`is_some` 内联）。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PickingMode {
    #[default]
    None,
    Picked(u64),
}

impl PickingMode {
    pub fn is_some(&self) -> bool {
        !matches!(self, PickingMode::None)
    }
}

/// 羊控制组件（O：`SheepControl`；`new` 0x1416f0ea0）。
#[derive(Component, Clone, Debug, Default)]
pub struct SheepControl(pub Gap);

/// `sheep_controls`（O 0x1416f11a0）。
pub fn sheep_controls(_world: &mut bevy_ecs::world::World) {}

/// `apply_sheep_snapshot`（O 0x1416f24f0）。
pub fn apply_sheep_snapshot(_world: &mut bevy_ecs::world::World) {}
