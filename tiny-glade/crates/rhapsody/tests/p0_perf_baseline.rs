//! Render framework performance baseline and the formal perf run mode.
//!
//! `render_perf_prepare`、`render_perf_strict_acceptance`、`render_perf_throughput` are manual,
//! ignored entry points. Correctness and throughput use the same raster/copy/readback scene;
//! throughput refuses to run without a matching passed acceptance report.
//!
//! 运行：`cargo test -p rhapsody --release --test p0_perf_baseline -- --ignored --nocapture`
//! 图：clear 纹理 A → fill 缓冲 B → 光栅画三角形到图内颜色目标（每帧新建并导入顶点缓冲）→
//! 颜色目标拷到 A → A 拷到 B → B 拷到常驻读回缓冲（导出 HostRead）。没有 Vulkan 设备时跳过。
#[allow(dead_code)]
mod allocation_counter;
mod draw_support;
use ash::vk::{self, Handle};
use draw_support::{Rig, SIZE, frame_clear_color, reference_coverage, rotated_triangle};
use parking_lot::Mutex;
use rhapsody::{
    device::{Device as CoreDevice, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        texture::TextureDesc,
    },
    rg::{
        attachments::{Attachment, AttachmentCache},
        executor::PassContext,
        graph::{BuiltGraph, RenderGraph},
        pass_api::{AshCommandBackend, RenderPassApi},
        profiling::{GpuCommandTiming, GpuTimingConfig},
        resource::ResourceHandle,
        vulkan::{
            AccessTable, GpuResource, ResourceSpec, SessionFrameReceipt, VulkanGraphBackend,
            VulkanGraphSession,
        },
    },
    sync::SyncPoint,
};
use std::{
    collections::{BTreeMap, VecDeque},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

const VERTEX_READ: u8 = 0x04;
const TRANSFER_READ: u8 = 0x25;
const HOST_READ: u8 = 0x26;
const COLOR_WRITE: u8 = 0x2e;
const TRANSFER_WRITE: u8 = 0x34;
const HOST_WRITE: u8 = 0x35;
const FRAMES: usize = 300;
const WARMUP: usize = 10;
const TIMEOUT: u64 = 10_000_000_000;
const IMAGE_BYTES: u64 = (SIZE * SIZE * 4) as u64;
const SCENE_ID: &str = "raster-copy-readback-v1";
const VERIFY_FRAMES: usize = 8;
const DEFAULT_PERF_WARMUP: u64 = 300;
const DEFAULT_PERF_SAMPLES: u64 = 3000;
const DEFAULT_PERF_REPEATS: u32 = 3;

fn extent() -> vk::Extent2D {
    vk::Extent2D {
        width: SIZE,
        height: SIZE,
    }
}

fn color_layers() -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        mip_level: 0,
        base_array_layer: 0,
        layer_count: 1,
    }
}

struct Bench {
    rig: Rig,
    session: VulkanGraphSession,
    attachments: Arc<Mutex<AttachmentCache>>,
    readbacks: Vec<Arc<Buffer>>,
    readback_tokens: Vec<u64>,
    imports: VecDeque<u64>,
    gpu_keep: Vec<GpuCommandTiming>,
    frames_in_flight: usize,
    pending: VecDeque<PendingFrame>,
    completed: Vec<CompletedFrame>,
    next_frame: u64,
}

/// 每帧一条观测。
struct FrameSample {
    cpu: Duration,
    commands: usize,
    barrier_commands: u32,
    barrier_transitions: u32,
    queue_submits: usize,
    submit_batches: usize,
    graph_batches: u32,
}

impl Bench {
    fn new() -> Option<Self> {
        Self::new_with_frames_and_timing(2, false)
    }
    fn new_with_frames(frames_in_flight: usize) -> Option<Self> {
        Self::new_with_frames_and_timing(frames_in_flight, true)
    }
    fn new_with_frames_and_timing(frames_in_flight: usize, gpu_timing: bool) -> Option<Self> {
        let device = draw_support::gpu_test_support::initialize(
            "render perf device",
            CoreDevice::new_headless(&DeviceConfig {
                frames_in_flight: frames_in_flight as u32,
                ..DeviceConfig::headless()
            }),
        )?;
        let rig = Rig::with_device(device.clone());
        let mut backend =
            VulkanGraphBackend::new(rig.device.clone(), AccessTable::original()).ok()?;
        if gpu_timing {
            backend
                .configure_gpu_timing(Some(GpuTimingConfig {
                    pass_spans: true,
                    pending_commands: 4096,
                    retained_samples: 4096,
                }))
                .ok()?;
        }
        let readback_desc =
            BufferDesc::new_gpu_to_cpu(IMAGE_BYTES, vk::BufferUsageFlags::TRANSFER_DST);
        let mut readbacks = Vec::with_capacity(frames_in_flight);
        let mut readback_tokens = Vec::with_capacity(frames_in_flight);
        for _ in 0..frames_in_flight {
            let readback = Arc::new(Buffer::new(rig.device.clone(), &readback_desc).unwrap());
            let token = backend
                .bind_import(GpuResource::Buffer(readback.clone()))
                .unwrap();
            readbacks.push(readback);
            readback_tokens.push(token);
        }
        let session = VulkanGraphSession::new(backend);
        let attachments = Arc::new(Mutex::new(
            AttachmentCache::new(rig.device.clone()).unwrap(),
        ));
        Some(Self {
            rig,
            session,
            attachments,
            readbacks,
            readback_tokens,
            imports: VecDeque::new(),
            gpu_keep: Vec::new(),
            frames_in_flight,
            pending: VecDeque::new(),
            completed: Vec::new(),
            next_frame: 0,
        })
    }

    fn release_finished_imports(&mut self) {
        while let Some(&token) = self.imports.front() {
            if self.session.release_import(token).is_err() {
                break;
            }
            self.imports.pop_front();
        }
    }

    /// 一帧：返回 CPU 墙钟（begin_frame + 构图 + execute_frame）与回执。
    fn frame(&mut self, frame: usize) -> (Duration, SessionFrameReceipt) {
        self.frame_slot(0, frame)
    }
    fn frame_slot(&mut self, slot: usize, frame: usize) -> (Duration, SessionFrameReceipt) {
        self.release_finished_imports();
        let start = Instant::now();
        let graph = self.session.begin_frame().expect("begin_frame");
        let receipt = self.build_and_execute_slot(graph, frame, slot);
        (start.elapsed(), receipt)
    }

