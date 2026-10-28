use super::*;
use super::submission::PreparedResourceStates;
use crate::command::command_buffer::recorded_state::order_recorded_patches;
use crate::command::command_buffer::submission::record_command_uses;
use crate::tracking::{OwnershipRange, SyncCell};
use crate::tracking::range_state::{NativeState, PatchAction, RangeOwner, StatePatch};
use crate::native::AccessScope;
use crate::sync::{QueueId, SyncPoint, SyncSet};

fn queue() -> QueueId {
    QueueId::new(0).unwrap()
}
fn range(offset: u64, size: u64) -> OwnershipRange {
    OwnershipRange::Buffer { offset, size }
}
fn cell(stats: &Arc<crate::accounting::AllocatorStats>) -> Arc<SyncCell> {
    let cell = Arc::new(
        SyncCell::for_device(crate::sync::DeviceGeneration::next())
            .with_ownership_range(range(0, 64)),
    );
    cell.with_range_state(NativeState::default(), stats, |_| Ok(()))
        .unwrap();
    cell
}
fn access(range: OwnershipRange, revision: u64, family: u32) -> StatePatch {
    StatePatch {
        range,
        expected_revision: revision,
        action: PatchAction::Access {
            scope: AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_WRITE,
                write: true,
            },
            layout: vk::ImageLayout::UNDEFINED,
            queue: queue(),
            family,
        },
    }
}
fn guard(
    cells: &[Arc<SyncCell>],
    stats: &Arc<crate::accounting::AllocatorStats>,
) -> PreparedResourceStates {
    PreparedResourceStates {
        cells: crate::native::submit::SubmitScratch::try_collect(
            cells.len(),
            stats,
            cells.iter().cloned().map(Ok),
        )
        .unwrap(),
        queue: queue(),
        dependencies: crate::sync::StagedDependencies::EMPTY,
        published: false,
    }
}
#[test]
fn cell_bucketing_preserves_command_explicit_and_opaque_patch_order() {
    let mut actual = Vec::new();
    for command in 0..16 {
        for kind in 0..2_u8 {
            for entry in 0..37 {
                actual.push(((entry * 3 + command) % 5, kind, command, entry));
            }
        }
    }
    let mut expected = actual.clone();
    expected.sort_by_key(|item| item.0);
    order_recorded_patches(&mut actual);
    assert_eq!(actual, expected);
}

