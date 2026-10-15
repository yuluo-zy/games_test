//! 多帧渲染动画测试（经由渲染图执行器）：与 `triangle_animation.rs` 相同的动画（48 帧、每帧旋转 15°、
//! 每帧不同清除色、2 帧在飞、逐像素对比 CPU 参考光栅化），但每帧都是一张 `RenderGraph`，交给
//! `VulkanGraphSession` 执行。除像素外还验证执行器的多帧链路：
//! - `FrameThrottle`：在飞帧数峰值 = 2，`begin_frame` 排到只剩 1，超容量的 `execute_frame` 被拒绝；
//! - 帧账本：`begin_frame` 清零、`execute_frame` 记提交并结束，全程没有账本告警；
//! - 帧间资源复用：图内创建的颜色目标走资源池（命中次数断言）；历史纹理走 `TemporalGpuResources`
//!   （parity 翻转、物理纹理恒等、"上一帧的渲染结果"逐字节一致）；
//! - 每帧新建的顶点缓冲在 `collect_after_completion` 后全部销毁；结束后 `wait_idle` 正常。
//! 只走 rhapsody 自己的 rg 接口，不接宿主适配层。没有 Vulkan 设备时跳过。
mod draw_support;
use ash::vk::{self, Handle};
use draw_support::*;
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
        temporal::TemporalRenderGraphState,
        vulkan::{
            AccessTable, GpuResource, ResourceSpec, SessionFrameReceipt, TemporalGpuResources,
            VulkanGraphBackend, VulkanGraphSession,
        },
    },
};
use std::sync::{
    Arc, Weak,
    atomic::{AtomicUsize, Ordering},
};

const FRAMES: usize = 48;
const STEP_DEG: f32 = 15.0;
/// 中途的"缩放/收尾级"排空点：等全部在飞帧完成并 `collect_after_completion`，之后动画继续。
const COLLECT_AT: usize = 24;
const TIMEOUT_NS: u64 = 10_000_000_000;

// 原版访问码（`AccessTable::original()`，见 raw/access_table.rs）。
const VERTEX_READ: u8 = 0x04;
const HOST_READ: u8 = 0x26;
const COLOR_WRITE: u8 = 0x2e;
const TRANSFER_READ: u8 = 0x25;
const TRANSFER_WRITE: u8 = 0x34;
const HOST_WRITE: u8 = 0x35;

// ---- 捕获 `log::warn!`：账本异常只告警不 panic，所以要从日志里断言没有告警 ----
struct Capture;
static WARNINGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
impl log::Log for Capture {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            WARNINGS.lock().push(format!("[{}] {}", record.target(), record.args()));
        }
    }
    fn flush(&self) {}
}
static CAPTURE: Capture = Capture;
fn install_log_capture() {
    let _ = log::set_logger(&CAPTURE);
    log::set_max_level(log::LevelFilter::Warn);
}
fn ledger_warnings() -> Vec<String> {
    WARNINGS.lock().iter().filter(|w| w.contains("账本")).cloned().collect()
}

fn buffer_spec(desc: BufferDesc) -> (ResourceSpec, rhapsody::rg::graph::Descriptor) {
    (ResourceSpec::Buffer(desc), ResourceSpec::Buffer(desc).descriptor())
}

fn extent() -> vk::Extent2D {
    vk::Extent2D { width: SIZE, height: SIZE }
}

fn full_image_copy() -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        mip_level: 0,
        base_array_layer: 0,
        layer_count: 1,
    }
}

/// 读回槽：GPU→CPU 缓冲，跨帧导入（token 固定），每帧导入/导出时的访问码都是 HostRead。
struct Slot {
    token: u64,
    buffer: Arc<Buffer>,
}

struct Harness {
    rig: Rig,
    session: VulkanGraphSession,
    attachments: Arc<Mutex<AttachmentCache>>,
    temporal_state: TemporalRenderGraphState,
    temporal: TemporalGpuResources,
    color_spec: ResourceSpec,
    hist_descriptor: rhapsody::rg::graph::Descriptor,
    readback: Vec<Slot>,
    history_readback: Vec<Slot>,
    /// 每帧新建的顶点缓冲，只留弱引用，用来证明退役后真的被销毁。
    vertex_buffers: Vec<Weak<Buffer>>,
    retired: Arc<AtomicUsize>,
}

