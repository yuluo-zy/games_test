//! Task-owned CPU constants. Actual byte backing, pool index, task stock and
//! published owner metadata all retain non-Clone host-budget reservations.
use parking_lot::{Mutex, MutexGuard};
use rhapsody::{Error, Result, resource::CpuBytes};
use rhapsody::accounting::{AllocatorStats, CpuStorageReservation};
use std::sync::Arc;

const IDLE_BUDGET: usize = 16 << 20;
const MAX_IDLE_BLOCKS: usize = 1024;
const TASK_BLOCKS: usize = 64;

struct ReclaimAttempt {
    scans: usize,
    destroys: usize,
    retried: bool,
}
impl Default for ReclaimAttempt {
    fn default() -> Self {
        Self {
            scans: 4096,
            destroys: 1024,
            retried: false,
        }
    }
}

#[cfg(any(test, feature = "cpu-probe"))]
fn billing_enabled() -> bool {
    #[cfg(test)]
    {
        true
    }
    #[cfg(not(test))]
    {
        crate::render_cpu_probe::management_enabled()
    }
}

fn shared_bytes<T>() -> Result<usize> {
    let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
        .extend(std::alloc::Layout::new::<T>())
        .map_err(|_| Error::OutOfHostMemory)?;
    Ok(layout.pad_to_align().size())
}

struct Idle {
    blocks: Vec<CpuBytes>,
    bytes: usize,
}

#[cfg(any(test, feature = "cpu-probe"))]
#[derive(Default)]
struct Counters {
    locks: std::sync::atomic::AtomicU64,
    scans: std::sync::atomic::AtomicU64,
    checkouts: std::sync::atomic::AtomicU64,
    returns: std::sync::atomic::AtomicU64,
    physical_allocations: std::sync::atomic::AtomicU64,
    physical_allocated_bytes: std::sync::atomic::AtomicU64,
    planned_tasks: std::sync::atomic::AtomicU64,
    unplanned_tasks: std::sync::atomic::AtomicU64,
    planned_cache_misses: std::sync::atomic::AtomicU64,
    unused_stock_blocks_returned: std::sync::atomic::AtomicU64,
    unused_stock_bytes_returned: std::sync::atomic::AtomicU64,
}
#[cfg(any(test, feature = "cpu-probe"))]
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub(crate) struct ConstantPoolBill {
    pub batch_constants: bool,
    pub lock_acquisitions: u64,
    pub scanned_idle_entries: u64,
    pub batch_checkouts: u64,
    pub batch_returns: u64,
    /// Successful nonempty CpuBytes::new results through this pool, including
    /// retry success; cached reuse and failed admissions do not increment it.
    pub physical_allocations: u64,
    /// Cumulative actual capacities, not requested lengths or live bytes.
    pub physical_allocated_bytes: u64,
    pub planned_tasks: u64,
    pub unplanned_tasks: u64,
    /// Local stock misses, including failed/retried requests, not unique demand.
    pub planned_cache_misses: u64,
    /// All stock remaining at task end, including deliberately recycled discards.
    pub unused_stock_blocks_returned: u64,
    pub unused_stock_bytes_returned: u64,
    pub idle_blocks: usize,
    pub idle_bytes: usize,
    pub metadata_bytes: u64,
}

pub(crate) struct ConstantPool {
    idle: Mutex<Idle>,
    budget: Arc<AllocatorStats>,
    #[cfg(feature = "cpu-probe")]
    batch_constants: bool,
    #[cfg(any(test, feature = "cpu-probe"))]
    counts: Counters,
    // Pool backing is dropped before the capacity credit.
    metadata: CpuStorageReservation,
}

