use super::*;
#[test]
fn manager_creation_rejects_cpu_budget_exhaustion_without_a_charge() {
    let stats = Arc::new(AllocatorStats::default());
    stats.set_total_budget(1).unwrap();
    assert!(matches!(
        ResourceManager::<u32>::new(stats.clone()),
        Err(Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn metadata_charges_follow_live_inners_and_cached_slot_capacity() {
    let stats = Arc::new(AllocatorStats::default());
    let store = ResourceManager::with_capacity_limit(stats.clone(), 2).unwrap();
    let initial = stats.cpu_memory_usage();
    assert!(initial.live > 0 && initial.cache > 0);
    let first = store.allocate(1u32).unwrap();
    let warm = stats.cpu_memory_usage();
    assert_eq!(
        warm.live - initial.live,
        shared_cpu_bytes::<ManagedInner<u32>>().unwrap()
    );
    assert!(warm.cache > initial.cache);
    let clone = first.clone();
    drop(first);
    assert_eq!(stats.cpu_memory_usage(), warm);
    drop(clone);
    assert_eq!(stats.cpu_memory_usage().live, initial.live);
    assert_eq!(stats.cpu_memory_usage().cache, warm.cache);
    for _ in 0..1000 {
        drop(store.allocate(7u32).unwrap());
    }
    assert_eq!(stats.cpu_memory_usage().live, initial.live);
    assert_eq!(stats.cpu_memory_usage().cache, warm.cache);
    drop(store);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn capacity_backpressure_and_cpu_oom_do_not_publish_a_slot() {
    let stats = Arc::new(AllocatorStats::default());
    let store = ResourceManager::with_capacity_limit(stats.clone(), 1).unwrap();
    let first = store.allocate(1u32).unwrap();
    assert!(matches!(store.allocate(2), Err(Error::Exhausted(_))));
    let key = first.key();
    drop(first);
    let used = stats.cpu_memory_usage().total();
    stats.set_total_budget(used).unwrap();
    assert!(matches!(store.allocate(3), Err(Error::OutOfHostMemory)));
    assert!(store.is_empty());
    assert!(store.get(key).is_none());
    assert_eq!(stats.cpu_memory_usage().total(), used);
    stats.set_total_budget(used + 4096).unwrap();
    let replacement = store.allocate(4).unwrap();
    assert_eq!(replacement.key().index(), key.index());
    assert_ne!(replacement.key(), key);
}
#[test]
fn rejected_slot_growth_preserves_the_previous_capacity_and_credit() {
    let stats = Arc::new(AllocatorStats::default());
    let store = ResourceManager::<u32>::new(stats.clone()).unwrap();
    let initial = stats.cpu_memory_usage();
    let growth = ManagedSlots::<u32>::backing_bytes(3).unwrap() - initial.cache;
    let inner = shared_cpu_bytes::<ManagedInner<u32>>().unwrap();
    stats
        .set_total_budget(initial.total() + inner + growth - 1)
        .unwrap();
    assert!(matches!(store.allocate(1), Err(Error::OutOfHostMemory)));
    assert!(store.is_empty());
    assert_eq!(store.state.slots.lock().slots.capacity(), 0);
    assert_eq!(stats.cpu_memory_usage(), initial);
}
#[test]
fn rejected_allocation_drops_payload_after_releasing_the_store_lock() {
    struct Reenter {
        store: ResourceManager<Self>,
        dropped: Arc<AtomicU64>,
    }
    impl Drop for Reenter {
        fn drop(&mut self) {
            let _ = self.store.len();
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }
    let store =
        ResourceManager::with_capacity_limit(Arc::new(AllocatorStats::default()), 1).unwrap();
    let dropped = Arc::new(AtomicU64::new(0));
    let create = || Reenter {
        store: store.clone(),
        dropped: dropped.clone(),
    };
    let first = store.allocate(create()).unwrap();
    assert!(matches!(store.allocate(create()), Err(Error::Exhausted(_))));
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
    drop(first);
    assert_eq!(dropped.load(Ordering::Relaxed), 2);
}
#[test]
fn typed_keys_reject_another_store_even_when_slot_bits_match() {
    let a = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
    let b = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
    let first = a.allocate(7u32).unwrap();
    let second = b.allocate(8u32).unwrap();
    assert_eq!(first.key().handle(), second.key().handle());
    assert_ne!(first.key(), second.key());
    assert!(a.get(second.key()).is_none());
    assert!(b.get(first.key()).is_none());
    assert_eq!(*a.clone().get(first.key()).unwrap(), 7);
}
#[test]
fn only_last_arc_release_invalidates_the_generation() {
    let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
    let first = store.allocate(3u32).unwrap();
    let key = first.key();
    let second = first.clone();
    drop(first);
    assert_eq!(*store.get(key).unwrap(), 3);
    drop(second);
    assert!(store.is_empty());
    assert!(store.get(key).is_none());
    let next = store.allocate(4u32).unwrap();
    assert_eq!(next.key().index(), key.index());
    assert_ne!(next.key().handle(), key.handle());
    assert!(store.get(key).is_none());
}
#[test]
fn typed_key_retrieval_keeps_the_raw_resource_alive_until_all_owners_leave() {
    let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
    let value = Arc::new(11u32);
    let weak = Arc::downgrade(&value);
    let first = store.allocate_shared(value).unwrap();
    let second = store.get(first.key()).unwrap();
    let raw = second.shared();
    let key = first.key();
    drop(first);
    drop(second);
    assert!(store.get(key).is_none());
    assert_eq!(*raw, 11);
    drop(raw);
    assert!(weak.upgrade().is_none());
}
#[test]
fn a_thousand_resource_batches_reuse_bounded_slot_storage() {
    let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
    let warm: Vec<_> = (0..8u32)
        .map(|value| store.allocate(value).unwrap())
        .collect();
    drop(warm);
    let capacity = store.state.slots.lock().slots.capacity();
    for _ in 0..1000 {
        let batch: Vec<_> = (0..8u32)
            .map(|value| store.allocate(value).unwrap())
            .collect();
        let keys: Vec<_> = batch.iter().map(Managed::key).collect();
        drop(batch);
        assert!(store.is_empty());
        for key in keys {
            assert!(store.get(key).is_none());
        }
        assert_eq!(store.state.slots.lock().slots.capacity(), capacity);
    }
}
#[test]
fn store_removal_does_not_hold_the_lock_during_resource_drop() {
    struct Reenter {
        store: ResourceManager<Self>,
        dropped: Arc<AtomicU64>,
    }
    impl Drop for Reenter {
        fn drop(&mut self) {
            assert!(self.store.is_empty());
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }
    let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
    let dropped = Arc::new(AtomicU64::new(0));
    let value = store
        .allocate(Reenter {
            store: store.clone(),
            dropped: dropped.clone(),
        })
        .unwrap();
    drop(value);
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
}
