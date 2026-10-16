//! 帧级常量、顺序与同步插槽：`GraphExecutor` / `GraphExecutionRecorder` 里与 Vulkan 无关的那一部分。
//!
//! 依据（原 EXE 机器码；`a1-executor` Ghidra 伪代码仅作对照，凡与机器码冲突处以机器码为准）：
//! * `GraphExecutor::new` 0x140bea2d0：`vkCreateQueryPool{TIMESTAMP, count 2}`（device vtable +0x168），
//!   再 `create_buffer("top_level_timing_query_buffer", size 0x10)`；对象 0xd0 字节，布局见 [`EXECUTOR_LAYOUT`]。
//! * `GraphExecutor::begin_frame` 0x140beabd0：profiler scope → 把在途队列排到只剩 1 个（每次
//!   `pop_front` + `wait_submit_done`；承接弹出项的是容量 2 的 ArrayVec，第 3 个会 panic）→ 建首条命令缓冲
//!   `"profiler start"` 并写入 timestamp(0) → 初始化 recorder（见 [`FrameLedger`]）。
//! * `execute_compiled` 0x1400fa190：帧内的"交换链租约"经 `SwapchainLeaseSync::into_sync_data`（0x140c0f040）
//!   取得 (acquire 信号量, 呈现信号量)，写入 recorder 的两个 `Option` 槽（[`SwapchainSyncSlots`]）。
//! * `end_frame` 0x140bec400：`ensure_active_cb`；用 CAS 把提交的时间线值抬到已登记值之上（[`raise_timeline_to`]）；
//!   末条命令缓冲状态必须为 1（录制中）；`vkEndCommandBuffer`/提交后调 `submit_pending_cbs`；
//!   其回执写回 recorder `+0x108`，该处在 `begin_frame` 时为 2（"本帧尚无提交"），`end_frame` 若仍为 2 就 panic；
//!   然后 push 回执入队、`PassPgoFrames` 前后帧交换。
//! * `GraphExecutor::last_known_frame_duration_ns` 0x140beaae0：返回 `Option<u64>`（rax=标志、rdx=值），
//!   **不是** bool——Ghidra 伪代码丢掉了 rdx；调用点 0x1402c22ce 的 `test al,1` 后继续用 rdx 即证据。
//!   `PassPgoStats::duration_ns` 0x140bea1e0 同理：命中时 `mov rdx,[bucket+8]; mov eax,1`，
//!   未命中 `xor eax,eax`；调用方 0x1402c22ce `test al,1; jne … ; xor edx,edx` 即 `unwrap_or(0)`。
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// `vkCreateQueryPool` 的 `queryType=2`（TIMESTAMP）、`queryCount=2`（begin / end 两个时间戳）。
pub const TIMESTAMP_QUERY_COUNT: u32 = 2;
/// 时间戳回读缓冲：两个 u64。`last_known_frame_duration_ns` 对小于 16 字节的映射切片 panic。
pub const TIMESTAMP_BUFFER_BYTES: u64 = 0x10;
/// `create_buffer` 的名字（0x142b14678，长 0x1d）。
pub const TOP_LEVEL_TIMING_QUERY_BUFFER_NAME: &str = "top_level_timing_query_buffer";
/// `begin_frame` 里首条命令缓冲的标签（0x142b14780，长 0xe）。
pub const BEGIN_FRAME_CB_LABEL: &str = "profiler start";
/// `ensure_active_cb` 新建命令缓冲的标签（0x142b14799，长 0xe）。
pub const EXECUTOR_CB_LABEL: &str = "rg executor cb";
/// `begin_frame` 里 gpu profiler 的 marker 名（0x142b1478e，长 0xb）。
pub const TOP_LEVEL_MARKER_NAME: &str = "Frame setup";
/// puffin scope 名（0x142b146b0 / 0x142b146c7）。
pub const SCOPE_WAIT_GPU_QUEUE_TRIM: &str = "Wait for GPU queue trim";
pub const SCOPE_WAIT_PREVIOUS_SUBMITS: &str = "Wait for previous submits";
/// `begin_frame` 排空在途队列时承接弹出项的 ArrayVec 容量（0x140beabd0：`if 1 < n { panic }` 先于存入）。
pub const BEGIN_FRAME_DRAIN_CAPACITY: usize = 2;
/// `submit_pending_cbs`（0x140bebdf0）里 `20000000 < elapsed_ns` 触发慢提交告警的阈值。
pub const SLOW_SUBMIT_THRESHOLD_NS: u64 = 20_000_000;
/// `last_known_frame_duration_ns` 的有效上限：`period × (end−begin) ≥ 1e9` 视为无效（`[0x1429a25a8]` = 1.0e9）。
pub const FRAME_DURATION_VALID_BELOW_NS: f64 = 1.0e9;
/// 交换链 acquire 信号量的等待阶段：`execute_compiled` 写入 `*(u32*)(recorder+0x10) = 0x400`
/// = `COLOR_ATTACHMENT_OUTPUT`。
pub const ACQUIRE_WAIT_STAGE_RAW: u32 = 0x400;

