use super::*;
fn config() -> CpuArenaConfig {
    CpuArenaConfig {
        block_bytes: 1024,
        max_arenas: 2,
        max_total_bytes: 16 << 10,
        max_cached_bytes: 8 << 10,
    }
}
#[test]
fn real_capacity_is_budgeted_until_last_cpu_owner_and_never_in_gpu_records() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = CpuArenaPool::new(config(), budget.clone()).unwrap();
    let first = pool.checkout().unwrap();
    let bytes = first.backing_bytes() as u64;
    assert!(bytes >= 1024);
    let metadata = pool.stats().metadata_bytes as u64;
    assert_eq!(budget.cpu_memory_usage().live, bytes + metadata);
    assert_eq!(
        budget.device_memory_usage().total(),
        0,
        "host bytes never charge the device"
    );
    assert_eq!(budget.report().totals.total_allocated_bytes, 0);
    assert!(budget.report().allocations.is_empty());
    assert_eq!(pool.trim_idle(0), 0);
    assert_eq!(budget.cpu_memory_usage().live, bytes + metadata);
    drop(first);
    assert_eq!(budget.cpu_memory_usage().cache, bytes);
    assert_eq!(pool.trim_idle(0) as u64, bytes);
    assert_eq!(budget.cpu_memory_usage().total(), metadata);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
    assert_eq!(budget.budget_usage().total(), 0);
}
#[test]
fn fixed_chunk_backpressure_and_parallel_owner_limits_recover_after_cpu_return() {
    let pool = CpuArenaPool::new(config(), Arc::new(AllocatorStats::default())).unwrap();
    let first = pool.checkout().unwrap();
    let second = pool.checkout().unwrap();
    assert!(matches!(pool.checkout(), Err(Error::Timeout)));
    first.slice_fill(4, 123u64).unwrap();
    assert!(matches!(
        first.slice_fill(first.backing_bytes() * 2, 0u8),
        Err(Error::Exhausted(_))
    ));
    let bytes = first.backing_bytes();
    drop(first);
    let reused = pool.checkout().unwrap();
    assert_eq!(reused.backing_bytes(), bytes);
    assert_eq!(reused.slice_copy(&[7u32, 11]).unwrap(), &[7, 11]);
    drop((second, reused));
    assert_eq!(pool.stats().backing_allocations, 2);
    assert_eq!(pool.stats().checked_out, 0);
}
#[test]
fn host_oom_is_distinct_from_local_backpressure_and_failed_allocation_is_unpublished() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = CpuArenaPool::new(config(), budget.clone()).unwrap();
    let metadata = pool.stats().metadata_bytes as u64;
    budget.set_total_budget(metadata + 100).unwrap();
    assert!(matches!(pool.checkout(), Err(Error::OutOfHostMemory)));
    assert_eq!(pool.stats().live_bytes, 0);
    assert_eq!(budget.cpu_memory_usage().total(), metadata);
    assert_eq!(budget.device_memory_usage().total(), 0);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
