//! `systems/ui/resources/ui_state.rs` — `UiState` / `UiWidgetContext` /
//! `WidgetId` / `HoverState` / `DragState` / `ActiveWidget`.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/resources/ui_state.rs`.
//! PDB named entities: `ActiveWidget`, `DragState`, `HoverState`, `UiState`,
//! `UiWidgetContext`, `WidgetId`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `UiState` widget registry is an `RwLock<HashMap<WidgetId,
//!   Arc<WidgetParams>>>` guarded via a lock-word at **+0x60**
//!   (`add_with_id` @0x14092e3e0 / `clearing_canvas_w_layer_and_sound`
//!   @0x14092df50 acquire/release the +0x60 write-lock, then insert). Each
//!   `WidgetParams` payload is **0xb0 B** (Arc alloc 0xc0).
//! - `UiWidgetContext` reads `UiState`: hover-state byte **@+0x18**, click-state
//!   byte **@+0x19**, active/hover `WidgetId` **@+0x38**, intersection payload
//!   **@+0x28..+0x58** (7 qwords; `intersecting` copies it or returns a
//!   tag-2 `None`).
//! - `WidgetId::from_name` @0x14092df30 = `utils::calculate_hash(name)` (u64).
//! - `clicked` @0x14092e5f0: `hover-byte > 1 && click-byte < 2 && (click & 1)`;
//!   `hovered` @0x14092e620 / `is_active` @0x14092e680: hover-byte != 0 && id
//!   @+0x38 == ctx id; `intersecting` @0x14092e640 (payload copy).
//! - `clearing_canvas` @0x14092e390 / `clearing_canvas_w_layer` @0x14092e340
//!   → `clearing_canvas_w_layer_and_sound` (a dedicated clearing-canvas widget
//!   with volume size `130×130` = `0x43020000,0x43020000`).

#![allow(dead_code, unused_imports)]

use std::collections::HashMap;
use std::sync::Arc;

use bevy_ecs::resource::Resource;
use parking_lot::RwLock;

use crate::Gap;

/// `WidgetId` (O named entity; u64 name-hash).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(pub u64);

impl WidgetId {
    /// `WidgetId::from_name` @0x14092df30 (O).
    pub fn from_name(name: &str) -> WidgetId {
        WidgetId(crate::utils::calculate_hash(name))
    }
}

/// `HoverState` (O named entity; hover-state discriminant).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HoverState {
    /// None (I).
    #[default]
    None,
    /// Hovering (I; byte 1).
    Hovering,
    /// Intersecting/dragging (I; byte > 1).
    Intersecting,
}

impl HoverState {
    /// `HoverState::is_some` (O @0x1408c5680 folding).
    pub fn is_some(self) -> bool {
        !matches!(self, HoverState::None)
    }
}

/// `DragState` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DragState {
    #[default]
    None,
    Started,
    Dragging,
    Finished,
}

impl DragState {
    /// `DragState::is_some` @0x14092e6b0 (O).
    pub fn is_some(self) -> bool {
        !matches!(self, DragState::None)
    }
    /// `DragState::is_started` @0x14092e6c0 (O).
    pub fn is_started(self) -> bool {
        matches!(self, DragState::Started | DragState::Dragging)
    }
    /// `DragState::is_finished` @0x14092e6d0 (O).
    pub fn is_finished(self) -> bool {
        matches!(self, DragState::Finished)
    }
}

/// `ActiveWidget` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ActiveWidget(pub Gap);

impl ActiveWidget {
    /// `ActiveWidget::is_dragging` @0x14092e6a0 (O).
    pub fn is_dragging(self) -> bool {
        false
    }
}

/// `UiWidgetContext` (O named entity): 16B `{&UiState, WidgetId}`。
///
/// Fresh B evidence（2026-10-17, `UiWidgetContext::*` @0x14092e5f0/e620/e640/
/// e680）：方法以 `in_RCX` 为 `{ui: &UiState @0, id: WidgetId @8}`，读
/// `(*in_RCX)+0x18/+0x19/+0x28..+0x60`。旧实现的“快照字段”与证据不符。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UiWidgetContext<'a> {
    /// 指向 `UiState`（O：方法首字段解引用）。
    pub ui: &'a UiState,
    /// 当前 widget id（O：+8，与 `UiState` 的交集 id 比较）。
    pub id: WidgetId,
}

