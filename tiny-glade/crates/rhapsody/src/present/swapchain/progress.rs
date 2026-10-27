//! Per-image presentation progress and fence tracking.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::device::Device;
use parking_lot::Mutex;
use std::sync::Arc;
use super::ACQUIRE_TIMEOUT_NS;
use super::wsi::DeviceFenceOperations;
use super::wsi::WsiFenceOperations;
use super::wsi::present_fence_is_certain;
use super::wsi::present_was_enqueued;
use super::wsi::wsi_call;

/// One semaphore per *image*, independent of the rotating acquire slots.
/// Reacquisition proves the previous present wait was consumed only after its
/// acquire fence signals. The generation itself owns ambiguous present failures.
pub(super) struct ImagePresentation {
    device: Arc<Device>,
    pub(super) rendered: vk::Semaphore,
    pub(super) fence: vk::Fence,
    pub(super) progress: Mutex<PresentProgress>,
}

/// A present fence that the image's next present or teardown must consume.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PendingFence {
    pub(super) serial: u64,
    /// False for presents the presentation engine rejected: their fence may
    /// never signal, so it is polled but never waited on.
    pub(super) certain: bool,
}

#[derive(Default)]
pub(super) struct PresentProgress {
    pub(super) issued: u64,
    pub(super) completed: u64,
    pub(super) pending_fence: Option<PendingFence>,
    // Render and rejected-present cleanup submits can both access rendered.
    // Fixed-capacity registration is infallible immediately after submit.
    pub(super) gpu_use: crate::sync::SyncSet,
}

impl PresentProgress {
    pub(super) fn generation_completed(&mut self) {
        self.completed = self.issued;
    }
    /// Final teardown. A rejected present only enqueued its semaphore wait
    /// (no image reached the presentation engine); once the device timeline
    /// passed its render submission (`finish_gpu_use`, done by the caller)
    /// that wait has been satisfied, so an unsignaled uncertain fence does
    /// not stall teardown.
    pub(super) fn finish_shutdown_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        fence: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if let Some(pending) = self.pending_fence
            && !pending.certain
        {
            if !operations.signaled(fence)? {
                log::debug!(target: "rhapsody::raw::swapchain", "rejected present fence unsignaled at teardown; using the render timeline proof");
            }
            self.acquired(pending.serial);
            self.pending_fence = None;
        }
        self.finish_with(operations, fence, timeout)?;
        if self.all_presentations_complete() {
            Ok(())
        } else {
            Err(Error::unsupported("最终呈现尚无完成证明；保留交换链所有权"))
        }
    }
    /// Consume a pending certain fence with a bounded wait. An uncertain one
    /// completes only if already signaled; otherwise this returns
    /// SurfaceOutOfDate instead of waiting for a signal that may never come.
    fn finish_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        fence: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if let Some(pending) = self.pending_fence {
            if pending.certain {
                operations.wait(fence, timeout)?;
            } else if !operations.signaled(fence)? {
                return Err(Error::SurfaceOutOfDate);
            }
            self.acquired(pending.serial);
            self.pending_fence = None;
        }
        Ok(())
    }
    #[cfg(test)]
    pub(super) fn prepare_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        fence: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        self.finish_with(operations, fence, timeout)?;
        if fence != vk::Fence::null() {
            operations.reset(fence)?;
        }
        Ok(())
    }
    pub(super) fn poll_with(&mut self, operations: &impl WsiFenceOperations, fence: vk::Fence) -> Result<()> {
        if self.pending_fence.is_some() && operations.signaled(fence)? {
            self.finish_with(operations, fence, 0)?;
        }
        Ok(())
    }
    pub(super) fn all_presentations_complete(&self) -> bool {
        self.completed == self.issued
    }
    pub(super) fn record_gpu_submission(&mut self, point: crate::sync::SyncPoint) {
        self.gpu_use.insert(point);
    }
    pub(super) fn retirement_ready(&self, completed: &crate::sync::CompletedValues) -> bool {
        self.all_presentations_complete() && self.gpu_use.is_reached(completed)
    }
    pub(super) fn record_result(&mut self, result: std::result::Result<bool, vk::Result>, has_fence: bool) {
        if present_was_enqueued(result) {
            let serial = self.issue();
            if has_fence {
                self.pending_fence = Some(PendingFence {
                    serial,
                    certain: present_fence_is_certain(result),
                });
            }
        }
    }
    pub(super) fn issue(&mut self) -> u64 {
        self.issued += 1;
        self.issued
    }
    pub(super) fn acquired(&mut self, serial: u64) {
        self.completed = self.completed.max(serial);
    }
    pub(super) fn completed(&self, serial: u64) -> bool {
        self.completed >= serial
    }
}

impl ImagePresentation {
    pub(super) fn finish_shutdown(&self, timeout: u64) -> Result<()> {
        self.progress.lock().finish_shutdown_with(
            &DeviceFenceOperations(&self.device),
            self.fence,
            timeout,
        )
    }
    pub(super) fn finish_gpu_use(&self, timeout: u64) -> Result<()> {
        let after = self.progress.lock().gpu_use;
        self.device.queues.timelines.wait(&after, timeout)
    }
    pub(super) fn poll(&self) -> Result<()> {
        self.progress
            .lock()
            .poll_with(&DeviceFenceOperations(&self.device), self.fence)
    }
    pub(super) fn prepare(&self, timeout: u64) -> Result<()> {
        let pending = self.progress.lock().pending_fence;
        let operations = DeviceFenceOperations(&self.device);
        if let Some(pending) = pending {
            if pending.certain { operations.wait(self.fence, timeout)?; }
            else if !operations.signaled(self.fence)? { return Err(Error::SurfaceOutOfDate); }
            let mut progress = self.progress.lock();
            if progress.pending_fence == Some(pending) {
                progress.acquired(pending.serial);
                progress.pending_fence = None;
            }
        }
        if self.fence != vk::Fence::null() { operations.reset(self.fence)?; }
        Ok(())
    }
    pub(super) fn new(device: Arc<Device>) -> Result<Self> {
        let rendered = wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
        })?;
        let fence = if device
            .caps()
            .is_enabled(crate::device::Feature::SwapchainMaintenance1)
        {
            match wsi_call(&device.queues.timelines, || unsafe {
                device
                    .core
                    .device
                    .create_fence(&vk::FenceCreateInfo::default(), None)
            }) {
                Ok(fence) => fence,
                Err(error) => {
                    unsafe {
                        device.core.device.destroy_semaphore(rendered, None);
                    }
                    return Err(error);
                }
            }
        } else {
            vk::Fence::null()
        };
        Ok(Self {
            device,
            rendered,
            fence,
            progress: Mutex::new(PresentProgress::default()),
        })
    }
}

impl Drop for ImagePresentation {
    fn drop(&mut self) {
        // A timed-out cleanup wait still owns a real successful submit point.
        // Protect rendered independently from presentation completion proofs.
        let completed = self
            .finish_gpu_use(ACQUIRE_TIMEOUT_NS)
            .and_then(|()| self.finish_shutdown(ACQUIRE_TIMEOUT_NS));
        if let Err(error) = completed
            && !error.is_device_lost()
        {
            log::error!(target: "rhapsody::raw::swapchain", "present resources retained: {error}");
            std::mem::forget(self.device.clone());
            return;
        }
        unsafe {
            self.device
                .core
                .device
                .destroy_semaphore(self.rendered, None);
            if self.fence != vk::Fence::null() {
                self.device.core.device.destroy_fence(self.fence, None);
            }
        }
    }
}
