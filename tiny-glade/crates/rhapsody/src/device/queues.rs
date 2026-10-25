//! [`Queues`]: the device's queues, per-queue timelines and the submission
//! side state (pending ownership acquires, deferred host flushes).
//!
//! Locks owned here: one external-synchronization lock per queue (inside
//! [`Timelines`]; `with_all_queues` takes them in queue-index order) and the
//! pending-acquire list. Neither is held while taking another device lock,
//! except that draining pending acquires reads resource sync cells.
use ash::vk;

use crate::sync::timeline::Timelines;

#[derive(Clone, Copy, Debug)]
pub struct DeviceQueue {
    pub handle: vk::Queue,
    pub family: u32,
    pub index: u32,
}

pub(crate) struct Queues {
    /// `QueueId(0)`: queue 0 of the primary (graphics) family.
    pub(crate) primary: vk::Queue,
    pub(crate) primary_family: u32,
    /// `QueueId(1..)` in [`crate::device::QueuePlan`] order.
    pub(crate) additional: Vec<DeviceQueue>,
    /// 配置入口创建时保留的类别映射；直接提供 VkDeviceCreateInfo 的底层入口无此映射。
    pub(crate) plan: crate::device::QueuePlan,
    /// One timeline semaphore per logical queue; destroyed when this drops.
    pub(crate) timelines: Timelines<ash::Device>,
    /// Released ownership ranges whose acquire is coalesced into the next
    /// submit on the destination family (see `Device::queue_pending_acquire`).
    pub(crate) pending_acquires: crate::device::PendingAcquires,
    /// Non-coherent dynamic-constants writes flushed before the next submit.
    pub(crate) host_flush: crate::raw::dynamic_constants::HostFlushQueue,
}

impl Queues {
    /// Fetch the queues requested by `info` and create one timeline semaphore
    /// per logical queue. On error nothing created here is leaked.
    ///
    /// # Safety
    /// `device` must have been created from `info`.
    pub(crate) unsafe fn create(
        device: &ash::Device,
        info: &vk::DeviceCreateInfo<'_>,
        plan: crate::device::QueuePlan,
    ) -> crate::Result<Self> {
        let primary_family = plan.queues[0].family;
        unsafe {
            let primary = device.get_device_queue(primary_family, 0);
            let requested = std::slice::from_raw_parts(
                info.p_queue_create_infos,
                info.queue_create_info_count as usize,
            );
            let mut additional = Vec::new();
            for request in requested {
                for index in 0..request.queue_count {
                    if request.queue_family_index == primary_family && index == 0 {
                        continue;
                    }
                    additional.push(DeviceQueue {
                        handle: device.get_device_queue(request.queue_family_index, index),
                        family: request.queue_family_index,
                        index,
                    });
                }
            }
            // VkDeviceQueueCreateInfo 按族排序，逻辑队列按 Graphics/Compute/Transfer 分配。
            // 统一顺序，确保 QueueId 与设备时间线、additional 一一对应。
            additional.sort_by_key(|queue| {
                plan.queues
                    .iter()
                    .position(|physical| {
                        physical.family == queue.family && physical.index == queue.index
                    })
                    .unwrap_or(usize::MAX)
            });
            if additional.len() + 1 > crate::sync::MAX_QUEUES {
                return Err(crate::Error::invalid_usage("队列配置超过设备时间线容量"));
            }
            let mut semaphores = Vec::new();
            for _ in 0..=additional.len() {
                let mut ty = vk::SemaphoreTypeCreateInfo::default()
                    .semaphore_type(vk::SemaphoreType::TIMELINE);
                match device
                    .create_semaphore(&vk::SemaphoreCreateInfo::default().push_next(&mut ty), None)
                {
                    Ok(semaphore) => semaphores.push(semaphore),
                    Err(error) => {
                        for semaphore in semaphores {
                            device.destroy_semaphore(semaphore, None);
                        }
                        return Err(error.into());
                    }
                }
            }
            let timelines = match Timelines::new(device.clone(), &semaphores) {
                Ok(timelines) => timelines,
                Err(error) => {
                    for semaphore in semaphores {
                        device.destroy_semaphore(semaphore, None);
                    }
                    return Err(error);
                }
            };
            Ok(Self {
                primary,
                primary_family,
                additional,
                plan,
                timelines,
                pending_acquires: Default::default(),
                host_flush: Default::default(),
            })
        }
    }
}

impl Drop for Queues {
    fn drop(&mut self) {
        // Runs after the device drained its retirements and before the
        // `VkDevice` is destroyed (see `Device` field order).
        for index in 0..self.timelines.queue_count() {
            if let Some(semaphore) = crate::sync::QueueId::new(index)
                .and_then(|queue| self.timelines.semaphore(queue).ok())
            {
                // SAFETY: the device is idle and nothing references the
                // timeline semaphores past this point.
                unsafe { self.timelines.api().destroy_semaphore(semaphore, None) };
            }
        }
    }
}

impl crate::device::Device {
    /// Interop escape hatch: queue 0 of the primary family (`QueueId(0)`).
    /// Submitting to it directly bypasses the queue's external-synchronization
    /// lock and timeline; use [`Self::submit`] instead.
    pub fn primary_queue(&self) -> vk::Queue {
        self.queues.primary
    }
    /// Queues other than [`Self::primary_queue`], indexed by `QueueId(1..)`.
    pub fn additional_queues(&self) -> &[DeviceQueue] {
        &self.queues.additional
    }
    /// Class → queue mapping chosen at creation.
    pub fn queue_plan(&self) -> &crate::device::QueuePlan {
        &self.queues.plan
    }
    /// Per-queue timelines (submitted/completed values, device-lost state).
    pub fn timelines(&self) -> &Timelines<ash::Device> {
        &self.queues.timelines
    }
}
