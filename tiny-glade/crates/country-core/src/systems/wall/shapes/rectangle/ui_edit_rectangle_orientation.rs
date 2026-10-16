//! B `systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs`:
//! rectangle orientation editing.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs.md`
//! (O, 0x140ad8000 / do_ui_edit_rectangle_rotate 0x140ad8be0). The rotation
//! angle uses the fast 8-coefficient acos polynomial; the sign comes from the
//! cross product. The result updates the UiState80 (variant 2) and emits an
//! EditRectangleCmd plus FinishEditRectangleCmd.

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::get_rectangle_orientation;
use super::finish_edit_rectangle::RectangleEditUiState;
use crate::resources::walls::InnerWalls;

/// 8 fast-acos coefficients (0x142ae0d44..0x142ae0d60).
const ACOS_COEFFS: [u32; 8] = [
    0x3bda90c5, 0x3cd90a90, 0x3c8bfc66, 0x3dfd10f8, 0xbd4d8392, 0x3db63a9e, 0xbe5bbfca, 0x3fc90fda,
];
/// pi (0x40490fdb @0x1429edba4).

/// Rotation input (UI producer 待接线).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RectangleOrientationEditInput {
    pub wall: crate::resources::walls::WallId,
    pub from_axis: Vec2,
    pub cursor: Vec2,
    pub submitting: bool,
}

/// B fast acos: `sqrt(1 - |x|) * P(x)` with the observed coefficients.
fn fast_acos(x: f32) -> f32 {
    let x = x.abs();
    let poly = ACOS_COEFFS
        .iter()
        .fold(0.0f32, |acc, &coef| acc * x + f32::from_bits(coef));
    poly * (1.0 - x).sqrt()
}

/// B 0x140ad8be0: rotate the rectangle toward the cursor direction.
pub fn do_ui_edit_rectangle_rotate(
    wall: crate::resources::walls::WallId,
    from_axis: Vec2,
    cursor: Vec2,
    state: &mut RectangleEditUiState,
    preview: &mut super::super::PreviewRectangle,
    commands: &mut EventWriter<super::super::EditRectangleCmd>,
    finish: &mut EventWriter<super::super::FinishEditRectangleCmd>,
) {
    let to_axis = if cursor.length_squared() > 0.0 {
        cursor.normalize()
    } else {
        from_axis
    };
    let cos = (from_axis.dot(to_axis))
        / ((from_axis.length_squared() * to_axis.length_squared()).max(1e-12).sqrt());
    let angle = fast_acos(cos);
    let cross = from_axis.x * to_axis.y - from_axis.y * to_axis.x;
    let signed = if cross < 0.0 { -angle } else { angle };
    let orientation = get_rectangle_orientation(signed);

    preview.orientation = orientation;

    state.0.fill(0);
    state.0[..4].copy_from_slice(&2u32.to_le_bytes());
    state.0[0x08..0x10].copy_from_slice(&wall.0.to_le_bytes());
    state.0[0x18..0x1c].copy_from_slice(&signed.to_le_bytes());

    commands.write(super::super::EditRectangleCmd {
        wall,
        center: preview.center,
        dimensions: preview.dimensions,
        orientation,
    });
    finish.write(super::super::FinishEditRectangleCmd { wall });
}

/// B 0x140ad8000 (system shell).
pub fn ui_edit_rectangle_orientation(
    input: Res<RectangleOrientationEditInput>,
    inner: Res<InnerWalls>,
    mut state: ResMut<RectangleEditUiState>,
    mut preview: ResMut<super::super::PreviewRectangle>,
    mut commands: EventWriter<super::super::EditRectangleCmd>,
    mut finish: EventWriter<super::super::FinishEditRectangleCmd>,
) {
    if !input.submitting {
        return;
    }
    if !inner.contains(input.wall) {
        return;
    }
    do_ui_edit_rectangle_rotate(
        input.wall,
        input.from_axis,
        input.cursor,
        &mut state,
        &mut preview,
        &mut commands,
        &mut finish,
    );
}
