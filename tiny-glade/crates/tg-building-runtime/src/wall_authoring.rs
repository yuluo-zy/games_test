//! Creates and edits authoritative walls, appearance, and deferred history input.
//! Startup IDs and creation settings are supplied explicitly by the caller.
use crate::{
    BuildingReport, BuildingStage, CreateRoof, PublicWalls, RemoveWall, Roof, UpsertRawWall,
    WallCollectionState, WallRoofMap,
};
use bevy_ecs::prelude::*;
use country_core::systems::wall::shapes::authoring::AuthoredBuilding;
use country_core::systems::wall::shapes::authoring::AuthoringShape;
use country_core::systems::wall::shapes::authoring::CreationSettingsObserved;
use country_core::systems::wall::shapes::authoring::PreviousBuildingObserved;
use country_core::systems::wall::shapes::authoring::RectangleEditOutcome;
use country_core::systems::wall::shapes::authoring::SelectedCreationAppearance;
use country_core::systems::wall::shapes::authoring::create_building_from_placement;
use country_core::systems::wall::shapes::authoring::edit_rectangle_source;
use country_core::resources::walls::publication_source::RawWallPublicationSource;
use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
use country_core::resources::history::wall_edit::WallCompoundHistoryEdit;
use country_core::resources::history::wall_edit::WallHistoryMetadata;
use country_core::resources::history::wall_edit::record_deleted_wall;
use country_core::resources::walls::publication_fields::WallId;
use std::collections::{HashMap, HashSet};

#[derive(Event, Clone)]
pub struct CreateBuildingFromPlacement {
    pub shape: AuthoringShape,
    pub placement: [u8; 80],
}
#[derive(Event, Clone, Copy)]
pub struct RemoveAuthoredWall(pub WallId);
#[derive(Event, Clone)]
pub struct EditAuthoredRectangle(pub [u8; 64]);

#[derive(Resource, Default)]
pub struct WallAuthoringState {
    /// Last allocated ID. Callers must initialize this before creating walls.
    pub id_counter: Option<u64>,
    pub settings: Option<CreationSettingsObserved>,
    pub dev_override_is_demo_mode: Option<bool>,
    pub history_metadata: HashMap<WallId, WallHistoryMetadata>,
    /// Last known colors retained across deletion for history restoration.
    pub retained_appearance: HashMap<WallId, SelectedCreationAppearance>,
    pub pending_deletion_history: Vec<WallCompoundHistoryEdit>,
    pub appearance: HashMap<WallId, SelectedCreationAppearance>,
    /// Source notifications scoped to the current update.
    pub new_sources: HashSet<WallId>,
    pub updated_sources: HashSet<WallId>,
    pub removed_sources: HashSet<WallId>,
    /// Snapshots accepted during active recording, awaiting wall/roof history.
    /// Unrecorded creation does not retain duplicate authority here.
    pub pending_creation_history: Vec<AuthoredBuilding>,
    pub created_this_frame: Vec<WallId>,
    pub rectangle_edits_this_frame: Vec<RectangleEditOutcome>,
    pub(crate) staged_sources: HashMap<WallId, RawWallPublicationSource>,
}

pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallAuthoringState>()
        .add_event::<CreateBuildingFromPlacement>()
        .add_event::<RemoveAuthoredWall>()
        .add_event::<EditAuthoredRectangle>()
        .add_systems(
            bevy_app::Update,
            (
                consume,
                edit_rectangles,
                crate::wall_decorator_archivist::consume_observed,
                crate::wall_rectangle_edit::consume_and_finish,
                crate::wall_pending_changed::consume,
                crate::wall_bulldoze::consume_and_animate,
                cleanup_removed,
            )
                .chain()
                .in_set(BuildingStage::Authoring),
        );
}

fn cleanup_removed(mut removed: EventReader<RemoveWall>, mut state: ResMut<WallAuthoringState>) {
    for event in removed.read() {
        state.appearance.remove(&event.0);
        state.history_metadata.remove(&event.0);
    }
}

