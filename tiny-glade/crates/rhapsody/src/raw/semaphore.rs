//! 实际 Vulkan 时间线信号量。候选接口复用原锁定 ash，不模拟 GPU 完成。
use super::device::VulkanDevice;
use ash::vk;
use std::sync::Arc;
pub struct TimelineSemaphore {
    pub handle: vk::Semaphore,
    pub device: Arc<VulkanDevice>,
    queue: Option<crate::sync::QueueId>,
}
impl TimelineSemaphore {
    pub fn new(device: Arc<VulkanDevice>, initial: u64) -> Result<Arc<Self>, vk::Result> {
        let mut timeline = vk::SemaphoreTypeCreateInfo::default()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(initial);
        let info = vk::SemaphoreCreateInfo::default().push_next(&mut timeline);
        let handle = unsafe { device.device.create_semaphore(&info, None)? };
        Ok(Arc::new(Self {
            handle,
            device,
            queue: None,
        }))
    }
    pub(crate) fn for_queue(
        device: Arc<VulkanDevice>,
        queue: crate::sync::QueueId,
    ) -> crate::Result<Arc<Self>> {
        let handle = device.timelines.semaphore(queue)?;
        Ok(Arc::new(Self {
            handle,
            device,
            queue: Some(queue),
        }))
    }
    pub fn completed(&self) -> Result<u64, vk::Result> {
        if let Some(queue) = self.queue {
            return self
                .device
                .timelines
                .poll()
                .map(|done| done.get(queue))
                .map_err(crate::device::legacy_vk_error);
        }
        unsafe { self.device.device.get_semaphore_counter_value(self.handle) }
    }
    pub fn wait(&self, value: u64, timeout: u64) -> Result<(), vk::Result> {
        if let Some(queue) = self.queue {
            return self
                .device
                .timelines
                .wait(
                    &crate::sync::SyncSet::from_point(crate::sync::SyncPoint { queue, value }),
                    timeout,
                )
                .map_err(crate::device::legacy_vk_error);
        }
        let semaphores = [self.handle];
        let values = [value];
        let info = vk::SemaphoreWaitInfo::default()
            .semaphores(&semaphores)
            .values(&values);
        unsafe { self.device.device.wait_semaphores(&info, timeout) }
    }
}
impl Drop for TimelineSemaphore {
    fn drop(&mut self) {
        if self.queue.is_some() {
            return;
        }
        self.device
            .bury(super::graveyard::Doomed::Semaphore(self.handle));
    }
}
