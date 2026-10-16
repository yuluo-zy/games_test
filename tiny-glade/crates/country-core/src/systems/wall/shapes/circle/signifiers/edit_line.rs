//! B `systems/wall/shapes/circle/signifiers/edit_line.rs`: circle radius line.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/signifiers/edit_line.rs.md`
//! (O, 0x140a64a40). Draws the edit radius line from the circle center toward
//! the ring at angle offsets 0.23 / 0.75 · 90°. The entity query/spawn is
//! render host 待接线; the library writes the signifier record.

use bevy_ecs::prelude::*;
use ::utils::geometry::Circle2d;

use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// 打包两个 f32 到 qword（I：40B 布局裁决后的端点承载）。
fn pack2(v: glam::Vec2) -> u64 {
    u64::from(v.x.to_bits()) | (u64::from(v.y.to_bits()) << 32)
}

/// Radius-line angular offsets (0x3e6b851f @0x142a89600 / 0x3f400000
/// @0x1429edb90) and 90.0 scale.
const LINE_ANGLE_OFFSET: f32 = 0.23;
const LINE_ANGLE_SCALE: f32 = 0.75;
const LINE_ANGLE_FULL: f32 = 90.0;

/// B 0x140a64a40 (4-param system): circle center -> ring endpoint.
pub fn signifier_edit_circle_line(
    preview: Res<super::super::super::PreviewCircle>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let circle = Circle2d::new(preview.center, preview.radius);
    let a = LINE_ANGLE_OFFSET * LINE_ANGLE_FULL;
    let b = LINE_ANGLE_SCALE * LINE_ANGLE_FULL;
    let start = circle.get_pos_at_u(a / 360.0);
    let end = circle.get_pos_at_u(b / 360.0);
    let mut extra = [0u8; 12];
    for (i, bytes) in [end.x, 0.0f32, end.y]
        .map(f32::to_le_bytes)
        .iter()
        .enumerate()
    {
        extra[i * 4..i * 4 + 4].copy_from_slice(bytes);
    }
    // 40B 布局（中央裁决）：端点半打包入 name 槽（I 映射，原 12B extra 保留语义）。
    stream.write(UiAppendSignifier {
        tag: 4,
        kind: 0,
        position: glam::Vec3::new(start.x, 0.0, start.y),
        scale: 0.0,
        name: pack2(end),
    });
}

/// B 0x140a66c90: component-registry slice (host 待接线).
pub fn register_required_components() -> ([u64; 0], [u64; 0]) {
    ([], [])
}