#[test]
fn failed_native_submission_cancels_staged_state_and_retry_publishes_once() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let cell = cell(&stats);
    let cells = [cell.clone()];
    {
        let _gate = cell.lock_submission();
        let mut prepared = guard(&cells, &stats);
        prepared
            .prepare_cell(0, &[access(range(0, 32), 0, 2)], 2, &stats)
            .unwrap();
        // Queue failure or a pre-submit error drops this actual production guard.
    }
    cell.with_range_state(NativeState::default(), &stats, |table| {
        assert_eq!(table.cells().len(), 1);
        assert_eq!(table.cells()[0].state.revision, 0);
        assert_eq!(table.cells()[0].state.owner, RangeOwner::Unclaimed);
        Ok(())
    })
    .unwrap();
    assert!(cell.snapshot().is_empty());
    let point = SyncPoint {
        queue: queue(),
        value: 7,
    };
    {
        let _gate = cell.lock_submission();
        let mut prepared = guard(&cells, &stats);
        prepared
            .prepare_cell(0, &[access(range(0, 32), 0, 2)], 2, &stats)
            .unwrap();
        prepared.publish(point);
        record_command_uses(&cells, point);
    }
    cell.with_range_state(NativeState::default(), &stats, |table| {
        let written = table
            .cells()
            .iter()
            .find(|cell| cell.range == range(0, 32))
            .unwrap();
        assert_eq!(written.state.revision, 1);
        assert_eq!(written.state.owner, RangeOwner::Owned(2));
        assert_eq!(written.state.writer.points, SyncSet::from_point(point));
        assert!(
            table
                .cells()
                .iter()
                .find(|cell| cell.range == range(32, 32))
                .unwrap()
                .state
                .writer
                .points
                .is_empty()
        );
        Ok(())
    })
    .unwrap();
    assert_eq!(cell.snapshot(), SyncSet::from_point(point));
    assert!(cell.validate_queue_range(3, range(0, 32)).is_err());
    assert!(cell.validate_queue_range(3, range(32, 32)).is_ok());
    let compute = QueueId::new(1).unwrap();
    {
        let _gate = cell.lock_submission();
        let mut prepared = guard(&cells, &stats);
        prepared.queue = compute;
        let mut patch = access(range(32, 32), 0, 3);
        if let PatchAction::Access { queue, .. } = &mut patch.action {
            *queue = compute;
        }
        prepared.prepare_cell(0, &[patch], 3, &stats).unwrap();
        let point = SyncPoint {
            queue: compute,
            value: 5,
        };
        prepared.publish(point);
        record_command_uses(&cells, point);
    }
    assert!(cell.validate_queue_range(2, range(0, 32)).is_ok());
    assert!(cell.validate_queue_range(3, range(32, 32)).is_ok());
    assert!(cell.validate_queue_range(2, range(32, 32)).is_err());
    assert!(cell.validate_queue_range(2, range(0, 64)).is_err());
}
#[test]
fn public_opaque_registration_waits_for_prepared_native_publication() {
    use std::{sync::mpsc, time::Duration};
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let cell = cell(&stats);
    let (prepared_tx, prepared_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let native_cell = cell.clone();
    let native = std::thread::spawn(move || {
        let cells = [native_cell.clone()];
        let _gate = native_cell.lock_submission();
        let mut prepared = guard(&cells, &stats);
        prepared
            .prepare_cell(0, &[access(range(0, 64), 0, 2)], 2, &stats)
            .unwrap();
        prepared_tx.send(()).unwrap();
        release_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let point = SyncPoint {
            queue: queue(),
            value: 3,
        };
        prepared.publish(point);
        record_command_uses(&cells, point);
    });
    prepared_rx.recv_timeout(Duration::from_secs(5)).unwrap();
    let opaque_cell = cell.clone();
    let (started_tx, started_rx) = mpsc::channel();
    let (published_tx, published_rx) = mpsc::channel();
    let opaque = std::thread::spawn(move || {
        started_tx.send(()).unwrap();
        opaque_cell.insert(SyncPoint {
            queue: QueueId::new(1).unwrap(),
            value: 9,
        });
        published_tx.send(()).unwrap();
    });
    started_rx.recv_timeout(Duration::from_secs(5)).unwrap();
    let blocked = published_rx
        .recv_timeout(Duration::from_millis(20))
        .is_err();
    // Release before asserting, so even a failed concurrency invariant
    // cannot strand a worker holding the production resource gate.
    release_tx.send(()).unwrap();
    native.join().unwrap();
    opaque.join().unwrap();
    assert!(
        blocked,
        "opaque registration must not cancel a prepared submit"
    );
    assert_eq!(cell.snapshot().get(queue()), 3);
    assert_eq!(cell.snapshot().get(QueueId::new(1).unwrap()), 9);
    cell.with_existing_range_state(|table| {
        let dependencies = table.cells()[0].state.dependencies(true);
        assert_eq!(dependencies.get(queue()), 3);
        assert_eq!(dependencies.get(QueueId::new(1).unwrap()), 9);
        Ok(())
    })
    .unwrap();
}
#[test]
fn recorded_preview_failure_cancels_every_unpublished_split() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let cell = cell(&stats);
    let error = cell
        .preview_range_state::<()>(
            NativeState::default(),
            &stats,
            &[access(range(0, 32), 0, 2), access(range(32, 32), 8, 2)],
            queue(),
            2,
            |_| panic!("a failed preview must not expose its partial program"),
        )
        .unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
    cell.with_range_state(NativeState::default(), &stats, |table| {
        assert_eq!(table.cells().len(), 1);
        assert_eq!(table.cells()[0].state.revision, 0);
        assert_eq!(table.prepared_cells().count(), 0);
        Ok(())
    })
    .unwrap();
    assert!(cell.snapshot().is_empty());
}
#[test]
fn stale_second_resource_aborts_all_prepared_resources_without_advancing_completion() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let first = cell(&stats);
    let second = cell(&stats);
    let cells = [first.clone(), second.clone()];
    {
        let _a = first.lock_submission();
        let _b = second.lock_submission();
        let mut prepared = guard(&cells, &stats);
        prepared
            .prepare_cell(0, &[access(range(0, 64), 0, 2)], 2, &stats)
            .unwrap();
        assert!(matches!(
            prepared.prepare_cell(1, &[access(range(0, 64), 9, 2)], 2, &stats),
            Err(error) if error.kind() == crate::ErrorKind::InvalidUsage
        ));
    }
    for cell in &cells {
        cell.with_range_state(NativeState::default(), &stats, |table| {
            assert_eq!(table.cells()[0].state.revision, 0);
            assert!(table.cells()[0].state.writer.points.is_empty());
            assert_eq!(table.prepared_cells().count(), 0);
            Ok(())
        })
        .unwrap();
        assert!(cell.snapshot().is_empty());
    }
}