    fn build_and_execute_slot(
        &mut self,
        graph: RenderGraph,
        frame: usize,
        slot: usize,
    ) -> SessionFrameReceipt {
        let built = self.build_frame(graph, frame, slot);
        self.session.execute_frame(built).expect("execute_frame")
    }
    fn build_frame(&mut self, mut graph: RenderGraph, frame: usize, slot: usize) -> BuiltGraph {
        let device = self.rig.device.clone();
        let backend = &mut self.session.backend;
        let mut a_desc = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM);
        a_desc.usage |= vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::TRANSFER_SRC;
        let a = backend
            .declare(&mut graph, "a", ResourceSpec::Image(a_desc))
            .unwrap();
        let b_desc = BufferDesc::new_gpu_only(
            IMAGE_BYTES,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
        );
        let b = backend
            .declare(&mut graph, "b", ResourceSpec::Buffer(b_desc))
            .unwrap();
        let mut color_desc =
            TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM).color_attachment();
        color_desc.usage |= vk::ImageUsageFlags::TRANSFER_SRC;
        let color = backend
            .declare(&mut graph, "color", ResourceSpec::Image(color_desc))
            .unwrap();

        let scene_frame = frame % VERIFY_FRAMES;
        let vertices: Vec<u8> = rotated_triangle(scene_frame as f32 * 0.1)
            .iter()
            .flat_map(|v| [v[0], -v[1], v[2], v[3]])
            .flat_map(f32::to_le_bytes)
            .collect();
        let vb_desc =
            BufferDesc::new_cpu_to_gpu(vertices.len() as u64, vk::BufferUsageFlags::VERTEX_BUFFER);
        let vb_buffer = Arc::new(Buffer::new(device.clone(), &vb_desc).unwrap());
        vb_buffer.write_bytes_unsynchronized(0, &vertices).unwrap();
        let vb_token = backend.bind_import(GpuResource::Buffer(vb_buffer)).unwrap();
        self.imports.push_back(vb_token);
        let vb = graph
            .import_unowned(
                "vb",
                ResourceSpec::Buffer(vb_desc).descriptor(),
                vb_token,
                HOST_WRITE,
            )
            .unwrap();
        let rb_desc = BufferDesc::new_gpu_to_cpu(IMAGE_BYTES, vk::BufferUsageFlags::TRANSFER_DST);
        let rb = graph
            .import_unowned(
                "readback",
                ResourceSpec::Buffer(rb_desc).descriptor(),
                self.readback_tokens[slot],
                HOST_READ,
            )
            .unwrap();

        add_clear(&mut graph, &device, a, scene_frame);
        add_fill(&mut graph, &device, b, frame as u32);
        self.add_triangle(&mut graph, vb, color, scene_frame);
        add_copy_image(&mut graph, &device, color, a);
        add_copy_image_to_buffer(&mut graph, &device, a, b);
        add_copy_buffer(&mut graph, &device, b, rb);
        graph.export(rb, HOST_READ).unwrap();
        graph.build()
    }

    fn readback(&self, slot: usize) -> Vec<u8> {
        let mut pixels = vec![0u8; IMAGE_BYTES as usize];
        unsafe {
            self.readbacks[slot]
                .read_bytes_after_completion(0, &mut pixels)
                .expect("readback after completion")
        };
        pixels
    }

    fn add_triangle(
        &self,
        graph: &mut RenderGraph,
        vb: ResourceHandle,
        color: ResourceHandle,
        frame: usize,
    ) {
        let device = self.rig.device.clone();
        let pipeline = self.rig.pipeline.clone();
        let cache = self.attachments.clone();
        let clear = frame_clear_color(frame);
        let clear = [
            clear[0] as f32 / 255.0,
            clear[1] as f32 / 255.0,
            clear[2] as f32 / 255.0,
            clear[3] as f32 / 255.0,
        ];
        let (color_index, vb_index) = (color.raw(), vb.raw());
        let mut pass = graph.add_pass("triangle").with_side_effect();
        pass.add_resource_access(vb, VERTEX_READ, 0).unwrap();
        pass.add_resource_access(color, COLOR_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else {
                return false;
            };
            let (Ok(texture), Ok(vertices)) = (
                managed.texture_for(color_index),
                managed.buffer_for(vb_index),
            ) else {
                return false;
            };
            let Ok(attachment) = Attachment::color(texture.shared(), 0, Some(clear)) else {
                return false;
            };
            let Ok(prepared) = cache.lock().prepare_for_rendering(
                &[attachment],
                extent(),
                1,
                rhapsody::rg::attachments::RenderingPolicy::RequireDynamic,
            ) else {
                return false;
            };
            if let Err(error) = prepared.retain_for_pass(&ctx) {
                ctx.report_error(error);
                return false;
            }

            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            // SAFETY: 命令缓冲处于录制状态，资源由执行器保活到执行完成。
            let mut backend = unsafe { AshCommandBackend::new_for_device(&device) };
            let mut api = RenderPassApi::new(&mut backend, cb, 1, false);
            if api.begin_prepared(&prepared).is_err() {
                return false;
            }
            api.set_default_view_and_scissor(extent());
            api.set_stencil_ref(0);
            // SAFETY: 同上；管线与 render pass 兼容。
            unsafe {
                device.device.cmd_bind_pipeline(
                    cb,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.handle,
                )
            };
            api.bind_vertex_buffer_by_binding_point(0, vertices.raw());
            api.draw(3, 1, 0, 0);
            api.end_render_pass();
            true
        });
    }
}

type Device = Arc<rhapsody::raw::device::Device>;

fn add_clear(graph: &mut RenderGraph, device: &Device, target: ResourceHandle, frame: usize) {
    let raw = device.clone();
    let index = target.raw();
    let clear = frame_clear_color(frame);
    let value = [
        clear[0] as f32 / 255.0,
        clear[1] as f32 / 255.0,
        clear[2] as f32 / 255.0,
        clear[3] as f32 / 255.0,
    ];
    let mut pass = graph.add_pass("clear a").with_side_effect();
    pass.add_resource_access(target, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else {
            return false;
        };
        let Ok(image) = managed.texture_for(index) else {
            return false;
        };

        // SAFETY: 录制中的命令缓冲；图已把纹理转到 TRANSFER_DST_OPTIMAL。
        unsafe {
            raw.device.cmd_clear_color_image(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                image.raw(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue { float32: value },
                &[vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)],
            );
        }
        true
    });
}

