//! B `systems/wall/shapes/rectangle/signifier_edit_dims_line.rs`: dims edit
//! line signifier (`rectangle_edit_dims_line_` prefix).
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs.md`
//! (O, 0x1408dda10). The four edges of the rectangle preview are emitted as
//! edit-line signifiers.

use bevy_ecs::prelude::*;

use super::super::PreviewRectangle;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// B 0x1408dda10: emit the four dims-edit edge lines.
pub fn signifier_edit_dims(
    preview: Res<PreviewRectangle>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let rect = ::utils::geometry::Rectangle2d {
        axis: preview.orientation,
        center: preview.center,
        size: preview.dimensions,
    };
    let points = rect.as_points2();
    for edge in 0..4 {
        let a = points[edge];
        let b = points[(edge + 1) % 4];
        let mut extra = [0u8; 12];
        let floats = [b.x, 0.0f32, b.y].map(f32::to_le_bytes);
        for (i, bytes) in floats.iter().enumerate() {
            extra[i * 4..i * 4 + 4].copy_from_slice(bytes);
        }
        stream.write(UiAppendSignifier {
            tag: 6,
            position: glam::Vec3::new(a.x, 0.0, a.y),
            extra,
        });
    }
}

/// B 0x1408e0e80: component-registry slice (host 待接线).
pub fn register_required_components() -> ([u64; 0], [u64; 0]) {
    ([], [])
}
