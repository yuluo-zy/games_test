//! 性能追踪基准（不依赖第三方库）。
//!
//! 用法：
//!   cargo bench --bench perf                      # 运行并写 perf/last.json
//!   cargo bench --bench perf -- --save-baseline   # 把本次结果存为 perf/baseline.json
//!   cargo bench --bench perf -- --check           # 与基线比较，计数器回退则失败（退出码 1）
//!
//! 两层指标：
//! - `counters`（idle 等待、提交数、显存分配数等）是确定性的，作为**硬门禁**；
//! - `ns_per_op` 受机器噪声影响，只作为**参考**，超过基线 2 倍才给出警告。
//!
//! GPU 场景在无 Vulkan 设备时跳过；本机可用 `VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`。
//! lavapipe 是软件实现，GPU 场景的耗时只反映 CPU 侧驱动调用开销，不代表真实 GPU 吞吐。
use ash::vk::{self, Handle};
use rhapsody::{
    app::{PassInput, ResourceInput, Scenario, run},
    raw::{
        allocator::AllocatorStats,
        buffer::{Buffer, BufferDesc},
        device::VulkanDevice,
        init::create_headless_device,
        managed::{resources::ResourceManager, tracking::CommandState},
        setup::{init_texture_data, update_texture},
        suballoc::{AllocationMode, set_default_allocation_mode},
        shader::{ComputePipelineDesc, PipelineCache, PipelineLayout, ShaderModule, VulkanPipeline},
        texture::{Texture, TextureDesc},
    },
    rg::{
        executor::PassPgoDuration,
        executor::PassPgoNameId,
        graph::RenderGraph,
        parallel::{ChunkStrategy, ParallelRecording, TaskRunner},
        vulkan::{AccessSpec, AccessTable, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use serde_json::{Map, Value, json};
use std::{sync::Arc, time::Instant};

const BASELINE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/perf/baseline.json");
const LAST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/perf/last.json");

struct Measured {
    name: String,
    ops: u64,
    ns_per_op: u64,
    /// 确定性计数器：硬门禁，任何一项高于基线即失败。
    counters: Map<String, Value>,
    /// 只记录、不门禁的参考信息（依赖实测耗时的量，如并行块数、场景内计时）。
    info: Map<String, Value>,
}
/// 取多次运行的中位数，抑制抖动。`body` 返回该次运行完成的操作数与计数器。
fn measure(
    name: &str,
    repeats: usize,
    mut body: impl FnMut() -> (u64, Map<String, Value>),
) -> Measured {
    measure_with_info(name, repeats, || {
        let (ops, counters) = body();
        (ops, counters, Map::new())
    })
}
fn measure_with_info(
    name: &str,
    repeats: usize,
    mut body: impl FnMut() -> (u64, Map<String, Value>, Map<String, Value>),
) -> Measured {
    let mut samples = Vec::new();
    let mut last = (0, Map::new(), Map::new());
    for _ in 0..repeats {
        let start = Instant::now();
        last = body();
        samples.push(start.elapsed().as_nanos() as u64 / last.0.max(1));
    }
    samples.sort_unstable();
    Measured {
        name: name.into(),
        ops: last.0,
        ns_per_op: samples[samples.len() / 2],
        counters: last.1,
        info: last.2,
    }
}
fn counters(pairs: &[(&str, u64)]) -> Map<String, Value> {
    pairs.iter().map(|(k, v)| ((*k).into(), json!(v))).collect()
}

// ---- CPU 场景 ----
fn scenario(passes: usize) -> Scenario {
    let resources = passes / 2 + 2;
    Scenario {
        resources: (0..resources)
            .map(|i| ResourceInput {
                name: format!("r{i}"),
                kind: "buffer".into(),
                size: 256,
                imported: None,
                initial_access: 0,
            })
            .collect(),
        passes: (0..passes)
            .map(|i| PassInput {
                name: format!("p{i}"),
                accesses: vec![
                    ((i % resources) as u32, 40, 0),
                    (((i + 1) % resources) as u32, 4, 0),
                ],
            })
            .collect(),
        exports: vec![(0, 55)],
        batch_per_pass: false,
        fail_callback: None,
        fail_end: false,
        queue_error: 0,
        timeline_start: 1,
        complete: true,
    }
}
fn cpu_graph(out: &mut Vec<Measured>) {
    for passes in [32usize, 128, 512] {
        let input = scenario(passes);
        out.push(measure(&format!("cpu/graph_run/passes={passes}"), 7, || {
            run(&input).unwrap();
            (passes as u64, Map::new())
        }));
    }
}
fn cpu_allocator_accounting(out: &mut Vec<Measured>) {
    for live in [256u64, 1024, 4096] {
        out.push(measure(&format!("cpu/alloc_accounting/live={live}"), 5, || {
            let stats = AllocatorStats::default();
            let ids: Vec<u64> = (0..live)
                .map(|_| {
                    stats
                        .register(4096, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, String::new())
                        .unwrap()
                })
                .collect();
            for id in ids {
                stats.release(id).unwrap();
            }
            (live * 2, Map::new())
        }));
    }
}

/// 多线程对同一托管句柄并发 `track`：衡量控制块锁的争用（并行录制回调里绑定共享资源时会发生）。
fn cpu_managed_track_contention(out: &mut Vec<Measured>) {
    for threads in [1usize, 4, 8] {
        out.push(measure(&format!("cpu/managed_track/threads={threads}"), 5, || {
            let store = ResourceManager::new();
            let handle = store.allocate(0u8);
            let per_thread = 200_000u64;
            std::thread::scope(|scope| {
                for t in 0..threads {
                    let handle = handle.clone();
                    scope.spawn(move || {
                        for i in 0..per_thread {
                            handle
                                .track(CommandState {
                                    tag: 1,
                                    sequence: 1 + (i + t as u64) % 1000,
                                })
                                .unwrap();
                        }
                    });
                }
            });
            (per_thread * threads as u64, Map::new())
        }));
    }
}

/// 多线程按键 `get` 同一槽位：槽位表无锁读取（原作 `AppendOnlyVec` 布局）的争用基线。
fn cpu_managed_get_contention(out: &mut Vec<Measured>) {
    for threads in [1usize, 4, 8] {
        out.push(measure(&format!("cpu/managed_get/threads={threads}"), 5, || {
            let store = ResourceManager::new();
            let handle = store.allocate(0u8);
            let key = handle.key();
            let per_thread = 200_000u64;
            std::thread::scope(|scope| {
                for _ in 0..threads {
                    let store = &store;
                    scope.spawn(move || {
                        for _ in 0..per_thread {
                            assert!(store.get(key).is_some());
                        }
                    });
                }
            });
            (per_thread * threads as u64, Map::new())
        }));
    }
}

// ---- GPU 场景 ----
fn snapshot(device: &VulkanDevice) -> (u64, u64, u64) {
    let g = device.garbage_stats();
    (
        g.submit_epoch,
        g.idle_waits,
        device.allocator_stats.total_registered(),
    )
}
fn delta(device: &VulkanDevice, before: (u64, u64, u64)) -> Map<String, Value> {
    let now = snapshot(device);
    counters(&[
        ("queue_submits", now.0 - before.0),
        ("idle_waits", now.1 - before.1),
        ("memory_allocations", now.2 - before.2),
    ])
}
/// 创建 N 个互不相同键的 compute 管线，使用 `threads` 个线程；衡量管线创建的可扩展性。
fn gpu_pipelines(out: &mut Vec<Measured>, device: &Arc<VulkanDevice>) {
    const WORDS: &[u32] = &[
        0x07230203, 0x00010000, 0, 6, 0, 0x00020011, 1, 0x0003000e, 0, 1, 0x0005000f, 5, 4,
        0x6e69616d, 0, 0x00060010, 4, 17, 1, 1, 1, 0x00020013, 1, 0x00030021, 2, 1, 0x00050036, 1,
        4, 0, 2, 0x000200f8, 5, 0x000100fd, 0x00010038,
    ];
    let bytes: Vec<u8> = WORDS.iter().flat_map(|w| w.to_le_bytes()).collect();
    let count = 64usize;
    for threads in [1usize, 4, 8] {
        out.push(measure(&format!("gpu/pipeline_create/threads={threads}"), 3, || {
            let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
            let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
            let shaders: Vec<_> = (0..count)
                .map(|_| ShaderModule::from_spirv(device.clone(), &bytes).unwrap())
                .collect();
            let before = snapshot(device);
            std::thread::scope(|scope| {
                for part in shaders.chunks(count.div_ceil(threads)) {
                    let (cache, layout) = (cache.clone(), layout.clone());
                    scope.spawn(move || {
                        for shader in part {
                            VulkanPipeline::create_compute_pipeline(
                                &cache,
                                ComputePipelineDesc::new(shader.clone(), layout.clone()),
                            )
                            .unwrap();
                        }
                    });
                }
            });
            (count as u64, delta(device, before))
        }));
    }
}
/// 专用与子分配两种模式创建同一批缓冲：`memory_blocks`（真实 vkAllocateMemory 存活数）是确定性计数器。
fn gpu_allocation_modes(out: &mut Vec<Measured>) {
    for (label, mode) in [
        ("dedicated", AllocationMode::Dedicated),
        ("suballocated", AllocationMode::Suballocated),
    ] {
        set_default_allocation_mode(mode);
        let Ok(device) = create_headless_device() else {
            continue;
        };
        let n = 512u64;
        out.push(measure(&format!("gpu/alloc_mode/{label}/buffers={n}"), 3, || {
            let before = snapshot(&device);
            let buffers: Vec<_> = (0..n)
                .map(|_| {
                    Buffer::new(
                        device.clone(),
                        &BufferDesc::new_gpu_only(4096, vk::BufferUsageFlags::STORAGE_BUFFER),
                    )
                    .unwrap()
                })
                .collect();
            let mut counters = delta(&device, before);
            counters.insert("memory_blocks".into(), json!(device.memory_block_count()));
            drop(buffers);
            (n, counters)
        }));
    }
    set_default_allocation_mode(AllocationMode::Dedicated);
}
fn gpu_resources(out: &mut Vec<Measured>, device: &Arc<VulkanDevice>) {
    let n = 256u64;
    out.push(measure(&format!("gpu/buffer_create_drop/n={n}"), 3, || {
        let before = snapshot(device);
        let buffers: Vec<_> = (0..n)
            .map(|_| {
                Buffer::new(
                    device.clone(),
                    &BufferDesc::new_gpu_only(4096, vk::BufferUsageFlags::STORAGE_BUFFER),
                )
                .unwrap()
            })
            .collect();
        drop(buffers);
        (n, delta(device, before))
    }));
    let n = 128u64;
    out.push(measure(&format!("gpu/texture_create_drop/n={n}"), 3, || {
        let before = snapshot(device);
        let textures: Vec<_> = (0..n)
            .map(|_| Texture::new(device.clone(), &TextureDesc::new_2d(64, 64, vk::Format::R8G8B8A8_UNORM)).unwrap())
            .collect();
        drop(textures);
        (n, delta(device, before))
    }));
    let n = 16u64;
    out.push(measure(&format!("gpu/texture_upload/n={n}"), 3, || {
        let mut desc = TextureDesc::new_2d(64, 64, vk::Format::R8G8B8A8_UNORM);
        desc.usage = vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
        let texture = Texture::new(device.clone(), &desc).unwrap();
        let bytes = vec![0x5au8; 64 * 64 * 4];
        let before = snapshot(device);
        init_texture_data(device, &texture, &bytes).unwrap();
        for _ in 1..n {
            update_texture(device, &texture, &bytes).unwrap();
        }
        (n, delta(device, before))
    }));
}
fn frame_graph(
    session: &mut VulkanGraphSession,
    passes: usize,
) -> rhapsody::rg::graph::BuiltGraph {
    let mut graph: RenderGraph = session.begin_frame().unwrap();
    let resource = session
        .backend
        .declare(
            &mut graph,
            "frame",
            ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::TRANSFER_DST)),
        )
        .unwrap();
    for i in 0..passes {
        let device = session.backend.device.clone();
        let resolver = session.backend.resolver.clone();
        let mut pass = graph.add_pass(format!("fill{i}"));
        pass.add_resource_access(resource, 1, 0).unwrap();
        pass.render(move |handle| {
            unsafe {
                device.device.cmd_fill_buffer(
                    vk::CommandBuffer::from_raw(handle),
                    resolver.buffer("frame").unwrap(),
                    0,
                    64,
                    i as u32,
                )
            };
            true
        });
    }
    graph.build()
}
fn gpu_frames(out: &mut Vec<Measured>, device: &Arc<VulkanDevice>) {
    for passes in [1usize, 8] {
        let frames = 32u64;
        out.push(measure(&format!("gpu/frame_loop/passes={passes}"), 3, || {
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
            let mut session =
                VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access).unwrap());
            let before = snapshot(device);
            let mut ids = Vec::new();
            let (mut barrier_commands, mut barrier_transitions) = (0u64, 0u64);
            for _ in 0..frames {
                let built = frame_graph(&mut session, passes);
                let receipt = session.execute_frame(built).unwrap();
                barrier_commands += u64::from(receipt.stats.barrier_commands);
                barrier_transitions += u64::from(receipt.stats.barrier_transitions);
                ids.push(receipt.frame);
            }
            // 在途帧只能按序等待：最近 pending_frame_count 个帧依次排空。
            while session.pending_frame_count() > 0 {
                let front = ids[ids.len() - session.pending_frame_count()];
                session.wait_frame(front, u64::MAX).unwrap();
            }
            let mut counters = delta(device, before);
            counters.insert("barrier_commands".into(), json!(barrier_commands));
            counters.insert("barrier_transitions".into(), json!(barrier_transitions));
            (frames, counters)
        }));
    }
}

