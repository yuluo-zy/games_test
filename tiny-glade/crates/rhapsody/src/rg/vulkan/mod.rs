//! 渲染图的 Vulkan 后端与会话；显式访问表给出每个访问码的阶段、访问掩码和布局。

mod presentation;
mod queue_runtime;
pub use presentation::{PRESENT_ACCESS, PresentTarget, SwapchainContents};

/// 后端导入 token 与跨帧 temporal token 共用的物理资源标识空间。
///
/// 两边曾各自从 1 起分配，会撞号：`bind_import_named` 会因“token 已属于其他物理资源”拒绝导入。
/// 自动分配只使用低 62 位；带高位标记的宿主编号属于独立域。
/// 不把自动计数器推进到宿主域：宿主下一次分配的编号可能恰好覆盖
/// 已退役但仍在资源池／计划缓存中的内部资源。显式宿主编号只接受高位域。
/// 0 与 `u64::MAX` 保留为无效。
const GENERATED_TOKEN_LIMIT: u64 = 1 << 62;
static NEXT_TOKEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
fn try_allocate_token() -> Option<u64> {
    use std::sync::atomic::Ordering;
    NEXT_TOKEN
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |t| {
            (t < GENERATED_TOKEN_LIMIT).then(|| t + 1)
        })
        .ok()
}

mod access_table;
mod resource_spec;
mod capture;
mod named_resources;
mod backend;
mod session;
pub use access_table::{AccessTable, AccessEntries};
pub use resource_spec::{ResourceSpec, GpuResource};
pub use capture::{SUBMISSION_CAPTURE_LIMIT, GpuSubmissionCapture};
pub use backend::{VulkanGraphBackend, TimelineWait, TimelineSignal};
pub use session::{SessionFrameReceipt, PreparedSessionFrame, DisplayFrameId, CompletedNamedResources, VulkanGraphSession};
pub(crate) use session::run_retire_callbacks;

#[cfg(test)]
mod submission_capture_tests;

#[cfg(test)]
mod import_contract_tests;

#[cfg(test)]
mod retire_callback_tests;

#[cfg(test)]
mod enabled_access_stage_tests;
/// A session and its compiled frames may move to a dedicated render thread.
const _: fn() = || {
    fn send<T: Send>() {}
    send::<VulkanGraphSession>();
    send::<PreparedSessionFrame>();
};

#[cfg(test)]
mod declaration_rejection_tests;
