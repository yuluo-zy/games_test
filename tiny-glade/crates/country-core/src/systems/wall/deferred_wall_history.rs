//! B deferred freehand wall expansion and wall-merge history, using the existing
//! typed compound history and its recording consumer.
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::Vec2;
use utils::curve::Curve2 as InputCurve;

use crate::{
    resources::{
        history::{History, wall_edit::{
            CompoundEditBuilder, WallAuthoritySnapshot, WallCompoundHistoryEdit, WallHistoryEdit,
            record_deleted_wall,
        }},
        roofs::geometry::surface::Curve2,
        walls::{InnerWalls, WallId, WallState},
    },
    systems::wall::history_recording::{RecordWallHistory, record_wall_immediately},
};

#[derive(Clone, Debug)]
pub struct PendingWallChanged {
    pub wall: WallId,
    pub previous: WallAuthoritySnapshot,
    pub next: WallAuthoritySnapshot,
    pub observed_operation: [u8; 12],
}

/// This is the same pending owner written by FinishWall and drained by the
/// deferred history system. No parallel source or default previous wall exists.
#[derive(Resource, Clone, Debug, Default)]
pub struct PendingWallHistory {
    pub wall_changed: Option<PendingWallChanged>,
    /// Original `pending_wall_history.merge_walls` (`Option<PendingMergeWalls>`,
    /// written once by `merge_walls`, drained by `history_record_merge_walls`).
    pub merge_walls: Option<PendingMergeWalls>,
}

/// Original `AdjustHeight` payload inside `PendingMergeWalls` (pending +0x08
/// wall id, +0x10 `from` f32, +0x14 `to` f32; fed to `new_height_changed`
/// 0x140a83fb0 as wall / param_3 / param_4).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PendingAdjustHeight {
    pub wall: WallId,
    pub from: f32,
    pub to: f32,
}

/// Original `deferred_wall_history::PendingMergeWalls`; the field names are the
/// ones printed by the `merge_walls` assertion text (0x142abb1da). Memory
/// layout: adjust_height +0x00/+0x08/+0x10/+0x14, wall_state_a +0x18,
/// wall_state_b +0x88, wall_a +0xf8, wall_b +0x100, merged_wall_id +0x108,
/// wall_color_a +0x110, wall_color_b +0x114, merged_color_id +0x118.
#[derive(Clone, Debug)]
pub struct PendingMergeWalls {
    pub adjust_height: Option<PendingAdjustHeight>,
    pub wall_a: WallId,
    pub wall_b: WallId,
    pub wall_state_a: WallState,
    pub wall_state_b: WallState,
    pub merged_wall_id: WallId,
    pub merged_color_id: u32,
    pub wall_color_a: u32,
    pub wall_color_b: u32,
}

/// Adapt only representation: retain original authority points_u/length rather
/// than replacing its stored parameterization with newly computed values.
fn input_curve(curve: &Curve2) -> InputCurve {
    let mut input = InputCurve::crashy_new_from_points(
        curve.points.iter().copied().map(Vec2::from_array).collect(),
        false,
    );
    input.points_u.clone_from(&curve.points_u);
    input.length = curve.length;
    input
}

impl PendingWallChanged {
    /// Original 0x1408dc9f0. A direction byte chooses the previous first/last
    /// endpoint, projected onto the next curve; only a strict old subrange exists.
    pub fn try_new(
        wall: WallId,
        previous: WallAuthoritySnapshot,
        next: WallAuthoritySnapshot,
        extend_start: bool,
    ) -> Option<Self> {
        assert!(
            !previous.source.curve.points.is_empty(),
            "!before.curve.is_empty()"
        );
        assert!(
            !next.source.curve.points.is_empty(),
            "!after.curve.is_empty()"
        );
        let endpoint = if extend_start {
            previous.source.curve.points[0]
        } else {
            *previous.source.curve.points.last().unwrap()
        };
        // Reuse the recovered original strict-distance / first-tie projection.
        // The existing geometry API asserts its final parameter is finite.
        let coordinate =
            input_curve(&next.source.curve).get_approx_u_from_pos(Vec2::from_array(endpoint));
        let coordinate = coordinate.clamp(0.0, 1.0);
        let (low, high) = if extend_start {
            (coordinate, 1.0)
        } else {
            (0.0, coordinate)
        };
        if !(low < high) {
            return None;
        }
        let mut observed_operation = [0; 12];
        observed_operation[4..8].copy_from_slice(&low.to_le_bytes());
        observed_operation[8..12].copy_from_slice(&high.to_le_bytes());
        Some(Self {
            wall,
            previous,
            next,
            observed_operation,
        })
    }

