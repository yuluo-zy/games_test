//! 性能回归护栏：用确定性计数器与宽松的复杂度上界，防止已消除的停顿/二次复杂度回归。
//! 精确数值与耗时趋势见 `cargo bench --bench perf` 与 perf/baseline.json。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{allocator::AllocatorStats, buffer::BufferDesc, init::create_headless_device},
    rg::vulkan::{AccessSpec, AccessTable, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
};
use std::time::{Duration, Instant};

#[test]
fn allocator_accounting_stays_near_linear_with_live_allocations() {
    // 退化为 O(n²) 时 5 万个存活分配需要十几秒；线性实现只需毫秒级。上界取 5 秒以容忍调试构建与噪声。
    let stats = AllocatorStats::default();
    let start = Instant::now();
    let ids: Vec<u64> = (0..50_000)
        .map(|_| {
            stats
                .register(4096, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, String::new())
                .unwrap()
        })
        .collect();
    for id in ids {
        stats.release(id).unwrap();
    }
    assert!(
        start.elapsed() < Duration::from_secs(5),
        "分配统计出现二次复杂度回退：{:?}",
        start.elapsed()
    );
    assert_eq!(stats.report().allocations.len(), 0);
}

#[test]
fn steady_state_frame_loop_never_stalls_the_whole_device() {
    let Ok(device) = create_headless_device() else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    let mut access = AccessTable::default();
    access.0.insert(
        1,
        AccessSpec {
            original_mask: 0x40,
            write: true,
            stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_WRITE,
            layout: vk::ImageLayout::GENERAL,
        },
    );
    let mut session = VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access).unwrap());
    let before = device.garbage_stats();
    let mut ids = Vec::new();
    for i in 0..16u32 {
        let mut graph = session.begin_frame().unwrap();
        let resource = session
            .backend
            .declare(
                &mut graph,
                "frame",
                ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::TRANSFER_DST)),
            )
            .unwrap();
        let (dev, resolver) = (session.backend.device.clone(), session.backend.resolver.clone());
        let mut pass = graph.add_pass("fill");
        pass.add_resource_access(resource, 1, 0).unwrap();
        pass.render(move |handle| {
            unsafe {
                dev.device.cmd_fill_buffer(
                    vk::CommandBuffer::from_raw(handle),
                    resolver.buffer("frame").unwrap(),
                    0,
                    64,
                    i,
                )
            };
            true
        });
        ids.push(session.execute_frame(graph.build()).unwrap().frame);
    }
    let during = device.garbage_stats();
    // 帧节流本身只等时间线，不应触发整设备 idle（旧实现每帧 2 次 command reset idle）。
    assert_eq!(during.idle_waits - before.idle_waits, 0, "帧循环出现整设备 idle");
    assert_eq!(during.submit_epoch - before.submit_epoch, 16, "每帧应恰好一次 vkQueueSubmit");
    while session.pending_frame_count() > 0 {
        let front = ids[ids.len() - session.pending_frame_count()];
        session.wait_frame(front, u64::MAX).unwrap();
    }
}
