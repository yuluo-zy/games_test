//! CPU recording memory. A checkout owns a fixed bumpalo chunk exclusively and
//! returns it only when the last CPU borrow has ended. This storage must never
//! back GPU-visible upload/readback memory: no SyncPoint controls its reset.
use crate::{
    Error, Result,
    device::BudgetCategory,
    raw::allocator::{AllocatorStats, CpuMemoryReservation},
};
use bumpalo::Bump;
use parking_lot::Mutex;
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
struct State {
    idle: Vec<Arena>,
    _idle_memory: CpuMemoryReservation,
    stats: CpuArenaStats,
}
struct Shared {
    config: CpuArenaConfig,
    budget: Arc<AllocatorStats>,
    state: Mutex<State>,
}
#[derive(Clone)]
pub struct CpuArenaPool {
    shared: Arc<Shared>,
}
impl CpuArenaPool {
    pub fn new(config: CpuArenaConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        config.validate()?;
        let mut idle = Vec::new();
        idle.try_reserve_exact(config.max_arenas)
            .map_err(|_| Error::OutOfHostMemory)?;
        let metadata_bytes = idle
            .capacity()
            .checked_mul(std::mem::size_of::<Arena>())
            .ok_or_else(|| Error::invalid_usage("CPU free-list size overflow"))?;
        if metadata_bytes >= config.max_total_bytes {
            return Err(Error::invalid_usage(
                "CPU arena limit cannot hold its fixed free-list",
            ));
        }
        let idle_memory = budget.reserve_cpu_bytes(metadata_bytes as u64, BudgetCategory::Live)?;
        Ok(Self {
            shared: Arc::new(Shared {
                config,
                budget,
                state: Mutex::new(State {
                    idle,
                    _idle_memory: idle_memory,
                    stats: CpuArenaStats {
                        metadata_bytes,
                        peak_total_bytes: metadata_bytes,
                        ..Default::default()
                    },
                }),
            }),
        })
    }
    /// Acquire inside the actual worker task. The !Sync lease cannot be shared
    /// across concurrent workers; moving it transfers its exclusive CPU owner.
    pub fn checkout(&self) -> Result<CpuArenaLease> {
        let mut state = self.shared.state.lock();
        let mut arena = if let Some(arena) = state.idle.pop() {
            state.stats.cached_bytes -= arena.bytes;
            state.stats.cached_arenas -= 1;
            arena
        } else {
            if state.stats.checked_out >= self.shared.config.max_arenas {
                state.stats.capacity_stalls += 1;
                return Err(Error::Timeout);
            }
            let arena = Arena::new(self.shared.config, &self.shared.budget)?;
            if state
                .stats
                .live_bytes
                .checked_add(state.stats.metadata_bytes)
                .and_then(|bytes| bytes.checked_add(arena.bytes))
                .is_none_or(|total| total > self.shared.config.max_total_bytes)
            {
                state.stats.capacity_stalls += 1;
                return Err(Error::Timeout);
            }
            state.stats.backing_allocations += 1;
            arena
        };
        arena.reservation.reclassify(BudgetCategory::Live);
        state.stats.checked_out += 1;
        state.stats.live_bytes += arena.bytes;
        state.stats.peak_checked_out = state.stats.peak_checked_out.max(state.stats.checked_out);
        state.stats.peak_total_bytes = state
            .stats
            .peak_total_bytes
            .max(state.stats.live_bytes + state.stats.cached_bytes + state.stats.metadata_bytes);
        Ok(CpuArenaLease {
            arena: Some(arena),
            shared: self.shared.clone(),
        })
    }
    pub fn stats(&self) -> CpuArenaStats {
        self.shared.state.lock().stats
    }
    pub fn config(&self) -> CpuArenaConfig {
        self.shared.config
    }
    /// Clear only cached CPU blocks. Outstanding leases keep their reservation
    /// and storage, independent of any GPU completion or cache eviction request.
    pub fn trim_idle(&self, target_bytes: usize) -> usize {
        let mut state = self.shared.state.lock();
        let mut freed = 0;
        while state.stats.cached_bytes > target_bytes {
            let Some(arena) = state.idle.pop() else {
                break;
            };
            state.stats.cached_bytes -= arena.bytes;
            state.stats.cached_arenas -= 1;
            freed += arena.bytes;
            drop(arena);
        }
        freed
    }
}
/// The allocator's borrows prevent reset/return while temporary slices are used.
/// Copy-only allocations intentionally exclude destructors and GPU Arc owners.
pub struct CpuArenaLease {
    arena: Option<Arena>,
    shared: Arc<Shared>,
}
impl CpuArenaLease {
    fn arena(&self) -> &Arena {
        self.arena.as_ref().expect("active CPU lease")
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
    pub(crate) fn borrow_slice_with<'a, T>(
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
    pub fn available_bytes(&self) -> usize {
        self.arena().bump.chunk_capacity()
    }
}
impl Drop for CpuArenaLease {
    fn drop(&mut self) {
        let Some(mut arena) = self.arena.take() else {
            return;
        };
        // CPU borrows have ended. GPU work may still be in flight; this block
        // contains no GPU-visible memory and must not wait for that work.
        arena.bump.reset();
        let mut state = self.shared.state.lock();
        state.stats.checked_out -= 1;
        state.stats.live_bytes -= arena.bytes;
        state.stats.bump_requests = state
            .stats
            .bump_requests
            .saturating_add(arena.requests.replace(0));
        state.stats.capacity_stalls = state
            .stats
            .capacity_stalls
            .saturating_add(arena.failures.replace(0));
        state.stats.peak_used_bytes = state.stats.peak_used_bytes.max(arena.peak_used.replace(0));
        if state
            .stats
            .cached_bytes
            .checked_add(arena.bytes)
            .is_some_and(|total| total <= self.shared.config.max_cached_bytes)
            && !matches!(
                self.shared.budget.budget_action(),
                crate::device::BudgetAction::EvictCache | crate::device::BudgetAction::Backpressure
            )
        {
            arena.reservation.reclassify(BudgetCategory::Cache);
            state.stats.cached_bytes += arena.bytes;
            state.stats.cached_arenas += 1;
            state.idle.push(arena);
        }
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

pub(crate) fn scratch_borrows<'a, T>(
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
mod tests {
    use super::*;
    fn config() -> CpuArenaConfig {
        CpuArenaConfig {
            block_bytes: 1024,
            max_arenas: 2,
            max_total_bytes: 16 << 10,
            max_cached_bytes: 8 << 10,
        }
    }
    #[test]
    fn real_capacity_is_budgeted_until_last_cpu_owner_and_never_in_gpu_records() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = CpuArenaPool::new(config(), budget.clone()).unwrap();
        let first = pool.checkout().unwrap();
        let bytes = first.backing_bytes() as u64;
        assert!(bytes >= 1024);
        let metadata = pool.stats().metadata_bytes as u64;
        assert_eq!(budget.cpu_memory_usage().live, bytes + metadata);
        assert_eq!(
            budget.device_memory_usage().total(),
            0,
            "host bytes never charge the device"
        );
        assert_eq!(budget.report().totals.total_allocated_bytes, 0);
        assert!(budget.report().allocations.is_empty());
        assert_eq!(pool.trim_idle(0), 0);
        assert_eq!(budget.cpu_memory_usage().live, bytes + metadata);
        drop(first);
        assert_eq!(budget.cpu_memory_usage().cache, bytes);
        assert_eq!(pool.trim_idle(0) as u64, bytes);
        assert_eq!(budget.cpu_memory_usage().total(), metadata);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
        assert_eq!(budget.budget_usage().total(), 0);
    }
    #[test]
    fn fixed_chunk_backpressure_and_parallel_owner_limits_recover_after_cpu_return() {
        let pool = CpuArenaPool::new(config(), Arc::new(AllocatorStats::default())).unwrap();
        let first = pool.checkout().unwrap();
        let second = pool.checkout().unwrap();
        assert!(matches!(pool.checkout(), Err(Error::Timeout)));
        first.slice_fill(4, 123u64).unwrap();
        assert!(matches!(
            first.slice_fill(first.backing_bytes() * 2, 0u8),
            Err(Error::Exhausted(_))
        ));
        let bytes = first.backing_bytes();
        drop(first);
        let reused = pool.checkout().unwrap();
        assert_eq!(reused.backing_bytes(), bytes);
        assert_eq!(reused.slice_copy(&[7u32, 11]).unwrap(), &[7, 11]);
        drop((second, reused));
        assert_eq!(pool.stats().backing_allocations, 2);
        assert_eq!(pool.stats().checked_out, 0);
    }
    #[test]
    fn host_oom_is_distinct_from_local_backpressure_and_failed_allocation_is_unpublished() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = CpuArenaPool::new(config(), budget.clone()).unwrap();
        let metadata = pool.stats().metadata_bytes as u64;
        budget.set_total_budget(metadata + 100).unwrap();
        assert!(matches!(pool.checkout(), Err(Error::OutOfHostMemory)));
        assert_eq!(pool.stats().live_bytes, 0);
        assert_eq!(budget.cpu_memory_usage().total(), metadata);
        assert_eq!(budget.device_memory_usage().total(), 0);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
}
