//! `handle_external_decorator_cmds.rs` — 外部装饰命令处理（0x14081a380，O）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x14081a380-*.txt`）：
//! 11 参 ECS 面；两条外部命令事件流（游标推进）：
//! - 流 A（0x28B 记录，墙命令）：逐条构造 op（kind 强制 **4=remove**）→
//!   `apply_decorator_op`（0x141276e30）→ `History::add`（tag 10 =
//!   DecoratorV5，载荷 Arc{1,1,**5**}，file handle_external_decorator_cmds.rs
//!   0x142a90338）；
//! - 流 B（0x30B 记录，屋顶命令）：op kind = 记录自带 → `apply_decorator_op`
//!   → `History::add`（tag 10，载荷 Arc{1,1,**3**}）；
//! 命令载荷：`{wall u64@0(+8), info 12B@+8..0x14, dst 16B@+4..+0x14,
//! …}`（0x28B）/`{kind@0, …@0x28, dst@+0x10.., roof tag@+0x22, …}`（0x30B）。
//! 事件中还含 `{u64} * 2` 流（0x10B 元素：cancel 类，U）。
//!
//! 事件类型（I）：系统侧模型 [`ExternalDecoratorCmd`]；原生事件族归属
//! core/UI（U 登记）。History 链经 core `EditType::DecoratorV5`
//! （`record_to_history_edit` 模型适配，见 deferred_decorator_history.rs）。

use bevy_ecs::prelude::*;
use std::sync::Arc;

use country_core::resources::history::{EditType, History};
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::decorator::{
    DecoratorId, DecoratorInfo,
};

use crate::apply_decorator_op::{apply_decorator_op, DecoratorOp};
use crate::deferred_decorator_history::{
    record_to_history_edit, DeferredDecoratorEditHistory,
};

/// Compatibility name for the authoritative core queue; no second resource.
pub use country_core::resources::walls::decorator::DeferredDecoratorOpEvents as DecoratorOpEvents;

/// 外部装饰命令（I：两条流模型；0x28B 墙 remove / 0x30B 屋顶命令）。
pub use country_core::resources::walls::decorator::ExternalDecoratorCmd;

/// B 14081a57c..afce: reanchor decorations on the two old segments when
/// their common stair point is removed. The event's retained pair is sorted.
/// `lengths` come from the previous assembly's sampled 3D curves; missing
/// or invalid curves contribute zero, as in the original consumer.
fn merged_stair_destination(
    retained: (u32, u32), removed: u32,
    old: country_core::resources::walls::decorator::StairDecoratorDst,
    lengths: (f32, f32),
) -> Option<country_core::resources::walls::decorator::StairDecoratorDst> {
    use country_core::resources::walls::decorator::{DecoratorRotationXZ, StairDecoratorDst};
    let (a, b) = retained;
    let pair = |x: u32, y: u32| if x < y { (x, y) } else { (y, x) };
    let first = (old.first_id, old.second_id) == pair(a, removed);
    if !first && (old.first_id, old.second_id) != pair(b, removed) { return None; }
    // MAXSS returns its second operand when unordered.
    let sum = lengths.1 + lengths.0;
    let denominator = if sum > 0.01 { sum } else { 0.01 };
    let split = lengths.0 / denominator;
    let forward = if first { a < removed } else { b > removed };
    let u = if forward { old.coordinate.y } else { utils::scalar::inverse_lerp(1.0, 0.0, old.coordinate.y) };
    let u = if first { split * u } else { split + (1.0 - split) * u };
    let x = if forward { old.coordinate.x } else {
        f32::from_bits(old.coordinate.x.to_bits() ^ 0x8000_0000)
    };
    let yaw = if forward { old.yaw.0 } else { old.yaw.0 + std::f32::consts::PI };
    Some(StairDecoratorDst { first_id: a, second_id: b,
        coordinate: glam::Vec2::new(x, u), yaw: DecoratorRotationXZ(yaw) })
}

/// Replays the third native event stream against the authoritative Stair
/// bucket. The event owner supplies the sorted retained IDs and removed ID.
#[allow(clippy::too_many_arguments)]
fn reanchor_deleted_stair_point(
    retained: (u32, u32), removed: u32,
    requests: &country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests,
    cached: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    storage: &mut DecoratorStorage,
    archivist: &mut country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    archival: &country_core::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    deferred: &mut DeferredDecoratorEditHistory,
    events: &DecoratorOpEvents,
) {
    use country_core::resources::stairs::{stairs_assembly_params::StairGraphEdgeIdentity, stairs_state::StairSegment};
    use country_core::resources::walls::decorator::DecoratorDst;
    use crate::apply_decorator_op::DecoratorChangePatch;
    let length = |a, b| {
        let segment = StairSegment::new(DecoratorId(a), DecoratorId(b));
        requests.prev_requests().sampled_points.get(&StairGraphEdgeIdentity::from(segment))
            .and_then(|p| utils::curve::Curve3::try_new_from_points(p.points.clone(), true).ok())
            .map_or(0.0, |curve| curve.length)
    };
    let lengths = (length(retained.0, removed), length(retained.1, removed));
    let address = DecoratorStorageAddr::Stair;
    // The native consumer clones the bucket before applying mutations.
    let Some(bucket) = storage.get(address) else { return };
    let candidates: Vec<_> = bucket.decorators.iter().map(|(&id, info)| (id, *info)).collect();
    for (id, info) in candidates {
        if info.dst_tag() != 4 { continue; }
        let DecoratorDst::Stair(old) = info.destination() else { continue };
        let Some(destination) = merged_stair_destination(retained, removed, old, lengths) else { continue };
        let position = cached.try_position(id).unwrap_or(glam::Vec3::ZERO);
        let patch = DecoratorChangePatch::rotation(DecoratorId(id), address,
            DecoratorDst::Stair(destination), position);
        archivist.archive_current_decorators_at_with_resources(address, storage, archival, "move preview src");
        apply_decorator_op(&DecoratorOp::from_change(address, patch), 0, address, storage,
            Some(archivist), Some(deferred), events);
    }
}

