//! Native object retirement using per-queue completion and bounded maintenance.
use super::device::Device;
use crate::sync::{CompletedValues, SyncSet};
use ash::vk;
use smallvec::SmallVec;
use std::sync::atomic::Ordering;
const DESTRUCTION_BATCH_SIZE: usize = 64;
pub enum DeferredMemory {
    Shared(std::sync::Arc<super::transient_backing::SharedBacking>),
    /// 专用分配：整块 `VkDeviceMemory`。
    Dedicated {
        handle: vk::DeviceMemory,
        accounting_id: u64,
    },
    /// 子分配：归还 gpu-allocator 里的区间。
    #[cfg(feature = "suballocation")]
    Sub {
        allocation: super::suballoc::SubAllocation,
    },
}

/// 等待销毁的 Vulkan 对象。
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct GraveyardStats {
    pub pending: usize,
    pub submit_epoch: u64,
    pub completed_epoch: u64,
    pub idle_waits: u64,
    pub retirement_ages: crate::sync::retire::RetirementAges,
}
impl Device {
    /// Never-submitted objects are destroyed immediately. Submitted objects
    /// stay queued until a maintenance tick or explicit completed-object drain.
    pub fn retire(&self, after: SyncSet, doomed: Doomed) {
        if after.is_empty() {
            self.destroy_now(doomed);
        } else {
            self.retirement.lock().enqueue(after, doomed);
        }
    }
    pub fn collect_garbage(&self) -> usize {
        self.try_collect_garbage().unwrap_or(0)
    }
    /// Perform one bounded maintenance tick. Pressure selects the configured
    /// larger budget, but does not turn this into a full drain.
    pub fn try_collect_garbage(&self) -> crate::Result<usize> {
        let completed = self.reclamation_completed()?;
        let budget = self
            .reclamation
            .budget(self.allocator_stats.budget_action());
        let count =
            self.collect_completed_budget(&completed, budget.max_scans, budget.max_destroys);
        self.trim_unused_allocations();
        Ok(count)
    }
    /// Destroy every queued object whose dependencies have completed at this
    /// snapshot. This explicit drain is intended for teardown and callers that
    /// require all completed storage to have been returned before continuing.
    pub fn try_collect_all_completed(&self) -> crate::Result<usize> {
        let completed = self.reclamation_completed()?;
        let count = self.collect_completed_budget(&completed, usize::MAX, usize::MAX);
        self.trim_unused_allocations();
        Ok(count)
    }
    fn reclamation_completed(&self) -> crate::Result<CompletedValues> {
        let completed = match self.timelines.poll() {
            Err(crate::Error::DeviceLost) => CompletedValues::all(),
            other => other?,
        };
        self.allocator_stats.observe_completed(&completed);
        Ok(completed)
    }
    fn collect_completed_budget(
        &self,
        completed: &CompletedValues,
        mut scans_left: usize,
        mut destroys_left: usize,
    ) -> usize {
        let mut count = 0;
        while scans_left != 0 && destroys_left != 0 {
            // This never spills: the callback limit is the inline capacity.
            // Collection can therefore release retiring memory even when no
            // extra temporary CPU/GPU budget is available.
            let mut ready: SmallVec<[Doomed; DESTRUCTION_BATCH_SIZE]> = SmallVec::new();
            let progress = self.retirement.lock().collect_limited(
                completed,
                scans_left,
                destroys_left.min(DESTRUCTION_BATCH_SIZE),
                |item| ready.push(item),
            );
            scans_left -= progress.scanned;
            destroys_left -= progress.destroyed;
            count += progress.destroyed;
            for doomed in ready {
                self.destroy_now(doomed);
            }
            if progress.destroyed == 0 {
                break;
            }
        }
        count
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
    pub fn garbage_stats(&self) -> GraveyardStats {
        let submitted = self.timelines.submitted_set();
        let completed = self.timelines.poll().unwrap_or_default();
        let retirement = self.retirement.lock();
        GraveyardStats {
            pending: retirement.len(),
            retirement_ages: retirement.ages(),
            submit_epoch: submitted.points().map(|p| p.value).sum(),
            completed_epoch: submitted.points().map(|p| completed.get(p.queue)).sum(),
            idle_waits: self.idle_waits.load(Ordering::Relaxed),
        }
    }
    pub(crate) fn drain_graveyard_for_drop(&self) {
        let _ = unsafe { self.device.device_wait_idle() };
        self.collect_completed_budget(&CompletedValues::all(), usize::MAX, usize::MAX);
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
        self.allocator_stats
            .release(accounting_id)
            .expect("内存登记与释放必须一一对应");
    }
}
