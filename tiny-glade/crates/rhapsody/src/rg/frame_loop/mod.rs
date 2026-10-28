//! Production ownership for graph submission, acquisition, the presentation
//! graph, presentation and bounded in-flight frames. Uses the same device
//! timelines as the graph.
//!
//! Frame order: the main graph is submitted first (it never depends on the
//! swapchain image), then the image is acquired, then a *presentation graph*
//! that imports the acquired image as a first-class resource is built and
//! executed, then the image is presented. The GPU therefore executes the main
//! graph while the CPU may block in acquire, and a frame without an image still
//! executes its main graph (uploads and temporal-history writes are never
//! dropped); only the presentation graph and present are skipped.
//!
//! The presentation graph may have any number of batches and queues: the
//! session injects the acquire wait into the first batch touching the image,
//! the PRESENT_SRC_KHR transition after its last use and the present wait
//! signal into the last batch touching it (see `rg::vulkan::PresentTarget`).
use super::{graph::{BuiltGraph, RenderGraph}, vulkan::{PresentTarget, SessionFrameReceipt, SwapchainContents, VulkanGraphSession}};
use crate::present::PresentPolicy;
use crate::{Error, Result, sync::SyncPoint};
use crate::present::swapchain::{AcquiredImageLease, PresentTicket, Swapchain};
use crate::native::surface::Surface;
use ash::vk;
use std::{collections::VecDeque, sync::Arc, time::Instant};

fn observe_device_result<A: crate::sync::timeline::TimelineApi, T>(
    timelines: &crate::sync::timeline::Timelines<A>,
    result: Result<T>,
) -> Result<T> {
    if result.as_ref().is_err_and(Error::is_device_lost) {
        timelines.mark_device_lost();
    }
    result
}

/// The production control flow is independent of individual Vulkan calls so its
/// ownership and recovery ordering can be exercised without a window or driver.
trait FrameOperations {
    type Lease;
    type GraphReceipt;
    type Target;
    type PresentationReceipt;
    fn is_device_lost(&self) -> bool;
    fn observe_error(&mut self, error: &Error);
    fn acquire(&mut self) -> Result<Option<Self::Lease>>;
    /// Ownership of all graph retirement callbacks moves to this operation,
    /// including when only part of the graph reaches the GPU.
    fn execute_graph(&mut self, built: BuiltGraph) -> Result<Self::GraphReceipt>;
    /// Build and execute the presentation graph around the acquired image.
    /// Consumes the lease: on error the image has already been returned to
    /// the swapchain (released, or discarded and presented when a partial
    /// graph consumed the acquire wait).
    fn render_presentation(
        &mut self,
        lease: Self::Lease,
    ) -> Result<(Self::Target, Self::GraphReceipt)>;
    /// Present consumes the rendered target even on failure; its
    /// implementation owns rejected-present cleanup.
    fn present(&mut self, target: Self::Target) -> Result<Self::PresentationReceipt>;
}

/// The graph's result is valid whenever it was submitted; the presentation
/// result is independent of it.
struct FrameProtocolOutput<G, P> {
    graph: G,
    presentation: Result<Option<(G, P)>>,
}

fn execute_frame_protocol<O: FrameOperations>(
    operations: &mut O,
    built: BuiltGraph,
) -> Result<FrameProtocolOutput<O::GraphReceipt, O::PresentationReceipt>> {
    if operations.is_device_lost() {
        // Unsubmitted callbacks retire now; declaration rejection and retirement
        // errors never mask the terminal device loss.
        let (_, _, retire, _declaration_error) = built.into_parts();
        if let Err(error) = super::vulkan::run_retire_callbacks(retire) {
            log::warn!("设备丢失后未提交帧的退役回调失败: {error}");
        }
        return Err(Error::DeviceLost);
    }
    let graph = match operations.execute_graph(built) {
        Ok(receipt) => receipt,
        Err(error) => {
            operations.observe_error(&error);
            return Err(error);
        }
    };
    let presentation = present_after_graph(operations);
    Ok(FrameProtocolOutput {
        graph,
        presentation,
    })
}

