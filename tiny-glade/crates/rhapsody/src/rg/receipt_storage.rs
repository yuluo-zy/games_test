//! Bounded immutable CPU receipts. GPU completion is independent: an external
//! receipt clone keeps its own snapshot capacity and budget alive until Drop.
use super::graph;
use crate::{
    Error, Result,
    device::BudgetCategory,
    collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue},
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
    pool: CpuPool<Arc<ReceiptSlot>>,
    config: ReceiptStorageConfig,
    budget: Arc<AllocatorStats>,
}
pub(crate) struct ReceiptLease {
    lease: CpuPoolLease<Arc<ReceiptSlot>>,
}
impl CpuPoolValue for Arc<ReceiptSlot> {
    type Statistics = ();
    fn capacity_bytes(&self) -> usize { self.bytes }
    fn available(&self) -> bool { Arc::strong_count(self) == 1 && Arc::weak_count(self) == 0 }
    fn reset(&mut self) {
        let slot = Arc::get_mut(self).expect("exclusive receipt slot");
        slot.data.receipts.clear();
        slot.data.snapshot.physical_mapping.clear();
        slot.data.snapshot.pool_hits.clear();
    }
    fn reclassify(&mut self, category: BudgetCategory) {
        Arc::get_mut(self).expect("exclusive receipt slot").reservation.reclassify(category);
    }
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
        let pool = CpuPool::new(CpuPoolLimits {
            items: config.max_receipts, total_bytes: config.max_total_bytes,
            cached_bytes: config.max_cached_bytes, full: || Error::Timeout,
        }, budget.clone())?;
        Ok(Self { pool, config, budget })
    }
    pub(crate) fn statistics(&self) -> ReceiptStorageStats {
        let stats = self.pool.stats();
        ReceiptStorageStats {
            slots: stats.checked_out + stats.cached + stats.pinned,
            borrowed: stats.checked_out + stats.pinned,
            live_bytes: stats.live_bytes, cached_bytes: stats.cached_bytes,
            metadata_bytes: stats.metadata_bytes, capacity_growths: stats.capacity_growths,
            capacity_stalls: stats.capacity_stalls,
        }
    }
    pub(crate) fn trim_idle(&self, target: usize) -> usize { self.pool.trim_idle(target) }
    /// An owned reservation follows the compiled frame through execute/cancel;
    /// a canceled preparation immediately returns its storage to the same pool.
    pub(crate) fn prepare(&self, source: &graph::Snapshot, receipts: usize) -> Result<ReceiptLease> {
        self.pool.trim_idle(self.config.max_cached_bytes);
        let mut lease = ReceiptLease { lease: self.pool.checkout(|| {
            let snapshot = empty_snapshot(source);
            let bytes = capacity(&snapshot, 0);
            let reservation = self.budget.reserve_cpu_bytes(bytes as u64, BudgetCategory::Live)?;
            Ok(Arc::new(ReceiptSlot { data: ReceiptData { receipts: Vec::new(), snapshot }, bytes, reservation }))
        })? };
        let slot = lease.lease.value();
        let required = desired(&slot.data.snapshot, source, slot.data.receipts.capacity().max(receipts));
        if let Err(error) = lease.resize(required, |data| {
            reserve(&mut data.receipts, receipts)?;
            copy_snapshot(&mut data.snapshot, source)
        }) {
            lease.lease.discard_on_return();
            return Err(error);
        }
        Ok(lease)
    }
}
impl ReceiptLease {
    fn resize(&mut self, required: usize, change: impl FnOnce(&mut ReceiptData) -> Result<()>) -> Result<()> {
        let previous = self.lease.value().bytes;
        self.lease.update(required.saturating_sub(previous), |arc, available| {
            let slot = Arc::get_mut(arc).expect("unpublished receipt is exclusive");
            let limit = slot.bytes.saturating_add(available);
            slot.reservation.resize(required as u64)?;
            let result = change(&mut slot.data);
            let actual = capacity(&slot.data.snapshot, slot.data.receipts.capacity());
            let accounted = if actual > limit { Err(Error::Timeout) } else { slot.reservation.resize(actual as u64) };
            if let Err(error) = accounted {
                // Release enlarged vectors before crediting their reservation.
                drop(std::mem::take(&mut slot.data.receipts));
                drop(std::mem::take(&mut slot.data.snapshot.physical_mapping));
                drop(std::mem::take(&mut slot.data.snapshot.pool_hits));
                slot.bytes = capacity(&slot.data.snapshot, 0);
                slot.reservation.resize(slot.bytes as u64).expect("shrinking receipt reservation");
                return Err(error);
            }
            slot.bytes = actual;
            result
        })
    }
    /// The exact bound count is known after ownership preludes, before submit.
    pub(crate) fn reserve_points(&mut self, count: usize) -> Result<()> {
        let slot = self.lease.value();
        if count <= slot.data.receipts.capacity() { return Ok(()); }
        let required = capacity(&slot.data.snapshot, count);
        self.resize(required, |data| reserve(&mut data.receipts, count))
    }
    pub(crate) fn publish(mut self, receipts: &[crate::sync::SyncPoint]) -> Arc<ReceiptSlot> {
        let arc = self.lease.value_mut();
        let slot = Arc::get_mut(arc).expect("unpublished receipt is exclusive");
        assert!(receipts.len() <= slot.data.receipts.capacity(), "receipt capacity admitted before submit");
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
    fn cancellation_returns_storage_and_statistics_do_not_reclassify_weakly_pinned_slots() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = ReceiptStorage::new(ReceiptStorageConfig { max_receipts: 1, ..config() }, budget.clone()).unwrap();
        let source = snapshot(2);
        let prepared = pool.prepare(&source, 3).unwrap();
        assert_eq!(pool.statistics().borrowed, 1);
        drop(prepared);
        assert_eq!(pool.statistics().borrowed, 0);
        let prepared = pool.prepare(&source, 3).unwrap();
        let published = prepared.publish(&[]);
        let weak = Arc::downgrade(&published);
        drop(published);
        assert!(matches!(pool.prepare(&source, 3), Err(Error::Timeout)));
        assert_eq!(pool.trim_idle(0), 0);
        drop(weak);
        let usage = budget.cpu_memory_usage();
        assert_eq!(pool.statistics().borrowed, 0);
        assert!(pool.statistics().cached_bytes > 0);
        assert_eq!(budget.cpu_memory_usage().live, usage.live);
        assert_eq!(budget.cpu_memory_usage().cache, usage.cache);
        assert!(pool.trim_idle(0) > 0);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn retained_clones_bound_reuse_and_own_bytes_after_session_pool_drop() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = ReceiptStorage::new(config(), stats.clone()).unwrap();
        let mut source = snapshot(12);
        let index = pool.prepare(&source, 16).unwrap();
        let point = crate::sync::SyncPoint {
            queue: crate::sync::QueueId::new(0).unwrap(),
            value: 1,
        };
        let first = index.publish(&[point]);
        let cloned = first.clone();
        source.physical_mapping[0].token = Some(22);
        let index = pool.prepare(&source, 16).unwrap();
        let second = index.publish(&[point]);
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
            let reused = index.publish(&[point]);
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
        let pool = ReceiptStorage::new(config(), stats.clone()).unwrap();
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
        drop(index.publish(&[]));
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
