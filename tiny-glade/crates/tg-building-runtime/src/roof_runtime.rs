//! Runs roof creation, association, edits, animation, deletion, and CPU tile projection.
//! Geometry is validated before admission or mutation; failed projections withdraw stale output.
use crate::{
    Animation, BuildingFrame, BuildingReport, CreateRoof, DeleteRoof, ObjectTransform, PublicWalls,
    RecalculateWallHoles, ReplayRoofEvent, Roof, RoofBundle, RoofCpuTiles, RoofGeometryChanged,
    RoofMarkedForDeletion, RoofRimHistoryRequests, RoofSound, RoofSpire, RoofSupportBeams,
    RoofTiles, SetFlatRoofRimStyle, SetFlatRoofStyle, SwitchRoofDirection, TileSimulation,
    TriggerVolume, WallGableDirty, WallRoofMap, WallRoofPresenceChanged, WallSpatialDirty, nani,
    roof_invalid, roof_record::RoofRecord,
};
use bevy_ecs::prelude::*;
use country_core::resources::roofs::associations::ResolvedRoofAssociation;
use country_core::resources::roofs::components::RoofTileSimulation;
use country_core::resources::roofs::components::prepare_resolved_roof_components;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::roofs::deletion::match_deletion_request;
use country_core::resources::roofs::deletion::should_despawn;
use country_core::resources::roofs::spatial::synchronize_resolved_roof;
use country_core::resources::walls::publication_fields::WallId;

/// Validate the semantic geometry fields, leaving enum padding and IDs opaque.
pub(super) fn roof_geometry_is_finite(roof: &[u8; 88]) -> bool {
    let shape = RoofRecord(roof).shape();
    if shape > 1 || roof[0x3c] > 1 {
        return false;
    }
    let shape_end = if shape == 0 { 28 } else { 36 };
    let mut geometry = roof[12..shape_end]
        .as_chunks::<4>()
        .0
        .iter()
        .chain(roof[0x24..0x3c].as_chunks::<4>().0.iter())
        .chain(roof[0x4c..0x50].as_chunks::<4>().0.iter());
    if !geometry.all(|bytes| f32::from_le_bytes(*bytes).is_finite()) {
        return false;
    }
    if roof[0x40] == 2
        && !roof[0x44..0x4c]
            .as_chunks::<4>()
            .0
            .iter()
            .all(|bytes| f32::from_le_bytes(*bytes).is_finite())
    {
        return false;
    }
    RoofRecord(roof).sound_pivot().is_some()
}

pub(super) fn roof_request_is_finite(request: &RoofCreateRequestObserved) -> bool {
    let record = request.record();
    record[32..56]
        .as_chunks::<4>()
        .0
        .iter()
        .all(|bytes| f32::from_le_bytes(*bytes).is_finite())
        && (record[20] != 2
            || record[24..32]
                .as_chunks::<4>()
                .0
                .iter()
                .all(|bytes| f32::from_le_bytes(*bytes).is_finite()))
        && record[56] <= 1
}

pub(super) fn create_roofs(
    mut commands: Commands,
    mut requests: EventReader<CreateRoof>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof, Without<RoofMarkedForDeletion>>,
    mut gable: EventWriter<WallGableDirty>,
    mut report: ResMut<BuildingReport>,
) {
    let mut occupied: std::collections::HashSet<_> = roofs
        .iter()
        .map(|roof| RoofRecord(&roof.0).wall())
        .collect();
    for request in requests.read() {
        if !roof_request_is_finite(&request.0) {
            report.faults.push(format!(
                "Invalid roof create geometry {}",
                request.0.wall_id().0
            ));
            continue;
        }
        let Some(wall) = walls.walls.get(&request.0.wall_id()) else {
            continue;
        };
        if occupied.contains(&request.0.wall_id()) {
            report.faults.push(format!(
                "Roof already exists for wall {}",
                request.0.wall_id().0
            ));
            continue;
        }
        match prepare_resolved_roof_components([0; 88], [0; 24], &request.0, &wall.state) {
            Ok(components) => {
                if !roof_geometry_is_finite(&components.roof) {
                    report.faults.push(format!(
                        "Invalid resolved roof geometry {}",
                        request.0.wall_id().0
                    ));
                    continue;
                }
                occupied.insert(request.0.wall_id());
                let entity = commands
                    .spawn(RoofBundle {
                        roof: Roof(components.roof),
                        tile: TileSimulation(components.tile_simulation),
                        animation: Animation(components.animation),
                        trigger: TriggerVolume::default(),
                        tiles: RoofTiles::default(),
                        spire: RoofSpire,
                        supports: RoofSupportBeams,
                        nani: nani::NaniStoneFloorBundle::default(),
                    })
                    .id();
                report.created.push(entity);
                if let Some(id) = components.creation.gable_notification {
                    gable.write(WallGableDirty(id));
                }
            }
            Err(e) => report
                .faults
                .push(format!("Roof create {}: {e:?}", request.0.wall_id().0)),
        }
    }
}

