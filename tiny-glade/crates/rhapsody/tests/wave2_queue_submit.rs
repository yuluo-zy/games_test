//! wave2：`queue_submit`（0x140c04e20）本体流程、setup 命令缓冲槽（dev+0x9f0..0xa4f）、按序退役（0x1418c7ae0）。
//! 纯 CPU 测试，不需要 Vulkan 设备。
use ash::vk;
use rhapsody::raw::{
    graveyard::InOrderRetire,
    submit::{
        Batch, Command, DeviceState, QueueId, QueueState, SetupCommandSlot, SetupTakeError,
        SubmissionEngine, SubmissionInput, SETUP_BARRIER_ACCESS, SETUP_SLOT_NONE,
    },
};
use std::cell::Cell;
use std::collections::VecDeque;

const PRIMARY: QueueId = QueueId { tag: 0, index: 0 };

fn qstate(sem: u64) -> QueueState {
    QueueState {
        handle: vk::Queue::null(),
        timeline_semaphore: ash::vk::Handle::from_raw(sem),
        pending_timelines: VecDeque::new(),
        pending_commands: VecDeque::new(),
    }
}
fn cmd(state: u64, seq: u64) -> Command {
    Command {
        state,
        sequence: seq,
        queue: PRIMARY,
        handle: ash::vk::Handle::from_raw(0x100 + seq),
    }
}
fn engine() -> SubmissionEngine {
    SubmissionEngine::new(DeviceState {
        primary: qstate(0x77),
        auxiliary: vec![],
        next_timeline: 10,
        oldest_pending: None,
    })
}
fn input(commands: Vec<Command>) -> SubmissionInput {
    SubmissionInput {
        flushed_command: None,
        prior: None,
        current: Batch { commands, ..Batch::default() },
    }
}

#[test]
fn setup_slot_none_is_state_4_and_take_returns_none_without_side_effects() {
    let mut slot = SetupCommandSlot::default();
    assert_eq!(slot.raw_state(), SETUP_SLOT_NONE);
    let barrier = Cell::new(false);
    let taken = slot.take_for_submit(|_| barrier.set(true), |_| panic!("not called")).unwrap();
    assert!(taken.is_none());
    assert!(!barrier.get(), "空槽不记录屏障（屏障在 state != 4 分支内）");
}

#[test]
fn setup_take_records_global_barrier_before_state_check_then_ends() {
    let mut slot = SetupCommandSlot::default();
    slot.ensure_recording(|| cmd(0, 1));
    assert_eq!(slot.raw_state(), 1);
    let order = std::cell::RefCell::new(Vec::new());
    let taken = slot
        .take_for_submit(|_| order.borrow_mut().push("barrier"), |_| {
            order.borrow_mut().push("end");
            0
        })
        .unwrap()
        .expect("some");
    assert_eq!(*order.borrow(), ["barrier", "end"]);
    assert_eq!(taken.state, 2);
    assert_eq!(slot.raw_state(), SETUP_SLOT_NONE, "取走后槽置 4");
    assert_eq!(SETUP_BARRIER_ACCESS.as_raw(), 0x18000);
}

#[test]
fn setup_take_wrong_state_still_emits_barrier_then_errors() {
    let mut slot = SetupCommandSlot { command: Some(cmd(2, 1)) };
    let barrier = Cell::new(false);
    let err = slot.take_for_submit(|_| barrier.set(true), |_| 0).unwrap_err();
    assert!(barrier.get());
    assert_eq!(err, SetupTakeError::Unreachable { state: 2 });
    assert_eq!(slot.raw_state(), SETUP_SLOT_NONE);
}

#[test]
fn setup_take_end_failure_is_reported() {
    let mut slot = SetupCommandSlot::default();
    slot.ensure_recording(|| cmd(0, 1));
    let err = slot.take_for_submit(|_| {}, |_| -4).unwrap_err();
    assert_eq!(err, SetupTakeError::End(-4));
}

