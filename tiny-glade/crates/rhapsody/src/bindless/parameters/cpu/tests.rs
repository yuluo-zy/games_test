use super::*;
fn pool(budget: Arc<AllocatorStats>) -> ParameterCpuPool {
    ParameterCpuPool::with_limits(budget, CpuPoolLimits { items: 2, total_bytes: 16384,
        cached_bytes: 8192, full: || Error::exhausted("test credit pool full") }, 1024).unwrap()
}
#[test]
fn completed_payloads_reuse_callback_credit_and_published_payloads_pin_it() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = pool(budget.clone());
    let mut cursor = ParameterCpuCursor::default();
    drop(cursor.claim(&pool, 128).unwrap());
    let usage = budget.cpu_memory_usage().total();
    for _ in 0..1000 { drop(cursor.claim(&pool, 128).unwrap()); }
    assert_eq!(budget.cpu_memory_usage().total(), usage);
    assert_eq!(pool.pool.stats().creations, 1);
    let a = cursor.claim(&pool, 1024).unwrap();
    let b = cursor.claim(&pool, 1024).unwrap();
    assert!(!Arc::ptr_eq(&a, &b));
    assert!(matches!(cursor.claim(&pool, 1024), Err(Error::Exhausted(_))));
    assert_eq!(pool.pool.trim_idle(0), 0);
    drop(a);
    assert!(pool.pool.trim_idle(0) > 0);
    drop(cursor.claim(&pool, 1024).unwrap());
    drop(cursor);
    drop(pool);
    assert!(budget.cpu_memory_usage().total() > 0, "published block must outlive its pool");
    drop(b);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
#[test]
fn failed_growth_keeps_existing_credit_and_budget_unchanged() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = pool(budget.clone());
    let mut cursor = ParameterCpuCursor::default();
    let old = cursor.claim(&pool, 1024).unwrap();
    let address = Arc::as_ptr(&old);
    drop(old);
    let usage = budget.cpu_memory_usage().total();
    budget.set_host_budget(usage).unwrap();
    assert!(matches!(cursor.claim(&pool, 2048), Err(Error::OutOfHostMemory)));
    assert_eq!(budget.cpu_memory_usage().total(), usage);
    assert_eq!(Arc::as_ptr(&cursor.claim(&pool, 1024).unwrap()), address);
    budget.set_host_budget(0).unwrap();
    drop(cursor);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
#[test]
fn pinned_credit_survives_failed_new_block_admission_and_retries() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = pool(budget.clone());
    let mut cursor = ParameterCpuCursor::default();
    let published = cursor.claim(&pool, 1024).unwrap();
    let usage = budget.cpu_memory_usage().total();
    budget.set_host_budget(usage).unwrap();
    for bytes in [1024, 1] {
        assert!(matches!(cursor.claim(&pool, bytes), Err(Error::OutOfHostMemory)));
        assert_eq!(cursor.remaining, 0);
        assert!(cursor.current.is_none());
        assert_eq!(budget.cpu_memory_usage().total(), usage);
        assert_eq!(pool.pool.stats().pinned, 1);
    }
    let address = Arc::as_ptr(&published);
    drop(published);
    budget.set_host_budget(0).unwrap();
    assert_eq!(Arc::as_ptr(&cursor.claim(&pool, 1024).unwrap()), address);
    drop(cursor);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
#[test]
fn cached_block_growth_respects_pool_capacity_without_corrupting_statistics() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = ParameterCpuPool::with_limits(budget.clone(), CpuPoolLimits {
        items: 2, total_bytes: 4096, cached_bytes: 3072,
        full: || Error::exhausted("bounded credit test"),
    }, 1024).unwrap();
    let mut cursor = ParameterCpuCursor::default();
    drop(cursor.claim(&pool, 1024).unwrap());
    drop(cursor);
    let before = pool.pool.stats();
    assert_eq!(before.cached, 1);
    let usage = budget.cpu_memory_usage().total();
    let mut cursor = ParameterCpuCursor::default();
    assert!(matches!(cursor.claim(&pool, 4096), Err(Error::Exhausted(_))));
    let rejected = pool.pool.stats();
    assert_eq!(rejected.checked_out, 1);
    assert_eq!(rejected.cached, 0);
    assert_eq!(rejected.live_bytes, before.cached_bytes);
    assert_eq!(budget.cpu_memory_usage().total(), usage);
    assert_eq!(cursor.claim(&pool, 1024).unwrap().credit_bytes, 1024);
    drop(cursor);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
