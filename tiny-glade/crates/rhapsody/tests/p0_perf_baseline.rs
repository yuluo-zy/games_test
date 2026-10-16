//! P0 性能基线（`#[ignore]`，手动运行）：用一张有代表性的多 pass 图跑若干帧，打印每帧 CPU 墙钟
//! （begin_frame + 构图 + execute_frame）的 mean/p50/p95，以及每帧命令缓冲数、屏障数、提交数。
//!
//! 运行：`cargo test -p rhapsody --test p0_perf_baseline -- --ignored --nocapture`（可加 `--release`）。
//! 图：clear 纹理 A → fill 缓冲 B → 光栅画三角形到图内颜色目标（每帧新建并导入顶点缓冲）→
//! 颜色目标拷到 A → A 拷到 B → B 拷到常驻读回缓冲（导出 HostRead）。没有 Vulkan 设备时跳过。
mod draw_support;
use ash::vk::{self, Handle};
use draw_support::{rotated_triangle, Rig, SIZE};
use parking_lot::Mutex;
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        texture::TextureDesc,
    },
    rg::{
        attachments::{Attachment, AttachmentCache},
        executor::PassContext,
        graph::RenderGraph,
        pass_api::{AshCommandBackend, RenderPassApi},
        resource::ResourceHandle,
        vulkan::{
            AccessTable, GpuResource, ResourceSpec, SessionFrameReceipt, VulkanGraphBackend,
            VulkanGraphSession,
        },
    },
};
use std::{
    collections::VecDeque,
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

fn extent() -> vk::Extent2D {
    vk::Extent2D { width: SIZE, height: SIZE }
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
    readback_token: u64,
    imports: VecDeque<u64>,
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
        let rig = Rig::new()?;
        let mut backend = VulkanGraphBackend::new(rig.device.clone(), AccessTable::original()).ok()?;
        let readback_desc = BufferDesc::new_gpu_to_cpu(IMAGE_BYTES, vk::BufferUsageFlags::TRANSFER_DST);
        let readback = Arc::new(Buffer::new(rig.device.clone(), &readback_desc).unwrap());
        let readback_token = backend.bind_import(GpuResource::Buffer(readback));
        let session = VulkanGraphSession::new(backend);
        let attachments = Arc::new(Mutex::new(AttachmentCache::new(rig.device.clone())));
        Some(Self { rig, session, attachments, readback_token, imports: VecDeque::new() })
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
        self.release_finished_imports();
        let start = Instant::now();
        let graph = self.session.begin_frame().expect("begin_frame");
        let receipt = self.build_and_execute(graph, frame);
        (start.elapsed(), receipt)
    }

    fn build_and_execute(&mut self, mut graph: RenderGraph, frame: usize) -> SessionFrameReceipt {
        let device = self.rig.device.clone();
        let backend = &mut self.session.backend;
        let mut a_desc = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM);
        a_desc.usage |= vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::TRANSFER_SRC;
        let a = backend.declare(&mut graph, "a", ResourceSpec::Image(a_desc)).unwrap();
        let b_desc = BufferDesc::new_gpu_only(
            IMAGE_BYTES,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
        );
        let b = backend.declare(&mut graph, "b", ResourceSpec::Buffer(b_desc)).unwrap();
        let mut color_desc = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM).color_attachment();
        color_desc.usage |= vk::ImageUsageFlags::TRANSFER_SRC;
        let color = backend.declare(&mut graph, "color", ResourceSpec::Image(color_desc)).unwrap();

        let vertices: Vec<u8> = rotated_triangle(frame as f32 * 0.1)
            .iter()
            .flat_map(|v| [v[0], -v[1], v[2], v[3]])
            .flat_map(f32::to_le_bytes)
            .collect();
        let vb_desc = BufferDesc::new_cpu_to_gpu(vertices.len() as u64, vk::BufferUsageFlags::VERTEX_BUFFER);
        let vb_buffer = Arc::new(Buffer::new(device.clone(), &vb_desc).unwrap());
        vb_buffer.write_bytes_unsynchronized(0, &vertices).unwrap();
        let vb_token = backend.bind_import(GpuResource::Buffer(vb_buffer));
        self.imports.push_back(vb_token);
        let vb = graph
            .import_unowned("vb", ResourceSpec::Buffer(vb_desc).descriptor(), vb_token, HOST_WRITE)
            .unwrap();
        let rb_desc = BufferDesc::new_gpu_to_cpu(IMAGE_BYTES, vk::BufferUsageFlags::TRANSFER_DST);
        let rb = graph
            .import_unowned("readback", ResourceSpec::Buffer(rb_desc).descriptor(), self.readback_token, HOST_READ)
            .unwrap();

        add_clear(&mut graph, &device, a, frame);
        add_fill(&mut graph, &device, b, frame as u32);
        self.add_triangle(&mut graph, vb, color, frame);
        add_copy_image(&mut graph, &device, color, a);
        add_copy_image_to_buffer(&mut graph, &device, a, b);
        add_copy_buffer(&mut graph, &device, b, rb);
        graph.export(rb, HOST_READ).unwrap();
        self.session.execute_frame(graph.build()).expect("execute_frame")
    }

    fn add_triangle(&self, graph: &mut RenderGraph, vb: ResourceHandle, color: ResourceHandle, frame: usize) {
        let device = self.rig.device.clone();
        let pipeline = self.rig.pipeline.clone();
        let cache = self.attachments.clone();
        let clear = [(frame % 255) as f32 / 255.0, 0.2, 0.3, 1.0];
        let (color_index, vb_index) = (color.raw(), vb.raw());
        let mut pass = graph.add_pass("triangle");
        pass.add_resource_access(vb, VERTEX_READ, 0).unwrap();
        pass.add_resource_access(color, COLOR_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else { return false };
            let (Ok(texture), Ok(vertices)) = (managed.texture_for(color_index), managed.buffer_for(vb_index)) else {
                return false;
            };
            let attachment = Attachment::color(texture.shared(), 0, Some(clear));
            let Ok(prepared) = cache.lock().prepare(&[attachment], extent(), 1) else { return false };
            ctx.keep_alive(prepared.framebuffer.clone());
            ctx.keep_alive(pipeline.clone());
            ctx.keep_alive(Arc::new(vertices.clone()));
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            // SAFETY: 命令缓冲处于录制状态，资源由 keep_alive / 执行器保活到执行完成。
            let mut backend = unsafe { AshCommandBackend::new(&device.device) };
            let mut api = RenderPassApi::new(&mut backend, cb, 1, false);
            api.begin_attachments(&prepared);
            api.set_default_view_and_scissor(extent());
            api.set_stencil_ref(0);
            // SAFETY: 同上；管线与 render pass 兼容。
            unsafe { device.device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pipeline.handle) };
            api.bind_vertex_buffer_by_binding_point(0, vertices.raw());
            api.draw(3, 1, 0, 0);
            api.end_render_pass();
            true
        });
    }
}

