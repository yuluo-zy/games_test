//! Production ownership for graph submission, acquisition, display submission,
//! and bounded in-flight frames. Uses the same device timeline as the graph.
//!
//! Frame order: the main graph is submitted first (it never depends on the
//! swapchain image), then the image is acquired, then display work is recorded,
//! submitted and presented. The GPU therefore executes the graph while the CPU
//! may block in acquire, and a frame without an image still executes its graph
//! (uploads and temporal-history writes are never dropped); only display and
//! present are skipped.
use super::{
    graph::BuiltGraph,
    vulkan::{SessionFrameReceipt, VulkanGraphSession},
};
use crate::{
    Error, Result,
    raw::{
        command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
        swapchain::{AcquiredImageLease, DisplayTarget, PresentTicket, Surface, Swapchain},
    },
    sync::{SyncPoint, SyncSet},
};
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
    type Command;
    type PresentationReceipt;
    fn is_device_lost(&self) -> bool;
    fn observe_error(&mut self, error: &Error);
    fn acquire(&mut self) -> Result<Option<Self::Lease>>;
    /// Ownership of all graph retirement callbacks moves to this operation,
    /// including when only part of the graph reaches the GPU.
    fn execute_graph(&mut self, built: BuiltGraph) -> Result<Self::GraphReceipt>;
    fn record_display(&mut self, lease: &Self::Lease) -> Result<Self::Command>;
    /// Present consumes the lease even on failure; its implementation owns the
    /// already-submitted display work and any rejected-present cleanup.
    fn present(
        &mut self,
        lease: Self::Lease,
        command: Self::Command,
    ) -> Result<Self::PresentationReceipt>;
    fn cancel(&mut self, lease: Self::Lease) -> Result<()>;
    fn abandon_device_lost(&mut self, lease: Self::Lease);
}

fn recover_acquisition<O: FrameOperations>(operations: &mut O, lease: O::Lease, error: &Error) {
    operations.observe_error(error);
    if operations.is_device_lost() {
        operations.abandon_device_lost(lease);
    } else if let Err(cleanup) = operations.cancel(lease) {
        operations.observe_error(&cleanup);
        log::warn!("帧失败后的 acquire 清理失败，保留主错误 {error}: {cleanup}");
    }
}

/// The graph's result is valid whenever it was submitted; the presentation
/// result is independent of it.
struct FrameProtocolOutput<G, P> {
    graph: G,
    presentation: Result<Option<P>>,
}

fn execute_frame_protocol<O: FrameOperations>(
    operations: &mut O,
    built: BuiltGraph,
) -> Result<FrameProtocolOutput<O::GraphReceipt, O::PresentationReceipt>> {
    if operations.is_device_lost() {
        // Unsubmitted callbacks retire now; their own errors never mask loss.
        let (_, _, retire) = built.into_parts();
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
) -> Result<Option<O::PresentationReceipt>> {
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
    let recorded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        operations.record_display(&lease)
    }))
    .unwrap_or_else(|_| Err(Error::invalid_usage("呈现录制回调 panic")));
    let command = match recorded {
        Ok(command) => command,
        Err(error) => {
            recover_acquisition(operations, lease, &error);
            return Err(error);
        }
    };
    let result = operations.present(lease, command);
    if let Err(error) = &result {
        operations.observe_error(error);
    }
    result.map(Some)
}