/// 并行录制场景：每个 pass 回调除 `cmd_fill_buffer` 外再空转 `spin_ns`，模拟真实 pass 的录制成本。
/// 返回 (每帧耗时 ns, 并行块数, 提交次数)。
fn recording_frames(
    device: &Arc<VulkanDevice>,
    config: ParallelRecording,
    passes: usize,
    spin_ns: u64,
    frames: u64,
) -> (u64, u32, u64) {
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
    let mut session =
        VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access).unwrap());
    session.chain.parallel = config;
    // 用上一帧耗时驱动分块：播种与真实回调相近的耗时。
    for i in 0..passes {
        session.chain.pgo.previous.record(PassPgoDuration::new(
            PassPgoNameId::from_name(&format!("fill{i}")),
            spin_ns.max(1),
        ));
    }
    let before = device.garbage_stats().submit_epoch;
    let (mut ids, mut chunks) = (Vec::new(), 0);
    let start = Instant::now();
    for _ in 0..frames {
        let mut graph: RenderGraph = session.begin_frame().unwrap();
        let resource = session
            .backend
            .declare(
                &mut graph,
                "data",
                ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
                    passes as u64 * 16,
                    vk::BufferUsageFlags::TRANSFER_DST,
                )),
            )
            .unwrap();
        for i in 0..passes {
            let device = session.backend.device.clone();
            let resolver = session.backend.resolver.clone();
            let mut pass = graph.add_pass(format!("fill{i}"));
            pass.add_resource_access(resource, 1, 0).unwrap();
            pass.render(move |handle| {
                let until = Instant::now() + std::time::Duration::from_nanos(spin_ns);
                while Instant::now() < until {
                    std::hint::spin_loop();
                }
                unsafe {
                    device.device.cmd_fill_buffer(
                        vk::CommandBuffer::from_raw(handle),
                        resolver.buffer("data").unwrap(),
                        i as u64 * 16,
                        16,
                        i as u32,
                    )
                };
                true
            });
        }
        let receipt = session.execute_frame(graph.build()).unwrap();
        chunks = receipt.stats.chunks;
        ids.push(receipt.frame);
    }
    while session.pending_frame_count() > 0 {
        let front = ids[ids.len() - session.pending_frame_count()];
        session.wait_frame(front, u64::MAX).unwrap();
    }
    let elapsed = start.elapsed().as_nanos() as u64 / frames;
    (elapsed, chunks, device.garbage_stats().submit_epoch - before)
}
fn gpu_recording(out: &mut Vec<Measured>, device: &Arc<VulkanDevice>) {
    let balanced = ChunkStrategy::Balanced {
        max_chunks: 8,
        chunk_overhead_ns: 20_000,
    };
    // 回调成本扫描：越轻，并行的调度开销越显著，用来找出并行开始划算的临界点。
    for spin in [0u64, 2_000, 10_000] {
        for (label, config) in [
            ("serial", ParallelRecording::SERIAL),
            ("original_pool", ParallelRecording::ORIGINAL),
        ] {
            out.push(measure_with_info(&format!("gpu/record_spin{spin}ns/{label}"), 3, || {
                let frames = 16;
                let (per_frame, chunks, submits) =
                    recording_frames(device, config, 64, spin, frames);
                (
                    frames,
                    counters(&[("queue_submits", submits)]),
                    counters(&[("chunks", u64::from(chunks)), ("frame_ns_inner", per_frame)]),
                )
            }));
        }
    }
    // 每 pass 约 50µs 的 CPU 录制成本（真实游戏 pass 通常更重），64 个 pass。
    for (label, config) in [
        ("serial", ParallelRecording::SERIAL),
        ("original_pool", ParallelRecording::ORIGINAL),
        (
            "balanced_pool",
            ParallelRecording {
                strategy: balanced,
                runner: TaskRunner::ComputePool,
                min_total_ns: 0,
            },
        ),
    ] {
        out.push(measure_with_info(&format!("gpu/record_heavy/{label}"), 3, || {
            let frames = 16;
            let (per_frame, chunks, submits) = recording_frames(device, config, 64, 50_000, frames);
            (
                frames,
                counters(&[("queue_submits", submits)]),
                counters(&[("chunks", u64::from(chunks)), ("frame_ns_inner", per_frame)]),
            )
        }));
    }
}
// ---- 报告与门禁 ----
fn to_json(results: &[Measured]) -> Value {
    let map: Map<String, Value> = results
        .iter()
        .map(|m| {
            (
                m.name.clone(),
                json!({"ops": m.ops, "ns_per_op": m.ns_per_op, "counters": m.counters, "info": m.info}),
            )
        })
        .collect();
    json!({"schema": 1, "results": map})
}
/// 返回是否通过：计数器（按操作数归一）高于基线即失败；耗时超 2 倍仅警告。
fn check(current: &Value, baseline: &Value) -> bool {
    let mut ok = true;
    let base = baseline["results"].as_object().unwrap();
    for (name, now) in current["results"].as_object().unwrap() {
        let Some(old) = base.get(name) else {
            println!("新增场景（无基线）：{name}");
            continue;
        };
        for (key, value) in now["counters"].as_object().unwrap() {
            let (new, old_value) = (value.as_u64().unwrap_or(0), old["counters"][key].as_u64());
            if let Some(old_value) = old_value {
                if new > old_value {
                    println!("回退 {name} {key}: {old_value} -> {new}");
                    ok = false;
                }
            }
        }
        let (t_new, t_old) = (now["ns_per_op"].as_u64().unwrap(), old["ns_per_op"].as_u64().unwrap());
        if t_old > 0 && t_new > t_old * 2 {
            println!("警告(耗时参考) {name}: {t_old}ns -> {t_new}ns");
        }
    }
    ok
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut results = Vec::new();
    cpu_graph(&mut results);
    cpu_allocator_accounting(&mut results);
    cpu_managed_track_contention(&mut results);
    cpu_managed_get_contention(&mut results);
    match create_headless_device() {
        Ok(device) => {
            gpu_allocation_modes(&mut results);
            gpu_pipelines(&mut results, &device);
            gpu_resources(&mut results, &device);
            gpu_frames(&mut results, &device);
            gpu_recording(&mut results, &device);
        }
        Err(error) => eprintln!("跳过 GPU 场景：{error:?}"),
    }
    let report = to_json(&results);
    for m in &results {
        println!(
            "{:<40} {:>12} ns/op  {} {}",
            m.name,
            m.ns_per_op,
            Value::Object(m.counters.clone()),
            Value::Object(m.info.clone())
        );
    }
    std::fs::write(LAST, serde_json::to_string_pretty(&report).unwrap()).unwrap();
    if args.iter().any(|a| a == "--save-baseline") {
        std::fs::write(BASELINE, serde_json::to_string_pretty(&report).unwrap()).unwrap();
        println!("已写入基线 {BASELINE}");
    }
    if args.iter().any(|a| a == "--check") {
        let baseline: Value = serde_json::from_str(
            &std::fs::read_to_string(BASELINE).expect("缺少 perf/baseline.json，先用 --save-baseline 生成"),
        )
        .unwrap();
        if !check(&report, &baseline) {
            std::process::exit(1);
        }
        println!("计数器门禁通过");
    }
}
