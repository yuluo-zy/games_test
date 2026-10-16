//! B `systems/wall/shapes/rectangle/signifier_edit_dims_line.rs`: dims edit
//! line signifier (`rectangle_edit_dims_line_` prefix).
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs.md`
//! (O, 0x1408dda10). The four edges of the rectangle preview are emitted as
//! edit-line signifiers.

use bevy_ecs::prelude::*;

use super::super::PreviewRectangle;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// 打包两个 f32 到 qword（I）。
fn pack2(v: glam::Vec2) -> u64 {
    u64::from(v.x.to_bits()) | (u64::from(v.y.to_bits()) << 32)
}

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
        // 40B 布局（中央裁决）：端点半打包入 name 槽。
        stream.write(UiAppendSignifier {
            tag: 6,
            kind: 0,
            position: glam::Vec3::new(a.x, 0.0, a.y),
            scale: 0.0,
            name: pack2(b),
        });
    }
}

/// B 0x1408e0e80: component-registry slice (host 待接线).
pub fn register_required_components() -> ([u64; 0], [u64; 0]) {
    ([], [])
}
