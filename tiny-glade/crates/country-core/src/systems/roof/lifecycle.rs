//! Original B lifecycle policies with PublicWalls as the sole association owner.
use crate::{
    recovered_building::{
        roof_associations::{AssociationAction, association_action},
        roof_components::prepare_resolved_roof_components,
        roof_deletion::{match_deletion_request, should_despawn},
        roof_invalid_notifications::{
            InvalidRoofRow, process_invalid_roof_notifications as invalid_roofs,
        },
        roof_spatial::synchronize_resolved_roof,
        wall_state::WallId as ObservedWallId,
    },
    resources::{
        roofs::{
            Roof, RoofAnimation, RoofMarkedForDeletion, RoofTileSimulation, RoofTriggerVolume,
            commands::*,
        },
        time::GameTime,
        walls::{
            WallId,
            public_walls::{OnWallChanged, PublicWalls},
        },
    },
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;
use hashbrown::HashMap;

/// B 0x1421aaea0. Missing walls and unsupported shapes fail the original
/// invariant. Commands own the entity and deferred component insertion.
pub fn create_roof(
    mut commands: Commands,
    mut requests: EventReader<CreateRoofCmd>,
    walls: Res<PublicWalls>,
    mut gable: EventWriter<WallGableChanged>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut sounds: EventWriter<RoofSound>,
) {
    for request in requests.read() {
        let id = WallId(request.0.wall_id().0);
        let wall = walls.get(id).expect("create_roof requires a public wall");
        let prepared = prepare_resolved_roof_components(
            [0; 88],
            [0; 24],
            &request.0,
            &wall.building_projection(),
        )
        .expect("create_roof requires a supported wall with a valid height cache");
        // Only ABI padding survives the initializer: Roof[0x55..0x58] and
        // TileSimulation[21..24]. Zeroing those bytes defines no game fields.
        let roof = Roof::from_observed(prepared.roof);
        let pivot = roof.pivot_3d().to_array();
        let pitched = roof.is_pitched();
        let style = roof.0[0x54];
        let entity = commands
            .spawn((
                roof,
                RoofAnimation(prepared.animation),
                RoofTileSimulation(prepared.tile_simulation),
                RoofTriggerVolume::default(),
                // 1421ab2bf..2f2: zero translation, identity rotation, unit scale.
                Transform::identity(),
            ))
            .id();
        sounds.write(RoofSound {
            entity,
            wall: id,
            pivot,
            kind: if pitched { 0 } else { 6 },
            flat_surface: (!pitched).then_some(style),
        });
        geometry.write(RoofGeometryChanged {
            entity,
            wall: id,
            kind: 0,
        });
        if let Some(id) = prepared.creation.gable_notification {
            gable.write(WallGableChanged(WallId(id.0)));
        }
    }
}

/// The source consumes the same AppMode resource as the mode manager. Its
/// association invariant forbids invalid live roofs in modes 13 and 14.
pub fn wall_roof_hashmap(
    mode: Res<crate::systems::mode_manager::AppMode>,
    roofs: Query<(Entity, &Roof, Has<RoofMarkedForDeletion>)>,
    mut walls: ResMut<PublicWalls>,
    mut invalid: EventWriter<RoofIsInvalid>,
    mut presence: EventWriter<WallRoofPresenceChanged>,
) {
    let mut entities = HashMap::new();
    for (entity, roof, marked) in &roofs {
        let id = roof.wall_id();
        let projection = walls.get(id).map(|wall| wall.building_projection());
        let action = association_action(
            projection.as_ref(),
            marked,
            mode.native_tag(),
            entities.contains_key(&id),
        )
        .expect("invalid wall/roof association");
        match action {
            AssociationAction::Insert => {
                entities.insert(id, entity);
            }
            AssociationAction::KeepDeleting => {}
            AssociationAction::RequestDeletion => {
                invalid.write(RoofIsInvalid(id));
            }
        }
    }
    for wall in walls.replace_roof_entities(entities) {
        presence.write(WallRoofPresenceChanged(wall));
    }
}

/// B 0x1421a7710. Each queried roof is updated once even if the event batch
/// repeats its wall; style and shape each emit their own kind-2 notification.
pub fn update_roof_spatial(
    mut changed: EventReader<OnWallChanged>,
    walls: Res<PublicWalls>,
    mut roofs: Query<(Entity, &mut Roof, &mut Transform), Without<RoofMarkedForDeletion>>,
    mut geometry: EventWriter<RoofGeometryChanged>,
) {
    let ids: Vec<_> = changed.read().map(|event| event.0).collect();
    for (entity, mut roof, mut transform) in &mut roofs {
        let id = roof.wall_id();
        if !ids.contains(&id) {
            continue;
        }
        let Some(wall) = walls.get(id) else {
            continue;
        };
        let mut translation_projection = [0; 48];
        let (mut roof_tick, mut transform_tick) = (0, 0);
        let outcome = synchronize_resolved_roof(
            &mut roof.0,
            &mut translation_projection,
            &wall.building_projection(),
            ObservedWallId(id.0),
            entity.to_bits(),
            &mut roof_tick,
            &mut transform_tick,
            1,
        )
        .expect("update_roof_spatial requires a valid published wall");
        transform.translation = glam::Vec3::from_array(core::array::from_fn(|index| {
            let offset = 16 + index * 4;
            f32::from_le_bytes(
                translation_projection[offset..offset + 4]
                    .try_into()
                    .unwrap(),
            )
        }));
        for change in outcome.geometry_changes {
            geometry.write(RoofGeometryChanged {
                entity,
                wall: id,
                kind: change.observed_kind,
            });
        }
    }
}

/// B invalid-roof policy preserves reset precedence and history payloads.
/// RecordRoofHistory is an explicit pending mutation, not a history commit.
pub fn process_invalid_roof_notifications(
    roofs: Query<(&Roof, Has<RoofMarkedForDeletion>)>,
    mut invalid: EventReader<RoofIsInvalid>,
    mut resets: EventReader<ResetRoofs>,
    mut deletion: EventWriter<DeleteRoofCmd>,
    mut history: EventWriter<RecordRoofHistory>,
) {
    let ids: Vec<_> = invalid
        .read()
        .map(|event| ObservedWallId(event.0.0))
        .collect();
    let reset = resets.read().count() != 0;
    let rows: Vec<_> = roofs
        .iter()
        .map(|(roof, marked)| InvalidRoofRow {
            roof: roof.observed(),
            marked_for_deletion: marked,
        })
        .collect();
    for request in invalid_roofs(&rows, &ids, reset) {
        deletion.write(DeleteRoofCmd(WallId(request.wall_id.0)));
        if let Some(mutation) = request.history_mutation {
            history.write(RecordRoofHistory(mutation));
        }
    }
}

pub fn mark_roof_for_deletion(
    mut commands: Commands,
    mut requests: EventReader<DeleteRoofCmd>,
    roofs: Query<(Entity, &Roof), Without<RoofMarkedForDeletion>>,
    mut sound: EventWriter<RoofSound>,
) {
    let ids: Vec<_> = requests
        .read()
        .map(|event| ObservedWallId(event.0.0))
        .collect();
    for (entity, roof) in &roofs {
        if let Some(mark) = match_deletion_request(&ids, entity.to_bits(), roof.observed()) {
            commands.entity(entity).insert(RoofMarkedForDeletion);
            sound.write(RoofSound {
                entity,
                wall: roof.wall_id(),
                pivot: mark.pivot,
                kind: mark.sound_event_kind,
                flat_surface: None,
            });
        }
    }
}

pub fn wall_height_dt(mut roofs: Query<(&Roof, &mut RoofTileSimulation)>) {
    for (roof, mut simulation) in &mut roofs {
        simulation.update_wall_height(roof.floor_y());
    }
}

pub fn roof_animation(
    time: Res<GameTime>,
    mut roofs: Query<(&Roof, &mut RoofAnimation, Has<RoofMarkedForDeletion>)>,
) {
    for (roof, mut animation, marked) in &mut roofs {
        // An unchanged delta must not set a Bevy changed tick merely because a
        // mutable query row was visited. The recovered leaf writes this marker
        // only when one channel's delta is nonzero (including unordered NaN).
        let mut wrote = 0;
        animation.bypass_change_detection().0.advance(
            roof.observed(),
            marked,
            time.delta,
            &mut wrote,
            1,
        );
        if wrote != 0 {
            animation.set_changed();
        }
    }
}

pub fn delete_roofs(
    mut commands: Commands,
    roofs: Query<(Entity, &RoofAnimation), With<RoofMarkedForDeletion>>,
) {
    for (entity, animation) in &roofs {
        if should_despawn(true, &animation.0) {
            commands.entity(entity).despawn();
        }
    }
}

/// Connect roof edits and association presence to the public wall's owned
/// height cache. Removing the last roof restores the regular wall cache.
pub fn update_wall_gable_heights(
    mut gable: EventReader<WallGableChanged>,
    mut spatial: EventReader<OnWallChanged>,
    mut presence: EventReader<WallRoofPresenceChanged>,
    roofs: Query<&Roof>,
    mut walls: ResMut<PublicWalls>,
) {
    let mut ids: Vec<_> = gable
        .read()
        .map(|event| event.0)
        .chain(spatial.read().map(|event| event.0))
        .chain(presence.read().map(|event| event.0))
        .collect();
    ids.sort_unstable_by_key(|id| id.0);
    ids.dedup();
    for id in ids {
        let roof = walls
            .roof_entity(id)
            .and_then(|entity| roofs.get(entity).ok());
        if let Some(wall) = walls.get_mut(id) {
            wall.update_gable_heights(roof.map(Roof::observed))
                .expect("published roof requires compatible wall gable geometry");
        }
    }
}
