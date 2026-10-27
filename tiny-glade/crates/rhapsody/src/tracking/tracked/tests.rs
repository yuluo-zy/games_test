fn sealed(cells: &[std::sync::Arc<crate::tracking::SyncCell>]) -> crate::tracking::tracked::SubmissionUseCells {
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
    let mut uses = crate::tracking::tracked::SubmissionUseCells::new(4096, stats, &cells[0]).unwrap();
    for cell in cells {
        uses.insert(cell).unwrap();
    }
    uses.seal();
    uses
}
#[test]
fn normalized_commands_merge_unique_cells_before_spill_admission() {
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
    stats.set_total_budget(1).unwrap();
    let cells: Vec<_> = (0..3)
        .map(|_| std::sync::Arc::new(super::SyncCell::new()))
        .collect();
    let repeated: Vec<_> = (0..1000)
        .map(|index| cells[index % cells.len()].clone())
        .collect();
    let repeated = sealed(&repeated);
    let sources = [repeated.ordered().unwrap(), repeated.ordered().unwrap()];
    let uses = super::collect_submission_uses(&sources, &stats).unwrap();
    assert_eq!(uses.len(), 3);
    let guards = super::lock_submission_uses(uses.ordered(), &stats).unwrap();
    assert_eq!(guards.len(), 3);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    assert!(
        uses.windows(2)
            .all(|p| std::sync::Arc::as_ptr(&p[0]) < std::sync::Arc::as_ptr(&p[1]))
    );
    drop(guards);
    let many: Vec<_> = (0..33)
        .map(|_| std::sync::Arc::new(super::SyncCell::new()))
        .collect();
    let many = sealed(&many);
    assert!(matches!(
        super::collect_submission_uses(
            &[many.ordered().unwrap(), many.ordered().unwrap()],
            &stats
        ),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    stats.set_total_budget(1 << 20).unwrap();
    let uses = super::collect_submission_uses(
        &[many.ordered().unwrap(), many.ordered().unwrap()],
        &stats,
    )
    .unwrap();
    assert_eq!(uses.len(), many.as_slice().len());
    assert!(
        uses.iter()
            .zip(many.as_slice())
            .all(|(a, b)| std::sync::Arc::ptr_eq(a, b))
    );
    drop(uses);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn heap_merge_handles_empty_interleaved_and_repeated_worker_lists() {
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
    let mut cells: Vec<_> = (0..257)
        .map(|_| std::sync::Arc::new(super::SyncCell::new()))
        .collect();
    cells.sort_unstable_by_key(|cell| std::sync::Arc::as_ptr(cell) as usize);
    let lanes: Vec<_> = (0..16)
        .map(|lane| {
            sealed(
                &cells
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| (i + lane) % 7 == 0 || i % 13 == 0)
                    .map(|(_, cell)| cell.clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let empty = super::collect_submission_uses(&[], &stats).unwrap();
    let mut sources: Vec<_> = lanes.iter().map(|lane| lane.ordered().unwrap()).collect();
    sources.push(empty.ordered());
    let actual = super::collect_submission_uses(&sources, &stats).unwrap();
    assert_eq!(actual.len(), cells.len());
    assert!(
        actual
            .iter()
            .zip(&cells)
            .all(|(a, b)| std::sync::Arc::ptr_eq(a, b))
    );
    drop(actual);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    assert!(
        super::collect_submission_uses(&[], &stats)
            .unwrap()
            .is_empty()
    );
    let single = super::collect_submission_uses(&sources[..1], &stats).unwrap();
    assert_eq!(single.len(), lanes[0].as_slice().len());
}
#[test]
fn recording_seal_late_insert_and_resets_preserve_the_only_lock_order() {
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
    let mut cells: Vec<_> = (0..4)
        .map(|_| std::sync::Arc::new(super::SyncCell::new()))
        .collect();
    cells.sort_unstable_by_key(|cell| std::sync::Arc::as_ptr(cell) as usize);
    let mut uses = super::SubmissionUseCells::new(3, stats, &cells[2]).unwrap();
    assert!(uses.ordered().is_err());
    uses.insert(&cells[0]).unwrap();
    uses.insert(&cells[0]).unwrap();
    uses.seal();
    uses.insert(&cells[1]).unwrap();
    assert!(
        uses.as_slice()
            .iter()
            .zip(&cells[..3])
            .all(|(a, b)| std::sync::Arc::ptr_eq(a, b))
    );
    assert!(uses.insert(&cells[3]).is_err());
    assert_eq!(uses.ordered().unwrap().len(), 3);
    uses.release_completed(&cells[2]);
    assert!(uses.is_sealed());
    assert_eq!(uses.as_slice().len(), 1);
    uses.reset(&cells[2]);
    assert!(!uses.is_sealed());
    assert!(uses.ordered().is_err());
    uses.release_completed(&cells[2]);
    assert!(!uses.is_sealed());
    uses.seal();
    uses.insert(&cells[3]).unwrap();
    uses.insert(&cells[0]).unwrap();
    assert!(
        uses.as_slice()
            .windows(2)
            .all(|p| std::sync::Arc::as_ptr(&p[0]) < std::sync::Arc::as_ptr(&p[1]))
    );
    assert_eq!(uses.ordered().unwrap().len(), 3);
}
#[test]
fn failed_spill_does_not_publish_membership_or_break_sealed_order() {
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
    let cells: Vec<_> = (0..3)
        .map(|_| std::sync::Arc::new(super::SyncCell::new()))
        .collect();
    let mut uses = super::SubmissionUseCells::new(8, stats.clone(), &cells[0]).unwrap();
    uses.insert(&cells[1]).unwrap();
    uses.seal();
    let before = stats.cpu_memory_usage().total();
    stats.set_total_budget(before).unwrap();
    assert!(matches!(
        uses.insert(&cells[2]),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(uses.ordered().unwrap().len(), 2);
    assert_eq!(stats.cpu_memory_usage().total(), before);
    stats.set_total_budget(1 << 20).unwrap();
    uses.insert(&cells[2]).unwrap();
    assert_eq!(uses.ordered().unwrap().len(), 3);
    assert!(
        uses.as_slice()
            .windows(2)
            .all(|p| std::sync::Arc::as_ptr(&p[0]) < std::sync::Arc::as_ptr(&p[1]))
    );
    drop(uses);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
use super::*;

fn q(index: usize) -> QueueId {
    QueueId::new(index).unwrap()
}

#[test]
fn submission_rejects_foreign_and_uninitialized_use_cells() {
    let first = crate::sync::DeviceGeneration::next();
    let second = crate::sync::DeviceGeneration::next();
    let cell = SyncCell::for_device(first);
    assert!(cell.validate_device(first).is_ok());
    assert!(cell.validate_device(second).is_err());
    assert!(SyncCell::new().validate_device(first).is_err());
    assert!(
        SyncCell::new()
            .validate_device(crate::sync::DeviceGeneration::INVALID)
            .is_err()
    );
    assert!(cell.snapshot().is_empty());
}

#[test]
fn insert_keeps_the_latest_value_per_queue() {
    let cell = SyncCell::new();
    cell.insert(SyncPoint {
        queue: q(0),
        value: 5,
    });
    cell.insert(SyncPoint {
        queue: q(0),
        value: 3,
    });
    cell.insert(SyncPoint {
        queue: q(2),
        value: 7,
    });

    let snapshot = cell.snapshot();
    assert_eq!(snapshot.get(q(0)), 5, "只增不减");
    assert_eq!(snapshot.get(q(1)), 0);
    assert_eq!(snapshot.get(q(2)), 7);
}

#[test]
fn merge_applies_every_queue_dependency() {
    let cell = SyncCell::new();
    let mut set = SyncSet::from_point(SyncPoint {
        queue: q(0),
        value: 4,
    });
    set.insert(SyncPoint {
        queue: q(1),
        value: 2,
    });
    cell.merge(&set);
    assert_eq!(cell.snapshot(), set);
}

#[test]
fn reached_requires_every_registered_queue() {
    let cell = SyncCell::new();
    cell.insert(SyncPoint {
        queue: q(0),
        value: 4,
    });
    cell.insert(SyncPoint {
        queue: q(1),
        value: 2,
    });

    let mut done = CompletedValues::default();
    done.raise(q(0), 10);
    assert!(!cell.is_reached(&done), "队列 1 尚未完成");
    done.raise(q(1), 2);
    assert!(cell.is_reached(&done));
    assert!(SyncCell::new().is_reached(&CompletedValues::default()));
}

#[test]
fn concurrent_inserts_are_serialized_and_lossless() {
    use std::sync::Arc;

    let cell = Arc::new(SyncCell::new());
    std::thread::scope(|scope| {
        for worker in 0..8u64 {
            let cell = Arc::clone(&cell);
            scope.spawn(move || {
                for value in 1..=1000 {
                    cell.insert(SyncPoint {
                        queue: q((worker % 4) as usize),
                        value,
                    });
                }
            });
        }
    });

    let snapshot = cell.snapshot();
    for queue in 0..4 {
        assert_eq!(snapshot.get(q(queue)), 1000, "每个队列都有线程写满到 1000");
    }
}

#[test]
fn host_reservation_prevents_new_submits_until_access_is_finished() {
    use std::{
        sync::{Arc, mpsc},
        time::Duration,
    };
    let cell = Arc::new(SyncCell::new());
    let host = cell.lock_submission();
    let initial = cell.snapshot();
    let (started, ready) = mpsc::channel();
    let (published, received) = mpsc::channel();
    let worker_cell = cell.clone();
    let worker = std::thread::spawn(move || {
        let cells = sealed(&[worker_cell]);
        let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
        started.send(()).unwrap();
        let _reservation = lock_submission_uses(cells.ordered().unwrap(), &stats).unwrap();
        cells.as_slice()[0].insert_overall(SyncPoint {
            queue: q(0),
            value: 1,
        });
        published.send(()).unwrap();
    });
    ready.recv_timeout(Duration::from_secs(2)).unwrap();
    assert!(received.recv_timeout(Duration::from_millis(20)).is_err());
    assert_eq!(
        cell.snapshot(),
        initial,
        "host completion snapshot remains valid under its reservation"
    );
    drop(host);
    received.recv_timeout(Duration::from_secs(2)).unwrap();
    worker.join().unwrap();
    assert_eq!(cell.snapshot().get(q(0)), 1);
}

#[test]
fn native_state_query_leaves_untracked_image_available_for_real_import_initialization() {
    use super::super::range_state::{NativeState, RangeOwner};
    let budget = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
    let cell = SyncCell::for_device(crate::sync::DeviceGeneration::next())
        .with_ownership_range(OwnershipRange::Image {
            aspects: ash::vk::ImageAspectFlags::COLOR.as_raw(),
            base_mip: 0,
            mip_count: 1,
            base_layer: 0,
            layer_count: 1,
        });
    assert_eq!(cell.with_existing_range_state(|_| Ok(42)).unwrap(), None);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
    let initial = NativeState {
        layout: ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        owner: RangeOwner::Owned(3),
        ..Default::default()
    };
    cell.with_range_state(initial, &budget, |_| Ok(())).unwrap();
    assert_eq!(
        cell.with_existing_range_state(|table| Ok(table.cells()[0].state))
            .unwrap(),
        Some(initial)
    );
}

#[test]
fn standalone_cell_capacity_follows_the_last_recording_owner() {
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
    stats.set_total_budget(1).unwrap();
    assert!(matches!(
        SyncCell::for_device_budgeted(crate::sync::DeviceGeneration::next(), &stats),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    stats.set_total_budget(u64::MAX).unwrap();
    let owner =
        SyncCell::for_device_budgeted(crate::sync::DeviceGeneration::next(), &stats).unwrap();
    let recording = owner.clone();
    let bytes = crate::accounting::shared_cpu_bytes::<SyncCell>().unwrap();
    assert_eq!(stats.cpu_memory_usage().live, bytes);
    drop(owner);
    assert_eq!(stats.cpu_memory_usage().live, bytes);
    drop(recording);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