#[test]
fn in_order_retire_parks_and_drains_in_order() {
    let mut r = InOrderRetire::new();
    assert_eq!(r.retire(1), vec![1]);
    assert!(r.retire(4).is_empty(), "4 > next+1：暂存");
    assert!(r.retire(3).is_empty());
    assert_eq!(r.parked(), 3.min(r.parked()));
    assert_eq!(r.retire(2), vec![2, 3, 4], "补上缺口后按序一次性排出");
    assert_eq!(r.next(), 4);
    assert_eq!(r.parked(), 0);
}

#[test]
#[should_panic]
fn in_order_retire_rejects_already_retired() {
    let mut r = InOrderRetire::new();
    r.retire(1);
    r.retire(1);
}

#[test]
#[should_panic]
fn in_order_retire_rejects_duplicate_parked_slot() {
    let mut r = InOrderRetire::new();
    r.retire(3);
    r.retire(3);
}

#[test]
fn queue_submit_with_setup_orders_setup_first_and_assigns_monotonic_timeline() {
    let mut e = engine();
    let mut seen = Vec::new();
    let (out, _) = e.queue_submit(
        PRIMARY,
        Some(cmd(2, 1)),
        input(vec![cmd(2, 2), cmd(2, 3)]),
        |_| 0,
        |_, prepared| {
            for p in prepared {
                seen.push((p.commands.clone(), p.signal_values.clone()));
            }
            (0, true)
        },
    );
    assert!(out.panic.is_none());
    assert!(out.setup_submitted);
    // 先 setup（值 10），再 current（值 11、12）；每条命令各带一个队列时间线信号。
    assert_eq!(seen[0].1, vec![10]);
    assert_eq!(seen[1].1, vec![11, 12]);
    assert_eq!(out.receipt, Some((PRIMARY, 12)), "回执 = 最后一个时间线值");
    assert_eq!(e.state.next_timeline, 13);
    // 全部命令状态 3，已入环（项带时间线）。
    let q = e.queue(PRIMARY).unwrap();
    assert_eq!(q.pending_commands.len(), 3);
    assert!(q.pending_commands.iter().all(|p| p.command.state == 3));
    assert_eq!(q.pending_commands.iter().map(|p| p.timeline).collect::<Vec<_>>(), [10, 11, 12]);
    assert_eq!(out.in_flight, Some(13 - 10), "事件 0xd：计数 - 最旧待完成值");
}

#[test]
fn engine_queue_submit_updates_before_allocating_and_reclaims_tail_in_order() {
    let mut e = engine();
    // 第一次提交：3 条命令，timeline 10..12。
    e.queue_submit(PRIMARY, None, input(vec![cmd(2, 1), cmd(2, 2), cmd(2, 3)]), |_| 0, |_, _| (0, true));
    // 第二次提交前 GPU 已完成到 11：update 在分配前弹出 10、11，尾部回收出队命令 1、2。
    let (out, rec) = e.queue_submit(PRIMARY, None, input(vec![cmd(2, 4)]), |_| 11, |_, _| (0, true));
    assert_eq!(out.receipt, Some((PRIMARY, 13)));
    assert_eq!(rec.handles.len(), 2);
    assert_eq!(rec.retired_sequences, vec![1, 2]);
    assert_eq!(e.completed_value(PRIMARY), 11);
    // in_flight = 14 - 最旧待完成值(12)
    assert_eq!(out.in_flight, Some(2));
    // 之后完成到 13：回收 3、4，序号连续。
    e.update_pending_submissions(PRIMARY, 13);
    let rec = e.reclaim_command_buffer_resources(PRIMARY);
    assert_eq!(rec.retired_sequences, vec![3, 4]);
}

#[test]
fn queue_submit_failure_does_not_reclaim_and_keeps_registered_state() {
    let mut e = engine();
    let (out, rec) = e.queue_submit(PRIMARY, None, input(vec![cmd(2, 1)]), |_| 0, |_, _| (-4, false));
    assert_eq!(out.panic, Some("backend-error"));
    assert!(rec.handles.is_empty());
    // 注册与状态 3 写入发生在后端调用前：失败不回滚。
    assert_eq!(e.queue(PRIMARY).unwrap().pending_commands.len(), 1);
}
