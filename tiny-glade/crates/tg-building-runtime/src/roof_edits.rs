//! Applies roof parameter edits atomically and publishes geometry/type feedback.
//! History is recorded by explicit owner events, so replay cannot record itself.
use crate::{
    BuildingFrame, BuildingReport, Roof, RoofGeometryChanged, RoofSound, WallCollectionState,
    WallGableDirty, WallRoofMap, roof_record::RoofRecord,
};
use bevy_ecs::prelude::*;
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::roofs::geometry::edit_rules::EditFeedback;
use country_core::resources::roofs::geometry::edit_rules::EditOutcome;
use country_core::resources::roofs::geometry::edit_rules::HeightCrossing;
use country_core::resources::roofs::geometry::edit_rules::RoofEditEvent;
use country_core::resources::roofs::geometry::edit_rules::apply_edit;
use std::collections::HashSet;

#[derive(Event, Clone, Copy, Debug)]
pub struct EditRoof(pub RoofEditEvent);

#[derive(Event, Clone, Copy, Debug)]
pub struct RoofEditFeedback {
    pub wall: WallId,
    pub entity: Entity,
    pub feedback: EditFeedback,
}

/// A pitch threshold crossing used to update derived type and wall height.
#[derive(Event, Clone, Copy, Debug)]
pub struct RoofHeightCrossed {
    pub wall: WallId,
    pub entity: Entity,
    pub source_flag: u8,
    pub crossing: HeightCrossing,
}

#[derive(Clone, Debug)]
pub struct RoofEditResult {
    pub wall: WallId,
    pub entity: Entity,
    pub outcome: EditOutcome,
}

#[derive(Resource, Default, Debug)]
pub struct RoofParameterEdits {
    pub applied: Vec<RoofEditResult>,
}

#[derive(Resource, Default, Debug)]
pub struct RoofTypeTransitions {
    pub applied: Vec<RoofHeightCrossed>,
    /// Flat roofs whose source-height adjustment needs a missing wall owner.
    pub pending_wall_height_owner: Vec<WallId>,
}

pub fn register(app: &mut bevy_app::App) {
    app.add_event::<EditRoof>()
        .add_event::<RoofEditFeedback>()
        .add_event::<RoofHeightCrossed>()
        .init_resource::<RoofParameterEdits>()
        .init_resource::<RoofTypeTransitions>();
}

/// Updates CPU type feedback. Geometry changes rebuild tiles in the visual stage.
pub fn switch_type_cpu(
    mut events: EventReader<RoofHeightCrossed>,
    roofs: Query<&Roof>,
    sources: Res<WallCollectionState>,
    mut heights: EventWriter<crate::wall_height::SetWallSourceHeight>,
    mut state: ResMut<RoofTypeTransitions>,
    mut gable: EventWriter<WallGableDirty>,
    mut sounds: EventWriter<RoofSound>,
    mut report: ResMut<BuildingReport>,
) {
    state.applied.clear();
    state.pending_wall_height_owner.clear();
    // Parameters are already at the batch's final state. Only the last
    // crossing for each roof can drive its derived type and source-height owner.
    let events: Vec<_> = events.read().copied().collect();
    let mut seen = HashSet::new();
    let mut latest: Vec<_> = events
        .into_iter()
        .rev()
        .filter(|event| seen.insert(event.entity))
        .collect();
    latest.reverse();
    for event in &latest {
        let Ok(roof) = roofs.get(event.entity) else {
            report
                .faults
                .push(format!("Roof type invalid entity {:?}", event.entity));
            continue;
        };
        let record = RoofRecord(&roof.0);
        if !crate::roof_geometry_is_finite(&roof.0) {
            report
                .faults
                .push(format!("Invalid current roof geometry {}", event.wall.0));
            continue;
        }
        let pitched = country_core::resources::roofs::roof_shape::ty_observed(record.pitch());
        if pitched != (event.crossing == HeightCrossing::Up) {
            report.faults.push(format!(
                "Roof type event disagrees with current state {}",
                event.wall.0
            ));
            continue;
        }
        let Some(pivot) = record.sound_pivot() else {
            report.faults.push(format!(
                "Roof type has invalid sound pivot {}",
                event.wall.0
            ));
            continue;
        };
        if record.ridge() == 1.0 {
            gable.write(WallGableDirty(event.wall));
        }
        let sound = RoofSound {
            entity: event.entity,
            wall: event.wall,
            pivot,
            kind: if pitched { 0 } else { 6 },
        };
        sounds.write(sound);
        report.sounds.push(sound);
        if !pitched && event.source_flag == 1 {
            if let Some(source) = sources.raw_sources.get(&event.wall) {
                if source.extra != 1.6 {
                    heights.write(crate::wall_height::SetWallSourceHeight {
                        wall: event.wall,
                        height: 1.6,
                    });
                }
            } else {
                state.pending_wall_height_owner.push(event.wall);
                report.faults.push(format!(
                    "Roof Flat height requires source owner {}",
                    event.wall.0
                ));
            }
        }
        state.applied.push(*event);
    }
}