/// `GraphExecutor` 的已读偏移（0xd0 字节）。`+0x48`/`+0x88` 两个 0x40 字节的 `PassPgoStats`
/// 在 `end_frame` 里整块交换；`+0xc8` 是查询池句柄。
pub struct ExecutorLayout {
    pub size: usize,
    pub pending_submissions: usize,
    pub timestamp_buffer: usize,
    pub pgo_a: usize,
    pub pgo_b: usize,
    pub query_pool: usize,
}
pub const EXECUTOR_LAYOUT: ExecutorLayout = ExecutorLayout {
    size: 0xd0,
    pending_submissions: 0x00,
    timestamp_buffer: 0x20,
    pgo_a: 0x48,
    pgo_b: 0x88,
    query_pool: 0xc8,
};

/// `last_known_frame_duration_ns`（0x140beaae0）：读两个时间戳（映射切片 ≥ 16 字节，否则 panic），
/// `period_ns_per_tick (f32) as f64 × (end − begin) as f64`，不小于 1e9 → `None`，否则 `Some(v as u64)`（饱和）。
///
/// `timestamps = None` 对应"缓冲当前没有可读映射"（0x140bd4cd0 返回空）→ `None`。
pub fn last_known_frame_duration_ns(
    timestamps: Option<&[u64]>,
    period_ns_per_tick: f32,
) -> Option<u64> {
    let slice = timestamps?;
    assert!(
        slice.len() * 8 >= TIMESTAMP_BUFFER_BYTES as usize,
        "timestamp readback slice shorter than 16 bytes"
    );
    // 机器码是 `end − begin` 的 u64 回绕减法再转 f64（movq/punpckldq/subpd 这套无符号转换序列）。
    let delta = slice[1].wrapping_sub(slice[0]);
    let nanoseconds = f64::from(period_ns_per_tick) * delta as f64;
    // ucomisd(1e9, v)：v ≥ 1e9 或 NaN 都走 `jbe` → None。
    if nanoseconds.partial_cmp(&FRAME_DURATION_VALID_BELOW_NS) != Some(std::cmp::Ordering::Less) {
        return None;
    }
    // 饱和转换：负数 → 0，超大 → u64::MAX（cvttsd2si + 符号掩码序列，对应 Rust `as`）。
    Some(nanoseconds as u64)
}

/// `submit_pending_cbs` 的慢提交判据。
pub fn is_slow_submit(elapsed: Duration) -> bool {
    elapsed.as_nanos() > u128::from(SLOW_SUBMIT_THRESHOLD_NS)
}

/// `submit_pending_cbs`（0x140bebdf0）调试同步分支的日志目标（长 0x16）与两条 Info 日志（level 常量 3 = Info，
/// 先判 `log` 最大级别 >= 3；文件 `crates/rhapsody/src/rg/executor.rs` 第 598 / 603 行）。
pub const DEBUG_SYNC_LOG_TARGET: &str = "rhapsody::rg::executor";
pub const DEBUG_SYNC_WAITING_MESSAGE: &str = "Waiting for the device to be idle...";
pub const DEBUG_SYNC_HICCUP_MESSAGE: &str =
    "Device idle wait hiccup detected. Suspicious; waiting more.";
/// 慢等待之后额外睡眠：`FUN_140d55980(2, 0)` 是 Windows `std::thread::sleep(Duration::new(2, 0))`
/// （`CreateWaitableTimerExW` + 100ns 刻度乘数 `0x989680`）。
pub const DEBUG_SYNC_EXTRA_SLEEP: Duration = Duration::from_secs(2);

