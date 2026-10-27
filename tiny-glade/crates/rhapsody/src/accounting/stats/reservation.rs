//! CPU and descriptor reservations that charge the allocator statistics.
use crate::Error;
use crate::Result;
use std::sync::atomic::Ordering;
use super::AllocatorStats;
use super::super::budget::BudgetCategory;

/// Known host allocations are charged to the host budget. They never become
/// VkDeviceMemory records or physical-GPU allocation totals. The ticket follows
/// the actual owner; cache removal cannot credit bytes held by an active lease.
pub(crate) struct CpuMemoryReservation {
    pub(super) stats: std::sync::Arc<AllocatorStats>,
    pub(super) bytes: u64,
    pub(super) category: crate::accounting::BudgetCategory,
}

/// Known payload plus the two reference counters of a shared allocation. The
/// reservation stays in the payload, including when its Arc escapes a cache.
pub(crate) fn shared_cpu_bytes<T>() -> Result<u64> {
    let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
        .extend(std::alloc::Layout::new::<T>())
        .map_err(|_| Error::OutOfHostMemory)?;
    u64::try_from(layout.pad_to_align().size()).map_err(|_| Error::OutOfHostMemory)
}

/// Admit renderer-owned Arc payload and separately owned CPU buffers before a
/// native object is created. Keep this ticket in the actual allocation owner.
pub(crate) fn reserve_shared_cpu_metadata<T>(
    stats: &std::sync::Arc<AllocatorStats>,
    extra_bytes: u64,
) -> Result<CpuMemoryReservation> {
    let bytes = shared_cpu_bytes::<T>()?
        .checked_add(extra_bytes)
        .ok_or(Error::OutOfHostMemory)?;
    stats.reserve_cpu_bytes(bytes, crate::accounting::BudgetCategory::Live)
}

pub(crate) fn cpu_vec_bytes<T>(capacity: usize) -> Result<u64> {
    let layout = std::alloc::Layout::array::<T>(capacity).map_err(|_| Error::OutOfHostMemory)?;
    u64::try_from(layout.size()).map_err(|_| Error::OutOfHostMemory)
}

pub(super) fn add_cpu(usage: &mut crate::accounting::BudgetUsage, category: BudgetCategory, bytes: u64) {
    match category {
        BudgetCategory::Live => usage.live += bytes,
        BudgetCategory::Cache => usage.cache += bytes,
        // Construction only admits live or cached host categories.
        BudgetCategory::InFlight | BudgetCategory::Retiring => {}
    }
}

pub(super) fn sub_cpu(usage: &mut crate::accounting::BudgetUsage, category: BudgetCategory, bytes: u64) {
    match category {
        BudgetCategory::Live => usage.live -= bytes,
        BudgetCategory::Cache => usage.cache -= bytes,
        BudgetCategory::InFlight | BudgetCategory::Retiring => {}
    }
}

impl CpuMemoryReservation {
    pub(crate) fn bytes(&self) -> u64 { self.bytes }
    pub(crate) fn belongs_to(&self, budget: &std::sync::Arc<AllocatorStats>) -> bool { std::sync::Arc::ptr_eq(&self.stats, budget) }
    pub(crate) fn split_off(&mut self, bytes: u64) -> Result<Self> {
        if bytes > self.bytes { return Err(Error::invalid_usage("CPU credit split exceeds reservation")); }
        self.bytes -= bytes;
        Ok(Self { stats: self.stats.clone(), bytes, category: self.category })
    }
    pub(crate) fn resize(&mut self, bytes: u64) -> Result<()> {
        if bytes == self.bytes {
            return Ok(());
        }
        let category = self.category;
        let previous = self.bytes;
        self.stats.with_host(|host| {
            if bytes > previous {
                host.budget
                    .reserve(category, bytes - previous)
                    .map_err(|_| Error::OutOfHostMemory)?;
            } else {
                host.budget.release(category, previous - bytes);
            }
            sub_cpu(&mut host.usage, category, previous);
            add_cpu(&mut host.usage, category, bytes);
            host.peak = host.peak.max(host.usage.total());
            Ok::<(), Error>(())
        })?;
        self.bytes = bytes;
        Ok(())
    }
    pub(crate) fn reclassify(&mut self, category: BudgetCategory) {
        if category == self.category
            || !matches!(category, BudgetCategory::Live | BudgetCategory::Cache)
        {
            return;
        }
        let (from, bytes) = (self.category, self.bytes);
        if bytes == 0 {
            self.category = category;
            return;
        }
        self.stats.with_host(|host| {
            host.budget.reclassify(from, category, bytes);
            sub_cpu(&mut host.usage, from, bytes);
            add_cpu(&mut host.usage, category, bytes);
        });
        self.category = category;
    }
}

impl Drop for CpuMemoryReservation {
    fn drop(&mut self) {
        let (category, bytes) = (self.category, self.bytes);
        if bytes == 0 {
            return;
        }
        self.stats.with_host(|host| {
            host.budget.release(category, bytes);
            sub_cpu(&mut host.usage, category, bytes);
        });
    }
}

pub(crate) struct DescriptorReservation {
    pub(super) stats: std::sync::Arc<AllocatorStats>,
    pub(super) count: u64,
}

impl Drop for DescriptorReservation {
    fn drop(&mut self) {
        self.stats
            .update_after_bind_descriptors
            .fetch_sub(self.count, Ordering::AcqRel);
    }
}
