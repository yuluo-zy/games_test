//! A fixed-capacity, budgeted byte owner. Mutable access is a slice, so a caller
//! cannot clone or grow the Vec while silently sharing its capacity credit.
use crate::{
    Error, Result,
    device::{AllocatorStats, CpuStorageReservation},
};
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
mod tests {
    use super::*;
    #[test]
    fn refusal_precedes_storage_and_reuse_preserves_exact_credit() {
        let stats = Arc::new(AllocatorStats::default());
        stats.set_host_budget(128).unwrap();
        assert!(matches!(
            CpuBytes::new(&stats, 129),
            Err(Error::OutOfHostMemory)
        ));
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        let mut bytes = CpuBytes::new(&stats, 128).unwrap();
        let pointer = bytes.as_ptr();
        bytes.fill(0xab);
        bytes.clear();
        assert_eq!(stats.cpu_memory_usage().total(), bytes.capacity() as u64);
        bytes.reset_zeroed(128).unwrap();
        assert_eq!(pointer, bytes.as_ptr());
        assert!(bytes.iter().all(|byte| *byte == 0));
        assert!(bytes.reset_zeroed(129).is_err());
        assert_eq!(bytes.len(), 128);
        drop(bytes);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn slice_borrows_share_admitted_backing_without_changing_credit() {
        let stats = Arc::new(AllocatorStats::default());
        let mut bytes = CpuBytes::new(&stats, 16).unwrap();
        let pointer = bytes.as_ptr();
        let capacity = bytes.capacity();
        let usage = stats.cpu_memory_usage().total();
        {
            let view: &mut [u8] = bytes.as_mut_slice();
            view[0] = 0xab;
            view[1..].fill(0xcd);
        }
        let view: &[u8] = bytes.as_slice();
        assert_eq!(view.as_ptr(), pointer);
        assert_eq!(view.len(), 16);
        assert_eq!(view[0], 0xab);
        assert!(view[1..].iter().all(|byte| *byte == 0xcd));
        assert_eq!(bytes.capacity(), capacity);
        assert_eq!(bytes.accounted_bytes(), capacity as u64);
        assert_eq!(stats.cpu_memory_usage().total(), usage);
        drop(bytes);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        let mut empty = CpuBytes::default();
        assert!(empty.as_slice().is_empty());
        assert!(empty.as_mut_slice().is_empty());
    }

    #[test]
    fn empty_storage_needs_no_credit_and_cannot_grow() {
        let mut bytes = CpuBytes::default();
        assert_eq!(bytes.accounted_bytes(), 0);
        assert!(bytes.reset_zeroed(1).is_err());
    }
}
