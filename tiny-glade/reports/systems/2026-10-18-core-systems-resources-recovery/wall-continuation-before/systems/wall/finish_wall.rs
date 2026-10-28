//! B FinishWall(true), fresh unsnapped wall/fence completion. The existing
//! compound-history producer retains seed-create and full-growth groups.
use bevy_ecs::prelude::*;

use crate::{
    resources::{
        in_progress_curve::InProgressCurve,
        onboarding::NotifyHintSystemToolWasJustUsed,
        walls::{InnerWalls, WallId, WallState},
    },
    systems::wall::{
        draw_wall::{WallDrawingIssue, WallDrawingIssues},
        history_recording::RecordWallHistory,
        snapping::SnappingMode,
    },
};

/// Original EventInstance is sixteen bytes: EventId and a bool payload.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FinishWall(pub bool);

/// Existing-wall continuation captures the real previous authority. Fresh
/// drawing keeps this None. The producer is restored separately, never by
/// filling in a default WallState or assuming a continuation succeeded.
#[derive(Resource, Clone, Debug, Default)]
pub struct ContinueWallOriginalState {
    pub previous: Option<WallState>,
}

/// Native ordinary-finish gate at 0x140a80a36..a63. The ordered positive check
/// rejects NaN; positive infinity passes this gate, before history's slicing.
pub fn is_valid_finished_wall(point_count: usize, length: f32) -> bool {
    point_count >= 2 && length > 0.0 && !(length < 0.2)
}

pub fn finish_wall(
    mut events: EventReader<FinishWall>,
    mut walls: ResMut<InnerWalls>,
    mut progress: ResMut<InProgressCurve>,
    continuation: Res<ContinueWallOriginalState>,
    snap: Res<SnappingMode>,
    mut records: EventWriter<RecordWallHistory>,
    mut used: EventWriter<NotifyHintSystemToolWasJustUsed>,
    mut issues: ResMut<WallDrawingIssues>,
) {
    // Original consumes every pending event and uses the final bool, rather
    // than replaying completion once for each unread event.
    let Some(event) = events.read().last().copied() else {
        return;
    };
    let Some(wall) = progress.wall else {
        return;
    };
    if !event.0 || continuation.previous.is_some() || snap.mode > 1 {
        issues.0.push(WallDrawingIssue::UnsupportedFinish {
            wall,
            commit: event.0,
            snapped_mode: snap.mode,
            continued: continuation.previous.is_some(),
        });
        return;
    }
    let Some(metadata) = progress.drawing.as_ref() else {
        issues.0.push(WallDrawingIssue::MissingFinishMetadata {
            wall,
            commit: event.0,
        });
        return;
    };
    let state = walls.get_unwrapped(wall);
    let curve = &state.source().curve;
    if is_valid_finished_wall(curve.points.len(), curve.length) {
        // 0x140a80b1c overrides only the captured history state's +54 height.
        // Live authority can still contain a clamped/interpolated height.
        let mut snapshot = state.snapshot().clone();
        snapshot.source.extra = metadata.desired_height;
        let captured = WallState::from_snapshot(snapshot, state.freehand_ty());
        records.write(
            RecordWallHistory::new_wall(wall, &captured, metadata.color_id)
                .expect("original finished wall history slice must be valid"),
        );
        used.write(NotifyHintSystemToolWasJustUsed);
    } else {
        walls.remove(wall);
    }
    // Original drops the active curve and writes its None niche on both paths.
    progress.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_finish_point_length_and_unordered_gates() {
        for (count, length, valid) in [
            (0, 1.0, false),
            (1, 1.0, false),
            (2, -1.0, false),
            (2, 0.0, false),
            (2, -0.0, false),
            (2, 0.199_999_99, false),
            (2, 0.2, true),
            (2, 0.200_000_02, true),
            (2, f32::NAN, false),
            (2, f32::INFINITY, true),
        ] {
            assert_eq!(is_valid_finished_wall(count, length), valid);
        }
    }
}
