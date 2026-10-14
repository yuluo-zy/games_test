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
    assert_eq!(store.generation_at(key.index), Some(key.resource_checksum));
    assert_eq!(store.take_out_dropped_waste_checked(), Ok(1));
    assert_eq!(
        store.generation_at(key.index),
        Some(key.resource_checksum + 1)
    );
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
    assert_eq!(store.generation_at(key.index), Some(key.resource_checksum));
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

#[test]
fn clone_if_live_never_resurrects_a_handle_whose_references_reached_zero() {
    use rhapsody::raw::managed::tracking::{ControlBlock, clone_if_live, drop_reference};
    let control = ControlBlock::new(7);
    // 校验和不匹配：克隆失败且引用数恢复。
    assert!(!clone_if_live(&control, 8));
    assert_eq!(control.references.load(Ordering::SeqCst), 1);
    // 匹配：克隆成功。
    assert!(clone_if_live(&control, 7));
    assert_eq!(control.references.load(Ordering::SeqCst), 2);
    assert!(!drop_reference(&control, false).unwrap());
    // 最后一个引用释放后（进入退役），任何克隆都必须失败，引用数保持为 0。
    assert!(drop_reference(&control, false).unwrap());
    assert!(!clone_if_live(&control, 7));
    assert_eq!(control.references.load(Ordering::SeqCst), 0);
}

#[test]
fn concurrent_tracking_keeps_the_maximum_sequence_without_a_lock() {
    let store = ResourceManager::new();
    let handle = store.allocate(0u8);
    std::thread::scope(|scope| {
        for t in 0..8u64 {
            let handle = handle.clone();
            scope.spawn(move || {
                for i in 0..10_000u64 {
                    handle.track(recording(1 + (i * 8 + t) % 50_000)).unwrap();
                }
            });
        }
    });
    // 最大值 = max((i*8+t) % 50000 + 1) 的上界 50_000，且被某个线程命中。
    assert_eq!(handle.tracked_sequence(), 50_000);
    // 8 个线程各持一个克隆，已全部释放；只剩原句柄。
    assert_eq!(handle.reference_count(), 1);
}

#[test]
fn concurrent_get_and_drop_never_returns_a_disposed_handle() {
    let store = Arc::new(ResourceManager::new());
    let counters = DeviceCounters::new(0);
    let seq = counters
        .begin_sequence(rhapsody::raw::device::QueueIdentity::Primary)
        .unwrap();
    store.begin_command_bucket(seq).unwrap();
    for _ in 0..200 {
        let handle = store.allocate(1u32);
        let key = handle.key();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let (store2, barrier2) = (store.clone(), barrier.clone());
        let getter = std::thread::spawn(move || {
            barrier2.wait();
            // 要么在退役前拿到有效句柄，要么拿到 None；绝不能拿到已退役资源的句柄。
            store2.get(key).map(|m| *m.resource())
        });
        barrier.wait();
        drop(handle);
        if let Some(value) = getter.join().unwrap() {
            assert_eq!(value, 1);
        }
        store.take_out_dropped_waste_checked().unwrap();
    }
}

#[test]
fn slot_table_grows_lock_free_and_keys_stay_unique_across_threads() {
    // 8 线程各分配 2000 个资源，迫使 AppendOnlyVec 跨多个内部块增长；所有存活句柄的槽位必须互不相同，
    // 且并发 get 读到的永远是该键自己的值。
    let store = Arc::new(ResourceManager::new());
    let handles: Vec<_> = (0..8u32)
        .map(|thread| {
            let store = store.clone();
            std::thread::spawn(move || {
                (0..2000u32)
                    .map(|i| {
                        let value = thread * 10_000 + i;
                        let handle = store.allocate(value);
                        let key = handle.key();
                        // 其它线程正在增长表时读取自己的槽位。
                        assert_eq!(store.get(key).map(|m| *m.resource()), Some(value));
                        (handle, value)
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let all: Vec<_> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    let mut indices: Vec<u32> = all.iter().map(|(h, _)| h.key().index).collect();
    indices.sort_unstable();
    indices.dedup();
    assert_eq!(indices.len(), 16_000, "存活句柄的槽位不得重复");
    for (handle, value) in &all {
        assert_eq!(store.get(handle.key()).map(|m| *m.resource()), Some(*value));
        assert_eq!(
            store.generation_at(handle.key().index),
            Some(handle.key().resource_checksum)
        );
    }
    assert_eq!(
        store.generation_at(16_000),
        None,
        "越界槽位应返回 None 而不是 panic"
    );
}

#[test]
fn concurrent_allocate_drop_recycles_slots_and_rejects_stale_keys() {
    let store = Arc::new(ResourceManager::new());
    let counters = DeviceCounters::new(0);
    let seq = counters
        .begin_sequence(rhapsody::raw::device::QueueIdentity::Primary)
        .unwrap();
    store.begin_command_bucket(seq).unwrap();
    let stale: Vec<_> = (0..64u32).map(|i| store.allocate(i)).collect();
    let stale_keys: Vec<_> = stale.iter().map(|h| h.key()).collect();
    drop(stale);
    store.take_out_dropped_waste_checked().unwrap();
    // 槽位已回收复用：旧键的校验和已过期，任何线程都不能再用它拿到句柄。
    let workers: Vec<_> = (0..4)
        .map(|t| {
            let (store, keys) = (store.clone(), stale_keys.clone());
            std::thread::spawn(move || {
                for round in 0..300u32 {
                    let fresh = store.allocate(10_000 + t * 1000 + round);
                    for key in &keys {
                        assert!(store.get(*key).is_none(), "过期键不得命中复用后的槽位");
                    }
                    drop(fresh);
                }
            })
        })
        .collect();
    for w in workers {
        w.join().unwrap();
    }
    store.take_out_dropped_waste_checked().unwrap();
}
