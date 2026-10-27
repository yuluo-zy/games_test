use super::*;
use crate::Error;
use ash::vk;
use std::sync::Arc;
use crate::accounting::BudgetUsage;
#[test]
fn native_spare_moves_to_ranges_and_back_without_double_charging() {
    let stats = AllocatorStats::default();
    stats.set_total_budget(100).unwrap();
    let a = stats
        .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
        .unwrap();
    assert_eq!(
        stats.budget_usage(),
        BudgetUsage {
            live: 20,
            cache: 80,
            ..Default::default()
        }
    );
    let b = stats
        .register_suballocation(30, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 50)
        .unwrap();
    assert_eq!(
        stats.budget_usage(),
        BudgetUsage {
            live: 50,
            cache: 50,
            ..Default::default()
        }
    );
    stats.mark_retiring(a);
    stats.release_suballocation(a, 70).unwrap();
    assert_eq!(
        stats.budget_usage(),
        BudgetUsage {
            live: 30,
            cache: 70,
            ..Default::default()
        }
    );
    stats.release_suballocation(b, 100).unwrap();
    assert_eq!(stats.report().totals.total_allocated_bytes, 0);
    assert_eq!(stats.budget_usage().cache, 100);
    assert!(stats.set_heap_budget(0, 99).is_err());
    stats.trim_suballocator_spare(0, 0);
    assert_eq!(stats.budget_usage().total(), 0);
    stats.set_heap_budget(0, 0).unwrap();
}
#[test]
fn failed_native_growth_preserves_the_entire_previous_ledger() {
    let stats = AllocatorStats::default();
    stats.set_total_budget(100).unwrap();
    let a = stats
        .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
        .unwrap();
    let before = stats.report();
    let usage = stats.budget_usage();
    assert!(matches!(
        stats.register_suballocation(40, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 70),
        Err(Error::OutOfDeviceMemory)
    ));
    assert_eq!(stats.report(), before);
    assert_eq!(stats.budget_usage(), usage);
    assert_eq!(stats.suballocator_spare_bytes(), 80);
    assert_eq!(stats.total_registered(), 1);
    stats.release_suballocation(a, 0).unwrap();
    assert_eq!(stats.budget_usage().total(), 0);
}
#[test]
fn host_reservations_use_a_separate_budget_from_device_memory() {
    let stats = Arc::new(AllocatorStats::default());
    stats.set_total_budget(100).unwrap();
    stats.set_host_budget(30).unwrap();
    let host = stats
        .reserve_cpu_bytes(20, crate::accounting::BudgetCategory::Cache)
        .unwrap();
    // Host metadata does not consume device headroom...
    let a = stats
        .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
        .unwrap();
    assert_eq!(stats.device_memory_usage().total(), 100);
    assert_eq!(stats.budget_usage().total(), 120);
    assert_eq!(stats.cpu_memory_usage().total(), 20);
    // ...and host exhaustion is a host error, never device OOM.
    assert!(matches!(
        stats.reserve_cpu_bytes(11, crate::accounting::BudgetCategory::Live),
        Err(Error::OutOfHostMemory)
    ));
    assert!(matches!(
        stats.register(
            1,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            String::new()
        ),
        Err(Error::OutOfDeviceMemory)
    ));
    stats.release_suballocation(a, 100).unwrap();
    assert_eq!(stats.device_memory_usage().cache, 100);
    drop(host);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    stats.trim_suballocator_spare(0, 0);
    assert_eq!(stats.budget_usage().total(), 0);
}
#[test]
fn driver_heap_budgets_exclude_foreign_usage_and_raise_pressure() {
    let stats = AllocatorStats::default();
    let a = stats
        .register(
            60,
            0,
            1,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            String::new(),
        )
        .unwrap();
    // Heap 1: budget 200, process usage 100 of which 60 are ours.
    stats.apply_driver_heap_budgets(&[(1000, 0), (200, 100)]);
    assert_eq!(stats.heap_usage(1).unwrap(), (60, Some(160)));
    assert_eq!(stats.budget_action(), crate::accounting::BudgetAction::None);
    assert!(matches!(
        stats.register(
            101,
            0,
            1,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            String::new()
        ),
        Err(Error::OutOfDeviceMemory)
    ));
    let b = stats
        .register(
            80,
            0,
            1,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            String::new(),
        )
        .unwrap();
    assert_ne!(stats.budget_action(), crate::accounting::BudgetAction::None);
    stats.release(b).unwrap();
    stats.release(a).unwrap();
    assert!(stats.heap_usage(16).is_err());
}
#[test]
fn native_object_count_never_exceeds_the_device_limit() {
    let stats = AllocatorStats::default();
    stats.set_max_native_objects(2);
    stats.acquire_native_object().unwrap();
    let id = stats
        .register_native(8, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL)
        .unwrap();
    stats.acquire_native_object().unwrap();
    assert_eq!(
        stats.acquire_native_object().unwrap_err().kind(),
        crate::ErrorKind::Exhausted
    );
    stats.release_native_object();
    stats.release(id).unwrap();
    assert_eq!(stats.native_object_count(), 0);
}
