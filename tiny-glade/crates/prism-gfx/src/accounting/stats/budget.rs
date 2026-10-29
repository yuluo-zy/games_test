//! Host, device and heap budgets and the resulting budget action.
use crate::Error;
use crate::Result;
use crate::sync::QueueId;
use std::sync::atomic::Ordering;
use super::AllocatorStats;
use super::reservation::CpuMemoryReservation;
use super::reservation::DescriptorReservation;
use super::reservation::add_cpu;
use super::state::HostState;
use super::state::MAX_HEAPS;
use super::state::StatisticsState;
use super::state::encode_action;
use super::state::heap_index;
use super::super::budget::BudgetCategory;

impl AllocatorStats {
    pub(super) fn with_state<R>(&self, run: impl FnOnce(&mut StatisticsState) -> R) -> R {
        let mut state = self.state.lock();
        self.drain_pending(&mut state);
        let result = run(&mut state);
        self.action
            .store(encode_action(state.action()), Ordering::Release);
        result
    }

    pub(super) fn with_host<R>(&self, run: impl FnOnce(&mut HostState) -> R) -> R {
        let mut host = self.host.lock();
        let result = run(&mut host);
        self.host_action
            .store(encode_action(host.budget.tick()), Ordering::Release);
        result
    }

    fn drain_pending(&self, state: &mut StatisticsState) {
        if self.pending_count.load(Ordering::Acquire) == 0 {
            return;
        }
        for (index, pending) in self.pending_uses.iter().enumerate() {
            let Some(queue) = QueueId::new(index) else {
                continue;
            };
            let mut scratch = std::mem::take(&mut state.scratch);
            {
                let mut pending = pending.lock();
                if pending.is_empty() {
                    state.scratch = scratch;
                    continue;
                }
                std::mem::swap(&mut *pending, &mut scratch);
            }
            self.pending_count
                .fetch_sub(scratch.len(), Ordering::AcqRel);
            for &(value, id) in &scratch {
                state.apply_use(queue, value, id);
            }
            scratch.clear();
            state.scratch = scratch;
        }
    }

    #[doc(hidden)]
    pub fn reserve_cpu_bytes(
        self: &std::sync::Arc<Self>,
        bytes: u64,
        category: BudgetCategory,
    ) -> Result<CpuMemoryReservation> {
        if !matches!(category, BudgetCategory::Live | BudgetCategory::Cache) {
            return Err(Error::invalid_usage(
                "CPU allocation must be live or cached",
            ));
        }
        if bytes != 0 {
            self.with_host(|host| {
                host.budget
                    .reserve(category, bytes)
                    .map_err(|_| Error::OutOfHostMemory)?;
                add_cpu(&mut host.usage, category, bytes);
                host.peak = host.peak.max(host.usage.total());
                Ok::<(), Error>(())
            })?;
        }
        Ok(CpuMemoryReservation {
            stats: self.clone(),
            bytes,
            category,
        })
    }

    pub fn cpu_memory_usage(&self) -> crate::accounting::BudgetUsage {
        self.host.lock().usage
    }

    pub fn cpu_peak_bytes(&self) -> u64 {
        self.host.lock().peak
    }

    /// Host metadata limit; `0` means unbounded. Fails without change when the
    /// current host usage already exceeds `bytes`.
    pub fn set_host_budget(&self, bytes: u64) -> Result<()> {
        self.with_host(|host| {
            host.budget
                .set_limit(bytes)
                .map_err(|_| Error::OutOfHostMemory)
        })
    }

    pub fn host_budget_limit(&self) -> u64 {
        self.host.lock().budget.limit()
    }