fn present_after_graph<O: FrameOperations>(
    operations: &mut O,
) -> Result<Option<(O::GraphReceipt, O::PresentationReceipt)>> {
    if operations.is_device_lost() {
        return Err(Error::DeviceLost);
    }
    let lease = match operations.acquire() {
        Ok(Some(lease)) => lease,
        Ok(None) => return Ok(None),
        Err(error) => {
            operations.observe_error(&error);
            return Err(error);
        }
    };
    let rendered = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        operations.render_presentation(lease)
    }))
    .unwrap_or_else(|_| Err(Error::invalid_usage("呈现图构建回调 panic")));
    let (target, receipt) = match rendered {
        Ok(rendered) => rendered,
        Err(error) => {
            operations.observe_error(&error);
            return Err(error);
        }
    };
    let result = operations.present(target);
    if let Err(error) = &result {
        operations.observe_error(error);
    }
    result.map(|presented| Some((receipt, presented)))
}

#[derive(Clone, Copy, Debug)]
pub struct PresentationReceipt {
    /// Submission that signalled the present wait (the last batch touching
    /// the swapchain image); it completes after every use of the image.
    pub point: SyncPoint,
    pub generation: u64,
    pub device_generation: crate::sync::DeviceGeneration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameStatus {
    /// The graph executed but no image was presented this frame.
    Skipped,
    Presented,
    /// The swapchain was actually replaced. This can include a presentation,
    /// or a newly created chain that could not acquire an image this frame.
    Recreated,
}

#[derive(Debug)]
pub struct FrameOutcome {
    pub status: FrameStatus,
    /// The main graph always executes, including on skipped presentations.
    pub graph: SessionFrameReceipt,
    /// The presentation graph, when an image was acquired and presented.
    pub presentation_graph: Option<SessionFrameReceipt>,
    pub presentation: Option<PresentationReceipt>,
    /// The current swapchain generation after this frame's acquire/recreation.
    pub generation: u64,
}

fn frame_status(before: u64, after: u64, presented: bool) -> FrameStatus {
    // A stale/resize request alone is not evidence that recreation happened.
    if before != after {
        FrameStatus::Recreated
    } else if presented {
        FrameStatus::Presented
    } else {
        FrameStatus::Skipped
    }
}

/// Wall-clock API scopes. Acquire and present can include WSI waits; neither
/// value alone measures pure vsync or GPU execution time.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameCpuTimings {
    pub acquire_ns: u64,
    /// Build, compile, record and submit of the presentation graph.
    pub presentation_graph_ns: u64,
    pub present_ns: u64,
}
pub struct FrameLoop {
    cpu_timings: FrameCpuTimings,
    pub graph: VulkanGraphSession,
    swapchain: Arc<Swapchain>,
    pending: VecDeque<PresentTicket>,
    contents: SwapchainContents,
}
impl FrameLoop {
    pub fn new(graph: VulkanGraphSession, swapchain: Arc<Swapchain>) -> Result<Self> {
        if swapchain.policy() != crate::present::swapchain::SwapchainPolicy::Strict {
            return Err(Error::unsupported(
                "生产 FrameLoop 需要 Strict 交换链关闭保证",
            ));
        }
        if !Arc::ptr_eq(&graph.backend.device, &swapchain.device) {
            return Err(Error::invalid_usage("图会话与交换链必须属于同一设备"));
        }
        if graph.is_poisoned() {
            return Err(Error::DeviceLost);
        }
        let capacity = swapchain.device.frames_in_flight() as usize;
        Ok(Self {
            cpu_timings: FrameCpuTimings::default(),
            graph,
            swapchain,
            pending: VecDeque::with_capacity(capacity),
            contents: SwapchainContents::Discard,
        })
    }
    /// The *current* swapchain generation. The frame loop replaces it on
    /// recreation, so this is a borrow rather than a shareable `Arc`: a
    /// retained clone would silently refer to a retired generation. Compare
    /// [`Self::generation`] values to detect replacement.
    pub fn swapchain(&self) -> &Swapchain {
        &self.swapchain
    }
    pub fn device(&self) -> &Arc<crate::device::Device> {
        &self.swapchain.device
    }
    /// Generation of the current swapchain; changes on every replacement.
    pub fn generation(&self) -> u64 {
        self.swapchain.generation()
    }
    /// How presentation graphs see the acquired image's previous contents.
    pub fn set_swapchain_contents(&mut self, contents: SwapchainContents) {
        self.contents = contents;
    }
    /// What happens to an acquired image whose presentation graph fails
    /// after the acquire was consumed (or before anything was submitted):
    /// [`PresentPolicy::SkipPresent`] (default) never shows undefined
    /// memory; `ClearTo` presents a defined colour. `PreviousFrame` is
    /// unsupported (the swapchain keeps no copy) and returns an error.
    pub fn set_on_failure(&mut self, policy: PresentPolicy) -> Result<()> {
        self.graph.set_present_policy(policy)
    }
    pub fn on_failure(&self) -> PresentPolicy {
        self.graph.present_policy()
    }
    /// Request a new extent; the next acquisition rebuilds the swapchain.
    pub fn resize(&self, extent: vk::Extent2D) -> Result<()> {
        self.ensure_alive()?;
        self.swapchain.resize(extent)
    }
    /// Supply a new surface after [`Error::SurfaceLost`]. Frames keep
    /// returning SurfaceLost cheaply, without native calls, until then.
    pub fn replace_surface(&mut self, surface: Arc<Surface>) -> Result<()> {
        self.ensure_alive()?;
        let result = Swapchain::replace_surface(&mut self.swapchain, surface);
        self.observe(result)
    }
    /// Forward a host focus event to the current swapchain generation.
    pub fn set_focused(&self, focused: bool) -> Result<crate::present::swapchain::FullscreenStatus> {
        self.ensure_alive()?;
        self.observe(self.swapchain.set_focused(focused))
    }
    /// Forward a host window-mode change; the next acquire rebuilds if needed.
    ///
    /// # Safety
    /// An Exclusive monitor must satisfy Swapchain::set_window_mode's lifetime
    /// and Win32 window/monitor association requirements.
    pub unsafe fn set_window_mode(
        &self,
        mode: crate::present::swapchain::WindowMode,
    ) -> Result<crate::present::swapchain::FullscreenStatus> {
        self.ensure_alive()?;
        self.observe(unsafe { self.swapchain.set_window_mode(mode) })
    }
    pub fn last_cpu_timings(&self) -> FrameCpuTimings {
        self.cpu_timings
    }
    fn ensure_alive(&self) -> Result<()> {
        if self.graph.is_poisoned() {
            Err(Error::DeviceLost)
        } else if self.swapchain.is_closing() {
            Err(Error::invalid_usage("帧循环正在关闭"))
        } else {
            Ok(())
        }
    }
    fn observe<T>(&self, result: Result<T>) -> Result<T> {
        observe_device_result(&self.swapchain.device.queues.timelines, result)
    }
    /// Throttle on the oldest in-flight presented frame, then poll WSI
    /// completion proofs and device garbage once for this frame.
    pub fn begin_frame(&mut self) -> Result<RenderGraph> {
        self.ensure_alive()?;
        let result = self.retire_tickets();
        self.observe(result)?;
        if self.pending.len() >= self.swapchain.device.frames_in_flight() as usize
            && let Some(oldest) = self.pending.front()
        {
            let result = oldest.wait(crate::present::swapchain::ACQUIRE_TIMEOUT_NS);
            self.observe(result)?;
            let result = self.retire_tickets();
            self.observe(result)?;
        }
        let result = self.collect_wsi();
        self.observe(result)?;
        // The main and presentation graphs of this frame share one in-flight slot.
        let result = self.graph.begin_display_frame();
        self.observe(result)?;
        let result = self.graph.begin_frame();
        if result.is_err() {
            // `execute` never runs for this frame, so nothing else would close it.
            self.graph.end_display_frame();
        }
        self.observe(result)
    }
    pub fn execute_graph(&mut self, built: BuiltGraph) -> Result<SessionFrameReceipt> {
        // Session owns the graph's retire callbacks, including when a poisoned
        // device rejects an unsubmitted graph.
        let result = self.graph.execute_frame(built);
        self.observe(result)
    }
    /// Submit the main graph, acquire, then build, execute and present the
    /// presentation graph.
    ///
    /// `build_presentation` receives a fresh graph in which the acquired image
    /// is already imported ([`PresentTarget::resource`]) with discarded
    /// contents and exported for presentation; it adds the passes that write
    /// the image. They may span any number of batches and queues. On any error
    /// after acquisition the image is returned to the swapchain before this
    /// returns; the main graph stays owned by the session either way.
    pub fn execute(
        &mut self,
        built: BuiltGraph,
        build_presentation: impl FnOnce(&mut RenderGraph, &PresentTarget) -> Result<()>,
    ) -> Result<FrameOutcome> {
        self.cpu_timings = FrameCpuTimings::default();
        let before = self.swapchain.generation();
        let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| execute_frame_protocol(
            &mut LiveFrameOperations {
                frame_loop: self,
                build_presentation: Some(build_presentation),
            },
            built,
        )));
        // Close the display frame on success and failure alike.
        self.graph.end_display_frame();
        let output = match output {
            Ok(output) => output?,
            Err(payload) => std::panic::resume_unwind(payload),
        };
        let presentation = output.presentation?;
        let generation = self.swapchain.generation();
        let (presentation_graph, presentation) = presentation.unzip();
        Ok(FrameOutcome {
            status: frame_status(before, generation, presentation.is_some()),
            graph: output.graph,
            presentation_graph,
            presentation,
            generation,
        })
    }
    /// Acquire the next image; `None` skips presentation for this frame.
    /// Import it with [`VulkanGraphSession::import_swapchain_image`] (or drop
    /// or cancel it to return it), then [`Self::present`] after executing.
    pub fn acquire(&mut self) -> Result<Option<AcquiredImageLease>> {
        self.ensure_alive()?;
        let started = Instant::now();
        let result = Swapchain::validate_swapchain_and_acquire_image(&mut self.swapchain);
        self.cpu_timings.acquire_ns = started.elapsed().as_nanos() as u64;
        let policy = self.graph.present_policy();
        self.observe(result.and_then(|lease| {
            lease
                .map(|mut lease| lease.set_failure_policy(policy).map(|()| lease))
                .transpose()
        }))
    }
    /// Present the image rendered by `target`'s executed presentation graph.
    pub fn present(&mut self, target: &PresentTarget) -> Result<PresentationReceipt> {
        if self.graph.is_poisoned() {
            let _ = self.graph.cancel_presentation();
            return Err(Error::DeviceLost);
        }
        if target.generation() != self.swapchain.generation() {
            let _ = self.graph.cancel_presentation();
            return Err(Error::invalid_usage("呈现目标不属于当前交换链代际"));
        }
        if let Err(error) = self.pending.try_reserve(1) {
            let _ = self.graph.cancel_presentation();
            log::warn!("present ticket reservation failed: {error}");
            return Err(Error::OutOfHostMemory);
        }
        let started = Instant::now();
        let result = self.graph.present(target);
        self.cpu_timings.present_ns = started.elapsed().as_nanos() as u64;
        let ticket = self.observe(result)?;
        let receipt = PresentationReceipt {
            point: ticket.point,
            generation: ticket.generation(),
            device_generation: self.swapchain.device.generation(),
        };
        self.pending.push_back(ticket);
        Ok(receipt)
    }
    pub fn wait_render(&mut self, receipt: PresentationReceipt, timeout: u64) -> Result<()> {
        self.ensure_alive()?;
        if receipt.device_generation != self.swapchain.device.generation() {
            return Err(Error::invalid_usage("呈现回执属于其他设备代际"));
        }
        self.swapchain
            .device
            .queues
            .timelines
            .wait(&receipt.point.into(), timeout)?;
        self.collect()
    }
    pub fn collect(&mut self) -> Result<()> {
        self.ensure_alive()?;
        let result = self.collect_inner();
        self.observe(result)
    }
    fn collect_inner(&mut self) -> Result<()> {
        self.retire_tickets()?;
        self.collect_wsi()
    }
    fn retire_tickets(&mut self) -> Result<()> {
        while let Some(ticket) = self.pending.front() {
            if !ticket.is_complete()? {
                break;
            }
            self.pending.pop_front();
        }
        Ok(())
    }
    fn collect_wsi(&self) -> Result<()> {
        self.swapchain.collect_presentations()?;
        self.swapchain.device.try_collect_garbage()?;
        Ok(())
    }
    pub fn wait_idle(&mut self, timeout: u64) -> Result<()> {
        self.ensure_alive()?;
        let result = self.graph.wait_submits_done(timeout);
        self.observe(result)?;
        let presentations = self.pending.iter().try_for_each(|ticket| ticket.wait(timeout));
        self.observe(presentations)?;
        self.collect()?;
        let result = self
            .swapchain
            .device
            .try_collect_all_completed()
            .map(|_| ());
        self.observe(result)
    }
    /// Finish rendering and presentation before the window/device is released.
    /// A failed bounded wait retains all owners and can be retried; no further
    /// frame work is accepted once swapchain shutdown has started.
    pub fn shutdown(&mut self, timeout: u64) -> Result<()> {
        if self.graph.is_poisoned() {
            return Err(Error::DeviceLost);
        }
        let result = self.graph.cancel_presentation();
        self.observe(result)?;
        let result = self.graph.wait_submits_done(timeout);
        self.observe(result)?;
        for ticket in &self.pending {
            ticket.wait(timeout)?;
        }
        // Shutdown retries bypass ensure_alive's closing check.
        let result = self.collect_inner();
        self.observe(result)?;
        let result = self.swapchain.shutdown(timeout);
        self.observe(result)?;
        let result = self
            .swapchain
            .device
            .try_collect_all_completed()
            .map(|_| ());
        self.observe(result)
    }
}

