//! 直接消费真实 ControlBlock 和命令桶，验证 CPU dispose→GPU destroy 的功能链。
use rhapsody::raw::{
    device::DeviceCounters,
    managed::{
        resources::ResourceManager,
        tracking::{CommandKey, CommandState, Failure},
    },
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
struct Allocation(Arc<AtomicUsize>);
impl Drop for Allocation {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
fn recording(sequence: u64) -> CommandState {
    CommandState { tag: 1, sequence }
}
fn counter() -> Arc<AtomicUsize> {
    Arc::new(AtomicUsize::new(0))
}

#[test]
fn real_counter_tracker_cpu_dispose_and_delayed_raw_destroy_use_same_sequence() {
    let store = ResourceManager::new();
    let freed = counter();
    let counters = DeviceCounters::new(0);
    let one = counters
        .begin_sequence(rhapsody::raw::device::QueueIdentity::Primary)
        .unwrap();
    let two = counters
        .begin_sequence(rhapsody::raw::device::QueueIdentity::Primary)
        .unwrap();
    assert_eq!((one, two), (1, 2));
    store.begin_command_bucket(one).unwrap();
    store.begin_command_bucket(two).unwrap();
    let handle = store.allocate(Allocation(freed.clone()));
    let key = handle.key();
    handle.track(recording(two)).unwrap();
    handle.track(recording(one)).unwrap();
    assert_eq!(handle.tracked_sequence(), two);
    let clone = handle.clone();
    assert_eq!(handle.reference_count(), 2);
    drop(handle);
    assert_eq!(store.pending_disposals(), 0);
    drop(clone);
    assert_eq!(store.pending_disposals(), 1);
    assert!(store.get(key).is_none());
    assert_eq!(store.generation_at(key.index), Some(key.generation));
    assert_eq!(store.take_out_dropped_waste_checked(), Ok(1));
    assert_eq!(store.generation_at(key.index), Some(key.generation + 1));
    assert_eq!(store.pending_destroy(), 1);
    assert_eq!(freed.load(Ordering::SeqCst), 0);
    // CPU 槽位在 GPU 未完成时已经复用；旧 raw 对象拥有权仍在 sequence2 的桶里。
    let replacement = store.allocate(Allocation(freed.clone()));
    assert_eq!(replacement.key().index, key.index);
    assert_eq!(replacement.reference_count(), 1);
    assert_eq!(replacement.tracked_sequence(), 0);
    assert!(store.get(key).is_none());
    assert_eq!(
        store.drain_completed_sequence(two),
        Err(Failure::DuplicateCompletion)
    );
    assert_eq!(store.drain_completed_sequence(one), Ok(0));
    assert_eq!(freed.load(Ordering::SeqCst), 0);
    assert_eq!(store.drain_completed_sequence(two), Ok(1));
    assert_eq!(freed.load(Ordering::SeqCst), 1);
    drop(replacement);
    assert_eq!(store.take_out_dropped_waste(), 1);
    assert_eq!(freed.load(Ordering::SeqCst), 2);
}

#[test]
fn missing_begin_retains_record_until_private_producer_is_connected() {
    let store = ResourceManager::new();
    let freed = counter();
    let handle = store.allocate(Allocation(freed.clone()));
    let key = handle.key();
    handle.track(recording(1)).unwrap();
    drop(handle);
    assert_eq!(
        store.take_out_dropped_waste_checked(),
        Err(Failure::MissingBucket)
    );
    assert_eq!(store.pending_disposals(), 1);
    assert_eq!(freed.load(Ordering::SeqCst), 0);
    assert_eq!(store.generation_at(key.index), Some(key.generation));
    store.begin_command_bucket(1).unwrap();
    assert_eq!(store.take_out_dropped_waste_checked(), Ok(1));
    assert_eq!(store.drain_completed_sequence(1), Ok(1));
    assert_eq!(freed.load(Ordering::SeqCst), 1);
}

#[test]
fn cross_domain_is_explicit_gap_and_does_not_create_a_second_tracker() {
    let store = ResourceManager::new();
    let handle = store.allocate(17_u64);
    assert_eq!(
        handle.track_on(1, recording(9)),
        Err(Failure::CrossDomainMappingUnknown)
    );
    assert_eq!(handle.tracked_sequence(), 0);
    assert_eq!(
        handle.track(CommandState {
            tag: 2,
            sequence: 9
        }),
        Err(Failure::InvalidCommandState)
    );
    handle.track(recording(2)).unwrap();
    handle.track(recording(1)).unwrap();
    assert_eq!(handle.tracked_sequence(), 2);
    let key = handle.key();
    assert!(ResourceManager::<u64>::new().get(key).is_none());
    store.begin_command_bucket(1).unwrap();
    store.begin_command_bucket(2).unwrap();
    let parsed = store.get_tracked(key, 0, recording(2)).unwrap();
    assert_eq!(parsed.reference_count(), 2);
    drop(parsed);
    drop(handle);
    store.take_out_dropped_waste();
    store.drain_completed_sequence(1).unwrap();
    store.drain_completed_sequence(2).unwrap();
}

#[test]
fn late_cpu_drop_after_actual_sequence_drain_is_immediately_destroyed() {
    let store = ResourceManager::new();
    let freed = counter();
    store.begin_command_bucket(1).unwrap();
    let handle = store.allocate(Allocation(freed.clone()));
    handle.track(recording(1)).unwrap();
    assert_eq!(store.drain_completed_sequence(1), Ok(0));
    drop(handle);
    assert_eq!(store.take_out_dropped_waste(), 1);
    assert_eq!(freed.load(Ordering::SeqCst), 1);
}

#[test]
fn failed_recording_cleanup_requires_successful_actual_idle_wait() {
    let store = ResourceManager::new();
    let freed = counter();
    let handle = store.allocate(Allocation(freed.clone()));
    handle.track(recording(9)).unwrap();
    drop(handle);
    assert_eq!(
        store.dispose_after_wait(|| Err::<(), _>("等待失败")),
        Err("等待失败")
    );
    assert_eq!(freed.load(Ordering::SeqCst), 0);
    assert_eq!(store.pending_disposals(), 1);
    assert_eq!(store.dispose_after_wait(|| Ok::<_, ()>(())), Ok(1));
    assert_eq!(freed.load(Ordering::SeqCst), 1);
}

#[test]
fn legacy_receipts_keep_timeline_separate_and_reject_domain_queue_switches() {
    let store = ResourceManager::new();
    let freed = counter();
    let handle = store.allocate(Allocation(freed.clone()));
    handle.track(recording(3)).unwrap();
    let command = CommandKey {
        domain: 0,
        sequence: 3,
    };
    assert_eq!(
        store.record_submission(4, 100, &[command, command]),
        Err(Failure::DuplicateBucket)
    );
    store.record_submission(4, 100, &[command]).unwrap();
    assert_eq!(
        store.record_submission(
            5,
            101,
            &[CommandKey {
                domain: 0,
                sequence: 4
            }]
        ),
        Err(Failure::CrossDomainMappingUnknown)
    );
    drop(handle);
    assert_eq!(store.take_out_dropped_waste(), 1);
    store.complete(4, 99).unwrap();
    assert_eq!(freed.load(Ordering::SeqCst), 0);
    store.complete(4, 100).unwrap();
    assert_eq!(freed.load(Ordering::SeqCst), 1);
    assert_eq!(store.complete(4, 99), Err(Failure::TimelineRegression));
}