/// 一帧提交后的观测值。
struct Submitted {
    receipt: SessionFrameReceipt,
    parity: u32,
    /// 本帧写入的历史纹理 / 读取的上一帧历史纹理的 token。
    hist_cur_token: u64,
    hist_prev_token: u64,
    temporal_reused: [bool; 2],
    color_token: u64,
    /// 颜色目标是否来自资源池（`Snapshot::pool_hits` 含其逻辑下标）。
    color_reused: bool,
    pending_before_execute: usize,
}

fn new_slot(backend: &mut VulkanGraphBackend) -> Slot {
    let desc = BufferDesc::new_gpu_to_cpu(u64::from(SIZE * SIZE * 4), vk::BufferUsageFlags::TRANSFER_DST);
    let buffer = Arc::new(Buffer::new(backend.device.clone(), &desc).unwrap());
    let token = backend.bind_import(GpuResource::Buffer(buffer.clone()));
    Slot { token, buffer }
}

impl Harness {
    fn new() -> Option<Self> {
        let rig = Rig::new()?;
        let backend = VulkanGraphBackend::new(rig.device.clone(), AccessTable::original()).ok()?;
        let mut session = VulkanGraphSession::new(backend);
        // 提交引擎镜像：之后帧完成会回收设备侧的命令缓冲。
        session.attach_graphics_device(rig.gd.clone());
        let attachments = Arc::new(Mutex::new(AttachmentCache::new(rig.device.clone())));
        let mut temporal = TemporalGpuResources::new(rig.device.clone());
        let mut hist = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM);
        hist.usage |= vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
        let hist_descriptor = temporal.register("hist:0", ResourceSpec::Image(hist));
        temporal.register("hist:1", ResourceSpec::Image(hist));
        let color_spec = ResourceSpec::Image(TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM));
        Some(Self {
            rig,
            session,
            attachments,
            temporal_state: TemporalRenderGraphState::default(),
            temporal,
            color_spec,
            hist_descriptor,
            readback: vec![],
            history_readback: vec![],
            vertex_buffers: vec![],
            retired: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// 帧节流点：在飞帧排到只剩最近一个。返回时第 `frame-2` 帧及更早的帧必已完成，
    /// 此刻（`execute_frame` 之前）读它们的读回槽是安全的。
    fn begin(&mut self) -> RenderGraph {
        let graph = self.session.begin_frame().expect("begin_frame");
        assert!(self.session.pending_frame_count() <= 1, "begin_frame 后在飞帧数必须 ≤ 1");
        graph
    }

    /// 在 `begin` 返回的图上构图并 execute_frame。
    fn submit(&mut self, frame: usize, mut graph: RenderGraph) -> Submitted {
        let session = &mut self.session;

        // 1. 历史纹理（temporal 对）：必须在其他 bind_import 之前建立，两边的 token 空间是同一个 u64 域。
        let pair = self
            .temporal_state
            .get_or_create_owned_pair(&mut graph, "hist", self.hist_descriptor, &mut self.temporal)
            .expect("temporal pair");
        for lookup in [pair.first, pair.second] {
            let token = lookup.resource.token;
            self.temporal.import_into(&mut session.backend, &format!("hist-token-{token}"), token).unwrap();
        }
        // pair.first = 本帧写入的历史纹理；pair.second = 上一帧写入的那张。
        let (hist_cur, hist_prev) = (pair.first.handle, pair.second.handle);

        // 2. 读回槽（首帧建立，之后按 frame % 2 复用）。
        if self.readback.is_empty() {
            for _ in 0..2 {
                self.readback.push(new_slot(&mut session.backend));
            }
            for _ in 0..2 {
                self.history_readback.push(new_slot(&mut session.backend));
            }
        }
        let slot = frame % 2;
        let (_, buffer_descriptor) = buffer_spec(BufferDesc::new_gpu_to_cpu(
            u64::from(SIZE * SIZE * 4),
            vk::BufferUsageFlags::TRANSFER_DST,
        ));
        let rb = graph
            .import_unowned(format!("rb{slot}"), buffer_descriptor, self.readback[slot].token, HOST_READ)
            .unwrap();

        // 3. 每帧新建顶点缓冲（y 取反：rg 的 set_default_view_and_scissor 用负高度视口翻转 Y）。
        let triangle = rotated_triangle(frame as f32 * STEP_DEG.to_radians());
        let bytes: Vec<u8> = triangle.iter().flat_map(|v| [v[0], -v[1], v[2], v[3]]).flat_map(f32::to_le_bytes).collect();
        let vb_desc = BufferDesc::new_cpu_to_gpu(bytes.len() as u64, vk::BufferUsageFlags::VERTEX_BUFFER);
        let vertex_buffer = Arc::new(Buffer::new(self.rig.device.clone(), &vb_desc).unwrap());
        vertex_buffer.write_bytes_unsynchronized(0, &bytes).unwrap();
        self.vertex_buffers.push(Arc::downgrade(&vertex_buffer));
        let vb_token = session.backend.bind_import(GpuResource::Buffer(vertex_buffer));
        let vb = graph
            .import_unowned("triangle vb", ResourceSpec::Buffer(vb_desc).descriptor(), vb_token, HOST_WRITE)
            .unwrap();
        let retired = self.retired.clone();
        graph.on_retire(move || {
            retired.fetch_add(1, Ordering::SeqCst);
        });

        // 4. 图内创建的颜色目标（由执行器的资源池分配/复用）。
        let color = session.backend.declare(&mut graph, "color", self.color_spec).unwrap();

        // pass 1：渲染（begin render pass → 管线 → 顶点缓冲 → draw），经 rg 的 RenderPassApi + AttachmentCache。
        {
            let device = self.rig.device.clone();
            let pipeline = self.rig.pipeline.clone();
            let cache = self.attachments.clone();
            let clear = frame_clear_color(frame).map(|c| f32::from(c) / 255.0);
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
                // 初始 stencil 缓存值取 1，使下面的 set_stencil_ref(0) 一定发出（管线的 stencil 引用是动态状态）。
                let mut api = RenderPassApi::new(&mut backend, cb, 1, false);
                api.begin_attachments(&prepared);
                api.set_default_view_and_scissor(extent());
                api.set_stencil_ref(0);
                // rg 的 RenderPassApi 没有 bind_pipeline（原版在 pass 之外绑定管线），这里直接用 ash。
                unsafe { device.device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pipeline.handle) };
                api.bind_vertex_buffer_by_binding_point(0, vertices.raw());
                api.draw(3, 1, 0, 0);
                api.end_render_pass();
                true
            });
        }
        // pass 2：颜色目标 → 本帧读回槽 + 本帧历史纹理。
        {
            let device = self.rig.device.clone();
            let (color_index, rb_index, hist_index) = (color.raw(), rb.raw(), hist_cur.raw());
            let mut pass = graph.add_pass("copy_out");
            pass.add_resource_access(color, TRANSFER_READ, 0).unwrap();
            pass.add_resource_access(rb, TRANSFER_WRITE, 0).unwrap();
            pass.add_resource_access(hist_cur, TRANSFER_WRITE, 0).unwrap();
            pass.render_with_context(move |ctx: PassContext<'_>| {
                let Some(managed) = ctx.managed else { return false };
                let (Ok(texture), Ok(history), Ok(readback)) =
                    (managed.texture_for(color_index), managed.texture_for(hist_index), managed.buffer_for(rb_index))
                else {
                    return false;
                };
                ctx.keep_alive(Arc::new(texture.clone()));
                ctx.keep_alive(Arc::new(history.clone()));
                ctx.keep_alive(Arc::new(readback.clone()));
                let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                unsafe {
                    device.device.cmd_copy_image_to_buffer(
                        cb,
                        texture.raw(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        readback.raw(),
                        &[vk::BufferImageCopy {
                            image_subresource: full_image_copy(),
                            image_extent: vk::Extent3D { width: SIZE, height: SIZE, depth: 1 },
                            ..Default::default()
                        }],
                    );
                    device.device.cmd_copy_image(
                        cb,
                        texture.raw(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        history.raw(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[vk::ImageCopy {
                            src_subresource: full_image_copy(),
                            dst_subresource: full_image_copy(),
                            extent: vk::Extent3D { width: SIZE, height: SIZE, depth: 1 },
                            ..Default::default()
                        }],
                    );
                }
                true
            });
        }
        // pass 3（第 1 帧起）：上一帧写入的历史纹理 → 历史读回槽。
        let hb = (frame > 0).then(|| {
            let hb = graph
                .import_unowned(format!("hb{slot}"), buffer_descriptor, self.history_readback[slot].token, HOST_READ)
                .unwrap();
            let device = self.rig.device.clone();
            let (hist_index, hb_index) = (hist_prev.raw(), hb.raw());
            let mut pass = graph.add_pass("history_readback");
            pass.add_resource_access(hist_prev, TRANSFER_READ, 0).unwrap();
            pass.add_resource_access(hb, TRANSFER_WRITE, 0).unwrap();
            pass.render_with_context(move |ctx: PassContext<'_>| {
                let Some(managed) = ctx.managed else { return false };
                let (Ok(history), Ok(readback)) = (managed.texture_for(hist_index), managed.buffer_for(hb_index)) else {
                    return false;
                };
                ctx.keep_alive(Arc::new(history.clone()));
                ctx.keep_alive(Arc::new(readback.clone()));
                let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
                unsafe {
                    device.device.cmd_copy_image_to_buffer(
                        cb,
                        history.raw(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        readback.raw(),
                        &[vk::BufferImageCopy {
                            image_subresource: full_image_copy(),
                            image_extent: vk::Extent3D { width: SIZE, height: SIZE, depth: 1 },
                            ..Default::default()
                        }],
                    );
                }
                true
            });
            hb
        });
        // 导出：读回缓冲（HostRead，使 GPU 写对主机可见）、temporal 纹理（0x37）。
        graph.export(rb, HOST_READ).unwrap();
        if let Some(hb) = hb {
            graph.export(hb, HOST_READ).unwrap();
        }
        self.temporal_state.export_temporal(&mut graph).unwrap();

        let pending_before_execute = session.pending_frame_count();
        let receipt = session.execute_frame(graph.build()).expect("execute_frame");
        let color_index = color.raw() as usize;
        let mapping = &receipt.snapshot.physical_mapping[color_index];
        for handle in [hist_cur, hist_prev] {
            assert!(receipt.snapshot.exports.contains(&(handle.raw() as u16)), "temporal 纹理必须出现在本帧导出里");
        }
        Submitted {
            parity: pair.parity,
            hist_cur_token: pair.first.resource.token,
            hist_prev_token: pair.second.resource.token,
            temporal_reused: [pair.first.reused, pair.second.reused],
            color_token: mapping.token.expect("颜色目标有物理资源"),
            color_reused: receipt.snapshot.pool_hits.contains(&color.raw()),
            pending_before_execute,
            receipt,
        }
    }

    /// 读回槽内容。调用者必须已确认对应帧完成（`begin_frame` 排空或 `wait_submits_done`）。
    fn read_slot(slot: &Slot) -> Vec<u8> {
        let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];
        // SAFETY: 调用点已等到对应帧的时间线；导出的 HostRead 访问码提供了可见性。
        unsafe { slot.buffer.read_bytes_after_completion(0, &mut pixels).unwrap() };
        pixels
    }
}