fn add_fill(graph: &mut RenderGraph, device: &Device, target: ResourceHandle, value: u32) {
    let raw = device.clone();
    let index = target.raw();
    let mut pass = graph.add_pass("fill b").with_side_effect();
    pass.add_resource_access(target, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else {
            return false;
        };
        let Ok(buffer) = managed.buffer_for(index) else {
            return false;
        };

        // SAFETY: 录制中的命令缓冲；缓冲带 TRANSFER_DST。
        unsafe {
            raw.device.cmd_fill_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                buffer.raw(),
                0,
                vk::WHOLE_SIZE,
                value,
            );
        }
        true
    });
}

fn add_copy_image(
    graph: &mut RenderGraph,
    device: &Device,
    src: ResourceHandle,
    dst: ResourceHandle,
) {
    let raw = device.clone();
    let (s, d) = (src.raw(), dst.raw());
    let mut pass = graph.add_pass("color to a").with_side_effect();
    pass.add_resource_access(src, TRANSFER_READ, 0).unwrap();
    pass.add_resource_access(dst, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else {
            return false;
        };
        let (Ok(src), Ok(dst)) = (managed.texture_for(s), managed.texture_for(d)) else {
            return false;
        };

        // SAFETY: 录制中的命令缓冲；布局由图的访问声明保证。
        unsafe {
            raw.device.cmd_copy_image(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                src.raw(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst.raw(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[vk::ImageCopy {
                    src_subresource: color_layers(),
                    dst_subresource: color_layers(),
                    extent: vk::Extent3D {
                        width: SIZE,
                        height: SIZE,
                        depth: 1,
                    },
                    ..Default::default()
                }],
            );
        }
        true
    });
}

fn add_copy_image_to_buffer(
    graph: &mut RenderGraph,
    device: &Device,
    src: ResourceHandle,
    dst: ResourceHandle,
) {
    let raw = device.clone();
    let (s, d) = (src.raw(), dst.raw());
    let mut pass = graph.add_pass("a to b").with_side_effect();
    pass.add_resource_access(src, TRANSFER_READ, 0).unwrap();
    pass.add_resource_access(dst, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else {
            return false;
        };
        let (Ok(src), Ok(dst)) = (managed.texture_for(s), managed.buffer_for(d)) else {
            return false;
        };

        // SAFETY: 同上。
        unsafe {
            raw.device.cmd_copy_image_to_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                src.raw(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst.raw(),
                &[vk::BufferImageCopy {
                    image_subresource: color_layers(),
                    image_extent: vk::Extent3D {
                        width: SIZE,
                        height: SIZE,
                        depth: 1,
                    },
                    ..Default::default()
                }],
            );
        }
        true
    });
}

fn add_copy_buffer(
    graph: &mut RenderGraph,
    device: &Device,
    src: ResourceHandle,
    dst: ResourceHandle,
) {
    let raw = device.clone();
    let (s, d) = (src.raw(), dst.raw());
    let mut pass = graph.add_pass("b to readback").with_side_effect();
    pass.add_resource_access(src, TRANSFER_READ, 0).unwrap();
    pass.add_resource_access(dst, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else {
            return false;
        };
        let (Ok(src), Ok(dst)) = (managed.buffer_for(s), managed.buffer_for(d)) else {
            return false;
        };

        // SAFETY: 同上。
        unsafe {
            raw.device.cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                src.raw(),
                dst.raw(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: IMAGE_BYTES,
                }],
            );
        }
        true
    });
}

fn percentile(sorted: &[Duration], p: f64) -> Duration {
    let index = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[index]
}

fn mean<T: Copy + Into<f64>>(values: impl Iterator<Item = T>) -> f64 {
    let (sum, n) = values.fold((0.0, 0usize), |(s, n), v| (s + v.into(), n + 1));
    sum / n.max(1) as f64
}

#[test]
#[ignore = "性能基线：手动运行，打印数字不做断言"]
fn representative_graph_frame_cost_baseline() {
    let Some(mut bench) = Bench::new() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    let mut samples = Vec::with_capacity(FRAMES);
    for frame in 0..FRAMES {
        let (cpu, receipt) = bench.frame(frame);
        let captures = &bench.session.backend.submissions;
        samples.push(FrameSample {
            cpu,
            commands: receipt.stats.commands_recorded as usize,
            barrier_commands: receipt.stats.barrier_commands,
            barrier_transitions: receipt.stats.barrier_transitions,
            queue_submits: captures.len(),
            submit_batches: captures.iter().map(|c| c.batches.len()).sum(),
            graph_batches: receipt.stats.batches,
        });
    }
    bench.session.wait_submits_done(TIMEOUT).unwrap();
    assert!(!bench.session.is_poisoned());

    let steady = &samples[WARMUP..];
    let mut cpu: Vec<Duration> = steady.iter().map(|s| s.cpu).collect();
    cpu.sort_unstable();
    let mean_cpu = cpu.iter().sum::<Duration>() / cpu.len() as u32;
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    println!(
        "p0 基线（{profile}，{} 帧，丢弃前 {WARMUP} 帧热身，6 pass）",
        FRAMES
    );
    println!(
        "CPU begin+构图+execute: mean {:.3} ms  p50 {:.3} ms  p95 {:.3} ms  max {:.3} ms",
        mean_cpu.as_secs_f64() * 1e3,
        percentile(&cpu, 0.50).as_secs_f64() * 1e3,
        percentile(&cpu, 0.95).as_secs_f64() * 1e3,
        cpu.last().unwrap().as_secs_f64() * 1e3,
    );
    println!(
        "每帧：命令缓冲 {:.2}  屏障命令 {:.2}  屏障转换 {:.2}  vkQueueSubmit {:.2}  submit batch {:.2}  图 batch {:.2}",
        mean(steady.iter().map(|s| s.commands as u32)),
        mean(steady.iter().map(|s| s.barrier_commands)),
        mean(steady.iter().map(|s| s.barrier_transitions)),
        mean(steady.iter().map(|s| s.queue_submits as u32)),
        mean(steady.iter().map(|s| s.submit_batches as u32)),
        mean(steady.iter().map(|s| s.graph_batches)),
    );
}

// ---- Formal render performance mode ----

fn perf_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/render-perf")
}

fn fnv_hex(bytes: &[u8]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn scene_fingerprint(references: &[Vec<u8>]) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(SCENE_ID.as_bytes());
    bytes.extend_from_slice(&SIZE.to_le_bytes());
    bytes.extend_from_slice(&(VERIFY_FRAMES as u32).to_le_bytes());
    bytes.extend_from_slice(&0.1f32.to_le_bytes());
    for reference in references {
        bytes.extend_from_slice(reference);
    }
    fnv_hex(&bytes)
}

