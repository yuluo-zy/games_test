//! 实际 Vulkan 时间线信号量。候选接口复用原锁定 ash，不模拟 GPU 完成。
use super::device::Device;
use ash::vk;
use std::sync::Arc;
pub struct TimelineSemaphore {
    pub(crate) handle: vk::Semaphore,
    pub(crate) device: Arc<Device>,
    queue: Option<crate::sync::QueueId>,
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    _memory: super::allocator::CpuMemoryReservation,
}

impl TimelineSemaphore {
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::Semaphore {
        self.handle
    }
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}
impl TimelineSemaphore {
    pub fn new(device: Arc<Device>, initial: u64) -> crate::Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let mut timeline = vk::SemaphoreTypeCreateInfo::default()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(initial);
        let info = vk::SemaphoreCreateInfo::default().push_next(&mut timeline);
        let memory =
            super::allocator::reserve_shared_cpu_metadata::<Self>(&device.memory.stats, 0)?;
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let handle = super::error::observe_vulkan(&device, unsafe {
            device.core.device.create_semaphore(&info, None)
        })?;
        Ok(Arc::new(Self {
            handle,
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            queue: None,
            _memory: memory,
        }))
    }
    pub(crate) fn for_queue(
        device: Arc<Device>,
        queue: crate::sync::QueueId,
    ) -> crate::Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let memory =
            super::allocator::reserve_shared_cpu_metadata::<Self>(&device.memory.stats, 0)?;
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let handle = device.queues.timelines.semaphore(queue)?;
        Ok(Arc::new(Self {
            handle,
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            queue: Some(queue),
            _memory: memory,
        }))
    }
    pub fn completed(&self) -> crate::Result<u64> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if let Some(queue) = self.queue {
            return self
                .device
                .queues
                .timelines
                .poll()
                .map(|done| done.get(queue));
        }
        super::error::observe_vulkan(&self.device, unsafe {
            self.device
                .core
                .device
                .get_semaphore_counter_value(self.handle)
        })
    }
    pub fn wait(&self, value: u64, timeout: u64) -> crate::Result<()> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if let Some(queue) = self.queue {
            return self.device.queues.timelines.wait(
                &crate::sync::SyncSet::from_point(crate::sync::SyncPoint { queue, value }),
                timeout,
            );
        }
        let semaphores = [self.handle];
        let values = [value];
        let info = vk::SemaphoreWaitInfo::default()
            .semaphores(&semaphores)
            .values(&values);
        super::error::observe_vulkan(&self.device, unsafe {
            self.device.core.device.wait_semaphores(&info, timeout)
        })
    }
}
impl Drop for TimelineSemaphore {
    fn drop(&mut self) {
        if self.queue.is_some() {
            return;
        }
        self.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::Semaphore(self.handle),
        );
    }
}

impl crate::resource::GpuUse for TimelineSemaphore {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;

        Ok(())
    }
}
