//! CPU recording memory. A checkout owns a fixed bumpalo chunk exclusively and
//! returns it only when the last CPU borrow has ended. This storage must never
//! back GPU-visible upload/readback memory: no SyncPoint controls its reset.
use crate::{Error, Result, collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue}};
use crate::accounting::{AllocatorStats, BudgetCategory, CpuMemoryReservation};
use bumpalo::Bump;
use std::{cell::Cell, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub struct CpuArenaConfig {
    /// Minimum payload capacity requested from bumpalo. Its exact allocator
    /// capacity, including chunk metadata/alignment, is measured and budgeted.
    pub block_bytes: usize,
    pub max_arenas: usize,
    pub max_total_bytes: usize,
    /// Retained bump chunks; fixed free-list storage is charged separately.
    pub max_cached_bytes: usize,
}
impl Default for CpuArenaConfig {
    fn default() -> Self {
        Self {
            block_bytes: 64 << 10,
            max_arenas: 32,
            max_total_bytes: 4 << 20,
            max_cached_bytes: 1 << 20,
        }
    }
}
impl CpuArenaConfig {
    fn validate(self) -> Result<()> {
        if self.block_bytes == 0
            || self.block_bytes > isize::MAX as usize
            || !(1..=256).contains(&self.max_arenas)
            || self.max_total_bytes == 0
            || self.max_cached_bytes > self.max_total_bytes
            || self.block_bytes > self.max_total_bytes
        {
            return Err(Error::invalid_usage(
                "invalid bounded CPU arena configuration",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct CpuArenaStats {
    pub checked_out: usize,
    pub cached_arenas: usize,
    pub live_bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub peak_total_bytes: usize,
    pub peak_checked_out: usize,
    /// Actual backing allocations, not the number of bump allocation requests.
    pub backing_allocations: u64,
    /// Requests from returned leases; active workers keep local non-atomic counters.
    pub bump_requests: u64,
    pub capacity_stalls: u64,
    pub peak_used_bytes: usize,
}
struct Arena {
    // Drop the real allocation before crediting its reservation.
    bump: Bump,
    reservation: CpuMemoryReservation,
    bytes: usize,
    initial_capacity: usize,
    requests: Cell<u64>,
    failures: Cell<u64>,
    peak_used: Cell<usize>,
}
impl Arena {
    fn new(config: CpuArenaConfig, budget: &Arc<AllocatorStats>) -> Result<Self> {
        let bump =
            Bump::try_with_capacity(config.block_bytes).map_err(|_| Error::OutOfHostMemory)?;
        // Once this initial chunk fills, report backpressure instead of growing
        // a second backing allocation or silently exceeding the local byte cap.
        bump.set_allocation_limit(Some(bump.allocated_bytes()));
        let bytes = bump.allocated_bytes_including_metadata();
        if bytes > config.max_total_bytes {
            return Err(Error::exhausted(
                "CPU arena block exceeds the configured byte limit",
            ));
        }
        let reservation = budget.reserve_cpu_bytes(bytes as u64, BudgetCategory::Live)?;
        let initial_capacity = bump.chunk_capacity();
        Ok(Self {
            bump,
            reservation,
            bytes,
            initial_capacity,
            requests: Cell::new(0),
            failures: Cell::new(0),
            peak_used: Cell::new(0),
        })
    }
    fn outcome<T>(&self, result: std::result::Result<T, bumpalo::AllocErr>) -> Result<T> {
        self.requests.set(self.requests.get().saturating_add(1));
        self.peak_used.set(
            self.peak_used.get().max(
                self.initial_capacity
                    .saturating_sub(self.bump.chunk_capacity()),
            ),
        );
        result.map_err(|_| {
            self.failures.set(self.failures.get().saturating_add(1));
            Error::exhausted("CPU arena chunk is full")
        })
    }
}
#[derive(Clone, Copy, Default)]
struct ArenaStatistics {
    requests: u64,
    failures: u64,
    peak_used: usize,
}
impl CpuPoolValue for Arena {
    type Statistics = ArenaStatistics;
    fn capacity_bytes(&self) -> usize { self.bytes }
    fn reset(&mut self) {
        // All CPU borrows have ended; this storage never backs GPU memory.
        self.bump.reset();
    }
    fn record_return(&mut self, stats: &mut ArenaStatistics) {
        stats.requests = stats.requests.saturating_add(self.requests.replace(0));
        stats.failures = stats.failures.saturating_add(self.failures.replace(0));
        stats.peak_used = stats.peak_used.max(self.peak_used.replace(0));
    }
    fn reclassify(&mut self, category: BudgetCategory) { self.reservation.reclassify(category); }
}
#[derive(Clone)]
pub struct CpuArenaPool {
    pool: CpuPool<Arena>,
    config: CpuArenaConfig,
    budget: Arc<AllocatorStats>,
}
impl CpuArenaPool {
    pub fn new(config: CpuArenaConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        config.validate()?;
        let pool = CpuPool::new(CpuPoolLimits {
            items: config.max_arenas, total_bytes: config.max_total_bytes,
            cached_bytes: config.max_cached_bytes, full: || Error::Timeout,
        }, budget.clone())?;
        Ok(Self { pool, config, budget })
    }
    /// Acquire inside the worker. Bump's !Sync property prevents sharing a
    /// lease between concurrent workers while allowing exclusive transfer.
    pub fn checkout(&self) -> Result<CpuArenaLease> {
        Ok(CpuArenaLease { lease: self.pool.checkout(|| Arena::new(self.config, &self.budget))? })
    }
    pub fn stats(&self) -> CpuArenaStats {
        let stats = self.pool.stats();
        CpuArenaStats {
            checked_out: stats.checked_out, cached_arenas: stats.cached,
            live_bytes: stats.live_bytes, cached_bytes: stats.cached_bytes,
            metadata_bytes: stats.metadata_bytes, peak_total_bytes: stats.peak_total_bytes,
            peak_checked_out: stats.peak_checked_out, backing_allocations: stats.creations,
            bump_requests: stats.payload.requests,
            capacity_stalls: stats.capacity_stalls.saturating_add(stats.payload.failures),
            peak_used_bytes: stats.payload.peak_used,
        }
    }
    pub fn config(&self) -> CpuArenaConfig { self.config }
    /// Release whole cached blocks until at most `target_bytes` remain. The
    /// returned byte count can exceed the difference because blocks are indivisible.
    pub fn trim_idle(&self, target_bytes: usize) -> usize { self.pool.trim_idle(target_bytes) }
}
/// The allocator's borrows prevent reset/return while temporary slices are used.
/// Copy-only allocations intentionally exclude destructors and GPU Arc owners.
pub struct CpuArenaLease {
    lease: CpuPoolLease<Arena>,
}
impl CpuArenaLease {
    fn arena(&self) -> &Arena {
        self.lease.value()
    }
    fn check_layout<T>(len: usize) -> Result<()> {
        std::alloc::Layout::array::<T>(len)
            .map(|_| ())
            .map_err(|_| Error::invalid_usage("CPU scratch slice layout overflows"))
    }
    #[allow(
        clippy::mut_from_ref,
        reason = "bump allocations are disjoint and the lease cannot reset while borrowed"
    )]
    pub fn slice_copy<T: Copy>(&self, values: &[T]) -> Result<&mut [T]> {
        Self::check_layout::<T>(values.len())?;
        self.arena()
            .outcome(self.arena().bump.try_alloc_slice_copy(values))
    }
    #[allow(
        clippy::mut_from_ref,
        reason = "bump allocations are disjoint and the lease cannot reset while borrowed"
    )]
    pub fn slice_fill<T: Copy>(&self, len: usize, value: T) -> Result<&mut [T]> {
        Self::check_layout::<T>(len)?;
        self.arena()
            .outcome(self.arena().bump.try_alloc_slice_fill_copy(len, value))
    }
    #[allow(
        clippy::mut_from_ref,
        reason = "bump allocations are disjoint and the lease cannot reset while borrowed"
    )]
    pub fn slice_with<T: Copy>(
        &self,
        len: usize,
        initialize: impl FnMut(usize) -> T,
    ) -> Result<&mut [T]> {
        Self::check_layout::<T>(len)?;
        self.arena()
            .outcome(self.arena().bump.try_alloc_slice_fill_with(len, initialize))
    }
    /// Only borrowed references are stored here. Unlike owned values, these
    /// require no destructor and cannot transfer GPU ownership into the arena.
    #[allow(
        clippy::mut_from_ref,
        reason = "bump allocations are disjoint and references cannot outlive this lease"
    )]
    pub(crate) fn borrow_slice_with<'a, T: ?Sized>(
        &'a self,
        len: usize,
        initialize: impl FnMut(usize) -> &'a mut T,
    ) -> Result<&'a mut [&'a mut T]> {
        Self::check_layout::<&mut T>(len)?;
        self.arena()
            .outcome(self.arena().bump.try_alloc_slice_fill_with(len, initialize))
    }
    pub fn backing_bytes(&self) -> usize {
        self.arena().bytes
    }

}
/// Internal fallback for CPU test/third-party backends that do not provide the
/// production pool. Vulkan always supplies an arena; no implicit growth occurs.
pub(crate) enum ScratchSlice<'a, T> {
    Arena(&'a mut [T]),
    Heap(Vec<T>),
}
impl<T> std::ops::Deref for ScratchSlice<'_, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        match self {
            Self::Arena(slice) => slice,
            Self::Heap(values) => values,
        }
    }
}
impl<T> std::ops::DerefMut for ScratchSlice<'_, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        match self {
            Self::Arena(slice) => slice,
            Self::Heap(values) => values,
        }
    }
}
pub(crate) fn scratch_slice<T: Copy>(
    arena: Option<&CpuArenaLease>,
    len: usize,
    mut initialize: impl FnMut(usize) -> T,
) -> Result<ScratchSlice<'_, T>> {
    match arena {
        Some(arena) => arena.slice_with(len, initialize).map(ScratchSlice::Arena),
        None => {
            CpuArenaLease::check_layout::<T>(len)?;
            let mut values = Vec::new();
            values
                .try_reserve_exact(len)
                .map_err(|_| Error::OutOfHostMemory)?;
            values.extend((0..len).map(&mut initialize));
            Ok(ScratchSlice::Heap(values))
        }
    }
}

pub(crate) fn scratch_borrows<'a, T: ?Sized>(
    arena: Option<&'a CpuArenaLease>,
    len: usize,
    mut initialize: impl FnMut(usize) -> &'a mut T,
) -> Result<ScratchSlice<'a, &'a mut T>> {
    match arena {
        Some(arena) => arena
            .borrow_slice_with(len, initialize)
            .map(ScratchSlice::Arena),
        None => {
            CpuArenaLease::check_layout::<&mut T>(len)?;
            let mut values = Vec::new();
            values
                .try_reserve_exact(len)
                .map_err(|_| Error::OutOfHostMemory)?;
            values.extend((0..len).map(&mut initialize));
            Ok(ScratchSlice::Heap(values))
        }
    }
}

#[cfg(test)]
mod tests;