/// `submit_pending_cbs` 调试同步分支（`recorder+0x188 != 0` 时才进入，见 0x140bebffa..0x140bec1f4）：
/// 1. 日志 `Waiting for the device to be idle...`；2. 计时并 `wait_idle`；
/// 3. `elapsed.secs != 0 || elapsed.nanos >= 20_000_001`（`FUN_140d63910` 返回 `Instant::elapsed()` 的秒部分，
///    `edx` 为纳秒；即 [`is_slow_submit`]）则日志 hiccup 并睡 2 秒；4. **无论是否慢**再 `wait_idle` 一次（0x140bec1f4）。
pub fn debug_sync_wait(
    mut log_info: impl FnMut(&str),
    timed_wait_idle: impl FnOnce() -> Duration,
    sleep: impl FnOnce(Duration),
    wait_idle_again: impl FnOnce(),
) -> bool {
    log_info(DEBUG_SYNC_WAITING_MESSAGE);
    let slow = is_slow_submit(timed_wait_idle());
    if slow {
        log_info(DEBUG_SYNC_HICCUP_MESSAGE);
        sleep(DEBUG_SYNC_EXTRA_SLEEP);
    }
    wait_idle_again();
    slow
}

/// `end_frame` 的 CAS 循环（0x140bec400）：`*cell = max(*cell, value)`，返回写入前的值。
pub fn raise_timeline_to(cell: &AtomicU64, value: u64) -> u64 {
    let mut seen = cell.load(Ordering::Relaxed);
    loop {
        let next = seen.max(value);
        match cell.compare_exchange(seen, next, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(previous) => return previous,
            Err(actual) => seen = actual,
        }
    }
}

/// 一帧的阶段，顺序即 `tiny_glade::render_loop::render`（0x1404e8010）里的调用顺序。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrameStage {
    Compile,
    BeginFrame,
    ExecuteMain,
    Retire,
    Acquire,
    ExecutePresentation,
    EndFrame,
    Present,
    WaitGpuQueueTrim,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameOrderError {
    OutOfOrder {
        last: Option<FrameStage>,
        attempted: FrameStage,
    },
    /// 展示图与 present 只在 acquire 成功后才有。
    NotAcquired(FrameStage),
}

/// 帧顺序检查器。可选阶段按原调用点的条件跳过：
/// * `Acquire`/`ExecutePresentation`/`Present`：仅"编译成功且有 swapchain"（0x1404e9bc2）；acquire 返回
///   "跳过"字节（== 7）时整帧不展示、不 present；
/// * `WaitGpuQueueTrim`：仅 `gpu_pacing` 且 `last_known_frame_duration_ns` 为 `Some`（0x1404ea052）。
#[derive(Clone, Debug, Default)]
pub struct FrameSequence {
    last: Option<FrameStage>,
    acquired: bool,
}
impl FrameSequence {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn last(&self) -> Option<FrameStage> {
        self.last
    }
    /// 登记下一阶段。`Acquire` 之后调用 [`Self::set_acquired`] 表示拿到了图像。
    pub fn advance(&mut self, stage: FrameStage) -> Result<(), FrameOrderError> {
        use FrameStage::*;
        let ok = match (self.last, stage) {
            (_, Compile) => true, // 新一帧：任何位置都可重新开始，状态清零
            (Some(prev), s) => s > prev && s != Compile,
            (None, _) => false,
        };
        if !ok {
            return Err(FrameOrderError::OutOfOrder {
                last: self.last,
                attempted: stage,
            });
        }
        // 必经阶段不能被跳过：BeginFrame→ExecuteMain→Retire 与 EndFrame 必须出现。
        if let (Some(prev), s) = (self.last, stage) {
            let must_follow = match s {
                ExecuteMain => prev == BeginFrame,
                Retire => prev == ExecuteMain,
                Acquire => prev == Retire,
                ExecutePresentation => prev == Acquire,
                EndFrame => matches!(prev, Retire | Acquire | ExecutePresentation),
                Present => prev == EndFrame,
                _ => true,
            };
            if !must_follow {
                return Err(FrameOrderError::OutOfOrder {
                    last: self.last,
                    attempted: stage,
                });
            }
        }
        if matches!(stage, ExecutePresentation | Present) && !self.acquired {
            return Err(FrameOrderError::NotAcquired(stage));
        }
        if stage == Compile {
            self.acquired = false;
        }
        self.last = Some(stage);
        Ok(())
    }
    /// acquire 是否拿到了图像（原版 `validate_swapchain_and_acquire_image` 返回字节 != 7）。
    pub fn set_acquired(&mut self, acquired: bool) {
        self.acquired = acquired;
    }
}

