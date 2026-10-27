use super::*;
#[test]
fn bounded_scratch_reuses_storage_and_error_or_panic_drops_only_cpu_values() {
    let budget = Arc::new(AllocatorStats::default());
    let mut list = TempList::new(4, budget.clone()).unwrap();
    for _ in 0..1000 {
        list.with_reserved(4, |items| items.extend([1u64, 2, 3, 4]))
            .unwrap();
    }
    assert_eq!(list.growth_count(), 1);
    assert!(list.as_slice().is_empty());
    assert_eq!(
        budget.cpu_memory_usage().cache,
        list.capacity_bytes() as u64
    );
    assert!(matches!(list.reserve(5), Err(Error::Exhausted(_))));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        list.with_reserved(4, |items| {
            items.push(9);
            panic!("injected CPU failure");
        })
    }));
    assert!(result.is_err());
    assert!(list.as_slice().is_empty());
    assert!(list.trim_idle());
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
#[test]
fn growth_oom_keeps_original_values_and_accounted_capacity() {
    let budget = Arc::new(AllocatorStats::default());
    let mut list = TempList::new(32, budget.clone()).unwrap();
    list.add(7u64).unwrap();
    let bytes = list.capacity_bytes();
    budget.set_total_budget(bytes as u64).unwrap();
    assert!(matches!(list.reserve(32), Err(Error::OutOfHostMemory)));
    assert_eq!(list.as_slice(), &[7]);
    assert_eq!(list.capacity_bytes(), bytes);
    assert_eq!(budget.cpu_memory_usage().total(), bytes as u64);
}
