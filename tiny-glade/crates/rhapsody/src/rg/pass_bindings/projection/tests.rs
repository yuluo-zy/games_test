use super::*;

#[test]
fn vector_growth_charges_real_capacity_and_releases_its_replaced_backing() {
    let stats = Arc::new(AllocatorStats::default());
    let mut memory = stats.reserve_cpu_bytes(16, BudgetCategory::Live).unwrap();
    let mut bytes = 16;
    let mut growths = 0;
    let mut values = Vec::<u64>::new();
    for needed in [3, 7, 7] {
        let mut state = Reservation {
            memory: &mut memory,
            bytes: &mut bytes,
            growths: &mut growths,
            limit: 4096,
        };
        reserve(&mut values, needed, 16, &mut state).unwrap();
        assert_eq!(bytes, 16 + values.capacity() * size_of::<u64>());
        assert_eq!(stats.cpu_memory_usage().total(), bytes as u64);
        values.resize(needed, 9);
    }
    assert_eq!(growths, 2);
    assert!(values.iter().all(|value| *value == 9));
    drop(values);
    drop(memory);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn rejected_vector_growth_preserves_existing_storage_and_global_accounting() {
    let stats = Arc::new(AllocatorStats::default());
    let mut memory = stats.reserve_cpu_bytes(16, BudgetCategory::Cache).unwrap();
    let mut bytes = 16;
    let mut growths = 0;
    let mut values = Vec::<u64>::new();
    let mut state = Reservation {
        memory: &mut memory,
        bytes: &mut bytes,
        growths: &mut growths,
        limit: 4096,
    };
    reserve(&mut values, 2, 16, &mut state).unwrap();
    values.extend([11, 12]);
    let pointer = values.as_ptr();
    let old_usage = stats.cpu_memory_usage();
    stats.set_total_budget(old_usage.total() + 1).unwrap();
    assert!(matches!(
        reserve(&mut values, 8, 16, &mut state),
        Err(Error::OutOfHostMemory)
    ));
    assert_eq!(values, [11, 12]);
    assert_eq!(values.as_ptr(), pointer);
    assert_eq!(stats.cpu_memory_usage(), old_usage);
    assert_eq!(*state.growths, 1);
    assert!(matches!(
        reserve(&mut values, 17, 16, &mut state),
        Err(Error::Timeout)
    ));
    assert_eq!(stats.cpu_memory_usage(), old_usage);
}
