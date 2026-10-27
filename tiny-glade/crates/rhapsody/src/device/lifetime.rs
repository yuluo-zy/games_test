//! [`Lifetime`]: deferred destruction of native objects (retirement queue)
//! and the bounded per-tick reclamation policy.
//!
//! The retirement queue's per-queue FIFO locks are leaves: entries are
//! popped first and destroyed with no lock held, so destruction may take
//! memory (allocator) locks freely.
use super::Device;
use crate::device::graveyard::{DeferredMemory, Doomed, RetirementStats};
use crate::sync::{CompletedValues, SyncSet};
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) struct Lifetime {
    pub(crate) retirement: crate::retire::Retirement<Doomed>,
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

pub(crate) type RetireTicket = crate::retire::RetirementTicket<Doomed>;

/// Already-admitted CPU storage transferred with a newly bound raw resource.
/// The object and synchronization cell then keep their credits independently.
pub(crate) struct ResourceStorage {
    pub retirement: RetireTicket,
    pub object: crate::accounting::CpuMemoryReservation,
    pub synchronization: crate::tracking::tracked::SyncCellStorage,
}

impl Device {
    pub(crate) fn reserve_retirement(&self) -> crate::Result<RetireTicket> {
        RetireTicket::reserve(&self.memory.stats)
    }
    pub(crate) fn retire_reserved(&self, after: SyncSet, doomed: Doomed, ticket: RetireTicket) {
        if after.is_empty() {
            match self.destroy_now(doomed) {
                Ok(()) => drop(ticket),
                Err(doomed) => self
                    .lifetime
                    .retirement
                    .enqueue_reserved(after, doomed, ticket),
            }
        } else {
            self.lifetime
                .retirement
                .enqueue_reserved(after, doomed, ticket);
        }
    }
    /// Never-submitted objects are destroyed immediately. Submitted objects
    /// stay queued until a maintenance tick or explicit completed-object drain.
    /// If the queue cannot grow (host OOM) the object's dependencies are
    /// waited for and it is destroyed synchronously instead of aborting.
    pub fn retire(&self, after: SyncSet, doomed: Doomed) {
        let doomed = if after.is_empty() {
            match self.destroy_now(doomed) {
                Ok(()) => return,
                Err(doomed) => doomed,
            }
        } else {
            doomed
        };
        if let Err(doomed) = self.lifetime.retirement.try_enqueue(after, doomed) {
            // Legacy raw-handle callers have no pre-reserved storage. Internal
            // resource Drop never calls this compatibility admission path.
            log::error!("unreserved raw retirement is out of host memory; waiting synchronously");
            match self.queues.timelines.wait(&after, u64::MAX) {
                Ok(()) | Err(crate::Error::DeviceLost) => {
                    if let Err(doomed) = self.destroy_now(doomed) {
                        log::error!(
                            "native host owner remains busy; leaking unreserved raw object"
                        );
                        std::mem::forget(doomed);
                    }
                }
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
        let action = self.memory.stats.budget_action();
        if action != crate::accounting::BudgetAction::None {
            // This O(1) empty-queue check releases only idle CPU backing.
            // It neither polls again nor spends/refreshes the native retirement
            // scan/destroy quota. The return value below still counts native
            // retirement entries, even when this releases nonzero CPU credit.
            self.extensions.trim_under_pressure();
        }
        let budget = self.lifetime.reclamation.budget(action);
        let count = self
            .lifetime
            .retirement
            .collect_limited_retry(
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
        let count = self.collect_completed_snapshot(&completed);
        self.trim_unused_allocations();
        Ok(count)
    }
    pub(crate) fn collect_completed_snapshot(&self, completed: &CompletedValues) -> usize {
        
        self
            .lifetime
            .retirement
            .collect_limited_retry(completed, usize::MAX, usize::MAX, |doomed| {
                self.destroy_now(doomed)
            })
            .destroyed
    }
    pub(crate) fn pressure_reclamation_budget(&self) -> crate::device::ReclamationBudget {
        self.lifetime.reclamation.pressure
    }
    pub(crate) fn collect_pressure_snapshot(
        &self,
        completed: &CompletedValues,
        remaining: &mut crate::device::ReclamationBudget,
    ) -> usize {
        let progress = self.lifetime.retirement.collect_limited_retry(
            completed,
            remaining.max_scans,
            remaining.max_destroys,
            |doomed| self.destroy_now(doomed),
        );
        remaining.max_scans -= progress.scanned;
        remaining.max_destroys -= progress.destroyed;
        progress.destroyed
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
        if self.memory.stats.budget_action() == crate::accounting::BudgetAction::EvictCache
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
            self.lifetime.retirement.collect_limited_retry(
                &CompletedValues::all(),
                usize::MAX,
                usize::MAX,
                |doomed| self.destroy_now(doomed),
            );
        }
    }
    fn destroy_now(&self, doomed: Doomed) -> Result<(), Doomed> {
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
                Doomed::CommandBuffer(handle, pool, mut waiting) => {
                    if !pool.try_free(handle, &mut waiting) {
                        return Err(Doomed::CommandBuffer(handle, pool, waiting));
                    }
                }
                Doomed::Owners(owners) => drop(owners),
            }
        }
        Ok(())
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
