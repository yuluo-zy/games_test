//! 实际句柄控制块及原指令规则；不保留另一套影子资源/队列 EventOracle。
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[repr(C)]
#[derive(Debug)]
pub struct ControlBlock {
    pub last_sequence: AtomicU64,
    pub references: AtomicU32,
    pub generation: u32,
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
    if control.generation != handle_generation {
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
    control: &mut ControlBlock,
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
    control: &mut ControlBlock,
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
    if control.generation != handle_generation {
        return Err(Failure::DisposalGenerationMismatch);
    }
    enqueue_destroy()?;
    if control.references.load(Ordering::SeqCst) != 0 {
        return Err(Failure::ReferencesRemain);
    }
    control.generation = control.generation.wrapping_add(1);
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommandKey {
    pub domain: u32,
    pub sequence: u64,
}
