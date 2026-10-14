//! B timed new-wall state growth and original internal concurrent-group boundary.
//! Whole native process proof is kept in evidence/building-wall-history.
use crate::recovered_building::{
    building_wall_history_edit::{
        InstantWallReplayOutcome, WallAuthoritySnapshot, WallCompoundHistoryEdit, WallHistoryEdit,
        WallReplayAudio, WallReplaySourceError, WallSnapshotError, apply_instant_wall_edit,
        slice_source_curve,
    },
    building_wall_state::WallId,
};
#[derive(Clone, Debug)]
pub enum WallReplayError {
    Source(WallReplaySourceError),
    Snapshot(WallSnapshotError),
    UnrestoredOperation(u32),
    Enclosure(crate::recovered_building::building_wall_freeform_cache::ShapeHeightError),
    UnrestoredConcurrentGroup,
}
#[derive(Clone, Debug)]
pub struct WallReplayCommand {
    pub edit: WallHistoryEdit,
    pub timed: bool,
    pub duration: f32,
    pub elapsed: f32,
    pub undo: bool,
    pub finished: bool,
}
#[derive(Clone, Debug, Default)]
pub struct TimedWallReplayOutcome {
    pub updated_source_notifications: Vec<WallId>,
    pub height_request:
        Option<crate::recovered_building::building_history_transactions::WallHeightReplayRequest>,
    pub audio: Option<WallReplayAudio>,
    pub instantaneous: Option<InstantWallReplayOutcome>,
}

#[derive(Clone, Debug, Default)]
pub struct WallConcurrentReplayOutcome {
    /// In actual command order; the adapter publishes each recovered effect.
    pub effects: Vec<TimedWallReplayOutcome>,
    pub completed_groups: usize,
    pub finished: bool,
}
fn wall_id(edit: &WallHistoryEdit) -> WallId {
    match edit {
        WallHistoryEdit::Create { wall, .. }
        | WallHistoryEdit::Delete { wall, .. }
        | WallHistoryEdit::StateChange { wall, .. }
        | WallHistoryEdit::EnclosureChange { wall, .. } => *wall,
    }
}

/// Full replay_wall's internal boundary: completed adjacent groups continue in
/// the same invocation only when their WallId sets are disjoint. In particular
/// ev_new_wall's Create and StateChange groups always require separate calls,
/// including instant replay. This never pops or enters an outer ReplayBucket.
pub fn advance_wall_groups(
    groups: &mut std::collections::VecDeque<Vec<WallReplayCommand>>,
    sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
    dt: f32,
    terrain: &crate::recovered_building::building_wall_collection::TerrainHeightGrid512,
) -> Result<WallConcurrentReplayOutcome, WallReplayError> {
    let mut out = WallConcurrentReplayOutcome::default();
    loop {
        let Some(front) = groups.front_mut() else {
            out.finished = true;
            break;
        };
        for command in front.iter_mut() {
            out.effects.push(crate::recovered_building::building_wall_history_transition::advance_wall_transition(command, sources, dt, terrain)?);
        }
        if front.iter().any(|command| !command.finished) {
            break;
        }
        let current: std::collections::HashSet<_> =
            front.iter().map(|c| wall_id(&c.edit)).collect();
        let next: std::collections::HashSet<_> = groups
            .get(1)
            .into_iter()
            .flatten()
            .map(|c| wall_id(&c.edit))
            .collect();
        let disjoint = current.is_disjoint(&next);
        groups.pop_front();
        out.completed_groups += 1;
        if !disjoint {
            break;
        }
    }
    out.finished = groups.is_empty();
    Ok(out)
}

