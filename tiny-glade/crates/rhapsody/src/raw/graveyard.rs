//! Native object retirement using per-queue completion and bounded maintenance.
use super::device::Device;
use crate::sync::{CompletedValues, SyncSet};
use ash::vk;
use std::sync::atomic::Ordering;
pub enum DeferredMemory {
    Shared(std::sync::Arc<super::transient_backing::SharedBacking>),
    /// A whole dedicated `VkDeviceMemory` object.
    Dedicated {
        handle: vk::DeviceMemory,
        accounting_id: u64,
    },
    /// A range returned to gpu-allocator.
    #[cfg(feature = "suballocation")]
    Sub {
        allocation: super::suballoc::SubAllocation,
    },
}

/// A Vulkan object awaiting destruction.
pub enum Doomed {
    Buffer(vk::Buffer, DeferredMemory),
    Image(vk::Image, DeferredMemory),
    ImageView(vk::ImageView),
    Sampler(vk::Sampler),
    Semaphore(vk::Semaphore),
    QueryPool(vk::QueryPool),
    PipelineLayout(vk::PipelineLayout),
    Pipeline(vk::Pipeline),
    DescriptorSetLayout(vk::DescriptorSetLayout),
    DescriptorPool(std::sync::Arc<super::descriptor_set::DescriptorPoolAllocation>),
    DescriptorSet(
        vk::DescriptorSet,
        std::sync::Arc<super::descriptor_set::DescriptorPoolAllocation>,
    ),
    CommandBuffer(
        vk::CommandBuffer,
        std::sync::Arc<super::command_buffer::CommandPoolAllocation>,
    ),
}