#[derive(Default)]
struct Stats {
    frames_checked: usize,
    worst_mismatch: usize,
    worst_centroid_error: f32,
    history_checked: usize,
    previous_pixels: Option<Vec<u8>>,
}

/// 按帧序校验已完成的第 `frame` 帧：像素 + 质心，以及"历史纹理 = 上一帧渲染结果"。
fn verify(harness: &Harness, frame: usize, stats: &mut Stats) {
    let pixels = Harness::read_slot(&harness.readback[frame % 2]);
    let check = check_animation_frame(frame, STEP_DEG, &pixels);
    if frame > 0 {
        let history = Harness::read_slot(&harness.history_readback[frame % 2]);
        assert_eq!(
            Some(&history),
            stats.previous_pixels.as_ref(),
            "第 {frame} 帧经 temporal 纹理读到的上一帧内容与第 {} 帧的渲染结果不一致",
            frame - 1
        );
        stats.history_checked += 1;
    }
    if frame % 6 == 0 {
        println!(
            "第 {frame} 帧（旋转 {:.0}°）三角形像素 {}，不一致 {}，质心误差 {:.2}px",
            frame as f32 * STEP_DEG,
            check.filled,
            check.mismatches,
            check.centroid_error
        );
    }
    stats.frames_checked += 1;
    stats.worst_mismatch = stats.worst_mismatch.max(check.mismatches);
    stats.worst_centroid_error = stats.worst_centroid_error.max(check.centroid_error);
    stats.previous_pixels = Some(pixels);
}

