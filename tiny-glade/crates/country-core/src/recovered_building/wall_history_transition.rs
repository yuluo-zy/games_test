//! B additional StateChange operation branches; ABI proof is separate from the
//! already accepted new-wall producer and timed operation0 implementation.
use crate::recovered_building::{
    building_wall_history_edit::{
        WallAuthoritySnapshot, WallHistoryEdit, WallReplayAudio, slice_source_curve,
    },
    building_wall_history_replay::{TimedWallReplayOutcome, WallReplayCommand, WallReplayError},
    building_wall_state::WallId,
};

fn add(a: f32, b: f32) -> f32 {
    let v = a + b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
fn progress(elapsed: f32, duration: f32) -> f32 {
    let value = elapsed / duration;
    let value = if value.is_nan() && !elapsed.is_nan() && !duration.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        value
    };
    let value = if value < 0.0 { 0.0 } else { value };
    let value = if value > 1.0 { 1.0 } else { value };
    if value < 0.5 {
        ((4.0 * value) * value) * value
    } else {
        let x = (value + value) - 2.0;
        ((0.5 * x) * x) * x + 1.0
    }
}

/// Operation1 contracts the previous curve over its supplied start/end range;
/// operation2 immediately selects the complete snapshot, even in timed mode.
/// This preserves the stable operation0 module and delegates all instant edits
/// to its existing implementation. No unknown operation is accepted.
pub fn advance_wall_transition(
    command: &mut WallReplayCommand,
    sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
    dt: f32,
    terrain: &crate::recovered_building::building_wall_collection::TerrainHeightGrid512,
) -> Result<TimedWallReplayOutcome, WallReplayError> {
    if command.finished || !command.timed {
        return command.advance(sources, dt, terrain);
    }
    let WallHistoryEdit::StateChange {
        wall,
        previous,
        next,
        observed_operation,
    } = &command.edit
    else {
        return command.advance(sources, dt, terrain);
    };
    let operation = u32::from_le_bytes(observed_operation[..4].try_into().unwrap());
    if operation == 0 {
        return command.advance(sources, dt, terrain);
    }
    if operation > 2 {
        return Err(WallReplayError::UnrestoredOperation(operation));
    }
    let mut out = TimedWallReplayOutcome::default();
    let p = progress(command.elapsed, command.duration);
    if operation == 2 || p >= 1.0 || p.is_nan() {
        if let Some(target) = sources.get_mut(wall) {
            *target = if command.undo { previous } else { next }.clone();
            out.updated_source_notifications.push(*wall);
        }
        command.finished = true;
    } else {
        let p = if command.undo { 1.0 - p } else { p };
        let old = previous.source.extra;
        let new = next.source.extra;
        if old != new || old.is_nan() || new.is_nan() {
            out.height_request = Some(
                crate::recovered_building::building_history_transactions::WallHeightReplayRequest {
                    wall_id: wall.0,
                    absolute_height: add((1.0 - p) * old, new * p),
                    observed_audio_flag: 1,
                },
            );
        }
        let start = f32::from_le_bytes(observed_operation[4..8].try_into().unwrap());
        let end = f32::from_le_bytes(observed_operation[8..12].try_into().unwrap());
        let low = add(0.0 * (1.0 - p), start * p);
        let high = add(1.0 - p, end * p);
        let curve = slice_source_curve(&previous.source.curve, low, high)
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
        let x = 1.0 - p;
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
    command.elapsed = add(command.elapsed, dt);
    Ok(out)
}
