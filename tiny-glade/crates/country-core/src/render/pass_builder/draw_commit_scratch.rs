//! Device-owned Commit scratch. The owner checks out an exclusive lease and
//! admits actual Vec capacity before resolving graph-local IDs. Returned
//! scratch contains no graph IDs, guards, resources or external owners.
use rhapsody::{
    Error, Result,
    collections::cpu_scratch::{CpuScratchLease, CpuScratchPool, CpuScratchValue},
};

const MAX_LEASE_BYTES: usize = 1 << 20;
pub type DrawCommitScratchPool = CpuScratchPool<DrawCommitScratch>;
pub type DrawCommitScratchLease = CpuScratchLease<DrawCommitScratch>;

#[derive(Default)]
pub struct DrawCommitScratch {
    /// Input order preserves duplicate token last-write semantics.
    pub(crate) resolved: Vec<(u32, usize, u32)>,
    pub(crate) slots: Vec<Option<u32>>,
    pub(crate) skip: Vec<u8>,
    pub(crate) new_bindings: Vec<usize>,
    pub(crate) access_indices: Vec<usize>,
}
impl CpuScratchValue for DrawCommitScratch {
    fn capacity_bytes(&self) -> usize {
        self.resolved
            .capacity()
            .saturating_mul(core::mem::size_of::<(u32, usize, u32)>())
            .saturating_add(
                self.slots
                    .capacity()
                    .saturating_mul(core::mem::size_of::<Option<u32>>()),
            )
            .saturating_add(self.skip.capacity())
            .saturating_add(
                self.new_bindings
                    .capacity()
                    .saturating_mul(core::mem::size_of::<usize>()),
            )
            .saturating_add(
                self.access_indices
                    .capacity()
                    .saturating_mul(core::mem::size_of::<usize>()),
            )
    }
    fn reset(&mut self) {
        self.resolved.clear();
        self.slots.clear();
        self.skip.clear();
        self.new_bindings.clear();
        self.access_indices.clear();
    }
}
fn growth<T>(value: &Vec<T>, needed: usize) -> Result<usize> {
    if needed <= value.capacity() {
        return Ok(0);
    }
    Ok(std::alloc::Layout::array::<T>(needed)
        .map_err(|_| Error::OutOfHostMemory)?
        .size())
}
fn reserve<T>(value: &mut Vec<T>, needed: usize) -> Result<()> {
    value
        .try_reserve_exact(needed.saturating_sub(value.len()))
        .map_err(|_| Error::OutOfHostMemory)
}
fn projected<T>(value: &Vec<T>, needed: usize) -> Result<usize> {
    Ok(std::alloc::Layout::array::<T>(value.capacity().max(needed))
        .map_err(|_| Error::OutOfHostMemory)?
        .size())
}