fn consume(
    mut creates: EventReader<CreateBuildingFromPlacement>,
    mut removes: EventReader<RemoveAuthoredWall>,
    walls: Res<PublicWalls>,
    history: Res<crate::history::BuildingHistory>,
    source: Res<WallCollectionState>,
    map: Res<WallRoofMap>,
    roofs: Query<&Roof>,
    mut state: ResMut<WallAuthoringState>,
    mut upsert: EventWriter<UpsertRawWall>,
    mut create_roof: EventWriter<CreateRoof>,
    mut remove_wall: EventWriter<RemoveWall>,
    mut report: ResMut<BuildingReport>,
) {
    state.new_sources.clear();
    state.updated_sources.clear();
    state.removed_sources.clear();
    state.created_this_frame.clear();
    state.staged_sources.clear();
    let recording = history
        .transactions
        .as_ref()
        .is_some_and(|transaction| transaction.is_recording());
    let mut created = Vec::new();
    for event in creates.read() {
        if recording && state.dev_override_is_demo_mode.is_none() {
            report
                .faults
                .push("Wall creation history requires demo-mode metadata".into());
            continue;
        }
        let Some(settings) = state.settings else {
            report
                .faults
                .push("Wall creation requires actual creation settings".into());
            continue;
        };
        let Some(mut counter) = state.id_counter else {
            report
                .faults
                .push("Wall creation requires an observed ID counter".into());
            continue;
        };
        if counter.checked_add(1).is_none() {
            report
                .faults
                .push("Wall creation ID space is exhausted".into());
            continue;
        }
        let hint = (event.placement[0x2c] != 2).then(|| {
            WallId(u64::from_le_bytes(
                event.placement[0x20..0x28].try_into().unwrap(),
            ))
        });
        let prior_host = hint.and_then(|id| walls.walls.get(&id).map(|host| (id, host)));
        let previous = if let Some((id, host)) = prior_host {
            // A regular host reads its actual color arrays. Missing metadata
            // must not silently become the currently selected creation colors.
            let colors = if host.source.shape.tag() < 2 {
                let Some(colors) = state.appearance.get(&id) else {
                    report.faults.push(format!(
                        "Previous wall {} has no actual color metadata",
                        id.0
                    ));
                    continue;
                };
                (colors.wall_color_id, colors.timber_color_id)
            } else {
                (0, 0)
            }; // Freeform branch never reads these inactive values.
            Some(PreviousBuildingObserved {
                public_point_count: host.state.source_point_count(),
                public_raw_length: host.source.source_curve_length,
                public_shape_tag: host.source.shape.tag(),
                public_style: host.source.style,
                wall_color_id: colors.0,
                timber_color_id: colors.1,
            })
        } else {
            None
        };
        let prior_roof = hint
            .and_then(|id| map.entity(id))
            .and_then(|entity| roofs.get(entity).ok());
        match create_building_from_placement(
            &mut counter,
            event.shape,
            &event.placement,
            settings,
            previous,
            prior_roof.map(|roof| &roof.0),
        ) {
            Ok(building) => {
                let wall = building.wall.wall_id;
                if !crate::raw_source_is_valid(&building.wall.source)
                    || !crate::roof_runtime::roof_request_is_finite(&building.roof_request)
                {
                    report
                        .faults
                        .push(format!("Invalid authored building geometry {}", wall.0));
                    continue;
                }
                if source.raw_sources.contains_key(&wall)
                    || walls.walls.contains_key(&wall)
                    || state.new_sources.contains(&wall)
                {
                    report.faults.push(format!(
                        "Wall creation counter selects an existing wall {}",
                        wall.0
                    ));
                    continue;
                }
                state.id_counter = Some(counter);
                state
                    .new_sources
                    .insert(building.wall.new_source_notification);
                state
                    .updated_sources
                    .insert(building.wall.updated_source_notification);
                state.created_this_frame.push(wall);
                state.appearance.insert(wall, building.appearance);
                state.retained_appearance.insert(wall, building.appearance);
                if let Some(demo) = state.dev_override_is_demo_mode {
                    state
                        .history_metadata
                        .insert(wall, WallHistoryMetadata::for_new_wall(demo));
                }
                if recording {
                    state.pending_creation_history.push(building.clone());
                }
                created.push(building);
            }
            Err(error) => report.faults.push(format!("Wall create: {error:?}")),
        }
    }
    for event in removes.read() {
        if (!source.raw_sources.contains_key(&event.0) && !state.new_sources.contains(&event.0))
            || state.removed_sources.contains(&event.0)
        {
            report.faults.push(format!(
                "Original source removal requires existing wall {}",
                event.0.0
            ));
            continue;
        }
        let observed_source = source.raw_sources.get(&event.0).or_else(|| {
            created
                .iter()
                .find(|b| b.wall.wall_id == event.0)
                .map(|b| &b.wall.source)
        });
        if history
            .transactions
            .as_ref()
            .is_some_and(|t| t.is_recording())
        {
            if let (Some(raw), Some(metadata), Some(appearance)) = (
                observed_source,
                state.history_metadata.get(&event.0),
                state.appearance.get(&event.0),
            ) {
                let snapshot = WallAuthoritySnapshot {
                    source: raw.clone(),
                    metadata: *metadata,
                };
                let edit = record_deleted_wall(event.0, &snapshot, appearance.wall_color_id);
                state.pending_deletion_history.push(edit);
            } else {
                report.faults.push(format!(
                    "Wall {} deletion history lacks observed source metadata/colors",
                    event.0.0
                ));
                continue;
            }
        }
        state.removed_sources.insert(event.0);
        state.appearance.remove(&event.0);
        remove_wall.write(RemoveWall(event.0));
    }
    // A same-update removal suppresses publication of newly created geometry;
    // the allocated ID and history still describe the accepted user action.
    for building in created {
        if state.removed_sources.contains(&building.wall.wall_id) {
            continue;
        }
        state
            .staged_sources
            .insert(building.wall.wall_id, building.wall.source.clone());
        upsert.write(UpsertRawWall {
            wall: building.wall.wall_id,
            source: building.wall.source,
        });
        create_roof.write(CreateRoof(building.roof_request));
    }
}

