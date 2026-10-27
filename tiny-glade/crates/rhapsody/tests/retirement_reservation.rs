//! Actual allocator calls during pending Drop / busy-pool retirement, with a
//! closed GPU gate and a full host budget. This is a correctness diagnostic.
mod allocation_counter;
mod gpu_test_support;
use ash::vk;
use rhapsody::device::{Device, DeviceConfig, DeviceQueue};
use rhapsody::command::command_buffer::VulkanCommandPool;
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use rhapsody::resource::sampler::Sampler;
use rhapsody::device::semaphore::TimelineSemaphore;
use std::sync::Arc;

struct CpuOwner;
impl rhapsody::tracking::GpuUse for CpuOwner {
    fn visit_uses(&self, _: &mut rhapsody::tracking::UseVisitor<'_>) -> rhapsody::Result<()> {
        Ok(())
    }
    fn held_until_completion(&self) -> bool {
        true
    }
}

#[test]
fn pending_drop_uses_both_reserved_nodes_without_allocating_under_host_pressure() {
    let Some(device) = gpu_test_support::initialize(
        "pre-reserved retirement",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let gate = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let cpu = Arc::new(CpuOwner);
    let weak = Arc::downgrade(&cpu);
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    command.retain_opaque(buffer.clone()).unwrap();
    command.retain_object(cpu.clone()).unwrap();
    unsafe {
        device
            .raw_device()
            .cmd_fill_buffer(command.raw_handle(), buffer.raw_handle(), 0, 16, 7);
    }
    command.end().unwrap();
    let pending = command
        .submit_to(
            DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[(gate.clone(), 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    drop(buffer);
    drop(cpu);
    let used = device.allocator_stats().cpu_memory_usage().total();
    device.allocator_stats().set_host_budget(used).unwrap();
    let (_, sample) = allocation_counter::measured(|| drop(pending));
    let retained = weak.upgrade().is_some();
    let count = device.retirement_stats().pending;
    // Release the gate before asserting, even if an allocation counter fails.
    unsafe {
        device
            .raw_device()
            .signal_semaphore(
                &vk::SemaphoreSignalInfo::default()
                    .semaphore(gate.raw_handle())
                    .value(1),
            )
            .unwrap();
    }
    device.wait(&point.into(), 10_000_000_000).unwrap();
    device.allocator_stats().set_host_budget(0).unwrap();
    device.try_collect_all_completed().unwrap();
    assert_eq!(
        sample.allocations, 0,
        "pending Drop must use reserved nodes: sizes={:?}",
        sample.allocation_sizes
    );
    assert!(
        retained,
        "CPU-only owner waits for the real completion point"
    );
    assert!(
        count >= 3,
        "buffer, native command and CPU owner payload retire separately"
    );
    assert!(weak.upgrade().is_none());
    assert_eq!(device.retirement_stats().pending, 0);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

#[test]
fn a_busy_recording_pool_reuses_its_retirement_node_without_allocating_another_queue() {
    let Some(device) = gpu_test_support::initialize(
        "busy retirement",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut first = pool.allocate().unwrap();
    let mut recording = pool.allocate().unwrap();
    first.begin().unwrap();
    first.end().unwrap();
    let pending = first.submit().unwrap();
    pending.wait(10_000_000_000).unwrap();
    recording.begin().unwrap();
    drop(pending);
    let (_, sample) = allocation_counter::measured(|| device.try_collect_all_completed().unwrap());
    let retained = device.retirement_stats().pending;
    recording.end().unwrap();
    device.try_collect_all_completed().unwrap();
    assert_eq!(sample.allocations, 0);
    assert_eq!(retained, 1);
    assert_eq!(pool.pending_free_count(), 0);
    assert_eq!(device.retirement_stats().pending, 0);
}

#[test]
fn denied_creation_returns_all_host_credits_and_can_recover() {
    let Some(device) = gpu_test_support::initialize(
        "retirement admission",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let initial = device.allocator_stats().cpu_memory_usage().total();
    let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
        .extend(std::alloc::Layout::new::<rhapsody::tracking::SyncCell>())
        .unwrap();
    let sync_bytes = layout.pad_to_align().size() as u64;
    device
        .allocator_stats()
        .set_host_budget(initial + sync_bytes)
        .unwrap();
    let failure = Sampler::new(device.clone(), &vk::SamplerCreateInfo::default());
    assert!(matches!(failure, Err(rhapsody::Error::OutOfHostMemory)));
    assert_eq!(device.allocator_stats().cpu_memory_usage().total(), initial);
    device.allocator_stats().set_host_budget(0).unwrap();
    drop(Sampler::new(device.clone(), &vk::SamplerCreateInfo::default()).unwrap());
    assert_eq!(device.allocator_stats().cpu_memory_usage().total(), initial);
}

#[test]
fn released_statistics_keep_the_exact_peak_without_allocating_in_release() {
    let stats = rhapsody::accounting::AllocatorStats::default();
    let first = stats
        .register(
            64,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "first".into(),
        )
        .unwrap();
    let second = stats
        .register(
            128,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "second".into(),
        )
        .unwrap();
    let (_, sample) = allocation_counter::measured(|| stats.release(first).unwrap());
    assert_eq!(sample.allocations, 0);
    assert_eq!(stats.report().allocations.len(), 1);
    assert!(stats.release(first).is_err());
    let smaller = stats
        .register(
            1,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "smaller".into(),
        )
        .unwrap();
    let peak = stats.peak_allocator_report().unwrap();
    assert_eq!(peak.totals.total_allocated_bytes, 192);
    assert_eq!(
        peak.allocations
            .iter()
            .map(|record| record.id)
            .collect::<Vec<_>>(),
        [first, second]
    );
    stats.release(second).unwrap();
    stats.release(smaller).unwrap();
    let larger = stats
        .register(
            256,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "larger".into(),
        )
        .unwrap();
    let (_, sample) = allocation_counter::measured(|| stats.release(larger).unwrap());
    assert_eq!(sample.allocations, 0);
    assert!(stats.report().allocations.is_empty());
    let peak = stats.peak_allocator_report().unwrap();
    assert_eq!(peak.totals.total_allocated_bytes, 256);
    assert_eq!(peak.allocations.len(), 1);
    assert_eq!(peak.allocations[0].id, larger);
}

#[test]
fn unused_dedicated_buffer_drop_does_not_allocate_a_statistics_snapshot() {
    let Some(device) = gpu_test_support::initialize(
        "unused retirement",
        Device::new_headless(&DeviceConfig {
            allocation: rhapsody::device::AllocationStrategy::Dedicated,
            ..DeviceConfig::headless()
        }),
    ) else {
        return;
    };
    let buffer = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap();
    let (_, sample) = allocation_counter::measured(|| drop(buffer));
    assert_eq!(
        sample.allocations, 0,
        "unused Drop sizes={:?}",
        sample.allocation_sizes
    );
    assert!(device.allocator_stats().report().allocations.is_empty());
    assert!(
        device
            .allocator_stats()
            .peak_allocator_report()
            .unwrap()
            .totals
            .total_allocated_bytes
            >= 16
    );
}

#[test]
fn rejected_registration_and_independent_heap_peaks_preserve_peak_evidence() {
    use rhapsody::accounting::AllocatorStats;
    let stats = AllocatorStats::default();
    let first = stats
        .register(
            100,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "gpu".into(),
        )
        .unwrap();
    let second = stats
        .register(
            50,
            1,
            1,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            "host".into(),
        )
        .unwrap();
    stats.release(first).unwrap();
    stats.set_device_budget(50).unwrap();
    assert!(matches!(
        stats.register(
            1,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "denied".into()
        ),
        Err(rhapsody::Error::OutOfDeviceMemory)
    ));
    let peak = stats.peak_allocator_report().unwrap();
    assert_eq!(peak.totals.total_allocated_bytes, 150);
    assert_eq!(peak.totals.device_local_bytes, 100);
    assert_eq!(peak.totals.system_bytes, 50);
    assert_eq!(
        peak.allocations
            .iter()
            .map(|item| item.id)
            .collect::<Vec<_>>(),
        [first, second]
    );
    stats.set_device_budget(0).unwrap();
    stats.release(second).unwrap();
    let third = stats
        .register(
            120,
            1,
            1,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            "host peak".into(),
        )
        .unwrap();
    let totals = stats.peak_totals();
    assert_eq!(totals.total_allocated_bytes, 150);
    assert_eq!(totals.device_local_bytes, 100);
    assert_eq!(totals.system_bytes, 120);
    let peak = stats.peak_allocator_report().unwrap();
    assert_eq!(
        peak.totals.system_bytes, 50,
        "report stays at the total-byte peak, not independent maxima"
    );
    stats.release(third).unwrap();
}

#[test]
fn image_view_and_second_command_ticket_refusals_restore_credits_and_recover() {
    use rhapsody::resource::texture::{Texture, TextureDesc, TextureView, TextureViewDesc};
    let Some(device) = gpu_test_support::initialize(
        "image view and dual-command retirement admission",
        Device::new_headless(&DeviceConfig::headless()),
    ) else { return; };
    let stats = device.allocator_stats();
    let shared_bytes = |layout: std::alloc::Layout| -> u64 {
        std::alloc::Layout::new::<[usize; 2]>().extend(layout).unwrap().0.pad_to_align().size() as u64
    };
    let sync_bytes = shared_bytes(std::alloc::Layout::new::<rhapsody::tracking::SyncCell>());
    // A real ordinary sampler owns only SyncCell storage and one retirement
    // node. Calibrate the actual charged node capacity, not a mirrored Node
    // layout or fake native handle. This is not native-create instrumentation.
    let initial = stats.cpu_memory_usage().total();
    let sampler = Sampler::new(device.clone(), &vk::SamplerCreateInfo::default()).unwrap();
    let ticket_bytes = stats.cpu_memory_usage().total().checked_sub(initial + sync_bytes).unwrap();
    assert!(ticket_bytes > 0);
    drop(sampler);
    assert_eq!(stats.cpu_memory_usage().total(), initial);

    let desc = TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM).usage(vk::ImageUsageFlags::SAMPLED);
    stats.set_host_budget(initial + sync_bytes + shared_bytes(std::alloc::Layout::new::<Texture>())).unwrap();
    assert!(matches!(Texture::new(device.clone(), &desc), Err(rhapsody::Error::OutOfHostMemory)));
    assert_eq!(stats.cpu_memory_usage().total(), initial);
    stats.set_host_budget(0).unwrap();
    let image = Arc::new(Texture::new(device.clone(), &desc).unwrap());
    let view_desc = TextureViewDesc::new(&desc).unwrap();
    let with_image = stats.cpu_memory_usage().total();
    stats.set_host_budget(with_image + shared_bytes(std::alloc::Layout::new::<TextureView>())).unwrap();
    assert!(matches!(image.image_view(&view_desc), Err(rhapsody::Error::OutOfHostMemory)));
    assert_eq!(stats.cpu_memory_usage().total(), with_image);
    stats.set_host_budget(0).unwrap();
    drop(image.image_view(&view_desc).unwrap());
    assert_eq!(stats.cpu_memory_usage().total(), with_image);
    drop(image);

    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    // Warm the existing pool's separate reusable recording storage before
    // measuring a denied allocation. One node fits, the second cannot fit.
    drop(pool.allocate().unwrap());
    let with_pool = stats.cpu_memory_usage().total();
    let pending = device.retirement_stats().pending;
    stats.set_host_budget(with_pool + ticket_bytes).unwrap();
    assert!(matches!(pool.allocate(), Err(rhapsody::Error::OutOfHostMemory)));
    assert_eq!(stats.cpu_memory_usage().total(), with_pool);
    assert_eq!(device.retirement_stats().pending, pending);
    stats.set_host_budget(0).unwrap();
    drop(pool.allocate().unwrap());
    assert_eq!(stats.cpu_memory_usage().total(), with_pool);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}