/// Retirement queue state. Per-queue timeline values are reported as-is; sums
/// across queues have no meaning.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct RetirementStats {
    pub pending: usize,
    /// Last successfully submitted value per queue (0 = none).
    pub submitted: [u64; crate::sync::MAX_QUEUES],
    /// Last observed completed value per queue (cached, no device query).
    pub completed: [u64; crate::sync::MAX_QUEUES],
    pub idle_waits: u64,
}
/// Former name of [`RetirementStats`].
pub type GraveyardStats = RetirementStats;
impl Device {
    /// Never-submitted objects are destroyed immediately. Submitted objects
    /// stay queued until a maintenance tick or explicit completed-object drain.
    /// If the queue cannot grow (host OOM) the object's dependencies are
    /// waited for and it is destroyed synchronously instead of aborting.
    pub fn retire(&self, after: SyncSet, doomed: Doomed) {
        if after.is_empty() {
            self.destroy_now(doomed);
            return;
        }
        if let Err(doomed) = self.retirement.try_enqueue(after, doomed) {
            log::error!("retirement queue is out of host memory; waiting synchronously");
            match self.timelines.wait(&after, u64::MAX) {
                Ok(()) | Err(crate::Error::DeviceLost) => self.destroy_now(doomed),
                Err(error) => {
                    log::error!("cannot confirm GPU completion ({error}); leaking object");
                    std::mem::forget(doomed);
                }
            }
        }
    }
    /// Maintenance tick that logs (instead of returning) a failure.
    pub fn collect_garbage(&self) -> usize {
        match self.try_collect_garbage() {
            Ok(count) => count,
            Err(error) => {
                log::warn!("retirement maintenance failed: {error}");
                0
            }
        }
    }
    /// Perform one bounded maintenance tick: refresh completion and driver
    /// memory budgets, then destroy reached objects within the configured
    /// budget (larger under memory pressure). Reached objects are always
    /// destroyed in FIFO order per queue, so none can starve.
    pub fn try_collect_garbage(&self) -> crate::Result<usize> {
        let completed = self.reclamation_completed()?;
        if let Err(error) = self.refresh_memory_budget() {
            log::warn!("memory budget refresh failed: {error}");
        }
        let budget = self
            .reclamation
            .budget(self.allocator_stats.budget_action());
        let count = self
            .retirement
            .collect_limited(&completed, budget.max_scans, budget.max_destroys, |doomed| {
                self.destroy_now(doomed)
            })
            .destroyed;
        self.trim_unused_allocations();
        Ok(count)
    }
    /// Destroy every queued object whose dependencies have completed at this
    /// snapshot. This explicit drain is intended for teardown and callers that
    /// require all completed storage to have been returned before continuing.
    pub fn try_collect_all_completed(&self) -> crate::Result<usize> {
        let completed = self.reclamation_completed()?;
        let count = self
            .retirement
            .collect_limited(&completed, usize::MAX, usize::MAX, |doomed| {
                self.destroy_now(doomed)
            })
            .destroyed;
        self.trim_unused_allocations();
        Ok(count)
    }
    fn reclamation_completed(&self) -> crate::Result<CompletedValues> {
        match self.poll() {
            Err(crate::Error::DeviceLost) => {
                let completed = CompletedValues::all();
                self.allocator_stats.observe_completed(&completed);
                Ok(completed)
            }
            other => other,
        }
    }
    fn trim_unused_allocations(&self) {
        #[cfg(feature = "suballocation")]
        if self.allocator_stats.budget_action() == crate::device::BudgetAction::EvictCache
            && let Some(suballocator) = &self.suballocator
        {
            suballocator.trim_unused();
        }
    }
    pub fn wait_idle_and_collect(&self) -> crate::Result<usize> {
        self.idle_waits.fetch_add(1, Ordering::Relaxed);
        self.timelines
            .with_all_queues(|| unsafe { self.device.device_wait_idle() }.map_err(Into::into))?;
        self.try_collect_all_completed()
    }
    pub fn wait_submitted(&self) -> crate::Result<()> {
        self.timelines
            .wait(&self.timelines.submitted_set(), u64::MAX)
    }
    pub fn retirement_stats(&self) -> RetirementStats {
        let submitted = self.timelines.submitted_set();
        let completed = self.timelines.completed();
        let mut stats = RetirementStats {
            pending: self.retirement.len(),
            idle_waits: self.idle_waits.load(Ordering::Relaxed),
            ..Default::default()
        };
        for index in 0..self.timelines.queue_count() {
            if let Some(queue) = crate::sync::QueueId::new(index) {
                stats.submitted[index] = submitted.get(queue);
                stats.completed[index] = completed.get(queue);
            }
        }
        stats
    }
    /// Former name of [`Self::retirement_stats`].
    pub fn garbage_stats(&self) -> RetirementStats {
        self.retirement_stats()
    }
    pub(crate) fn drain_retirement_for_drop(&self) {
        if let Err(error) = unsafe { self.device.device_wait_idle() } {
            log::warn!("vkDeviceWaitIdle during device drop failed: {error}");
        }
        self.retirement
            .collect_limited(&CompletedValues::all(), usize::MAX, usize::MAX, |doomed| {
                self.destroy_now(doomed)
            });
    }
    fn destroy_now(&self, doomed: Doomed) {
        unsafe {
            match doomed {
                Doomed::Buffer(handle, memory) => {
                    self.device.destroy_buffer(handle, None);
                    self.free_deferred(memory);
                }
                Doomed::Image(handle, memory) => {
                    self.device.destroy_image(handle, None);
                    self.free_deferred(memory);
                }
                Doomed::ImageView(handle) => self.device.destroy_image_view(handle, None),
                Doomed::Sampler(handle) => self.device.destroy_sampler(handle, None),
                Doomed::Semaphore(handle) => self.device.destroy_semaphore(handle, None),
                Doomed::QueryPool(handle) => self.device.destroy_query_pool(handle, None),
                Doomed::PipelineLayout(handle) => self.device.destroy_pipeline_layout(handle, None),
                Doomed::Pipeline(handle) => self.device.destroy_pipeline(handle, None),
                Doomed::DescriptorSetLayout(handle) => {
                    self.device.destroy_descriptor_set_layout(handle, None)
                }
                Doomed::DescriptorPool(pool) => drop(pool),
                Doomed::DescriptorSet(handle, pool) => pool.free(handle),
                Doomed::CommandBuffer(handle, pool) => pool.free(handle),
            }
        }
    }
    unsafe fn free_deferred(&self, memory: DeferredMemory) {
        let accounting_id = match memory {
            DeferredMemory::Shared(backing) => {
                drop(backing);
                return;
            }
            DeferredMemory::Dedicated {
                handle,
                accounting_id,
            } => {
                unsafe { self.device.free_memory(handle, None) };
                accounting_id
            }
            #[cfg(feature = "suballocation")]
            DeferredMemory::Sub { allocation } => {
                drop(allocation);
                return;
            }
        };
        if let Err(error) = self.allocator_stats.release(accounting_id) {
            log::error!("memory accounting release failed: {error}");
        }
    }
}