/// `false` means the first page will adopt the worker's binding domain; it
/// needs no alias/access remap arrays and does not pay to initialize them.
pub fn prepare(
    lease: &mut DrawCommitScratchLease,
    deferred: usize,
    bindings: usize,
    accesses: usize,
    remap: bool,
) -> Result<()> {
    let scratch = lease.value();
    let mut extra = growth(&scratch.resolved, deferred)?
        .checked_add(growth(&scratch.slots, bindings)?)
        .ok_or(Error::OutOfHostMemory)?;
    if remap {
        extra = extra
            .checked_add(growth(&scratch.skip, accesses)?)
            .and_then(|v| {
                growth(&scratch.new_bindings, bindings)
                    .ok()
                    .and_then(|n| v.checked_add(n))
            })
            .and_then(|v| {
                growth(&scratch.access_indices, accesses)
                    .ok()
                    .and_then(|n| v.checked_add(n))
            })
            .ok_or(Error::OutOfHostMemory)?;
    }
    let projected = projected(&scratch.resolved, deferred)?
        .checked_add(projected(&scratch.slots, bindings)?)
        .and_then(|v| {
            projected(&scratch.skip, if remap { accesses } else { 0 })
                .ok()
                .and_then(|n| v.checked_add(n))
        })
        .and_then(|v| {
            projected(&scratch.new_bindings, if remap { bindings } else { 0 })
                .ok()
                .and_then(|n| v.checked_add(n))
        })
        .and_then(|v| {
            projected(&scratch.access_indices, if remap { accesses } else { 0 })
                .ok()
                .and_then(|n| v.checked_add(n))
        })
        .ok_or(Error::OutOfHostMemory)?;
    if projected > MAX_LEASE_BYTES {
        return Err(Error::exhausted(
            "Commit scratch exceeds one-MiB capacity limit",
        ));
    }
    let initialize = |scratch: &mut DrawCommitScratch| {
        reserve(&mut scratch.resolved, deferred)?;
        reserve(&mut scratch.slots, bindings)?;
        scratch.slots.resize(bindings, None);
        if remap {
            reserve(&mut scratch.skip, accesses)?;
            reserve(&mut scratch.new_bindings, bindings)?;
            reserve(&mut scratch.access_indices, accesses)?;
            scratch.skip.resize(accesses, 0);
            scratch.access_indices.resize(accesses, 0);
        }
        if scratch.capacity_bytes() > MAX_LEASE_BYTES {
            *scratch = DrawCommitScratch::default();
            return Err(Error::exhausted(
                "actual Commit scratch capacity exceeds limit",
            ));
        }
        Ok(())
    };
    if extra == 0 {
        lease.with_prepared(initialize)
    } else {
        lease.prepare(extra, initialize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn cancelled_commit_reuses_capacity_and_clears_all_graph_ids() {
        let budget = Arc::new(rhapsody::device::AllocatorStats::default());
        let pool = DrawCommitScratchPool::new(budget.clone(), 2, 8 << 20, 1 << 20).unwrap();
        let mut first = pool.checkout().unwrap();
        prepare(&mut first, 3, 65, 130, true).unwrap();
        let pointer = first.value().slots.as_ptr();
        first.with_prepared(|scratch| {
            scratch.resolved.push((0x8000_0000, 0, 7));
            scratch.slots[0] = Some(7);
            scratch.skip[0] = 1;
            scratch.new_bindings.push(4);
            scratch.access_indices[0] = 17;
        });
        drop(first);
        let second = pool.checkout().unwrap();
        let value = second.value();
        assert_eq!(pointer, value.slots.as_ptr());
        assert!(value.resolved.is_empty() && value.slots.is_empty() && value.skip.is_empty());
        assert!(value.new_bindings.is_empty() && value.access_indices.is_empty());
        drop(second);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn scratch_budget_refusal_preserves_existing_capacity_and_recovers() {
        let budget = Arc::new(rhapsody::device::AllocatorStats::default());
        let pool = DrawCommitScratchPool::new(budget.clone(), 1, 8 << 20, 1 << 20).unwrap();
        let mut lease = pool.checkout().unwrap();
        let usage = budget.cpu_memory_usage().total();
        budget.set_host_budget(usage).unwrap();
        assert!(matches!(
            prepare(&mut lease, 3, 65, 130, true),
            Err(Error::OutOfHostMemory)
        ));
        assert_eq!(lease.value().capacity_bytes(), 0);
        assert_eq!(budget.cpu_memory_usage().total(), usage);
        budget.set_host_budget(0).unwrap();
        prepare(&mut lease, 3, 65, 130, true).unwrap();
        drop(lease);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn first_page_avoids_full_access_remap_storage() {
        let budget = Arc::new(rhapsody::device::AllocatorStats::default());
        let pool = DrawCommitScratchPool::new(budget, 1, 8 << 20, 1 << 20).unwrap();
        let mut lease = pool.checkout().unwrap();
        prepare(&mut lease, 1, 3, 100_000, false).unwrap();
        assert_eq!(lease.value().skip.capacity(), 0);
        assert_eq!(lease.value().access_indices.capacity(), 0);
        assert_eq!(lease.value().new_bindings.capacity(), 0);
    }
}
