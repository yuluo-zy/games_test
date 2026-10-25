//! `systems/ui/resources/slider3d.rs` — `Slider3d` (a bevy `SystemParam`) and
//! `Slider3dState` (world-space line-slider drag state).
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/resources/slider3d.rs`.
//! PDB named entities: `Slider3d`, `Slider3dState`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `Slider3d` is a `bevy_ecs::system_param::SystemParam`
//!   (`impl SystemParam::{init_state,get_param,validate_param,new_archetype,apply,queue}`,
//!   646 `new_archetype` callers) — wraps `UiState`, `UiSignifierStream`,
//!   `CameraCursorRay`, cursor/camera/collision resources, and an
//!   `EventWriter` for widget drag ops.
//! - `Slider3dState::project_cursor_onto_line` @0x140a9bd40 (8 callers) —
//!   cursor → nearest point on the slider line.
//! - `Slider3d::update<C, F>` @0x140b1cc80 (generic over the widget context +
//!   drag closure).
//! - `Slider3d::origin_ws` (O).
//! - `impl$1::into_drag_state` (55 callers).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `Slider3dState` (O named entity): a 3D line slider drag state.
#[derive(Clone, Copy, Debug, Default)]
pub struct Slider3dState {
    /// slider origin (world space).
    pub origin: glam::Vec3,
    /// slider direction.
    pub direction: glam::Vec3,
    pub _gap: Gap,
}

impl Slider3dState {
    /// `project_cursor_onto_line` @0x140a9bd40 (O).
    pub fn project_cursor_onto_line(&self, _cursor: glam::Vec3) -> f32 {
        todo!("recover Slider3dState::project_cursor_onto_line")
    }
    /// `update` (O inline).
    pub fn update(&mut self) {
        todo!("recover Slider3dState::update")
    }
}

/// `Slider3d` (O named entity; bevy SystemParam). Layout U — typed gap.
#[derive(Clone, Debug, Default)]
pub struct Slider3d {
    pub _gap: Gap,
}

impl Slider3d {
    /// `Slider3d::origin_ws` (O).
    pub fn origin_ws(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
    /// `Slider3d::update<C, F>` @0x140b1cc80 (O generic).
    pub fn update<C, F>(&mut self, _context: &C, _on_change: F) {
        todo!("recover Slider3d::update")
    }
    /// `into_drag_state` (O; 55 callers).
    pub fn into_drag_state(&self) -> crate::systems::ui::resources::ui_state::DragState {
        crate::systems::ui::resources::ui_state::DragState::None
    }
}
