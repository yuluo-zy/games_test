//! `systems/ui/ui_update.rs` — the main UI state-update system
//! (`ui_update`, `enter_hover`, `get_ui_volume`).
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/ui_update.rs`.
//! O evidence (Ghidra, B patched EXE; `ui_update` @0x140b2de60, 1829-line
//! system):
//! - iterates the `UiState` widget registry
//!   (`HashMap<WidgetId, Arc<WidgetParams>>`);
//! - per widget: `get_ui_volume` (world-space volume from `WidgetParams`),
//!   hover/click/drag state transitions (HoverState / DragState), sounds via
//!   `PlayUiSound`, and `enter_hover`;
//! - clears the per-frame intersection list (line 1098 `clear`).
//! - `get_ui_volume` (4 callers) / `enter_hover` (inline host).

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn ui_update(_ui_state: &mut Gap, _commands: &mut Gap) {
    todo!("recover ui_update (hover/click/drag state machine)")
}

/// `get_ui_volume` (O; 4 callers): widget world volume from `WidgetParams`.
pub fn get_ui_volume(_params: &Gap) -> crate::systems::ui::resources::widget_params::WidgetVolumeShape {
    crate::systems::ui::resources::widget_params::WidgetVolumeShape::default()
}

/// `enter_hover` (O inline host).
pub fn enter_hover() {
    todo!("recover enter_hover")
}