#[test]
fn a_rotating_triangle_renders_correctly_through_the_graph_executor() {
    install_log_capture();
    let Some(mut harness) = Harness::new() else { return };
    let mut stats = Stats::default();
    let mut max_pending = 0;
    let mut submitted: Vec<Submitted> = Vec::new();
    let mut next_verify = 0;

    for frame in 0..FRAMES {
        if frame == COLLECT_AT {
            // 收尾级排空：等所有在飞帧完成，校验，再显式淘汰缓存（temporal 与读回槽的 token 保留）。
            harness.session.wait_submits_done(TIMEOUT_NS).unwrap();
            assert_eq!(harness.session.pending_frame_count(), 0);
            while next_verify < frame {
                verify(&harness, next_verify, &mut stats);
                next_verify += 1;
            }
            let mut retained: Vec<u64> = harness.readback.iter().chain(&harness.history_readback).map(|s| s.token).collect();
            retained.extend([submitted.last().unwrap().hist_cur_token, submitted.last().unwrap().hist_prev_token]);
            harness.attachments.lock().clear();
            let removed = harness.session.collect_after_completion(&retained).unwrap();
            let alive = harness.vertex_buffers.iter().filter(|w| w.upgrade().is_some()).count();
            assert_eq!(alive, 0, "collect_after_completion 之后，此前每帧新建的顶点缓冲必须全部销毁");
            let collected_buffers = harness.vertex_buffers.len();
            // 被淘汰：24 个顶点缓冲 + 池里的颜色目标（>= 2）。
            assert!(removed >= COLLECT_AT + 2, "淘汰数 {removed} 小于预期");
            println!("第 {frame} 帧前排空点：淘汰物理资源 {removed} 个（含 {collected_buffers} 个顶点缓冲）");
        }
        let graph = harness.begin();
        // begin_frame 已把 frame-2 排空：在本帧 execute_frame（会覆盖同一读回槽）之前校验它。
        while next_verify + 2 <= frame {
            verify(&harness, next_verify, &mut stats);
            next_verify += 1;
        }
        let sub = harness.submit(frame, graph);
        // 账本：提交后本帧已结束且记到提交，全程无告警。
        assert!(harness.session.frame_ledger().has_ended());
        assert!(harness.session.frame_ledger().submissions() >= 1);
        assert_eq!(sub.receipt.frame, frame as u64 + 1, "帧回执序号单调递增");
        max_pending = max_pending.max(harness.session.pending_frame_count());
        assert!(harness.session.pending_frame_count() <= 2, "在飞帧数不得超过 FrameThrottle 容量 2");
        submitted.push(sub);
    }
    harness.session.wait_submits_done(TIMEOUT_NS).unwrap();
    assert_eq!(harness.session.pending_frame_count(), 0);
    while next_verify < FRAMES {
        verify(&harness, next_verify, &mut stats);
        next_verify += 1;
    }

    // ---- 多帧行为断言 ----
    assert_eq!(max_pending, 2, "必须真的出现 2 帧同时在飞");
    assert_eq!(stats.frames_checked, FRAMES);
    assert_eq!(stats.history_checked, FRAMES - 1);
    // 每帧 execute_frame 之前的在飞数是 begin_frame 排空后的值：首帧与排空点后的首帧为 0，其余恰为 1。
    let before: Vec<usize> = submitted.iter().map(|s| s.pending_before_execute).collect();
    for (frame, &n) in before.iter().enumerate() {
        let expected = usize::from(frame != 0 && frame != COLLECT_AT);
        assert_eq!(n, expected, "第 {frame} 帧 begin_frame 之后的在飞数：{before:?}");
    }

    // temporal：parity 0,1,0,1…；物理纹理恒等（本帧读的 = 上一帧写的）；首帧新建，其后全部命中。
    for (frame, s) in submitted.iter().enumerate() {
        assert_eq!(s.parity, (frame % 2) as u32, "第 {frame} 帧 parity");
        assert_ne!(s.hist_cur_token, s.hist_prev_token);
        if frame == 0 {
            assert_eq!(s.temporal_reused, [false, false]);
        } else {
            assert_eq!(s.temporal_reused, [true, true], "第 {frame} 帧 temporal 纹理必须复用");
            assert_eq!(s.hist_prev_token, submitted[frame - 1].hist_cur_token, "第 {frame} 帧读的历史纹理必须是上一帧写的那张");
            assert_eq!(s.hist_cur_token, submitted[frame - 1].hist_prev_token, "双缓冲交替");
        }
    }
    let temporal_hits: usize = submitted.iter().map(|s| s.temporal_reused.iter().filter(|&&r| r).count()).sum();
    assert_eq!(temporal_hits, 2 * (FRAMES - 1));
    assert_eq!(harness.temporal.len(), 2, "temporal 物理纹理全程只有 2 张");

    // 资源池：颜色目标图内创建。预热（首帧、次帧）和排空点后的头两帧必然未命中，其余全部命中。
    let pool_reused = submitted.iter().filter(|s| s.color_reused).count();
    let distinct_colors = submitted.iter().map(|s| s.color_token).collect::<std::collections::BTreeSet<_>>();
    println!(
        "池复用命中帧 {pool_reused}/{FRAMES}，颜色目标物理纹理 {} 个，temporal 复用命中 {temporal_hits} 次",
        distinct_colors.len()
    );
    let expected_misses = [0, 1, COLLECT_AT, COLLECT_AT + 1];
    for (frame, s) in submitted.iter().enumerate() {
        assert_eq!(s.color_reused, !expected_misses.contains(&frame), "第 {frame} 帧颜色目标池命中情况");
    }
    assert_eq!(distinct_colors.len(), 4, "2 帧在飞 × 2 段（排空点清池）= 4 张物理颜色纹理");

    // 退役：每帧的 on_retire 回调都已在帧完成时执行。
    assert_eq!(harness.retired.load(Ordering::SeqCst), FRAMES);

    // 收尾：再次淘汰，剩余顶点缓冲也全部销毁；提交引擎镜像里没有遗留命令；设备可空闲。
    harness.attachments.lock().clear();
    let removed = harness.session.collect_after_completion(&[]).unwrap();
    assert!(removed >= FRAMES - COLLECT_AT);
    assert_eq!(harness.vertex_buffers.iter().filter(|w| w.upgrade().is_some()).count(), 0);
    let engine_pending = harness
        .rig
        .gd
        .with_submission_engine(|engine| (engine.state.primary.pending_commands.len(), engine.state.primary.pending_timelines.len()))
        .expect("提交引擎已挂接");
    harness.rig.device.wait_idle().unwrap();
    assert!(!harness.session.is_poisoned());

    println!(
        "共校验 {} 帧；最大在飞帧数 {max_pending}；与参考光栅化最多 {} 个像素不一致；质心最大误差 {:.2} px；\
         历史纹理校验 {} 次；提交引擎遗留 (命令, 时间线) = {engine_pending:?}",
        stats.frames_checked, stats.worst_mismatch, stats.worst_centroid_error, stats.history_checked
    );
    assert!(ledger_warnings().is_empty(), "帧账本出现告警：{:?}", ledger_warnings());
}

