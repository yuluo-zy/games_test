//! shapes/rectangle/edit_rectangle.rs — applies live rectangle dimension edits.
//!
//! PDB procedure `edit_rectangle` 0x14097f620 (834 B). FunctionSystem signature O.
//! Business logic (O, decompiled 0x14097f620): for each `EditRectangleCmd { wall, dims }`,
//! rebuilds the wall curve/enclosure from the new rectangle dims via
//! `InnerWalls::update_wall_state_curve` + `update_wall_state_enclosure`.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::ResMut;

use crate::audio_events::{PlayBuildingAudio, RequestWallMaterialAudio};
use crate::resources::events::EditRectangleCmd;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallShapeSnapshot;

pub fn edit_rectangle(
    mut reader: EventReader<EditRectangleCmd>,
    mut inner_walls: ResMut<InnerWalls>,
    mut material_writer: EventWriter<RequestWallMaterialAudio>,
    mut building_writer: EventWriter<PlayBuildingAudio>,
) {
    for cmd in reader.read() {
        // O: rebuild the rectangle wall from the new dims.
        let mut shape = [0u8; 28];
        shape[..4].copy_from_slice(&1u32.to_le_bytes());
        shape[4..12].copy_from_slice(&cmd.dims.to_array().into_iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<_>>());
        let shape = WallShapeSnapshot::from_observed(shape);
        inner_walls.update_wall_state_curve(
            cmd.wall,
            crate::utils::curve::Curve { raw: crate::typed_gap::Gap },
        );
        inner_walls.update_wall_state_enclosure(cmd.wall, shape);
        material_writer.write(RequestWallMaterialAudio {
            wall: cmd.wall,
            pos: glam::Vec3::ZERO,
            raw: crate::typed_gap::Gap,
        });
        building_writer.write(PlayBuildingAudio { id: 0, position: glam::Vec3::ZERO, editing_speed: 1.0, kind: 0 });
    }
}
