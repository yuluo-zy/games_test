//! P0 稳定性安全网：长时间跑一张小图（图内创建的纹理被 pass 清除 + 每帧新建并导入的 uniform 缓冲被拷进
//! 图内创建的缓冲），定期 `collect_after_completion`，断言会话内部账本不随帧数增长。
//!
//! 在稳态的同一相位取两组样本比较（热身后 vs 结束前）：一组在两次排空回收之间（有帧在飞、资源池非空），
//! 一组刚做完排空回收。
//! 没有 Vulkan 设备时跳过。
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    init::create_headless_device,
    texture::TextureDesc,
};
use rhapsody::rg::{
    executor::PassContext,
    vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
};
use std::{collections::VecDeque, sync::Arc};

const TRANSFER_READ: u8 = 0x25;
const TRANSFER_WRITE: u8 = 0x34;
const HOST_WRITE: u8 = 0x35;
const TIMEOUT: u64 = 10_000_000_000;
const FRAMES: usize = 1000;
/// 每隔多少帧做一次排空 + `collect_after_completion`（模拟缩放/收尾级回收点）。
const COLLECT_EVERY: usize = 100;
/// 热身后的取样周期；结束前的取样周期是最后一个。
const WARM_CYCLE: usize = 2;
/// 两次回收之间的取样相位。
const MID_PHASE: usize = COLLECT_EVERY / 2;
const UNIFORM_BYTES: u64 = 256;
const SIZE: u32 = 16;

/// 会话内部账本的一次快照。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Bookkeeping {
    retirement_pending: usize,
    pending_frames: usize,
    resources: usize,
    commands: usize,
    pending_commands: usize,
    completed_commands: usize,
    recording_commands: usize,
    submission_captures: usize,
    imports_waiting_release: usize,
}

fn sample(session: &VulkanGraphSession, imports_waiting_release: usize) -> Bookkeeping {
    Bookkeeping {
        retirement_pending: session.backend.device.garbage_stats().pending,
        pending_frames: session.pending_frame_count(),
        resources: session.resource_count(),
        commands: session.command_count(),
        pending_commands: session.chain.pending_command_count(),
        completed_commands: session.chain.completed_command_count(),
        recording_commands: session.chain.recording_command_count(),
        submission_captures: session.backend.submissions.len(),
        imports_waiting_release,
    }
}

fn session() -> Option<VulkanGraphSession> {
    let device = gpu_test_support::initialize("stability device", create_headless_device())?;
    let backend = gpu_test_support::initialize(
        "stability graph backend",
        VulkanGraphBackend::new(device, AccessTable::original()),
    )?;
    Some(VulkanGraphSession::new(backend))
}

/// 一帧：清除图内创建的纹理；把本帧新建并导入的 uniform 缓冲拷进图内创建的缓冲。返回导入 token。
fn run_frame(session: &mut VulkanGraphSession, frame: usize) -> u64 {
    let device = session.backend.device.clone();
    let mut graph = session.begin_frame().expect("begin_frame");

    let mut color_desc = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM);
    color_desc.usage |= vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::TRANSFER_SRC;
    let color = session
        .backend
        .declare(&mut graph, "color", ResourceSpec::Image(color_desc))
        .unwrap();
    let scratch_desc = BufferDesc::new_gpu_only(UNIFORM_BYTES, vk::BufferUsageFlags::TRANSFER_DST);
    let scratch = session
        .backend
        .declare(&mut graph, "scratch", ResourceSpec::Buffer(scratch_desc))
        .unwrap();

    let uniform_desc =
        BufferDesc::new_cpu_to_gpu(UNIFORM_BYTES, vk::BufferUsageFlags::TRANSFER_SRC);
    let uniform = Arc::new(Buffer::new(device.clone(), &uniform_desc).unwrap());
    uniform
        .write_bytes_unsynchronized(0, &[frame as u8; UNIFORM_BYTES as usize])
        .unwrap();
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(uniform))
        .unwrap();
    let ub = graph
        .import_unowned(
            "uniform",
            ResourceSpec::Buffer(uniform_desc).descriptor(),
            token,
            HOST_WRITE,
        )
        .unwrap();

    {
        let raw = device.clone();
        let index = color.raw();
        let value = [frame as f32 / FRAMES as f32, 0.5, 0.25, 1.0];
        let mut pass = graph.add_pass("clear color").with_side_effect();
        pass.add_resource_access(color, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else {
                return false;
            };
            let Ok(image) = managed.texture_for(index) else {
                return false;
            };

            // SAFETY: 命令缓冲处于录制状态；图已把纹理转到 TRANSFER_DST_OPTIMAL，资源由图的实际命令拥有者保活。
            unsafe {
                raw.raw_device().cmd_clear_color_image(
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
    {
        let raw = device.clone();
        let (src, dst) = (ub.raw(), scratch.raw());
        let mut pass = graph.add_pass("upload uniform").with_side_effect();
        pass.add_resource_access(ub, TRANSFER_READ, 0).unwrap();
        pass.add_resource_access(scratch, TRANSFER_WRITE, 0)
            .unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else {
                return false;
            };
            let (Ok(src), Ok(dst)) = (managed.buffer_for(src), managed.buffer_for(dst)) else {
                return false;
            };

            // SAFETY: 同上；两个缓冲都由图声明了相应访问。
            unsafe {
                raw.raw_device().cmd_copy_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    src.raw(),
                    dst.raw(),
                    &[vk::BufferCopy {
                        src_offset: 0,
                        dst_offset: 0,
                        size: UNIFORM_BYTES,
                    }],
                );
            }
            true
        });
    }
    session.execute_frame(graph.build()).expect("execute_frame");
    token
}