pub fn consume(
    mut events: EventReader<EditRoof>,
    associations: Res<WallRoofMap>,
    mut roofs: Query<&mut Roof>,
    frame: Res<BuildingFrame>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableDirty>,
    mut feedback: EventWriter<RoofEditFeedback>,
    mut crossed: EventWriter<RoofHeightCrossed>,
    mut results: ResMut<RoofParameterEdits>,
    mut report: ResMut<BuildingReport>,
) {
    results.applied.clear();
    for event in events.read() {
        let wall = WallId(event.0.wall_id);
        let valid_scalars = [event.0.profile, event.0.height, event.0.ridge, event.0.eave]
            .into_iter()
            .all(|edit| edit.mode == 0 || edit.value.is_finite());
        if !valid_scalars || (event.0.tip.mode != 0 && !event.0.tip.value.is_finite()) {
            report
                .faults
                .push(format!("Invalid roof edit geometry {}", wall.0));
            continue;
        }
        // A rejected event is reported without discarding later edits.
        let Some(entity) = associations.entity(wall) else {
            report
                .faults
                .push(format!("Roof edit missing association {}", wall.0));
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            report
                .faults
                .push(format!("Roof edit invalid entity {entity:?}"));
            continue;
        };
        let mut observed_tick = 0;
        let mut candidate = roof.0;
        if !crate::roof_geometry_is_finite(&candidate) {
            report
                .faults
                .push(format!("Invalid current roof geometry {}", wall.0));
            continue;
        }
        let outcome = apply_edit(&mut candidate, &event.0, &mut observed_tick, frame.tick);
        if !crate::roof_geometry_is_finite(&candidate) {
            report
                .faults
                .push(format!("Roof edit produced invalid geometry {}", wall.0));
            continue;
        }
        if outcome.changed_fields != 0 {
            roof.0 = candidate;
        }
        if outcome.rebuild_gable {
            gable.write(WallGableDirty(wall));
        }
        for value in &outcome.feedback {
            feedback.write(RoofEditFeedback {
                wall,
                entity,
                feedback: *value,
            });
        }
        // Every accepted edit invalidates dependent geometry, even a Keep
        // event whose parameters compare equal and do not mark Roof changed.
        let update = RoofGeometryChanged {
            wall,
            entity,
            kind: 1,
        };
        geometry.write(update);
        report.geometry_changes.push(update);
        if let Some(crossing) = outcome.height_crossing {
            crossed.write(RoofHeightCrossed {
                wall,
                entity,
                source_flag: event.0.source_flag,
                crossing,
            });
        }
        results.applied.push(RoofEditResult {
            wall,
            entity,
            outcome,
        });
    }
}