type Device = Arc<rhapsody::raw::device::VulkanDevice>;

fn add_clear(graph: &mut RenderGraph, device: &Device, target: ResourceHandle, frame: usize) {
    let raw = device.clone();
    let index = target.raw();
    let value = [(frame % 255) as f32 / 255.0, 0.5, 0.25, 1.0];
    let mut pass = graph.add_pass("clear a");
    pass.add_resource_access(target, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else { return false };
        let Ok(image) = managed.texture_for(index) else { return false };
        ctx.keep_alive(Arc::new(image.clone()));
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
    let mut pass = graph.add_pass("fill b");
    pass.add_resource_access(target, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else { return false };
        let Ok(buffer) = managed.buffer_for(index) else { return false };
        ctx.keep_alive(Arc::new(buffer.clone()));
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

fn add_copy_image(graph: &mut RenderGraph, device: &Device, src: ResourceHandle, dst: ResourceHandle) {
    let raw = device.clone();
    let (s, d) = (src.raw(), dst.raw());
    let mut pass = graph.add_pass("color to a");
    pass.add_resource_access(src, TRANSFER_READ, 0).unwrap();
    pass.add_resource_access(dst, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else { return false };
        let (Ok(src), Ok(dst)) = (managed.texture_for(s), managed.texture_for(d)) else { return false };
        ctx.keep_alive(Arc::new(src.clone()));
        ctx.keep_alive(Arc::new(dst.clone()));
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
                    extent: vk::Extent3D { width: SIZE, height: SIZE, depth: 1 },
                    ..Default::default()
                }],
            );
        }
        true
    });
}

fn add_copy_image_to_buffer(graph: &mut RenderGraph, device: &Device, src: ResourceHandle, dst: ResourceHandle) {
    let raw = device.clone();
    let (s, d) = (src.raw(), dst.raw());
    let mut pass = graph.add_pass("a to b");
    pass.add_resource_access(src, TRANSFER_READ, 0).unwrap();
    pass.add_resource_access(dst, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else { return false };
        let (Ok(src), Ok(dst)) = (managed.texture_for(s), managed.buffer_for(d)) else { return false };
        ctx.keep_alive(Arc::new(src.clone()));
        ctx.keep_alive(Arc::new(dst.clone()));
        // SAFETY: 同上。
        unsafe {
            raw.device.cmd_copy_image_to_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                src.raw(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst.raw(),
                &[vk::BufferImageCopy {
                    image_subresource: color_layers(),
                    image_extent: vk::Extent3D { width: SIZE, height: SIZE, depth: 1 },
                    ..Default::default()
                }],
            );
        }
        true
    });
}

fn add_copy_buffer(graph: &mut RenderGraph, device: &Device, src: ResourceHandle, dst: ResourceHandle) {
    let raw = device.clone();
    let (s, d) = (src.raw(), dst.raw());
    let mut pass = graph.add_pass("b to readback");
    pass.add_resource_access(src, TRANSFER_READ, 0).unwrap();
    pass.add_resource_access(dst, TRANSFER_WRITE, 0).unwrap();
    pass.render_with_context(move |ctx: PassContext<'_>| {
        let Some(managed) = ctx.managed else { return false };
        let (Ok(src), Ok(dst)) = (managed.buffer_for(s), managed.buffer_for(d)) else { return false };
        ctx.keep_alive(Arc::new(src.clone()));
        ctx.keep_alive(Arc::new(dst.clone()));
        // SAFETY: 同上。
        unsafe {
            raw.device.cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                src.raw(),
                dst.raw(),
                &[vk::BufferCopy { src_offset: 0, dst_offset: 0, size: IMAGE_BYTES }],
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
            commands: receipt.command_sequences.len(),
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
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    println!("p0 基线（{profile}，{} 帧，丢弃前 {WARMUP} 帧热身，6 pass）", FRAMES);
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
