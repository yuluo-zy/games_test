//! B `systems/wall/snapping/telegraph_walls_are_merged.rs`: merge preview line.
//!
//! Evidence: `chapters/systems/wall/snapping/telegraph_walls_are_merged.rs.md`
//! (O, structure; 0x14086ad70). For every public wall a preview curve is built
//! by offsetting the simplified Curve2 by `f32@+0x284 * 0.5` (the +0x284 field
//! name is 待取证) and pushed as a kind-5 signifier. The PlayLineAcrossWall
//! source pairing and callback registration are 待接线 (render host).

use bevy_ecs::prelude::*;

use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// B 0x14086ad70 (8-param system).
pub fn telegraph_walls_are_merged(
    public: Res<PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
) {
    for (_id, state) in public.all() {
        state.assert_valid();
        let curve = state.simplified_curve_2d();
        if curve.points.len() < 2 {
            continue;
        }
        // Offset field f32@+0x284 * 0.5 applied along one lateral axis. The
        // +0x284 field role is 待取证; the observed shift is per-point.
        let offset = state.building_projection().float_0x284() * 0.5;
        let first = curve.points[0];
        let shifted = curve
            .points
            .iter()
            .map(|p| [p[0] + offset, p[1]])
            .collect::<Vec<_>>();
        let _ = first; // endpoint retained; exact preview pairing 待取证.
        let start = shifted[0];
        let end = *shifted.last().unwrap();
        stream.write(UiAppendSignifier {
            tag: 5,
            position: glam::Vec3::new(start[0], 0.0, start[1]),
            extra: {
                let mut extra = [0u8; 12];
                for (i, bytes) in [end[0], 0.0f32, end[1]]
                    .map(f32::to_le_bytes)
                    .iter()
                    .enumerate()
                {
                    extra[i * 4..i * 4 + 4].copy_from_slice(bytes);
                }
                extra
            },
        });
    }
}
