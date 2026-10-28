//! A fixed-capacity, budgeted byte owner. Mutable access is a slice, so a caller
//! cannot clone or grow the Vec while silently sharing its capacity credit.
use crate::{Error, Result};
use crate::accounting::{AllocatorStats, CpuStorageReservation};
use core::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct CpuBytes {
    // Free real backing before returning its reservation.
    bytes: Vec<u8>,
    memory: Option<CpuStorageReservation>,
}
impl CpuBytes {
    /// Empty bytes have no physical allocation to transfer. Nonempty storage
    /// must remain charged to its originating allocator throughout reuse.
    pub fn belongs_to(&self, budget: &Arc<AllocatorStats>) -> bool {
        self.bytes.capacity() == 0
            || self
                .memory
                .as_ref()
                .is_some_and(|memory| memory.belongs_to(budget))
    }
    pub fn new(budget: &Arc<AllocatorStats>, size: usize) -> Result<Self> {
        if size == 0 {
            return Ok(Self::default());
        }
        let size = u64::try_from(size).map_err(|_| Error::OutOfHostMemory)?;
        let mut memory = budget.reserve_cpu_storage(size)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size as usize)
            .map_err(|_| Error::OutOfHostMemory)?;
        memory.resize(u64::try_from(bytes.capacity()).map_err(|_| Error::OutOfHostMemory)?)?;
        bytes.resize(size as usize, 0);
        Ok(Self {
            bytes,
            memory: Some(memory),
        })
    }
    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.bytes.as_mut_slice()
    }
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }
    pub fn accounted_bytes(&self) -> u64 {
        self.memory.as_ref().map_or(0, CpuStorageReservation::bytes)
    }
    pub fn clear(&mut self) {
        self.bytes.clear();
    }
    /// Reuse only the admitted backing. No allocation or credit change occurs.
    pub fn reset_zeroed(&mut self, len: usize) -> Result<()> {
        if len > self.bytes.capacity() {
            return Err(Error::invalid_usage("CPU bytes exceed admitted capacity"));
        }
        self.bytes.resize(len, 0);
        self.bytes.fill(0);
        Ok(())
    }
}
impl Deref for CpuBytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}
impl DerefMut for CpuBytes {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}
impl PartialEq for CpuBytes {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}
impl Eq for CpuBytes {}

#[cfg(test)]
mod tests;
