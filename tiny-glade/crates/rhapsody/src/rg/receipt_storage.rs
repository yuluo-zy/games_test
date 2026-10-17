//! Bounded immutable CPU receipts. GPU completion is independent: an external
//! receipt clone keeps its own snapshot capacity and budget alive until Drop.
use super::graph;
use crate::{
    Error, Result,
    device::BudgetCategory,
    raw::allocator::{AllocatorStats, CpuMemoryReservation},
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct ReceiptStorageConfig {
    pub max_receipts: usize,
    pub max_total_bytes: usize,
    pub max_cached_bytes: usize,
}
impl Default for ReceiptStorageConfig {
    fn default() -> Self {
        Self {
            max_receipts: 32,
            max_total_bytes: 4 << 20,
            max_cached_bytes: 1 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ReceiptStorageStats {
    pub slots: usize,
    pub borrowed: usize,
    pub live_bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub capacity_growths: u64,
    pub capacity_stalls: u64,
}
/// Read-only receipt contents. SessionFrameReceipt exposes these through Deref;
/// changing pooled vector capacities through a published receipt is forbidden.
#[derive(Debug)]
pub struct ReceiptData {
    pub receipts: Vec<crate::sync::SyncPoint>,
    pub snapshot: graph::Snapshot,
}
pub(crate) struct ReceiptSlot {
    pub data: ReceiptData,
    bytes: usize,
    // Drop data before crediting its capacity, including when receipts outlive
    // the Session and the pool's baseline Arc owner.
    reservation: CpuMemoryReservation,
}
impl std::fmt::Debug for ReceiptSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
pub(crate) struct ReceiptStorage {
    slots: Vec<Arc<ReceiptSlot>>,
    config: ReceiptStorageConfig,
    budget: Arc<AllocatorStats>,
    growths: u64,
    stalls: u64,
    metadata_bytes: usize,
    _metadata: CpuMemoryReservation,
}
fn bytes<T>(capacity: usize) -> usize {
    capacity.saturating_mul(std::mem::size_of::<T>())
}
fn reserve<T>(values: &mut Vec<T>, count: usize) -> Result<()> {
    if values.capacity() < count {
        values
            .try_reserve_exact(count.saturating_sub(values.len()))
            .map_err(|_| Error::OutOfHostMemory)?;
    }
    Ok(())
}
fn capacity(snapshot: &graph::Snapshot, receipts: usize) -> usize {
    std::mem::size_of::<ReceiptSlot>()
        .saturating_add(2 * std::mem::size_of::<usize>())
        .saturating_add(std::mem::align_of::<ReceiptSlot>())
        .saturating_add(bytes::<crate::sync::SyncPoint>(receipts))
        .saturating_add(bytes::<graph::PhysicalMapping>(
            snapshot.physical_mapping.capacity(),
        ))
        .saturating_add(bytes::<u32>(snapshot.pool_hits.capacity()))
}
fn desired(current: &graph::Snapshot, source: &graph::Snapshot, receipts: usize) -> usize {
    capacity(current, receipts)
        .saturating_add(bytes::<graph::PhysicalMapping>(
            source
                .physical_mapping
                .len()
                .saturating_sub(current.physical_mapping.capacity()),
        ))
        .saturating_add(bytes::<u32>(
            source
                .pool_hits
                .len()
                .saturating_sub(current.pool_hits.capacity()),
        ))
}
fn copy_snapshot(target: &mut graph::Snapshot, source: &graph::Snapshot) -> Result<()> {
    reserve(&mut target.physical_mapping, source.physical_mapping.len())?;
    reserve(&mut target.pool_hits, source.pool_hits.len())?;
    target.shared.clone_from(&source.shared);
    target.physical_mapping.clone_from(&source.physical_mapping);
    target.pool_hits.clone_from(&source.pool_hits);
    Ok(())
}
fn empty_snapshot(source: &graph::Snapshot) -> graph::Snapshot {
    graph::Snapshot {
        shared: source.shared.clone(),
        physical_mapping: Vec::new(),
        pool_hits: Vec::new(),
    }
}
impl ReceiptStorage {
    pub(crate) fn new(config: ReceiptStorageConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        if !(1..=256).contains(&config.max_receipts)
            || config.max_cached_bytes > config.max_total_bytes
        {
            return Err(Error::invalid_usage(
                "invalid bounded receipt storage configuration",
            ));
        }
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(config.max_receipts)
            .map_err(|_| Error::OutOfHostMemory)?;
        let metadata_bytes =
            bytes::<Arc<ReceiptSlot>>(slots.capacity()).saturating_add(std::mem::size_of::<Self>());
        if metadata_bytes >= config.max_total_bytes {
            return Err(Error::invalid_usage(
                "receipt storage limit cannot hold its slots",
            ));
        }
        let metadata = budget.reserve_cpu_bytes(metadata_bytes as u64, BudgetCategory::Live)?;
        Ok(Self {
            slots,
            config,
            budget,
            growths: 0,
            stalls: 0,
            metadata_bytes,
            _metadata: metadata,
        })
    }
    pub(crate) fn statistics(&mut self) -> ReceiptStorageStats {
        let mut stats = ReceiptStorageStats {
            slots: self.slots.len(),
            metadata_bytes: self.metadata_bytes,
            capacity_growths: self.growths,
            capacity_stalls: self.stalls,
            ..Default::default()
        };
        for arc in &mut self.slots {
            if let Some(slot) = Arc::get_mut(arc) {
                slot.reservation.reclassify(BudgetCategory::Cache);
                stats.cached_bytes += slot.bytes;
            } else {
                stats.borrowed += 1;
                stats.live_bytes += arc.bytes;
            }
        }
        stats
    }
    pub(crate) fn trim_idle(&mut self, target: usize) -> usize {
        let mut cached = self.statistics().cached_bytes;
        let mut freed = 0;
        let mut index = 0;
        while cached > target && index < self.slots.len() {
            if Arc::get_mut(&mut self.slots[index]).is_some() {
                let slot = self.slots.swap_remove(index);
                cached -= slot.bytes;
                freed += slot.bytes;
                drop(slot);
            } else {
                index += 1;
            }
        }
        freed
    }
    /// Admit all receipt capacity before any irreversible queue submission.
    pub(crate) fn prepare(&mut self, source: &graph::Snapshot, receipts: usize) -> Result<usize> {
        self.trim_idle(self.config.max_cached_bytes);
        let existing = self
            .slots
            .iter_mut()
            .position(|slot| Arc::get_mut(slot).is_some());
        if existing.is_none() && self.slots.len() == self.config.max_receipts {
            self.stalls += 1;
            return Err(Error::Timeout);
        }
        let total = self.metadata_bytes + self.slots.iter().map(|s| s.bytes).sum::<usize>();
        let mut new_slot = None;
        let index = existing.unwrap_or(self.slots.len());
        let slot = if let Some(index) = existing {
            Arc::get_mut(&mut self.slots[index]).expect("exclusive receipt slot")
        } else {
            new_slot = Some(ReceiptSlot {
                data: ReceiptData {
                    receipts: Vec::new(),
                    snapshot: empty_snapshot(source),
                },
                bytes: 0,
                reservation: self.budget.reserve_cpu_bytes(0, BudgetCategory::Live)?,
            });
            new_slot.as_mut().expect("new receipt slot")
        };
        let previous = slot.bytes;
        let required = desired(
            &slot.data.snapshot,
            source,
            slot.data.receipts.capacity().max(receipts),
        );
        if total.saturating_add(required.saturating_sub(previous)) > self.config.max_total_bytes {
            self.stalls += 1;
            return Err(Error::Timeout);
        }
        slot.reservation.reclassify(BudgetCategory::Live);
        slot.reservation.resize(required as u64)?;
        let result = reserve(&mut slot.data.receipts, receipts)
            .and_then(|()| copy_snapshot(&mut slot.data.snapshot, source));
        slot.data.receipts.clear();
        let actual = capacity(&slot.data.snapshot, slot.data.receipts.capacity());
        slot.bytes = actual;
        let accounted = slot.reservation.resize(actual as u64);
        let valid = result.and(accounted).and_then(|()| {
            if total - previous + actual > self.config.max_total_bytes {
                Err(Error::Timeout)
            } else {
                Ok(())
            }
        });
        if let Err(error) = valid {
            if existing.is_some() {
                self.slots.swap_remove(index);
            }
            return Err(error);
        }
        self.growths += u64::from(actual > previous);
        if existing.is_none() {
            self.slots
                .push(Arc::new(new_slot.expect("new receipt slot")));
        }
        Ok(index)
    }
    pub(crate) fn publish(
        &mut self,
        index: usize,
        receipts: &[crate::sync::SyncPoint],
    ) -> Arc<ReceiptSlot> {
        let arc = &mut self.slots[index];
        let slot = Arc::get_mut(arc).expect("unpublished receipt slot is exclusive");
        assert!(
            receipts.len() <= slot.data.receipts.capacity(),
            "receipt capacity admitted before submit"
        );
        slot.data.receipts.extend_from_slice(receipts);
        arc.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn snapshot(resources: usize) -> graph::Snapshot {
        let shared = Arc::new(graph::SnapshotStatic {
            pass_order: Vec::new(),
            pass_ranges: Vec::new(),
            accesses: Vec::new(),
            lifetimes: Vec::new(),
            image_usage: Vec::new(),
            buffer_usage: Vec::new(),
            lifetime_events: Vec::new(),
            batches: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        });
        graph::Snapshot {
            shared,
            physical_mapping: (0..resources)
                .map(|logical| graph::PhysicalMapping {
                    logical: logical as u32,
                    physical: logical as u16,
                    reused: false,
                    initial_access: 0x26,
                    payload_tag: 0,
                    token: Some(11 + logical as u64),
                })
                .collect(),
            pool_hits: vec![],
        }
    }
    fn config() -> ReceiptStorageConfig {
        ReceiptStorageConfig {
            max_receipts: 2,
            max_total_bytes: 256 << 10,
            max_cached_bytes: 128 << 10,
        }
    }
    #[test]
    fn retained_clones_bound_reuse_and_own_bytes_after_session_pool_drop() {
        let stats = Arc::new(AllocatorStats::default());
        let mut pool = ReceiptStorage::new(config(), stats.clone()).unwrap();
        let mut source = snapshot(12);
        let index = pool.prepare(&source, 16).unwrap();
        let point = crate::sync::SyncPoint {
            queue: crate::sync::QueueId::new(0).unwrap(),
            value: 1,
        };
        let first = pool.publish(index, &[point]);
        let cloned = first.clone();
        source.physical_mapping[0].token = Some(22);
        let index = pool.prepare(&source, 16).unwrap();
        let second = pool.publish(index, &[point]);
        assert_eq!(first.data.snapshot.physical_mapping[0].token, Some(11));
        assert_eq!(second.data.snapshot.physical_mapping[0].token, Some(22));
        assert!(matches!(pool.prepare(&source, 16), Err(Error::Timeout)));
        drop(first);
        assert!(matches!(pool.prepare(&source, 16), Err(Error::Timeout)));
        assert_eq!(pool.trim_idle(0), 0);
        let pointer = cloned.data.snapshot.physical_mapping.as_ptr();
        drop(cloned);
        for _ in 0..1000 {
            let index = pool.prepare(&source, 16).unwrap();
            let reused = pool.publish(index, &[point]);
            assert_eq!(reused.data.snapshot.physical_mapping.as_ptr(), pointer);
            assert_eq!(reused.data.receipts, [point]);
        }
        assert_eq!(pool.statistics().capacity_growths, 2);
        assert!(pool.trim_idle(0) > 0);
        let retained_bytes = second.bytes as u64;
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), retained_bytes);
        drop(second);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn local_receipt_capacity_backpressure_and_host_budget_errors_are_distinct() {
        let stats = Arc::new(AllocatorStats::default());
        let mut pool = ReceiptStorage::new(config(), stats.clone()).unwrap();
        let source = snapshot(1);
        assert!(matches!(
            pool.prepare(&source, 100_000),
            Err(Error::Timeout)
        ));
        assert_eq!(pool.statistics().slots, 0);
        stats
            .set_total_budget(stats.cpu_memory_usage().total() + 1)
            .unwrap();
        assert!(matches!(
            pool.prepare(&source, 1),
            Err(Error::OutOfHostMemory)
        ));
        stats.set_total_budget(0).unwrap();
        let index = pool.prepare(&source, 1).unwrap();
        drop(pool.publish(index, &[]));
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
