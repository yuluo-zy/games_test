//! raw-core 命令缓冲 / 提交引擎 / 设备创建尾段 / 统计的行为测试（机器码事实见各被测函数文档）。
//! 需要 Vulkan 的用例在无设备时跳过；本机可用 lavapipe。
use ash::vk;
use rhapsody::device::stats::RenderStats;
use rhapsody::device::selection::create_headless_device;
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
fn device_creates_distinct_named_command_buffers() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    let gd = device;
    let family = gd.queue_family_index();
    let mut first = gd.create_command_buffer(family, "a").unwrap();
    let mut second = gd.create_command_buffer(family, "b").unwrap();
    assert_ne!(first.raw_handle(), second.raw_handle());
    for cb in [&mut first, &mut second] {
        cb.begin().unwrap();
        cb.end().unwrap();
    }
    gd.wait_idle().unwrap();
    drop((first, second));
    let again = gd.create_command_buffer(family, "c").unwrap();
    assert!(!again.is_recording() && !again.is_executable());
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
            &rhapsody::resource::buffer::BufferDesc::new_gpu_to_cpu(
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
