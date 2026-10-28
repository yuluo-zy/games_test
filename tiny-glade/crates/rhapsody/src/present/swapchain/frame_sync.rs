//! Per-frame semaphores and slot state.
use ash::vk;
use crate::Result;
use crate::device::Device;
use parking_lot::Mutex;
use std::sync::Arc;
use super::ACQUIRE_TIMEOUT_NS;
use super::progress::ImagePresentation;
use super::wsi::DeviceFenceOperations;
use super::wsi::WsiFenceOperations;
use super::wsi::wsi_call;

pub(super) struct FrameSemaphores {
    pub(super) device: Arc<Device>,
    pub(super) acquired: vk::Semaphore,
    pub(super) acquire_ready: vk::Fence,
    pub(super) fence: vk::Fence,
    pub(super) state: Mutex<FrameSlotState>,
}

#[derive(Default)]
pub(super) struct FrameSlotState {
    pub(super) reserved: bool,
    pub(super) submitted: bool,
    pub(super) acquired: bool,
    pub(super) abandoned: bool,
    pub(super) previous_present: Option<(Arc<ImagePresentation>, u64)>,
}

impl FrameSlotState {
    fn finish_acquire_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        acquire_ready: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if self.acquired {
            operations.wait(acquire_ready, timeout)?;
            if let Some((image, serial)) = self.previous_present.take() {
                image.progress.lock().acquired(serial);
            }
            self.acquired = false;
        }
        Ok(())
    }
    pub(super) fn reserve_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        acquire_ready: vk::Fence,
        submitted: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if self.reserved {
            return Err(vk::Result::NOT_READY.into());
        }
        if self.submitted {
            operations.wait(submitted, timeout)?;
            self.submitted = false;
        }
        self.finish_acquire_with(operations, acquire_ready, timeout)?;
        operations.reset(acquire_ready)?;
        self.reserved = true;
        Ok(())
    }
}

impl FrameSemaphores {
    pub(super) fn new(device: Arc<Device>) -> Result<Self> {
        let acquired = wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
        })?;
        let acquire_ready = match wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }) {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.core.device.destroy_semaphore(acquired, None);
                }
                return Err(error);
            }
        };
        let fence = match wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }) {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.core.device.destroy_fence(acquire_ready, None);
                    device.core.device.destroy_semaphore(acquired, None);
                }
                return Err(error);
            }
        };
        Ok(Self {
            device,
            acquired,
            acquire_ready,
            fence,
            state: Mutex::new(FrameSlotState::default()),
        })
    }
    pub(super) fn finish_acquire(&self, state: &mut FrameSlotState, timeout: u64) -> Result<()> {
        state.finish_acquire_with(
            &DeviceFenceOperations(&self.device),
            self.acquire_ready,
            timeout,
        )
    }
    pub(super) fn reserve(&self, timeout: u64) -> Result<()> {
        self.state.lock().reserve_with(
            &DeviceFenceOperations(&self.device),
            self.acquire_ready,
            self.fence,
            timeout,
        )
    }
    pub(super) fn release(&self) {
        self.state.lock().reserved = false;
    }
}

impl Drop for FrameSemaphores {
    fn drop(&mut self) {
        // Abandoned acquire slots can also be replaced within a live generation.
        // Their acquisition fence proves that the unused binary signal operation
        // has finished; it is separate from queue timeline completion.
        let mut state = self.state.lock();
        let result = self
            .finish_acquire(&mut state, ACQUIRE_TIMEOUT_NS)
            .and_then(|()| {
                if !state.submitted {
                    return Ok(());
                }
                wsi_call(&self.device.queues.timelines, || unsafe {
                    self.device
                        .core
                        .device
                        .wait_for_fences(&[self.fence], true, ACQUIRE_TIMEOUT_NS)
                })
            });
        if let Err(error) = result
            && !error.is_device_lost()
        {
            log::error!(target: "rhapsody::raw::swapchain", "acquire resources retained: {error}");
            std::mem::forget(self.device.clone());
            std::mem::forget(state.previous_present.take());
            return;
        }
        unsafe {
            self.device.core.device.destroy_fence(self.fence, None);
            self.device
                .core
                .device
                .destroy_fence(self.acquire_ready, None);
            self.device
                .core
                .device
                .destroy_semaphore(self.acquired, None);
        }
    }
}

impl crate::tracking::GpuUse for FrameSemaphores {
    fn visit_uses(&self, _visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        Ok(())
    }
}

pub(super) fn color_barrier(image: vk::Image) -> vk::ImageMemoryBarrier<'static> {
    vk::ImageMemoryBarrier::default()
        .image(image)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .subresource_range(
            vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1),
        )
}
