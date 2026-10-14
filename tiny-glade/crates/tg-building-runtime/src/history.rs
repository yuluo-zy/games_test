//! Roof owner history over an explicit observed ledger. Replay preserves the
//! native front-bucket emission/removal policy; cross-channel World scheduling
//! remains the adapter graph, not a recovered engine fork registration graph.
use crate::roof_edits::EditRoof;
use crate::{BuildingFrame, BuildingReport, ReplayRoofEvent, RoofRimHistoryRequests};
use bevy_ecs::prelude::*;
use country_core::recovered_building::building_history_transactions::{
    BuildingEdit, BuildingHistoryTransactions, BuildingReplayBucket, BuildingReplayBucketSelection,
    BuildingReplayOutput, RoofHistoryEdit, RoofVisualHistoryRecord, advance_roof_bucket,
    collect_last_visual_history,
};
use country_core::recovered_building::history_transactions::HistoryCursorStep;
use std::collections::VecDeque;
#[derive(Resource, Default)]
pub struct BuildingHistory {
    pub transactions: Option<BuildingHistoryTransactions>,
    finish_requests: usize,
    pub committed_this_frame: Vec<usize>,
    pub replayed_this_frame: usize,
}
/// Original owner source messages, separate from consumers: replayed EditRoof
/// and CreateRoof/DeleteRoof must never implicitly record fresh history.
#[derive(Event, Clone, Debug)]
pub struct RecordRoofVisualChange(pub RoofVisualHistoryRecord);
#[derive(Event, Clone, Copy, Debug)]
pub struct RecordRoofDirectionChange {
    pub wall: u64,
    pub from: u8,
    pub to: u8,
}
#[derive(Event, Clone, Debug)]
pub struct RecordRoofOwner(pub RoofHistoryEdit);
#[derive(Resource, Default)]
pub struct BuildingHistoryRequests(pub Vec<BuildingEdit>);
#[derive(Resource)]
pub struct RoofHistoryReplay {
    pub buckets: VecDeque<BuildingReplayBucketSelection>,
    /// Explicit CPU adapter setting; original replay speed is an input resource.
    pub speed: f32,
    pub emissions: Vec<BuildingReplayOutput>,
    pub completed_this_frame: Vec<usize>,
}
impl Default for RoofHistoryReplay {
    fn default() -> Self {
        Self {
            buckets: VecDeque::new(),
            speed: 1.0,
            emissions: Vec::new(),
            completed_this_frame: Vec::new(),
        }
    }
}
#[derive(Event, Clone, Debug)]
pub struct WallHistoryReplayAudio(
    pub country_core::recovered_building::building_wall_history_edit::WallReplayAudio,
);
#[derive(Event, Clone, Debug)]
pub struct WallHistoryRestoreColor(
    pub country_core::recovered_building::building_wall_history_edit::WallRestoreColorRequest,
);
#[derive(Event, Clone, Copy, Debug)]
pub enum HistoryAction {
    ResetAfterFutureClear,
    Start,
    TryStart,
    Finish,
    Undo { instant: bool },
    Redo { instant: bool },
}
#[derive(Event, Clone, Copy, Debug)]
pub struct InputReplayRecordingFinished {
    pub entry_index: usize,
    pub observed_command_tag: u8,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<BuildingHistory>()
        .init_resource::<BuildingHistoryRequests>()
        .init_resource::<RoofHistoryReplay>()
        .add_event::<HistoryAction>()
        .add_event::<WallHistoryReplayAudio>()
        .add_event::<WallHistoryRestoreColor>()
        .add_event::<RecordRoofVisualChange>()
        .add_event::<RecordRoofDirectionChange>()
        .add_event::<RecordRoofOwner>()
        .add_event::<InputReplayRecordingFinished>();
}
pub fn prepare(
    mut actions: EventReader<HistoryAction>,
    mut history: ResMut<BuildingHistory>,
    frame: Res<BuildingFrame>,
    pending: Res<RoofRimHistoryRequests>,
    owners: Res<BuildingHistoryRequests>,
    mut buffer: ResMut<RoofHistoryReplay>,
    mut replay: EventWriter<ReplayRoofEvent>,
    mut visual: EventWriter<EditRoof>,
    mut report: ResMut<BuildingReport>,
    collection: Res<crate::WallCollectionState>,
    mut authoring: ResMut<crate::wall_authoring::WallAuthoringState>,
    mut upsert: EventWriter<crate::UpsertRawWall>,
    mut remove: EventWriter<crate::RemoveWall>,
    mut heights: EventWriter<crate::wall_height::AdjustWallHeight>,
    mut audio: EventWriter<WallHistoryReplayAudio>,
    mut colors: EventWriter<WallHistoryRestoreColor>,
) {
    history.committed_this_frame.clear();
    history.replayed_this_frame = 0;
    history.finish_requests = 0;
    buffer.emissions.clear();
    buffer.completed_this_frame.clear();
    for action in actions.read() {
        if matches!(action, HistoryAction::ResetAfterFutureClear) {
            if !pending.0.is_empty() || !owners.0.is_empty() || !buffer.buckets.is_empty() {
                report.faults.push("History reset requires outstanding owner mutations and replay buckets to be resolved".into());
                return;
            }
            history.transactions = Some(BuildingHistoryTransactions::after_observed_future_clear());
            continue;
        }
        let Some(transaction) = history.transactions.as_mut() else {
            report
                .faults
                .push("History requires an explicit observed ledger or reset state".into());
            return;
        };
        match action {
            HistoryAction::Start => {
                if let Err(error) = transaction.start_input_recording(frame.phase) {
                    report.faults.push(format!("Start history: {error:?}"));
                    return;
                }
            }
            HistoryAction::TryStart => {
                transaction.try_start_input_recording(frame.phase);
            }
            HistoryAction::Finish => history.finish_requests += 1,
            HistoryAction::Undo { instant } | HistoryAction::Redo { instant } => {
                // Serial cursor admission is an adapter guard until the complete
                // process_replay_mode/Wall-owner scheduling contract is recovered.
                if !buffer.buckets.is_empty() {
                    report
                        .faults
                        .push("History cursor requested during an active replay bucket".into());
                    return;
                }
                let step = if matches!(action, HistoryAction::Undo { .. }) {
                    HistoryCursorStep::Undo
                } else {
                    HistoryCursorStep::Redo
                };
                match transaction.cursor_step(step, *instant) {
                    Ok(Some(selection)) => {
                        buffer.buckets.extend(selection.into_replay_buckets());
                    }
                    Ok(None) => {}
                    Err(error) => {
                        report.faults.push(format!("History cursor: {error:?}"));
                        return;
                    }
                }
            }
            HistoryAction::ResetAfterFutureClear => unreachable!(),
        }
    }
    let scaled_delta = frame.delta_time * buffer.speed;
    if let Some(bucket) = buffer.buckets.front_mut() {
        match &mut bucket.owner {
            BuildingReplayBucket::Roof(commands) => {
                let outputs = match advance_roof_bucket(commands, scaled_delta) {
                    Ok(outputs) => outputs,
                    Err(error) => {
                        report.faults.push(format!("Roof bucket replay: {error:?}"));
                        return;
                    }
                };
                for output in &outputs {
                    match output {
                        BuildingReplayOutput::Discrete(event) => {
                            replay.write(ReplayRoofEvent(event.clone()));
                            history.replayed_this_frame += 1;
                        }
                        BuildingReplayOutput::Visual(event) => {
                            visual.write(EditRoof(*event));
                            history.replayed_this_frame += 1;
                        }
                        BuildingReplayOutput::NoEvent => {}
                    }
                }
                buffer.emissions = outputs;
            }
            BuildingReplayBucket::Wall(groups) => {
                let Some(terrain) = collection.terrain.as_ref() else {
                    report
                        .faults
                        .push("Wall replay requires actual terrain".into());
                    return;
                };
                let mut sources = std::collections::HashMap::new();
                for (wall, source) in &collection.raw_sources {
                    if let Some(metadata) = authoring.history_metadata.get(wall) {
                        sources.insert(*wall,country_core::recovered_building::building_wall_history_edit::WallAuthoritySnapshot{source:source.clone(),metadata:*metadata});
                    }
                }
                // Missing snapshot metadata must not make a present source look
                // absent to native restore/removal semantics.
                for command in groups.iter().flatten() {
                    let wall=match &command.edit {
                        country_core::recovered_building::building_wall_history_edit::WallHistoryEdit::Create{wall,..}|
                        country_core::recovered_building::building_wall_history_edit::WallHistoryEdit::Delete{wall,..}|
                        country_core::recovered_building::building_wall_history_edit::WallHistoryEdit::StateChange{wall,..}|
                        country_core::recovered_building::building_wall_history_edit::WallHistoryEdit::EnclosureChange{wall,..}=>wall,
                    };
                    if collection.raw_sources.contains_key(wall) && !sources.contains_key(wall) {
                        report.faults.push(format!(
                            "Wall {} replay lacks observed source metadata",
                            wall.0
                        ));
                        return;
                    }
                }
                let outcome=match country_core::recovered_building::building_wall_history_replay::advance_wall_groups(groups,&mut sources,scaled_delta,terrain) {
                    Ok(outcome)=>outcome,
                    Err(error)=>{report.faults.push(format!("Wall bucket replay: {error:?}"));return;}
                };
                let mut dirty = std::collections::HashSet::new();
                for effect in outcome.effects {
                    dirty.extend(effect.updated_source_notifications);
                    if let Some(request) = effect.height_request {
                        let mut payload = [0; 40];
                        request.write_observed_fields(&mut payload);
                        heights.write(crate::wall_height::AdjustWallHeight {
                            request:country_core::recovered_building::building_wall_height_command::AdjustWallHeightRequest::from_observed(&payload),
                        });
                    }
                    if let Some(value) = effect.audio {
                        audio.write(WallHistoryReplayAudio(value));
                    }
                    if let Some(instant) = effect.instantaneous {
                        dirty.extend(instant.new_source_notification);
                        dirty.extend(instant.updated_source_notification);
                        if let Some(wall) = instant.removed_source_notification {
                            remove.write(crate::RemoveWall(wall));
                            authoring.history_metadata.remove(&wall);
                        }
                        if let Some(value) = instant.audio {
                            audio.write(WallHistoryReplayAudio(value));
                        }
                        if let Some(value) = instant.color_request {
                            if let Some(mut appearance) =
                                authoring.retained_appearance.get(&value.wall).copied()
                            {
                                appearance.wall_color_id = value.color_id;
                                authoring.appearance.insert(value.wall, appearance);
                            }
                            colors.write(WallHistoryRestoreColor(value));
                        }
                    }
                    history.replayed_this_frame += 1;
                }
                for wall in dirty {
                    if let Some(snapshot) = sources.remove(&wall) {
                        authoring.history_metadata.insert(wall, snapshot.metadata);
                        upsert.write(crate::UpsertRawWall {
                            wall,
                            source: snapshot.source,
                        });
                    }
                }
            }
        }
        let done = buffer.buckets.front().is_some_and(|b| match &b.owner {
            BuildingReplayBucket::Roof(commands) => commands.is_empty(),
            BuildingReplayBucket::Wall(groups) => groups.is_empty(),
        });
        // Original replay_pop_bucket removes at most one outer bucket per call.
        if done {
            let completed = buffer.buckets.pop_front().unwrap();
            buffer.completed_this_frame.push(completed.entry_index);
        }
    }
}
pub fn record_and_finish(
    mut history: ResMut<BuildingHistory>,
    mut authoring: ResMut<crate::wall_authoring::WallAuthoringState>,
    mut invalid: ResMut<crate::roof_invalid::RoofInvalidNotificationState>,
    mut pending: ResMut<RoofRimHistoryRequests>,
    mut owners: ResMut<BuildingHistoryRequests>,
    mut visual: EventReader<RecordRoofVisualChange>,
    mut direction: EventReader<RecordRoofDirectionChange>,
    mut presence: EventReader<RecordRoofOwner>,
    mut finished: EventWriter<InputReplayRecordingFinished>,
    mut report: ResMut<BuildingReport>,
) {
    if history
        .transactions
        .as_ref()
        .is_some_and(|t| t.is_recording())
    {
        if let Some(demo_mode) = authoring.dev_override_is_demo_mode {
            let mut retained = Vec::new();
            for building in std::mem::take(&mut authoring.pending_creation_history) {
                if !authoring
                    .created_this_frame
                    .contains(&building.wall.wall_id)
                {
                    retained.push(building);
                    continue;
                }
                let wall = building.wall.wall_id;
                let metadata=country_core::recovered_building::building_wall_history_edit::WallHistoryMetadata::for_new_wall(demo_mode);
                let snapshot =
                    country_core::recovered_building::building_wall_history_edit::WallAuthoritySnapshot {
                        source: building.wall.source,
                        metadata,
                    };
                match country_core::recovered_building::building_wall_history_edit::record_new_wall(
                    wall,
                    &snapshot,
                    building.appearance.wall_color_id,
                ) {
                    Ok(edit) => {
                        owners.0.push(BuildingEdit::Wall(std::sync::Arc::new(edit)));
                        let record = building.roof_request.record();
                        owners.0.push(BuildingEdit::Roof(RoofHistoryEdit::create(
                            wall.0,
                            record[16..60].try_into().unwrap(),
                        )));
                    }
                    Err(error) => report.faults.push(format!("New Wall history: {error:?}")),
                }
            }
            authoring.pending_creation_history = retained;
        } else if !authoring.created_this_frame.is_empty() {
            report
                .faults
                .push("New Wall history requires observed DEV_OVERRIDE_IS_DEMO_MODE".into());
        }
        // Original bulldoze owner records Roof deletion before Wall compound.
        // This direct authority removal adapter has no shrink producer yet;
        // preserve owner order while keeping that unresolved operation explicit.
        let mut roof_deletions = std::mem::take(&mut invalid.recording_pending);
        for wall_edit in authoring.pending_deletion_history.drain(..) {
            let wall=match wall_edit.groups.first().and_then(|g|g.first()) {
                Some(country_core::recovered_building::building_wall_history_edit::WallHistoryEdit::Delete{wall,..})=>Some(*wall),
                _=>None,
            };
            if let Some(index) =
                wall.and_then(|wall| roof_deletions.iter().position(|e| e.wall_id() == wall.0))
            {
                let edit = roof_deletions.remove(index);
                invalid.history_pending.retain(|e| e != &edit);
                owners.0.push(BuildingEdit::Roof(edit));
            }
            owners
                .0
                .push(BuildingEdit::Wall(std::sync::Arc::new(wall_edit)));
        }
        for edit in roof_deletions {
            invalid.history_pending.retain(|e| e != &edit);
            owners.0.push(BuildingEdit::Roof(edit));
        }
    }
    let records: Vec<_> = visual.read().map(|event| *event.0.record()).collect();
    if let Some(edit) = collect_last_visual_history(&records) {
        owners.0.push(edit);
    }
    owners.0.extend(direction.read().map(|event| {
        BuildingEdit::Roof(RoofHistoryEdit::direction(event.wall, event.from, event.to))
    }));
    owners.0.extend(
        presence
            .read()
            .map(|event| BuildingEdit::Roof(event.0.clone())),
    );
    let requests = history.finish_requests;
    let Some(transaction) = history.transactions.as_mut() else {
        return;
    };
    if (!pending.0.is_empty() || !owners.0.is_empty()) && !transaction.is_recording() {
        report
            .faults
            .push("History::add requires active input recording".into());
        return;
    }
    // Source-owner order here is explicit adapter scheduling. Within each
    // original reader: visual is last-only, direction is all, Rim remains all.
    owners.0.extend(pending.0.drain(..).map(BuildingEdit::from));
    // Unsupported owners are an adapter boundary. Reject before ledger mutation
    // and preserve their complete source queue for recovery/retry.
    if let Some(tag) = owners.0.iter().find_map(|e| match e {
        BuildingEdit::UnresolvedOwner { observed_tag } => Some(*observed_tag),
        _ => None,
    }) {
        report
            .faults
            .push(format!("Record history: unresolved owner {tag}"));
        return;
    }
    while let Some(edit) = owners.0.first().cloned() {
        if let Err(error) = transaction.add(edit) {
            report.faults.push(format!("Record history: {error:?}"));
            return;
        }
        owners.0.remove(0);
    }
    let mut commits = Vec::new();
    for _ in 0..requests {
        match transaction.finish_input_recording() {
            Ok(Some(commit)) => {
                finished.write(InputReplayRecordingFinished {
                    entry_index: commit.entry_index,
                    observed_command_tag: commit.observed_input_replay_command_tag,
                });
                commits.push(commit.entry_index);
            }
            Ok(None) => {}
            Err(error) => {
                report.faults.push(format!("Finish history: {error:?}"));
                break;
            }
        }
    }
    history.committed_this_frame = commits;
}
