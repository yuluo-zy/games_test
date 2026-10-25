//! [`Lifetime`]: deferred destruction of native objects (retirement queue)
//! and the bounded per-tick reclamation policy.
//!
//! The retirement queue's per-queue FIFO locks are leaves: entries are
//! popped first and destroyed with no lock held, so destruction may take
//! memory (allocator) locks freely.
use super::Device;
use crate::raw::graveyard::{DeferredMemory, Doomed, RetirementStats};
use crate::sync::{CompletedValues, SyncSet};
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) struct Lifetime {
    pub(crate) retirement: crate::sync::retire::Retirement<Doomed>,
    pub(crate) reclamation: crate::device::ReclamationConfig,
    pub(crate) idle_waits: AtomicU64,
}

impl Lifetime {
    pub(crate) fn new(reclamation: crate::device::ReclamationConfig) -> Self {
        Self {
            retirement: Default::default(),
            reclamation,
            idle_waits: AtomicU64::new(0),
        }
    }
}

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
        if let Err(doomed) = self.lifetime.retirement.try_enqueue(after, doomed) {
            log::error!("retirement queue is out of host memory; waiting synchronously");
            match self.queues.timelines.wait(&after, u64::MAX) {
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
        if let Err(error) = self.refresh_memory_budget_if_due() {
            log::warn!("memory budget refresh failed: {error}");
        }
        let budget = self
            .lifetime
            .reclamation
            .budget(self.memory.stats.budget_action());
        let count = self
            .lifetime
            .retirement
            .collect_limited(
                &completed,
                budget.max_scans,
                budget.max_destroys,
                |doomed| self.destroy_now(doomed),
            )
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
            .lifetime
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
                self.memory.stats.observe_completed(&completed);
                Ok(completed)
            }
            other => other,
        }
    }
    fn trim_unused_allocations(&self) {
        #[cfg(feature = "suballocation")]
        if self.memory.stats.budget_action() == crate::device::BudgetAction::EvictCache
            && let Some(suballocator) = &self.memory.suballocator
        {
            suballocator.trim_unused();
        }
    }
    pub fn wait_idle_and_collect(&self) -> crate::Result<usize> {
        self.lifetime.idle_waits.fetch_add(1, Ordering::Relaxed);
        let wait = self
            .queues
            .timelines
            .with_all_queues(|| unsafe { self.core.device.device_wait_idle() }.map_err(Into::into));
        let collected = self.try_collect_all_completed();
        wait?;
        collected
    }
    pub fn retirement_stats(&self) -> RetirementStats {
        let submitted = self.queues.timelines.submitted_set();
        let completed = self.queues.timelines.completed();
        let mut stats = RetirementStats {
            pending: self.lifetime.retirement.len(),
            idle_waits: self.lifetime.idle_waits.load(Ordering::Relaxed),
            ..Default::default()
        };
        for index in 0..self.queues.timelines.queue_count() {
            if let Some(queue) = crate::sync::QueueId::new(index) {
                stats.submitted[index] = submitted.get(queue);
                stats.completed[index] = completed.get(queue);
            }
        }
        stats
    }
    pub(crate) fn drain_retirement_for_drop(&self) {
        if let Err(error) = unsafe { self.core.device.device_wait_idle() } {
            log::warn!("vkDeviceWaitIdle during device drop failed: {error}");
        }
        while !self.lifetime.retirement.is_empty() {
            self.lifetime.retirement.collect_limited(
                &CompletedValues::all(),
                usize::MAX,
                usize::MAX,
                |doomed| self.destroy_now(doomed),
            );
        }
    }
    fn destroy_now(&self, doomed: Doomed) {
        unsafe {
            match doomed {
                Doomed::Buffer(handle, memory) => {
                    self.core.device.destroy_buffer(handle, None);
                    self.free_deferred(memory);
                }
                Doomed::Image(handle, memory) => {
                    self.core.device.destroy_image(handle, None);
                    self.free_deferred(memory);
                }
                Doomed::ImageView(handle) => self.core.device.destroy_image_view(handle, None),
                Doomed::Sampler(handle) => self.core.device.destroy_sampler(handle, None),
                Doomed::Semaphore(handle) => self.core.device.destroy_semaphore(handle, None),
                Doomed::QueryPool(handle) => self.core.device.destroy_query_pool(handle, None),
                Doomed::PipelineLayout(handle) => {
                    self.core.device.destroy_pipeline_layout(handle, None)
                }
                Doomed::Pipeline(handle) => self.core.device.destroy_pipeline(handle, None),
                Doomed::DescriptorSetLayout(handle) => {
                    self.core.device.destroy_descriptor_set_layout(handle, None)
                }
                Doomed::DescriptorPool(pool) => drop(pool),
                Doomed::DescriptorSet(handle, pool) => pool.free(handle),
                Doomed::CommandBuffer(handle, pool) => pool.free(handle),
                Doomed::Owners(owners) => drop(owners),
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
                unsafe { self.core.device.free_memory(handle, None) };
                accounting_id
            }
            #[cfg(feature = "suballocation")]
            DeferredMemory::Sub { allocation } => {
                drop(allocation);
                return;
            }
        };
        if let Err(error) = self.memory.stats.release(accounting_id) {
            log::error!("memory accounting release failed: {error}");
        }
    }
}
