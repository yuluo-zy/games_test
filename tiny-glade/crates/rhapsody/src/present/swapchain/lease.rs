//! Acquired-image leases and present tickets.
use super::wsi::WsiFenceOperations;
use ash::vk;
use crate::Error;
use crate::Result;
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::command::command_buffer::VulkanCommandPool;
use crate::device::Device;
use std::sync::Arc;
use super::ACQUIRE_TIMEOUT_NS;
use super::Swapchain;
use super::frame_sync::FrameSemaphores;
use super::frame_sync::color_barrier;
use super::policy::PresentPolicy;
use super::policy::SwapchainPolicy;
use super::progress::ImagePresentation;
use super::wsi::DeviceFenceOperations;
use super::wsi::wsi_call;

pub struct SwapchainImageLease {
    pub swapchain: Arc<Swapchain>,
    pub index: u32,
    pub suboptimal: bool,
    pub(super) released: bool,
    pub(super) external_acquire_fence: vk::Fence,
}

impl SwapchainImageLease {
    pub fn image(&self) -> vk::Image {
        self.swapchain.images[self.index as usize]
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.swapchain.views[self.index as usize]
    }
    /// # Safety
    /// The image layout, rendering completion and semaphore ownership must
    /// satisfy [`Swapchain::present`]'s contract.
    pub unsafe fn present(mut self, wait: &[vk::Semaphore]) -> Result<bool> {
        let result = unsafe { self.swapchain.present_unnoted(self.index, wait) };
        // Early failures may precede native presentation. Returning an unconsumed
        // acquisition makes its generation stale through the existing Drop path.
        self.released = result.is_ok();
        if result
            .as_ref()
            .is_ok_and(|&present| present || self.suboptimal)
        {
            self.swapchain.note_suboptimal();
        }
        result.map(|present| present || self.suboptimal)
    }
}

impl Drop for SwapchainImageLease {
    fn drop(&mut self) {
        let unproven = self.external_acquire_fence != vk::Fence::null()
            && DeviceFenceOperations(&self.swapchain.device)
                .wait(self.external_acquire_fence, ACQUIRE_TIMEOUT_NS)
                .is_err_and(|error| !error.is_device_lost());
        let mut inner = self.swapchain.inner.write();
        inner.unproven_external_acquire |= unproven;
        inner.return_acquisition(self.released);
    }
}

/// An acquired image accepts exactly one display submission. Dropping it
/// unconsumed cancels the acquisition and retires the generation.
pub struct AcquiredImageLease {
    pub(super) swapchain: Arc<Swapchain>,
    pub(super) index: u32,
    pub(super) lease: Option<SwapchainImageLease>,
    pub(super) sync: Arc<FrameSemaphores>,
    pub(super) slot_index: usize,
    pub(super) presentation: Arc<ImagePresentation>,
    pub(super) recover_on_drop: bool,
    pub(super) on_failure: PresentPolicy,
}

pub(super) fn consumed() -> Error {
    Error::invalid_usage("交换链图像租约已被消费")
}

