use super::*;
#[test]
fn noncoherent_ranges_use_the_physical_base_and_isolate_neighboring_atoms() {
    let range = mapped_host_subrange(1024, 512, 500, 65, 1, 64, false).unwrap();
    assert_eq!((range.memory_offset, range.memory_size), (1088, 64));
    assert_eq!((range.buffer_offset, range.buffer_size), (64, 64));
    let range = mapped_host_subrange(1024, 512, 500, 65, 1, 1, false).unwrap();
    assert_eq!((range.memory_offset, range.memory_size), (1089, 1));
    assert_eq!((range.buffer_offset, range.buffer_size), (65, 1));
    assert!(matches!(
        mapped_host_subrange(1056, 512, 500, 0, 1, 64, false),
        Err(Error::Unsupported(_))
    ));
}
#[test]
fn only_dedicated_memory_can_use_a_non_aligned_end_of_object() {
    let range = mapped_host_subrange(0, 1000, 1000, 999, 1, 256, true).unwrap();
    assert_eq!(
        (range.memory_offset, range.memory_size),
        (768, vk::WHOLE_SIZE)
    );
    assert_eq!((range.buffer_offset, range.buffer_size), (768, 232));
    assert!(mapped_host_subrange(0, 1000, 1000, 999, 1, 256, false).is_err());
    assert!(mapped_host_subrange(0, 1000, 1000, 999, 2, 256, true).is_err());
    assert!(mapped_host_subrange(u64::MAX - 1, 4, 4, 0, 1, 1, true).is_err());
}

#[test]
fn eviction_runs_under_pressure_or_when_the_heap_cannot_hold_the_request() {
    use crate::accounting::BudgetAction;
    assert!(!needs_eviction(BudgetAction::None, None, 1 << 40));
    assert!(!needs_eviction(BudgetAction::None, Some(100), 100));
    assert!(needs_eviction(BudgetAction::None, Some(99), 100));
    assert!(needs_eviction(BudgetAction::EvictCache, Some(u64::MAX), 1));
    assert!(!needs_eviction(
        BudgetAction::Backpressure,
        Some(u64::MAX),
        1
    ));
    assert!(is_memory_exhaustion(&Error::OutOfDeviceMemory));
    assert!(is_memory_exhaustion(&Error::exhausted("native objects")));
    assert!(!is_memory_exhaustion(&Error::OutOfHostMemory));
    assert!(!is_memory_exhaustion(&Error::DeviceLost));
}
