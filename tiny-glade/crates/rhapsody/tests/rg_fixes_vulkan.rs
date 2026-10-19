//! rg 缺陷修复的 lavapipe 回归：会话失败路径的资源归还、wait_frame 出错时仍完成清理、提交捕获有界、
//! temporal 导出 + 编译导出校验接入会话帧路径（ping-pong 多帧）。没有 Vulkan 设备时跳过。
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    init::create_headless_device,
    texture::TextureDesc,
};
use rhapsody::rg::{
    executor::PassContext,
    graph::RenderGraph,
    vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const TRANSFER_READ: u8 = 0x25;
const HOST_READ: u8 = 0x26;
const TRANSFER_WRITE: u8 = 0x34;
const TIMEOUT: u64 = 10_000_000_000;
const SIZE: u32 = 4;

fn session() -> Option<VulkanGraphSession> {
    let device = gpu_test_support::initialize("graph regression device", create_headless_device())?;
    let backend = gpu_test_support::initialize(
        "graph regression backend",
        VulkanGraphBackend::new(device, AccessTable::original()),
    )?;
    Some(VulkanGraphSession::new(backend))
}

fn scratch() -> ResourceSpec {
    ResourceSpec::Buffer(BufferDesc::new_gpu_only(
        256,
        vk::BufferUsageFlags::STORAGE_BUFFER,
    ))
}

#[test]
fn recording_failures_and_panics_roll_back_and_allow_the_next_frame() {
    let Some(mut session) = session() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    let mut warm_commands = None;
    for panic_callback in [false, true] {
        for _ in 0..4 {
            let mut graph = session.begin_frame().unwrap();
            let a = session
                .backend
                .declare(&mut graph, "failed", scratch())
                .unwrap();
            let mut pass = graph.add_pass("fail").with_side_effect();
            pass.add_resource_access(a, TRANSFER_WRITE, 0).unwrap();
            pass.render(move |_| {
                assert!(!panic_callback, "injected callback panic");
                false
            });
            let retired = Arc::new(AtomicBool::new(false));
            let flag = retired.clone();
            graph.on_retire(move || flag.store(true, Ordering::SeqCst));
            let error = session.execute_frame(graph.build()).unwrap_err();
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
            assert!(retired.load(Ordering::SeqCst));
            assert!(!session.is_poisoned());
            assert_eq!(session.pending_frame_count(), 0);
            assert!(session.chain.recording_command_count() == 0);
            let (graph, _) = scratch_frame(&mut session, false);
            let receipt = session.execute_frame(graph.build()).unwrap();
            assert_eq!(receipt.stats.commands_recorded as usize, 1);
            session.wait_submits_done(TIMEOUT).unwrap();
            let count = session.command_count();
            assert!(count <= session.backend.device().frames_in_flight() as usize);
            if let Some(warm) = warm_commands {
                assert_eq!(
                    count, warm,
                    "failed recording must reuse its frame pool after warmup"
                );
            } else {
                warm_commands = Some(count);
            }
        }
    }
    assert!(session.backend.command_pool_reset_count() > 0);
}

/// 声明 `a`（被 pass 写入）；`export_only` 时再声明一个只导出、从未被访问的 `b`。
fn scratch_frame(session: &mut VulkanGraphSession, export_only: bool) -> (RenderGraph, u32) {
    let mut graph = session.begin_frame().unwrap();
    let a = session.backend.declare(&mut graph, "a", scratch()).unwrap();
    let mut pass = graph.add_pass("write a").with_side_effect();
    pass.add_resource_access(a, TRANSFER_WRITE, 0).unwrap();
    pass.render(|_| true);
    if export_only {
        let b = session.backend.declare(&mut graph, "b", scratch()).unwrap();
        graph.export(b, HOST_READ).unwrap();
    }
    (graph, a.raw())
}

#[test]
fn a_rejected_export_only_resource_keeps_the_session_usable_and_returns_resources() {
    let Some(mut session) = session() else { return };
    // 新建资源在失败后交还后端。
    let (graph, _) = scratch_frame(&mut session, true);
    let error = session.execute_frame(graph.build()).unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert!(!session.is_poisoned(), "命令开始前的失败不应停止会话");
    assert_eq!(session.resource_count(), 0, "失败帧新建的资源必须释放");
    assert_eq!(session.pending_frame_count(), 0);

    // 正常帧：`a` 新建，完成后进入资源池。
    let (graph, _) = scratch_frame(&mut session, false);
    session.execute_frame(graph.build()).unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
    assert_eq!(session.resource_count(), 1);

    // 失败帧从池中取走 `a`，失败后必须放回。
    let (graph, _) = scratch_frame(&mut session, true);
    assert!(session.execute_frame(graph.build()).is_err());
    let (graph, a) = scratch_frame(&mut session, false);
    let receipt = session.execute_frame(graph.build()).unwrap();
    assert!(
        receipt.snapshot.pool_hits.contains(&a),
        "失败帧取出的池资源必须归还资源池"
    );
    assert_eq!(session.resource_count(), 1);
    session.wait_submits_done(TIMEOUT).unwrap();
}

#[test]
fn wait_frame_retires_resources_and_runs_callbacks() {
    let Some(mut session) = session() else { return };
    let device = session.backend.device().clone();
    let desc = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(Arc::new(
            Buffer::new(device, &desc).unwrap(),
        )))
        .unwrap();
    let (mut graph, _) = scratch_frame(&mut session, false);
    let imported = graph
        .import_unowned(
            "imported",
            ResourceSpec::Buffer(desc).descriptor(),
            token,
            0,
        )
        .unwrap();
    let mut pass = graph.add_pass("touch import").with_side_effect();
    pass.add_resource_access(imported, TRANSFER_WRITE, 0)
        .unwrap();
    pass.render(|_| true);
    let retired = Arc::new(AtomicBool::new(false));
    let flag = retired.clone();
    graph.on_retire(move || flag.store(true, Ordering::SeqCst));
    session.execute_frame(graph.build()).unwrap();
    let commands = session.command_count();

    // Completion retires real command/resource ownership without a second
    // synthetic reference count that callers could corrupt.
    session.wait_submits_done(TIMEOUT).unwrap();

    assert_eq!(
        session.pending_frame_count(),
        0,
        "出错也不能把已完成帧留在队列里"
    );
    assert!(retired.load(Ordering::SeqCst), "退役回调必须执行");
    let (graph, a) = scratch_frame(&mut session, false);
    let receipt = session.execute_frame(graph.build()).unwrap();
    assert!(
        receipt.snapshot.pool_hits.contains(&a),
        "创建的资源必须回到资源池"
    );
    let bound = commands * session.backend.device().frames_in_flight() as usize;
    assert!(session.command_count() <= bound);
    session.wait_submits_done(TIMEOUT).unwrap();
    let warm_commands = session.command_count();
    for _ in 0..session.backend.device().frames_in_flight() * 3 {
        let (graph, _) = scratch_frame(&mut session, false);
        session.execute_frame(graph.build()).unwrap();
        session.wait_submits_done(TIMEOUT).unwrap();
        assert_eq!(
            session.command_count(),
            warm_commands,
            "completed commands must stabilize across every frame slot"
        );
    }
    assert!(session.backend.command_pool_reset_count() > 0);
}

