//! B `systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs`:
//! orientation edit widget.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs.md`
//! (O, UiRectangleOrientationEdit::widget 0x14083d3f0 / signifier 0x14083da10).
//! Rotation-sensitivity constants 1.5 / 0.3 / 0.25 drive the trigger volume;
//! the widget layer is render host 待接线.

use bevy_ecs::prelude::*;

/// B rotation sensitivity constants (0x3fc00000 / 0x3e99999a / 0x3e800000).
const ROTATION_SENSITIVITY: f32 = 1.5;
const ROTATION_SNAP: f32 = 0.25;

/// B 0x14083d3f0: widget layer (host 待接线).
pub fn ui_rectangle_orientation_edit_widget(
    _preview: Res<crate::systems::wall::shapes::PreviewRectangle>,
) {
    let _ = (ROTATION_SENSITIVITY, ROTATION_SNAP);
}

/// B 0x14083da10: orientation signifier (host 待接线).
pub fn ui_rectangle_orientation_edit_signifier() {}
