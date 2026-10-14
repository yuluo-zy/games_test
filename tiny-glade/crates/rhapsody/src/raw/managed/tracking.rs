//! 实际句柄控制块及原指令规则；不保留另一套影子资源/队列 EventOracle。
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[repr(C)]
#[derive(Debug)]
pub struct ControlBlock {
    pub last_sequence: AtomicU64,
    pub references: AtomicU32,
    /// 原 `resource_checksum`：槽位复用时递增。原子量使控制块可无锁共享（`Arc<ControlBlock>`）。
    pub resource_checksum: AtomicU32,
}
impl ControlBlock {
    pub fn new(resource_checksum: u32) -> Self {
        Self {
            last_sequence: AtomicU64::new(0),
            references: AtomicU32::new(1),
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

pub fn track_sequence(tracker: &AtomicU64, command: CommandState) -> Result<(), Failure> {
    if command.tag != 1 {
        return Err(Failure::InvalidCommandState);
    }
    tracker.fetch_max(command.sequence, Ordering::SeqCst);
    Ok(())
}

pub fn track_handle(
    control: &ControlBlock,
    handle_generation: u32,
    command: CommandState,
) -> Result<TrackOutcome, Failure> {
    if control.checksum() != handle_generation {
        return Ok(TrackOutcome::GenerationSkipped);
    }
    track_sequence(&control.last_sequence, command)?;
    Ok(TrackOutcome::Updated)
}

pub fn clone_reference(control: &ControlBlock) -> u32 {
    control
        .references
        .fetch_add(1, Ordering::SeqCst)
        .wrapping_add(1)
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
