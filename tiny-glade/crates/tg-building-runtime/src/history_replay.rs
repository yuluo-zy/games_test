//! Advances wall replay without committing progress from a failed owner bucket.
use country_core::resources::walls::publication_source::TerrainHeightGrid512;
use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
use country_core::systems::history::wall_replay_interpreter::TimedWallReplayOutcome;
use country_core::systems::history::wall_replay_interpreter::WallReplayCommand;
use country_core::systems::history::wall_replay_interpreter::WallReplayError;
use country_core::systems::history::wall_replay_transition::advance_wall_transition;
use country_core::resources::walls::publication_fields::WallId;
use std::collections::{HashMap, HashSet, VecDeque};

/// Sources are the caller's temporary target snapshots and must be discarded on
/// error. Only elapsed/completion state changes inside commands; checkpointing
/// those scalars avoids cloning their immutable curve/history payloads.
pub(crate) fn advance_wall_bucket(
    groups: &mut VecDeque<Vec<WallReplayCommand>>,
    sources: &mut HashMap<WallId, WallAuthoritySnapshot>,
    delta_time: f32,
    terrain: &TerrainHeightGrid512,
) -> Result<Vec<TimedWallReplayOutcome>, WallReplayError> {
    let checkpoint: Vec<_> = groups
        .iter()
        .flatten()
        .map(|command| (command.elapsed, command.finished))
        .collect();
    let mut effects = Vec::new();
    let mut completed = 0;
    while completed < groups.len() {
        let group = &mut groups[completed];
        for command in group.iter_mut() {
            match advance_wall_transition(command, sources, delta_time, terrain) {
                Ok(effect) => effects.push(effect),
                Err(error) => {
                    for (command, (elapsed, finished)) in
                        groups.iter_mut().flatten().zip(checkpoint)
                    {
                        command.elapsed = elapsed;
                        command.finished = finished;
                    }
                    return Err(error);
                }
            }
        }
        if group.iter().any(|command| !command.finished) {
            break;
        }
        // Adjacent completed groups may advance together only when their target
        // sets are disjoint. A shared wall needs the next frame's published state.
        let current: HashSet<_> = groups[completed]
            .iter()
            .filter_map(|command| command.edit.wall_id())
            .collect();
        completed += 1;
        let next: HashSet<_> = groups
            .get(completed)
            .into_iter()
            .flatten()
            .filter_map(|command| command.edit.wall_id())
            .collect();
        if !current.is_disjoint(&next) {
            break;
        }
    }
    // Keep completed commands available for rollback until every visited group
    // succeeds. Source snapshots and emitted effects are committed by the caller.
    drop(groups.drain(..completed));
    Ok(effects)
}
