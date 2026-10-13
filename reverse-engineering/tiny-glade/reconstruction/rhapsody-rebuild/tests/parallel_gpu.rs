//! 并行录制的真实 Vulkan 验证：同一张多 pass 图，串行与并行（Inline、真实任务池）的 GPU 读回
//! 必须逐字节一致，并确认并行路径确实被走到。无 Vulkan 设备时跳过；本机可用 lavapipe。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{buffer::BufferDesc, init::create_headless_device},
    rg::{
        executor::PassPgoDuration,
        executor::PassPgoNameId,
        graph::RenderGraph,
        parallel::{ChunkStrategy, ParallelRecording, TaskRunner},
        vulkan::{
            AccessSpec, AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend,
            VulkanGraphSession,
        },
    },
};

const PASSES: usize = 32;
const WORDS_PER_PASS: u64 = 4;

fn access_table() -> AccessTable {
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
    access.0.insert(
        55,
        AccessSpec {
            original_mask: 0x800,
            write: false,
            stages: vk::PipelineStageFlags::HOST,
            access: vk::AccessFlags::HOST_READ,
            layout: vk::ImageLayout::GENERAL,
        },
    );
    access
}

/// 运行一帧：PASSES 个 pass 依次写同一缓冲的互不重叠区间（写后写由屏障保证顺序）。
/// 返回 (读回数据, 提交次数, 本帧并行块数)。
fn run_frame(config: ParallelRecording, seed: Option<u64>) -> Option<(Vec<u8>, u32, u32)> {
    let device = create_headless_device().ok()?;
    let mut session = VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access_table()).unwrap());
    session.chain.parallel = config;
    if let Some(cost) = seed {
        for i in 0..PASSES {
            session.chain.pgo.previous.record(PassPgoDuration::new(
                PassPgoNameId::from_name(&format!("fill{i}")),
                cost,
            ));
        }
    }
    let size = PASSES as u64 * WORDS_PER_PASS * 4;
    let mut graph: RenderGraph = session.begin_frame().unwrap();
    let resource = session
        .backend
        .declare(
            &mut graph,
            "data",
            ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(size, vk::BufferUsageFlags::TRANSFER_DST)),
        )
        .unwrap();
    for i in 0..PASSES {
        let (dev, resolver) = (session.backend.device.clone(), session.backend.resolver.clone());
        let mut pass = graph.add_pass(format!("fill{i}"));
        pass.add_resource_access(resource, 1, 0).unwrap();
        pass.render(move |handle| {
            unsafe {
                dev.device.cmd_fill_buffer(
                    vk::CommandBuffer::from_raw(handle),
                    resolver.buffer("data").unwrap(),
                    i as u64 * WORDS_PER_PASS * 4,
                    WORDS_PER_PASS * 4,
                    0x1000_0000 + i as u32,
                )
            };
            true
        });
    }
    graph.export(resource, 55).unwrap();
    let receipt = session.execute_frame(graph.build()).unwrap();
    let chunks = receipt.stats.chunks;
    session.wait_frame(receipt.frame, u64::MAX).unwrap();
    let handle = session.backend.resolver.buffer("data").unwrap();
    let buffer = session
        .backend
        .resources
        .values()
        .find_map(|r| match r {
            GpuResource::Buffer(b) if b.handle == handle => Some(b.clone()),
            _ => None,
        })
        .unwrap();
    let mut out = vec![0u8; size as usize];
    buffer.read_bytes(0, &mut out).unwrap();
    let submits = device.garbage_stats().submit_epoch as u32;
    Some((out, submits, chunks))
}

fn expected() -> Vec<u8> {
    (0..PASSES)
        .flat_map(|i| std::iter::repeat_n((0x1000_0000 + i as u32).to_le_bytes(), WORDS_PER_PASS as usize).flatten())
        .collect()
}

#[test]
fn parallel_recording_reads_back_exactly_like_serial() {
    let Some((serial, serial_submits, serial_chunks)) = run_frame(ParallelRecording::SERIAL, None) else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    assert_eq!(serial, expected(), "串行基线本身必须正确");
    assert_eq!(serial_chunks, 0, "串行配置不应分块");
    let balanced = ChunkStrategy::Balanced { max_chunks: 4, chunk_overhead_ns: 0 };
    for config in [
        ParallelRecording { strategy: balanced, runner: TaskRunner::Inline, min_total_ns: 0 },
        ParallelRecording { strategy: balanced, runner: TaskRunner::ComputePool, min_total_ns: 0 },
        ParallelRecording::ORIGINAL,
    ] {
        for seed in [None, Some(10)] {
            let (data, submits, chunks) = run_frame(config, seed).unwrap();
            assert_eq!(data, serial, "{config:?} seed={seed:?}");
            // 必须真的走了并行：32 个 pass 被切成多块（原作第一帧每 pass 一块，其余按预算）。
            assert!(chunks >= 2, "{config:?} seed={seed:?} 没有走并行路径 chunks={chunks}");
            // 并行只改变录制，不改变提交形态：每帧仍是一次 vkQueueSubmit。
            assert_eq!(submits, serial_submits, "{config:?}");
        }
    }
}

