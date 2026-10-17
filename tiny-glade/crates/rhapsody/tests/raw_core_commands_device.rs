//! raw-core 命令缓冲 / 提交引擎 / 设备创建尾段 / 统计的行为测试（机器码事实见各被测函数文档）。
//! 需要 Vulkan 的用例在无设备时跳过；本机可用 lavapipe。
use ash::vk;
use rhapsody::raw::{init::create_headless_device, stats::RenderStats};
use std::sync::atomic::Ordering;

#[test]
fn render_stats_snapshot_is_three_atomic_xchg_and_clears() {
    let stats = RenderStats::default();
    stats.counter0.store(1, Ordering::Relaxed);
    stats.counter1.store(2, Ordering::Relaxed);
    stats.counter2.store(3, Ordering::Relaxed);
    let snap = stats.snapshot_and_reset();
    assert_eq!((snap.counter0, snap.counter1, snap.counter2), (1, 2, 3));
    let again = stats.snapshot_and_reset();
    assert_eq!((again.counter0, again.counter1, again.counter2), (0, 0, 0));
}

#[test]
fn device_does_not_own_completed_command_allocations() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    let gd = device;
    let family = gd.queue_family_index();
    assert_eq!(gd.free_command_buffers(family), 0);
    let mut first = gd.create_command_buffer(family, "a").unwrap();
    let mut second = gd.create_command_buffer(family, "b").unwrap();
    let (h1, h2) = (first.handle, second.handle);
    assert_ne!(h1, h2);
    for cb in [&mut first, &mut second] {
        cb.begin().unwrap();
        cb.end().unwrap();
    }
    // 提交并等待完成后才可回池。
    gd.wait_idle().unwrap();
    unsafe {
        gd.recycle_command_buffer(first).unwrap();
        gd.recycle_command_buffer(second).unwrap();
    }
    assert_eq!(gd.free_command_buffers(family), 0);
    let again = gd.create_command_buffer(family, "c").unwrap();
    assert_eq!(gd.free_command_buffers(family), 0);
    assert!(!again.is_recording() && !again.is_executable());
}

#[test]
fn device_micro_functions_and_creation_tail_state() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    let gd = device;
    assert_eq!(
        gd.queue_family_index_for(0).unwrap(),
        gd.queue_family_index()
    );
    // 没有 EXT_subgroup_size_control 时 Option 为 None，数值版保持旧行为 1。
    if gd.min_subgroup_size_option().is_none() {
        assert_eq!(gd.min_subgroup_size(), 1);
    }
    // refresh_memory_budget：门关/未挂接物理设备包装 → Ok(None)。
    if !gd.memory_budget_enabled() || gd.physical_device_info().is_none() {
        assert!(gd.refresh_memory_budget().unwrap().is_none());
    }
    assert_eq!(gd.device_uuid(), gd.device_uuid());
}

#[test]
fn device_is_released_after_last_resource_without_cache_cycles() {
    let Ok(device) = create_headless_device() else {
        eprintln!("SKIP: Vulkan device unavailable");
        return;
    };
    let weak_device = std::sync::Arc::downgrade(&device);
    let buffer = device
        .create_buffer(
            &rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu(
                16,
                vk::BufferUsageFlags::TRANSFER_DST,
            ),
            "lifetime",
            (),
        )
        .unwrap();
    drop(device);
    assert!(weak_device.upgrade().is_some());
    drop(buffer);
    assert!(weak_device.upgrade().is_none());
}
