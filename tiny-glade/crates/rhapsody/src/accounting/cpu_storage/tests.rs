use super::*;
#[test]
fn rejected_growth_keeps_original_credit_and_drop_returns_it() {
    let stats = Arc::new(AllocatorStats::default());
    stats.set_host_budget(128).unwrap();
    let mut owner = stats.reserve_cpu_storage(128).unwrap();
    assert!(matches!(
        owner.resize(129),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(owner.bytes(), 128);
    assert_eq!(stats.cpu_memory_usage().total(), 128);
    owner.resize(64).unwrap();
    assert_eq!(stats.cpu_memory_usage().total(), 64);
    drop(owner);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn split_moves_credit_without_readmission_or_early_release() {
    let stats = Arc::new(AllocatorStats::default());
    stats.set_host_budget(128).unwrap();
    let mut stock = stats.reserve_cpu_storage(128).unwrap();
    let escaped = stock.split_off(80).unwrap();
    assert_eq!(stock.bytes(), 48);
    assert_eq!(escaped.bytes(), 80);
    assert_eq!(stats.cpu_memory_usage().total(), 128);
    assert!(stock.split_off(49).is_err());
    assert_eq!(stock.bytes(), 48);
    drop(stock);
    assert_eq!(stats.cpu_memory_usage().total(), 80);
    drop(escaped);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
