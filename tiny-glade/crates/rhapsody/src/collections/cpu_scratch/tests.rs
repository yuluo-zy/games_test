use super::*;
#[derive(Default)]
struct Scratch(Vec<u32>);
impl CpuScratchValue for Scratch {
    fn capacity_bytes(&self) -> usize {
        self.0.capacity() * std::mem::size_of::<u32>()
    }
    fn reset(&mut self) {
        self.0.clear();
    }
}
#[test]
fn growth_is_admitted_before_allocation_and_values_clear_on_cancel() {
    let stats = Arc::new(AllocatorStats::default());
    let pool = CpuScratchPool::<Scratch>::new(stats.clone(), 2, 1 << 20, 1 << 19).unwrap();
    let metadata = stats.cpu_memory_usage().total();
    let mut first = pool.checkout().unwrap();
    stats.set_host_budget(metadata).unwrap();
    let mut called = false;
    assert!(matches!(
        first.prepare(256, |value| {
            called = true;
            value
                .0
                .try_reserve_exact(64)
                .map_err(|_| Error::OutOfHostMemory)
        }),
        Err(Error::OutOfHostMemory)
    ));
    assert!(!called);
    assert_eq!(stats.cpu_memory_usage().total(), metadata);
    stats.set_host_budget(0).unwrap();
    first
        .prepare(256, |value| {
            value
                .0
                .try_reserve_exact(64)
                .map_err(|_| Error::OutOfHostMemory)
        })
        .unwrap();
    first.with_prepared(|value| value.0.push(7));
    drop(first);
    let second = pool.checkout().unwrap();
    assert!(second.value().0.is_empty());
    assert!(second.value().0.capacity() >= 64);
    drop(second);
    drop(pool);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn pool_capacity_refusal_and_device_budgets_stay_independent() {
    let first_stats = Arc::new(AllocatorStats::default());
    let second_stats = Arc::new(AllocatorStats::default());
    let first =
        CpuScratchPool::<Scratch>::new(first_stats.clone(), 1, 1 << 20, 1 << 19).unwrap();
    let second =
        CpuScratchPool::<Scratch>::new(second_stats.clone(), 1, 1 << 20, 1 << 19).unwrap();
    let mut held = first.checkout().unwrap();
    held.prepare(256, |value| {
        value
            .0
            .try_reserve_exact(64)
            .map_err(|_| Error::OutOfHostMemory)
    })
    .unwrap();
    assert!(matches!(first.checkout(), Err(Error::Exhausted(_))));
    assert_eq!(second.stats().live_bytes, 0);
    assert_eq!(second.stats().cached_bytes, 0);
    assert_eq!(first.stats().live_bytes, 256);
    drop(held);
    first.trim();
    assert_eq!(first.stats().live_bytes + first.stats().cached_bytes, 0);
    drop(first);
    drop(second);
    assert_eq!(first_stats.cpu_memory_usage().total(), 0);
    assert_eq!(second_stats.cpu_memory_usage().total(), 0);
}
#[test]
fn preparation_panic_reconciles_credit_before_lease_unwind() {
    let stats = Arc::new(AllocatorStats::default());
    let pool = CpuScratchPool::<Scratch>::new(stats.clone(), 1, 1 << 20, 1 << 19).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut lease = pool.checkout().unwrap();
        let _: Result<()> = lease.prepare(256, |value| {
            value
                .0
                .try_reserve_exact(64)
                .map_err(|_| Error::OutOfHostMemory)?;
            value.0.push(17);
            panic!("injected capacity preparation failure");
        });
    }));
    assert!(result.is_err());
    assert_eq!(pool.stats().checked_out, 0);
    let lease = pool.checkout().unwrap();
    assert!(lease.value().0.is_empty());
    drop(lease);
    drop(pool);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
