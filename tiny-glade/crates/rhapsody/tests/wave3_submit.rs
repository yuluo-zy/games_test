//! wave3：提交路径与设备级剩余未读项的机器码结论（纯 CPU，不需要 Vulkan 设备）。
//! 证据见 `reverse-engineering/tiny-glade/evidence/render-recovery/rhapsody-alignment/reports/wave3-submit.md`。
use ash::vk;
use rhapsody::raw::device::{QueueFamily, QueuePlan};
use rhapsody::raw::submit::{
    Batch, Command, DeviceState, QueueId, QueueState, SubmissionEngine, SubmissionInput,
};
use rhapsody::rg::frame::{
    DEBUG_SYNC_EXTRA_SLEEP, DEBUG_SYNC_HICCUP_MESSAGE, DEBUG_SYNC_WAITING_MESSAGE, debug_sync_wait,
    is_slow_submit,
};
use rhapsody::rg::frame_throttle::FrameThrottle;
use std::collections::VecDeque;
use std::time::Duration;

const PRIMARY: QueueId = QueueId { tag: 0, index: 0 };

fn engine() -> SubmissionEngine {
    SubmissionEngine::new(DeviceState {
        primary: QueueState {
            handle: vk::Queue::null(),
            timeline_semaphore: ash::vk::Handle::from_raw(0x77),
            pending_timelines: VecDeque::new(),
            pending_commands: VecDeque::new(),
        },
        auxiliary: vec![],
        next_timeline: 10,
        oldest_pending: None,
    })
}
fn cmd(seq: u64) -> Command {
    Command {
        state: 2,
        sequence: seq,
        queue: PRIMARY,
        handle: ash::vk::Handle::from_raw(0x100 + seq),
    }
}

/// 0x140c04e20 @0x140c051b6..0x140c05597：小 Vec = [请求记录(prior), setup]，最后才压“当前”记录。
#[test]
fn setup_record_goes_after_prior_and_before_current() {
    let mut e = engine();
    let prior = Batch {
        commands: vec![cmd(1)],
        ..Batch::default()
    };
    let input = SubmissionInput {
        flushed_command: None,
        prior: Some(prior),
        current: Batch {
            commands: vec![cmd(3)],
            ..Batch::default()
        },
    };
    let mut handles = Vec::new();
    let (out, _) = e.queue_submit(
        PRIMARY,
        Some(cmd(2)),
        input,
        |_| 0,
        |_, prepared| {
            handles = prepared.iter().map(|p| p.commands.clone()).collect();
            (0, true)
        },
    );
    assert!(out.panic.is_none());
    let seqs: Vec<u64> = handles
        .iter()
        .flatten()
        .map(|h| ash::vk::Handle::as_raw(*h) - 0x100)
        .collect();
    assert_eq!(seqs, vec![1, 2, 3], "prior, setup, current");
}

/// 0x140bf2e10..0x140bf2e48：第二队列族 = 族属性数组里首个 `flags & 3 == 2`。
#[test]
fn second_queue_family_is_the_first_compute_only_family() {
    let families = [
        QueueFamily {
            flags: 0b111,
            queue_count: 1,
            family_index: 0,
        },
        QueueFamily {
            flags: 0b110,
            queue_count: 4,
            family_index: 1,
        }, // 含 graphics? 0b110 = compute|transfer：flags&3 == 2
        QueueFamily {
            flags: 0b010,
            queue_count: 2,
            family_index: 2,
        },
        QueueFamily {
            flags: 0b011,
            queue_count: 1,
            family_index: 3,
        }, // graphics+compute：不是
    ];
    let plan = QueuePlan::select(&families).unwrap();
    assert_eq!(plan.primary_family, 0);
    assert_eq!(plan.auxiliary_family, Some(1), "首个 flags&3==2");
    assert_eq!(plan.auxiliary_count, 4);
    let none = QueuePlan::select(&families[..1]).unwrap();
    assert_eq!(none.auxiliary_family, None, "dev+0x20 tag = 0 → None");
}

/// 0x140bebffa..0x140bec1f4：慢提交调试同步分支的顺序与阈值（`secs != 0 || nanos >= 20_000_001`）。
#[test]
fn debug_sync_wait_logs_sleeps_only_when_slow_and_always_waits_again() {
    for (elapsed, slow) in [
        (Duration::from_millis(5), false),
        (Duration::from_nanos(20_000_000), false),
        (Duration::from_nanos(20_000_001), true),
        (Duration::from_secs(1), true),
    ] {
        assert_eq!(is_slow_submit(elapsed), slow);
        let events = std::cell::RefCell::new(Vec::<String>::new());
        let log = std::cell::RefCell::new(Vec::<String>::new());
        let result = debug_sync_wait(
            |m| log.borrow_mut().push(m.to_string()),
            || {
                events.borrow_mut().push("wait1".into());
                elapsed
            },
            |d| {
                assert_eq!(d, DEBUG_SYNC_EXTRA_SLEEP);
                events.borrow_mut().push("sleep".into());
            },
            || events.borrow_mut().push("wait2".into()),
        );
        assert_eq!(result, slow);
        let log = log.into_inner();
        let events = events.into_inner();
        assert_eq!(log[0], DEBUG_SYNC_WAITING_MESSAGE);
        if slow {
            assert_eq!(log, [DEBUG_SYNC_WAITING_MESSAGE, DEBUG_SYNC_HICCUP_MESSAGE]);
            assert_eq!(events, ["wait1", "sleep", "wait2"]);
        } else {
            assert_eq!(log.len(), 1);
            assert_eq!(events, ["wait1", "wait2"]);
        }
    }
}

/// 0x140beabd0：begin_frame 承接弹出项的 ArrayVec 容量 2；会话 `execute_frame` 要求在途 <= 1 才提交，所以在途最多 2。
#[test]
fn begin_frame_bounded_drain_matches_the_two_slot_arrayvec() {
    let mut t = FrameThrottle::new();
    t.end_frame(0);
    t.end_frame(1);
    let drained = t.begin_frame_bounded::<()>(|_| Ok(())).unwrap();
    assert_eq!(drained, [0]);
    assert_eq!(t.in_flight(), 1);
    let mut t = FrameThrottle::new();
    for i in 0..4 {
        t.end_frame(i);
    }
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = t.begin_frame_bounded::<()>(|_| Ok(()));
    }));
    assert!(panicked.is_err(), "在途 >= 4：第三次弹出越过 ArrayVec 容量");
}
