//! B `systems/wall/shapes/rectangle/ui_modes/edit_dims.rs`: dims edit widget.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs.md`
//! (O, UiRectangleDimsEdit::signifier 0x14155eba0 / widget 0x14155fc70).
//! The widget signifier samples the four rectangle corners and emits a
//! `dims_edit_` preview line; the trigger-volume widget layer is render host
//! 待接线.

use bevy_ecs::prelude::*;

use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};
use crate::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::distance_to_edge;

/// B 0x14155eba0: dims-edit signifier from the preview rectangle.
pub fn ui_rectangle_dims_edit_signifier(
    preview: Res<crate::systems::wall::shapes::PreviewRectangle>,
    cursor: Res<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let rect = ::utils::geometry::Rectangle2d {
        axis: preview.orientation,
        center: preview.center,
        size: preview.dimensions,
    };
    let cursor_pos = glam::Vec2::new(cursor.pos.x, cursor.pos.z);
    // Emit the closest edge distance as the edit line position.
    let (mut best_edge, mut best) = (0usize, f32::MAX);
    for edge in 0..4 {
        let distance = distance_to_edge(rect, edge, cursor_pos).abs();
        if distance < best {
            best = distance;
            best_edge = edge;
        }
    }
    let (a, _b) = rect.get_edge_pts(best_edge);
    let mut extra = [0u8; 12];
    extra[..4].copy_from_slice(&best.to_le_bytes());
    // 40B 布局（中央裁决）：best 值入 scale 槽。
    stream.write(UiAppendSignifier {
        tag: 6,
        kind: 0,
        position: glam::Vec3::new(a.x, 0.0, a.y),
        scale: best,
        name: 0,
    });
}

/// B 0x14155fc70: widget layer (host 待接线).
pub fn ui_rectangle_dims_edit_widget() {}
