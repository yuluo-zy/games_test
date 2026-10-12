//! B parameter consumer using actual Bevy resolution and change tracking.
//! History sources are separate: edit_roof itself is not a recording producer.
use crate::{
    BuildingFrame, BuildingReport, Roof, RoofGeometryChanged, RoofSound, WallCollectionState,
    WallGableDirty, WallRoofMap,
};
use bevy_ecs::prelude::*;
use tiny_glade_reconstruction::{
    building_wall_state::WallId,
    roof_edit_rules::{EditFeedback, EditOutcome, HeightCrossing, RoofEditEvent, apply_edit},
};

#[derive(Event, Clone, Copy, Debug)]
pub struct EditRoof(pub RoofEditEvent);

#[derive(Event, Clone, Copy, Debug)]
pub struct RoofEditFeedback {
    pub wall: WallId,
    pub entity: Entity,
    pub feedback: EditFeedback,
}

/// Original threshold event fields. The visual type-switch consumer is
/// recovered separately; retaining this event does not claim GPU ownership.
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
    /// Failed source resolution for the original source_flag==1 Flat branch.
    pub pending_wall_height_owner: Vec<WallId>,
}

pub fn register(app: &mut bevy_app::App) {
    app.add_event::<EditRoof>()
        .add_event::<RoofEditFeedback>()
        .add_event::<RoofHeightCrossed>()
        .init_resource::<RoofParameterEdits>()
        .init_resource::<RoofTypeTransitions>();
}

/// Recovered CPU side of switch_roof_type. Drawable/VAO/transient mesh commands
/// remain backend-owned. The existing Changed<Roof> consumer replaces CPU tiles.
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
    for event in events.read() {
        let Ok(roof) = roofs.get(event.entity) else {
            report
                .faults
                .push(format!("Roof type invalid entity {:?}", event.entity));
            return;
        };
        let height = f32::from_le_bytes(roof.0[0x30..0x34].try_into().unwrap());
        let pitched = tiny_glade_reconstruction::roof_state::ty_observed(height);
        if pitched != (event.crossing == HeightCrossing::Up) {
            report.faults.push(format!(
                "Roof type event disagrees with current state {}",
                event.wall.0
            ));
            return;
        }
        let ridge = f32::from_le_bytes(roof.0[0x34..0x38].try_into().unwrap());
        if ridge == 1.0 {
            gable.write(WallGableDirty(event.wall));
        }
        let shape = u32::from_le_bytes(roof.0[8..12].try_into().unwrap()) as usize;
        if shape > 1 {
            report
                .faults
                .push("Roof type has unsupported pivot shape".into());
            return;
        }
        let field = |offset| f32::from_le_bytes(roof.0[offset..offset + 4].try_into().unwrap());
        let world_height = tiny_glade_reconstruction::surface::height(&roof.0);
        let pivot = [
            field(12 + shape * 8) + world_height * 0.0,
            field(0x4c) + world_height,
            field(16 + shape * 8) + world_height * 0.0,
        ];
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
        // Original entity lookup/query unwrap fails; preserve the failed
        // consumer boundary as a diagnostic rather than silently skipping it.
        let Some(entity) = associations.entity(wall) else {
            report
                .faults
                .push(format!("Roof edit missing association {}", wall.0));
            return;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            report
                .faults
                .push(format!("Roof edit invalid entity {entity:?}"));
            return;
        };
        let mut observed_tick = 0;
        let outcome = apply_edit(
            &mut roof.bypass_change_detection().0,
            &event.0,
            &mut observed_tick,
            frame.tick,
        );
        if outcome.changed_fields != 0 {
            roof.set_changed();
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
        // Actual original resolved suffix emits this for every valid event,
        // including Keep; it is independent of Changed<Roof>.
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