    #[doc(hidden)]
    pub fn reserve_update_after_bind_descriptors(
        self: &std::sync::Arc<Self>,
        count: u64,
        limit: u64,
    ) -> Result<DescriptorReservation> {
        self.update_after_bind_descriptors
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(count).filter(|next| *next <= limit)
            })
            .map_err(|_| Error::OutOfDeviceMemory)?;
        Ok(DescriptorReservation {
            stats: self.clone(),
            count,
        })
    }

    /// Device memory limit across all heaps; `0` means unbounded.
    pub fn set_device_budget(&self, bytes: u64) -> Result<()> {
        self.with_state(|state| {
            state
                .budget
                .set_limit(bytes)
                .map_err(|_| Error::OutOfDeviceMemory)
        })
    }

    /// Set both the device memory limit and the host metadata limit to
    /// `bytes` (each is enforced independently). Nothing changes on failure.
    pub fn set_total_budget(&self, bytes: u64) -> Result<()> {
        let mut state = self.state.lock();
        self.drain_pending(&mut state);
        let mut host = self.host.lock();
        let mut device_budget = state.budget;
        device_budget
            .set_limit(bytes)
            .map_err(|_| Error::OutOfDeviceMemory)?;
        let mut host_budget = host.budget;
        host_budget
            .set_limit(bytes)
            .map_err(|_| Error::OutOfHostMemory)?;
        state.budget = device_budget;
        host.budget = host_budget;
        self.action
            .store(encode_action(state.action()), Ordering::Release);
        self.host_action
            .store(encode_action(host.budget.tick()), Ordering::Release);
        Ok(())
    }

    /// Device memory usage by phase (host metadata excluded).
    pub fn device_memory_usage(&self) -> crate::accounting::BudgetUsage {
        self.with_state(|state| state.budget.usage())
    }

    /// Combined report: device memory by phase plus host metadata
    /// ([`Self::cpu_memory_usage`]) in its live/cache categories. Admission
    /// still applies the device and host limits independently.
    pub fn budget_usage(&self) -> crate::accounting::BudgetUsage {
        let mut usage = self.device_memory_usage();
        let host = self.cpu_memory_usage();
        usage.live = usage.live.saturating_add(host.live);
        usage.cache = usage.cache.saturating_add(host.cache);
        usage
    }

    /// Bytes still admissible on `heap` under the device and heap limits.
    #[doc(hidden)]
    pub fn heap_available(&self, heap: u32) -> u64 {
        let state = self.state.lock();
        let device = if state.budget.limit() == 0 {
            u64::MAX
        } else {
            state.budget.available()
        };
        let heap = heap as usize;
        let heap = match (heap < MAX_HEAPS).then(|| state.heap_limit(heap)).flatten() {
            Some(limit) => limit.saturating_sub(state.heap_used[heap]),
            None => u64::MAX,
        };
        device.min(heap)
    }

    pub fn suballocator_spare_bytes(&self) -> u64 {
        self.state.lock().suballocator_spare.iter().sum()
    }

    /// Lock-free pressure recommendation combining the device budget, heap
    /// budgets and the host metadata budget.
    pub fn budget_action(&self) -> crate::accounting::BudgetAction {
        use crate::accounting::BudgetAction;
        let decode = |value| match value {
            1 => BudgetAction::EvictCache,
            2 => BudgetAction::Backpressure,
            _ => BudgetAction::None,
        };
        let device = decode(self.action.load(Ordering::Acquire));
        let host = decode(self.host_action.load(Ordering::Acquire));
        if device == BudgetAction::EvictCache || host == BudgetAction::EvictCache {
            BudgetAction::EvictCache
        } else if device == BudgetAction::Backpressure || host == BudgetAction::Backpressure {
            BudgetAction::Backpressure
        } else {
            BudgetAction::None
        }
    }

    /// Explicit per-heap limit. Fails without change when the heap already
    /// holds more than `bytes`.
    pub fn set_heap_budget(&self, heap: u32, bytes: u64) -> Result<()> {
        let index = heap_index(heap)?;
        self.with_state(|state| {
            if state.heap_used[index] > bytes {
                return Err(Error::invalid_usage(
                    "heap budget is below its current usage",
                ));
            }
            state.heap_budgets[index] = Some(bytes);
            state.refresh_heap_pressure();
            Ok(())
        })
    }

    /// Apply a `VK_EXT_memory_budget` snapshot. Each heap's limit becomes the
    /// driver budget minus memory this tracker did not allocate. A limit below
    /// current usage is accepted: new allocations fail and pressure is raised.
    #[doc(hidden)]
    pub fn apply_driver_heap_budgets(&self, heaps: &[(u64, u64)]) {
        self.with_state(|state| {
            for (index, &(budget, usage)) in heaps.iter().enumerate().take(MAX_HEAPS) {
                let foreign = usage.saturating_sub(state.heap_used[index]);
                state.heap_limits[index] = Some(budget.saturating_sub(foreign));
            }
            state.refresh_heap_pressure();
        });
    }

    pub fn heap_usage(&self, heap: u32) -> Result<(u64, Option<u64>)> {
        let index = heap_index(heap)?;
        let state = self.state.lock();
        Ok((state.heap_used[index], state.heap_limit(index)))
    }

    #[doc(hidden)]
    pub fn set_max_native_objects(&self, count: u64) {
        self.max_native_objects.store(count, Ordering::Release);
    }

    pub fn native_object_count(&self) -> u64 {
        self.native_objects.load(Ordering::Acquire)
    }

    /// Reserve one VkDeviceMemory object under `maxMemoryAllocationCount`.
    #[doc(hidden)]
    pub fn acquire_native_object(&self) -> Result<()> {
        let max = self.max_native_objects.load(Ordering::Acquire);
        self.native_objects
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                (max == 0 || count < max).then_some(count + 1)
            })
            .map(|_| ())
            .map_err(|_| Error::exhausted("maxMemoryAllocationCount reached"))
    }

    #[doc(hidden)]
    pub fn release_native_object(&self) {
        let _ = self
            .native_objects
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                count.checked_sub(1)
            });
    }
}
