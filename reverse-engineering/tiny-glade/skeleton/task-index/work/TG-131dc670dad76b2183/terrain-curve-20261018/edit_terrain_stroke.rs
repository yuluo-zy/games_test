//! B0x14094e000 point/parameter edits. The original FunctionSystem declares
//! EventWriter<PlayTerrainSound>; folded grow_one aliases do not identify stairs.
//! Reuse current tg_terrain::edit::apply_resolved numeric/audio observations.
use super::{
    PlayTerrainSound, RecalculateHeightMapCmd,
    delayed_terrain_changed_msg::ScheduledTerrainReadback,
};
use crate::resources::{
    terrain_editing::{StrokeId, TerrainEditingData},
    time::GameTime,
};
use bevy_ecs::prelude::*;
use glam::Vec3;
use tg_terrain::edit::{EditCommand, StrokeParameters, apply_resolved};

#[derive(Event, Clone, Copy, Debug)]
pub struct EditTerrainStrokeCmd(pub EditCommand);

/// Native payload Vec@0, position@0x18, source ID@0x24; event ID follows at0x28.
#[derive(Event, Clone, Debug)]
pub struct SetTerrainStrokePtsCmd {
    pub points: Vec<[f32; 3]>,
    pub position: Vec3,
    pub id: StrokeId,
}

pub fn edit_terrain_stroke(
    mut edits: EventReader<EditTerrainStrokeCmd>,
    mut point_sets: EventReader<SetTerrainStrokePtsCmd>,
    mut terrain: ResMut<TerrainEditingData>,
    mut sounds: EventWriter<PlayTerrainSound>,
    readback: Res<ScheduledTerrainReadback>,
    mut recalculate: EventWriter<RecalculateHeightMapCmd>,
    time: Res<GameTime>,
) {
    for command in point_sets.read() {
        let Some(stroke) = terrain.strokes.get_mut(&command.id) else {
            // fresh 第一循环错误串（@0x142ab3a20）："Was requested to set points of
            // terrain stroke {} that doesnt exist"（第二循环才是 "edit a terrain stroke"）。
            log::error!(
                "Was requested to set points of terrain stroke {} that doesnt exist",
                command.id.0
            );
            continue;
        };
        // Original indexing asserts before clearing the old point vector.
        let old = stroke.points[0];
        let new = command.points[0];
        let dx = old[0] - new[0];
        let dy = old[1] - new[1];
        let dz = old[2] - new[2];
        // RNE 0x14094e6be/6cd: y*y+x*x, then z*z + that sum.
        let magnitude =
            (time.delta * (dz * dz + (dy * dy + dx * dx)).sqrt()) / f32::from_bits(0x3a83126f);
        stroke.points.clone_from(&command.points);
        sounds.write(PlayTerrainSound::Points {
            position: command.position,
            falloff: stroke.falloff,
            height: stroke.height_ws,
            magnitude,
        });
        recalculate.write(RecalculateHeightMapCmd);
        readback.on_terrain_modified();
    }
    for event in edits.read() {
        let command = event.0;
        let Some(stroke) = terrain.strokes.get_mut(&command.id) else {
            log::error!(
                "Was requested to edit a terrain stroke {} that doesnt exist",
                command.id.0
            );
            continue;
        };
        // Reuse the already instruction-verified private game algorithm.
        let outcome = apply_resolved(
            StrokeParameters {
                falloff: stroke.falloff,
                height: stroke.height_ws,
            },
            command,
        );
        stroke.falloff = outcome.state.falloff;
        stroke.height_ws = outcome.state.height;
        let position = Vec3::from_array(command.position);
        for sample in outcome.audio {
            sounds.write(match sample.tag {
                1 => PlayTerrainSound::Falloff {
                    position,
                    falloff: sample.falloff,
                    magnitude: sample.magnitude,
                },
                2 => PlayTerrainSound::Height {
                    position,
                    falloff: stroke.falloff,
                    height: sample.height.expect("height audio payload"),
                    magnitude: sample.magnitude,
                },
                4 => PlayTerrainSound::Removed {
                    position,
                    falloff: stroke.falloff,
                    magnitude: sample.magnitude,
                },
                _ => unreachable!("apply_resolved returned an unknown terrain audio tag"),
            });
        }
        recalculate.write(RecalculateHeightMapCmd);
        readback.on_terrain_modified();
    }
}
