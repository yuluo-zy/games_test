use super::*;
use crate::sync::QueueId;
fn options(key: u64, epoch: u64, priority: UploadPriority) -> UploadOptions {
    UploadOptions {
        key: UploadKey(key),
        epoch,
        priority,
    }
}
fn queue(chunk: usize) -> UploadQueue<()> {
    UploadQueue::new(
        Arc::new(AllocatorStats::default()),
        UploadQueueConfig::default(),
        chunk,
    )
    .unwrap()
}
#[test]
fn batches_respect_payload_region_target_bounds_and_physical_fifo() {
    let mut queue = UploadQueue::new(
        Arc::new(AllocatorStats::default()),
        UploadQueueConfig {
            max_jobs: 128,
            ..Default::default()
        },
        1024,
    )
    .unwrap();
    let first = Arc::new(());
    for epoch in 1..=2 {
        queue
            .enqueue_owned(
                first.clone(),
                0,
                vec![epoch as u8; 4],
                options(epoch, epoch, UploadPriority::High),
                UploadMode::OrderedDelta,
            )
            .unwrap();
    }
    for key in 3..103 {
        queue
            .enqueue_owned(
                Arc::new(()),
                0,
                vec![1; 4],
                options(key, 1, UploadPriority::High),
                UploadMode::OrderedDelta,
            )
            .unwrap();
    }
    let bytes = queue.next_batch(BudgetAction::None, 64, 256);
    assert_eq!(bytes.len(), 16);
    assert_eq!(&bytes[..3], &[0, 2, 3]);
    let regions = queue.next_batch(BudgetAction::None, 1024, 3);
    assert_eq!(&regions[..], &[0, 2, 3]);
    let targets = queue.next_batch(BudgetAction::None, 1024, 256);
    assert_eq!(targets.len(), super::super::MAX_UPLOAD_TARGETS);
    assert!(
        !targets.contains(&1),
        "the second write to the same owner stays pending"
    );
}
#[test]
fn unsubmitted_batch_failure_keeps_unrelated_tickets_pending() {
    let mut queue = queue(64);
    let tickets = (0..3)
        .map(|key| {
            queue
                .enqueue_owned(
                    Arc::new(()),
                    0,
                    vec![1; 4],
                    options(key, 1, UploadPriority::Normal),
                    UploadMode::OrderedDelta,
                )
                .unwrap()
        })
        .collect::<Vec<_>>();
    let failed = queue.fail_batch(&[0, 2], Error::OutOfDeviceMemory);
    for index in [0, 2] {
        let status = tickets[index].status.lock();
        assert!(matches!(&*status, UploadStatus::Failed(error) if Arc::ptr_eq(error, &failed)));
    }
    assert!(matches!(*tickets[1].status.lock(), UploadStatus::Pending));
    assert_eq!(queue.queued_jobs(), 1);
    assert!(Arc::ptr_eq(&queue.jobs[0].task, &tickets[1]));
}
// Synthetic values test CPU accounting/publication only; production records
// the proof at command.submit_to success, before fallible acquire registration.
fn point(value: u64) -> SyncPoint {
    SyncPoint {
        queue: QueueId::new(0).unwrap(),
        value,
    }
}
fn record_and_publish(queue: &mut UploadQueue<()>, index: usize, point: SyncPoint) {
    let bytes = queue.job(index).bytes() as u64;
    queue.record_native_submission(1, Some(bytes));
    queue.publish(index, point);
}
#[test]
fn submitted_snapshot_native_proof_survives_post_submit_acquire_error_without_publication() {
    let mut queue = queue(64);
    let ticket = queue.enqueue_owned(Arc::new(()), 0, vec![1; 4],
        options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    // Failure before native submission produces no proof and no work.
    queue.record_native_submission(1, None);
    assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
    // Model native success followed by acquire registration failure: the
    // original protocol leaves this accepted job pending, but GPU work ran.
    let submitted_payload = Some(queue.job(0).bytes() as u64);
    let post_submit_result: Result<()> = Err(Error::OutOfHostMemory);
    queue.record_native_submission(1, submitted_payload);
    assert!(post_submit_result.is_err());
    assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
    assert_eq!(queue.counters.submitted_jobs, 0);
    assert!(matches!(*ticket.status.lock(), UploadStatus::Pending));
    // The retry submits the same payload again: two physical executions,
    // one published ticket. Publication itself must not count a third copy.
    queue.record_native_submission(1, submitted_payload);
    queue.publish(0, point(2));
    assert_eq!(queue.submitted_snapshot().unwrap(), (2, 8));
    assert_eq!(queue.counters.submitted_jobs, 1);
    assert!(matches!(*ticket.status.lock(), UploadStatus::Submitted(p) if p == point(2)));
}
#[test]
fn submitted_snapshot_manager_identity_changes_on_rebuild_and_exhaustion_is_invalid() {
    let first_id = {
        let queue = queue(64);
        assert!(queue.manager_probe_id != 0);
        assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
        queue.manager_probe_id
    };
    let mut replacement = queue(64);
    assert!(replacement.manager_probe_id != 0);
    assert_ne!(replacement.manager_probe_id, first_id);
    // Simulate identity allocation exhaustion without mutating the global
    // token source shared by parallel tests or the production queue protocol.
    replacement.manager_probe_id = 0;
    assert!(matches!(replacement.submitted_snapshot(), Err(Error::InvalidUsage { .. })));
    let ticket = replacement.enqueue_owned(Arc::new(()), 0, vec![1; 4],
        options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    record_and_publish(&mut replacement, 0, point(1));
    assert!(matches!(*ticket.status.lock(), UploadStatus::Submitted(p) if p == point(1)));
    assert!(replacement.submitted_snapshot().is_err());
}
#[test]
fn submitted_snapshot_keeps_successful_prefix_after_failure_cancel_and_refusal() {
    let mut queue = queue(64);
    let first = queue.enqueue_owned(Arc::new(()), 0, vec![1; 4],
        options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    let failed = queue.enqueue_owned(Arc::new(()), 0, vec![2; 8],
        options(2, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    let cancelled = queue.enqueue_owned(Arc::new(()), 0, vec![3; 12],
        options(3, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
    queue.defer();
    assert_eq!(queue.submitted_snapshot().unwrap(), (0, 0));
    record_and_publish(&mut queue, 0, point(1));
    assert!(matches!(*first.status.lock(), UploadStatus::Submitted(p) if p == point(1)));
    assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
    queue.fail_batch(&[0], Error::OutOfDeviceMemory);
    assert!(matches!(*failed.status.lock(), UploadStatus::Failed(_)));
    assert!(queue.cancel_task(&cancelled));
    assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
    let mut oversized = Vec::with_capacity(queue.config.max_cpu_bytes);
    oversized.extend_from_slice(&[7; 4]);
    let pointer = oversized.as_ptr();
    let refused = queue.enqueue_owned(Arc::new(()), 0, oversized,
        options(4, 1, UploadPriority::Normal), UploadMode::OrderedDelta).err().expect("real CPU budget refusal");
    assert_eq!(refused.payload.as_ptr(), pointer);
    assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
    queue.enqueue_owned(Arc::new(()), 0, vec![8; 6],
        options(5, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    assert_eq!(queue.submitted_snapshot().unwrap(), (1, 4));
    record_and_publish(&mut queue, 0, point(2));
    assert_eq!(queue.submitted_snapshot().unwrap(), (2, 10));
}
#[test]
fn submitted_snapshot_counts_effective_regions_not_gaps_or_merged_request_bytes() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let opts = options(1, 1, UploadPriority::Normal);
    queue.enqueue(target.clone(), 0, &[1; 4], opts).unwrap();
    queue.enqueue(target.clone(), 8, &[2; 4], opts).unwrap();
    assert_eq!(queue.job(0).bytes(), 8); // Four-byte gap is not a payload.
    record_and_publish(&mut queue, 0, point(1));
    assert_eq!(queue.submitted_snapshot().unwrap(), (1, 8));
    queue.enqueue(target.clone(), 0, &[1; 4], opts).unwrap();
    queue.enqueue(target.clone(), 8, &[2; 4], opts).unwrap();
    queue.enqueue(target, 3, &[3; 6], opts).unwrap();
    assert_eq!(queue.job(0).bytes(), 12); // Request lengths sum to14, overlap is copied once.
    record_and_publish(&mut queue, 0, point(2));
    assert_eq!(queue.submitted_snapshot().unwrap(), (2, 20));
}
#[test]
fn submitted_snapshot_counts_jobs_sharing_a_success_point_independently() {
    let mut queue = queue(64);
    for key in 0..2 {
        queue.enqueue_owned(Arc::new(()), 0, vec![key as u8; 4],
            options(key, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    }
    record_and_publish(&mut queue, 1, point(7));
    record_and_publish(&mut queue, 0, point(7));
    assert_eq!(queue.submitted_snapshot().unwrap(), (2, 8));
}
#[test]
fn submitted_snapshot_overflow_is_sticky_without_reverting_real_publication() {
    for (jobs, bytes) in [(u64::MAX, 0), (0, u64::MAX - 3)] {
        let mut queue = queue(64);
        queue.native_submitted_jobs = jobs;
        queue.submitted_bytes = bytes;
        let ticket = queue.enqueue_owned(Arc::new(()), 0, vec![1; 4],
            options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        record_and_publish(&mut queue, 0, point(1));
        assert!(matches!(*ticket.status.lock(), UploadStatus::Submitted(p) if p == point(1)));
        assert_eq!(queue.queued_jobs(), 0);
        assert!(matches!(queue.submitted_snapshot(), Err(Error::InvalidUsage { .. })));
        assert_eq!((queue.native_submitted_jobs, queue.submitted_bytes), (jobs.saturating_add(1), bytes));
        queue.enqueue_owned(Arc::new(()), 0, vec![2; 1],
            options(2, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
        record_and_publish(&mut queue, 0, point(2));
        assert!(queue.submitted_snapshot().is_err());
        assert_eq!((queue.native_submitted_jobs, queue.submitted_bytes), (jobs.saturating_add(2), bytes));
    }
    let mut queue = queue(64);
    queue.native_submitted_jobs = u64::MAX - 1;
    queue.submitted_bytes = u64::MAX - 4;
    queue.enqueue_owned(Arc::new(()), 0, vec![3; 4],
        options(1, 1, UploadPriority::Normal), UploadMode::OrderedDelta).unwrap();
    record_and_publish(&mut queue, 0, point(1));
    assert_eq!(queue.submitted_snapshot().unwrap(), (u64::MAX, u64::MAX));
}
#[test]
fn owned_deltas_preserve_all_epochs_and_the_original_allocation() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let bytes = vec![1, 2, 3, 4];
    let pointer = bytes.as_ptr();
    let first = queue
        .enqueue_owned(
            target.clone(),
            0,
            bytes,
            options(1, 1, UploadPriority::Normal),
            UploadMode::OrderedDelta,
        )
        .unwrap();
    queue
        .enqueue_owned(
            target.clone(),
            8,
            vec![5; 4],
            options(1, 2, UploadPriority::High),
            UploadMode::OrderedDelta,
        )
        .unwrap();
    // Even the legacy replacing API must not implicitly cancel a delta.
    queue
        .enqueue(target, 16, &[9; 4], options(1, 3, UploadPriority::High))
        .unwrap();
    assert_eq!(queue.queued_jobs(), 3);
    assert_eq!(queue.job(0).segments[0].bytes().as_ptr(), pointer);
    assert!(matches!(*first.status.lock(), UploadStatus::Pending));
    assert_eq!(queue.next(BudgetAction::None), Some(0));
    queue.publish(0, point(1));
    assert_eq!(queue.job(0).segments[0].offset, 8);
    queue.publish(0, point(2));
    assert_eq!(queue.job(0).segments[0].offset, 16);
}

#[test]
fn owned_admission_failure_returns_storage_and_does_not_advance_epoch() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let mut bytes = Vec::with_capacity(queue.config.max_cpu_bytes);
    bytes.extend_from_slice(&[7; 4]);
    let pointer = bytes.as_ptr();
    let before = queue.stats().known_cpu_bytes;
    let error = queue
        .enqueue_owned(
            target.clone(),
            0,
            bytes,
            options(1, 9, UploadPriority::Normal),
            UploadMode::OrderedDelta,
        )
        .err()
        .expect("budget rejection");
    assert_eq!(error.payload.as_ptr(), pointer);
    assert_eq!(error.payload, [7; 4]);
    assert_eq!(queue.queued_jobs(), 0);
    assert_eq!(queue.stats().known_cpu_bytes, before);
    queue
        .enqueue_owned(
            target,
            0,
            vec![1; 4],
            options(1, 1, UploadPriority::Normal),
            UploadMode::OrderedDelta,
        )
        .unwrap();
}

#[test]
fn owned_deltas_in_the_same_epoch_keep_separate_completion_tickets() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let opts = options(1, 1, UploadPriority::Normal);
    let first = queue
        .enqueue_owned(
            target.clone(),
            0,
            vec![1; 4],
            opts,
            UploadMode::OrderedDelta,
        )
        .unwrap();
    let second = queue
        .enqueue_owned(target, 0, vec![2; 4], opts, UploadMode::OrderedDelta)
        .unwrap();
    assert!(!Arc::ptr_eq(&first, &second));
    queue.publish(0, point(1));
    assert!(matches!(*first.status.lock(), UploadStatus::Submitted(_)));
    assert!(matches!(*second.status.lock(), UploadStatus::Pending));
}
#[test]
fn dirty_regions_coalesce_latest_overlap_and_never_copy_a_gap() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let opts = options(1, 1, UploadPriority::Normal);
    let a = queue.enqueue(target.clone(), 0, &[1; 4], opts).unwrap();
    let b = queue.enqueue(target.clone(), 8, &[2; 4], opts).unwrap();
    assert!(Arc::ptr_eq(&a, &b));
    assert_eq!(queue.job(0).segments.len(), 2);
    assert_eq!(queue.job(0).bytes(), 8, "the untouched gap stays absent");
    queue.enqueue(target, 3, &[3; 6], opts).unwrap();
    let job = queue.job(0);
    assert_eq!(job.segments.len(), 1);
    assert_eq!(
        job.segments[0].bytes(),
        &[1, 1, 1, 3, 3, 3, 3, 3, 3, 2, 2, 2]
    );
    queue.publish(0, point(7));
    assert!(matches!(*a.status.lock(), UploadStatus::Submitted(p) if p == point(7)));
    assert!(matches!(*b.status.lock(), UploadStatus::Submitted(p) if p == point(7)));
    assert_eq!(queue.stats().merged_requests, 2);
}
#[test]
fn priority_is_fifo_and_never_reorders_the_same_physical_destination() {
    let mut queue = queue(4);
    let a = Arc::new(());
    let b = Arc::new(());
    let c = Arc::new(());
    queue
        .enqueue(
            a.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Background),
        )
        .unwrap();
    queue
        .enqueue(a, 0, &[2; 4], options(2, 1, UploadPriority::High))
        .unwrap();
    queue
        .enqueue(b, 0, &[3; 4], options(3, 1, UploadPriority::High))
        .unwrap();
    queue
        .enqueue(c, 0, &[4; 4], options(4, 1, UploadPriority::High))
        .unwrap();
    assert_eq!(queue.next(BudgetAction::Backpressure), Some(2));
    queue.publish(2, point(1));
    assert_eq!(queue.next(BudgetAction::EvictCache), Some(2));
    queue.publish(2, point(2));
    assert_eq!(queue.next(BudgetAction::Backpressure), None);
    assert_eq!(queue.next(BudgetAction::None), Some(0));
    queue.publish(0, point(3));
    assert_eq!(queue.next(BudgetAction::Backpressure), Some(0));
}
#[test]
fn intervening_logical_key_prevents_coalescing_an_earlier_write() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let old = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Normal),
        )
        .unwrap();
    queue
        .enqueue(
            target.clone(),
            0,
            &[2; 4],
            options(2, 1, UploadPriority::Normal),
        )
        .unwrap();
    let newest = queue
        .enqueue(target, 0, &[3; 4], options(1, 1, UploadPriority::High))
        .unwrap();
    assert!(!Arc::ptr_eq(&old, &newest));
    assert_eq!(queue.stats().queued_jobs, 3);
    assert_eq!(queue.next(BudgetAction::None), Some(0));
}
#[test]
fn newer_epochs_cancel_pending_but_not_submitted_and_reject_late_old_requests() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let old = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Normal),
        )
        .unwrap();
    let current = queue
        .enqueue(
            target.clone(),
            0,
            &[2; 4],
            options(1, 2, UploadPriority::Normal),
        )
        .unwrap();
    assert!(matches!(
        *old.status.lock(),
        UploadStatus::Cancelled(UploadCancellation::Superseded)
    ));
    assert!(matches!(
        queue.enqueue(
            target.clone(),
            0,
            &[3; 4],
            options(1, 1, UploadPriority::Normal)
        ),
        Err(Error::InvalidUsage { .. })
    ));
    queue.publish(0, point(9));
    queue.cancel_key_before(&target, UploadKey(1), 3);
    assert!(matches!(*current.status.lock(), UploadStatus::Submitted(p) if p == point(9)));
    assert!(
        queue
            .enqueue(target, 0, &[4; 4], options(1, 2, UploadPriority::Normal))
            .is_err()
    );
}
#[test]
fn failed_admission_preserves_accepted_bytes_epoch_and_budget() {
    let stats = Arc::new(AllocatorStats::default());
    let mut queue = UploadQueue::new(stats.clone(), UploadQueueConfig::default(), 64).unwrap();
    let target = Arc::new(());
    let old = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Normal),
        )
        .unwrap();
    let used = stats.budget_usage().total();
    stats.set_total_budget(used).unwrap();
    assert!(matches!(
        queue.enqueue(
            target.clone(),
            0,
            &[2; 8],
            options(1, 2, UploadPriority::Normal)
        ),
        Err(Error::OutOfHostMemory)
    ));
    assert_eq!(stats.budget_usage().total(), used);
    assert!(matches!(*old.status.lock(), UploadStatus::Pending));
    assert_eq!(queue.job(0).segments[0].bytes(), &[1; 4]);
    stats.set_total_budget(0).unwrap();
    assert!(
        queue
            .enqueue(target, 0, &[3; 4], options(1, 1, UploadPriority::Normal))
            .is_ok()
    );
}
#[test]
fn escaped_ticket_envelopes_backpressure_reuse_and_keep_real_cpu_credit() {
    let stats = Arc::new(AllocatorStats::default());
    let config = UploadQueueConfig {
        max_jobs: 1,
        max_tickets: 1,
        ..Default::default()
    };
    let mut queue = UploadQueue::new(stats.clone(), config, 64).unwrap();
    let target = Arc::new(());
    let held = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::High),
        )
        .unwrap();
    queue.publish(0, point(1));
    assert!(matches!(
        queue.enqueue(
            target.clone(),
            0,
            &[2; 4],
            options(1, 2, UploadPriority::High)
        ),
        Err(Error::Timeout)
    ));
    drop(held);
    let pending = queue
        .enqueue(target, 0, &[2; 4], options(1, 2, UploadPriority::High))
        .unwrap();
    assert_eq!(queue.stats().task_slots, 1);
    drop(queue);
    assert!(matches!(
        *pending.status.lock(),
        UploadStatus::Cancelled(UploadCancellation::ManagerDropped)
    ));
    assert!(stats.cpu_memory_usage().total() > 0);
    drop(pending);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn live_key_directory_is_bounded_and_dead_buffer_identity_can_be_reclaimed() {
    let config = UploadQueueConfig {
        max_keys: 1,
        ..Default::default()
    };
    let mut queue = UploadQueue::new(Arc::new(AllocatorStats::default()), config, 64).unwrap();
    let target = Arc::new(());
    queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Normal),
        )
        .unwrap();
    queue.publish(0, point(1));
    assert!(matches!(
        queue.enqueue(
            Arc::new(()),
            0,
            &[2; 4],
            options(2, 1, UploadPriority::Normal)
        ),
        Err(Error::Timeout)
    ));
    drop(target);
    for epoch in 1..1001 {
        let target = Arc::new(());
        queue
            .enqueue(
                target,
                0,
                &[3; 4],
                options(2, epoch, UploadPriority::Normal),
            )
            .unwrap();
        queue.publish(0, point(epoch + 1));
        assert_eq!(queue.stats().active_keys, 1);
    }
}
#[test]
fn failure_has_no_completion_point_and_explicit_expiry_cannot_be_undone() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let failed = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Normal),
        )
        .unwrap();
    queue.fail_batch(&[0], Error::OutOfDeviceMemory);
    assert!(
        matches!(&*failed.status.lock(), UploadStatus::Failed(error) if matches!(error.error(), Error::OutOfDeviceMemory))
    );
    let cancelled = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 2, UploadPriority::Normal),
        )
        .unwrap();
    assert_eq!(queue.cancel_before(3), 1);
    assert!(matches!(
        *cancelled.status.lock(),
        UploadStatus::Cancelled(UploadCancellation::Explicit)
    ));
    assert!(
        queue
            .enqueue(target, 0, &[1; 4], options(1, 2, UploadPriority::Normal))
            .is_err()
    );
    assert_eq!(queue.stats().submitted_jobs, 0);
}
#[test]
fn escaped_failure_diagnostic_keeps_its_own_budget_after_task_reuse_and_manager_drop() {
    let stats = Arc::new(AllocatorStats::default());
    let mut queue = UploadQueue::new(stats.clone(), UploadQueueConfig::default(), 64).unwrap();
    let target = Arc::new(());
    let ticket = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Normal),
        )
        .unwrap();
    let failure = queue.fail_batch(
        &[0],
        Error::invalid_usage("injected structured upload failure"),
    );
    let original = failure.to_error();
    drop(ticket);
    queue
        .enqueue(target, 0, &[2; 4], options(1, 2, UploadPriority::Normal))
        .unwrap();
    drop(queue);
    drop(failure);
    assert!(
        stats.cpu_memory_usage().total() > 0,
        "the escaped structured source still owns its failure envelope"
    );
    assert!(std::error::Error::source(&original).is_some());
    drop(original);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn pressure_trim_only_releases_cpu_exclusive_envelopes_and_preserves_live_epoch_history() {
    let mut queue = queue(64);
    let target = Arc::new(());
    let held = queue
        .enqueue(
            target.clone(),
            0,
            &[1; 4],
            options(1, 1, UploadPriority::Normal),
        )
        .unwrap();
    queue.publish(0, point(1));
    assert_eq!(queue.trim_idle(), 0);
    drop(held);
    let before = queue.stats().known_cpu_bytes;
    assert_eq!(queue.trim_idle(), 1);
    assert!(queue.stats().known_cpu_bytes < before);
    assert_eq!(queue.stats().active_keys, 1);
    assert!(
        queue
            .enqueue(target, 0, &[1; 4], options(1, 0, UploadPriority::Normal))
            .is_err()
    );
}