fn write_ppm(path: &std::path::Path, pixels: &[u8]) {
    let mut output = Vec::with_capacity(32 + (SIZE * SIZE * 3) as usize);
    output.extend_from_slice(format!("P6\n{SIZE} {SIZE}\n255\n").as_bytes());
    for pixel in pixels.as_chunks::<4>().0 {
        output.extend_from_slice(&pixel[..3]);
    }
    std::fs::write(path, output).unwrap();
}

#[derive(Clone, Copy, Debug, Default)]
struct PixelCheck {
    mismatches: usize,
    illegal: usize,
    filled: usize,
    centroid_error: f32,
}

fn compare_pixels(pixels: &[u8], reference: &[u8], frame: usize) -> PixelCheck {
    let clear = frame_clear_color(frame);
    let mut check = PixelCheck::default();
    let (mut sum_x, mut sum_y) = (0.0f32, 0.0f32);
    for (index, (got, want)) in pixels
        .as_chunks::<4>()
        .0
        .iter()
        .zip(reference.as_chunks::<4>().0.iter())
        .enumerate()
    {
        if got != want {
            check.mismatches += 1;
        }
        if *got != draw_support::FILL && *got != clear {
            check.illegal += 1;
        }
        if *got == draw_support::FILL {
            let x = (index as u32 % SIZE) as f32 + 0.5;
            let y = (index as u32 / SIZE) as f32 + 0.5;
            sum_x += x;
            sum_y += y;
            check.filled += 1;
        }
    }
    if check.filled > 0 {
        let measured = (sum_x / check.filled as f32, sum_y / check.filled as f32);
        let angle = frame as f32 * 0.1;
        let (s, c) = angle.sin_cos();
        let expected = (
            (s * (1.0 / 6.0) + 1.0) * 0.5 * SIZE as f32,
            (-c * (1.0 / 6.0) + 1.0) * 0.5 * SIZE as f32,
        );
        check.centroid_error =
            ((measured.0 - expected.0).powi(2) + (measured.1 - expected.1).powi(2)).sqrt();
    } else {
        check.centroid_error = f32::INFINITY;
    }
    check
}

fn pixel_check_passes(check: &PixelCheck) -> bool {
    check.mismatches <= 8
        && check.illegal == 0
        && (470..=555).contains(&check.filled)
        && check.centroid_error < 0.8
}

fn device_identity(device: &Arc<rhapsody::raw::device::Device>) -> (String, u32, u32) {
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    (
        unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_string_lossy()
            .into_owned(),
        properties.vendor_id,
        properties.driver_version,
    )
}

fn build_profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