#[test]
fn submission_captures_stay_bounded_without_begin_frame() {
    let Some(mut session) = session() else { return };
    for _ in 0..6 {
        let mut graph = RenderGraph::new();
        graph.add_pass("noop").with_side_effect().render(|_| true);
        session.execute_frame(graph.build()).unwrap();
        session.wait_submits_done(TIMEOUT).unwrap();
    }
    assert!(
        session.backend.submissions.len() <= 1,
        "{}",
        session.backend.submissions.len()
    );
}

fn clear_value(frame: usize) -> [u8; 4] {
    [frame as u8 * 40 + 10, 255 - frame as u8 * 30, 7, 255]
}

#[test]
fn a_temporal_pair_ping_pongs_through_the_session_frame_path() {
    let Some(mut session) = session() else { return };
    let device = session.backend.device().clone();
    let mut desc = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM);
    desc.usage |= vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
    let readback_desc = BufferDesc::new_gpu_to_cpu(
        u64::from(SIZE * SIZE * 4),
        vk::BufferUsageFlags::TRANSFER_DST,
    );
    let mut previous_tokens = None;
    for frame in 0..4 {
        let mut graph = session.begin_frame().unwrap();
        let pair = session.temporal_texture(&mut graph, "hist", desc).unwrap();
        assert_eq!(
            [pair.current_history_valid(), pair.history_valid()],
            [frame >= 2, frame > 0]
        );
        assert_eq!(
            (pair.current_history_valid() && pair.history_valid()),
            frame >= 2
        );
        let tokens = (
            session
                .named_texture_resource(&graph, pair.current())
                .unwrap()
                .raw_handle(),
            session
                .named_texture_resource(&graph, pair.previous())
                .unwrap()
                .raw_handle(),
        );
        if let Some((first, second)) = previous_tokens {
            assert_eq!(tokens, (second, first), "pair 每帧交换");
        }
        previous_tokens = Some(tokens);
        // 写入本帧历史。
        {
            let raw = device.clone();
            let index = pair.current().raw().raw();
            let color = clear_value(frame).map(|c| f32::from(c) / 255.0);
            let mut pass = graph.add_pass("write history").with_side_effect();
            pass.add_resource_access(pair.current().raw(), TRANSFER_WRITE, 0)
                .unwrap();
            pass.render_with_context(move |ctx: PassContext<'_>| {
                assert_eq!(
                    ctx.history_valid(pair.current()).unwrap(),
                    pair.current_history_valid()
                );
                assert_eq!(
                    ctx.history_valid(pair.previous()).unwrap(),
                    pair.history_valid()
                );
                let Some(managed) = ctx.managed else {
                    return false;
                };
                let Ok(image) = managed.texture_for(index) else {
                    return false;
                };

                unsafe {
                    raw.raw_device().cmd_clear_color_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        image.raw(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &vk::ClearColorValue { float32: color },
                        &[vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .level_count(1)
                            .layer_count(1)],
                    );
                }
                true
            });
        }
        // 读回上一帧历史。
        let readback = (frame > 0).then(|| {
            let buffer = Arc::new(Buffer::new(device.clone(), &readback_desc).unwrap());
            let token = session
                .backend
                .bind_import(GpuResource::Buffer(buffer.clone()))
                .unwrap();
            let rb = graph
                .import_unowned(
                    "readback",
                    ResourceSpec::Buffer(readback_desc).descriptor(),
                    token,
                    HOST_READ,
                )
                .unwrap();
            let raw = device.clone();
            let (history, target) = (pair.previous().raw().raw(), rb.raw());
            let mut pass = graph.add_pass("read history").with_side_effect();
            pass.add_resource_access(pair.previous().raw(), TRANSFER_READ, 0)
                .unwrap();
            pass.add_resource_access(rb, TRANSFER_WRITE, 0).unwrap();
            pass.render_with_context(move |ctx: PassContext<'_>| {
                let Some(managed) = ctx.managed else {
                    return false;
                };
                let (Ok(image), Ok(out)) =
                    (managed.texture_for(history), managed.buffer_for(target))
                else {
                    return false;
                };

                unsafe {
                    raw.raw_device().cmd_copy_image_to_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        image.raw(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        out.raw(),
                        &[vk::BufferImageCopy {
                            image_subresource: vk::ImageSubresourceLayers {
                                aspect_mask: vk::ImageAspectFlags::COLOR,
                                mip_level: 0,
                                base_array_layer: 0,
                                layer_count: 1,
                            },
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
            graph.export(rb, HOST_READ).unwrap();
            (buffer, token)
        });
        let receipt = session.execute_frame(graph.build()).unwrap();
        assert!(
            receipt
                .snapshot
                .exports
                .contains(&(pair.current().raw().raw() as u16)),
            "本帧已写历史自动导出"
        );
        assert_eq!(
            receipt
                .snapshot
                .exports
                .contains(&(pair.previous().raw().raw() as u16)),
            frame > 0,
            "尚未写入的首帧 previous 不得发布为可读历史"
        );
        session.wait_submits_done(TIMEOUT).unwrap();
        if let Some((buffer, token)) = readback {
            let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];
            unsafe { buffer.read_bytes_after_completion(0, &mut pixels).unwrap() };
            for pixel in pixels.chunks(4) {
                assert_eq!(
                    pixel,
                    clear_value(frame - 1),
                    "第 {frame} 帧读到的历史必须是上一帧写入的内容"
                );
            }
            session.release_import(token).unwrap();
        }
    }
    // 放弃构图：请求 temporal 后丢弃图，下一帧仍可再次请求。
    let mut graph = session.begin_frame().unwrap();
    session.temporal_texture(&mut graph, "hist", desc).unwrap();
    drop(graph);
    let mut graph = session.begin_frame().unwrap();
    let pair = session.temporal_texture(&mut graph, "hist", desc).unwrap();
    assert!((pair.current_history_valid() && pair.history_valid()));
    graph.add_pass("noop").with_side_effect().render(|_| true);
    session.execute_frame(graph.build()).unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
}

#[test]
fn partial_submission_failure_recycles_all_commands_and_runs_retire_once() {
    let Some(mut session) = session() else { return };
    let mut warm_commands = None;
    for _ in 0..8 {
        let resources_before = session.resource_count();
        let mut graph = session.begin_frame().unwrap();
        graph.batch_per_pass(true);
        for (name, ok) in [("submitted", true), ("abandoned", false)] {
            let buffer = session
                .backend
                .declare(&mut graph, name, scratch())
                .unwrap();
            let mut pass = graph.add_pass(name).with_side_effect();
            pass.add_resource_access(buffer, TRANSFER_WRITE, 0).unwrap();
            pass.render(move |_| ok);
        }
        let retired = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let flag = retired.clone();
        graph.on_retire(move || {
            flag.fetch_add(1, Ordering::SeqCst);
        });
        let error = session.execute_frame(graph.build()).unwrap_err();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        // The first batch entered the GPU. Error cleanup cancels only the
        // second command; retirement belongs to the retained execution until
        // its real completion is observed, even if the GPU finishes quickly.
        assert_eq!(retired.load(Ordering::SeqCst), 0);
        assert_eq!(session.pending_frame_count(), 1);
        assert_eq!(session.chain.pending_command_count(), 1);
        assert_eq!(session.chain.recording_command_count(), 0);
        assert_eq!(session.backend.submissions.len(), 1);
        assert_eq!(session.resource_count(), 2);
        assert!(!session.is_poisoned());

        session.wait_submits_done(TIMEOUT).unwrap();
        assert_eq!(retired.load(Ordering::SeqCst), 1);
        assert_eq!(session.pending_frame_count(), 0);
        assert_eq!(session.chain.pending_command_count(), 0);
        assert_eq!(session.chain.completed_command_count(), 0);
        assert_eq!(session.resource_count(), resources_before);
        session.wait_submits_done(TIMEOUT).unwrap();
        assert_eq!(
            retired.load(Ordering::SeqCst),
            1,
            "retirement cannot run twice"
        );

        let (graph, _) = scratch_frame(&mut session, false);
        session.execute_frame(graph.build()).unwrap();
        session.wait_submits_done(TIMEOUT).unwrap();
        assert_eq!(
            retired.load(Ordering::SeqCst),
            1,
            "the next frame cannot retire an old frame again"
        );
        let count = session.command_count();
        assert!(count <= 2 * session.backend.device().frames_in_flight() as usize);
        if let Some(warm) = warm_commands {
            assert_eq!(
                count, warm,
                "submitted and abandoned commands must stabilize after warming all frame slots"
            );
        } else {
            warm_commands = Some(count);
        }
    }
    assert!(session.backend.command_pool_reset_count() > 0);
}

#[test]
fn two_sessions_share_the_device_submission_timeline() {
    let Some(mut first) = session() else { return };
    let device = first.backend.device().clone();
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut second = VulkanGraphSession::new(backend);
    let mut previous = device
        .timelines()
        .submitted(rhapsody::sync::QueueId::new(0).unwrap())
        .unwrap();
    for index in 0..4 {
        let session = if index % 2 == 0 {
            &mut first
        } else {
            &mut second
        };
        let (graph, _) = scratch_frame(session, false);
        let receipt = session.execute_frame(graph.build()).unwrap();
        let value = receipt.receipts.last().unwrap().value;
        assert_eq!(value, previous + 1);
        previous = value;
        session.wait_submits_done(TIMEOUT).unwrap();
    }
}

#[test]
fn device_lost_is_visible_to_every_session() {
    let Some(mut first) = session() else { return };
    let device = first.backend.device().clone();
    let mut second = VulkanGraphSession::new(
        VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap(),
    );
    // 没有在飞命令时注入终止状态；无需让驱动真的丢失设备。
    device.timelines().mark_device_lost();
    for session in [&mut first, &mut second] {
        assert!(session.is_poisoned());
        assert_eq!(
            session.begin_frame().err().unwrap().kind(),
            rhapsody::ErrorKind::DeviceLost
        );
    }
}

#[test]
fn raw_submission_cannot_signal_the_owned_device_timeline() {
    use rhapsody::raw::command_buffer::VulkanCommandPool;
    let Some(session) = session() else { return };
    let device = session.backend.device().clone();
    let queue = rhapsody::raw::device::DeviceQueue {
        handle: device.primary_queue(),
        family: device.queue_family_index(),
        index: 0,
    };
    let id = device.queue_id(queue.handle).unwrap();
    let previous = device.timelines().submitted(id).unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    command.end().unwrap();
    let result = command.submit_to(
        queue,
        &[],
        &[(session.backend.timeline.clone(), previous + 10)],
    );
    assert!(
        result.is_err(),
        "设备拥有的时间线不能经外部信号数组再次 signal"
    );
    assert_eq!(device.timelines().submitted(id).unwrap(), previous);
}

#[test]
fn named_slots_validate_scope_initialization_resize_and_reset() {
    let Some(mut session) = session() else { return };
    let mut graph = session.begin_frame().unwrap();
    let mut foreign = rhapsody::rg::graph::RenderGraph::new();
    let desc = BufferDesc::new_gpu_only(
        64,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
    );
    assert_eq!(
        session
            .named_buffer(&mut foreign, "slot", desc)
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(session.named_resource_count(), 0);
    // Invalid requests must not permanently consume the bounded name table.
    for attempt in 0..1025 {
        let invalid = TextureDesc::new_2d(0, 4, vk::Format::R8G8B8A8_UNORM);
        assert_eq!(
            session
                .named_texture(&mut graph, &format!("invalid-{attempt}"), invalid)
                .unwrap_err()
                .kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
    }
    let fresh = session.named_buffer(&mut graph, "slot", desc).unwrap();
    assert!(!fresh.history_valid);
    let mut pass = graph.add_pass("undefined named read").with_side_effect();
    pass.add_resource_access(fresh.buffer.raw(), TRANSFER_READ, 0)
        .unwrap();
    pass.render(|_| true);
    assert_eq!(
        session.execute_frame(graph.build()).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert!(session.backend.submissions.is_empty());

    let mut prior_token = None;
    for frame in 0..4 {
        if frame == 2 {
            session.reset_temporal("slot").unwrap();
        }
        let mut graph = session.begin_frame().unwrap();
        let current_desc = if frame == 3 {
            BufferDesc { size: 128, ..desc }
        } else {
            desc
        };
        let named = session
            .named_buffer(&mut graph, "slot", current_desc)
            .unwrap();
        assert_eq!(named.history_valid, frame == 1);
        assert!(
            session.reset_temporal("slot").is_err(),
            "reset must reject an active request"
        );
        let device = session.backend.device().clone();
        let mut pass = graph.add_pass("initialize named slot");
        pass.add_resource_access(named.buffer.raw(), TRANSFER_WRITE, 0)
            .unwrap();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            assert_eq!(ctx.buffer_history_valid(named.buffer)?, named.history_valid);
            let managed = ctx
                .managed
                .ok_or_else(|| rhapsody::Error::invalid_usage("named managed resources absent"))?;
            let buffer = managed
                .buffer_for(named.buffer.raw().raw())
                .map_err(|error| {
                    rhapsody::Error::invalid_usage_with("named buffer resolution", error)
                })?;
            unsafe {
                device.raw_device().cmd_fill_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    buffer.raw(),
                    0,
                    current_desc.size,
                    7,
                );
            }
            Ok(())
        });
        let receipt = session.execute_frame(graph.build()).unwrap();
        let token = receipt.snapshot.physical_mapping[named.buffer.raw().raw() as usize]
            .token
            .unwrap();
        if let Some(prior) = prior_token {
            assert_eq!(
                token == prior,
                frame != 3,
                "reset retains allocation; resize replaces it"
            );
        }
        prior_token = Some(token);
        session.wait_frame(receipt.frame, TIMEOUT).unwrap();
    }
    // Replaced physical owners are removed once their last pending frame completes.
    let _ = session.begin_frame().unwrap();
    assert_eq!(session.named_resource_count(), 1);
    assert_eq!(session.resource_count(), 1);
}

#[test]
fn zeroed_named_buffer_may_be_read_before_its_first_write() {
    let Some(mut session) = session() else { return };
    let device = session.backend.device().clone();
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let mut graph = session.begin_frame().unwrap();
    let desc = BufferDesc::new_gpu_only(
        64,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
    );
    let named = session
        .named_buffer_zeroed(&mut graph, "zeroed", desc)
        .unwrap();
    assert!(!named.history_valid, "zero fill is not frame history");
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(readback.clone()))
        .unwrap();
    let out = graph
        .import_unowned(
            "out",
            ResourceSpec::Buffer(readback.desc).descriptor(),
            token,
            HOST_READ,
        )
        .unwrap();
    let mut pass = graph.add_pass("read zeroed").with_side_effect();
    pass.add_resource_access(named.buffer.raw(), TRANSFER_READ, 0)
        .unwrap();
    pass.add_resource_access(out, TRANSFER_WRITE, 0).unwrap();
    let (source, target) = (named.buffer.raw().raw(), out.raw());
    pass.render_with_result_context(move |ctx: PassContext<'_>| {
        let managed = ctx
            .managed
            .ok_or_else(|| rhapsody::Error::invalid_usage("managed resources absent"))?;
        let source = managed.buffer_for(source).map_err(|error| {
            rhapsody::Error::invalid_usage_with("named buffer resolution", error)
        })?;
        let target = managed
            .buffer_for(target)
            .map_err(|error| rhapsody::Error::invalid_usage_with("readback resolution", error))?;
        unsafe {
            device.raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                source.raw(),
                target.raw(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: 64,
                }],
            );
        }
        Ok(())
    });
    graph.export(out, HOST_READ).unwrap();
    let receipt = session.execute_frame(graph.build()).unwrap();
    session.wait_frame(receipt.frame, TIMEOUT).unwrap();
    let mut bytes = [0xffu8; 64];
    unsafe {
        readback.read_bytes_after_completion(0, &mut bytes).unwrap();
    }
    assert_eq!(bytes, [0; 64]);
}
