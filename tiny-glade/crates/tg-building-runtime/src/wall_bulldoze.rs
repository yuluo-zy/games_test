//! Records wall/roof removal history and advances wall shrink animations.
//! The caller supplies the selected range, anchor, and animation time step.
use crate::{
    BuildingReport, DeleteRoof, RemoveWall, Roof, UpsertRawWall, WallCollectionState, WallRoofMap,
    history::{BuildingHistoryRequests, WallHistoryReplayAudio},
    wall_authoring::WallAuthoringState,
};
use bevy_ecs::prelude::*;
use country_core::resources::history::building_transactions::BuildingEdit;
use country_core::systems::wall::eraser::bulldoze::WallBulldozeCommand;
use country_core::systems::wall::eraser::bulldoze::record_wall_bulldoze;
use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
use country_core::resources::walls::publication_fields::WallId;
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
    /// Authority inputs copied for this update; idle updates copy no sources.
    pub authority_snapshots_this_frame: usize,
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
    state.authority_snapshots_this_frame = 0;
    let requests: Vec<_> = requests.read().copied().collect();
    if requests.is_empty() && state.commands.is_empty() {
        return;
    }
    // Each command only accesses its own wall. Snapshot the union of request
    // and animation targets so unrelated curves never enter the working map.
    let targets: std::collections::HashSet<_> = requests
        .iter()
        .map(|request| request.wall)
        .chain(
            state
                .commands
                .iter()
                .filter(|command| !command.finished)
                .map(|command| command.wall),
        )
        .collect();
    let mut sources = std::collections::HashMap::with_capacity(targets.len());
    for wall in targets {
        if authoring.removed_sources.contains(&wall) {
            continue;
        }
        if let Some((source, metadata)) = authoring
            .staged_sources
            .get(&wall)
            .or_else(|| collection.raw_sources.get(&wall))
            .zip(authoring.history_metadata.get(&wall))
        {
            sources.insert(
                wall,
                WallAuthoritySnapshot {
                    source: source.clone(),
                    metadata: *metadata,
                },
            );
        }
    }
    state.authority_snapshots_this_frame = sources.len();
    for request in &requests {
        if !request.anchor.into_iter().all(f32::is_finite) {
            report
                .faults
                .push(format!("Bulldoze {} has invalid anchor", request.wall.0));
            continue;
        }
        if state
            .commands
            .iter()
            .any(|command| command.wall == request.wall && !command.finished)
        {
            report.faults.push(format!(
                "Bulldoze {} already has an active command",
                request.wall.0
            ));
            continue;
        }
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
    if !dt.is_finite() || dt < 0.0 {
        report
            .faults
            .push("Bulldoze requires a finite nonnegative time step".into());
        return;
    }
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
