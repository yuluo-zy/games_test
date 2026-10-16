//! B `systems/wall/shapes/circle/signifiers/edit_arrow.rs`: circle edit arrow.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/signifiers/edit_arrow.rs.md`
//! (O, 0x140b35120). Freehand walls (enclosure tag == 2) get no arrow; premade
//! circles draw an `edit_circle_` arrow at the wall position/orientation.
//! The entity spawn is render host 待接线; the library writes the signifier.

use bevy_ecs::prelude::*;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// B 0x140b35120 (3-param system).
pub fn signifier_edit_circle_arrow(
    public: Res<PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
) {
    for (_id, state) in public.all() {
        if state.shape().tag() != 0 {
            continue; // freehand (2) and rectangle (1) get no circle arrow.
        }
        let circle = state.is_circle().unwrap();
        stream.write(UiAppendSignifier {
            tag: 5,
            kind: 0,
            position: glam::Vec3::new(circle.center.x, 0.0, circle.center.y),
            // I：切线方向 {x, y} 打包入 name 槽、rotation 入 scale（40B 布局裁决后
            // 的映射；原 12B extra 三浮点语义保留）。
            scale: circle.rotation,
            name: pack2(circle.get_tangent_at_u(0.0)),
        });
    }
}

/// B 0x140b35ba0: component-registry slice (host 待接线).
pub fn register_required_components() -> ([u64; 0], [u64; 0]) {
    ([], [])
}

/// B 0x140002370: clone behavior marker (returns 0).
pub fn clone_behavior() -> u8 {
    0
}

/// 打包两个 f32 到 qword（I：40B 布局裁决后的切线方向承载）。
fn pack2(v: glam::Vec2) -> u64 {
    u64::from(v.x.to_bits()) | (u64::from(v.y.to_bits()) << 32)
}