fn dist_json(values: &[u64]) -> serde_json::Value {
    if values.is_empty() {
        return serde_json::json!({"samples":0});
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let p = |q: f64| sorted[((sorted.len() - 1) as f64 * q).ceil() as usize];
    serde_json::json!({
        "samples": sorted.len(),
        "min_ns": sorted[0],
        "mean_ns": sorted.iter().map(|v| *v as f64).sum::<f64>() / sorted.len() as f64,
        "p50_ns": p(0.50),
        "p95_ns": p(0.95),
        "p99_ns": p(0.99),
        "max_ns": sorted[sorted.len() - 1],
    })
}

fn count_json(values: &[u64]) -> serde_json::Value {
    if values.is_empty() {
        return serde_json::json!({"samples":0});
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let p = |q: f64| sorted[((sorted.len() - 1) as f64 * q).ceil() as usize];
    serde_json::json!({
        "samples": sorted.len(),
        "min": sorted[0],
        "mean": sorted.iter().map(|v| *v as f64).sum::<f64>() / sorted.len() as f64,
        "p50": p(0.50),
        "p95": p(0.95),
        "p99": p(0.99),
        "max": sorted[sorted.len() - 1],
    })
}

#[test]
#[ignore = "explicit prepare step for render performance mode"]
fn render_perf_prepare() {
    let dir = perf_dir().join("prepared");
    std::fs::create_dir_all(&dir).unwrap();
    let data_start = Instant::now();
    let coverage: Vec<Vec<bool>> = (0..VERIFY_FRAMES)
        .map(|frame| reference_coverage(&rotated_triangle(frame as f32 * 0.1)))
        .collect();
    let data_ns = data_start.elapsed().as_nanos() as u64;
    let reference_start = Instant::now();
    let references: Vec<Vec<u8>> = coverage
        .iter()
        .enumerate()
        .map(|(frame, covered)| {
            let clear = frame_clear_color(frame);
            let mut pixels = Vec::with_capacity(IMAGE_BYTES as usize);
            for pixel in covered {
                pixels.extend_from_slice(if *pixel { &draw_support::FILL } else { &clear });
            }
            pixels
        })
        .collect();
    let reference_ns = reference_start.elapsed().as_nanos() as u64;
    let fingerprint = scene_fingerprint(&references);
    let write_start = Instant::now();
    let mut files = Vec::new();
    for (frame, pixels) in references.iter().enumerate() {
        let bin = format!("reference-{frame:02}.bin");
        let ppm = format!("reference-{frame:02}.ppm");
        std::fs::write(dir.join(&bin), pixels).unwrap();
        write_ppm(&dir.join(&ppm), pixels);
        files.push(serde_json::json!({
            "frame": frame,
            "bin": bin,
            "ppm": ppm,
            "bytes": pixels.len(),
            "hash": fnv_hex(pixels),
        }));
    }
    let write_ns = write_start.elapsed().as_nanos() as u64;
    let manifest = serde_json::json!({
        "schema": 1,
        "scene": SCENE_ID,
        "fingerprint": fingerprint,
        "size": SIZE,
        "frames": VERIFY_FRAMES,
        "angle_step_rad": 0.1,
        "build_profile": build_profile(),
        "prepare_timing": {
            "data_generation_ns": data_ns,
            "reference_generation_ns": reference_ns,
            "write_ns": write_ns,
        },
        "files": files,
        "notes": [
            "The animation timeline is frame % 8; every sampled frame maps to one pre-generated reference.",
            "The scene is the existing raster triangle + clear/fill/copy/readback production graph.",
            "prepare does not create a Vulkan device.",
        ],
    });
    let serialize_start = Instant::now();
    let text = serde_json::to_string_pretty(&manifest).unwrap();
    let serialize_ns = serialize_start.elapsed().as_nanos() as u64;
    let manifest_write_start = Instant::now();
    std::fs::write(dir.join("manifest.json"), text).unwrap();
    let manifest_write_ns = manifest_write_start.elapsed().as_nanos() as u64;
    println!(
        "{}",
        serde_json::json!({
            "status": "prepared",
            "scene": SCENE_ID,
            "fingerprint": fingerprint,
            "prepared_dir": dir,
            "prepare_timing": {
                "data_generation_ns": data_ns,
                "reference_generation_ns": reference_ns,
                "write_ns": write_ns,
                "manifest_serialize_ns": serialize_ns,
                "manifest_write_ns": manifest_write_ns,
            }
        })
    );
}

fn load_prepared() -> (serde_json::Value, Vec<Vec<u8>>) {
    let dir = perf_dir().join("prepared");
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["scene"], SCENE_ID);
    assert_eq!(manifest["size"], SIZE);
    assert_eq!(manifest["frames"], VERIFY_FRAMES);
    let mut references = Vec::new();
    for frame in 0..VERIFY_FRAMES {
        let file = dir.join(format!("reference-{frame:02}.bin"));
        let bytes = std::fs::read(&file).unwrap();
        let entry = &manifest["files"][frame];
        assert_eq!(entry["hash"], fnv_hex(&bytes));
        references.push(bytes);
    }
    let fingerprint = scene_fingerprint(&references);
    assert_eq!(manifest["fingerprint"], fingerprint);
    (manifest, references)
}

#[test]
#[ignore = "strict acceptance for render performance mode; run in release before throughput"]
fn render_perf_strict_acceptance() {
    let load_started = Instant::now();
    let (manifest, references) = load_prepared();
    let data_load_ns = load_started.elapsed().as_nanos() as u64;
    let setup_started = Instant::now();
    let Some(mut bench) = Bench::new_with_frames_and_timing(1, false) else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    let setup_ns = setup_started.elapsed().as_nanos() as u64;
    let frames_start = Instant::now();
    let mut checks = Vec::new();
    for (frame, reference) in references.iter().enumerate() {
        let (_, receipt) = bench.frame_slot(0, frame);
        bench
            .session
            .wait_frame(receipt.frame, TIMEOUT)
            .expect("strict acceptance wait");
        let pixels = bench.readback(0);
        checks.push(compare_pixels(&pixels, reference, frame));
        bench.release_finished_imports();
    }
    let frames_ns = frames_start.elapsed().as_nanos() as u64;
    let passed = checks.iter().all(pixel_check_passes);
    let (device_name, vendor_id, driver_version) = device_identity(&bench.rig.device);
    let validation_layers: Vec<String> = bench
        .rig
        .device
        .instance
        .layer_names()
        .iter()
        .filter(|name| name.to_ascii_lowercase().contains("validation"))
        .cloned()
        .collect();
    let mut report = serde_json::json!({
        "schema": 1,
        "kind": "correctness",
        "status": if passed { "passed" } else { "failed" },
        "scene": SCENE_ID,
        "scene_fingerprint": scene_fingerprint(&references),
        "build_profile": build_profile(),
        "device": device_name,
        "vendor_id": vendor_id,
        "driver_version": driver_version,
        "validation_layers": validation_layers,
        "frames_checked": VERIFY_FRAMES,
        "pixels_checked": VERIFY_FRAMES * (SIZE * SIZE) as usize,
        "max_mismatches": checks.iter().map(|c| c.mismatches).max(),
        "max_illegal_pixels": checks.iter().map(|c| c.illegal).max(),
        "min_filled": checks.iter().map(|c| c.filled).min(),
        "max_filled": checks.iter().map(|c| c.filled).max(),
        "max_centroid_error": checks.iter().map(|c| c.centroid_error).fold(0.0f32, f32::max),
        "checks": checks.iter().map(|c| serde_json::json!({
            "mismatches": c.mismatches,
            "illegal": c.illegal,
            "filled": c.filled,
            "centroid_error": c.centroid_error,
        })).collect::<Vec<_>>(),
        "timings": {
            "data_load_ns": data_load_ns,
            "setup_ns": setup_ns,
            "frames_ns": frames_ns,
            "report_serialize_ns": 0,
            "report_write_ns": 0,
            "prepare": manifest["prepare_timing"],
        },
    });
    let serialize_start = Instant::now();
    let text = serde_json::to_string_pretty(&report).unwrap();
    let serialize_ns = serialize_start.elapsed().as_nanos() as u64;
    let write_start = Instant::now();
    std::fs::write(perf_dir().join("acceptance.json"), text).unwrap();
    let write_ns = write_start.elapsed().as_nanos() as u64;
    report["timings"]["report_serialize_ns"] = serde_json::json!(serialize_ns);
    report["timings"]["report_write_ns"] = serde_json::json!(write_ns);
    std::fs::write(
        perf_dir().join("acceptance.json"),
        serde_json::to_string_pretty(&report).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::json!({
            "status": report["status"],
            "acceptance": perf_dir().join("acceptance.json"),
            "max_mismatches": report["max_mismatches"],
            "max_illegal_pixels": report["max_illegal_pixels"],
            "frames_ns": frames_ns,
            "report_serialize_ns": serialize_ns,
            "report_write_ns": write_ns,
        })
    );
    assert!(passed, "strict render acceptance failed: {checks:?}");
}

#[derive(Clone, Copy, Debug)]
struct PerfSettings {
    warmup: u64,
    samples: u64,
    repeats: u32,
    sample_every: u64,
    frames_in_flight: usize,
    cache_enabled: bool,
}

fn perf_settings() -> PerfSettings {
    let env_u64 = |name: &str, default: u64| {
        std::env::var(name)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(default)
    };
    let settings = PerfSettings {
        warmup: env_u64("RHAPSODY_PERF_WARMUP", DEFAULT_PERF_WARMUP),
        samples: env_u64("RHAPSODY_PERF_SAMPLES", DEFAULT_PERF_SAMPLES),
        repeats: env_u64("RHAPSODY_PERF_REPEATS", DEFAULT_PERF_REPEATS as u64) as u32,
        sample_every: env_u64("RHAPSODY_PERF_SAMPLE_EVERY", 60).max(1),
        frames_in_flight: env_u64("RHAPSODY_PERF_FRAMES_IN_FLIGHT", 3).min(3) as usize,
        cache_enabled: match std::env::var("RHAPSODY_PERF_CACHE").as_deref() {
            Ok("on") | Err(_) => true,
            Ok("off") => false,
            Ok(other) => panic!("RHAPSODY_PERF_CACHE must be on or off, got {other}"),
        },
    };
    let allow_short = std::env::var("RHAPSODY_PERF_ALLOW_SHORT").is_ok_and(|v| v == "1");
    assert!(
        allow_short
            || (settings.warmup >= DEFAULT_PERF_WARMUP
                && settings.samples >= DEFAULT_PERF_SAMPLES
                && settings.repeats >= DEFAULT_PERF_REPEATS),
        "formal render perf requires warmup>=300, samples>=3000, repeats>=3; set RHAPSODY_PERF_ALLOW_SHORT=1 for a smoke run"
    );
    assert!((1..=3).contains(&settings.frames_in_flight));
    settings
}

fn load_acceptance(device_name: &str, driver_version: u32) -> serde_json::Value {
    let text = std::fs::read_to_string(perf_dir().join("acceptance.json"))
        .expect("run render_perf_strict_acceptance first");
    let acceptance: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(acceptance["status"], "passed");
    assert_eq!(acceptance["scene"], SCENE_ID);
    assert_eq!(acceptance["build_profile"], "release");
    assert_eq!(acceptance["validation_layers"], serde_json::json!([]));
    let allow_device_change =
        std::env::var("RHAPSODY_PERF_ALLOW_DEVICE_CHANGE").is_ok_and(|value| value == "1");
    if !allow_device_change {
        assert_eq!(acceptance["device"], device_name);
        assert_eq!(acceptance["driver_version"], driver_version);
    }
    acceptance
}

struct PendingFrame {
    receipt: SessionFrameReceipt,
    slot: usize,
    scene_frame: usize,
    sampled: bool,
    begin_ns: u64,
    build_ns: u64,
    execute_ns: u64,
    allocation: allocation_counter::Sample,
    gpu_allocations: u64,
}

struct CompletedFrame {
    begin_ns: u64,
    build_ns: u64,
    execute_ns: u64,
    stats: rhapsody::rg::executor::ExecutionStats,
    allocation: allocation_counter::Sample,
    gpu_allocations: u64,
    wait_slot_ns: u64,
    gpu_frame_ns: u64,
    gpu_measured: bool,
    gpu_passes: BTreeMap<String, u64>,
    readback: Option<PixelCheck>,
    readback_ns: u64,
}

impl Bench {
    fn set_caches(&mut self, enabled: bool) {
        self.session.chain.set_plan_cache_enabled(enabled);
        self.session.chain.set_subgraph_cache_enabled(enabled);
        self.session.set_result_cache_enabled(enabled);
    }

    fn gpu_for(
        &mut self,
        receipt: &SessionFrameReceipt,
    ) -> Result<(u64, BTreeMap<String, u64>, bool), rhapsody::Error> {
        let _ = self.session.backend.poll_gpu_timings()?;
        let points: std::collections::HashSet<(usize, u64)> = receipt
            .receipts
            .iter()
            .map(|point: &SyncPoint| (point.queue.index(), point.value))
            .collect();
        self.gpu_keep
            .extend(self.session.backend.drain_gpu_timings());
        if self.gpu_keep.len() > 4096 {
            return Err(rhapsody::Error::invalid_usage(
                "GPU timing samples did not map to any frame",
            ));
        }
        let mut total = 0u64;
        let mut matched = false;
        let mut passes = BTreeMap::new();
        self.gpu_keep.retain(|sample: &GpuCommandTiming| {
            if points.contains(&(sample.queue, sample.completion_value)) {
                matched = true;
                if let Some(index) = sample.pass_index {
                    if let Some(name) = receipt.snapshot.pass_order.get(index) {
                        *passes.entry(name.clone()).or_insert(0) += sample.duration_ns;
                    }
                } else {
                    total = total.saturating_add(sample.duration_ns);
                }
                false
            } else {
                true
            }
        });
        Ok((total, passes, matched))
    }

    fn run_pipelined(&mut self, references: &[Vec<u8>], sampled: bool) -> rhapsody::Result<()> {
        if self.session.pending_frame_count() >= self.frames_in_flight {
            let oldest = self
                .pending
                .front()
                .ok_or_else(|| rhapsody::Error::invalid_usage("missing oldest pending frame"))?
                .receipt
                .frame;
            let started = Instant::now();
            self.session.wait_frame(oldest, TIMEOUT)?;
            let wait_slot_ns = started.elapsed().as_nanos() as u64;
            self.complete_front(references, wait_slot_ns)?;
        }
        let slot = (self.next_frame as usize) % self.frames_in_flight;
        let scene_frame = self.next_frame as usize;
        self.next_frame += 1;
        let gpu_before = self.rig.device.allocator_stats.total_registered();
        let (result, allocation) = allocation_counter::measured(|| {
            let begin_started = Instant::now();
            let graph = self.session.begin_frame()?;
            let begin_ns = begin_started.elapsed().as_nanos() as u64;
            let build_started = Instant::now();
            let built = self.build_frame(graph, scene_frame, slot);
            let build_ns = build_started.elapsed().as_nanos() as u64;
            let execute_started = Instant::now();
            let receipt = self.session.execute_frame(built)?;
            let execute_ns = execute_started.elapsed().as_nanos() as u64;
            Ok::<_, rhapsody::Error>((receipt, begin_ns, build_ns, execute_ns))
        });
        let (receipt, begin_ns, build_ns, execute_ns) = result?;
        let gpu_allocations = self
            .rig
            .device
            .allocator_stats
            .total_registered()
            .saturating_sub(gpu_before);
        self.pending.push_back(PendingFrame {
            receipt,
            slot,
            scene_frame,
            sampled,
            begin_ns,
            build_ns,
            execute_ns,
            allocation,
            gpu_allocations,
        });
        Ok(())
    }

    fn complete_front(
        &mut self,
        references: &[Vec<u8>],
        wait_slot_ns: u64,
    ) -> rhapsody::Result<()> {
        let pending = self
            .pending
            .pop_front()
            .ok_or_else(|| rhapsody::Error::invalid_usage("missing pending frame"))?;
        let (gpu_frame_ns, gpu_passes, gpu_measured) = self.gpu_for(&pending.receipt)?;
        let stats = pending.receipt.stats;
        let readback_started = Instant::now();
        let readback = pending.sampled.then(|| {
            let pixels = self.readback(pending.slot);
            let frame = pending.scene_frame % VERIFY_FRAMES;
            compare_pixels(&pixels, &references[frame], frame)
        });
        let readback_ns = readback_started.elapsed().as_nanos() as u64;
        self.completed.push(CompletedFrame {
            begin_ns: pending.begin_ns,
            build_ns: pending.build_ns,
            execute_ns: pending.execute_ns,
            stats,
            allocation: pending.allocation,
            gpu_allocations: pending.gpu_allocations,
            wait_slot_ns,
            gpu_frame_ns,
            gpu_measured,
            gpu_passes,
            readback,
            readback_ns,
        });
        self.release_finished_imports();
        Ok(())
    }

    fn drain_pending(&mut self, references: &[Vec<u8>]) -> rhapsody::Result<u64> {
        let started = Instant::now();
        while let Some(frame) = self.pending.front().map(|pending| pending.receipt.frame) {
            self.session.wait_frame(frame, TIMEOUT)?;
            self.complete_front(references, 0)?;
        }
        Ok(started.elapsed().as_nanos() as u64)
    }
}

#[test]
#[ignore = "formal render throughput run; requires release + prepare + strict acceptance"]
fn render_perf_throughput() {
    if cfg!(debug_assertions) {
        panic!("formal render throughput requires a release test binary");
    }
    let settings = perf_settings();
    let load_started = Instant::now();
    let (manifest, references) = load_prepared();
    let data_load_ns = load_started.elapsed().as_nanos() as u64;
    let Some(probe) = Bench::new_with_frames(settings.frames_in_flight) else {
        eprintln!("skip: no Vulkan device or GPU timing unsupported");
        return;
    };
    let (device_name, vendor_id, driver_version) = device_identity(&probe.rig.device);
    let validation_layers: Vec<String> = probe
        .rig
        .device
        .instance
        .layer_names()
        .iter()
        .filter(|name| name.to_ascii_lowercase().contains("validation"))
        .cloned()
        .collect();
    let acceptance = load_acceptance(&device_name, driver_version);
    assert_eq!(acceptance["scene_fingerprint"], manifest["fingerprint"]);
    let allow_device_change =
        std::env::var("RHAPSODY_PERF_ALLOW_DEVICE_CHANGE").is_ok_and(|value| value == "1");
    drop(probe);

    let mut runs = Vec::with_capacity(settings.repeats as usize);
    for repeat in 0..settings.repeats {
        let setup_started = Instant::now();
        let mut bench = Bench::new_with_frames(settings.frames_in_flight)
            .expect("render throughput device creation");
        bench.set_caches(settings.cache_enabled);
        let (repeat_device, _, repeat_driver) = device_identity(&bench.rig.device);
        if !allow_device_change {
            assert_eq!(repeat_device, device_name);
            assert_eq!(repeat_driver, driver_version);
        }
        assert!(
            !bench
                .rig
                .device
                .instance
                .layer_names()
                .iter()
                .any(|name| name.to_ascii_lowercase().contains("validation")),
            "formal throughput must run without validation layers"
        );
        let setup_ns = setup_started.elapsed().as_nanos() as u64;
        for _ in 0..settings.warmup {
            bench
                .run_pipelined(&references, false)
                .expect("warmup frame");
        }
        let warmup_drain_ns = bench.drain_pending(&references).expect("warmup drain");
        bench.completed.clear();
        bench.gpu_keep.clear();

        let wall_started = Instant::now();
        let mut starts = Vec::with_capacity(settings.samples as usize);
        for index in 0..settings.samples {
            starts.push(Instant::now());
            let sampled = index.is_multiple_of(settings.sample_every);
            bench
                .run_pipelined(&references, sampled)
                .expect("throughput frame");
        }
        let drain_ns = bench.drain_pending(&references).expect("throughput drain");
        let wall_ns = wall_started.elapsed().as_nanos() as u64;
        let completed = std::mem::take(&mut bench.completed);

        let mut begin = Vec::with_capacity(completed.len());
        let mut build = Vec::with_capacity(completed.len());
        let mut compile = Vec::with_capacity(completed.len());
        let mut record = Vec::with_capacity(completed.len());
        let mut submit = Vec::with_capacity(completed.len());
        let mut cpu_frame = Vec::with_capacity(completed.len());
        let mut wait_slot = Vec::with_capacity(completed.len());
        let mut gpu_frame = Vec::with_capacity(completed.len());
        let mut cpu_allocations = Vec::with_capacity(completed.len());
        let mut cpu_allocated_bytes = Vec::with_capacity(completed.len());
        let mut gpu_allocations = Vec::with_capacity(completed.len());
        let mut gpu_passes: BTreeMap<String, Vec<u64>> = BTreeMap::new();
        let mut readback_samples = 0u64;
        let mut readback_failed = 0u64;
        let mut readback_read_ns = Vec::new();
        let mut max_mismatches = 0usize;
        let mut max_illegal = 0usize;
        let mut min_filled = usize::MAX;
        let mut max_filled = 0usize;
        let mut max_centroid_error = 0.0f32;
        let mut gpu_missing_frames = 0u64;
        let mut gpu_zero_span_frames = 0u64;
        for frame in &completed {
            begin.push(frame.begin_ns);
            build.push(frame.build_ns);
            compile.push(frame.stats.compile_ns);
            record.push(frame.stats.record_ns);
            submit.push(frame.stats.submit_ns);
            cpu_frame.push(
                frame
                    .begin_ns
                    .saturating_add(frame.build_ns)
                    .saturating_add(frame.execute_ns),
            );
            wait_slot.push(frame.wait_slot_ns);
            gpu_frame.push(frame.gpu_frame_ns);
            if !frame.gpu_measured {
                gpu_missing_frames += 1;
            } else if frame.gpu_frame_ns == 0 {
                gpu_zero_span_frames += 1;
            }
            cpu_allocations.push(frame.allocation.allocations);
            cpu_allocated_bytes.push(frame.allocation.allocated_bytes);
            gpu_allocations.push(frame.gpu_allocations);
            for (name, duration) in &frame.gpu_passes {
                gpu_passes.entry(name.clone()).or_default().push(*duration);
            }
            if let Some(check) = frame.readback {
                readback_samples += 1;
                if !pixel_check_passes(&check) {
                    readback_failed += 1;
                }
                max_mismatches = max_mismatches.max(check.mismatches);
                max_illegal = max_illegal.max(check.illegal);
                min_filled = min_filled.min(check.filled);
                max_filled = max_filled.max(check.filled);
                max_centroid_error = max_centroid_error.max(check.centroid_error);
                readback_read_ns.push(frame.readback_ns);
            }
        }
        let mut frame_interval = Vec::with_capacity(starts.len().saturating_sub(1));
        for pair in starts.windows(2) {
            frame_interval.push(pair[1].duration_since(pair[0]).as_nanos() as u64);
        }
        let gpu_passes: BTreeMap<String, serde_json::Value> = gpu_passes
            .iter()
            .map(|(name, values)| (name.clone(), dist_json(values)))
            .collect();
        let readback_passed = readback_samples.saturating_sub(readback_failed);
        let gpu_timing_ok = gpu_missing_frames.saturating_mul(10) <= completed.len() as u64;
        let status = if readback_samples > 0 && readback_failed == 0 && gpu_timing_ok {
            "passed"
        } else {
            "failed"
        };
        runs.push(serde_json::json!({
            "repeat": repeat,
            "pacing": "unlimited",
            "present_mode": "offscreen",
            "warmup_frames": settings.warmup,
            "measured_frames": completed.len(),
            "wall_ns": wall_ns,
            "throughput_fps": completed.len() as f64 * 1e9 / wall_ns.max(1) as f64,
            "frame_interval_ns": dist_json(&frame_interval),
            "cpu_begin_ns": dist_json(&begin),
            "cpu_build_ns": dist_json(&build),
            "cpu_compile_ns": dist_json(&compile),
            "cpu_record_ns": dist_json(&record),
            "cpu_submit_ns": dist_json(&submit),
            "cpu_frame_ns": dist_json(&cpu_frame),
            "wait_slot_ns": dist_json(&wait_slot),
            "gpu_frame_ns": dist_json(&gpu_frame),
            "gpu_missing_frames": gpu_missing_frames,
            "gpu_zero_span_frames": gpu_zero_span_frames,
            "gpu_timing_ok": gpu_timing_ok,
            "gpu_pass_ns": gpu_passes,
            "cpu_allocations_per_frame": count_json(&cpu_allocations),
            "cpu_allocated_bytes_per_frame": count_json(&cpu_allocated_bytes),
            "gpu_allocations_per_frame": count_json(&gpu_allocations),
            "peak_allocated_bytes": bench.rig.device.allocator_stats.peak_totals().total_allocated_bytes,
            "setup_ns": setup_ns,
            "drain_ns": drain_ns,
            "warmup_drain_ns": warmup_drain_ns,
            "readback": {
                "samples": readback_samples,
                "passed": readback_passed,
                "failed": readback_failed,
                "max_mismatches": max_mismatches,
                "max_illegal_pixels": max_illegal,
                "min_filled": if readback_samples == 0 { 0 } else { min_filled },
                "max_filled": max_filled,
                "max_centroid_error": max_centroid_error,
                "cpu_read_ns": dist_json(&readback_read_ns),
            },
            "status": status,
        }));
        bench.rig.device.wait_idle().expect("post-run wait_idle");
        println!(
            "render_perf repeat {repeat}: frames={} fps={:.2} frame p50={:.3}ms p99={:.3}ms cpu={:.3}ms gpu={:.3}ms wait={:.3}ms rb={}/{}",
            completed.len(),
            completed.len() as f64 * 1e9 / wall_ns.max(1) as f64,
            dist_json(&frame_interval)["p50_ns"].as_u64().unwrap_or(0) as f64 / 1e6,
            dist_json(&frame_interval)["p99_ns"].as_u64().unwrap_or(0) as f64 / 1e6,
            dist_json(&cpu_frame)["p50_ns"].as_u64().unwrap_or(0) as f64 / 1e6,
            dist_json(&gpu_frame)["p50_ns"].as_u64().unwrap_or(0) as f64 / 1e6,
            dist_json(&wait_slot)["p50_ns"].as_u64().unwrap_or(0) as f64 / 1e6,
            readback_passed,
            readback_samples,
        );
    }
    let failed = runs.iter().any(|run| run["status"] != "passed");
    let zero_span_note = runs
        .iter()
        .any(|run| run["gpu_zero_span_frames"].as_u64().unwrap_or(0) > 0)
        .then_some("Some GPU command/pass spans measured as zero; the workload may be below the driver timestamp resolution. CPU/wall metrics remain valid.");
    let mut report = serde_json::json!({
        "schema": 1,
        "kind": "bench",
        "status": if failed { "failed" } else { "passed" },
        "scene": SCENE_ID,
        "scene_fingerprint": manifest["fingerprint"],
        "acceptance": perf_dir().join("acceptance.json"),
        "environment": {
            "device": device_name,
            "vendor_id": vendor_id,
            "driver_version": driver_version,
            "build_profile": build_profile(),
            "validation_layers": validation_layers,
            "frames_in_flight": settings.frames_in_flight,
            "cache_enabled": settings.cache_enabled,
        },
        "settings": {
            "warmup": settings.warmup,
            "samples": settings.samples,
            "repeats": settings.repeats,
            "sample_every": settings.sample_every,
            "cache_enabled": settings.cache_enabled,
        },
        "prepare_timing": manifest["prepare_timing"],
        "data_load_ns": data_load_ns,
        "present_mode": "offscreen",
        "present_note": "Headless throughput entry. WSI acquire/present is covered by Window Lab --performance.",
        "gpu_scope": "gpu_frame_ns is the sum of command timestamp spans per physical queue; pass spans overlap and are reported separately, so they must not be added to frame latency.",
        "cpu_allocation_scope": "CPU allocation counts are test-thread System allocator calls inside begin_frame + graph build + execute_frame; worker/driver allocations are separate.",
        "gpu_zero_span_note": zero_span_note,
        "runs": runs,
        "failure": if failed { Some("readback sample failed, GPU timing was missing, or no sample was taken") } else { None },
    });
    let serialize_started = Instant::now();
    let text = serde_json::to_string_pretty(&report).unwrap();
    let serialize_ns = serialize_started.elapsed().as_nanos() as u64;
    let write_started = Instant::now();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = perf_dir().join(format!("bench-{timestamp}.json"));
    std::fs::write(&path, text).unwrap();
    let write_ns = write_started.elapsed().as_nanos() as u64;
    report["report_serialize_ns"] = serde_json::json!(serialize_ns);
    report["report_write_ns"] = serde_json::json!(write_ns);
    std::fs::write(&path, serde_json::to_string_pretty(&report).unwrap()).unwrap();
    println!(
        "{}",
        serde_json::json!({
            "status": report["status"],
            "report": path,
            "scene_fingerprint": report["scene_fingerprint"],
            "report_serialize_ns": serialize_ns,
            "report_write_ns": write_ns,
        })
    );
    assert!(!failed, "render throughput readback failed: {report}");
}
