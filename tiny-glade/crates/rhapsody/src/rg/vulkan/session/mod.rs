//! Frame session: imports, compilation, execution and retirement over a backend.
use crate::present::swapchain::PresentPolicy;
use std::collections::BTreeMap;
use std::sync::Arc;
use super::backend::VulkanGraphBackend;
use super::named_resources::NamedResources;
use super::presentation;
use crate::rg::executor::ExecutionChain;
use crate::rg::executor::GraphExecution;
use crate::rg::graph;
mod receipts;
mod frame_imports;
mod display;
mod compile;
mod execute;
mod named;

#[derive(Clone, Debug)]
pub struct SessionFrameReceipt {
    pub frame: u64,
    storage: Arc<crate::rg::receipt_storage::ReceiptSlot>,
    /// 本帧编译/录制/提交耗时与并行分块统计。
    pub stats: crate::rg::executor::ExecutionStats,
}

impl std::ops::Deref for SessionFrameReceipt {
    type Target = crate::rg::receipt_storage::ReceiptData;
    fn deref(&self) -> &Self::Target {
        &self.storage.data
    }
}

/// Real resource allocation and callback ownership before command recording.
/// Execute or cancel this value through the session that compiled it.
#[must_use = "execute or cancel this compiled frame through its originating session"]
pub struct PreparedSessionFrame {
    graph_id: crate::rg::resource::GraphId,
    generation: crate::sync::DeviceGeneration,
    prepared: crate::rg::executor::PreparedGraph,
    callbacks: Vec<graph::RenderCallback>,
    retire: Vec<graph::RetireCallback>,
    receipt_slot: crate::rg::receipt_storage::ReceiptLease,
}

impl PreparedSessionFrame {
    pub fn snapshot(&self) -> &graph::Snapshot {
        &self.prepared.compiled.snapshot
    }
}

/// Graphs recorded per display frame (main + presentation); sizes the command
/// pool slots so that `frames_in_flight` whole display frames fit.
pub(super) const COMMAND_SLOTS_PER_DISPLAY_FRAME: u32 = 2;

/// One displayed frame. Several graphs (main plus presentation) executed
/// inside one display frame share a single in-flight throttle slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayFrameId(pub u64);

pub(super) struct PendingSessionFrame {
    failed: bool,
    frame: u64,
    /// Display frame this graph belongs to; the throttle counts these groups.
    display: u64,
    execution: GraphExecution,
    retire: Vec<graph::RetireCallback>,
    started_at: std::time::Instant,
}

/// Quiescent transfer of the existing named-resource cache. Contains actual
/// GPU owners, parity, confirmed access/validity and queue ownership; no frame
/// callbacks, Rc graph/session, command pools or alternate resource backend.
/// Constructed only after this session confirms real GPU completion.
pub struct CompletedNamedResources {
    named: NamedResources,
    families: BTreeMap<u64, u32>,
}

pub struct VulkanGraphSession {
    pub backend: VulkanGraphBackend,
    pub chain: ExecutionChain,
    pub(crate) result_cache: crate::rg::result_cache::ResultCache,
    pool: graph::Pool,
    /// 在途提交队列（帧节流）。
    pending: crate::rg::frame_throttle::FrameThrottle<PendingSessionFrame>,
    next_frame: u64,
    /// Largest declaration seen so far; the next frame's graph reserves its vectors from it.
    declaration_shape: graph::DeclarationShape,
    /// Next display-frame id (ids start at 1).
    next_display: u64,
    /// Explicit display frame opened by `begin_display_frame`, if any.
    active_display: Option<u64>,
    /// Display frame of the graph currently being declared.
    current_display: u64,
    frame_imports: Option<crate::rg::frame_imports::FrameImports>,
    frame_import_config: crate::rg::frame_imports::FrameImportConfig,
    device_lost: bool,
    last_execution_partial: bool,
    pub(super) active_prepared: Option<crate::rg::resource::GraphId>,
    /// 上一帧从 begin_frame 到该帧完成的墙钟时长；仅统计用途。
    last_frame_duration_ns: u64,
    /// 本帧提交账本（`frame::FrameLedger`）。
    ledger: crate::rg::frame::FrameLedger,
    named: Option<NamedResources>,
    /// The acquired swapchain image imported into a presentation graph.
    pub(super) presentation: Option<presentation::SessionPresentation>,
    /// What happens to an acquired image whose presentation graph fails.
    pub(super) present_policy: PresentPolicy,
}

impl VulkanGraphSession {
    pub fn new(backend: VulkanGraphBackend) -> Self {
        let chain = backend.new_execution_chain();
        let named = Some(NamedResources::new(backend.device.clone()));
        Self {
            backend,
            chain,
            result_cache: Default::default(),
            pool: graph::Pool::default(),
            pending: Default::default(),
            next_frame: 1,
            declaration_shape: Default::default(),
            next_display: 1,
            active_display: None,
            current_display: 0,
            frame_imports: None,
            frame_import_config: Default::default(),
            device_lost: false,
            last_execution_partial: false,
            active_prepared: None,
            last_frame_duration_ns: 0,
            ledger: Default::default(),
            named,
            presentation: None,
            present_policy: PresentPolicy::default(),
        }
    }

    pub fn last_synchronization(&self) -> &crate::rg::sync_plan::BoundSynchronization {
        self.backend.bound_synchronization()
    }
    pub fn is_poisoned(&self) -> bool {
        self.device_lost || self.backend.device.queues.timelines.is_device_lost()
    }
    /// 最近一次完成帧的墙钟时长。
    pub fn last_known_frame_duration_ns(&self) -> u64 {
        self.last_frame_duration_ns
    }
    /// 本帧提交账本（`begin_frame` 清零，`execute_frame` 记提交，`end_frame` 校验）。
    pub fn frame_ledger(&self) -> &crate::rg::frame::FrameLedger {
        &self.ledger
    }

}

pub(crate) fn run_retire_callbacks(callbacks: Vec<graph::RetireCallback>) -> crate::Result<()> {
    let mut first_error = None;
    for callback in callbacks {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback)).is_err() {
            keep_first_error(
                &mut first_error,
                Err(crate::Error::invalid_usage("退役回调 panic")),
            );
        }
    }
    first_error.map_or(Ok(()), Err)
}

pub(super) fn keep_first_error(first: &mut Option<crate::Error>, result: crate::Result<()>) {
    if let Err(error) = result {
        log::warn!("帧完成清理步骤失败: {error}");
        first.get_or_insert(error);
    }
}

impl Drop for VulkanGraphSession {
    fn drop(&mut self) {
        while let Some(frame) = self.pending.front().map(|p| p.frame) {
            if let Err(error) = self.wait_frame(frame, u64::MAX) {
                // Device loss is terminal. An ordinary query failure needs an
                // actual idle proof before invoking callbacks or recycling pools.
                let safe = error.is_device_lost()
                    || self.backend.device.queues.timelines.is_device_lost()
                    || self.backend.device.wait_idle().is_ok();
                if safe {
                    while let Some(pending) = self.pending.pop_front() {
                        let _ = self.finish_frame(pending);
                    }
                } else {
                    log::error!("无法确认关闭时 GPU 已停止，保留在飞帧及退役回调: {error}");
                    while let Some(pending) = self.pending.pop_front() {
                        std::mem::forget(pending);
                    }
                }
                break;
            }
        }
        let _ = self.clear_after_completion();
    }
}
