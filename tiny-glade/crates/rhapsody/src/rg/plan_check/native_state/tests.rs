use super::*;
use crate::sync::QueueId;
use crate::tracking::QueueFamilies;
use crate::tracking::range_state::{RangeStateConfig, RangeStateTable, RangeTransfer};
use crate::accounting::AllocatorStats;
use ash::vk;
use std::sync::Arc;

fn point(queue: usize, value: u64) -> SyncPoint {
    SyncPoint {
        queue: QueueId::new(queue).unwrap(),
        value,
    }
}
fn buffer(offset: u64, size: u64) -> OwnershipRange {
    OwnershipRange::Buffer { offset, size }
}
fn scope(stage: vk::PipelineStageFlags, write: bool) -> AccessScope {
    AccessScope {
        stages: stage,
        access: if write {
            vk::AccessFlags::SHADER_WRITE
        } else {
            vk::AccessFlags::SHADER_READ
        },
        write,
    }
}
fn access(range: OwnershipRange, revision: u64, queue: usize, write: bool) -> StatePatch {
    StatePatch {
        range,
        expected_revision: revision,
        action: PatchAction::Access {
            scope: scope(vk::PipelineStageFlags::COMPUTE_SHADER, write),
            layout: vk::ImageLayout::GENERAL,
            queue: QueueId::new(queue).unwrap(),
            family: queue as u32,
        },
    }
}
fn table(domain: OwnershipRange, sharing: ResourceSharing) -> RangeStateTable {
    RangeStateTable::new(
        domain,
        sharing,
        NativeState::default(),
        RangeStateConfig::default(),
        Arc::new(AllocatorStats::default()),
    )
    .unwrap()
}
fn apply(
    table: &mut RangeStateTable,
    oracle: &mut StateOracle,
    patches: &[StatePatch],
    submitted: SyncPoint,
    waits: SyncSet,
) {
    let family = submitted.queue.index() as u32;
    let before = table.cells().to_vec();
    table.prepare(patches, submitted.queue, family).unwrap();
    table.publish(submitted);
    oracle
        .apply_success(patches, submitted, family, waits)
        .unwrap();
    oracle.check_snapshot(table.cells()).unwrap();
    check_state_publication(StatePublication {
        domain: table.domain(),
        sharing: oracle.sharing,
        before: &before,
        patches,
        submitted,
        family,
        waits,
        after: table.cells(),
    })
    .unwrap();
}

#[test]
fn readers_from_three_queues_are_preserved_and_every_reader_is_required_before_write() {
    let whole = buffer(0, 16);
    let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1, 2]).unwrap());
    let mut table = table(whole, sharing);
    let mut oracle = StateOracle::new(whole, sharing, table.cells()).unwrap();
    apply(
        &mut table,
        &mut oracle,
        &[access(whole, 0, 0, true)],
        point(0, 1),
        SyncSet::EMPTY,
    );
    for queue in [1, 2] {
        apply(
            &mut table,
            &mut oracle,
            &[access(buffer(0, 8), queue, queue as usize, false)],
            point(queue as usize, 1),
            point(0, 1).into(),
        );
    }
    let write = access(buffer(0, 8), 3, 0, true);
    for missing in [1, 2] {
        let mut waits = SyncSet::EMPTY;
        waits.insert(point(3 - missing, 1));
        assert!(
            oracle
                .apply_success(&[write], point(0, 2), 0, waits)
                .is_err()
        );
        oracle.check_snapshot(table.cells()).unwrap();
    }
    let mut waits = SyncSet::from(point(1, 1));
    waits.insert(point(2, 1));
    apply(&mut table, &mut oracle, &[write], point(0, 2), waits);

    // This patch reaches one matching region before a later stale region.
    // Neither the real table nor the independent oracle may partly commit.
    let stale = access(whole, 4, 0, true);
    assert!(table.prepare(&[stale], point(0, 3).queue, 0).is_err());
    assert!(
        oracle
            .apply_success(&[stale], point(0, 3), 0, waits)
            .is_err()
    );
    oracle.check_snapshot(table.cells()).unwrap();
    table.cancel_prepared();
    apply(
        &mut table,
        &mut oracle,
        &[access(buffer(8, 8), 1, 2, true)],
        point(2, 2),
        point(0, 1).into(),
    );
    // A later failed batch cannot roll back this already successful prefix.
    assert!(
        oracle
            .apply_success(&[access(whole, 0, 1, true)], point(1, 2), 1, waits)
            .is_err()
    );
    oracle.check_snapshot(table.cells()).unwrap();
}

