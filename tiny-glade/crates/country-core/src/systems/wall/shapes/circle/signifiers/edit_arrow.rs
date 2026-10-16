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
            position: glam::Vec3::new(circle.center.x, 0.0, circle.center.y),
            extra: {
                let mut extra = [0u8; 12];
                let dir = circle.get_tangent_at_u(0.0);
                let floats = [dir.x, dir.y, circle.rotation].map(f32::to_le_bytes);
                for (i, bytes) in floats.iter().enumerate() {
                    extra[i * 4..i * 4 + 4].copy_from_slice(bytes);
                }
                extra
            },
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
