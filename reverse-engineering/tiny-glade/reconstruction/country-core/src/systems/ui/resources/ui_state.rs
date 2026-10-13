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

/// `UiWidgetContext` (O named entity): per-widget cursor state snapshot.
#[derive(Clone, Copy, Debug, Default)]
pub struct UiWidgetContext {
    /// hover-state byte (@+0x18, O).
    pub hover: HoverState,
    /// click-state bits (@+0x19, O).
    pub click: u8,
    /// matched widget id (@+0x38, O).
    pub widget: WidgetId,
    /// intersection payload (@+0x28..+0x58, O copy).
    pub intersection: Gap,
}

impl UiWidgetContext {
    /// `id` @0x1406868b0 (O).
    pub fn id(&self) -> WidgetId {
        self.widget
    }
    /// `clicked` @0x14092e5f0 (O).
    pub fn clicked(&self) -> bool {
        self.hover.hover_byte() > 1 && self.click < 2 && (self.click & 1) != 0
    }
    /// `hovered` @0x14092e620 (O).
    pub fn hovered(&self) -> bool {
        self.hover.is_some()
    }
    /// `is_active` @0x14092e680 (O).
    pub fn is_active(&self) -> bool {
        self.hover.is_some()
    }
    /// `intersecting` @0x14092e640 (O: payload copy or None).
    pub fn intersecting(&self) -> Option<Gap> {
        if self.is_active() {
            Some(self.intersection)
        } else {
            None
        }
    }
    /// `dragged` (O @0x14092df10 folding with is_dragging).
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

/// `UiState` (O named entity; widget registry resource).
#[derive(Resource, Clone, Debug, Default)]
pub struct UiState {
    /// `RwLock<HashMap<WidgetId, Arc<WidgetParams>>>` (O; +0x60 lock word).
    pub widgets: Arc<Gap>,
    /// active widget + intersection state (O bytes @+0x18/+0x19/+0x38).
    pub context: UiWidgetContext,
}

impl UiState {
    /// `UiState::new` (O inline in `add_ui`).
    pub fn new() -> Self {
        UiState {
            widgets: Arc::new(Gap),
            context: UiWidgetContext::default(),
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
        _id: WidgetId,
        _params: crate::systems::ui::resources::widget_params::WidgetParams,
    ) {
        todo!("recover UiState::add_with_id (RwLock insert of Arc<WidgetParams> 0xb0 B)")
    }

    /// `widget` @0x14067dd80 (O; 1177 callers).
    pub fn widget(&self, _id: WidgetId) -> Option<Arc<crate::systems::ui::resources::widget_params::WidgetParams>> {
        None
    }

    /// `active_widget_state` @0x140002450 (O).
    pub fn active_widget_state(&self) -> Option<(WidgetId, crate::systems::ui::resources::widget_params::WidgetParams)> {
        None
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
        todo!("recover UiState::clearing_canvas_w_layer_and_sound")
    }

    /// `origin` (O inline into `signifier_animation`).
    pub fn origin(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
}