/// Actual expansion for the new-wall/deleted-wall producers recovered here.
/// Concurrent multi-operation groups from other producers need separate proof.
pub fn expand_wall_compound(
    edit: &WallCompoundHistoryEdit,
    undo: bool,
    instant: bool,
) -> Result<std::collections::VecDeque<Vec<WallReplayCommand>>, WallReplayError> {
    if edit.groups.iter().any(|g| g.len() != 1) {
        return Err(WallReplayError::UnrestoredConcurrentGroup);
    }
    let mut groups: Vec<_> = edit
        .groups
        .iter()
        .map(|g| {
            g.iter()
                .map(|e| WallReplayCommand {
                    edit: e.clone(),
                    timed: !instant
                        && matches!(
                            e,
                            WallHistoryEdit::StateChange { .. }
                                | WallHistoryEdit::EnclosureChange { .. }
                        ),
                    duration: 0.4,
                    elapsed: 0.0,
                    undo,
                    finished: false,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    if undo {
        groups.reverse();
    }
    Ok(groups.into())
}

fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        ((4.0 * t) * t) * t
    } else {
        let x = (t + t) + (-2.0);
        ((0.5 * x) * x) * x + 1.0
    }
}
fn divide(a: f32, b: f32) -> f32 {
    let v = a / b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
fn add(a: f32, b: f32) -> f32 {
    let v = a + b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
impl WallReplayCommand {
    /// Original progress is evaluated before elapsed+=dt. Finished commands
    /// skip without advancing. Constructor growth keeps the seed enclosure and
    /// metadata until completion, changing only curve/elevation along the way.
    pub fn advance(
        &mut self,
        sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
        dt: f32,
        terrain: &crate::recovered_building::building_wall_collection::TerrainHeightGrid512,
    ) -> Result<TimedWallReplayOutcome, WallReplayError> {
        let mut out = TimedWallReplayOutcome::default();
        if self.finished {
            return Ok(out);
        }
        if let WallHistoryEdit::EnclosureChange {
            wall,
            previous,
            next,
        } = &self.edit
        {
            let destination = if self.undo { previous } else { next };
            let mut completes = !self.timed;
            let shape = if self.timed {
                let ratio = divide(self.elapsed, self.duration);
                let ratio = if ratio < 0.0 { 0.0 } else { ratio };
                let ratio = if ratio > 1.0 { 1.0 } else { ratio };
                let progress = cubic_in_out(ratio);
                if progress >= 1.0 || progress.is_nan() {
                    completes = true;
                }
                let progress = if self.undo { 1.0 - progress } else { progress };
                crate::recovered_building::building_wall_history_edit::interpolate_rectangle_enclosure(previous, next, progress)
                    .map_err(WallReplayError::Enclosure)?
            } else {
                destination.clone()
            };
            let rectangle =
                crate::recovered_building::building_wall_history_edit::rectangle_from_enclosure(
                    &shape,
                )
                .map_err(WallReplayError::Enclosure)?;
            let curve =
                crate::recovered_building::building_wall_freeform_cache::rectangle_curve(rectangle)
                    .map_err(WallReplayError::Enclosure)?;
            if let Some(target) = sources.get_mut(wall) {
                target.source.shape = shape;
                target.source.curve_kind_tag = 1;
                out.updated_source_notifications.push(*wall);
                target.source.curve = curve;
                out.updated_source_notifications.push(*wall);
            }
            if self.timed {
                let rectangle = crate::recovered_building::building_wall_history_edit::rectangle_from_enclosure(destination)
                    .map_err(WallReplayError::Enclosure)?;
                let [x, z] = rectangle.center;
                out.audio = Some(WallReplayAudio {
                    wall: *wall,
                    anchor: [x, terrain.sample_at_world_xz(x, z), z],
                    kind: 0,
                    flag: 1,
                });
                self.elapsed = add(self.elapsed, dt);
            }
            self.finished = completes;
            return Ok(out);
        }
        if !self.timed {
            self.finished = true;
            out.instantaneous = Some(
                apply_instant_wall_edit(sources, &self.edit, self.undo, terrain)
                    .map_err(WallReplayError::Source)?,
            );
            return Ok(out);
        }
        let WallHistoryEdit::StateChange {
            wall,
            previous,
            next,
            observed_operation,
        } = &self.edit
        else {
            return Err(WallReplayError::UnrestoredOperation(u32::MAX));
        };
        let operation = u32::from_le_bytes(observed_operation[..4].try_into().unwrap());
        if operation != 0 {
            return Err(WallReplayError::UnrestoredOperation(operation));
        }
        let ratio = divide(self.elapsed, self.duration);
        let ratio = if ratio < 0.0 { 0.0 } else { ratio };
        let ratio = if ratio > 1.0 { 1.0 } else { ratio };
        let progress = cubic_in_out(ratio);
        if progress >= 1.0 || progress.is_nan() {
            if let Some(target) = sources.get_mut(wall) {
                *target = if self.undo { previous } else { next }.clone();
                out.updated_source_notifications.push(*wall);
            }
            self.finished = true;
        } else {
            let progress = if self.undo { 1.0 - progress } else { progress };
            let old = previous.source.extra;
            let new = next.source.extra;
            if old != new || old.is_nan() || new.is_nan() {
                out.height_request = Some(crate::recovered_building::building_history_transactions::WallHeightReplayRequest {
                    wall_id: wall.0, absolute_height: add((1.0 - progress) * old, new * progress), observed_audio_flag: 1,
                });
            }
            let start = f32::from_le_bytes(observed_operation[4..8].try_into().unwrap());
            let end = f32::from_le_bytes(observed_operation[8..12].try_into().unwrap());
            let low = add(start * (1.0 - progress), 0.0 * progress);
            let high = add((1.0 - progress) * end, progress);
            let curve = slice_source_curve(&next.source.curve, low, high)
                .map_err(WallReplayError::Snapshot)?;
            if old == new && curve.points.len() >= 2 && curve.length > 0.0 {
                let [x, z] = *curve.points.last().unwrap();
                out.audio = Some(WallReplayAudio {
                    wall: *wall,
                    anchor: [x, terrain.sample_at_world_xz(x, z), z],
                    kind: 2,
                    flag: 1,
                });
            }
            let x = 1.0 - progress;
            let elevation_progress = 1.0 - ((x * x) * x);
            let floor = add(
                previous.source.terrain_floor * (1.0 - elevation_progress),
                next.source.terrain_floor * elevation_progress,
            );
            if let Some(target) = sources.get_mut(wall) {
                target.source.curve = curve;
                out.updated_source_notifications.push(*wall);
                target.source.terrain_floor = floor;
                out.updated_source_notifications.push(*wall);
            }
        }
        self.elapsed = add(self.elapsed, dt);
        Ok(out)
    }
}