struct LiveFrameOperations<'a, F> {
    frame_loop: &'a mut FrameLoop,
    build_presentation: Option<F>,
}
impl<F> LiveFrameOperations<'_, F>
where
    F: FnOnce(&mut RenderGraph, &PresentTarget) -> Result<()>,
{
    fn render_presentation_inner(
        &mut self,
        lease: AcquiredImageLease,
    ) -> Result<(PresentTarget, SessionFrameReceipt)> {
        let build = self
            .build_presentation
            .take()
            .ok_or_else(|| Error::invalid_usage("呈现图构建回调每帧只能调用一次"))?;
        let frame_loop = &mut *self.frame_loop;
        // A failure here drops `lease`, which returns the unsubmitted image.
        let mut graph = frame_loop.graph.begin_frame()?;
        let target =
            match frame_loop
                .graph
                .import_swapchain_image(&mut graph, lease, frame_loop.contents)
            {
                Ok(target) => target,
                Err(error) => {
                    let _ = frame_loop.graph.cancel_frame(graph.build());
                    return Err(error);
                }
            };
        let built =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| build(&mut graph, &target)))
                .unwrap_or_else(|_| Err(Error::invalid_usage("呈现图构建回调 panic")));
        if let Err(error) = built {
            // Cancelling the graph also returns its swapchain image.
            if let Err(cleanup) = frame_loop.graph.cancel_frame(graph.build()) {
                log::warn!("呈现图取消失败，保留主错误 {error}: {cleanup}");
            }
            return Err(error);
        }
        // An execution failure has already returned the image (session).
        let receipt = frame_loop.graph.execute_frame(graph.build())?;
        Ok((target, receipt))
    }
}
impl<F> FrameOperations for LiveFrameOperations<'_, F>
where
    F: FnOnce(&mut RenderGraph, &PresentTarget) -> Result<()>,
{
    type Lease = AcquiredImageLease;
    type GraphReceipt = SessionFrameReceipt;
    type Target = PresentTarget;
    type PresentationReceipt = PresentationReceipt;
    fn is_device_lost(&self) -> bool {
        self.frame_loop.graph.is_poisoned()
    }
    fn observe_error(&mut self, error: &Error) {
        if error.is_device_lost() {
            self.frame_loop
                .swapchain
                .device
                .queues
                .timelines
                .mark_device_lost();
        }
    }
    fn acquire(&mut self) -> Result<Option<Self::Lease>> {
        self.frame_loop.acquire()
    }
    fn execute_graph(&mut self, built: BuiltGraph) -> Result<Self::GraphReceipt> {
        self.frame_loop.execute_graph(built)
    }
    fn render_presentation(
        &mut self,
        lease: Self::Lease,
    ) -> Result<(Self::Target, Self::GraphReceipt)> {
        let started = Instant::now();
        let result = self.render_presentation_inner(lease);
        if result.is_err() {
            // Defensive: nothing that failed may keep the image acquired.
            let cleanup = self.frame_loop.graph.cancel_presentation();
            if let Err(cleanup) = cleanup {
                self.observe_error(&cleanup);
                log::warn!("呈现图失败后的图像归还失败: {cleanup}");
            }
        }
        self.frame_loop.cpu_timings.presentation_graph_ns = started.elapsed().as_nanos() as u64;
        self.frame_loop.observe(result)
    }
    fn present(&mut self, target: Self::Target) -> Result<Self::PresentationReceipt> {
        self.frame_loop.present(&target)
    }
}

#[cfg(test)]
mod device_loss_tests;