pub(super) fn associations(
    roofs: Query<(Entity, &Roof, Has<RoofMarkedForDeletion>)>,
    walls: Res<PublicWalls>,
    mut map: ResMut<WallRoofMap>,
    frame: Res<BuildingFrame>,
    mut invalid: EventWriter<roof_invalid::RoofIsInvalid>,
    mut presence: EventWriter<WallRoofPresenceChanged>,
    mut report: ResMut<BuildingReport>,
) {
    let rows = roofs.iter().map(|(entity, roof, marked)| {
        let wall_id = RoofRecord(&roof.0).wall();
        ResolvedRoofAssociation {
            wall_id,
            entity: entity.to_bits(),
            marked,
            public_wall: walls.walls.get(&wall_id).map(|w| &w.state),
        }
    });
    match map.0.rebuild(rows, &walls.order, frame.phase) {
        Ok(changes) => {
            for id in changes.invalid_roof_requests {
                invalid.write(roof_invalid::RoofIsInvalid(id));
            }
            for id in changes.wall_presence_changes {
                presence.write(WallRoofPresenceChanged(id));
                report.presence_changes.push(id);
            }
        }
        Err(e) => report.faults.push(format!("Roof association: {e:?}")),
    }
}

pub(super) fn spatial(
    mut dirty: EventReader<WallSpatialDirty>,
    walls: Res<PublicWalls>,
    mut roofs: Query<(Entity, &mut Roof, &mut ObjectTransform), Without<RoofMarkedForDeletion>>,
    frame: Res<BuildingFrame>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut report: ResMut<BuildingReport>,
) {
    let ids: std::collections::HashSet<_> = dirty.read().map(|event| event.0).collect();
    if ids.is_empty() {
        return;
    }
    for (entity, mut roof, mut transform) in &mut roofs {
        let id = RoofRecord(&roof.0).wall();
        if !ids.contains(&id) {
            continue;
        }
        let Some(wall) = walls.walls.get(&id) else {
            continue;
        };
        let mut roof_tick = 0;
        let mut transform_tick = 0;
        let mut next_roof = roof.0;
        let mut next_transform = transform.0;
        match synchronize_resolved_roof(
            &mut next_roof,
            &mut next_transform,
            &wall.state,
            id,
            entity.to_bits(),
            &mut roof_tick,
            &mut transform_tick,
            frame.tick,
        ) {
            Ok(result) => {
                if !roof_geometry_is_finite(&next_roof) {
                    report
                        .faults
                        .push(format!("Roof spatial {} produced invalid geometry", id.0));
                    continue;
                }
                roof.0 = next_roof;
                transform.0 = next_transform;
                for change in result.geometry_changes {
                    let event = RoofGeometryChanged {
                        entity,
                        wall: id,
                        kind: change.observed_kind,
                    };
                    geometry.write(event);
                    report.geometry_changes.push(event);
                }
            }
            Err(e) => report.faults.push(format!("Roof spatial {}: {e:?}", id.0)),
        }
    }
}

pub(super) fn mark_roofs(
    mut commands: Commands,
    mut requests: EventReader<DeleteRoof>,
    roofs: Query<(Entity, &Roof), Without<RoofMarkedForDeletion>>,
    mut sound: EventWriter<RoofSound>,
    mut report: ResMut<BuildingReport>,
) {
    let ids: std::collections::HashSet<_> = requests.read().map(|event| event.0).collect();
    if ids.is_empty() {
        return;
    }
    for (entity, roof) in &roofs {
        let wall = RoofRecord(&roof.0).wall();
        if !ids.contains(&wall) {
            continue;
        }
        if !roof_geometry_is_finite(&roof.0) {
            // Cleanup must remain possible for malformed external components;
            // only geometry-dependent feedback is unsafe to derive here.
            commands.entity(entity).insert(RoofMarkedForDeletion);
            report.marked.push(entity);
            report.faults.push(format!(
                "Roof deletion {} has invalid geometry; sound omitted",
                wall.0
            ));
            continue;
        }
        if let Some(mark) = match_deletion_request(&[wall], entity.to_bits(), &roof.0) {
            commands.entity(entity).insert(RoofMarkedForDeletion);
            report.marked.push(entity);
            let event = RoofSound {
                entity,
                wall: mark.wall_id,
                pivot: mark.pivot,
                kind: mark.sound_event_kind,
            };
            sound.write(event);
            report.sounds.push(event);
        }
    }
}

