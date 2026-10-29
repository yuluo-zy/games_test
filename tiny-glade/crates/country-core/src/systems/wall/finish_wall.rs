//! B FinishWall distinguishes new-wall completion (true) from existing-wall
//! continuation completion (false); false is not a generic cancellation flag.
use bevy_ecs::prelude::*;

use crate::{
    resources::{
        color_id::{HalftimberColorIds, WallColorIds},
        history::History,
        in_progress_curve::InProgressCurve,
        onboarding::NotifyHintSystemToolWasJustUsed,
        walls::{InnerWalls, WallId, WallState},
    },
    systems::{wall::{
        deferred_wall_history::{PendingWallChanged, PendingWallHistory},
        draw_wall::{WallDrawingIssue, WallDrawingIssues},
        history_recording::{RecordWallHistory, record_wall_immediately},
        merge_walls::MergeWallsCmd,
        snapping::{SnappingMode, telegraph_walls_are_merged::PlayLineAcrossWall},
    }},
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

/// 原版 snap-merge 闭包 `FUN_140a82060`（0x140a82060，finish_wall 在两条提交成功路径上调用）。
/// 仅 `SnappingMode.mode == 2`（端点吸附）时生效：
/// 1. 目标墙 = `InnerWalls[snap.wall]`（缺失即 `Option::unwrap` panic，原版 `unwrap_failed`）；
/// 2. 目标墙 `source +0x65`（`WallState` 条目 +0x6d）与当前绘制 `kind`（InProgress +0x9c）
///    不一致则直接返回（墙不与栅栏合并）；
/// 3. `wall_a == wall_b`（吸附到自己的端点）：把该墙推入局部 Vec（随后逐个做 enclosure 的
///    CompoundEdit + `_try_update_state`），并写一条
///    `PlayLineAcrossWall(snap.wall, snap.end)`；
/// 4. 否则写 `MergeWallsCmd { wall_a, wall_b, height(+0x94), color_id(+0x98), flags(+0x9e),
///    snap.end(+0x18) }`，再写 `PlayLineAcrossWall(wall_a, flags)` 与
///    `PlayLineAcrossWall(snap.wall, snap.end)`。
/// 返回自合并墙（原版局部 Vec 的内容）。
fn snap_merge(
    wall: WallId,
    snap: &SnappingMode,
    progress: &InProgressCurve,
    walls: &InnerWalls,
    merges: &mut EventWriter<MergeWallsCmd>,
    lines: &mut EventWriter<PlayLineAcrossWall>,
) -> Option<WallId> {
    if snap.mode != 2 {
        return None;
    }
    let target = walls.get_unwrapped(snap.wall);
    let metadata = progress
        .drawing
        .as_ref()
        .expect("original reads in-progress +0x94..+0x9d after a successful commit");
    if metadata.kind != target.source_flag_65() {
        return None;
    }
    let snap_end = snap.end;
    if wall == snap.wall {
        lines.write(PlayLineAcrossWall {
            wall: snap.wall,
            flag: snap_end,
        });
        return Some(wall);
    }
    merges.write(MergeWallsCmd {
        wall_a: wall,
        wall_b: snap.wall,
        wall_a_original_height: metadata.desired_height,
        wall_a_original_color_id: metadata.color_id,
        reverse_a: progress.flags != 0,
        keep_b_direction: snap_end != 0,
    });
    lines.write(PlayLineAcrossWall {
        wall,
        flag: progress.flags,
    });
    lines.write(PlayLineAcrossWall {
        wall: snap.wall,
        flag: snap_end,
    });
    None
}

/// 原 140a816ea / 140a811xx：先记录封闭状态变更，成功后才更新权威墙。
fn record_enclosure<S>(wall: WallId, enclosed: bool, walls: &mut InnerWalls, history: &mut History<S>) {
    use crate::resources::history::wall_edit::{WallCompoundHistoryEdit, WallHistoryEdit};
    use crate::resources::walls::publication_fields::{WallId as HistoryWallId, WallShapeSnapshot};
    use crate::systems::wall::valid_enclosure::{IsEnclosed, ShapeParameters};
    let shape = walls.get_unwrapped(wall).snapshot().source.shape.clone();
    let previous = match shape.tag() {
        2 => IsEnclosed::No,
        3 => IsEnclosed::Yes,
        _ => IsEnclosed::PremadeShape(ShapeParameters::from_observed(*shape.bytes())),
    };
    let next = if enclosed { IsEnclosed::Yes } else { IsEnclosed::No };
    let edit = WallHistoryEdit::EnclosureChange { wall: HistoryWallId(wall.0), previous, next };
    record_wall_immediately(RecordWallHistory(std::sync::Arc::new(WallCompoundHistoryEdit {
        groups: vec![vec![edit]],
    })), history);
    if enclosed {
        let mut bytes = *shape.bytes();
        bytes[..4].copy_from_slice(&3u32.to_le_bytes());
        walls.update_wall_state_enclosure(wall, WallShapeSnapshot::from_observed(bytes));
    } else {
        walls.mark_as_non_enclosed(wall);
    }
}

pub fn finish_wall<S: Send + Sync + 'static>(
    mut events: EventReader<FinishWall>,
    mut walls: ResMut<InnerWalls>,
    mut progress: ResMut<InProgressCurve>,
    mut continuation: ResMut<ContinueWallOriginalState>,
    snap: Res<SnappingMode>,
    mut history: ResMut<History<S>>,
    mut used: EventWriter<NotifyHintSystemToolWasJustUsed>,
    mut issues: ResMut<WallDrawingIssues>,
    mut pending: ResMut<PendingWallHistory>,
    wall_colors: Res<WallColorIds>,
    fence_colors: Res<HalftimberColorIds>,
    mut merges: EventWriter<MergeWallsCmd>,
    mut lines: EventWriter<PlayLineAcrossWall>,
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
        let was_premade = state.source().curve_kind_tag < 2;
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
                // 0x140a80f26: snap-merge 闭包在 pending 写入之后、清 progress 之前。
                let looped = snap_merge(wall, &snap, &progress, &walls, &mut merges, &mut lines);
                if was_premade { record_enclosure(wall, false, &mut walls, &mut history); }
                progress.clear();
                if let Some(looped) = looped { record_enclosure(looped, true, &mut walls, &mut history); }
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
        record_wall_immediately(RecordWallHistory::deleted_wall_with_height(
            wall,
            &previous,
            color,
            metadata.desired_height,
        ), &mut history);
        progress.clear();
        return;
    }
    // 原 true 分支直接查询当前墙，不读取或取走续画快照；未找到时清进行中曲线。
    let Some(state) = walls.get(wall) else {
        progress.clear();
        return;
    };
    let Some(metadata) = progress.drawing.as_ref() else {
        issues.0.push(WallDrawingIssue::MissingFinishMetadata {
            wall,
            commit: event.0,
        });
        return;
    };
    let curve = &state.source().curve;
    if is_valid_finished_wall(curve.points.len(), curve.length) {
        // 0x140a80b1c overrides only the captured history state's +54 height.
        // Live authority can still contain a clamped/interpolated height.
        let mut snapshot = state.snapshot().clone();
        snapshot.source.extra = metadata.desired_height;
        let captured = WallState::from_snapshot(snapshot, state.freehand_ty());
        record_wall_immediately(
            RecordWallHistory::new_wall(wall, &captured, metadata.color_id)
                .expect("original finished wall history slice must be valid"),
            &mut history,
        );
        // 0x140a81000：ev_new_wall 之后、NotifyHint 之前。
        let looped = snap_merge(wall, &snap, &progress, &walls, &mut merges, &mut lines);
        used.write(NotifyHintSystemToolWasJustUsed);
        progress.clear();
        if let Some(looped) = looped { record_enclosure(looped, true, &mut walls, &mut history); }
        return;
    } else {
        walls.remove(wall);
    }
    // Original drops the active curve and writes its None niche on both paths.
    progress.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::history::recording::HistoryRecordingFailed;

    #[test]
    fn enclosure_history_records_premade_and_yes_before_authority_updates() {
        use crate::resources::{history::{EditType, wall_edit::WallHistoryEdit},
            roofs::geometry::surface::Curve2, walls::publication_fields::WallShapeSnapshot};
        use crate::systems::wall::valid_enclosure::IsEnclosed;
        let mut walls = InnerWalls::new();
        let wall = walls.new_wall_from_curve(WallState::new(
            Curve2::try_new(vec![[0., 0.], [1., 0.]], false).unwrap(), 3., 0, 0, false));
        let mut shape = [0u8; 28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        shape[4..8].copy_from_slice(&1.0f32.to_le_bytes());
        walls.update_wall_state_enclosure(wall, WallShapeSnapshot::from_observed(shape));
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(12).unwrap();
        record_enclosure(wall, false, &mut walls, &mut history);
        record_enclosure(wall, true, &mut walls, &mut history);
        assert_eq!(walls.get_unwrapped(wall).source().curve_kind_tag, 3);
        let edits = &history.pending().unwrap().edits;
        let EditType::Wall(first) = &edits[0] else { panic!("预制墙历史缺失") };
        assert!(matches!(&first.groups[0][0], WallHistoryEdit::EnclosureChange {
            previous: IsEnclosed::PremadeShape(_), next: IsEnclosed::No, .. }));
        let EditType::Wall(second) = &edits[1] else { panic!("闭环历史缺失") };
        assert!(matches!(&second.groups[0][0], WallHistoryEdit::EnclosureChange {
            previous: IsEnclosed::No, next: IsEnclosed::Yes, .. }));
        let mut no_recording = History::<()>::empty_glade();
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            record_enclosure(wall, false, &mut walls, &mut no_recording)));
        assert!(failed.is_err());
        assert_eq!(walls.get_unwrapped(wall).source().curve_kind_tag, 3);
    }

    #[test]
    fn finishing_premade_continuation_records_enclosure_after_pending_curve() {
        use crate::resources::{history::{EditType, wall_edit::WallHistoryEdit}, roofs::geometry::surface::Curve2,
            walls::{in_progress_wall::WallDrawingMetadata, publication_fields::WallShapeSnapshot}};
        use crate::systems::wall::valid_enclosure::IsEnclosed;
        let before = WallState::new(Curve2::try_new(vec![[0., 0.], [1., 0.]], false).unwrap(), 3., 0, 0, false);
        let live = WallState::new(Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap(), 3., 0, 0, false);
        let mut walls = InnerWalls::new();
        let wall = walls.new_wall_from_curve(live);
        let mut shape = [0; 28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        walls.update_wall_state_enclosure(wall, WallShapeSnapshot::from_observed(shape));
        let mut progress = InProgressCurve::default();
        progress.wall = Some(wall); progress.current_wall = wall;
        progress.drawing = Some(WallDrawingMetadata { first_unsnapped_cursor: None,
            desired_height: 3., color_id: 0, kind: 0, freehand_ty: 0 });
        let mut world = World::new();
        world.insert_resource(walls); world.insert_resource(progress);
        world.insert_resource(ContinueWallOriginalState { previous: Some(before) });
        world.init_resource::<SnappingMode>(); world.init_resource::<WallDrawingIssues>();
        world.init_resource::<PendingWallHistory>(); world.init_resource::<WallColorIds>();
        world.init_resource::<HalftimberColorIds>(); world.init_resource::<Events<FinishWall>>();
        world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
        world.init_resource::<Events<MergeWallsCmd>>(); world.init_resource::<Events<PlayLineAcrossWall>>();
        let mut history = History::<()>::empty_glade(); history.start_input_recording(12).unwrap();
        world.insert_resource(history); world.resource_mut::<Events<FinishWall>>().send(FinishWall(false));
        let mut schedule = Schedule::default(); schedule.add_systems(finish_wall::<()>); schedule.run(&mut world);
        assert!(world.resource::<PendingWallHistory>().wall_changed.is_some());
        assert!(world.resource::<InProgressCurve>().wall.is_none());
        assert_eq!(world.resource::<InnerWalls>().get_unwrapped(wall).source().curve_kind_tag, 2);
        let edits = &world.resource::<History<()>>().pending().unwrap().edits;
        let EditType::Wall(compound) = &edits[0] else { panic!("封闭历史缺失") };
        assert!(matches!(&compound.groups[0][0], WallHistoryEdit::EnclosureChange {
            previous: IsEnclosed::PremadeShape(_), next: IsEnclosed::No, .. }));
    }

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
        world.init_resource::<Events<crate::systems::wall::merge_walls::MergeWallsCmd>>();
        world.init_resource::<Events<crate::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<HistoryRecordingFailed>>();
        let mut immediate_history = History::<()>::empty_glade();
        immediate_history.start_input_recording(12).unwrap();
        world.insert_resource(immediate_history);
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
                finish_wall::<()>,
                history_record_wall_changed::<()>,
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
            let edits = &world.resource::<History<()>>().pending().unwrap().edits;
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
        let edits = &world.resource::<History<()>>().pending().unwrap().edits;
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
                .resource::<History<()>>()
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
        world.init_resource::<Events<crate::systems::wall::merge_walls::MergeWallsCmd>>();
        world.init_resource::<Events<crate::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<HistoryRecordingFailed>>();
        let mut immediate_history = History::<()>::empty_glade();
        immediate_history.start_input_recording(12).unwrap();
        world.insert_resource(immediate_history);
        world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
        world
            .resource_mut::<Events<FinishWall>>()
            .send(FinishWall(false));
        let mut schedule = Schedule::default();
        schedule.add_systems(finish_wall::<()>);
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

    /// 新墙提交后 snap-merge 闭包（0x140a82060）：自吸附 → 只写一条 PlayLineAcrossWall 并登记
    /// 自吸附同步记录并设置闭环；类型不匹配和非端点模式不触发。
    #[test]
    fn snap_merge_self_loop_kind_mismatch_and_non_end_modes() {
        use crate::resources::{
            roofs::geometry::surface::Curve2, walls::in_progress_wall::WallDrawingMetadata,
        };
        // (snap.mode, snap onto self, target flag 65, drawing.kind)
        for (mode, onto_self, target_flag, kind) in [
            (2u8, true, 0u8, 0u8),
            (2, false, 1, 0),
            (3, false, 0, 0),
            (0, false, 0, 0),
        ] {
            let state = |flag: u8, x: f32| {
                WallState::new(
                    Curve2::try_new(vec![[x, 0.0], [x + 2.0, 0.0]], false).unwrap(),
                    3.9,
                    0,
                    flag,
                    false,
                )
            };
            let mut world = World::new();
            let mut walls = InnerWalls::new();
            let wall = walls.new_wall_from_curve(state(0, 0.0));
            let other = walls.new_wall_from_curve(state(target_flag, 10.0));
            world.insert_resource(walls);
            let mut progress = InProgressCurve::default();
            progress.wall = Some(wall);
            progress.current_wall = wall;
            progress.drawing = Some(WallDrawingMetadata {
                first_unsnapped_cursor: None,
                desired_height: 3.9,
                color_id: 5,
                kind,
                freehand_ty: 0,
            });
            world.insert_resource(progress);
            // 原 true 分支不读取或取走残留续画快照。
            world.insert_resource(ContinueWallOriginalState {
                previous: (mode == 0).then(|| state(0, 20.0)),
            });
            let mut snap = SnappingMode::new();
            snap.mode = mode;
            snap.wall = if onto_self { wall } else { other };
            snap.end = 0;
            world.insert_resource(snap);
            world.init_resource::<WallDrawingIssues>();
            world.init_resource::<PendingWallHistory>();
            world.init_resource::<WallColorIds>();
            world.init_resource::<HalftimberColorIds>();
            world.init_resource::<Events<FinishWall>>();
            world.init_resource::<Events<RecordWallHistory>>();
        world.init_resource::<Events<HistoryRecordingFailed>>();
        let mut immediate_history = History::<()>::empty_glade();
        immediate_history.start_input_recording(12).unwrap();
        world.insert_resource(immediate_history);
            world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
            world.init_resource::<Events<MergeWallsCmd>>();
            world.init_resource::<Events<PlayLineAcrossWall>>();
            world
                .resource_mut::<Events<FinishWall>>()
                .send(FinishWall(true));
            let mut schedule = Schedule::default();
            schedule.add_systems(finish_wall::<()>);
            schedule.run(&mut world);
            let merges = world.resource::<Events<MergeWallsCmd>>().len();
            let lines: Vec<_> = world
                .resource::<Events<PlayLineAcrossWall>>()
                .get_cursor()
                .read(world.resource())
                .map(|l| (l.wall, l.flag))
                .collect();
            assert_eq!(merges, 0, "mode {mode} self {onto_self} flag {target_flag}");
            if mode == 2 && onto_self {
                assert_eq!(lines, [(wall, 0)]);
                assert_eq!(world.resource::<InnerWalls>().get_unwrapped(wall).source().curve_kind_tag, 3);
                assert!(world.resource::<WallDrawingIssues>().0.is_empty());
            } else {
                assert!(lines.is_empty());
                assert!(world.resource::<WallDrawingIssues>().0.is_empty());
            }
            // 普通提交照常发生。
            assert_eq!(world.resource::<History<()>>().pending().unwrap().edits.len(), if mode == 2 && onto_self { 2 } else { 1 });
            assert!(world.resource::<InProgressCurve>().wall.is_none());
            assert_eq!(world.resource::<ContinueWallOriginalState>().previous.is_some(), mode == 0);
        }
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
