mod gpu_test_support;
use ash::vk;
use rhapsody::device::{AllocationStrategy, Device, DeviceConfig};
use rhapsody::command::command_buffer::VulkanCommandPool;
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use rhapsody::device::semaphore::TimelineSemaphore;
use std::sync::Arc;

struct Release(Arc<TimelineSemaphore>, bool);
impl Release {
    fn signal(&mut self) {
        if !self.1 {
            unsafe {
                self.0
                    .device()
                    .raw_device()
                    .signal_semaphore(
                        &vk::SemaphoreSignalInfo::default()
                            .semaphore(self.0.raw_handle())
                            .value(1),
                    )
                    .unwrap();
            }
            self.1 = true;
        }
    }
}
impl Drop for Release {
    fn drop(&mut self) {
        self.signal();
    }
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
fn builtin_sampler_cache_charges_weak_allocations_and_has_no_device_cycle() {
    let Some(device) = gpu_test_support::initialize(
        "bounded builtin sampler metadata",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    // The managed-resource store is created on first use; create it now so the baseline is
    // non-zero (a total budget of 0 means unlimited) and does not move during the test.
    device.resources().unwrap();
    let stats = device.allocator_stats().clone();
    let baseline = stats.cpu_memory_usage().total();
    stats
        .set_total_budget(stats.budget_usage().total())
        .unwrap();
    assert!(matches!(
        device.get_built_in_sampler(0),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), baseline);
    stats.set_total_budget(u64::MAX).unwrap();

    let first = device.get_built_in_sampler(0).unwrap().unwrap();
    let same = device.get_built_in_sampler(0).unwrap().unwrap();
    assert!(Arc::ptr_eq(&first, &same));
    let live = stats.cpu_memory_usage().total();
    assert!(live > baseline);
    drop((first, same));
    device.try_collect_all_completed().unwrap();
    let weak_only = stats.cpu_memory_usage().total();
    assert!(weak_only > baseline && weak_only < live);
    for _ in 0..16 {
        let sampler = device.get_built_in_sampler(0).unwrap().unwrap();
        assert_eq!(stats.cpu_memory_usage().total(), live);
        drop(sampler);
        device.try_collect_all_completed().unwrap();
        assert_eq!(stats.cpu_memory_usage().total(), weak_only);
    }
    let weak_device = Arc::downgrade(&device);
    drop(device);
    assert!(weak_device.upgrade().is_none());
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn live_in_flight_and_retiring_memory_remains_charged_until_actual_gpu_completion() {
    let limit = 1024 * 1024;
    let config = DeviceConfig {
        memory_budget_bytes: limit,
        allocation: AllocationStrategy::Dedicated,
        ..DeviceConfig::headless()
    };
    let Some(device) =
        gpu_test_support::initialize("allocation lifecycle budget", Device::new_headless(&config))
    else {
        return;
    };
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(65536, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let bytes = gpu_usage(&device).live;
    assert!(bytes >= 65536 && bytes <= limit);
    let blocker = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let mut release = Release(blocker.clone(), false);
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.retain_opaque(buffer.clone()).unwrap();
    command.begin().unwrap();
    unsafe {
        device.raw_device().cmd_fill_buffer(
            command.raw_handle(),
            buffer.raw_handle(),
            0,
            buffer.size,
            0,
        );
    }
    command.end().unwrap();
    let pending = command
        .submit_to(
            rhapsody::device::DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[(blocker, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    assert_eq!(gpu_usage(&device).in_flight, bytes);
    drop(buffer);
    drop(pending);
    let retiring = gpu_usage(&device);
    assert_eq!(retiring.retiring, bytes);
    assert_eq!(retiring.total(), bytes);
    let rejected = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(limit, vk::BufferUsageFlags::TRANSFER_DST),
    );
    assert!(matches!(rejected, Err(rhapsody::Error::OutOfDeviceMemory)));
    assert_eq!(gpu_usage(&device).total(), bytes);
    release.signal();
    device.wait(&point.into(), 10_000_000_000).unwrap();
    device.try_collect_all_completed().unwrap();
    assert_eq!(gpu_usage(&device).total(), 0);
    let retry = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(limit / 2, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap();
    drop(retry);
    device.try_collect_all_completed().unwrap();
    assert_eq!(gpu_usage(&device).total(), 0);
}

#[test]
fn managed_payload_credit_follows_an_escaping_raw_owner_and_view() {
    use rhapsody::resource::texture::{TextureDesc, TextureViewDesc};
    let Some(device) = gpu_test_support::initialize(
        "managed CPU resource metadata",
        Device::new_headless(&DeviceConfig {
            allocation: AllocationStrategy::Dedicated,
            ..DeviceConfig::headless()
        }),
    ) else {
        return;
    };
    // The managed-resource store is created on first use and then lives with the device;
    // create it before taking the baseline the test compares against.
    device.resources().unwrap();
    let initial = device.allocator_stats().cpu_memory_usage();
    let desc = TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM).sampled();
    let managed = device
        .create_texture(&desc, "CPU payload lifetime", ())
        .unwrap();
    let key = managed.key();
    let raw = managed.shared();
    let view = Arc::new(
        raw.image_view(&TextureViewDesc::new(&desc).unwrap())
            .unwrap(),
    );
    let created = device.allocator_stats().cpu_memory_usage();
    assert!(created.live > initial.live);
    drop(managed);
    assert!(device.resources().unwrap().textures.get(key).is_none());
    let escaped = device.allocator_stats().cpu_memory_usage();
    assert!(
        escaped.live > initial.live,
        "raw resource and view remain charged"
    );
    assert!(
        escaped.live < created.live,
        "only the managed inner was released"
    );
    drop(raw);
    assert_eq!(
        device.allocator_stats().cpu_memory_usage(),
        escaped,
        "the view owns the raw texture"
    );
    drop(view);
    device.try_collect_all_completed().unwrap();
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().live,
        initial.live
    );
    assert!(device.allocator_stats().cpu_memory_usage().cache >= initial.cache);
}

#[test]
fn cpu_metadata_exhaustion_rejects_resource_and_view_creation_without_native_growth() {
    use rhapsody::resource::texture::{Texture, TextureDesc, TextureViewDesc};
    let Some(device) = gpu_test_support::initialize(
        "resource metadata admission",
        Device::new_headless(&DeviceConfig {
            allocation: AllocationStrategy::Dedicated,
            ..DeviceConfig::headless()
        }),
    ) else {
        return;
    };
    let desc = TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM).sampled();
    let texture = Arc::new(Texture::new(device.clone(), &desc).unwrap());
    let before = device.allocator_stats().report();
    let cpu = device.allocator_stats().cpu_memory_usage();
    device
        .allocator_stats()
        .set_total_budget(device.allocator_stats().budget_usage().total())
        .unwrap();
    assert!(matches!(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::TRANSFER_DST)
        ),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert!(matches!(
        Texture::new(device.clone(), &desc),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert!(matches!(
        texture.image_view(&TextureViewDesc::new(&desc).unwrap()),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(device.allocator_stats().report(), before);
    assert_eq!(device.allocator_stats().cpu_memory_usage(), cpu);
}

#[test]
fn throttled_budget_refresh_skips_a_second_query_inside_the_interval() {
    let Some(device) = gpu_test_support::initialize(
        "throttled budget refresh",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let initial = device.driver_memory_budget_snapshot();
    let first = device.refresh_memory_budget_if_due().unwrap();
    let second = device.refresh_memory_budget_if_due().unwrap();
    let actual = second
        .as_ref()
        .or(first.as_ref())
        .map(rhapsody::native::physical_device::MemoryBudgetSnapshot::from_vk)
        .or(initial);
    assert_eq!(
        device.driver_memory_budget_snapshot(),
        actual,
        "cached raw telemetry must match the latest actual query, independently of allocator limits"
    );
    if device.memory_budget_enabled() {
        // Under pressure the refresh is always due; otherwise the second call is skipped.
        let pressure =
            device.allocator_stats().budget_action() != rhapsody::accounting::BudgetAction::None;
        assert!(first.is_some(), "first call queries the driver");
        assert_eq!(second.is_some(), pressure);
    } else {
        assert!(first.is_none() && second.is_none());
    }
    // The forced variant is unaffected by the interval.
    assert_eq!(
        device.refresh_memory_budget().unwrap().is_some(),
        device.memory_budget_enabled()
    );
}
