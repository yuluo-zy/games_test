//! Native-reservation budget checks exercise the production gpu-allocator path.
#![cfg(feature = "suballocation")]
mod gpu_test_support;
use ash::vk;
use rhapsody::device::{AllocationStrategy, Device, DeviceConfig};
use rhapsody::device::allocator::MemoryBlock;
use std::sync::Arc;
const MIB: u64 = 1024 * 1024;
fn device(budget: u64) -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "native suballocation budget",
        Device::new_headless(&DeviceConfig {
            memory_budget_bytes: budget,
            allocation: AllocationStrategy::Suballocated,
            ..DeviceConfig::headless()
        }),
    )
}
fn allocate(device: &Arc<Device>, size: u64) -> rhapsody::Result<MemoryBlock> {
    let bits = u32::MAX >> (32 - device.memory_properties().memory_type_count);
    MemoryBlock::allocate_with_flags(
        device.clone(),
        vk::MemoryRequirements {
            size,
            alignment: 256,
            memory_type_bits: bits,
        },
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        vk::MemoryAllocateFlags::empty(),
    )
}
/// GPU phases exclude the independently charged host metadata; both remain
/// admitted by the same total device budget.
fn gpu_usage(device: &Device) -> rhapsody::accounting::BudgetUsage {
    let mut usage = device.allocator_stats().budget_usage();
    let cpu = device.allocator_stats().cpu_memory_usage();
    usage.live -= cpu.live;
    usage.cache -= cpu.cache;
    assert_eq!(
        device.allocator_stats().budget_usage().total(),
        usage.total() + cpu.total()
    );
    usage
}

#[test]
fn native_capacity_is_shared_charged_trimmed_and_has_no_device_back_edge() {
    let Some(device) = device(1024 * MIB) else {
        return;
    };
    let allocator = device.suballocator().unwrap().clone();
    let weak_device = Arc::downgrade(&device);
    let a = allocate(&device, MIB).unwrap();
    let b = allocate(&device, MIB).unwrap();
    assert!(a.is_suballocated() && b.is_suballocated());
    assert_eq!(
        a.handle, b.handle,
        "small ranges reuse the actual native allocation"
    );
    assert!(a.offset + a.size <= b.offset || b.offset + b.size <= a.offset);
    let native = device.memory_usage_snapshot();
    assert_eq!(native.native_memory_blocks, 1);
    assert_eq!(native.suballocated_used_bytes, a.size + b.size);
    assert!(native.suballocator_unused_bytes > 0);
    assert_eq!(
        gpu_usage(&device).total(),
        native.application_vk_memory_bytes
    );
    assert_eq!(
        device.allocator_stats().suballocator_spare_bytes(),
        native.suballocator_unused_bytes
    );
    drop(a);
    assert_eq!(
        device.trim_unused_suballocations(),
        0,
        "a live range pins its memory-type shard"
    );
    drop(b);
    let cached = device.memory_usage_snapshot();
    assert_eq!(cached.resource_range_bytes, 0);
    assert_eq!(gpu_usage(&device).cache, cached.suballocator_reserved_bytes);
    assert_eq!(
        device.trim_unused_suballocations(),
        cached.suballocator_reserved_bytes
    );
    assert_eq!(
        device.memory_usage_snapshot().application_vk_memory_bytes,
        0
    );
    assert_eq!(gpu_usage(&device).total(), 0);
    drop(device);
    assert!(
        weak_device.upgrade().is_none(),
        "allocator ownership must not retain Device"
    );
    assert_eq!(allocator.block_count(), 0);
}
#[test]
fn rejected_native_growth_rolls_back_the_real_range_and_native_block() {
    let Some(device) = device(1024 * MIB) else {
        return;
    };
    let probe = allocate(&device, MIB).unwrap();
    let capacity = device.memory_usage_snapshot().suballocator_reserved_bytes;
    drop(probe);
    let a = allocate(&device, capacity / 4).unwrap();
    let b = allocate(&device, capacity / 4).unwrap();
    let c = allocate(&device, capacity / 4).unwrap();
    let d = allocate(&device, capacity / 4).unwrap();
    device
        .allocator_stats()
        .set_total_budget(capacity + device.allocator_stats().cpu_memory_usage().total())
        .unwrap();
    drop(b);
    drop(d);
    // Aggregate free capacity is sufficient, but neither hole is contiguous
    // enough. A dependency-created additional block must be fully rolled back.
    let before = device.memory_usage_snapshot();
    let usage = gpu_usage(&device);
    let records = device.allocator_stats().report();
    let error = allocate(&device, capacity * 3 / 8)
        .err()
        .expect("native growth must exceed the admitted budget");
    assert_eq!(error.kind(), rhapsody::ErrorKind::OutOfMemory);
    let after = device.memory_usage_snapshot();
    assert_eq!(
        after.application_vk_memory_bytes,
        before.application_vk_memory_bytes
    );
    assert_eq!(after.native_memory_blocks, before.native_memory_blocks);
    assert_eq!(gpu_usage(&device), usage);
    assert_eq!(device.allocator_stats().report(), records);
    assert!(
        after.suballocator_peak_reserved_bytes > before.suballocator_reserved_bytes,
        "the dependency exposes native delta after allocation; the observed peak remains visible"
    );
    drop(a);
    drop(c);
    device.trim_unused_suballocations();
    assert_eq!(
        device.memory_usage_snapshot().application_vk_memory_bytes,
        0
    );
    assert_eq!(gpu_usage(&device).total(), 0);
}
#[test]
fn budgets_below_the_dependency_minimum_use_the_existing_dedicated_fallback() {
    let Some(device) = device(2 * MIB) else {
        return;
    };
    let allocation = allocate(&device, MIB).unwrap();
    assert!(!allocation.is_suballocated());
    let native = device.memory_usage_snapshot();
    assert_eq!(native.suballocator_reserved_bytes, 0);
    assert_eq!(native.application_vk_memory_bytes, allocation.size);
    drop(allocation);
    assert_eq!(gpu_usage(&device).total(), 0);
}

#[test]
fn heap_budget_below_a_native_block_uses_an_exact_dedicated_allocation() {
    let Some(device) = device(1024 * MIB) else {
        return;
    };
    let bits = u32::MAX >> (32 - device.memory_properties().memory_type_count);
    let memory_type = rhapsody::device::allocator::choose_memory_type(
        device.memory_properties(),
        bits,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .unwrap();
    let heap = device.memory_properties().memory_types[memory_type as usize].heap_index;
    device.allocator_stats().set_heap_budget(heap, MIB).unwrap();
    // A 256 MiB native block cannot fit the heap budget; the request itself can.
    let allocation = allocate(&device, MIB).unwrap();
    assert!(!allocation.is_suballocated());
    let native = device.memory_usage_snapshot();
    assert_eq!(native.suballocator_reserved_bytes, 0);
    assert_eq!(native.native_memory_blocks, 1, "the one dedicated object");
    assert_eq!(native.application_vk_memory_bytes, allocation.size);
    let error = allocate(&device, MIB)
        .err()
        .expect("the heap budget is exhausted");
    assert_eq!(error.kind(), rhapsody::ErrorKind::OutOfMemory);
    assert_eq!(device.allocator_stats().report().allocations.len(), 1);
    drop(allocation);
    assert_eq!(gpu_usage(&device).total(), 0);
}