/// 释放所有已不被在飞帧引用的导入 token。
fn release_finished_imports(session: &mut VulkanGraphSession, imports: &mut VecDeque<u64>) {
    while let Some(&token) = imports.front() {
        if session.release_import(token).is_err() {
            break;
        }
        imports.pop_front();
    }
}

/// 同一相位的两次样本：热身后与结束前。
struct Samples {
    mid: (Bookkeeping, Bookkeeping),
    collected: (Bookkeeping, Bookkeeping),
}

/// 跑满 `FRAMES` 帧并取样。没有 Vulkan 设备时返回 None。
fn run() -> Option<Samples> {
    let mut session = session()?;
    let mut imports = VecDeque::new();
    let (mut mid, mut collected) = (Vec::new(), Vec::new());
    for frame in 1..=FRAMES {
        release_finished_imports(&mut session, &mut imports);
        imports.push_back(run_frame(&mut session, frame));
        assert!(session.pending_frame_count() <= 2);
        if frame % COLLECT_EVERY == MID_PHASE {
            mid.push(sample(&session, imports.len()));
        }
        if frame % COLLECT_EVERY == 0 {
            session.wait_submits_done(TIMEOUT).unwrap();
            release_finished_imports(&mut session, &mut imports);
            session.collect_after_completion(&[]).unwrap();
            collected.push(sample(&session, imports.len()));
        }
    }
    assert!(!session.is_poisoned());
    let pick = |v: &[Bookkeeping]| (v[WARM_CYCLE - 1], *v.last().unwrap());
    Some(Samples {
        mid: pick(&mid),
        collected: pick(&collected),
    })
}

/// 会话外层账本：资源/命令/在飞帧/托管退役队列/提交捕获在稳态下不增长。
#[test]
fn session_bookkeeping_stays_bounded_over_many_frames() {
    let Some(samples) = run() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    for (warm, end) in [samples.mid, samples.collected] {
        eprintln!("warm={warm:?}\nend ={end:?}");
        assert_bookkeeping_bounded(&warm, &end);
    }
}

fn assert_bookkeeping_bounded(warm: &Bookkeeping, end: &Bookkeeping) {
    let bounded = |field: &str, a: usize, b: usize| assert!(b <= a, "{field} 增长: {a} -> {b}");
    bounded("pending_frames", warm.pending_frames, end.pending_frames);
    bounded("resources", warm.resources, end.resources);
    bounded("commands", warm.commands, end.commands);
    bounded(
        "pending_commands",
        warm.pending_commands,
        end.pending_commands,
    );
    bounded(
        "completed_commands",
        warm.completed_commands,
        end.completed_commands,
    );
    bounded(
        "recording_commands",
        warm.recording_commands,
        end.recording_commands,
    );
    bounded(
        "submission_captures",
        warm.submission_captures,
        end.submission_captures,
    );
    bounded(
        "imports_waiting_release",
        warm.imports_waiting_release,
        end.imports_waiting_release,
    );
}

/// 生命周期资源记录在回收后移除，不能随帧数增长。
#[test]
fn lifecycle_metadata_stays_bounded_over_many_frames() {
    let Some(samples) = run() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    for (warm, end) in [samples.mid, samples.collected] {
        assert!(
            end.retirement_pending <= warm.retirement_pending,
            "lifecycle resources 增长: {} -> {}",
            warm.retirement_pending,
            end.retirement_pending
        );
    }
}