/// 一帧内 recorder 的提交状态（`+0x108` 回执标签 + `+0x164` 结束标记）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameLedgerError {
    /// `end_frame` 时 `+0x164` 已为 2：同一帧结束了两次（0x140bec400 `FUN_1428d9310(0x142b148f0)`）。
    EndedTwice,
    /// `end_frame` 里 `submit_pending_cbs` 之后回执标签仍为 2：本帧没有任何提交
    /// （0x140bec400 `FUN_1428d9310(0x142b14908)`）。
    NoSubmission,
}
#[derive(Clone, Debug, Default)]
pub struct FrameLedger {
    submissions: u32,
    ended: bool,
}
impl FrameLedger {
    /// `begin_frame`：`+0x108 = 2`、计数清零。
    pub fn begin(&mut self) {
        *self = Self::default();
    }
    /// `submit_pending_cbs` 成功提交一次后写回回执。
    pub fn note_submission(&mut self) {
        self.submissions += 1;
    }
    pub fn submissions(&self) -> u32 {
        self.submissions
    }
    pub fn has_ended(&self) -> bool {
        self.ended
    }
    /// `end_frame`：先查"已结束"，再查"是否有过提交"。
    pub fn end(&mut self) -> Result<(), FrameLedgerError> {
        if std::mem::replace(&mut self.ended, true) {
            return Err(FrameLedgerError::EndedTwice);
        }
        if self.submissions == 0 {
            return Err(FrameLedgerError::NoSubmission);
        }
        Ok(())
    }
}

/// `execute_compiled` 写入 recorder 的两个 `Option` 槽（`+0x00`/`+0x18`）：
/// `+0x00` 等待（acquire）信号量，阶段固定 [`ACQUIRE_WAIT_STAGE_RAW`]；`+0x18` 呈现信号量。
/// 每个槽在一帧里最多设置一次；再次设置原版 panic（`FUN_1428d9430` 长 0x20 的断言串）。
/// 信号量以不透明 `u64` 句柄表示，具体 `vk::Semaphore` 由调用方持有。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SwapchainSyncSlots {
    acquire_wait: Option<u64>,
    present_signal: Option<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwapchainSyncError {
    AcquireWaitAlreadySet,
    PresentSignalAlreadySet,
}
impl SwapchainSyncSlots {
    pub fn set(&mut self, acquire: u64, present: u64) -> Result<(), SwapchainSyncError> {
        // 原顺序：先写 +0（旧值非空则 panic），再写 +0x18。
        if self.acquire_wait.replace(acquire).is_some() {
            return Err(SwapchainSyncError::AcquireWaitAlreadySet);
        }
        if self.present_signal.replace(present).is_some() {
            return Err(SwapchainSyncError::PresentSignalAlreadySet);
        }
        Ok(())
    }
    pub fn acquire_wait(&self) -> Option<(u64, u32)> {
        self.acquire_wait.map(|s| (s, ACQUIRE_WAIT_STAGE_RAW))
    }
    pub fn present_signal(&self) -> Option<u64> {
        self.present_signal
    }
    /// 提交消费后清空（`submit_pending_cbs` 入口把 `+0x00` 起的状态搬走并清零）。
    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use FrameStage::*;

    #[test]
    fn duration_is_none_over_one_second_and_saturates_below() {
        assert_eq!(last_known_frame_duration_ns(None, 1.0), None);
        assert_eq!(
            last_known_frame_duration_ns(Some(&[100, 1100]), 1.0),
            Some(1000)
        );
        assert_eq!(
            last_known_frame_duration_ns(Some(&[0, 2_000_000_000]), 1.0),
            None
        );
        assert_eq!(
            last_known_frame_duration_ns(Some(&[0, 999_999_999]), 1.0),
            Some(999_999_999)
        );
        // 周期 0.5 ns/tick
        assert_eq!(last_known_frame_duration_ns(Some(&[10, 30]), 0.5), Some(10));
        // end < begin：u64 回绕得到巨大值 → 超过 1e9 → None
        assert_eq!(last_known_frame_duration_ns(Some(&[10, 5]), 1.0), None);
        // 周期为 NaN → None
        assert_eq!(last_known_frame_duration_ns(Some(&[0, 5]), f32::NAN), None);
        // 周期为负：结果为负 → 饱和成 0
        assert_eq!(last_known_frame_duration_ns(Some(&[0, 5]), -1.0), Some(0));
    }
    #[test]
    #[should_panic(expected = "shorter than 16 bytes")]
    fn short_readback_slice_panics() {
        let _ = last_known_frame_duration_ns(Some(&[1]), 1.0);
    }

