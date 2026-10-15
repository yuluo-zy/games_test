//! B edit, direction and flat-style consumers over the owned Roof component.
use crate::{
    recovered_building::{
        building_history_transactions::RoofHistoryEdit,
        roof_customization::{apply_flat_rim_resolved, apply_flat_style_resolved},
        roof_direction::{RoofDirectionRequest, apply_resolved_direction},
        wall_state::WallId as ObservedWallId,
    },
    resources::{
        roofs::{Roof, commands::*},
        walls::{WallId, public_walls::PublicWalls},
    },
};
use bevy_ecs::prelude::*;

pub fn edit_roof(
    mut requests: EventReader<EditRoofCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableChanged>,
    mut feedback: EventWriter<RoofEditFeedback>,
    mut crossing: EventWriter<RoofTypeChanged>,
) {
    for request in requests.read() {
        let id = WallId(request.0.wall_id);
        let entity = walls
            .roof_entity(id)
            .expect("edit_roof requires a roof association");
        let mut roof = roofs
            .get_mut(entity)
            .expect("edit_roof associated entity has no Roof");
        let mut changed = 0;
        let result = system_roof::edit_rules::apply_edit(
            &mut roof.bypass_change_detection().0,
            &request.0,
            &mut changed,
            1,
        );
        if changed != 0 {
            roof.set_changed();
        }
        if result.rebuild_gable {
            gable.write(WallGableChanged(id));
        }
        for value in result.feedback {
            feedback.write(RoofEditFeedback {
                entity,
                wall: id,
                feedback: value,
            });
        }
        geometry.write(RoofGeometryChanged {
            entity,
            wall: id,
            kind: 1,
        });
        if let Some(value) = result.height_crossing {
            crossing.write(RoofTypeChanged {
                entity,
                wall: id,
                crossing: value,
                source_flag: request.0.source_flag,
            });
        }
    }
}

pub fn switch_roof_direction(
    mut requests: EventReader<SwitchRoofDirectionCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableChanged>,
    mut sound: EventWriter<RoofSound>,
) {
    for request in requests.read() {
        let entity = walls
            .roof_entity(request.wall)
            .expect("roof direction requires a roof association");
        let mut roof = roofs
            .get_mut(entity)
            .expect("roof direction associated entity has no Roof");
        let mut changed = 0;
        let result = apply_resolved_direction(
            &mut roof.0,
            RoofDirectionRequest {
                wall: ObservedWallId(request.wall.0),
                direction: request.direction,
            },
            entity.to_bits(),
            &mut changed,
            1,
        );
        geometry.write(RoofGeometryChanged {
            entity,
            wall: request.wall,
            kind: result.geometry_kind,
        });
        gable.write(WallGableChanged(request.wall));
        sound.write(RoofSound {
            entity,
            wall: request.wall,
            pivot: result.sound_pivot,
            kind: result.sound_kind,
            flat_surface: None,
        });
    }
}

pub fn set_flat_roof_style(
    mut requests: EventReader<SetFlatRoofStyleCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut holes: EventWriter<RecalculateWallHoles>,
    mut sounds: EventWriter<RoofSound>,
) {
    for request in requests.read() {
        let wall = WallId(request.0.wall_id.0);
        // B 0x142299dd0 skips absent associations/query rows, unlike edit_roof.
        let Some(entity) = walls.roof_entity(wall) else {
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            continue;
        };
        let mut changed = 0;
        let result = apply_flat_style_resolved(&mut roof.0, request.0, &mut changed, 1);
        if let Some(id) = result.recalculate_wall_holes {
            holes.write(RecalculateWallHoles(WallId(id.0)));
        }
        if let Some(sound) = result.sound {
            sounds.write(RoofSound {
                entity,
                wall,
                pivot: sound.pivot,
                kind: sound.observed_kind,
                flat_surface: None,
            });
        }
    }
}

pub fn set_flat_roof_rim_style(
    mut requests: EventReader<SetFlatRoofRimStyleCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut holes: EventWriter<RecalculateWallHoles>,
    mut history: EventWriter<RecordRoofHistory>,
) {
    for request in requests.read() {
        let wall = WallId(request.0.wall_id.0);
        let Some(entity) = walls.roof_entity(wall) else {
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            continue;
        };
        let mut changed = 0;
        let result = apply_flat_rim_resolved(&mut roof.0, request.0, &mut changed, 1);
        if let Some(id) = result.recalculate_wall_holes {
            holes.write(RecalculateWallHoles(WallId(id.0)));
        }
        if let Some(mutation) = result.history_mutation {
            history.write(RecordRoofHistory(RoofHistoryEdit::rim(mutation)));
        }
    }
}

/// B 0x1421ac4f0 gameplay effects of the type transition. The original drawable,
/// VAO and transient-mesh commands still require their rendering owners.
pub fn apply_roof_type_gameplay(
    mut requests: EventReader<RoofTypeChanged>,
    roofs: Query<&Roof>,
    mut walls: ResMut<crate::resources::walls::InnerWalls>,
    mut gable: EventWriter<WallGableChanged>,
    mut sounds: EventWriter<RoofSound>,
) {
    use system_roof::edit_rules::HeightCrossing;
    for request in requests.read() {
        let roof = roofs
            .get(request.entity)
            .expect("roof type change requires its entity");
        let pitched = roof.is_pitched();
        assert_eq!(
            pitched,
            request.crossing == HeightCrossing::Up,
            "roof type event disagrees with Roof::ty"
        );
        sounds.write(RoofSound {
            entity: request.entity,
            wall: request.wall,
            pivot: roof.pivot_3d().to_array(),
            kind: if pitched { 0 } else { 6 },
            flat_surface: (!pitched).then_some(roof.0[0x54]),
        });
        // Flat transitions also notify when ridge==1; is_gable() would suppress
        // that notification because the height has already become flat.
        if f32::from_le_bytes(roof.0[0x34..0x38].try_into().unwrap()) == 1.0 {
            gable.write(WallGableChanged(roof.wall_id()));
        }
        if !pitched && request.source_flag == 1 {
            let height = walls.get_unwrapped(request.wall).height();
            if height != 1.6 {
                // 1421acc7e..8a: MAXSS(1.6,height), MINSS(14,result).
                // Preserve second-operand NaN behavior and equal-value writes
                // for all source values other than exactly 1.6.
                let minimum = if 1.6 > height { 1.6 } else { height };
                let bounded = if 14.0 < minimum { 14.0 } else { minimum };
                walls.update_wall_state_height(request.wall, bounded);
            }
        }
    }
}
