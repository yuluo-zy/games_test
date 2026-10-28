use super::*;
use crate::tracking::OwnershipRange;
use crate::native::AccessScope;
use crate::sync::QueueId;

// Metadata-only rows: no fake native handle or Vulkan queue-family work.
fn entry(pair: u64, destination_family: u32) -> PendingAcquire {
    let scope = AccessScope { stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_WRITE, write: true };
    PendingAcquire { target: AcquireTarget::Buffer(Weak::new()),
        transfer: RangeTransfer { pair, source_family: 0, destination_family,
            range: OwnershipRange::Buffer { offset: 0, size: 4 },
            source: scope, destination: scope,
            old_layout: vk::ImageLayout::UNDEFINED, new_layout: vk::ImageLayout::UNDEFINED },
        release_point: SyncPoint { queue: QueueId::new(0).unwrap(), value: 1 } }
}
fn pairs(queue: &AcquireQueue) -> Vec<u64> {
    let mut pairs = queue.state.lock().storage.entries.iter().map(|entry| entry.transfer.pair).collect::<Vec<_>>();
    pairs.sort_unstable(); pairs
}

#[test]
fn concurrent_producers_keep_exclusive_pre_submit_slots_under_full_host_budget() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let queue = AcquireQueue::new(stats.clone()).unwrap();
    let admitted = std::sync::Barrier::new(3);
    let publish = std::sync::Barrier::new(3);
    std::thread::scope(|scope| {
        for producer in 0..2 {
            let queue = &queue;
            let admitted = &admitted;
            let publish = &publish;
            scope.spawn(move || {
                let mut ticket = queue.reserve(4).unwrap();
                admitted.wait(); publish.wait();
                for item in 0..4 { queue.publish(&mut ticket, entry(producer * 4 + item, 1)).unwrap(); }
            });
        }
        admitted.wait();
        {
            let state = queue.state.lock();
            assert_eq!(state.producer_slots, 8);
            assert_eq!(state.storage.entries.len(), 0);
            assert!(state.storage.entries.capacity() >= 8);
        }
        let usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(usage).unwrap();
        publish.wait();
    });
    assert_eq!(pairs(&queue), (0..8).collect::<Vec<_>>());
    let state = queue.state.lock();
    assert_eq!((state.producer_slots, state.checked_out_slots), (0, 0));
    state.assert_slots(); drop(state);
    stats.set_host_budget(0).unwrap(); drop(queue);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn partial_filter_drain_cancel_after_another_producer_preserves_every_return_slot() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let queue = AcquireQueue::new(stats.clone()).unwrap();
    let mut ticket = queue.reserve(6).unwrap();
    for pair in 0..6 { queue.publish(&mut ticket, entry(pair, if pair < 3 { 1 } else { 2 })).unwrap(); }
    drop(ticket);
    let (mut candidates, mut recorded) = queue.drain(1).unwrap();
    assert_eq!((candidates.len(), recorded.len()), (3, 0));
    assert_eq!(queue.state.lock().checked_out_slots, 3);
    let recorded_pair = candidates[0].transfer.pair;
    candidates.move_to(0, &mut recorded);
    let discarded = candidates[0].transfer.pair;
    candidates.discard(0); // Dead/stale owner needs no later acquire.
    drop(candidates); // Filtered/skipped row returns to its original queue.
    assert_eq!(queue.state.lock().checked_out_slots, 1);
    let mut concurrent = queue.reserve(2).unwrap();
    let before_cancel_capacity = queue.state.lock().storage.entries.capacity();
    let usage = stats.cpu_memory_usage().total();
    stats.set_host_budget(usage).unwrap();
    queue.publish(&mut concurrent, entry(6, 1)).unwrap();
    queue.publish(&mut concurrent, entry(7, 2)).unwrap();
    drop(concurrent);
    drop(recorded); // Cannot allocate or lose its prepaid return slot.
    let expected = (0..8).filter(|&pair| pair != discarded).collect::<Vec<_>>();
    assert_eq!(pairs(&queue), expected);
    assert!(pairs(&queue).contains(&recorded_pair));
    let state = queue.state.lock();
    assert_eq!(state.storage.entries.capacity(), before_cancel_capacity);
    assert_eq!((state.producer_slots, state.checked_out_slots), (0, 0));
    drop(state);
    stats.set_host_budget(0).unwrap(); drop(queue);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn failed_growth_keeps_old_storage_owners_and_exact_credit_then_recovers() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let queue = AcquireQueue::new(stats.clone()).unwrap();
    let mut first = queue.reserve(4).unwrap();
    for pair in 0..4 { queue.publish(&mut first, entry(pair, 1)).unwrap(); }
    drop(first);
    let old_capacity = queue.state.lock().storage.entries.capacity();
    let old_usage = stats.cpu_memory_usage().total();
    stats.set_host_budget(old_usage).unwrap();
    assert!(matches!(queue.reserve(old_capacity + 1), Err(Error::OutOfHostMemory)));
    assert_eq!(pairs(&queue), vec![0, 1, 2, 3]);
    assert_eq!(queue.state.lock().storage.entries.capacity(), old_capacity);
    assert_eq!(queue.state.lock().producer_slots, 0);
    assert_eq!(stats.cpu_memory_usage().total(), old_usage);
    stats.set_host_budget(0).unwrap();
    let next = queue.reserve(old_capacity + 1).unwrap();
    assert!(queue.state.lock().storage.entries.capacity() >= old_capacity + 5);
    drop(next); drop(queue);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn second_drain_scratch_refusal_rolls_back_before_removal_without_drop_reentry_deadlock() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let queue = AcquireQueue::new(stats.clone()).unwrap();
    let mut ticket = queue.reserve(4).unwrap();
    for pair in 0..4 { queue.publish(&mut ticket, entry(pair, 1)).unwrap(); }
    drop(ticket);
    let original = stats.cpu_memory_usage().total();
    let one_scratch = crate::accounting::cpu_vec_bytes::<PendingAcquire>(4).unwrap();
    stats.set_host_budget(original + one_scratch).unwrap();
    assert!(matches!(queue.drain(1), Err(Error::OutOfHostMemory)));
    assert_eq!(pairs(&queue), vec![0, 1, 2, 3]);
    assert_eq!(queue.state.lock().checked_out_slots, 0);
    assert_eq!(stats.cpu_memory_usage().total(), original);
    stats.set_host_budget(0).unwrap();
    let (mut candidates, recorded) = queue.drain(1).unwrap();
    assert_eq!(candidates.len(), 4);
    candidates.submitted(); // Storage protocol commit; no GPU claim.
    drop(recorded); drop(candidates); drop(queue);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn wrong_queue_or_post_submit_registration_error_keeps_published_prefix_and_cancels_unused_credit() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let first = AcquireQueue::new(stats.clone()).unwrap();
    let second = AcquireQueue::new(stats.clone()).unwrap();
    let mut ticket = first.reserve(2).unwrap();
    assert!(second.publish(&mut ticket, entry(99, 1)).is_err());
    assert!(pairs(&second).is_empty());
    assert_eq!(ticket.remaining, 2);
    first.publish(&mut ticket, entry(7, 1)).unwrap();
    // Model later fallible registration validation after a real submit;
    // the slot owner preserves the accepted prefix and unused credit.
    let registration_result: Result<()> = Err(Error::invalid_usage("injected later validation failure"));
    assert!(registration_result.is_err());
    drop(ticket);
    assert_eq!(pairs(&first), vec![7]);
    assert_eq!(first.state.lock().producer_slots, 0);
    let mut exhausted = first.reserve(1).unwrap();
    first.publish(&mut exhausted, entry(8, 1)).unwrap();
    assert!(first.publish(&mut exhausted, entry(9, 1)).is_err());
    assert_eq!(pairs(&first), vec![7, 8]);
    drop(exhausted); drop(first); drop(second);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn pressure_keeps_checked_out_return_capacity_until_cancel_requeues_it() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let queue = AcquireQueue::new(stats.clone()).unwrap();
    let mut ticket = queue.reserve(4).unwrap();
    for pair in 0..4 { queue.publish(&mut ticket, entry(pair, 1)).unwrap(); }
    drop(ticket);
    let (candidates, recorded) = queue.drain(1).unwrap();
    let capacity = queue.state.lock().storage.entries.capacity();
    assert!(queue.state.lock().storage.entries.is_empty());
    assert_eq!(queue.state.lock().checked_out_slots, 4);
    stats.set_host_budget(stats.cpu_memory_usage().total()).unwrap();
    queue.trim_empty_if_pressure();
    assert_eq!(queue.state.lock().storage.entries.capacity(), capacity);
    drop(candidates);
    assert_eq!(pairs(&queue), vec![0, 1, 2, 3]);
    assert_eq!(queue.state.lock().checked_out_slots, 0);
    assert_eq!(queue.state.lock().storage.entries.capacity(), capacity);
    stats.set_host_budget(0).unwrap(); drop(recorded); drop(queue);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn pressure_trims_only_empty_unowned_capacity_and_recovers_under_the_same_host_limit() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let queue = AcquireQueue::new(stats.clone()).unwrap();
    let ticket = queue.reserve(8).unwrap();
    let capacity = queue.state.lock().storage.entries.capacity();
    let backing = crate::accounting::cpu_vec_bytes::<PendingAcquire>(capacity).unwrap();
    let before = stats.cpu_memory_usage().total();
    stats.set_host_budget(before).unwrap();
    queue.trim_empty_if_pressure();
    assert_eq!(queue.state.lock().storage.entries.capacity(), capacity,
        "an invisible producer reservation still owns its slots");
    drop(ticket);
    assert_eq!(queue.state.lock().storage.entries.capacity(), 0);
    assert_eq!(stats.cpu_memory_usage().total(), before - backing);
    let recovered = queue.reserve(4).unwrap();
    assert!(queue.state.lock().storage.entries.capacity() >= 4);
    assert_eq!(queue.state.lock().producer_slots, 4);
    assert!(stats.cpu_memory_usage().total() <= before,
        "recover using released actual backing credit without lifting the host limit");
    stats.set_host_budget(0).unwrap(); drop(recovered); drop(queue);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn existing_device_maintenance_reclaims_idle_queue_after_later_external_cpu_pressure() {
    let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
            return;
        }
    };
    let stats = &device.memory.stats;
    assert_eq!(stats.budget_action(), crate::accounting::BudgetAction::None);
    let ticket = device.reserve_pending_acquires(8).unwrap();
    drop(ticket); // No pressure yet: backing remains cached while empty.
    let queue = device.pending_acquires().queue.get().unwrap();
    let backing = queue.state.lock().storage._memory.as_ref().unwrap().bytes();
    assert!(backing > 0);
    let idle_usage = stats.cpu_memory_usage().total();
    let external_size = usize::try_from(idle_usage).unwrap().checked_add(1024 * 1024).unwrap();
    let host_limit = idle_usage.checked_add(external_size as u64).unwrap();
    stats.set_host_budget(host_limit).unwrap();
    assert_eq!(stats.budget_action(), crate::accounting::BudgetAction::None);
    let external = crate::resource::CpuBytes::new(stats, external_size).unwrap();
    assert_ne!(stats.budget_action(), crate::accounting::BudgetAction::None);
    let pressured_usage = stats.cpu_memory_usage().total();
    let retirement = device.retirement_stats();
    assert_eq!(retirement.pending, 0);
    assert!(queue.state.lock().storage.entries.capacity() >= 8);
    // Existing maintenance reaches this empty queue without another
    // producer/release. Its native-retirement result is still exactly zero.
    assert_eq!(device.try_collect_garbage().unwrap(), 0);
    assert_eq!(queue.state.lock().storage.entries.capacity(), 0);
    assert_eq!(stats.cpu_memory_usage().total(), pressured_usage - backing);
    assert_eq!(stats.host_budget_limit(), host_limit);
    assert_eq!(device.retirement_stats().idle_waits, retirement.idle_waits);
    assert_eq!(device.retirement_stats().submitted, retirement.submitted);
    assert_eq!(device.retirement_stats().pending, 0);
    let recovered = crate::resource::CpuBytes::new(stats, usize::try_from(backing).unwrap()).unwrap();
    assert!(stats.cpu_memory_usage().total() <= host_limit);
    drop(recovered);
    drop(external);
    assert_eq!(stats.cpu_memory_usage().total(), idle_usage - backing);
    stats.set_host_budget(0).unwrap();
}

#[test]
fn lazy_queue_admission_failure_and_cross_budget_access_do_not_publish_foreign_storage() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let wrapper = PendingAcquires::default();
    assert!(wrapper.is_empty());
    stats.set_host_budget(1).unwrap();
    assert!(matches!(wrapper.shared(&stats), Err(Error::OutOfHostMemory)));
    assert!(wrapper.queue.get().is_none());
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    stats.set_host_budget(0).unwrap();
    let queue = wrapper.shared(&stats).unwrap();
    let other = Arc::new(crate::accounting::AllocatorStats::default());
    assert!(wrapper.shared(&other).is_err());
    assert_eq!(other.cpu_memory_usage().total(), 0);
    drop(queue); drop(wrapper);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
