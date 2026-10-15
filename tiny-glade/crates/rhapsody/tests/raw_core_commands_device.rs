//! raw-core 命令缓冲 / 提交引擎 / 设备创建尾段 / 统计的行为测试（机器码事实见各被测函数文档）。
//! 需要 Vulkan 的用例在无设备时跳过；本机可用 lavapipe。
use ash::vk;
use rhapsody::raw::{
    command_buffer::CommandBufferPool,
    device::GraphicsDevice,
    init::create_headless_device,
    stats::RenderStats,
    submit::{Command, DeviceState, PendingCommand, QueueId, QueueState, SubmissionEngine},
};
use std::collections::VecDeque;
use std::sync::atomic::Ordering;

fn queue(timelines: &[u64], commands: &[(u64, u64)]) -> QueueState {
    QueueState {
        handle: vk::Queue::null(),
        timeline_semaphore: vk::Semaphore::null(),
        pending_timelines: timelines.iter().copied().collect::<VecDeque<_>>(),
        pending_commands: commands
            .iter()
            .map(|&(state, timeline)| PendingCommand {
                command: Command {
                    state,
                    sequence: timeline,
                    queue: QueueId { tag: 0, index: 0 },
                    handle: vk::CommandBuffer::null(),
                },
                timeline,
            })
            .collect(),
    }
}

fn engine(primary: QueueState, aux: Vec<QueueState>) -> SubmissionEngine {
    SubmissionEngine::new(DeviceState {
        primary,
        auxiliary: aux,
        next_timeline: 100,
        oldest_pending: None,
    })
}

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
fn update_pending_pops_from_front_only_and_stops_at_first_greater() {
    // 0x140bf0b90：从队首弹出，遇到第一个 `counter < 值` 就停；`retain` 会错误地弹掉后面的 3。
    let mut e = engine(queue(&[1, 5, 3], &[]), vec![]);
    e.update_pending_submissions(QueueId { tag: 0, index: 0 }, 4);
    assert_eq!(e.state.primary.pending_timelines, VecDeque::from([5, 3]));
    assert_eq!(e.completed_value(QueueId { tag: 0, index: 0 }), 4);
    assert_eq!(e.state.oldest_pending, Some(5), "oldest 取各队列队首的最小值");
}

#[test]
fn update_queue_pending_submissions_walks_primary_then_each_auxiliary() {
    let mut e = engine(queue(&[1, 2], &[]), vec![queue(&[7, 8], &[]), queue(&[2, 9], &[])]);
    let mut seen = Vec::new();
    e.update_queue_pending_submissions_with(|q| {
        let counter = q.pending_timelines.front().copied().unwrap_or(0);
        seen.push(counter);
        counter
    });
    // 顺序：图形、辅助 0、辅助 1；每个队列按自己的计数推进。
    assert_eq!(seen, vec![1, 7, 2]);
    assert_eq!(e.state.primary.pending_timelines, VecDeque::from([2]));
    assert_eq!(e.state.auxiliary[0].pending_timelines, VecDeque::from([8]));
    assert_eq!(e.state.auxiliary[1].pending_timelines, VecDeque::from([9]));
    assert_eq!(e.completed_value(QueueId { tag: 1, index: 1 }), 2);
}

#[test]
fn reclaim_only_pops_front_items_covered_by_the_stored_completed_value() {
    // 完成计数 6：时间线 5 出队，7 留下；前面 3 也出队。
    let mut e = engine(queue(&[], &[(3, 3), (3, 5), (3, 7), (3, 4)]), vec![]);
    let id = QueueId { tag: 0, index: 0 };
    assert!(e.reclaim_queue(id).is_empty(), "未更新完成值（+0x80=0）时什么都不回收");
    e.update_pending_submissions(id, 6);
    assert_eq!(e.reclaim_queue(id).len(), 2);
    // 队首 7 > 6 挡住了后面的 4（与原版 cap-first 环的队首判定一致）。
    assert_eq!(e.state.primary.pending_commands.len(), 2);
}

#[test]
#[should_panic(expected = "entered unreachable code")]
fn reclaim_of_a_command_not_in_submitted_state_is_unreachable() {
    let mut e = engine(queue(&[], &[(2, 1)]), vec![]);
    let id = QueueId { tag: 0, index: 0 };
    e.update_pending_submissions(id, 5);
    e.reclaim_queue(id);
}

#[test]
fn legacy_all_queue_reclaim_merges_the_explicit_completed_value() {
    let mut e = engine(queue(&[], &[(3, 1)]), vec![queue(&[], &[(3, 2)])]);
    // 辅助队列命令的队列标识必须是“辅助”类别。
    e.state.auxiliary[0].pending_commands[0].command.queue = QueueId { tag: 1, index: 0 };
    assert_eq!(e.reclaim_command_buffer_resources_all_queues(2).len(), 2);
}

#[test]
fn command_buffer_pool_starts_empty_per_family_and_bounds_checks() {
    let pool = CommandBufferPool::new(3);
    assert_eq!(pool.family_count(), 3);
    assert!((0..3).all(|f| pool.free_count(f) == 0 && pool.take(f).is_none()));
    let oob = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| pool.take(3)));
    assert!(oob.is_err(), "越界族索引 panic（原版 FUN_1428d9518 越界断言）");
}

#[test]
fn create_command_buffer_reuses_the_recycled_buffer_lifo() {
    let Ok(device) = create_headless_device() else { return };
    let gd = GraphicsDevice::from_device(device);
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
    assert_eq!(gd.free_command_buffers(family), 2);
    // LIFO：先取到最后还回的那个。
    let again = gd.create_command_buffer(family, "c").unwrap();
    assert_eq!(again.handle, h2);
    assert_eq!(gd.free_command_buffers(family), 1);
    assert!(!again.is_recording() && !again.is_executable());
}

#[test]
fn device_micro_functions_and_creation_tail_state() {
    let Ok(device) = create_headless_device() else { return };
    let gd = GraphicsDevice::from_device(device);
    // from_device 路径不跑创建尾段。
    assert!(gd.pipeline_cache().is_none() && gd.crash_tracking_buffer().is_none());
    assert_eq!(gd.queue_family_index_for(0), gd.queue_family_index());
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
fn create_runs_the_device_creation_tail() {
    use rhapsody::raw::init::GraphicsInstance;
    let Ok(graphics) = GraphicsInstance::new() else { return };
    let instance = graphics.instance.clone();
    let Some(physical) = unsafe { instance.instance.enumerate_physical_devices() }
        .ok()
        .and_then(|devices| devices.first().copied())
    else {
        return;
    };
    let family = unsafe { instance.instance.get_physical_device_queue_family_properties(physical) }
        .iter()
        .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS));
    let Some(family) = family else { return };
    // 简化入口启用的扩展在个别驱动上不可用时，创建失败属于环境问题，跳过。
    let gd = match GraphicsDevice::create(instance, physical, family as u32) {
        Ok(gd) => gd,
        Err(error) => {
            eprintln!("skip: GraphicsDevice::create failed: {error:?}");
            return;
        }
    };
    // 尾段 E/G：管线缓存与 4 字节 crash tracking buffer 都已建立。
    assert!(gd.pipeline_cache().is_some());
    let crash = gd.crash_tracking_buffer().expect("crash tracking buffer");
    assert_eq!(crash.size, 4);
}