#[derive(Clone, Copy, Debug)]
pub struct PresentationReceipt {
    pub point: SyncPoint,
    pub generation: u64,
    pub device_generation: crate::device::DeviceGeneration,
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

/// Wall-clock API scopes. Acquire and display-submit/present can include WSI
/// waits; neither value alone measures pure vsync or GPU execution time.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameCpuTimings {
    pub acquire_ns: u64,
    pub display_record_ns: u64,
    pub display_submit_present_ns: u64,
}
pub struct FrameLoop {
    cpu_timings: FrameCpuTimings,
    pub graph: VulkanGraphSession,
    swapchain: Arc<Swapchain>,
    pools: Vec<Arc<VulkanCommandPool>>,
    pending: VecDeque<PresentTicket>,
    commands: Vec<VulkanCommandBuffer>,
    graph_completion: SyncSet,
}
impl FrameLoop {
    pub fn new(graph: VulkanGraphSession, swapchain: Arc<Swapchain>) -> Result<Self> {
        if swapchain.policy() != crate::raw::swapchain::SwapchainPolicy::Strict {
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
        let pool = observe_device_result(
            &swapchain.device.timelines,
            VulkanCommandPool::new(swapchain.device.clone()),
        )?;
        let capacity = swapchain.device.frames_in_flight() as usize;
        Ok(Self {
            cpu_timings: FrameCpuTimings::default(),
            graph,
            swapchain,
            pools: vec![pool],
            pending: VecDeque::with_capacity(capacity),
            commands: Vec::with_capacity(capacity),
            graph_completion: SyncSet::EMPTY,
        })
    }
    /// The *current* swapchain generation. The frame loop replaces it on
    /// recreation, so this is a borrow rather than a shareable `Arc`: a
    /// retained clone would silently refer to a retired generation. Compare
    /// [`Self::generation`] values to detect replacement.
    pub fn swapchain(&self) -> &Swapchain {
        &self.swapchain
    }
    pub fn device(&self) -> &Arc<crate::raw::device::Device> {
        &self.swapchain.device
    }
    /// Generation of the current swapchain; changes on every replacement.
    pub fn generation(&self) -> u64 {
        self.swapchain.generation()
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
    pub fn set_focused(&self, focused: bool) -> Result<crate::raw::swapchain::FullscreenStatus> {
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
        mode: crate::raw::swapchain::WindowMode,
    ) -> Result<crate::raw::swapchain::FullscreenStatus> {
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
        observe_device_result(&self.swapchain.device.timelines, result)
    }
    /// Throttle on the oldest in-flight display frame, then poll WSI
    /// completion proofs and device garbage once for this frame.
    pub fn begin_frame(&mut self) -> Result<super::graph::RenderGraph> {
        self.ensure_alive()?;
        let result = self.retire_tickets();
        self.observe(result)?;
        if self.pending.len() >= self.swapchain.device.frames_in_flight() as usize
            && let Some(oldest) = self.pending.front()
        {
            let result = oldest.wait(crate::raw::swapchain::ACQUIRE_TIMEOUT_NS);
            self.observe(result)?;
            let result = self.retire_tickets();
            self.observe(result)?;
        }
        let result = self.collect_wsi();
        self.observe(result)?;
        self.graph_completion = SyncSet::EMPTY;
        let result = self.graph.begin_frame();
        self.observe(result)
    }
    pub fn execute_graph(&mut self, built: BuiltGraph) -> Result<SessionFrameReceipt> {
        self.graph_completion = SyncSet::EMPTY;
        // Session owns the graph's retire callbacks, including when a poisoned
        // device rejects an unsubmitted graph.
        let result = self.graph.execute_frame(built);
        let receipt = self.observe(result)?;
        for point in &receipt.receipts {
            self.graph_completion.insert(*point);
        }
        Ok(receipt)
    }
    /// Submit the graph, then acquire, record and present display work.
    ///
    /// `record_display` only records draws/copies: the library transitions
    /// the image from UNDEFINED to `target.layout` before it and to
    /// PRESENT_SRC_KHR after it. On an error after the graph was submitted,
    /// the session still owns those batches and their retirement callbacks,
    /// and an unsubmitted acquisition is cancelled (released) before return.
    pub fn execute(
        &mut self,
        built: BuiltGraph,
        record_display: impl FnOnce(&mut VulkanCommandBuffer, &AcquiredImageLease) -> Result<()>,
        target: DisplayTarget,
    ) -> Result<FrameOutcome> {
        self.cpu_timings = FrameCpuTimings::default();
        let before = self.swapchain.generation();
        let output = execute_frame_protocol(
            &mut LiveFrameOperations {
                frame_loop: self,
                record_display: Some(record_display),
                target,
            },
            built,
        )?;
        let presentation = output.presentation?;
        let generation = self.swapchain.generation();
        Ok(FrameOutcome {
            status: frame_status(before, generation, presentation.is_some()),
            graph: output.graph,
            presentation,
            generation,
        })
    }
    pub fn acquire(&mut self) -> Result<Option<AcquiredImageLease>> {
        self.ensure_alive()?;
        let started = Instant::now();
        let result = Swapchain::validate_swapchain_and_acquire_image(&mut self.swapchain);
        self.cpu_timings.acquire_ns = started.elapsed().as_nanos() as u64;
        self.observe(result)
    }
    pub fn command(&mut self) -> Result<VulkanCommandBuffer> {
        self.ensure_alive()?;
        if let Some(mut command) = self.commands.pop() {
            let pool = command.pool.clone();
            // Each display slot owns one command pool, so completing that
            // command permits one whole-pool reset without affecting other frames.
            if let Err(error) = pool.reset_completed(&mut [&mut command]) {
                self.commands.push(command);
                return self.observe(Err(error));
            }
            Ok(command)
        } else {
            for pool in &self.pools {
                if pool.live_command_count() != 0 {
                    continue;
                }
                match pool.reset_completed(&mut []) {
                    Ok(()) => return self.observe(pool.allocate()),
                    Err(Error::Vulkan(vk::Result::NOT_READY)) => continue,
                    Err(error) => return self.observe(Err(error)),
                }
            }
            if self.pools.len() >= self.swapchain.device.frames_in_flight() as usize {
                return Err(vk::Result::NOT_READY.into());
            }
            let pool = self.observe(VulkanCommandPool::new(self.swapchain.device.clone()))?;
            let command = self.observe(pool.allocate())?;
            self.pools.push(pool);
            Ok(command)
        }
    }
    /// Submit recorded display work after this frame's graph and present it.
    /// The command must contain the lease's acquire/present transitions for
    /// `target` (see [`AcquiredImageLease::record_acquire_transition`]).
    pub fn present(
        &mut self,
        lease: AcquiredImageLease,
        command: VulkanCommandBuffer,
        target: DisplayTarget,
    ) -> Result<PresentationReceipt> {
        if self.graph.is_poisoned() {
            lease.abandon_device_lost();
            return Err(Error::DeviceLost);
        }
        if lease.generation() != self.swapchain.generation() {
            return Err(Error::invalid_usage("呈现租约不属于当前交换链代际"));
        }
        if self.pending.len() >= self.swapchain.device.frames_in_flight() as usize {
            return Err(Error::invalid_usage("须先 begin_frame 执行帧节流"));
        }
        self.pending
            .try_reserve(1)
            .map_err(|_| Error::OutOfHostMemory)?;
        let started = Instant::now();
        let result = lease.submit_after(command, target, self.graph_completion);
        self.cpu_timings.display_submit_present_ns = started.elapsed().as_nanos() as u64;
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
            if let Some(mut ticket) = self.pending.pop_front()
                && let Some(command) = ticket.take_completed_command()?
            {
                self.commands.push(command);
            }
        }
        Ok(())
    }
    fn collect_wsi(&mut self) -> Result<()> {
        self.swapchain.collect_presentations()?;
        self.swapchain.device.try_collect_garbage()?;
        Ok(())
    }
    pub fn wait_idle(&mut self, timeout: u64) -> Result<()> {
        self.ensure_alive()?;
        let result = self.graph.wait_submits_done(timeout);
        self.observe(result)?;
        for ticket in &self.pending {
            ticket.wait(timeout)?;
        }
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
    record_display: Option<F>,
    target: DisplayTarget,
}
impl<F> FrameOperations for LiveFrameOperations<'_, F>
where
    F: FnOnce(&mut VulkanCommandBuffer, &AcquiredImageLease) -> Result<()>,
{
    type Lease = AcquiredImageLease;
    type GraphReceipt = SessionFrameReceipt;
    type Command = VulkanCommandBuffer;
    type PresentationReceipt = PresentationReceipt;
    fn is_device_lost(&self) -> bool {
        self.frame_loop.graph.is_poisoned()
    }
    fn observe_error(&mut self, error: &Error) {
        if error.is_device_lost() {
            self.frame_loop
                .swapchain
                .device
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
    fn record_display(&mut self, lease: &Self::Lease) -> Result<Self::Command> {
        let started = Instant::now();
        let record = self
            .record_display
            .take()
            .ok_or_else(|| Error::invalid_usage("呈现录制回调每帧只能调用一次"))?;
        let mut command = self.frame_loop.command()?;
        self.frame_loop.observe(command.begin())?;
        self.frame_loop
            .observe(lease.record_acquire_transition(&mut command, self.target))?;
        self.frame_loop.observe(record(&mut command, lease))?;
        self.frame_loop
            .observe(lease.record_present_transition(&mut command, self.target))?;
        self.frame_loop.observe(command.end())?;
        self.frame_loop.cpu_timings.display_record_ns = started.elapsed().as_nanos() as u64;
        Ok(command)
    }
    fn present(
        &mut self,
        lease: Self::Lease,
        command: Self::Command,
    ) -> Result<Self::PresentationReceipt> {
        self.frame_loop.present(lease, command, self.target)
    }
    fn cancel(&mut self, lease: Self::Lease) -> Result<()> {
        lease.cancel(crate::raw::swapchain::ACQUIRE_TIMEOUT_NS)
    }
    fn abandon_device_lost(&mut self, lease: Self::Lease) {
        lease.abandon_device_lost();
    }
}

#[cfg(test)]
mod device_loss_tests {
    use super::*;
    use crate::sync::{
        QueueId,
        timeline::{TimelineApi, Timelines},
    };
    use ash::vk::Handle;

    struct NoGpuCalls;
    impl TimelineApi for NoGpuCalls {
        fn counter_value(&self, _: vk::Semaphore) -> Result<u64> {
            panic!("a poisoned frame loop must not query the GPU")
        }
        fn wait_all(&self, _: &[vk::Semaphore], _: &[u64], _: u64) -> Result<bool> {
            panic!("a poisoned frame loop must not wait on the GPU")
        }
    }
    fn timelines() -> Timelines<NoGpuCalls> {
        Timelines::new(NoGpuCalls, &[vk::Semaphore::from_raw(1)]).unwrap()
    }

    #[test]
    fn frame_status_requires_an_actual_generation_change_to_report_recreation() {
        assert_eq!(frame_status(10, 10, false), FrameStatus::Skipped);
        assert_eq!(frame_status(10, 10, true), FrameStatus::Presented);
        assert_eq!(frame_status(10, 11, false), FrameStatus::Recreated);
        assert_eq!(frame_status(10, 11, true), FrameStatus::Recreated);
    }

    #[test]
    fn observed_display_or_cleanup_device_loss_stops_all_queue_operations() {
        let timelines = timelines();
        let error = observe_device_result::<_, ()>(&timelines, Err(Error::DeviceLost)).unwrap_err();
        assert!(error.is_device_lost());
        assert!(timelines.is_device_lost());
        let queue = QueueId::new(0).unwrap();
        assert!(
            timelines
                .submit(queue, |_| -> Result<()> {
                    panic!("a display error must prevent the next submit")
                })
                .unwrap_err()
                .is_device_lost()
        );
        assert!(
            timelines
                .with_queue(queue, || -> Result<()> {
                    panic!("a display error must prevent the next present")
                })
                .unwrap_err()
                .is_device_lost()
        );
        assert!(timelines.poll().unwrap_err().is_device_lost());
        assert!(
            timelines
                .wait(&SyncSet::EMPTY, 0)
                .unwrap_err()
                .is_device_lost()
        );
        // A later successful CPU cleanup cannot clear the terminal state.
        observe_device_result(&timelines, Ok(())).unwrap();
        assert!(timelines.is_device_lost());
    }

    #[test]
    fn recoverable_display_errors_keep_the_original_error_and_allow_the_next_submit() {
        for original in [
            Error::Timeout,
            Error::OutOfHostMemory,
            Error::SurfaceOutOfDate,
        ] {
            let timelines = timelines();
            let kind = original.kind();
            let result = observe_device_result::<_, ()>(&timelines, Err(original));
            assert_eq!(result.unwrap_err().kind(), kind);
            assert!(!timelines.is_device_lost());
            let (point, ()) = timelines
                .submit(QueueId::new(0).unwrap(), |_| Ok(()))
                .unwrap();
            assert_eq!(point.value, 1);
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Failure {
        NoImage,
        Acquire,
        GraphFirstSubmit,
        GraphPartialSubmit,
        Display,
        DisplayPanic,
        Present,
    }
    struct ScriptedOperations {
        timelines: Timelines<NoGpuCalls>,
        failure: Option<Failure>,
        inject_device_loss: bool,
        cancel_error: Option<Error>,
        events: Vec<&'static str>,
        pending_retire: Vec<super::super::graph::RetireCallback>,
    }
    impl ScriptedOperations {
        fn new(failure: Failure) -> Self {
            Self {
                timelines: timelines(),
                failure: Some(failure),
                inject_device_loss: false,
                cancel_error: None,
                events: Vec::new(),
                pending_retire: Vec::new(),
            }
        }
        fn take_failure(&mut self, failure: Failure) -> bool {
            if self.failure == Some(failure) {
                self.failure = None;
                true
            } else {
                false
            }
        }
        fn error(&self) -> Error {
            if self.inject_device_loss {
                Error::DeviceLost
            } else {
                Error::OutOfDeviceMemory
            }
        }
        fn complete_graphs(&mut self) {
            super::super::vulkan::run_retire_callbacks(std::mem::take(&mut self.pending_retire))
                .unwrap();
        }
    }
    impl FrameOperations for ScriptedOperations {
        type Lease = ();
        type GraphReceipt = ();
        type Command = ();
        type PresentationReceipt = ();
        fn is_device_lost(&self) -> bool {
            self.timelines.is_device_lost()
        }
        fn observe_error(&mut self, error: &Error) {
            if error.is_device_lost() {
                self.timelines.mark_device_lost();
            }
        }
        fn acquire(&mut self) -> Result<Option<()>> {
            self.events.push("acquire");
            if self.take_failure(Failure::NoImage) {
                Ok(None)
            } else if self.take_failure(Failure::Acquire) {
                Err(self.error())
            } else {
                Ok(Some(()))
            }
        }
        fn execute_graph(&mut self, built: BuiltGraph) -> Result<()> {
            self.events.push("graph");
            let (_, _, retire) = built.into_parts();
            if self.take_failure(Failure::GraphFirstSubmit) {
                let _ = super::super::vulkan::run_retire_callbacks(retire);
                return Err(self.error());
            }
            // This is the Session's ownership boundary, not another GPU
            // transaction model. Both successful and partial submits retain
            // their callbacks until an actual completion is supplied.
            self.pending_retire.extend(retire);
            if self.take_failure(Failure::GraphPartialSubmit) {
                Err(self.error())
            } else {
                Ok(())
            }
        }
        fn record_display(&mut self, _: &()) -> Result<()> {
            self.events.push("record");
            assert!(
                !self.take_failure(Failure::DisplayPanic),
                "injected display panic"
            );
            if self.take_failure(Failure::Display) {
                Err(self.error())
            } else {
                Ok(())
            }
        }
        fn present(&mut self, _: (), _: ()) -> Result<()> {
            self.events.push("present");
            if self.take_failure(Failure::Present) {
                Err(self.error())
            } else {
                Ok(())
            }
        }
        fn cancel(&mut self, _: ()) -> Result<()> {
            self.events.push("cancel");
            self.cancel_error.take().map_or(Ok(()), Err)
        }
        fn abandon_device_lost(&mut self, _: ()) {
            self.events.push("abandon_lost");
        }
    }
    fn graph_with_retire(calls: &std::rc::Rc<std::cell::Cell<usize>>) -> BuiltGraph {
        let mut graph = super::super::graph::RenderGraph::new();
        let calls = calls.clone();
        graph.on_retire(move || calls.set(calls.get() + 1));
        graph.build()
    }

    fn run(
        operations: &mut ScriptedOperations,
        graph: BuiltGraph,
    ) -> Result<Option<()>> {
        execute_frame_protocol(operations, graph).and_then(|output| output.presentation)
    }

    #[test]
    fn graph_is_submitted_before_acquire_and_survives_skipped_presentation() {
        let calls = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut operations = ScriptedOperations::new(Failure::NoImage);
        let output = execute_frame_protocol(&mut operations, graph_with_retire(&calls)).unwrap();
        assert!(matches!(output.presentation, Ok(None)));
        assert_eq!(operations.events, ["graph", "acquire"]);
        assert_eq!(calls.get(), 0, "a skipped frame keeps its submitted graph");
        operations.complete_graphs();
        assert_eq!(calls.get(), 1);
        operations.events.clear();
        assert!(run(&mut operations, graph_with_retire(&calls)).unwrap().is_some());
        assert_eq!(operations.events, ["graph", "acquire", "record", "present"]);
    }

    #[test]
    fn production_frame_protocol_fault_matrix_preserves_retirement_and_recovers_next_frame() {
        for failure in [
            Failure::NoImage,
            Failure::Acquire,
            Failure::GraphFirstSubmit,
            Failure::GraphPartialSubmit,
            Failure::Display,
            Failure::DisplayPanic,
            Failure::Present,
        ] {
            let calls = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut operations = ScriptedOperations::new(failure);
            let output = execute_frame_protocol(&mut operations, graph_with_retire(&calls));
            let graph_failed = matches!(
                failure,
                Failure::GraphFirstSubmit | Failure::GraphPartialSubmit
            );
            assert_eq!(output.is_err(), graph_failed, "{failure:?}");
            let result = output.and_then(|output| output.presentation);
            if failure == Failure::NoImage {
                assert_eq!(result.unwrap(), None);
            } else if failure == Failure::DisplayPanic {
                assert_eq!(result.unwrap_err().kind(), crate::ErrorKind::InvalidUsage);
            } else {
                assert!(
                    matches!(result, Err(Error::OutOfDeviceMemory)),
                    "{failure:?}"
                );
            }
            // A failed graph never acquires; every later stage keeps its graph.
            assert_eq!(operations.events.contains(&"acquire"), !graph_failed);
            let unsubmitted = failure == Failure::GraphFirstSubmit;
            assert_eq!(calls.get(), usize::from(unsubmitted), "{failure:?}");
            let cancelled = matches!(failure, Failure::Display | Failure::DisplayPanic);
            assert_eq!(
                operations
                    .events
                    .iter()
                    .filter(|&&event| event == "cancel")
                    .count(),
                usize::from(cancelled)
            );
            operations.complete_graphs();
            assert_eq!(
                calls.get(),
                1,
                "the first frame retires exactly once: {failure:?}"
            );
            assert!(
                run(&mut operations, graph_with_retire(&calls))
                    .unwrap()
                    .is_some()
            );
            assert_eq!(calls.get(), 1, "submitted work remains retained");
            operations.complete_graphs();
            operations.complete_graphs();
            assert_eq!(
                calls.get(),
                2,
                "next frame retires exactly once: {failure:?}"
            );
        }
    }

    #[test]
    fn acquisition_cleanup_errors_preserve_primary_and_device_loss_is_terminal() {
        for cleanup in [Error::Timeout, Error::OutOfHostMemory, Error::DeviceLost] {
            let lost = cleanup.is_device_lost();
            let calls = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut operations = ScriptedOperations::new(Failure::Display);
            operations.cancel_error = Some(cleanup);
            assert!(matches!(
                run(&mut operations, graph_with_retire(&calls)),
                Err(Error::OutOfDeviceMemory)
            ));
            assert_eq!(calls.get(), 0, "cleanup does not retire submitted batches");
            assert_eq!(operations.is_device_lost(), lost);
            let before = operations.events.len();
            let retry = run(&mut operations, graph_with_retire(&calls));
            if lost {
                assert!(retry.unwrap_err().is_device_lost());
                assert_eq!(
                    operations.events.len(),
                    before,
                    "device loss forbids even graph submission"
                );
            } else {
                assert!(retry.unwrap().is_some());
            }
            operations.complete_graphs();
            assert_eq!(calls.get(), 2);
        }
    }

    #[test]
    fn every_device_loss_stage_stops_the_next_frame_without_rescue_submission() {
        for failure in [
            Failure::Acquire,
            Failure::GraphFirstSubmit,
            Failure::GraphPartialSubmit,
            Failure::Display,
            Failure::Present,
        ] {
            let calls = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut operations = ScriptedOperations::new(failure);
            operations.inject_device_loss = true;
            assert!(
                run(&mut operations, graph_with_retire(&calls))
                    .unwrap_err()
                    .is_device_lost()
            );
            assert!(operations.is_device_lost());
            assert!(
                !operations.events.contains(&"cancel"),
                "no rescue submit on device loss"
            );
            let before = operations.events.len();
            assert!(
                run(&mut operations, graph_with_retire(&calls))
                    .unwrap_err()
                    .is_device_lost()
            );
            assert_eq!(operations.events.len(), before);
            operations.complete_graphs();
            assert_eq!(calls.get(), 2);
        }
    }

    #[test]
    fn device_loss_keeps_its_error_even_when_an_unsubmitted_retire_callback_panics() {
        let calls = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut graph = super::super::graph::RenderGraph::new();
        graph.on_retire(|| panic!("injected unsubmitted retirement panic"));
        let retired = calls.clone();
        graph.on_retire(move || retired.set(retired.get() + 1));
        let mut operations = ScriptedOperations::new(Failure::NoImage);
        operations.timelines.mark_device_lost();
        let error = run(&mut operations, graph.build()).unwrap_err();
        assert!(error.is_device_lost());
        assert_eq!(calls.get(), 1);
        assert!(operations.events.is_empty());
    }
}