impl<'a> UiWidgetContext<'a> {
    /// `id` @0x1406868b0 (O)。
    pub fn id(&self) -> WidgetId {
        self.id
    }
    /// `clicked` @0x14092e5f0 (O：hover>1 && click<2 && click&1；不比较 id)。
    pub fn clicked(&self) -> bool {
        self.ui.hover > 1 && self.ui.click < 2 && (self.ui.click & 1) != 0
    }
    /// `hovered` @0x14092e620 (O：hover==1 && intersection id == self id)。
    pub fn hovered(&self) -> bool {
        self.ui.hover == 1 && self.ui.intersection.widget_id() == self.id
    }
    /// `is_active` @0x14092e680 (O：hover!=0 && intersection id == self id)。
    pub fn is_active(&self) -> bool {
        self.ui.hover != 0 && self.ui.intersection.widget_id() == self.id
    }
    /// `intersecting` @0x14092e640 (O：7 qword 负载拷贝或 None)。
    pub fn intersecting(&self) -> Option<UiIntersection> {
        if self.is_active() {
            Some(self.ui.intersection)
        } else {
            None
        }
    }
    /// `dragged` (O @0x14092df10/U)。
    pub fn dragged(&self) -> bool {
        false
    }
}

impl HoverState {
    fn hover_byte(&self) -> u8 {
        match self {
            HoverState::None => 0,
            HoverState::Hovering => 1,
            HoverState::Intersecting => 2,
        }
    }
}

/// `Intersection` 负载（O：`UiState+0x28..+0x60` 共 7 qword；`intersecting`
/// @0x14092e640 逐 qword 拷贝）。第 3 个 qword（绝对 +0x38）是 widget id。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiIntersection {
    /// +0x28..+0x60（7 × u64）。
    pub words: [u64; 7],
}

impl UiIntersection {
    /// widget id @+0x38（= words[2]）。
    pub fn widget_id(&self) -> WidgetId {
        WidgetId(self.words[2])
    }
}

/// `UiState`（O：`RwLock<HashMap<WidgetId, Arc<WidgetParams>>>` 资源）。
///
/// Fresh B evidence（2026-10-17）：
/// - `insert_resource<UiState>` @0x1415398d0 以 **`memcpy` 0x98 B** 复制
///   （`mov r8d, 0x98` @0x141539cd4）⇒ `size_of::<UiState>() == 0x98`。
/// - `UiState::add_with_id` @0x14092e3e0：锁字在 **+0x60**
///   （`lock cmpxchg qword [rdi+0x60]`），map 数据在 **+0x68**
///   （`lea r15,[rdi+0x68]` 传给 `HashMap::insert`）。
/// - `UiWidgetContext::*` @0x14092e5f0..：hover **+0x18**、click **+0x19**、
///   交集负载 **+0x28..+0x60**、widget id **+0x38**。
/// - `+0x00..0x18`、`+0x1a..0x28` 未在构造/方法中出现，登记为 unknown。
#[repr(C)]
#[derive(Resource, Debug)]
pub struct UiState {
    /// +0x00..+0x18 unknown（O：构造未写、方法未读）。
    pub unknown_00: [u8; 0x18],
    /// +0x18 hover 判别（O）。
    pub hover: u8,
    /// +0x19 click 位（O）。
    pub click: u8,
    /// +0x1a..+0x28 unknown。
    pub unknown_1a: [u8; 0x0e],
    /// +0x28..+0x60 交集负载（O）。
    pub intersection: UiIntersection,
    /// +0x60 锁字 +0x68 map（O：`parking_lot::RwLock<HashMap<..>>`）。
    pub widgets: RwLock<HashMap<WidgetId, Arc<crate::systems::ui::resources::widget_params::WidgetParams>>>,
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

impl UiState {
    /// `UiState::new` (O inline in `add_ui` @0x140945c60)。
    pub fn new() -> Self {
        UiState {
            unknown_00: [0; 0x18],
            hover: 0,
            click: 0,
            unknown_1a: [0; 0x0e],
            intersection: UiIntersection::default(),
            widgets: RwLock::new(HashMap::new()),
        }
    }

    /// `UiState::add(name, params)` @0x140922fb0 (O; 54 callers):
    /// build a `WidgetId` from the name and insert `Arc<WidgetParams>`.
    pub fn add(&mut self, name: &str, params: crate::systems::ui::resources::widget_params::WidgetParams) -> WidgetId {
        let id = WidgetId::from_name(name);
        self.add_with_id(id, params);
        id
    }

    /// `add_with_id` @0x14092e3e0 (O): insert into the registry.
    pub fn add_with_id(
        &mut self,
        id: WidgetId,
        params: crate::systems::ui::resources::widget_params::WidgetParams,
    ) {
        self.widgets.write().insert(id, Arc::new(params));
    }