    pub fn into_compound(self) -> WallCompoundHistoryEdit {
        WallCompoundHistoryEdit {
            groups: vec![vec![WallHistoryEdit::StateChange {
                wall: crate::resources::walls::publication_fields::WallId(self.wall.0),
                previous: self.previous,
                next: self.next,
                observed_operation: self.observed_operation,
            }]],
        }
    }
}

/// 原 1408dccf0 取出 pending 后直接写入同一个 History。
pub fn history_record_wall_changed<S: Send + Sync + 'static>(
    mut pending: ResMut<PendingWallHistory>,
    mut history: ResMut<History<S>>,
) {
    if let Some(changed) = pending.wall_changed.take() {
        record_wall_immediately(RecordWallHistory(Arc::new(changed.into_compound())), &mut history);
    }
}

fn recovered(wall: WallId) -> crate::resources::walls::publication_fields::WallId {
    crate::resources::walls::publication_fields::WallId(wall.0)
}

/// Original 0x1408dd1b0. Takes `merge_walls` once (the take marks the resource
/// changed even when empty). When an adjust-height record exists it is added as
/// its own wall edit (`EditType::new_height_changed`, one group) first; then one
/// compound of three single-edit groups is added: `DeleteWall(wall_a)` with
/// `wall_color_a`, `DeleteWall(wall_b)` with `wall_color_b`, and `CreateWall`
/// for the merged wall cloned from the *current* `InnerWalls` row (color =
/// `merged_color_id`, restored height/elevation = that row's +0x54/+0x58).
/// 两次写入均在本系统内同步完成。
///
/// The merged-wall lookup panics (`Option::unwrap`) exactly where the original
/// does: after the adjust-height record, before the compound is built.
pub fn history_record_merge_walls<S: Send + Sync + 'static>(
    mut pending: ResMut<PendingWallHistory>,
    inner_walls: Res<InnerWalls>,
    mut history: ResMut<History<S>>,
) {
    let Some(merge) = pending.merge_walls.take() else {
        return;
    };
    if let Some(adjust) = merge.adjust_height {
        record_wall_immediately(RecordWallHistory(Arc::new(WallCompoundHistoryEdit {
            groups: vec![vec![WallHistoryEdit::AdjustHeight {
                wall: recovered(adjust.wall),
                from: adjust.from,
                to: adjust.to,
            }]],
        })), &mut history);
    }
    let merged = inner_walls.get(merge.merged_wall_id).unwrap();
    let mut builder = CompoundEditBuilder::default();
    builder.new_edit_sequence();
    for (wall, state, color) in [
        (merge.wall_a, &merge.wall_state_a, merge.wall_color_a),
        (merge.wall_b, &merge.wall_state_b, merge.wall_color_b),
    ] {
        // 0x140a876a0 wall_deleted: Delete { source = state snapshot, color,
        // restored_height = state +0x54 }, appended as a new group.
        let mut deleted = record_deleted_wall(recovered(wall), state.snapshot(), color);
        builder.next(deleted.groups.remove(0).remove(0));
    }
    builder.next(WallHistoryEdit::Create {
        wall: recovered(merge.merged_wall_id),
        source: merged.snapshot().clone(),
        color_id: merge.merged_color_id,
        restored_height: merged.height(),
        restored_elevation: merged.elevation(),
    });
    record_wall_immediately(RecordWallHistory(Arc::new(builder.build())), &mut history);
}
