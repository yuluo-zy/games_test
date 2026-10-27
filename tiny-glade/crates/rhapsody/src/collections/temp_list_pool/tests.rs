use super::*;
fn config() -> TempListPoolConfig {
    TempListPoolConfig {
        max_lists: 2,
        max_items: 64,
        max_total_bytes: 16 << 10,
        max_cached_bytes: 8 << 10,
    }
}
#[test]
fn returning_elements_can_reenter_the_pool_and_panics_release_the_checkout() {
    struct Reenter(TempListPool<Reenter>);
    impl Drop for Reenter {
        fn drop(&mut self) { drop(self.0.checkout().unwrap()); }
    }
    let budget = Arc::new(AllocatorStats::default());
    let pool = TempListPool::new(config(), budget.clone()).unwrap();
    let mut lease = pool.checkout().unwrap();
    lease.add(Reenter(pool.clone())).unwrap();
    drop(lease);
    assert_eq!(pool.statistics().checked_out, 0);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);

    struct Panics;
    impl Drop for Panics { fn drop(&mut self) { panic!("element drop"); } }
    let pool = TempListPool::new(config(), budget.clone()).unwrap();
    let mut lease = pool.checkout().unwrap();
    lease.add(Panics).unwrap();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(lease))).is_err());
    assert_eq!(pool.statistics().checked_out, 0);
    assert_eq!(pool.statistics().bytes, 0);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
#[test]
fn zero_sized_values_still_obey_the_item_limit() {
    let pool = TempListPool::<()>::new(config(), Arc::new(AllocatorStats::default())).unwrap();
    let mut lease = pool.checkout().unwrap();
    for _ in 0..64 { lease.add(()).unwrap(); }
    assert!(matches!(lease.add(()), Err(Error::Exhausted(_))));
    assert_eq!(lease.as_slice().len(), 64);
}
#[test]
fn destructible_owner_leases_reuse_capacity_and_budget_until_last_cpu_drop() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = TempListPool::<Arc<()>>::new(config(), budget.clone()).unwrap();
    let owner = Arc::new(());
    let mut first = pool.checkout().unwrap();
    first.reserve(16).unwrap();
    first.add(owner.clone()).unwrap();
    let pointer = first.as_slice().as_ptr();
    assert_eq!(Arc::strong_count(&owner), 2);
    let second = pool.checkout().unwrap();
    assert!(matches!(pool.checkout(), Err(Error::Exhausted(_))));
    drop(first);
    assert_eq!(Arc::strong_count(&owner), 1);
    for _ in 0..1000 {
        let mut list = pool.checkout().unwrap();
        list.add(owner.clone()).unwrap();
        assert_eq!(list.as_slice().as_ptr(), pointer);
        let transferred = list.drain().next().unwrap();
        drop(list);
        assert_eq!(Arc::strong_count(&owner), 2);
        drop(transferred);
    }
    assert_eq!(pool.statistics().capacity_growths, 1);
    assert!(pool.trim_idle(0) > 0);
    let metadata = pool.statistics().metadata_bytes;
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), metadata as u64);
    drop(second);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
#[test]
fn local_limits_and_growth_oom_keep_previous_values_and_credit() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = TempListPool::<u64>::new(config(), budget.clone()).unwrap();
    let mut list = pool.checkout().unwrap();
    list.add(7).unwrap();
    let before = budget.cpu_memory_usage().total();
    budget.set_total_budget(before).unwrap();
    assert!(matches!(list.reserve(32), Err(Error::OutOfHostMemory)));
    assert_eq!(list.as_slice(), [7]);
    assert_eq!(budget.cpu_memory_usage().total(), before);
    assert!(matches!(list.reserve(65), Err(Error::Exhausted(_))));
    budget.set_total_budget(0).unwrap();
    drop(list);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