impl ConstantPool {
    #[inline]
    pub(crate) fn batch_enabled(&self) -> bool {
        #[cfg(feature = "cpu-probe")]
        {
            self.batch_constants
        }
        #[cfg(not(feature = "cpu-probe"))]
        {
            true
        }
    }
    fn reclaim_idle(&self, target: u64, attempt: &mut ReclaimAttempt) -> u64 {
        let mut freed = 0u64;
        while freed < target && attempt.scans != 0 && attempt.destroys != 0 {
            let Some(mut idle) = self.idle.try_lock() else {
                break;
            };
            #[cfg(any(test, feature = "cpu-probe"))]
            if billing_enabled() {
                self.counts
                    .locks
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            let Some(bytes) = idle.blocks.pop() else {
                break;
            };
            attempt.scans -= 1;
            attempt.destroys -= 1;
            #[cfg(any(test, feature = "cpu-probe"))]
            if billing_enabled() {
                self.counts
                    .scans
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            let capacity = bytes.capacity();
            idle.bytes -= capacity;
            drop(idle);
            drop(bytes);
            freed = freed.saturating_add(capacity as u64);
        }
        freed
    }
    pub(crate) fn trim_idle(&self, target: u64) -> u64 {
        self.reclaim_idle(target, &mut ReclaimAttempt::default())
    }
    fn reserve_metadata(
        &self,
        bytes: u64,
        attempt: &mut ReclaimAttempt,
    ) -> Result<CpuStorageReservation> {
        match self.budget.reserve_cpu_storage(bytes) {
            Err(Error::OutOfHostMemory) if !attempt.retried => {
                attempt.retried = true;
                self.reclaim_idle(bytes, attempt);
                self.budget.reserve_cpu_storage(bytes)
            }
            result => result,
        }
    }
    fn allocate_bytes(&self, size: usize) -> Result<CpuBytes> {
        let result = match CpuBytes::new(&self.budget, size) {
            Err(Error::OutOfHostMemory) => {
                let mut attempt = ReclaimAttempt {
                    retried: true,
                    ..Default::default()
                };
                self.reclaim_idle(size as u64, &mut attempt);
                CpuBytes::new(&self.budget, size)
            }
            result => result,
        };
        #[cfg(any(test, feature = "cpu-probe"))]
        if billing_enabled() {
            if let Ok(bytes) = &result {
                if bytes.capacity() != 0 {
                    use std::sync::atomic::Ordering::Relaxed;
                    self.counts.physical_allocations.fetch_add(1, Relaxed);
                    self.counts.physical_allocated_bytes.fetch_add(bytes.capacity() as u64, Relaxed);
                }
            }
        }
        result
    }
    pub(crate) fn new(budget: Arc<AllocatorStats>) -> Result<Arc<Self>> {
        #[cfg(feature = "cpu-probe")]
        let batch_constants = match std::env::var("TG_CONSTANT_BATCH") {
            Err(std::env::VarError::NotPresent) => true,
            Ok(value) if value == "1" => true,
            Ok(value) if value == "0" => false,
            _ => return Err(Error::invalid_usage("TG_CONSTANT_BATCH must be 0 or 1")),
        };
        let fixed = shared_bytes::<Self>()?;
        let requested = MAX_IDLE_BLOCKS
            .checked_mul(std::mem::size_of::<CpuBytes>())
            .and_then(|bytes| bytes.checked_add(fixed))
            .ok_or(Error::OutOfHostMemory)?;
        let mut metadata = budget.reserve_cpu_storage(requested as u64)?;
        let mut blocks = Vec::new();
        blocks
            .try_reserve_exact(MAX_IDLE_BLOCKS)
            .map_err(|_| Error::OutOfHostMemory)?;
        metadata.resize(
            blocks
                .capacity()
                .checked_mul(std::mem::size_of::<CpuBytes>())
                .and_then(|bytes| bytes.checked_add(fixed))
                .ok_or(Error::OutOfHostMemory)? as u64,
        )?;
        Ok(Arc::new(Self {
            idle: Mutex::new(Idle { blocks, bytes: 0 }),
            budget,
            #[cfg(feature = "cpu-probe")]
            batch_constants,
            #[cfg(any(test, feature = "cpu-probe"))]
            counts: Counters::default(),
            metadata,
        }))
    }
    fn lock_idle(&self) -> MutexGuard<'_, Idle> {
        #[cfg(any(test, feature = "cpu-probe"))]
        if billing_enabled() {
            self.counts
                .locks
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        self.idle.lock()
    }
    #[cfg(any(test, feature = "cpu-probe"))]
    pub(crate) fn bill(&self) -> ConstantPoolBill {
        use std::sync::atomic::Ordering::Relaxed;
        let idle = self.idle.lock();
        ConstantPoolBill {
            batch_constants: self.batch_enabled(),
            lock_acquisitions: self.counts.locks.load(Relaxed),
            scanned_idle_entries: self.counts.scans.load(Relaxed),
            batch_checkouts: self.counts.checkouts.load(Relaxed),
            batch_returns: self.counts.returns.load(Relaxed),
            physical_allocations: self.counts.physical_allocations.load(Relaxed),
            physical_allocated_bytes: self.counts.physical_allocated_bytes.load(Relaxed),
            planned_tasks: self.counts.planned_tasks.load(Relaxed),
            unplanned_tasks: self.counts.unplanned_tasks.load(Relaxed),
            planned_cache_misses: self.counts.planned_cache_misses.load(Relaxed),
            unused_stock_blocks_returned: self.counts.unused_stock_blocks_returned.load(Relaxed),
            unused_stock_bytes_returned: self.counts.unused_stock_bytes_returned.load(Relaxed),
            idle_blocks: idle.blocks.len(),
            idle_bytes: idle.bytes,
            metadata_bytes: self.metadata.bytes(),
        }
    }
    pub(crate) fn take(&self, size: usize) -> Result<CpuBytes> {
        if size == 0 {
            return Ok(CpuBytes::default());
        }
        let mut idle = self.lock_idle();
        let mut found = None;
        for (index, bytes) in idle.blocks.iter().enumerate() {
            #[cfg(any(test, feature = "cpu-probe"))]
            if billing_enabled() {
                self.counts
                    .scans
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            if bytes.capacity() == size {
                found = Some(index);
                break;
            }
        }
        let reused = found.map(|index| {
            let bytes = idle.blocks.swap_remove(index);
            idle.bytes -= bytes.capacity();
            bytes
        });
        drop(idle);
        let mut bytes = match reused {
            Some(bytes) => bytes,
            None => self.allocate_bytes(size)?,
        };
        bytes.reset_zeroed(size)?;
        Ok(bytes)
    }
    pub(crate) fn recycle(&self, bytes: CpuBytes) {
        if bytes.capacity() != 0 {
            self.recycle_batch([bytes]);
        }
    }
    pub(crate) fn recycle_batch(&self, blocks: impl IntoIterator<Item = CpuBytes>) {
        let mut blocks = blocks.into_iter().peekable();
        if blocks.peek().is_none() {
            return;
        }
        #[cfg(any(test, feature = "cpu-probe"))]
        if billing_enabled() {
            self.counts
                .returns
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let mut idle = self.lock_idle();
        for bytes in blocks {
            if bytes.capacity() == 0
                || bytes.capacity() > IDLE_BUDGET
                || !bytes.belongs_to(&self.budget)
            {
                drop(idle);
                drop(bytes);
                idle = self.lock_idle();
                continue;
            }
            while idle.blocks.len() >= MAX_IDLE_BLOCKS
                || idle
                    .bytes
                    .saturating_add(self.metadata.bytes() as usize)
                    .saturating_add(bytes.capacity())
                    > IDLE_BUDGET
            {
                let Some(old) = idle.blocks.pop() else {
                    break;
                };
                idle.bytes -= old.capacity();
                drop(idle);
                drop(old);
                idle = self.lock_idle();
            }
            if idle.blocks.len() >= MAX_IDLE_BLOCKS
                || idle
                    .bytes
                    .saturating_add(self.metadata.bytes() as usize)
                    .saturating_add(bytes.capacity())
                    > IDLE_BUDGET
            {
                drop(idle);
                drop(bytes);
                idle = self.lock_idle();
                continue;
            }
            assert!(
                idle.blocks.len() < idle.blocks.capacity(),
                "fixed admitted idle index"
            );
            idle.bytes += bytes.capacity();
            idle.blocks.push(bytes);
        }
    }
    fn refill(&self, size: usize, stock: &mut Vec<CpuBytes>) {
        #[cfg(any(test, feature = "cpu-probe"))]
        if billing_enabled() {
            self.counts
                .checkouts
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let mut idle = self.lock_idle();
        let mut index = idle.blocks.len();
        while index != 0 && stock.len() < TASK_BLOCKS {
            index -= 1;
            #[cfg(any(test, feature = "cpu-probe"))]
            if billing_enabled() {
                self.counts
                    .scans
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            if idle.blocks[index].capacity() == size {
                let bytes = idle.blocks.swap_remove(index);
                idle.bytes -= bytes.capacity();
                stock.push(bytes); // Task admitted the entire fixed index.
            }
        }
    }
    pub(crate) fn task(self: &Arc<Self>) -> Result<ConstantTask> {
        let fixed = std::mem::size_of::<ConstantTask>();
        #[cfg(feature = "cpu-probe")]
        let stock_blocks = if self.batch_enabled() { TASK_BLOCKS } else { 0 };
        #[cfg(not(feature = "cpu-probe"))]
        let stock_blocks = TASK_BLOCKS;
        let requested = stock_blocks
            .checked_mul(std::mem::size_of::<CpuBytes>())
            .and_then(|bytes| bytes.checked_add(fixed))
            .ok_or(Error::OutOfHostMemory)?;
        let owner_bytes = OwnerCredit::bytes()?;
        let total = requested
            .checked_add(owner_bytes)
            .ok_or(Error::OutOfHostMemory)?;
        let mut attempt = ReclaimAttempt::default();
        // One admission and one bounded reclaim allowance cover the stock and
        // its first published-owner group; splitting never re-admits credit.
        let mut metadata = self.reserve_metadata(total as u64, &mut attempt)?;
        let owner_memory = metadata.split_off(owner_bytes as u64)?;
        let mut stock = Vec::new();
        if stock.try_reserve_exact(stock_blocks).is_err() {
            if attempt.retried {
                return Err(Error::OutOfHostMemory);
            }
            attempt.retried = true;
            self.reclaim_idle(requested as u64, &mut attempt);
            stock
                .try_reserve_exact(stock_blocks)
                .map_err(|_| Error::OutOfHostMemory)?;
        }
        let actual = stock
            .capacity()
            .checked_mul(std::mem::size_of::<CpuBytes>())
            .and_then(|bytes| bytes.checked_add(fixed))
            .ok_or(Error::OutOfHostMemory)? as u64;
        if let Err(error) = metadata.resize(actual) {
            if !matches!(error, Error::OutOfHostMemory) || attempt.retried {
                return Err(error);
            }
            attempt.retried = true;
            self.reclaim_idle(actual.saturating_sub(metadata.bytes()), &mut attempt);
            metadata.resize(actual)?;
        }
        let owners = Arc::new(OwnerCredit {
            _memory: owner_memory,
        });
        Ok(ConstantTask {
            stock,
            pool: self.clone(),
            owners,
            published: 0,
            last_refill: None,
            planned: false,
            started: false,
            #[cfg(any(test, feature = "cpu-probe"))]
            planned_cache_misses: 0,
            metadata,
        })
    }
}

impl rhapsody::accounting::Evictable for ConstantPool {
    fn evict_idle(&self, _: &rhapsody::sync::CompletedValues, target_bytes: u64) -> u64 {
        self.trim_idle(target_bytes)
    }
}

struct OwnerCredit {
    _memory: CpuStorageReservation,
}
impl OwnerCredit {
    fn bytes() -> Result<usize> {
        shared_bytes::<CollectedConstants>()?
            .checked_mul(TASK_BLOCKS)
            .and_then(|bytes| {
                shared_bytes::<Self>()
                    .ok()
                    .and_then(|owner| bytes.checked_add(owner))
            })
            .ok_or(Error::OutOfHostMemory)
    }
    fn new(pool: &ConstantPool) -> Result<Arc<Self>> {
        let memory =
            pool.reserve_metadata(Self::bytes()? as u64, &mut ReclaimAttempt::default())?;
        Ok(Arc::new(Self { _memory: memory }))
    }
}

pub(crate) struct ConstantTask {
    stock: Vec<CpuBytes>,
    pool: Arc<ConstantPool>,
    owners: Arc<OwnerCredit>,
    published: usize,
    last_refill: Option<usize>,
    planned: bool,
    started: bool,
    #[cfg(any(test, feature = "cpu-probe"))]
    planned_cache_misses: u64,
    metadata: CpuStorageReservation,
}
impl ConstantTask {
    /// Transfer exactly this task's known mixed inventory in one bounded scan.
    /// No new backing or metadata is allocated while holding the pool lock.
    pub(crate) fn prepare_demand(&mut self, sizes: &[u32]) -> Result<()> {
        // Diagnostic legacy mode and externally configured larger chunks keep
        // their existing fallback semantics; never truncate actual work.
        if !self.pool.batch_enabled() || sizes.len() > TASK_BLOCKS { return Ok(()); }
        if self.planned || self.started || !self.stock.is_empty() {
            return Err(Error::invalid_usage("constant demand must precede task use"));
        }
        let mut sorted = [0u32; TASK_BLOCKS];
        sorted[..sizes.len()].copy_from_slice(sizes);
        sorted[..sizes.len()].sort_unstable();
        let mut demand = [(0u32, 0u8); TASK_BLOCKS];
        let mut groups = 0usize;
        let mut remaining = 0usize;
        for &size in &sorted[..sizes.len()] {
            if size == 0 { continue; }
            if groups != 0 && demand[groups - 1].0 == size {
                demand[groups - 1].1 += 1;
            } else {
                demand[groups] = (size, 1);
                groups += 1;
            }
            remaining += 1;
        }
        self.planned = true;
        if remaining == 0 { return Ok(()); }
        #[cfg(any(test, feature = "cpu-probe"))]
        if billing_enabled() {
            self.pool.counts.checkouts.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let mut idle = self.pool.lock_idle();
        let mut index = idle.blocks.len();
        while index != 0 && remaining != 0 {
            index -= 1;
            #[cfg(any(test, feature = "cpu-probe"))]
            if billing_enabled() {
                self.pool.counts.scans.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            let capacity = idle.blocks[index].capacity();
            let Ok(group) = demand[..groups].binary_search_by_key(&(capacity as u64), |&(size, _)| u64::from(size))
                else { continue; };
            if demand[group].1 == 0 { continue; }
            demand[group].1 -= 1;
            remaining -= 1;
            let bytes = idle.blocks.swap_remove(index);
            idle.bytes -= capacity;
            self.stock.push(bytes); // Fixed 64-slot metadata was admitted at task creation.
        }
        Ok(())
    }
    pub(crate) fn take(&mut self, size: usize) -> Result<CpuBytes> {
        self.started = true;
        #[cfg(feature = "cpu-probe")]
        if !self.pool.batch_enabled() {
            return self.pool.take(size);
        }
        if size == 0 {
            return Ok(CpuBytes::default());
        }
        let mut index = self
            .stock
            .iter()
            .rposition(|bytes| bytes.capacity() == size);
        if !self.planned && index.is_none() && self.stock.len() < TASK_BLOCKS && self.last_refill != Some(size) {
            self.pool.refill(size, &mut self.stock);
            self.last_refill = Some(size);
            index = self
                .stock
                .iter()
                .rposition(|bytes| bytes.capacity() == size);
        }
        let mut bytes = match index {
            Some(index) => self.stock.swap_remove(index),
            None => {
                #[cfg(any(test, feature = "cpu-probe"))]
                if self.planned && billing_enabled() { self.planned_cache_misses += 1; }
                self.pool.allocate_bytes(size)?
            }
        };
        bytes.reset_zeroed(size)?;
        Ok(bytes)
    }
    pub(crate) fn recycle(&mut self, bytes: CpuBytes) {
        #[cfg(feature = "cpu-probe")]
        if !self.pool.batch_enabled() {
            self.pool.recycle(bytes);
            return;
        }
        if bytes.capacity() == 0 {
            return;
        }
        if !bytes.belongs_to(&self.pool.budget) {
            return;
        }
        let held = self.stock.iter().map(CpuBytes::capacity).sum::<usize>();
        if self.stock.len() < TASK_BLOCKS
            && held
                .saturating_add(bytes.capacity())
                .saturating_add(self.metadata.bytes() as usize)
                <= IDLE_BUDGET
        {
            self.stock.push(bytes);
        } else {
            self.pool.recycle(bytes);
        }
    }
    /// Failure returns the original physical owner, including its capacity
    /// credit, so the caller can recycle or preserve the exact input.
    pub(crate) fn own(
        &mut self,
        bytes: CpuBytes,
    ) -> std::result::Result<CollectedConstants, (Error, CpuBytes)> {
        if !bytes.belongs_to(&self.pool.budget) {
            return Err((
                Error::invalid_usage("CPU constants belong to another allocator"),
                bytes,
            ));
        }
        if self.published == TASK_BLOCKS {
            match OwnerCredit::new(&self.pool) {
                Ok(owners) => {
                    self.owners = owners;
                    self.published = 0;
                }
                Err(error) => return Err((error, bytes)),
            }
        }
        self.published += 1;
        Ok(CollectedConstants {
            bytes,
            pool: self.pool.clone(),
            _owners: self.owners.clone(),
        })
    }
}
impl Drop for ConstantTask {
    fn drop(&mut self) {
        #[cfg(any(test, feature = "cpu-probe"))]
        if billing_enabled() {
            use std::sync::atomic::Ordering::Relaxed;
            let counter = if self.planned { &self.pool.counts.planned_tasks } else { &self.pool.counts.unplanned_tasks };
            counter.fetch_add(1, Relaxed);
            self.pool.counts.planned_cache_misses.fetch_add(self.planned_cache_misses, Relaxed);
            self.pool.counts.unused_stock_blocks_returned.fetch_add(self.stock.len() as u64, Relaxed);
            self.pool.counts.unused_stock_bytes_returned.fetch_add(self.stock.iter().map(|bytes| bytes.capacity() as u64).sum(), Relaxed);
        }
        self.pool.recycle_batch(self.stock.drain(..));
    }
}

pub(crate) struct CollectedConstants {
    pub(crate) bytes: CpuBytes,
    pool: Arc<ConstantPool>,
    _owners: Arc<OwnerCredit>,
}
impl Drop for CollectedConstants {
    fn drop(&mut self) {
        self.pool.recycle(std::mem::take(&mut self.bytes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cancelled_owned_constants_return_storage_and_clear_unset_uniforms() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(budget.clone()).unwrap();
        let mut task = pool.task().unwrap();
        let mut bytes = task.take(128).unwrap();
        bytes.fill(0xab);
        let pointer = bytes.as_ptr();
        drop(task.own(bytes).ok().unwrap());
        let bytes = pool.take(128).unwrap();
        assert_eq!(bytes.as_ptr(), pointer);
        assert!(bytes.iter().all(|byte| *byte == 0));
        drop(bytes);
        drop(task);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn capacity_and_index_stay_bounded_when_lengths_are_cleared() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(budget.clone()).unwrap();
        for _ in 0..100 {
            let mut bytes = CpuBytes::new(&budget, 1 << 20).unwrap();
            bytes.clear();
            pool.recycle(bytes);
        }
        pool.recycle(CpuBytes::new(&budget, IDLE_BUDGET + 1).unwrap());
        let idle = pool.idle.lock();
        assert!(idle.bytes + pool.metadata.bytes() as usize <= IDLE_BUDGET);
        assert_eq!(
            idle.bytes,
            idle.blocks.iter().map(CpuBytes::capacity).sum::<usize>()
        );
        assert!(idle.blocks.iter().all(|bytes| bytes.is_empty()));
        drop(idle);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn warm_sixty_four_blocks_use_one_checkout_and_one_return_lock() {
        let stats = Arc::new(AllocatorStats::default());
        #[allow(unused_mut)]
        let mut pool = ConstantPool::new(stats.clone()).unwrap();
        #[cfg(feature = "cpu-probe")]
        {
            Arc::get_mut(&mut pool).unwrap().batch_constants = true;
        }
        for _ in 0..64 {
            pool.recycle(CpuBytes::new(&stats, 128).unwrap());
        }
        let before = pool.bill();
        let mut task = pool.task().unwrap();
        let blocks = (0..64).map(|_| task.take(128).unwrap()).collect::<Vec<_>>();
        drop(task);
        pool.recycle_batch(blocks);
        let after = pool.bill();
        assert_eq!(after.lock_acquisitions - before.lock_acquisitions, 2);
        assert_eq!(after.scanned_idle_entries - before.scanned_idle_entries, 64);
        assert_eq!(after.batch_checkouts - before.batch_checkouts, 1);
        assert_eq!(after.batch_returns - before.batch_returns, 1);
        assert_eq!(after.idle_blocks, 64);
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn planned_mixed_warm_shapes_use_zero_new_backing_and_one_bounded_checkout() {
        let mut shapes = vec![vec![128; 64]];
        let mut skewed = vec![96; 64]; skewed[0] = 128; shapes.push(skewed);
        shapes.push((0..64).map(|i| if i % 2 == 0 { 128 } else { 96 }).collect());
        shapes.push((0..64).map(|i| [64, 96, 128][i % 3]).collect());
        shapes.push((0..64).map(|i| 16 + i * 16).collect());
        for sizes in shapes {
            let stats = Arc::new(AllocatorStats::default());
            #[allow(unused_mut)]
            let mut pool = ConstantPool::new(stats.clone()).unwrap();
            #[cfg(feature = "cpu-probe")]
            { Arc::get_mut(&mut pool).unwrap().batch_constants = true; }
            for &size in &sizes { pool.recycle(CpuBytes::new(&stats, size as usize).unwrap()); }
            for round in 0..3 {
                let before = pool.bill();
                let mut task = pool.task().unwrap();
                task.prepare_demand(&sizes).unwrap();
                assert_eq!(task.stock.len(), 64);
                let order: Vec<_> = if round % 2 == 0 { sizes.clone() } else { sizes.iter().rev().copied().collect() };
                let mut owners = Vec::with_capacity(64);
                for size in order {
                    let bytes = task.take(size as usize).unwrap();
                    owners.push(task.own(bytes).ok().unwrap());
                }
                assert!(task.stock.is_empty());
                drop(task);
                let blocks = owners.into_iter().map(|mut owner| std::mem::take(&mut owner.bytes)).collect::<Vec<_>>();
                pool.recycle_batch(blocks); // Same bounded bulk return as successful Commit.
                let after = pool.bill();
                assert_eq!(after.physical_allocations - before.physical_allocations, 0);
                assert_eq!(after.physical_allocated_bytes - before.physical_allocated_bytes, 0);
                assert_eq!(after.lock_acquisitions - before.lock_acquisitions, 2);
                assert_eq!(after.batch_checkouts - before.batch_checkouts, 1);
                assert_eq!(after.scanned_idle_entries - before.scanned_idle_entries, 64);
                assert_eq!(after.planned_tasks - before.planned_tasks, 1);
                assert_eq!(after.planned_cache_misses - before.planned_cache_misses, 0);
                assert_eq!(after.unused_stock_blocks_returned - before.unused_stock_blocks_returned, 0);
            }
            drop(pool);
            assert_eq!(stats.cpu_memory_usage().total(), 0);
        }
    }
    #[test]
    fn planned_shortage_refusal_keeps_stock_and_escaped_owner_then_recovers() {
        let stats = Arc::new(AllocatorStats::default());
        #[allow(unused_mut)]
        let mut pool = ConstantPool::new(stats.clone()).unwrap();
        #[cfg(feature = "cpu-probe")]
        { Arc::get_mut(&mut pool).unwrap().batch_constants = true; }
        for size in [128, 96, 96] { pool.recycle(CpuBytes::new(&stats, size).unwrap()); }
        let mut task = pool.task().unwrap();
        task.prepare_demand(&[128, 96, 96, 96]).unwrap();
        let bytes = task.take(96).unwrap();
        let escaped = Arc::new(task.own(bytes).ok().unwrap());
        let pointer = escaped.bytes.as_ptr();
        let second = task.take(96).unwrap();
        let usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(usage).unwrap();
        assert!(matches!(task.take(96), Err(Error::OutOfHostMemory)));
        assert_eq!(task.stock.len(), 1);
        assert_eq!(task.stock[0].capacity(), 128);
        assert_eq!(stats.cpu_memory_usage().total(), usage);
        let before = pool.bill();
        stats.set_host_budget(0).unwrap();
        let missing = task.take(96).unwrap();
        assert_eq!(pool.bill().physical_allocations - before.physical_allocations, 1);
        task.recycle(second); task.recycle(missing);
        drop(task); drop(pool);
        assert_eq!(escaped.bytes.as_ptr(), pointer);
        assert_eq!(escaped.bytes.accounted_bytes(), 96);
        assert!(stats.cpu_memory_usage().total() >= 96);
        drop(escaped);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn repeated_or_late_plan_keeps_original_owner_and_oversized_plan_keeps_legacy_work() {
        let stats = Arc::new(AllocatorStats::default());
        #[allow(unused_mut)]
        let mut pool = ConstantPool::new(stats.clone()).unwrap();
        #[cfg(feature = "cpu-probe")]
        { Arc::get_mut(&mut pool).unwrap().batch_constants = true; }
        for _ in 0..65 { pool.recycle(CpuBytes::new(&stats, 128).unwrap()); }
        let mut task = pool.task().unwrap();
        task.prepare_demand(&[128]).unwrap();
        let pointer = task.stock[0].as_ptr();
        assert!(matches!(task.prepare_demand(&[96]), Err(Error::InvalidUsage { .. })));
        assert_eq!(task.stock[0].as_ptr(), pointer);
        let bytes = task.take(128).unwrap();
        assert_eq!(bytes.as_ptr(), pointer);
        assert!(task.prepare_demand(&[128]).is_err());
        task.recycle(bytes); drop(task);
        let mut task = pool.task().unwrap();
        task.prepare_demand(&[128; 65]).unwrap();
        assert!(!task.planned);
        let mut owners = Vec::with_capacity(65);
        for _ in 0..65 {
            let bytes = task.take(128).unwrap();
            owners.push(task.own(bytes).ok().unwrap());
            assert!(task.stock.len() <= 64);
        }
        assert_eq!(owners.len(), 65); // No truncation or scheduling change.
        drop(task); drop(owners); drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn unplanned_greedy_diagnostic_still_reports_churn_despite_warm_inventory() {
        #[cfg(feature = "cpu-probe")]
        let modes = [true, false];
        #[cfg(not(feature = "cpu-probe"))]
        let modes = [true];
        for batch in modes {
            let stats = Arc::new(AllocatorStats::default());
            #[allow(unused_mut)]
            let mut pool = ConstantPool::new(stats.clone()).unwrap();
            #[cfg(feature = "cpu-probe")]
            { Arc::get_mut(&mut pool).unwrap().batch_constants = batch; }
            // Both sizes already have enough real backing for the complete
            // 64-draw task. The inventory is warm before the observation.
            for size in [128, 96] {
                for _ in 0..64 { pool.recycle(CpuBytes::new(&stats, size).unwrap()); }
            }
            let before = pool.bill();
            for _ in 0..3 {
                let mut task = pool.task().unwrap();
                let mut owners = Vec::with_capacity(64);
                let bytes = task.take(128).unwrap();
                owners.push(task.own(bytes).ok().unwrap());
                for index in 0..63 {
                    let bytes = task.take(96).unwrap();
                    if index == 0 {
                        // The first 96-byte request consumed its single stock
                        // slot; the global pool still has matching blocks.
                        let idle = pool.idle.lock();
                        assert!(idle.blocks.iter().any(|bytes| bytes.capacity() == 96));
                        assert_eq!(task.stock.len(), if batch { 63 } else { 0 });
                    }
                    owners.push(task.own(bytes).ok().unwrap());
                }
                drop(task); // Worker scope ends before its CPU packet owners.
                drop(owners); // Simulate owner cancellation/finished upload.
            }
            let after = pool.bill();
            let expected = if batch { 3 * 62 } else { 0 };
            assert_eq!(after.physical_allocations - before.physical_allocations, expected);
            assert_eq!(after.physical_allocated_bytes - before.physical_allocated_bytes,
                expected * 96);
            assert!(after.idle_blocks <= MAX_IDLE_BLOCKS);
            drop(pool);
            assert_eq!(stats.cpu_memory_usage().total(), 0);
        }
    }
    #[test]
    fn cancelled_task_and_escaped_owner_keep_credit_until_physical_storage_drops() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(stats.clone()).unwrap();
        let mut task = pool.task().unwrap();
        let mut bytes = task.take(128).unwrap();
        bytes.fill(0xab);
        let owner = Arc::new(task.own(bytes).ok().unwrap());
        let clone = owner.clone();
        drop(task);
        drop(pool);
        drop(owner);
        assert!(stats.cpu_memory_usage().total() >= clone.bytes.capacity() as u64);
        assert_eq!(clone.bytes[0], 0xab);
        drop(clone);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn failed_task_admission_does_not_allocate_or_leak_credit() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(stats.clone()).unwrap();
        let usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(usage).unwrap();
        assert!(matches!(pool.task(), Err(Error::OutOfHostMemory)));
        assert_eq!(stats.cpu_memory_usage().total(), usage);
        stats.set_host_budget(0).unwrap();
        drop(pool.task().unwrap());
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn owner_credit_exhaustion_returns_original_bytes_and_recovers() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(stats.clone()).unwrap();
        let mut task = pool.task().unwrap();
        let mut owners = Vec::new();
        for _ in 0..64 {
            let bytes = task.take(128).unwrap();
            owners.push(task.own(bytes).ok().unwrap());
        }
        let bytes = task.take(128).unwrap();
        let pointer = bytes.as_ptr();
        let usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(usage).unwrap();
        let (error, bytes) = task.own(bytes).err().unwrap();
        assert!(matches!(error, Error::OutOfHostMemory));
        assert_eq!(pointer, bytes.as_ptr());
        assert_eq!(stats.cpu_memory_usage().total(), usage);
        stats.set_host_budget(0).unwrap();
        owners.push(task.own(bytes).ok().unwrap());
        drop(owners);
        drop(task);
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn foreign_bytes_cannot_spend_another_allocator_credit() {
        let first = Arc::new(AllocatorStats::default());
        let second = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(second.clone()).unwrap();
        let mut task = pool.task().unwrap();
        let usage = second.cpu_memory_usage().total();
        second.set_host_budget(usage).unwrap();
        let foreign = CpuBytes::new(&first, 128).unwrap();
        let pointer = foreign.as_ptr();
        let (error, foreign) = task.own(foreign).err().unwrap();
        assert!(matches!(error, Error::InvalidUsage { .. }));
        assert_eq!(pointer, foreign.as_ptr());
        assert_eq!(first.cpu_memory_usage().total(), 128);
        task.recycle(foreign);
        assert_eq!(first.cpu_memory_usage().total(), 0);
        assert!(matches!(task.take(128), Err(Error::OutOfHostMemory)));
        let foreign = CpuBytes::new(&first, 128).unwrap();
        pool.recycle_batch([foreign]);
        assert_eq!(pool.bill().idle_blocks, 0);
        assert_eq!(first.cpu_memory_usage().total(), 0);
        assert_eq!(second.cpu_memory_usage().total(), usage);
        second.set_host_budget(0).unwrap();
        drop(task);
        drop(pool);
        assert_eq!(second.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn idle_live_credit_is_reclaimed_before_one_admission_retry() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(stats.clone()).unwrap();
        for _ in 0..128 {
            pool.recycle(CpuBytes::new(&stats, 256).unwrap());
        }
        let usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(usage).unwrap();
        let mut task = pool
            .task()
            .expect("stock and owner grant reclaim enough idle credit together");
        assert!(pool.bill().idle_blocks < 128);
        let before = pool.bill().idle_blocks;
        let bytes = task
            .take(512)
            .expect("a different size can reclaim idle backing for real byte admission");
        assert!(pool.bill().idle_blocks < before);
        assert_eq!(bytes.accounted_bytes(), bytes.capacity() as u64);
        assert!(stats.cpu_memory_usage().total() <= usage);
        drop(bytes);
        drop(task);
        stats.set_host_budget(0).unwrap();
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn busy_pool_does_not_wait_and_shared_reclaim_quota_is_not_restarted() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = ConstantPool::new(stats.clone()).unwrap();
        for _ in 0..10 {
            pool.recycle(CpuBytes::new(&stats, 128).unwrap());
        }
        let mut attempt = ReclaimAttempt {
            scans: 3,
            destroys: 2,
            retried: false,
        };
        assert_eq!(pool.reclaim_idle(u64::MAX, &mut attempt), 256);
        assert_eq!(pool.reclaim_idle(u64::MAX, &mut attempt), 0);
        assert_eq!(pool.bill().idle_blocks, 8);
        stats
            .set_host_budget(stats.cpu_memory_usage().total())
            .unwrap();
        let guard = pool.idle.lock();
        assert!(matches!(
            pool.allocate_bytes(256),
            Err(Error::OutOfHostMemory)
        ));
        drop(guard);
        stats.set_host_budget(0).unwrap();
        assert_eq!(pool.trim_idle(u64::MAX), 1024);
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[cfg(feature = "cpu-probe")]
    #[test]
    fn diagnostic_legacy_mode_keeps_real_budget_without_task_stock() {
        let stats = Arc::new(AllocatorStats::default());
        let mut pool = ConstantPool::new(stats.clone()).unwrap();
        Arc::get_mut(&mut pool).unwrap().batch_constants = false;
        let mut task = pool.task().unwrap();
        assert_eq!(task.stock.capacity(), 0);
        let mut bytes = task.take(128).unwrap();
        bytes.fill(0xab);
        let pointer = bytes.as_ptr();
        task.recycle(bytes);
        assert_eq!(task.stock.capacity(), 0);
        let bytes = task.take(128).unwrap();
        assert_eq!(pointer, bytes.as_ptr());
        assert!(bytes.iter().all(|v| *v == 0));
        assert_eq!(bytes.accounted_bytes(), bytes.capacity() as u64);
        drop(task.own(bytes).ok().unwrap());
        assert!(!pool.bill().batch_constants);
        drop(task);
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
