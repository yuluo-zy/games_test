//! 实际句柄控制块及原指令规则；不保留另一套影子资源/队列 EventOracle。
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU64, Ordering};

/// 原 `ManagedResource<T>` 的头部（槽 = 控制块，`AppendOnlyVec` 元素 24 B 起）：
/// `+0x00 AtomicU64 seq`、`+0x08 AtomicI32 refcount`、`+0x0c u32 checksum`，之后（`+0x10`）才是内联的资源（空闲时
/// 兼作 free-list next）。`clone` 0x140bd6b00 `lock inc dword [ctl+8]`、`drop` 0x140be24b0 `lock dec dword [ctl+8]`、
/// `track` 0x140bd6a70 读 `[ctl+0xc]` 并对 `[ctl]` 做 `fetch_max`。资源本体由 `ManagedInner::resource` 承载。
#[repr(C)]
#[derive(Debug)]
pub struct ControlBlock {
    pub last_sequence: AtomicU64,
    /// 原 refcount 是 `AtomicI32`（`lock inc/dec dword [inner+8]`，Clone=fetch_add、Drop=fetch_sub）。
    pub references: AtomicI32,
    /// 原 `resource_checksum`：槽位复用时递增。原子量使控制块可无锁共享（`Arc<ControlBlock>`）。
    pub resource_checksum: AtomicU32,
}
impl ControlBlock {
    pub fn new(resource_checksum: u32) -> Self {
        Self {
            last_sequence: AtomicU64::new(0),
            references: AtomicI32::new(1),
            resource_checksum: AtomicU32::new(resource_checksum),
        }
    }
    pub fn checksum(&self) -> u32 {
        self.resource_checksum.load(Ordering::SeqCst)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandState {
    pub tag: u32,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackOutcome {
    Updated,
    GenerationSkipped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    InvalidCommandState,
    MovedHandleDrop,
    DuplicateBucket,
    MissingBucket,
    DuplicateCompletion,
    UnknownHandle,
    CrossDomainMappingUnknown,
    TimelineRegression,
    DeviceMismatch,
    ResourceKindMismatch,
    DisposalGenerationMismatch,
    ReferencesRemain,
}

/// 原 panic 文本前缀（`RawManagedHandle::track` 0x140bd6a70 的 `fmt::Arguments` 单个静态片段 0x142b14bb0，51 B；
/// 后接 1 个参数：`CommandState` 的 Debug，函数 0x140bed620）。
pub const INVALID_COMMAND_STATE_MSG: &str = "Can't use a resource for a command buffer in state ";

/// 原 `CommandState` 的 Debug 文本（0x140bed620：tag0 `Initial`，tag1 `Recording{queue,seq}`，tag2 `Recorded{..}`，
/// tag3 `Submitted{..}`；tag 4..7 在原跳表里不可达）。本投影只携带 `seq`，没有恢复出的 `queue` 字段，
/// 因此 `queue` 不输出（⚠️ 与原文案相比缺 `queue: ..`）。
pub fn command_state_debug(command: CommandState) -> String {
    match command.tag {
        0 => "Initial".to_string(),
        1 => format!("Recording {{ seq: {} }}", command.sequence),
        2 => format!("Recorded {{ seq: {} }}", command.sequence),
        3 => format!("Submitted {{ seq: {} }}", command.sequence),
        other => format!("<invalid state tag {other}>"),
    }
}

/// 原版在命令缓冲状态 != Recording(1) 时的 panic（`track` 与 `CommandBufferSeqTracker::track` 共用文案）。
#[cold]
#[track_caller]
pub fn panic_invalid_command_state(command: CommandState) -> ! {
    panic!("{INVALID_COMMAND_STATE_MSG}{}", command_state_debug(command))
}

/// 原 `CommandBufferSeqTracker::track`/`RawManagedHandle::track` 的 `fetch_max`：tag != 1 → panic。
pub fn track_sequence(tracker: &AtomicU64, command: CommandState) {
    if try_track_sequence(tracker, command).is_err() {
        panic_invalid_command_state(command);
    }
}

/// `track_sequence` 的非 panic 兼容入口。
pub fn try_track_sequence(tracker: &AtomicU64, command: CommandState) -> Result<(), Failure> {
    if command.tag != 1 {
        return Err(Failure::InvalidCommandState);
    }
    tracker.fetch_max(command.sequence, Ordering::SeqCst);
    Ok(())
}

/// 原 `RawManagedHandle::track`（0x140bd6a70）：校验和不等 → 静默跳过；tag != 1 → panic；否则 `fetch_max`。
pub fn track_handle(
    control: &ControlBlock,
    handle_generation: u32,
    command: CommandState,
) -> TrackOutcome {
    if control.checksum() != handle_generation {
        return TrackOutcome::GenerationSkipped;
    }
    track_sequence(&control.last_sequence, command);
    TrackOutcome::Updated
}

/// `track_handle` 的非 panic 兼容入口（tag != 1 → `Err(InvalidCommandState)`）。
pub fn try_track_handle(
    control: &ControlBlock,
    handle_generation: u32,
    command: CommandState,
) -> Result<TrackOutcome, Failure> {
    if control.checksum() != handle_generation {
        return Ok(TrackOutcome::GenerationSkipped);
    }
    try_track_sequence(&control.last_sequence, command)?;
    Ok(TrackOutcome::Updated)
}

pub fn clone_reference(control: &ControlBlock) -> u32 {
    control
        .references
        .fetch_add(1, Ordering::SeqCst)
        .wrapping_add(1) as u32
}

/// 仅当引用数非零且校验和仍匹配时克隆引用，返回是否成功。
///
/// 原先靠控制块互斥锁保证“检查 → 克隆”与退役（检查引用为 0 → 递增校验和）互斥；
/// 现在用 CAS：引用数为 0 时克隆失败，所以进入退役的句柄不会被复活；
/// 克隆后再复核校验和，不匹配则撤销。
pub fn clone_if_live(control: &ControlBlock, expected_checksum: u32) -> bool {
    if control
        .references
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |refs| {
            (refs != 0).then_some(refs.wrapping_add(1))
        })
        .is_err()
    {
        return false;
    }
    if control.checksum() != expected_checksum {
        control.references.fetch_sub(1, Ordering::SeqCst);
        return false;
    }
    true
}

pub fn drop_reference(control: &ControlBlock, moved: bool) -> Result<bool, Failure> {
    if moved {
        return Err(Failure::MovedHandleDrop);
    }
    Ok(control
        .references
        .fetch_sub(1, Ordering::SeqCst)
        .wrapping_sub(1)
        == 0)
}

pub fn dispose_control(
    control: &ControlBlock,
    handle_generation: u32,
    handle_device: u16,
    device: u16,
    resource_kind: u8,
    mut external_dispose: impl FnMut(),
) -> Result<(), Failure> {
    dispose_control_kind(
        control,
        handle_generation,
        handle_device,
        device,
        resource_kind,
        1,
        || {
            external_dispose();
            Ok(())
        },
    )
}

/// 七类 typed store 在同一 consumer 中校验各自原 tag；回调只转移拥有权入桶。
#[allow(clippy::too_many_arguments)]
pub fn dispose_control_kind(
    control: &ControlBlock,
    handle_generation: u32,
    handle_device: u16,
    device: u16,
    resource_kind: u8,
    expected_kind: u8,
    mut enqueue_destroy: impl FnMut() -> Result<(), Failure>,
) -> Result<(), Failure> {
    if handle_device != device {
        return Err(Failure::DeviceMismatch);
    }
    if resource_kind != expected_kind {
        return Err(Failure::ResourceKindMismatch);
    }
    if control.checksum() != handle_generation {
        return Err(Failure::DisposalGenerationMismatch);
    }
    enqueue_destroy()?;
    if control.references.load(Ordering::SeqCst) != 0 {
        return Err(Failure::ReferencesRemain);
    }
    // 校验和只由退役递增；CAS 保证与并发的重复退役互斥，失败说明已被他人递增。
    control
        .resource_checksum
        .compare_exchange(
            handle_generation,
            handle_generation.wrapping_add(1),
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .map_err(|_| Failure::DisposalGenerationMismatch)?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommandKey {
    pub domain: u32,
    pub sequence: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    #[test]
    fn control_block_header_matches_original_slot_layout() {
        assert_eq!(offset_of!(ControlBlock, last_sequence), 0x00);
        assert_eq!(offset_of!(ControlBlock, references), 0x08);
        assert_eq!(offset_of!(ControlBlock, resource_checksum), 0x0c);
        assert_eq!(size_of::<ControlBlock>(), 0x10);
        // 新槽 `{seq=0, refcount=1, checksum=0}`（allocate push 的 `{0,1,resource}` 元素）。
        let c = ControlBlock::new(0);
        assert_eq!((c.last_sequence.load(Ordering::SeqCst), c.references.load(Ordering::SeqCst), c.checksum()), (0, 1, 0));
    }

    /// 0x140bd6a70：校验和不等 → 静默跳过；tag != 1 → panic；否则 `fetch_max`。
    #[test]
    fn track_follows_original_decision_order() {
        let c = ControlBlock::new(5);
        let ok = |sequence| CommandState { tag: 1, sequence };
        assert_eq!(track_handle(&c, 4, ok(9)), TrackOutcome::GenerationSkipped);
        assert_eq!(c.last_sequence.load(Ordering::SeqCst), 0);
        // 校验和不等时连 tag 也不看（先比校验和）。
        assert_eq!(
            track_handle(&c, 4, CommandState { tag: 0, sequence: 9 }),
            TrackOutcome::GenerationSkipped
        );
        assert_eq!(track_handle(&c, 5, ok(9)), TrackOutcome::Updated);
        assert_eq!(track_handle(&c, 5, ok(3)), TrackOutcome::Updated);
        assert_eq!(c.last_sequence.load(Ordering::SeqCst), 9, "fetch_max");
        assert_eq!(
            try_track_handle(&c, 5, CommandState { tag: 0, sequence: 99 }),
            Err(Failure::InvalidCommandState)
        );
        assert_eq!(c.last_sequence.load(Ordering::SeqCst), 9);
    }

    #[test]
    #[should_panic(expected = "Can't use a resource for a command buffer in state Submitted")]
    fn track_handle_panics_when_state_is_not_recording() {
        let c = ControlBlock::new(5);
        track_handle(&c, 5, CommandState { tag: 3, sequence: 1 });
    }

    #[test]
    #[should_panic(expected = "Can't use a resource for a command buffer in state Initial")]
    fn track_sequence_panics_when_state_is_not_recording() {
        track_sequence(&AtomicU64::new(0), CommandState { tag: 0, sequence: 1 });
    }

    #[test]
    fn mismatched_generation_never_panics_even_for_bad_state() {
        let c = ControlBlock::new(5);
        assert_eq!(
            track_handle(&c, 6, CommandState { tag: 2, sequence: 1 }),
            TrackOutcome::GenerationSkipped
        );
    }
}