    /// `widget` @0x14067dd80 (O; 1177 callers).
    pub fn widget(&self, id: WidgetId) -> Option<Arc<crate::systems::ui::resources::widget_params::WidgetParams>> {
        self.widgets.read().get(&id).cloned()
    }

    /// `active_widget_state` (O；负载语义 U)。
    pub fn active_widget_state(&self) -> Option<(WidgetId, crate::systems::ui::resources::widget_params::WidgetParams)> {
        if self.hover == 0 {
            return None;
        }
        let id = self.intersection.widget_id();
        self.widget(id).map(|p| (id, (*p).clone()))
    }

    /// `hovered`（O：`UiWidgetContext::hovered` 的 UiState 入口）。
    pub fn hovered(&self, id: WidgetId) -> bool {
        UiWidgetContext { ui: self, id }.hovered()
    }
    /// `clicked`（O）。
    pub fn clicked(&self, id: WidgetId) -> bool {
        UiWidgetContext { ui: self, id }.clicked()
    }
    /// `is_active`（O）。
    pub fn is_active(&self, id: WidgetId) -> bool {
        UiWidgetContext { ui: self, id }.is_active()
    }
    /// `intersecting`（O）。
    pub fn intersecting(&self, id: WidgetId) -> Option<UiIntersection> {
        UiWidgetContext { ui: self, id }.intersecting()
    }
    /// `current_intersection`（O）。
    pub fn current_intersection(&self) -> Option<UiIntersection> {
        if self.hover != 0 {
            Some(self.intersection)
        } else {
            None
        }
    }
    /// `is_widget_name_active`（O inline）。
    pub fn is_widget_name_active(&self, name: &str) -> bool {
        self.is_active(WidgetId::from_name(name))
    }

    /// `clearing_canvas` @0x14092e390 (O).
    pub fn clearing_canvas(&mut self) {
        self.clearing_canvas_w_layer_and_sound();
    }

    /// `clearing_canvas_w_layer` @0x14092e340 (O).
    pub fn clearing_canvas_w_layer(&mut self, _layer: &crate::systems::ui::resources::widget_params::WorldCanvasLayer) {
        self.clearing_canvas_w_layer_and_sound();
    }

    /// `clearing_canvas_w_layer_and_sound` @0x14092df50 (O): insert the
    /// dedicated clearing-canvas widget (volume 130×130 = `0x43020000²`).
    pub fn clearing_canvas_w_layer_and_sound(&mut self) {
        use crate::systems::ui::resources::widget_params::{UiSounds, WidgetParams, WidgetVolumeShape, WorldCanvasLayer};
        // O: format string containing `_canvas` @0x142aafe48; exact full name U,
        // but it must be stable across frames so the same widget id is reused.
        let params = WidgetParams::new(
            WidgetVolumeShape::clearing_canvas(WorldCanvasLayer(0)),
            UiSounds::empty(),
        );
        self.add("clearing_canvas", params);
    }

    /// `origin` (O inline into `signifier_animation`).
    pub fn origin(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn ui_state_layout_is_0x98() {
        assert_eq!(size_of::<UiState>(), 0x98);
        assert_eq!(offset_of!(UiState, unknown_00), 0x00);
        assert_eq!(offset_of!(UiState, hover), 0x18);
        assert_eq!(offset_of!(UiState, click), 0x19);
        assert_eq!(offset_of!(UiState, unknown_1a), 0x1a);
        assert_eq!(offset_of!(UiState, intersection), 0x28);
        assert_eq!(offset_of!(UiState, widgets), 0x60);
        assert_eq!(size_of::<UiIntersection>(), 0x38);
        assert_eq!(offset_of!(UiIntersection, words), 0x00);
    }

    #[test]
    fn ui_widget_context_layout_is_16b() {
        assert_eq!(size_of::<UiWidgetContext>(), 16);
        assert_eq!(offset_of!(UiWidgetContext, ui), 0);
        assert_eq!(offset_of!(UiWidgetContext, id), 8);
    }

    #[test]
    fn intersection_widget_id_is_at_0x38() {
        let mut it = UiIntersection::default();
        it.words[2] = 0xdead_beef;
        assert_eq!(it.widget_id().0, 0xdead_beef);
        // widget id = intersection 基址 + words[2] = 0x28 + 2×8 = 0x38。
        assert_eq!(
            offset_of!(UiState, intersection) + 2 * size_of::<u64>(),
            0x38,
            "widget id absolute offset"
        );
    }
}
