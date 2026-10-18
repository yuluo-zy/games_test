//! B FinishWall distinguishes new-wall completion (true) from existing-wall
//! continuation completion (false); false is not a generic cancellation flag.
use bevy_ecs::prelude::*;

use crate::{
    resources::{
        color_id::{HalftimberColorIds, WallColorIds},
        in_progress_curve::InProgressCurve,
        onboarding::NotifyHintSystemToolWasJustUsed,
        walls::{InnerWalls, WallId, WallState},
    },
    systems::wall::{
        deferred_wall_history::{PendingWallChanged, PendingWallHistory},
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
    mut continuation: ResMut<ContinueWallOriginalState>,
    snap: Res<SnappingMode>,
    mut records: EventWriter<RecordWallHistory>,
    mut used: EventWriter<NotifyHintSystemToolWasJustUsed>,
    mut issues: ResMut<WallDrawingIssues>,
    mut pending: ResMut<PendingWallHistory>,
    wall_colors: Res<WallColorIds>,
    fence_colors: Res<HalftimberColorIds>,
) {
    // Original consumes every pending event and uses the final bool, rather
    // than replaying completion once for each unread event.
    let Some(event) = events.read().last().copied() else {
        return;
    };
    let Some(wall) = progress.wall else {
        return;
    };
    if !event.0 {
        // ui_continue_wall writes false; its captured source selects this
        // family. There is no rollback/delete when no previous source exists.
        let Some(state) = walls.get(wall) else {
            progress.clear();
            return;
        };
        if continuation.previous.is_none() {
            progress.clear();
            return;
        }
        // Premade-shape enclosure and snapped merges have additional original
        // producers/consumers. Retain both owners until those paths are ready.
        if snap.mode > 1 || state.source().curve_kind_tag < 2 {
            issues.0.push(WallDrawingIssue::UnsupportedFinish {
                wall,
                commit: event.0,
                snapped_mode: snap.mode,
                continued: true,
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
        let previous = continuation.previous.take().unwrap();
        let mut captured = state.snapshot().clone();
        captured.source.extra = metadata.desired_height;
        let valid = is_valid_finished_wall(
            captured.source.curve.points.len(),
            captured.source.curve.length,
        );
        if valid {
            if let Some(changed) = PendingWallChanged::try_new(
                wall,
                previous.snapshot().clone(),
                captured,
                progress.flags & 1 != 0,
            ) {
                // Original replace happens before its collision assertion.
                assert!(
                    pending.wall_changed.replace(changed).is_none(),
                    "pending_wall_history.wall_changed.replace(expand).is_none()"
                );
                progress.clear();
                return;
            }
        }
        // Invalid continuation removes the live wall but records the actual
        // original source and the current color owner for undo restoration.
        // 0x140a80ff9 removes and drops authority before indexing color.
        walls.remove(wall);
        let color = if metadata.kind == 0 {
            wall_colors.0[wall.0 as usize]
        } else {
            fence_colors.0[wall.0 as usize]
        };
        records.write(RecordWallHistory::deleted_wall_with_height(
            wall,
            &previous,
            color,
            metadata.desired_height,
        ));
        progress.clear();
        return;
    }
    if continuation.previous.is_some() || snap.mode > 1 {
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

    fn complete(previous: bool, extend_start: bool, invalid: bool) -> World {
        use crate::{
            resources::{
                history::History, roofs::geometry::surface::Curve2,
                walls::in_progress_wall::WallDrawingMetadata,
            },
            systems::{
                history::recording::HistoryRecordingFailed,
                wall::{
                    deferred_wall_history::history_record_wall_changed,
                    history_recording::record_wall_history,
                },
            },
        };
        let mut world = World::new();
        let before = WallState::new(
            Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0]], false).unwrap(),
            3.9,
            0,
            0,
            false,
        );
        let points = if invalid {
            vec![[0.0, 0.0], [0.1, 0.0]]
        } else if extend_start {
            vec![[-1.0, 0.0], [1.0, 0.0]]
        } else {
            vec![[0.0, 0.0], [2.0, 0.0]]
        };
        let live = WallState::new(Curve2::try_new(points, false).unwrap(), 2.0, 0, 0, false);
        let mut walls = InnerWalls::new();
        let wall = walls.new_wall_from_curve(live);
        world.insert_resource(walls);
        let mut progress = InProgressCurve::default();
        progress.wall = Some(wall);
        progress.current_wall = wall;
        progress.flags = u8::from(extend_start);
        progress.drawing = Some(WallDrawingMetadata {
            first_unsnapped_cursor: None,
            desired_height: 7.0,
            color_id: 99,
            kind: 0,
            freehand_ty: 0,
        });
        world.insert_resource(progress);
        world.insert_resource(ContinueWallOriginalState {
            previous: previous.then_some(before),
        });
        world.init_resource::<SnappingMode>();
        world.init_resource::<WallDrawingIssues>();
        world.init_resource::<PendingWallHistory>();
        let mut colors = WallColorIds::new();
        colors.0[wall.0 as usize] = 17;
        world.insert_resource(colors);
        world.init_resource::<HalftimberColorIds>();
        world.init_resource::<Events<FinishWall>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
        world.init_resource::<Events<HistoryRecordingFailed>>();
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(12).unwrap();
        world.insert_resource(history);
        world
            .resource_mut::<Events<FinishWall>>()
            .send(FinishWall(false));
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                finish_wall,
                history_record_wall_changed,
                record_wall_history::<()>,
            )
                .chain(),
        );
        schedule.run(&mut world);
        schedule.run(&mut world); // drained pending/event cursors must not record twice
        world
    }

    #[test]
    fn false_finishes_existing_wall_and_records_exact_old_subrange_once() {
        use crate::resources::history::{EditType, History, wall_edit::WallHistoryEdit};
        for extend_start in [false, true] {
            let world = complete(true, extend_start, false);
            let wall = WallId(1);
            assert!(
                world
                    .resource::<ContinueWallOriginalState>()
                    .previous
                    .is_none()
            );
            assert!(world.resource::<InProgressCurve>().wall.is_none());
            assert!(
                world
                    .resource::<PendingWallHistory>()
                    .wall_changed
                    .is_none()
            );
            assert_eq!(
                world.resource::<InnerWalls>().get_unwrapped(wall).height(),
                2.0
            );
            assert!(
                world
                    .resource::<Events<NotifyHintSystemToolWasJustUsed>>()
                    .is_empty()
            );
            assert!(world.resource::<WallDrawingIssues>().0.is_empty());
            let edits = &world.resource::<History>().pending().unwrap().edits;
            assert_eq!(edits.len(), 1);
            let EditType::Wall(compound) = &edits[0] else {
                panic!("wall history");
            };
            assert_eq!((compound.groups.len(), compound.groups[0].len()), (1, 1));
            let WallHistoryEdit::StateChange {
                previous,
                next,
                observed_operation,
                ..
            } = &compound.groups[0][0]
            else {
                panic!("expanded wall");
            };
            assert_eq!(previous.source.extra, 3.9);
            assert_eq!(next.source.extra, 7.0);
            let low = f32::from_le_bytes(observed_operation[4..8].try_into().unwrap());
            let high = f32::from_le_bytes(observed_operation[8..12].try_into().unwrap());
            assert_eq!(
                (low, high),
                if extend_start { (0.5, 1.0) } else { (0.0, 0.5) }
            );
        }
    }

    #[test]
    fn invalid_continuation_records_previous_source_and_actual_color() {
        use crate::resources::history::{EditType, History, wall_edit::WallHistoryEdit};
        let world = complete(true, false, true);
        assert!(!world.resource::<InnerWalls>().contains(WallId(1)));
        assert!(
            world
                .resource::<ContinueWallOriginalState>()
                .previous
                .is_none()
        );
        assert!(world.resource::<InProgressCurve>().wall.is_none());
        let edits = &world.resource::<History>().pending().unwrap().edits;
        let EditType::Wall(compound) = &edits[0] else {
            panic!("wall history");
        };
        let WallHistoryEdit::Delete {
            source,
            color_id,
            restored_height,
            ..
        } = &compound.groups[0][0]
        else {
            panic!("previous source delete");
        };
        assert_eq!(source.source.curve.points, [[0.0, 0.0], [1.0, 0.0]]);
        assert_eq!(source.source.extra, 3.9);
        assert_eq!(*restored_height, 7.0);
        assert_eq!(*color_id, 17); // captured draw color was 99
    }

    #[test]
    fn false_without_previous_drops_progress_but_keeps_actual_live_wall() {
        use crate::resources::history::History;
        let world = complete(false, false, false);
        assert!(world.resource::<InnerWalls>().contains(WallId(1)));
        assert!(world.resource::<InProgressCurve>().wall.is_none());
        assert!(
            world
                .resource::<History>()
                .pending()
                .unwrap()
                .edits
                .is_empty()
        );
        assert!(world.resource::<WallDrawingIssues>().0.is_empty());
    }

    #[test]
    fn invalid_color_index_panics_after_original_live_wall_removal() {
        use crate::resources::{
            roofs::geometry::surface::Curve2, walls::in_progress_wall::WallDrawingMetadata,
        };
        let source = WallState::new(
            Curve2::try_new(vec![[0.0, 0.0], [0.1, 0.0]], false).unwrap(),
            3.9,
            0,
            0,
            false,
        );
        let mut world = World::new();
        let mut walls = InnerWalls::new();
        let wall = walls.new_wall_from_curve(source.clone());
        world.insert_resource(walls);
        let mut progress = InProgressCurve::default();
        progress.wall = Some(wall);
        progress.drawing = Some(WallDrawingMetadata {
            first_unsnapped_cursor: None,
            desired_height: 7.0,
            color_id: 99,
            kind: 0,
            freehand_ty: 0,
        });
        world.insert_resource(progress);
        world.insert_resource(ContinueWallOriginalState {
            previous: Some(source),
        });
        world.init_resource::<SnappingMode>();
        world.init_resource::<WallDrawingIssues>();
        world.init_resource::<PendingWallHistory>();
        // Native color indexing panics after remove; an empty owner forces it.
        world.insert_resource(WallColorIds(Vec::new()));
        world.init_resource::<HalftimberColorIds>();
        world.init_resource::<Events<FinishWall>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
        world
            .resource_mut::<Events<FinishWall>>()
            .send(FinishWall(false));
        let mut schedule = Schedule::default();
        schedule.add_systems(finish_wall);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                schedule.run(&mut world);
            }))
            .is_err()
        );
        assert!(!world.resource::<InnerWalls>().contains(wall));
        // Only the confirmed removal is asserted after panic: other unwound
        // owners are deliberately outside this native failure observation.
    }

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
