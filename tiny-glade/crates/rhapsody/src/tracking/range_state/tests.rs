use super::*;
use crate::tracking::ResourceSharing;
use crate::sync::{QueueId, SyncPoint};
use crate::native::access_table::AccessScope;
use crate::accounting::AllocatorStats;
use ash::vk;
use std::sync::Arc;

#[test]
fn ownership_admission_is_one_rule_for_every_owner_state() {
    let exclusive = ResourceSharing::Exclusive;
    let concurrent =
        ResourceSharing::Concurrent(crate::tracking::QueueFamilies::new(&[0, 2]).unwrap());
    let released = RangeOwner::Released {
        transfer: RangeTransfer {
            pair: 1,
            source_family: 2,
            destination_family: 0,
            range: NativeRange::Buffer { offset: 0, size: 4 },
            source: AccessScope::EMPTY,
            destination: AccessScope::EMPTY,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::UNDEFINED,
        },
        point: SyncPoint {
            queue: QueueId::new(1).unwrap(),
            value: 1,
        },
    };
    assert_eq!(RangeOwner::Unclaimed.admits(exclusive, 0), Ok(()));
    assert_eq!(RangeOwner::Owned(0).admits(exclusive, 0), Ok(()));
    assert_eq!(
        RangeOwner::Owned(2).admits(exclusive, 0),
        Err(OwnershipDenial::OtherFamily)
    );
    assert_eq!(
        released.admits(exclusive, 0),
        Err(OwnershipDenial::Released)
    );
    // Concurrent sharing ignores ownership but not membership.
    assert_eq!(released.admits(concurrent, 0), Ok(()));
    assert_eq!(
        RangeOwner::Unclaimed.admits(concurrent, 1),
        Err(OwnershipDenial::NotShared)
    );
}
#[test]
fn discarded_image_contents_are_ownerless_but_buffers_and_defined_images_are_not() {
    let exclusive = ResourceSharing::Exclusive;
    let image = NativeRange::Image {
        aspects: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip: 0,
        mip_count: 1,
        base_layer: 0,
        layer_count: 1,
    };
    let buffer = NativeRange::Buffer { offset: 0, size: 16 };
    let undefined = NativeState::default();
    let defined = NativeState {
        layout: vk::ImageLayout::GENERAL,
        ..NativeState::default()
    };
    assert!(undefined.contents_discarded(image));
    assert!(!undefined.contents_discarded(buffer), "buffers have no layout");
    assert!(!defined.contents_discarded(image));
    let owner = RangeOwner::Owned(2);
    assert_eq!(owner.admits_discarding(exclusive, 0, true), Ok(()));
    assert_eq!(
        owner.admits_discarding(exclusive, 0, false),
        Err(OwnershipDenial::OtherFamily)
    );
}
fn q(index: usize) -> QueueId {
    QueueId::new(index).unwrap()
}
fn point(queue: usize, value: u64) -> SyncPoint {
    SyncPoint {
        queue: q(queue),
        value,
    }
}
fn read() -> AccessScope {
    AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_READ,
        write: false,
    }
}
fn write() -> AccessScope {
    AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_WRITE,
        write: true,
    }
}
fn bytes(offset: u64, size: u64) -> NativeRange {
    NativeRange::Buffer { offset, size }
}
fn table(domain: NativeRange, state: NativeState) -> RangeStateTable {
    RangeStateTable::new(
        domain,
        ResourceSharing::Exclusive,
        state,
        RangeStateConfig::default(),
        Arc::new(AllocatorStats::default()),
    )
    .unwrap()
}
fn access(
    range: NativeRange,
    revision: u64,
    scope: AccessScope,
    layout: vk::ImageLayout,
    queue: usize,
) -> StatePatch {
    StatePatch {
        range,
        expected_revision: revision,
        action: PatchAction::Access {
            scope,
            layout,
            queue: q(queue),
            family: 0,
        },
    }
}
#[test]
fn opaque_public_points_preserve_all_older_unordered_queues() {
    let mut table = table(bytes(0, 16), NativeState::default());
    table.record_opaque_point(point(0, 7));
    table.record_opaque_point(point(1, 9));
    let dependencies = table.host_range_snapshot(bytes(0, 16), true).unwrap();
    assert_eq!(dependencies.get(q(0)), 7);
    assert_eq!(dependencies.get(q(1)), 9);
    assert_eq!(table.cells()[0].state.revision, 2);
}
#[test]
fn disjoint_intersections_never_evaluate_negative_extents() {
    assert_eq!(intersection(bytes(0, 4), bytes(8, 4)), None);
    assert_eq!(intersection(bytes(8, 4), bytes(0, 4)), None);
    assert_eq!(intersection(bytes(0, 4), bytes(4, 4)), None);
    let image = |mip, layer| NativeRange::Image {
        aspects: 1,
        base_mip: mip,
        mip_count: 1,
        base_layer: layer,
        layer_count: 1,
    };
    assert_eq!(intersection(image(0, 0), image(2, 0)), None);
    assert_eq!(intersection(image(2, 0), image(0, 0)), None);
    assert_eq!(intersection(image(0, 0), image(0, 2)), None);
    assert_eq!(intersection(image(0, 2), image(0, 0)), None);
    assert_eq!(
        difference(image(0, 0), image(0, 2)).as_slice(),
        &[image(0, 0)]
    );
}
#[test]
fn disjoint_versions_and_late_failure_are_transactional() {
    let mut table = table(bytes(0, 16), NativeState::default());
    table
        .prepare(
            &[access(
                bytes(0, 8),
                0,
                write(),
                vk::ImageLayout::UNDEFINED,
                0,
            )],
            q(0),
            0,
        )
        .unwrap();
    assert_eq!(table.cells().len(), 1);
    table.publish(point(0, 1));
    assert_eq!(table.cells().len(), 2);
    assert!(
        table
            .prepare(
                &[
                    access(bytes(8, 8), 0, write(), vk::ImageLayout::UNDEFINED, 0),
                    access(bytes(0, 8), 0, read(), vk::ImageLayout::UNDEFINED, 0)
                ],
                q(0),
                0
            )
            .is_err()
    );
    assert_eq!(
        table
            .cells()
            .iter()
            .find(|cell| cell.range == bytes(8, 8))
            .unwrap()
            .state
            .revision,
        0
    );
    table
        .prepare(
            &[access(
                bytes(8, 8),
                0,
                write(),
                vk::ImageLayout::UNDEFINED,
                0,
            )],
            q(0),
            0,
        )
        .unwrap();
    table.publish(point(0, 2));
    assert_eq!(
        table
            .host_range_snapshot(bytes(0, 8), false)
            .unwrap()
            .get(q(0)),
        1
    );
    assert_eq!(
        table
            .host_range_snapshot(bytes(8, 8), false)
            .unwrap()
            .get(q(0)),
        2
    );
}
#[test]
fn read_layout_conversion_is_a_checkpoint_for_another_queue() {
    let domain = NativeRange::Image {
        aspects: 1,
        base_mip: 0,
        mip_count: 1,
        base_layer: 0,
        layer_count: 1,
    };
    let mut table = table(domain, NativeState::default());
    table
        .prepare(
            &[access(
                domain,
                0,
                read(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                0,
            )],
            q(0),
            0,
        )
        .unwrap();
    table.publish(point(0, 7));
    let state = table.cells()[0].state;
    assert_eq!(state.dependencies(false).get(q(0)), 7);
    assert!(state.source_scope(q(1), false).is_empty());
    table
        .prepare(
            &[access(
                domain,
                1,
                read(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                1,
            )],
            q(1),
            0,
        )
        .unwrap();
    table.publish(point(1, 9));
    assert_eq!(table.cells()[0].state.dependencies(false).get(q(0)), 7);
    assert_eq!(table.cells()[0].state.dependencies(true).get(q(1)), 9);
}
#[test]
fn acquire_checkpoints_a_real_release_and_partial_pair_is_rejected() {
    let range = bytes(0, 16);
    let transfer = RangeTransfer {
        pair: 1,
        source_family: 0,
        destination_family: 1,
        range,
        source: write(),
        destination: read(),
        old_layout: vk::ImageLayout::UNDEFINED,
        new_layout: vk::ImageLayout::UNDEFINED,
    };
    let mut table = table(
        range,
        NativeState {
            owner: RangeOwner::Owned(0),
            ..NativeState::default()
        },
    );
    assert!(
        table
            .prepare(
                &[StatePatch {
                    range: bytes(0, 8),
                    expected_revision: 0,
                    action: PatchAction::Release(transfer)
                }],
                q(0),
                0
            )
            .is_err()
    );
    table
        .prepare(
            &[StatePatch {
                range,
                expected_revision: 0,
                action: PatchAction::Release(transfer),
            }],
            q(0),
            0,
        )
        .unwrap();
    table.publish(point(0, 2));
    table
        .prepare(
            &[StatePatch {
                range,
                expected_revision: 1,
                action: PatchAction::Acquire {
                    transfer,
                    release_point: point(0, 2),
                },
            }],
            q(1),
            1,
        )
        .unwrap();
    table.publish(point(1, 3));
    assert_eq!(table.cells()[0].state.dependencies(false).get(q(1)), 3);
    assert!(!table.cells()[0].state.writer.combined().write);
}
#[test]
fn host_access_rejects_released_contents_and_discard_preserves_exhausted_revision() {
    let range = bytes(0, 16);
    let transfer = RangeTransfer { pair: 7, source_family: 0, destination_family: 1,
        range, source: write(), destination: read(),
        old_layout: vk::ImageLayout::UNDEFINED, new_layout: vk::ImageLayout::UNDEFINED };
    let mut table = table(range, NativeState { revision: u64::MAX,
        owner: RangeOwner::Released { transfer, point: point(0, 2) }, ..Default::default() });
    assert_eq!(table.prepare_host(bytes(4, 4), true).unwrap_err().kind(), crate::ErrorKind::InvalidUsage);
    assert!(matches!(table.cells()[0].state.owner, RangeOwner::Released { .. }));
    table.discard_idle_contents();
    assert_eq!(table.cells().len(), 1);
    assert_eq!(table.cells()[0].range, range);
    assert_eq!(table.cells()[0].state.revision, u64::MAX);
    assert_eq!(table.cells()[0].state.owner, RangeOwner::Unclaimed);
    assert!(table.cells()[0].state.dependencies(true).is_empty());
}

#[test]
fn host_write_advances_only_its_range_without_a_gpu_point() {
    let mut table = table(bytes(0, 16), NativeState::default());
    table.prepare_host(bytes(4, 4), true).unwrap();
    table.publish_host();
    let state = table
        .cells()
        .iter()
        .find(|cell| cell.range == bytes(4, 4))
        .unwrap()
        .state;
    assert_eq!(state.revision, 1);
    assert!(state.writer.points.is_empty());
    assert_eq!(
        state.source_scope(q(1), false).access,
        vk::AccessFlags::HOST_WRITE
    );
    assert!(
        table
            .prepare(
                &[access(
                    bytes(4, 4),
                    0,
                    read(),
                    vk::ImageLayout::UNDEFINED,
                    0
                )],
                q(0),
                0
            )
            .is_err()
    );
}
#[test]
fn split_limit_failure_keeps_committed_state_and_credit() {
    let stats = Arc::new(AllocatorStats::default());
    let mut table = RangeStateTable::new(
        bytes(0, 16),
        ResourceSharing::Exclusive,
        NativeState::default(),
        RangeStateConfig {
            max_cells: 2,
            max_cpu_bytes: 1 << 20,
        },
        stats.clone(),
    )
    .unwrap();
    assert!(matches!(
        table.prepare(
            &[access(
                bytes(4, 4),
                0,
                write(),
                vk::ImageLayout::UNDEFINED,
                0
            )],
            q(0),
            0
        ),
        Err(error) if error.kind() == crate::ErrorKind::Exhausted
    ));
    assert_eq!(
        table.cells(),
        &[RangeCell {
            range: bytes(0, 16),
            state: NativeState::default()
        }]
    );
    drop(table);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
// 完整 prepare 路径作为前缀重放 oracle，逐边界比较全部暂存标记及预算历史。