    #[test]
    fn timeline_raise_is_a_monotonic_max() {
        let cell = AtomicU64::new(5);
        assert_eq!(raise_timeline_to(&cell, 3), 5);
        assert_eq!(cell.load(Ordering::SeqCst), 5);
        assert_eq!(raise_timeline_to(&cell, 9), 5);
        assert_eq!(cell.load(Ordering::SeqCst), 9);
    }

    #[test]
    fn full_frame_order_with_present() {
        let mut f = FrameSequence::new();
        f.advance(Compile).unwrap();
        f.advance(BeginFrame).unwrap();
        f.advance(ExecuteMain).unwrap();
        f.advance(Retire).unwrap();
        f.advance(Acquire).unwrap();
        f.set_acquired(true);
        f.advance(ExecutePresentation).unwrap();
        f.advance(EndFrame).unwrap();
        f.advance(Present).unwrap();
        f.advance(WaitGpuQueueTrim).unwrap();
        // 下一帧
        f.advance(Compile).unwrap();
    }
    #[test]
    fn frames_without_an_image_skip_presentation() {
        let mut f = FrameSequence::new();
        for s in [Compile, BeginFrame, ExecuteMain, Retire, Acquire] {
            f.advance(s).unwrap();
        }
        f.set_acquired(false);
        assert_eq!(
            f.advance(ExecutePresentation),
            Err(FrameOrderError::NotAcquired(ExecutePresentation))
        );
        f.advance(EndFrame).unwrap();
        assert!(f.advance(Present).is_err());
    }
    #[test]
    fn oom_retry_frames_skip_acquire_entirely() {
        // 世界图首次编译失败降分辨率重试：不 acquire、不 present（0x1404e9bc2 的 `!primary_ok`）
        let mut f = FrameSequence::new();
        for s in [Compile, BeginFrame, ExecuteMain, Retire, EndFrame] {
            f.advance(s).unwrap();
        }
    }
    #[test]
    fn order_violations_are_rejected() {
        let mut f = FrameSequence::new();
        assert!(f.advance(BeginFrame).is_err());
        f.advance(Compile).unwrap();
        assert!(f.advance(ExecuteMain).is_err());
        f.advance(BeginFrame).unwrap();
        assert!(f.advance(Retire).is_err());
        f.advance(ExecuteMain).unwrap();
        assert!(f.advance(EndFrame).is_err());
        f.advance(Retire).unwrap();
        f.advance(EndFrame).unwrap();
        assert!(f.advance(EndFrame).is_err());
    }

    #[test]
    fn ledger_requires_a_submission_and_rejects_double_end() {
        let mut l = FrameLedger::default();
        l.begin();
        assert_eq!(l.end(), Err(FrameLedgerError::NoSubmission));
        assert_eq!(l.end(), Err(FrameLedgerError::EndedTwice));
        l.begin();
        l.note_submission();
        assert_eq!(l.end(), Ok(()));
        assert_eq!(l.end(), Err(FrameLedgerError::EndedTwice));
    }

    #[test]
    fn sync_slots_are_set_once_per_frame_with_the_color_output_stage() {
        let mut s = SwapchainSyncSlots::default();
        s.set(1, 2).unwrap();
        assert_eq!(s.acquire_wait(), Some((1, 0x400)));
        assert_eq!(s.present_signal(), Some(2));
        assert_eq!(s.set(3, 4), Err(SwapchainSyncError::AcquireWaitAlreadySet));
        let taken = s.take();
        assert_eq!(taken.acquire_wait().unwrap().0, 3);
        s.set(5, 6).unwrap();
    }

    #[test]
    fn slow_submit_threshold_is_strictly_greater_than_20ms() {
        assert!(!is_slow_submit(Duration::from_millis(20)));
        assert!(is_slow_submit(Duration::from_nanos(20_000_001)));
    }

    #[test]
    fn executor_layout_offsets_match_the_machine_code() {
        assert_eq!(EXECUTOR_LAYOUT.pgo_b - EXECUTOR_LAYOUT.pgo_a, 0x40);
        assert_eq!(EXECUTOR_LAYOUT.query_pool + 8, 0xd0);
    }
}