#[test]
fn image_publications_match_dense_revision_grid_across_queues() {
    let whole = OwnershipRange::Image {
        aspects: 0b110,
        base_mip: 0,
        mip_count: 3,
        base_layer: 0,
        layer_count: 4,
    };
    let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1, 2]).unwrap());
    let mut table = table(whole, sharing);
    let mut oracle = StateOracle::new(whole, sharing, table.cells()).unwrap();
    let mut dense = [[[0u64; 4]; 3]; 2];
    let mut counts = [0u64; 3];
    let mut seed = 0x938b_ac76_u64;
    for _ in 0..200 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let aspect = seed as usize % 2;
        let mip = (seed >> 8) as usize % 3;
        let layer = (seed >> 16) as usize % 4;
        let queue = (seed >> 24) as usize % 3;
        let mut waits = SyncSet::EMPTY;
        for (prior_queue, value) in counts.into_iter().enumerate() {
            if value != 0 {
                waits.insert(point(prior_queue, value));
            }
        }
        counts[queue] += 1;
        let range = OwnershipRange::Image {
            aspects: 1 << (aspect + 1),
            base_mip: mip as u32,
            mip_count: 1,
            base_layer: layer as u32,
            layer_count: 1,
        };
        apply(
            &mut table,
            &mut oracle,
            &[access(
                range,
                dense[aspect][mip][layer],
                queue,
                seed & 1 != 0,
            )],
            point(queue, counts[queue]),
            waits,
        );
        dense[aspect][mip][layer] += 1;
        for (plane, rows) in dense.iter().enumerate() {
            for (mip, row) in rows.iter().enumerate() {
                for (layer, revision) in row.iter().enumerate() {
                    oracle
                        .cells
                        .visit(
                            OwnershipRange::Image {
                                aspects: 1 << (plane + 1),
                                base_mip: mip as u32,
                                mip_count: 1,
                                base_layer: layer as u32,
                                layer_count: 1,
                            },
                            |state| {
                                assert_eq!(state.revision, *revision);
                                Ok(())
                            },
                        )
                        .unwrap();
                }
            }
        }
    }
}

#[test]
fn acquire_requires_exact_release_identity_real_wait_and_matching_layout() {
    let whole = OwnershipRange::Image {
        aspects: 1,
        base_mip: 0,
        mip_count: 2,
        base_layer: 0,
        layer_count: 2,
    };
    let sharing = ResourceSharing::Exclusive;
    let mut table = table(whole, sharing);
    let mut oracle = StateOracle::new(whole, sharing, table.cells()).unwrap();
    apply(
        &mut table,
        &mut oracle,
        &[access(whole, 0, 0, true)],
        point(0, 1),
        SyncSet::EMPTY,
    );
    let transfer = RangeTransfer {
        pair: 53,
        source_family: 0,
        destination_family: 1,
        range: whole,
        source: scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
        destination: scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
        old_layout: vk::ImageLayout::GENERAL,
        new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    };
    apply(
        &mut table,
        &mut oracle,
        &[StatePatch {
            range: whole,
            expected_revision: 1,
            action: PatchAction::Release(transfer),
        }],
        point(0, 2),
        SyncSet::EMPTY,
    );
    let acquire = StatePatch {
        range: whole,
        expected_revision: 2,
        action: PatchAction::Acquire {
            transfer,
            release_point: point(0, 2),
        },
    };
    assert!(
        oracle
            .apply_success(&[acquire], point(1, 1), 1, point(0, 1).into())
            .is_err()
    );
    oracle.check_snapshot(table.cells()).unwrap();
    let wrong = StatePatch {
        action: PatchAction::Acquire {
            transfer,
            release_point: point(0, 1),
        },
        ..acquire
    };
    assert!(table.prepare(&[wrong], point(1, 1).queue, 1).is_err());
    assert!(
        oracle
            .apply_success(&[wrong], point(1, 1), 1, point(0, 2).into())
            .is_err()
    );
    oracle.check_snapshot(table.cells()).unwrap();
    table.cancel_prepared();
    apply(
        &mut table,
        &mut oracle,
        &[acquire],
        point(1, 1),
        point(0, 2).into(),
    );
    let mut forged = table.cells().to_vec();
    forged[0].state.layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
    assert!(oracle.check_snapshot(&forged).is_err());
    forged[0] = table.cells()[0];
    forged[0].state.writer.scopes[1].stages = vk::PipelineStageFlags::empty();
    assert!(oracle.check_snapshot(&forged).is_err());
}

#[test]
fn one_submit_replays_ordered_patches_before_publishing_its_single_receipt() {
    let whole = buffer(0, 16);
    let mut table = table(whole, ResourceSharing::Exclusive);
    let mut oracle =
        StateOracle::new(whole, ResourceSharing::Exclusive, table.cells()).unwrap();
    apply(
        &mut table,
        &mut oracle,
        &[
            access(whole, 0, 0, true),
            access(buffer(0, 8), 1, 0, false),
            access(buffer(0, 8), 2, 0, true),
            access(buffer(8, 8), 1, 0, false),
        ],
        point(0, 1),
        SyncSet::EMPTY,
    );
    oracle
        .cells
        .visit(buffer(0, 8), |state| {
            assert_eq!(state.revision, 3);
            assert!(state.readers.points.is_empty());
            Ok(())
        })
        .unwrap();
    oracle
        .cells
        .visit(buffer(8, 8), |state| {
            assert_eq!(state.revision, 2);
            assert_eq!(state.writer.points.get(point(0, 1).queue), 1);
            assert_eq!(state.readers.points.get(point(0, 1).queue), 1);
            Ok(())
        })
        .unwrap();
}