#[cfg(test)]
mod merge_tests {
    use super::merged_stair_destination;
    use country_core::resources::walls::decorator::{DecoratorRotationXZ, StairDecoratorDst};
    fn dst(a: u32, b: u32, x: f32, u: f32) -> StairDecoratorDst {
        StairDecoratorDst { first_id: a, second_id: b, coordinate: glam::Vec2::new(x, u), yaw: DecoratorRotationXZ(0.25) }
    }
    #[test]
    fn removed_middle_reanchors_both_segments_and_reverses_orientation() {
        let left = merged_stair_destination((2, 9), 5, dst(2, 5, 0.4, 0.5), (2.0, 6.0)).unwrap();
        assert_eq!((left.first_id, left.second_id), (2, 9));
        assert_eq!(left.coordinate, glam::Vec2::new(0.4, 0.125));
        let right = merged_stair_destination((2, 9), 5, dst(5, 9, 0.4, 0.5), (2.0, 6.0)).unwrap();
        assert_eq!(right.coordinate, glam::Vec2::new(0.4, 0.625));
        let reversed = merged_stair_destination((2, 9), 1, dst(1, 2, 0.4, 0.5), (2.0, 6.0)).unwrap();
        assert_eq!(reversed.coordinate, glam::Vec2::new(-0.4, 0.125));
        assert_eq!(reversed.yaw.0, 0.25 + std::f32::consts::PI);
        assert!(merged_stair_destination((2, 9), 5, dst(3, 5, 0.0, 0.0), (2.0, 6.0)).is_none());
        let endpoint = merged_stair_destination((2, 9), 1, dst(1, 2, 0.4, 1.0), (2.0, 6.0)).unwrap();
        assert_eq!(endpoint.coordinate.y.to_bits(), (-0.0f32).to_bits());
    }
}

/// `handle_external_decorator_cmds` @0x14081a380（O 结构）。
pub fn handle_external_decorator_cmds<C: Send + Sync + 'static>(
    mut history: ResMut<History>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    mut deferred: ResMut<DeferredDecoratorEditHistory>,
    events: Res<DecoratorOpEvents>,
    mut cmds: EventReader<ExternalDecoratorCmd>,
    mut stair_points: EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,
    requests: Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,
    cached: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    archival: country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
) {
    for event in stair_points.read() {
        reanchor_deleted_stair_point(event.retained, event.removed, &requests, &cached,
            &mut storage, &mut archivist, &archival, &mut deferred, &events);
    }
    for cmd in cmds.read() {
        if cmd.stream == 0 {
            let id = DecoratorId(cmd.decorator_id);
            let info = *storage.get(cmd.dst).and_then(|bucket| bucket.get(id))
                .expect("bulldozer decorator is missing from its recorded storage");
            let position = glam::Vec3::from_array(std::array::from_fn(|i|
                f32::from_le_bytes(cmd.payload[i * 4..i * 4 + 4].try_into().unwrap())));
            let op = DecoratorOp::remove_at(id, info, position, !history.is_recording());
            apply_decorator_op(&op, 0, cmd.dst, &mut storage, Some(&mut archivist), None, &events);
            // B14081a380 allocates the Arc and writes only Remove's tag 5.
            // Undo data is the entry-owned before/after diff extracted from
            // archivist by history_finish_recording, not inactive union bytes.
            let edit = country_core::resources::history::decorator_edit::DecoratorV5Edit {
                kind: 5, fields: [0; 3],
            };
            history.add(EditType::DecoratorV5(Arc::new(edit)))
                .expect("external decorator removal requires history recording");
            continue;
        }
        let op = DecoratorOp {
            decorator_id: DecoratorId(cmd.decorator_id),
            kind: if cmd.stream == 0 {
                DecoratorOp::KIND_REMOVE
            } else {
                cmd.wall
            },
            info: DecoratorInfo {
                dst_wall: cmd.wall,
                dst_secondary: u64::from(cmd.decorator_id),
                ..DecoratorInfo::default()
            },
            ..DecoratorOp::default()
        };
        let Some(record) = build_external_record(cmd) else {
            continue;
        };
        // 机器码：逐命令 apply + History::add（tag 10，Arc{1,1,kind}）。
        apply_decorator_op(
            &op,
            0,
            cmd.dst,
            &mut storage,
            Some(&mut archivist),
            Some(&mut deferred),
            &events,
        );
        let edit = record_to_history_edit(&record);
        let _ = history.add(EditType::DecoratorV5(Arc::new(edit)));
    }
}

/// 外部命令 → 延迟历史记录（O 载荷 kind：流 A=5、流 B=3）。
fn build_external_record(cmd: &ExternalDecoratorCmd) -> Option<crate::deferred_decorator_history::DeferredDecoratorRecord> {
    let kind = if cmd.stream == 0 {
        crate::deferred_decorator_history::DeferredDecoratorRecord::KIND_REMOVE
    } else {
        crate::deferred_decorator_history::DeferredDecoratorRecord::KIND_CHANGE
    };
    Some(crate::deferred_decorator_history::DeferredDecoratorRecord {
        kind,
        f1: u64::from(cmd.decorator_id),
        ..crate::deferred_decorator_history::DeferredDecoratorRecord::default()
    })
}

/// 外部命令流事件（流 A 0x28B / 流 B 0x30B / cancel 0x10B 的登记面）。
#[allow(dead_code)]
fn _stream_surface() {}