pub(super) fn animate(
    mut roofs: Query<(
        &Roof,
        &mut Animation,
        Has<RoofMarkedForDeletion>,
        &mut TileSimulation,
    )>,
    frame: Res<BuildingFrame>,
    mut report: ResMut<BuildingReport>,
) {
    for (roof, mut animation, marked, mut tile) in &mut roofs {
        let mut tick = 0;
        // Keep base-height displacement separate from the appearance animation.
        let mut bytes = *tile.0.bytes();
        let current = RoofRecord(&roof.0).base_height();
        let previous = f32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let height_delta = current - previous;
        if current.is_finite() && previous.is_finite() && height_delta.is_finite() {
            bytes[0..4].copy_from_slice(&height_delta.to_le_bytes());
            bytes[4..8].copy_from_slice(&current.to_le_bytes());
            tile.0 = RoofTileSimulation::from_observed(bytes);
        } else {
            report.faults.push(format!(
                "Roof tile simulation {} has an unrepresentable height step",
                RoofRecord(&roof.0).wall().0
            ));
        }
        animation.bypass_change_detection().0.advance(
            &roof.0,
            marked,
            frame.delta_time,
            &mut tick,
            frame.tick,
        );
        if tick == frame.tick {
            animation.set_changed();
        }
    }
}

pub(super) fn delete_roofs(
    mut commands: Commands,
    roofs: Query<(Entity, &Animation), With<RoofMarkedForDeletion>>,
    mut report: ResMut<BuildingReport>,
) {
    for (entity, animation) in &roofs {
        if should_despawn(true, &animation.0) {
            commands.entity(entity).despawn();
            report.despawned.push(entity);
        }
    }
}

pub(super) fn build_cpu_tiles(
    mut commands: Commands,
    roofs: Query<(Entity, &Roof), (Changed<Roof>, Without<RoofMarkedForDeletion>)>,
    mut report: ResMut<BuildingReport>,
) {
    for (entity, roof) in &roofs {
        if !roof_geometry_is_finite(&roof.0) {
            commands.entity(entity).remove::<RoofCpuTiles>();
            report
                .faults
                .push(format!("Roof CPU tiles {entity:?}: invalid geometry"));
            continue;
        }
        let record = RoofRecord(&roof.0);
        if !country_core::resources::roofs::roof_shape::ty_observed(record.pitch()) {
            commands.entity(entity).remove::<RoofCpuTiles>();
            continue;
        }
        let rectangle = record.rectangle();
        match country_core::resources::roofs::geometry::pipeline::assemble_roof_tiles_observed(
            &roof.0,
            (record.shape() == 1).then_some(rectangle),
            false,
        ) {
            Ok(batch) => {
                commands.entity(entity).insert(RoofCpuTiles(batch));
            }
            Err(e) => {
                commands.entity(entity).remove::<RoofCpuTiles>();
                report
                    .faults
                    .push(format!("Roof CPU tiles {entity:?}: {e:?}"));
            }
        }
    }
}

pub(super) fn switch_directions(
    mut events: EventReader<SwitchRoofDirection>,
    map: Res<WallRoofMap>,
    mut roofs: Query<&mut Roof>,
    frame: Res<BuildingFrame>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableDirty>,
    mut sound: EventWriter<RoofSound>,
    mut report: ResMut<BuildingReport>,
) {
    for event in events.read() {
        if event.0.direction > 1 {
            report
                .faults
                .push(format!("Invalid roof direction {}", event.0.wall.0));
            continue;
        }
        let Some(entity) = map.entity(event.0.wall) else {
            report.faults.push(format!(
                "Roof direction missing association {}",
                event.0.wall.0
            ));
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            report
                .faults
                .push(format!("Roof direction invalid entity {entity:?}"));
            continue;
        };
        if !roof_geometry_is_finite(&roof.0) {
            report
                .faults
                .push(format!("Invalid current roof geometry {}", event.0.wall.0));
            continue;
        }
        let mut tick = 0;
        let change =
            country_core::resources::roofs::direction::apply_resolved_direction(
                &mut roof.0,
                event.0,
                entity.to_bits(),
                &mut tick,
                frame.tick,
            );
        let update = RoofGeometryChanged {
            entity,
            wall: change.wall,
            kind: change.geometry_kind,
        };
        geometry.write(update);
        report.geometry_changes.push(update);
        gable.write(WallGableDirty(change.wall));
        let feedback = RoofSound {
            entity,
            wall: change.wall,
            pivot: change.sound_pivot,
            kind: change.sound_kind,
        };
        sound.write(feedback);
        report.sounds.push(feedback);
    }
}

