//! Actual bulldoze snapshot producer and animation connected to authority owners.
//! Original UI hit/range selection and GameTime+58 remain explicit source inputs.
use crate::{
    BuildingReport, DeleteRoof, RemoveWall, Roof, UpsertRawWall, WallCollectionState, WallRoofMap,
    history::{BuildingHistoryRequests, WallHistoryReplayAudio},
    wall_authoring::WallAuthoringState,
};
use bevy_ecs::prelude::*;
use tiny_glade_reconstruction::{
    building_history_transactions::BuildingEdit,
    building_wall_bulldoze::{WallBulldozeCommand, record_wall_bulldoze},
    building_wall_history_edit::WallAuthoritySnapshot,
    building_wall_state::WallId,
};
#[derive(Event, Clone, Copy, Debug)]
pub struct BulldozeWall {
    pub wall: WallId,
    pub low: f32,
    pub high: f32,
    pub anchor: [f32; 3],
}
#[derive(Resource, Default)]
pub struct WallBulldozeState {
    pub commands: Vec<WallBulldozeCommand>,
    pub roof_owners_this_frame: std::collections::HashSet<WallId>,
    pub updated_this_frame: Vec<WallId>,
    pub removed_this_frame: Vec<WallId>,
}
#[derive(Resource, Default)]
pub struct WallBulldozeTiming {
    pub observed_game_time_58: Option<f32>,
}
pub fn register(app: &mut bevy_app::App) {
    app.add_event::<BulldozeWall>()
        .init_resource::<WallBulldozeState>()
        .init_resource::<WallBulldozeTiming>();
}
pub(crate) fn consume_and_animate(
    mut requests: EventReader<BulldozeWall>,
    collection: Res<WallCollectionState>,
    map: Res<WallRoofMap>,
    roofs: Query<&Roof>,
    mut authoring: ResMut<WallAuthoringState>,
    mut state: ResMut<WallBulldozeState>,
    timing: Res<WallBulldozeTiming>,
    mut owners: ResMut<BuildingHistoryRequests>,
    mut roof_delete: EventWriter<DeleteRoof>,
    mut upsert: EventWriter<UpsertRawWall>,
    mut remove: EventWriter<RemoveWall>,
    mut audio: EventWriter<WallHistoryReplayAudio>,
    mut report: ResMut<BuildingReport>,
) {
    state.roof_owners_this_frame.clear();
    state.updated_this_frame.clear();
    state.removed_this_frame.clear();
    let mut sources = std::collections::HashMap::new();
    for (wall, source) in collection
        .raw_sources
        .iter()
        .chain(authoring.staged_sources.iter())
    {
        if authoring.removed_sources.contains(wall) {
            continue;
        }
        if let Some(metadata) = authoring.history_metadata.get(wall) {
            sources.insert(
                *wall,
                WallAuthoritySnapshot {
                    source: source.clone(),
                    metadata: *metadata,
                },
            );
        }
    }
    for request in requests.read() {
        let Some(snapshot) = sources.get(&request.wall) else {
            report.faults.push(format!(
                "Bulldoze {} requires actual authority/metadata",
                request.wall.0
            ));
            continue;
        };
        let Some(colors) = authoring.appearance.get(&request.wall) else {
            report.faults.push(format!(
                "Bulldoze {} requires actual wall color",
                request.wall.0
            ));
            continue;
        };
        let roof = map.entity(request.wall).and_then(|e| roofs.get(e).ok());
        match record_wall_bulldoze(
            request.wall,
            snapshot,
            request.low,
            request.high,
            colors.wall_color_id,
            roof.map(|r| &r.0),
        ) {
            Ok(history) => {
                if let Some(edit) = history.roof {
                    state.roof_owners_this_frame.insert(request.wall);
                    owners.0.push(BuildingEdit::Roof(edit));
                    roof_delete.write(DeleteRoof(request.wall));
                }
                owners
                    .0
                    .push(BuildingEdit::Wall(std::sync::Arc::new(history.wall)));
                state.commands.push(WallBulldozeCommand {
                    original: snapshot.clone(),
                    wall: request.wall,
                    low: request.low,
                    high: request.high,
                    anchor: request.anchor,
                    phase: 0.,
                    finished: false,
                });
            }
            Err(error) => report
                .faults
                .push(format!("Bulldoze {} history: {error:?}", request.wall.0)),
        }
    }
    if state.commands.is_empty() {
        return;
    }
    for command in &state.commands {
        let present = collection.raw_sources.contains_key(&command.wall)
            || authoring.staged_sources.contains_key(&command.wall);
        if present
            && !authoring.removed_sources.contains(&command.wall)
            && !sources.contains_key(&command.wall)
        {
            report.faults.push(format!(
                "Bulldoze {} lacks current source metadata",
                command.wall.0
            ));
            return;
        }
    }
    let Some(dt) = timing.observed_game_time_58 else {
        report
            .faults
            .push("Bulldoze requires observed GameTime+58 step".into());
        return;
    };
    let mut updated = Vec::new();
    let mut removed = Vec::new();
    for command in &mut state.commands {
        match command.advance(&mut sources, dt) {
            Ok(effect) => {
                updated.extend(effect.updated);
                removed.extend(effect.removed);
                if let Some(value) = effect.audio {
                    audio.write(WallHistoryReplayAudio(value));
                }
            }
            Err(error) => report
                .faults
                .push(format!("Bulldoze {} animation: {error:?}", command.wall.0)),
        }
    }
    state.commands.retain(|c| !c.finished);
    for wall in &updated {
        if let Some(snapshot) = sources.get(wall) {
            authoring.history_metadata.insert(*wall, snapshot.metadata);
            authoring
                .staged_sources
                .insert(*wall, snapshot.source.clone());
            upsert.write(UpsertRawWall {
                wall: *wall,
                source: snapshot.source.clone(),
            });
        }
    }
    for wall in &removed {
        authoring.staged_sources.remove(wall);
        remove.write(RemoveWall(*wall));
    }
    state.updated_this_frame = updated;
    state.removed_this_frame = removed;
}