#[test]
fn parallel_recording_is_stable_over_many_frames_on_the_real_task_pool() {
    let Some(device) = create_headless_device().ok() else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    let mut session = VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access_table()).unwrap());
    session.chain.parallel = ParallelRecording::ORIGINAL;
    let size = PASSES as u64 * WORDS_PER_PASS * 4;
    let mut ids = Vec::new();
    for frame in 0..24u32 {
        let mut graph: RenderGraph = session.begin_frame().unwrap();
        let resource = session
            .backend
            .declare(
                &mut graph,
                "data",
                ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(size, vk::BufferUsageFlags::TRANSFER_DST)),
            )
            .unwrap();
        for i in 0..PASSES {
            let (dev, resolver) = (session.backend.device.clone(), session.backend.resolver.clone());
            let mut pass = graph.add_pass(format!("fill{i}"));
            pass.add_resource_access(resource, 1, 0).unwrap();
            pass.render(move |handle| {
                unsafe {
                    dev.device.cmd_fill_buffer(
                        vk::CommandBuffer::from_raw(handle),
                        resolver.buffer("data").unwrap(),
                        i as u64 * WORDS_PER_PASS * 4,
                        WORDS_PER_PASS * 4,
                        frame * 1000 + i as u32,
                    )
                };
                true
            });
        }
        graph.export(resource, 55).unwrap();
        ids.push(session.execute_frame(graph.build()).unwrap().frame);
    }
    while session.pending_frame_count() > 0 {
        let front = ids[ids.len() - session.pending_frame_count()];
        session.wait_frame(front, u64::MAX).unwrap();
    }
    // 最后一帧的内容必须是该帧写入的值（帧间命令池复用没有串扰）。
    let handle = session.backend.resolver.buffer("data").unwrap();
    let buffer = session
        .backend
        .resources
        .values()
        .find_map(|r| match r {
            GpuResource::Buffer(b) if b.handle == handle => Some(b.clone()),
            _ => None,
        })
        .unwrap();
    let mut out = vec![0u8; size as usize];
    buffer.read_bytes(0, &mut out).unwrap();
    for i in 0..PASSES {
        let at = i * WORDS_PER_PASS as usize * 4;
        for w in 0..WORDS_PER_PASS as usize {
            let word = u32::from_le_bytes(out[at + w * 4..at + w * 4 + 4].try_into().unwrap());
            assert_eq!(word, 23 * 1000 + i as u32, "pass {i} word {w}");
        }
    }
}

#[test]
fn a_panicking_callback_poisons_the_session_instead_of_continuing_silently() {
    let Some(device) = create_headless_device().ok() else {
        eprintln!("跳过：无可用 Vulkan 设备");
        return;
    };
    for config in [ParallelRecording::SERIAL, ParallelRecording::ORIGINAL] {
        let mut session =
            VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access_table()).unwrap());
        session.chain.parallel = config;
        for i in 0..PASSES {
            session.chain.pgo.previous.record(PassPgoDuration::new(
                PassPgoNameId::from_name(&format!("fill{i}")),
                10,
            ));
        }
        let mut graph: RenderGraph = session.begin_frame().unwrap();
        let resource = session
            .backend
            .declare(
                &mut graph,
                "data",
                ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
                    PASSES as u64 * 16,
                    vk::BufferUsageFlags::TRANSFER_DST,
                )),
            )
            .unwrap();
        for i in 0..PASSES {
            let mut pass = graph.add_pass(format!("fill{i}"));
            pass.add_resource_access(resource, 1, 0).unwrap();
            pass.render(move |_| {
                if i == 7 {
                    panic!("故意 panic");
                }
                true
            });
        }
        let built = graph.build();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            session.execute_frame(built)
        }));
        assert!(result.is_err(), "{config:?}: panic 必须传播");
        assert!(session.is_poisoned(), "{config:?}: panic 后会话必须为 poisoned");
        assert!(session.begin_frame().is_err(), "{config:?}: poisoned 会话必须拒绝新帧");
    }
}
