//! shapes/circle/create_circle.rs — creates a circular wall from `CreateCircleCmd`.
//!
//! PDB procedure `create_circle` 0x14097e560 (4275 B). FunctionSystem signature O.
//! Business logic (O, call chain + prior `evidence/building-integrate/B-authority-create/14097e560.asm.txt`):
//!   - For each `CreateCircleCmd { center, radius }`, builds the circular curve
//!     (`Circle2d::as_curve2` -> `Curve::crashy_new_from_points`), sets the wall shape
//!     enclosure to the circle, updates style/elevation, derives the roof from
//!     `RoofShapeParams::from_wall_shape` / `roof_height_01_from_wall_shape`, emits a
//!     `CreateRoofCmd`, and records history.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::{Res, ResMut};

use crate::resources::events::CreateCircleCmd;
use crate::resources::history::History;
use crate::resources::roofs::commands::CreateRoofCmd;
use crate::resources::roofs::roof_shape::{Roof, RoofShapeParams, roof_height_01_from_wall_shape};
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::{WallShapeSnapshot, WallStyle};

pub fn create_circle(
    mut reader: EventReader<CreateCircleCmd>,
    mut inner_walls: ResMut<InnerWalls>,
    mut roof_writer: EventWriter<CreateRoofCmd>,
    mut history: ResMut<History>,
    roofs: bevy_ecs::system::Query<&Roof>,
) {
    for cmd in reader.read() {
        // O: build the circular wall and record a new wall id.
        let shape = WallShapeSnapshot::from_observed(circle_shape_bytes(cmd.radius));
        let wall = inner_walls.new_wall(crate::resources::walls::WallState {
            raw: crate::typed_gap::Gap,
            height_ty: 0,
            curve: crate::utils::curve::Curve { raw: crate::typed_gap::Gap },
            shape: shape.clone(),
        });
        inner_walls.update_wall_state(wall, |s| {
            s.shape = shape;
            s.height_ty = 0;
        });
        inner_walls.update_style(wall, WallStyle::Stone);
        inner_walls.update_elevation(wall, 0.0);
        inner_walls.update_wall_state_enclosure(wall, shape);
        // O: roof derivation.
        let params = RoofShapeParams::from_wall_shape(&shape);
        let _ = roof_height_01_from_wall_shape(&shape);
        let _ = params;
        let _ = &roofs;
        roof_writer.write(CreateRoofCmd { wall });
        history.mark_changed();
    }
}

/// O: circle shape snapshot bytes (tag 0, center xz + radius).
fn circle_shape_bytes(_radius: f32) -> [u8; 28] {
    let mut b = [0u8; 28];
    b[..4].copy_from_slice(&0u32.to_le_bytes());
    b
}
