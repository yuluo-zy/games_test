//! `systems/ui_focus_mode_shape_height.rs` — focus-mode shape-height gizmo.
//!
//! PDB: `ui_focus_mode_shape_height @0x140a623e0` (9215 B) + `closure$0
//! @0x140a647e0` + `closure$3 @0x140a66a30`. Named entity `LerpedGizmoPosition`.
//!
//! O: projects the cursor onto the shape-height ray and adjusts the selected
//! shape's height with a lerped gizmo position.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::Gap;

/// `LerpedGizmoPosition` (O named entity; gizmo position resource).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct LerpedGizmoPosition {
    pub position: Vec3,
    pub _gap: Gap,
}

/// `ui_focus_mode_shape_height` @0x140a623e0 (O).
pub fn ui_focus_mode_shape_height(
    mut _gizmo: ResMut<LerpedGizmoPosition>,
    _cursor: Res<Gap>,
    _shape: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover ui_focus_mode_shape_height (cursor ray → shape height)")
}