impl AcquiredImageLease {
    pub(crate) fn abandon_device_lost(mut self) {
        // Drop only releases CPU bookkeeping. No fence waits or rescue submits
        // are valid once the caller has observed device loss.
        self.recover_on_drop = false;
    }
    pub fn generation(&self) -> u64 {
        self.swapchain.generation()
    }
    pub(crate) fn device(&self) -> &Arc<Device> {
        &self.swapchain.device
    }
    /// Whether this image was handed to the presentation engine before in
    /// this generation, i.e. it is in PRESENT_SRC_KHR with defined contents.
    pub(crate) fn was_presented(&self) -> bool {
        self.presentation.progress.lock().issued > 0
    }
    pub fn image(&self) -> vk::Image {
        self.swapchain.images[self.index as usize]
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.swapchain.views[self.index as usize]
    }
    pub fn format(&self) -> vk::Format {
        self.swapchain.format
    }
    pub fn index(&self) -> u32 {
        self.index
    }
    pub fn slot_index(&self) -> usize {
        self.slot_index
    }
    pub fn acquire_semaphore(&self) -> vk::Semaphore {
        self.sync.acquired
    }
    pub fn render_semaphore(&self) -> vk::Semaphore {
        self.presentation.rendered
    }
    pub fn submission_fence(&self) -> vk::Fence {
        self.sync.fence
    }
    pub fn extent(&self) -> vk::Extent2D {
        self.swapchain.extent
    }
    /// A graph can borrow this actual acquired image while its generation is
    /// retained. Acquisition and present remain owned by the linear WSI lease.
    pub(crate) fn texture(&self) -> Result<Arc<crate::resource::texture::Texture>> {
        let chain = &self.swapchain;
        let desc = crate::resource::texture::TextureDesc::new_2d(
            chain.extent.width,
            chain.extent.height,
            chain.format,
        )
        .usage(chain.usage);
        Ok(Arc::new(crate::resource::texture::Texture::from_swapchain_image(
            chain.clone(),
            chain.device.clone(),
            self.image(),
            desc,
        )?))
    }
    /// Cancel before any submit has consumed the acquire semaphore, per the
    /// image's [`PresentPolicy`]: release it (maintenance1), or submit a real
    /// clear that consumes the acquire exactly once and present that.
    pub fn cancel(mut self, timeout: u64) -> Result<()> {
        self.recover_on_drop = false;
        self.cancel_unsubmitted(timeout)
    }
    fn cancel_unsubmitted(&mut self, timeout: u64) -> Result<()> {
        let chain = self.swapchain.clone();
        if self.lease.is_none() {
            return Err(consumed());
        }
        if chain.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if chain.policy == SwapchainPolicy::AcquireOnly {
            self.sync
                .finish_acquire(&mut self.sync.state.lock(), timeout)?;
            // An acquired image may be destroyed with its swapchain after its
            // acquisition completes. It cannot be acquired again without a
            // presentation or release, so retire this acquire-only generation.
            chain.mark_stale();
            self.sync.state.lock().abandoned = true;
            self.sync.release();
            self.lease.take();
            return Ok(());
        }
        self.return_without_content(crate::sync::SyncSet::EMPTY, false, timeout)
    }
    /// Return an image that never received complete content, per
    /// [`PresentPolicy`]. `acquire_consumed` says an earlier submit already
    /// waited the acquire semaphore (it is never waited a second time);
    /// `after` holds every submitted use of the image.
    ///
    /// - `SkipPresent` with a usable release call releases the image; no
    ///   frame reaches the screen.
    /// - Otherwise a real submit consumes the pending acquire wait (or
    ///   chains after `after`), clears the image and presents it, so the
    ///   acquire/render semaphores stay balanced and nothing is leaked.
    /// - Only when neither release nor a clear is possible the contents are
    ///   discarded and presented (undefined, logged).
    pub(crate) fn return_without_content(
        &mut self,
        after: crate::sync::SyncSet,
        acquire_consumed: bool,
        timeout: u64,
    ) -> Result<()> {
        // If recovery itself fails, retire this generation instead of
        // recursively attempting another recovery from Drop.
        self.recover_on_drop = false;
        if self.swapchain.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let chain = self.swapchain.clone();
        let can_clear = chain.usage.contains(vk::ImageUsageFlags::TRANSFER_DST);
        let color = match self.on_failure {
            PresentPolicy::ClearTo(color) if can_clear => Some(color),
            _ => None,
        };
        if color.is_none() && chain.can_release_images() {
            return self.release_unpresented(after, acquire_consumed, timeout);
        }
        let color = color.or_else(|| can_clear.then_some(PresentPolicy::FALLBACK_COLOR));
        if color.is_none() {
            log::warn!(
                target: "rhapsody::raw::swapchain",
                "cannot release or clear an abandoned swapchain image; presenting undefined contents"
            );
        }
        self.present_without_content(color, after, acquire_consumed)
            .map(drop)
    }
    /// Release the acquired image without presenting (maintenance1).
    fn release_unpresented(
        &mut self,
        after: crate::sync::SyncSet,
        acquire_consumed: bool,
        timeout: u64,
    ) -> Result<()> {
        let chain = self.swapchain.clone();
        let index = self.lease.as_ref().ok_or_else(consumed)?.index;
        if acquire_consumed {
            // Every submitted use must finish before the image can be
            // acquired again; the acquire semaphore itself is already spent.
            chain.device.queues.timelines.wait(&after, timeout)?;
        } else {
            self.sync
                .finish_acquire(&mut self.sync.state.lock(), timeout)?;
        }
        chain.release_image(index)?;
        {
            let mut state = self.sync.state.lock();
            // Release-images does not consume a pending acquire signal.
            state.abandoned |= !acquire_consumed;
            state.reserved = false;
        }
        if let Some(mut lease) = self.lease.take() {
            lease.released = true;
        }
        Ok(())
    }
    /// Submit a command that leaves the image in PRESENT_SRC_KHR (cleared to
    /// `color`, or with discarded contents) and present it.
    fn present_without_content(
        &mut self,
        color: Option<[f32; 4]>,
        after: crate::sync::SyncSet,
        acquire_consumed: bool,
    ) -> Result<PresentTicket> {
        let source = match (acquire_consumed, color) {
            (true, _) => vk::PipelineStageFlags::ALL_COMMANDS,
            (false, Some(_)) => vk::PipelineStageFlags::TRANSFER,
            (false, None) => vk::PipelineStageFlags::TOP_OF_PIPE,
        };
        let command = match color {
            Some(color) => self.clear_to_present_command(color, source)?,
            None => self.discard_to_present_command(source)?,
        };
        if acquire_consumed {
            self.submit_with_stages(command, None, vk::PipelineStageFlags::ALL_COMMANDS, after)
        } else {
            // The barrier's source scope is the acquire wait stage, chaining
            // the layout transition after that wait.
            self.submit_with_stages(
                command,
                Some(source),
                vk::PipelineStageFlags::TOP_OF_PIPE,
                crate::sync::SyncSet::EMPTY,
            )
        }
    }
    /// The policy applied when this image is abandoned (default
    /// [`PresentPolicy::SkipPresent`]).
    pub fn failure_policy(&self) -> PresentPolicy {
        self.on_failure
    }
    /// Choose what happens if this acquisition is abandoned; unsupported
    /// policies are rejected without changing the current one.
    pub fn set_failure_policy(&mut self, policy: PresentPolicy) -> Result<()> {
        policy.validate()?;
        self.on_failure = policy;
        Ok(())
    }
    /// `UNDEFINED -> TRANSFER_DST -> clear -> PRESENT_SRC_KHR` on the
    /// presentation queue family. `source` must cover the stage at which the
    /// submit waits, so the first transition is chained after that wait.
    fn clear_to_present_command(
        &self,
        color: [f32; 4],
        source: vk::PipelineStageFlags,
    ) -> Result<VulkanCommandBuffer> {
        let chain = &self.swapchain;
        let pool = wsi_call(&chain.device.queues.timelines, || {
            VulkanCommandPool::new(chain.device.clone())
        })?;
        let mut command = wsi_call(&chain.device.queues.timelines, || pool.allocate())?;
        wsi_call(&chain.device.queues.timelines, || command.begin())?;
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        let mut to_clear =
            crate::command::barrier::SynchronizationScope::new(source, vk::PipelineStageFlags::TRANSFER);
        to_clear.images.push(
            color_barrier(self.image())
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE),
        );
        to_clear.issue_into(&command)?;
        unsafe {
            chain.device.core.device.cmd_clear_color_image(
                command.handle,
                self.image(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue { float32: color },
                &[range],
            );
        }
        let mut to_present = crate::command::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
        );
        to_present.images.push(
            color_barrier(self.image())
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE),
        );
        to_present.issue_into(&command)?;
        wsi_call(&chain.device.queues.timelines, || command.end())?;
        Ok(command)
    }
    /// One command holding only `UNDEFINED -> PRESENT_SRC_KHR` on the
    /// presentation queue family. `source` must cover the stage at which the
    /// submit waits, so the layout transition is chained after that wait.
    fn discard_to_present_command(
        &self,
        source: vk::PipelineStageFlags,
    ) -> Result<VulkanCommandBuffer> {
        let chain = &self.swapchain;
        let pool = wsi_call(&chain.device.queues.timelines, || {
            VulkanCommandPool::new(chain.device.clone())
        })?;
        let mut command = wsi_call(&chain.device.queues.timelines, || pool.allocate())?;
        wsi_call(&chain.device.queues.timelines, || command.begin())?;
        let mut scope = crate::command::barrier::SynchronizationScope::new(
            source,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
        );
        scope.images.push(
            color_barrier(self.image())
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR),
        );
        scope.issue_into(&command)?;
        wsi_call(&chain.device.queues.timelines, || command.end())?;
        Ok(command)
    }
    /// Submit `command` on the presentation queue and present it. With
    /// `acquire_stage`, the submit consumes the acquire semaphore at that stage
    /// and arms the slot fence that guards the semaphore's reuse; without it,
    /// the acquire wait was already consumed by an earlier (graph) submit.
    fn submit_with_stages(
        &mut self,
        mut command: VulkanCommandBuffer,
        acquire_stage: Option<vk::PipelineStageFlags>,
        dependency_stage: vk::PipelineStageFlags,
        dependencies: crate::sync::SyncSet,
    ) -> Result<PresentTicket> {
        if !command.is_executable()
            || acquire_stage.is_some_and(|stage| stage.is_empty())
            || dependency_stage.is_empty()
        {
            return Err(Error::invalid_usage("呈现命令未结束或等待阶段为空"));
        }
        if self.lease.is_none() {
            return Err(consumed());
        }
        let chain = self.swapchain.clone();
        chain.policy.allow_present()?;
        if !Arc::ptr_eq(&command.pool.device, &chain.device) {
            return Err(Error::invalid_usage("呈现命令来自其他设备"));
        }
        if command.pool.family != chain.device.queues.primary_family {
            return Err(Error::invalid_usage("呈现命令的命令池不属于呈现队列族"));
        }
        command.retain_object(chain.clone())?;
        command.retain_object(self.sync.clone())?;
        command.validate_submission()?;
        let device = chain.device.clone();
        let mut waits = crate::native::submit::SubmitArray::default();
        let mut wait_values = crate::native::submit::SubmitArray::default();
        let mut stages = crate::native::submit::SubmitArray::default();
        let fence = if let Some(acquire_stage) = acquire_stage {
            if let Err(error) = self.presentation.prepare(ACQUIRE_TIMEOUT_NS) {
                self.recover_on_drop &= !error.is_device_lost();
                return Err(error);
            }
            let fence = self.sync.fence;
            if let Err(error) = wsi_call(&device.queues.timelines, || unsafe {
                device.core.device.reset_fences(&[fence])
            }) {
                self.recover_on_drop &= !error.is_device_lost();
                return Err(error);
            }
            waits.push(self.sync.acquired);
            wait_values.push(0);
            stages.push(acquire_stage);
            fence
        } else {
            vk::Fence::null()
        };
        for point in dependencies.points() {
            waits.push(device.queues.timelines.semaphore(point.queue)?);
            wait_values.push(point.value);
            stages.push(dependency_stage);
        }
        let signals = [self.presentation.rendered];
        let commands = [command.handle];
        let uses = crate::tracking::tracked::collect_submission_uses(&[command.submission_uses()?], &device.memory.stats)?;
        let _reservations = crate::tracking::tracked::lock_submission_uses(uses.ordered(), &device.memory.stats)?;
        command.validate_submission()?;
        let native_states = unsafe {
            crate::command::command_buffer::prepare_submission_states(
                &[&command],
                &uses,
                device.queue_id(device.queues.primary)?,
                device.queues.primary_family,
                &device.memory.stats,
            )?
        };
        let mut prepared = [crate::native::submit::PreparedSubmit {
            commands: commands.into_iter().collect(),
            wait_semaphores: waits,
            wait_values,
            wait_stages: stages,
            signal_semaphores: signals.into_iter().collect(),
            signal_values: [0].into_iter().collect(),
        }];
        device.add_timeline_waits(
            device.queues.primary,
            &mut prepared,
            &native_states.dependencies(),
        )?;
        let point = match device.submit_prepared(device.queues.primary, &prepared, fence) {
            Ok(point) => point,
            Err(error) => {
                if error.is_device_lost() {
                    self.recover_on_drop = false;
                }
                return Err(error);
            }
        };
        native_states.publish(point);
        self.presentation
            .progress
            .lock()
            .record_gpu_submission(point);
        command.record_submission(point);
        if acquire_stage.is_some() {
            self.sync.state.lock().submitted = true;
        }
        self.present_submitted(point, Some(command))
    }
    pub(super) fn present_submitted(
        &mut self,
        point: crate::sync::SyncPoint,
        command: Option<VulkanCommandBuffer>,
    ) -> Result<PresentTicket> {
        // acquire has already been consumed. Drop must never rescue it by
        // waiting on the binary semaphore a second time.
        self.recover_on_drop = false;
        let lease = self.lease.take().ok_or_else(consumed)?;
        // After a successful submit, establish the owner before presenting so
        // the in-flight command stays protected whatever present returns.
        let serial = self.presentation.progress.lock().issued + 1;
        let mut pending = PresentTicket {
            command,
            sync: self.sync.clone(),
            point,
            swapchain: self.swapchain.clone(),
            presentation: self.presentation.clone(),
            serial,
            image_index: lease.index,
            suboptimal: lease.suboptimal,
        };
        let chain = self.swapchain.clone();
        // The submitted command owns the layout transition; its binary signal
        // and generation are retained by `pending` and the image protocol.
        match unsafe { lease.present(&[self.presentation.rendered]) } {
            Ok(suboptimal) => {
                pending.suboptimal |= suboptimal;
                Ok(pending)
            }
            Err(error) => {
                chain.mark_stale();
                // A present that never enqueued its wait leaves rendered
                // signaled and the image acquired. acquire was already
                // consumed by the render submit; never wait on it again.
                let enqueued = self.presentation.progress.lock().issued >= serial;
                if !enqueued
                    && !error.is_device_lost()
                    && let Err(cleanup) = pending.finish_rejected_present(ACQUIRE_TIMEOUT_NS)
                {
                    log::warn!(target: "rhapsody::raw::swapchain", "present cleanup failed after {error}: {cleanup}");
                }
                drop(pending);
                Err(error)
            }
        }
    }
}