fn edit_rectangles(
    mut events: EventReader<EditAuthoredRectangle>,
    collection: Res<WallCollectionState>,
    frame: Res<crate::BuildingFrame>,
    mut state: ResMut<WallAuthoringState>,
    mut upsert: EventWriter<UpsertRawWall>,
    mut report: ResMut<BuildingReport>,
) {
    state.rectangle_edits_this_frame.clear();
    for event in events.read() {
        let wall = WallId(u64::from_le_bytes(event.0[8..16].try_into().unwrap()));
        let mut source = if state.removed_sources.contains(&wall) {
            None
        } else {
            state
                .staged_sources
                .get(&wall)
                .or_else(|| collection.raw_sources.get(&wall))
                .cloned()
        };
        match edit_rectangle_source(source.as_mut(), &event.0, frame.delta_time) {
            Ok(outcome) => {
                if let Some(source) = source {
                    if !crate::raw_source_is_valid(&source) {
                        report.faults.push(format!(
                            "Rectangle edit produced invalid authority {}",
                            wall.0
                        ));
                        continue;
                    }
                    state.updated_sources.insert(wall);
                    state.staged_sources.insert(wall, source.clone());
                    upsert.write(UpsertRawWall { wall, source });
                }
                state.rectangle_edits_this_frame.push(outcome);
            }
            Err(error) => report.faults.push(format!("Rectangle edit: {error:?}")),
        }
    }
}
