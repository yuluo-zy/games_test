//! B `systems/wall/shapes/shape_placement_signifiers.rs`: placement preview.
//!
//! Evidence: `chapters/systems/wall/shapes/shape_placement_signifiers.rs.md`
//! (O, 0x1408dfc60 / 0x1408dec70). `preview_snapping_line` builds the
//! expanded premade-shape anchor used by snapping_signifiers; the signifier
//! drawing itself (DebugLine/DebugSphere, Commands spawn) is host 待接线, so
//! the library writes [`UiAppendSignifier`] records to the signifier stream.

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use crate::resources::walls::public_wall_state::PublicWallState;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};
use crate::systems::wall::valid_enclosure::ShapeParameters;

/// Circle expand radius (0x3ea147ae @0x142aa3280).
const CIRCLE_EXPAND: f32 = 0.315;
/// Rectangle per-axis expand (0x3f2147ae @0x142aa4550/4554) = 2 * CIRCLE_EXPAND.
const RECTANGLE_EXPAND: f32 = 0.63;

/// B 0x1408dfc60: the expanded premade-shape anchor point (u = 0) used as the
/// start of the snapping preview line. `None` for freehand walls or when no
/// shape is present.
pub fn preview_snapping_line(state: &PublicWallState) -> Option<Vec3> {
    let shape = state.try_get_wall_shape()?;
    let params = ShapeParameters::from_snapshot(shape);
    let anchor = match params.tag() {
        0 => {
            let circle = params.as_circle();
            let expanded = ::utils::geometry::Circle2d {
                center: circle.center,
                radius: circle.radius + CIRCLE_EXPAND,
                rotation: circle.rotation,
            };
            expanded.get_pos_at_u(0.0)
        }
        1 => {
            let rect = params.as_rectangle();
            let expanded = ::utils::geometry::Rectangle2d::with_expanded_size(
                rect,
                Vec2::splat(RECTANGLE_EXPAND),
            );
            expanded.get_pos_at_u(0.0)
        }
        _ => return None,
    };
    Some(glam::Vec3::new(anchor.x, state.calculated_max_y().unwrap_or(0.0), anchor.y))
}

/// B 0x1408dec70: shape placement signifier system — draw the snapping preview
/// line and the shape outline. The outline spawn/mesh is host 待接线; the
/// library records the snapping anchor.
pub fn shape_placement_signifiers(
    mode: Res<crate::systems::wall::snapping::SnappingMode>,
    public: Res<crate::resources::walls::public_walls::PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if mode.wall.0 == 0 {
        return;
    }
    let Some(wall) = public.get(mode.wall) else {
        return;
    };
    if let Some(anchor) = preview_snapping_line(wall) {
        stream.write(UiAppendSignifier {
            tag: 3,
            kind: 0,
            position: anchor,
            scale: 0.0,
            name: 0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freehand_wall_has_no_preview_anchor() {
        let mut world = World::new();
        world.insert_resource(crate::systems::wall::snapping::SnappingMode::new());
        world.insert_resource(crate::resources::walls::public_walls::PublicWalls::default());
        world.init_resource::<UiSignifierStream>();
        let mut schedule = Schedule::default();
        schedule.add_systems(shape_placement_signifiers);
        schedule.run(&mut world);
        assert!(world.resource_mut::<UiSignifierStream>().drain().count() == 0);
    }
}