impl Drop for AcquiredImageLease {
    fn drop(&mut self) {
        if self.lease.is_some()
            && std::mem::take(&mut self.recover_on_drop)
            && let Err(error) = self.cancel_unsubmitted(ACQUIRE_TIMEOUT_NS)
        {
            log::warn!(target: "rhapsody::raw::swapchain", "acquired-image cleanup failed: {error}");
        }
        if self.lease.is_some() {
            self.swapchain.mark_stale();
            let mut state = self.sync.state.lock();
            state.abandoned = true;
            state.reserved = false;
        }
    }
}

/// Render and presentation have different completion domains. Dropping a ticket
/// releases CPU ownership without an explicit GPU wait. Final destruction of
/// the last swapchain owner still runs the generation's teardown protocol.
pub struct PresentTicket {
    command: Option<VulkanCommandBuffer>,
    sync: Arc<FrameSemaphores>,
    swapchain: Arc<Swapchain>,
    presentation: Arc<ImagePresentation>,
    serial: u64,
    image_index: u32,
    pub point: crate::sync::SyncPoint,
    pub suboptimal: bool,
}

impl PresentTicket {
    fn finish_rejected_present(&self, timeout: u64) -> Result<()> {
        self.wait(timeout)?;
        let device = &self.swapchain.device;
        let point = device.submit_prepared(
            device.queues.primary,
            &[crate::native::submit::PreparedSubmit {
                commands: Default::default(),
                wait_semaphores: [self.presentation.rendered].into_iter().collect(),
                wait_values: [0].into_iter().collect(),
                wait_stages: [vk::PipelineStageFlags::TOP_OF_PIPE].into_iter().collect(),
                signal_semaphores: Default::default(),
                signal_values: Default::default(),
            }],
            vk::Fence::null(),
        )?;
        self.presentation
            .progress
            .lock()
            .record_gpu_submission(point);
        device.queues.timelines.wait(&point.into(), timeout)?;
        if device
            .caps()
            .is_enabled(crate::device::Feature::SwapchainMaintenance1)
        {
            self.swapchain.release_image(self.image_index)?;
        }
        Ok(())
    }
    pub fn generation(&self) -> u64 {
        self.swapchain.generation()
    }
    /// Wait only for rendering. This does not authorize present semaphore reuse.
    pub fn wait(&self, timeout: u64) -> Result<()> {
        self.sync
            .device
            .queues
            .timelines
            .wait(&self.point.into(), timeout)
    }
    pub fn is_complete(&self) -> Result<bool> {
        Ok(crate::sync::SyncSet::from(self.point)
            .is_reached(&self.sync.device.queues.timelines.poll()?))
    }
    /// Without maintenance1, a newer acquisition fence proves presentation
    /// completion. Shutdown never treats queue-idle as presentation completion.
    pub fn is_present_complete(&self) -> Result<bool> {
        self.swapchain.collect_presentations()?;
        Ok(self.presentation.progress.lock().completed(self.serial))
    }
}

impl Drop for PresentTicket {
    fn drop(&mut self) {
        self.command.take();
        self.sync.release();
    }
}