/// `FrameThrottle` / `wait_*` / 账本的返回语义，用空 pass 的小图隔离验证（不依赖像素）。
#[test]
fn frame_throttle_and_wait_semantics_of_the_session() {
    install_log_capture();
    let Some(rig) = Rig::new() else { return };
    let backend = VulkanGraphBackend::new(rig.device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);

    // 空会话。
    assert_eq!(session.pending_frame_count(), 0);
    assert_eq!(session.wait_gpu_queue_trim(0), Ok(true), "没有待完成帧：视为已完成");
    assert!(session.wait_frame(1, TIMEOUT_NS).is_err(), "没有待完成帧时 wait_frame 报错");
    session.wait_submits_done(TIMEOUT_NS).unwrap();

    let noop_frame = |session: &mut VulkanGraphSession| {
        let mut graph = session.begin_frame().unwrap();
        assert_eq!(session.frame_ledger().submissions(), 0, "begin_frame 清零账本");
        assert!(!session.frame_ledger().has_ended());
        graph.add_pass("noop").render(|_| true);
        let receipt = session.execute_frame(graph.build()).unwrap();
        assert!(session.frame_ledger().has_ended(), "execute_frame 结束本帧账本");
        assert!(session.frame_ledger().submissions() >= 1, "execute_frame 记提交");
        receipt.frame
    };
    let f1 = noop_frame(&mut session);
    assert_eq!(session.pending_frame_count(), 1);
    let f2 = noop_frame(&mut session);
    assert_eq!(session.pending_frame_count(), 2, "容量 2：第二帧提交后两帧在飞");
    assert_eq!((f1, f2), (1, 2));

    // 超过容量：不 begin_frame（不节流）直接再提交被拒绝，在飞数不增长。
    let rejected = session.execute_frame(RenderGraph::new().build());
    assert!(rejected.is_err(), "2 帧在飞时未经 begin_frame 的 execute_frame 必须被拒绝");
    assert_eq!(session.pending_frame_count(), 2);

    // wait_gpu_queue_trim 只等队首，不弹出；长超时返回 Ok(true)。
    assert_eq!(session.wait_gpu_queue_trim(TIMEOUT_NS), Ok(true));
    assert_eq!(session.pending_frame_count(), 2, "trim 只等待，不改变队列");
    // 0 超时：GPU 是否已完成不确定，但只能是 Ok(_)，不能是 Err。
    assert!(session.wait_gpu_queue_trim(0).is_ok());

    // wait_frame 只接受队首帧。
    assert!(session.wait_frame(f2, TIMEOUT_NS).is_err(), "非队首帧");
    assert_eq!(session.pending_frame_count(), 2, "失败的 wait_frame 不改变队列");
    session.wait_frame(f1, TIMEOUT_NS).unwrap();
    assert_eq!(session.pending_frame_count(), 1);

    // begin_frame 在只剩 1 帧时不等待；再提交后又是 2。
    let f3 = noop_frame(&mut session);
    assert_eq!((session.pending_frame_count(), f3), (2, 3));
    // 再 begin_frame：排到只剩 1（等 f2 完成），而不是无限增长。
    let _graph = session.begin_frame().unwrap();
    assert_eq!(session.pending_frame_count(), 1);
    assert_eq!(session.wait_gpu_queue_trim(TIMEOUT_NS), Ok(true));
    drop(_graph);

    session.wait_submits_done(TIMEOUT_NS).unwrap();
    assert_eq!(session.pending_frame_count(), 0);
    assert_eq!(session.collect_after_completion(&[]).map(|_| ()), Ok(()));
    rig.device.wait_idle().unwrap();
    assert!(ledger_warnings().is_empty(), "帧账本出现告警：{:?}", ledger_warnings());
}
