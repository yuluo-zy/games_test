//! B `systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs`: rectangle
//! dimension editing.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs.md`
//! (O, 0x140ad6250 / do_ui_edit_rectangle_dims 0x140ad6e00). Dragging an edge
//! moves it along its normal and writes the new dims into the UiState80
//! (variant 1) plus an EditRectangleCmd. The exact projection constants are
//! 待取证; Rectangle2d::expand_along_edge is reused.

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::super::EditRectangleCmd;
use super::finish_edit_rectangle::RectangleEditUiState;
use crate::resources::walls::InnerWalls;
use crate::systems::wall::valid_enclosure::ShapeParameters;

/// Edge-drag input (UI producer 待接线).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RectangleDimsEditInput {
    pub wall: crate::resources::walls::WallId,
    pub edge: Option<usize>,
    pub cursor: Vec2,
    /// Release submits the EditRectangleCmd.
    pub submitting: bool,
}

/// B 0x140c9a5a0: signed distance from the cursor to the given edge line
/// (positive on the outward normal side).
pub fn distance_to_edge(rect: ::utils::geometry::Rectangle2d, edge: usize, cursor: Vec2) -> f32 {
    let (p0, _p1) = rect.get_edge_pts(edge);
    let normal = rect.get_edge_normal(edge);
    let rel = cursor - p0;
    rel.dot(normal)
}

/// B 0x140ad6e00: edge drag -> new dims + state + command.
pub fn do_ui_edit_rectangle_dims(
    wall: crate::resources::walls::WallId,
    edge: usize,
    cursor: Vec2,
    inner: &InnerWalls,
    state: &mut RectangleEditUiState,
    preview: &mut super::super::PreviewRectangle,
    commands: &mut EventWriter<EditRectangleCmd>,
) {
    let Some(source) = inner
        .get(wall)
        .and_then(|s| s.get_enclosure_params())
        .cloned()
    else {
        return;
    };
    let rect = ShapeParameters::from_snapshot(&source).as_rectangle();
    let distance = distance_to_edge(rect, edge, cursor);
    let mut moved = rect;
    moved.expand_along_edge(edge, distance * 2.0, 0.0, 30.0);

    preview.center = moved.center;
    preview.dimensions = moved.size;
    preview.orientation = moved.axis;

    // UiState80 variant 1: wall id at +0x10, dims at +0x48/+0x4c.
    state.0.fill(0);
    state.0[..4].copy_from_slice(&1u32.to_le_bytes());
    state.0[0x10..0x18].copy_from_slice(&wall.0.to_le_bytes());
    state.0[0x30..0x34].copy_from_slice(&moved.size.x.to_le_bytes());
    state.0[0x34..0x38].copy_from_slice(&moved.size.y.to_le_bytes());

    commands.write(EditRectangleCmd {
        wall,
        center: moved.center,
        dimensions: moved.size,
        orientation: moved.axis,
    });
}

/// B 0x140ad6250 (system shell).
pub fn ui_edit_rectangle_dims(
    input: Res<RectangleDimsEditInput>,
    inner: Res<InnerWalls>,
    mut state: ResMut<RectangleEditUiState>,
    mut preview: ResMut<super::super::PreviewRectangle>,
    mut commands: EventWriter<EditRectangleCmd>,
) {
    if !input.submitting {
        return;
    }
    let Some(edge) = input.edge else {
        return;
    };
    do_ui_edit_rectangle_dims(
        input.wall,
        edge,
        input.cursor,
        &inner,
        &mut state,
        &mut preview,
        &mut commands,
    );
}
