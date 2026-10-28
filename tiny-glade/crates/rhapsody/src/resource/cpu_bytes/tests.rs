use super::*;
#[test]
fn refusal_precedes_storage_and_reuse_preserves_exact_credit() {
    let stats = Arc::new(AllocatorStats::default());
    stats.set_host_budget(128).unwrap();
    assert!(matches!(
        CpuBytes::new(&stats, 129),
        Err(Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    let mut bytes = CpuBytes::new(&stats, 128).unwrap();
    let pointer = bytes.as_ptr();
    bytes.fill(0xab);
    bytes.clear();
    assert_eq!(stats.cpu_memory_usage().total(), bytes.capacity() as u64);
    bytes.reset_zeroed(128).unwrap();
    assert_eq!(pointer, bytes.as_ptr());
    assert!(bytes.iter().all(|byte| *byte == 0));
    assert!(bytes.reset_zeroed(129).is_err());
    assert_eq!(bytes.len(), 128);
    drop(bytes);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn slice_borrows_share_admitted_backing_without_changing_credit() {
    let stats = Arc::new(AllocatorStats::default());
    let mut bytes = CpuBytes::new(&stats, 16).unwrap();
    let pointer = bytes.as_ptr();
    let capacity = bytes.capacity();
    let usage = stats.cpu_memory_usage().total();
    {
        let view: &mut [u8] = bytes.as_mut_slice();
        view[0] = 0xab;
        view[1..].fill(0xcd);
    }
    let view: &[u8] = bytes.as_slice();
    assert_eq!(view.as_ptr(), pointer);
    assert_eq!(view.len(), 16);
    assert_eq!(view[0], 0xab);
    assert!(view[1..].iter().all(|byte| *byte == 0xcd));
    assert_eq!(bytes.capacity(), capacity);
    assert_eq!(bytes.accounted_bytes(), capacity as u64);
    assert_eq!(stats.cpu_memory_usage().total(), usage);
    drop(bytes);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    let mut empty = CpuBytes::default();
    assert!(empty.as_slice().is_empty());
    assert!(empty.as_mut_slice().is_empty());
}

#[test]
fn empty_storage_needs_no_credit_and_cannot_grow() {
    let mut bytes = CpuBytes::default();
    assert_eq!(bytes.accounted_bytes(), 0);
    assert!(bytes.reset_zeroed(1).is_err());
}