pub(super) fn route_replay(
    mut events: EventReader<ReplayRoofEvent>,
    mut create: EventWriter<CreateRoof>,
    mut delete: EventWriter<DeleteRoof>,
    mut direction: EventWriter<SwitchRoofDirection>,
    mut rim: EventWriter<SetFlatRoofRimStyle>,
) {
    use country_core::resources::roofs::geometry::replay::ObservedReplayEvent;
    for event in events.read() {
        match &event.0 {
            ObservedReplayEvent::Create { .. } => {
                // All authoritative payload bytes come from the explicit
                // presence44. Queue message ID/padding are not game input.
                let mut record = [0; 64];
                event.0.write_observed_fields(&mut record);
                create.write(CreateRoof(RoofCreateRequestObserved::from_observed(record)));
            }
            ObservedReplayEvent::Delete { wall_index } => {
                delete.write(DeleteRoof(WallId(*wall_index)));
            }
            ObservedReplayEvent::Direction {
                wall_index,
                direction: value,
            } => {
                direction.write(SwitchRoofDirection(
                    country_core::resources::roofs::direction::RoofDirectionRequest {
                        wall: WallId(*wall_index),
                        direction: *value,
                    },
                ));
            }
            ObservedReplayEvent::FlatRimStyle { wall_index, style } => {
                rim.write(SetFlatRoofRimStyle(
                    country_core::resources::roofs::customization::FlatRoofRimRequest {
                        wall_id: WallId(*wall_index),
                        from: *style,
                        to: *style,
                        observed_record_history: 0,
                    },
                ));
            }
        }
    }
}

pub(super) fn customize_roofs(
    mut styles: EventReader<SetFlatRoofStyle>,
    mut rims: EventReader<SetFlatRoofRimStyle>,
    map: Res<WallRoofMap>,
    mut roofs: Query<&mut Roof>,
    frame: Res<BuildingFrame>,
    mut holes: EventWriter<RecalculateWallHoles>,
    mut sounds: EventWriter<RoofSound>,
    mut history: ResMut<RoofRimHistoryRequests>,
    mut report: ResMut<BuildingReport>,
) {
    use country_core::resources::roofs::customization::*;
    // Full-style updates precede rim-only updates when both target one roof.
    let requests = styles
        .read()
        .map(|e| (e.0.wall_id, Some(e.0), None))
        .chain(rims.read().map(|e| (e.0.wall_id, None, Some(e.0))));
    for (wall, style, rim) in requests {
        if style.is_some_and(|request| {
            let bytes = request.style.bytes();
            bytes[0] == 2
                && !bytes[4..12]
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .all(|bytes| f32::from_le_bytes(*bytes).is_finite())
        }) {
            report
                .faults
                .push(format!("Invalid flat roof style geometry {}", wall.0));
            continue;
        }
        // Missing associations are harmless: the roof may have been deleted
        // before this optional customization reaches its consumer.
        let Some(entity) = map.entity(wall) else {
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            continue;
        };
        let mut tick = 0;
        let mut candidate = roof.0;
        let outcome = if let Some(request) = style {
            apply_flat_style_resolved(&mut candidate, request, &mut tick, frame.tick)
        } else if let Some(request) = rim {
            apply_flat_rim_resolved(&mut candidate, request, &mut tick, frame.tick)
        } else {
            unreachable!("each request contains either a full style or a rim")
        };
        if !roof_geometry_is_finite(&candidate) {
            report.faults.push(format!(
                "Roof customization produced invalid geometry {}",
                wall.0
            ));
            continue;
        }
        if tick == frame.tick {
            roof.0 = candidate;
        }
        if let Some(wall) = outcome.recalculate_wall_holes {
            holes.write(RecalculateWallHoles(wall));
            report.wall_holes_requests.push(wall);
        }
        if let Some(sound) = outcome.sound {
            let sound = RoofSound {
                entity,
                wall,
                pivot: sound.pivot,
                kind: sound.observed_kind,
            };
            sounds.write(sound);
            report.sounds.push(sound);
        }
        if let Some(mutation) = outcome.history_mutation {
            history.0.push(mutation);
            report.rim_history_requests.push(mutation);
        }
    }
}
