//! Dense per-range storage.
use crate::Error;
use crate::Result;
use crate::accounting::AllocatorStats;
use crate::accounting::BudgetCategory;
use crate::accounting::CpuMemoryReservation;
use std::sync::Arc;
use super::patch::RangeStateConfig;

pub(crate) struct Storage<T> {
    pub(crate) values: Vec<T>,
    _memory: CpuMemoryReservation,
}

impl<T> Storage<T> {
    pub(crate) fn new(capacity: usize, budget: &Arc<AllocatorStats>) -> Result<Self> {
        let bytes = capacity
            .checked_mul(size_of::<T>())
            .ok_or(Error::OutOfHostMemory)?;
        let mut memory = budget.reserve_cpu_bytes(bytes as u64, BudgetCategory::Live)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(capacity)
            .map_err(|_| Error::OutOfHostMemory)?;
        memory.resize((values.capacity() * size_of::<T>()) as u64)?;
        Ok(Self {
            values,
            _memory: memory,
        })
    }
    pub(crate) fn bytes(&self) -> usize {
        self.values.capacity() * size_of::<T>()
    }
    pub(crate) fn grow(
        &mut self,
        capacity: usize,
        other_bytes: usize,
        config: RangeStateConfig,
        budget: &Arc<AllocatorStats>,
    ) -> Result<()> {
        if self.values.capacity() >= capacity {
            return Ok(());
        }
        if capacity > config.max_cells
            || self
                .bytes()
                .checked_add(other_bytes)
                .and_then(|bytes| bytes.checked_add(capacity.checked_mul(size_of::<T>())?))
                .is_none_or(|bytes| bytes > config.max_cpu_bytes)
        {
            return Err(Error::exhausted("native range state capacity exceeded"));
        }
        // The old ticket stays charged until its actual old backing is freed.
        let mut next = Self::new(capacity, budget)?;
        next.values.append(&mut self.values);
        std::mem::swap(self, &mut next);
        Ok(())
    }
}
