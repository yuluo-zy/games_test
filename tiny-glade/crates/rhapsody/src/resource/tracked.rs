//! 资源“最后一次 GPU 使用”的登记与查询（[`SyncCell`]）。
//!
//! 每次提交成功得到 [`SyncPoint`]，执行器把它登记到本次提交引用的每个资源的
//! `SyncCell` 上；资源最后一个句柄 drop 时，携带 [`SyncCell::snapshot`] 进入
//! [`Retirement`](crate::sync::retire::Retirement)，待对应队列完成后再销毁。
//!
//! 与“先 `Mutex<SyncSet>`、基准显示争用再换原子数组”的保守路线不同，这里直接采用
//! 逐队列 `AtomicU64` 的 `fetch_max`：登记是每帧热路径（每次提交 × 每个引用资源），而
//! 读只发生在帧回收边界。每个队列一条独立原子，登记只写一条 cache line，不跨队列串行，
//! 也不分配堆内存。

use crate::sync::{CompletedValues, MAX_QUEUES, QueueId, SyncPoint, SyncSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// The precise Vulkan range whose release/acquire barriers must match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OwnershipRange {
    Buffer {
        offset: u64,
        size: u64,
    },
    Image {
        aspects: u32,
        base_mip: u32,
        mip_count: u32,
        base_layer: u32,
        layer_count: u32,
    },
}
#[derive(Debug)]
pub struct SyncCell {
    accounting: Option<AllocationAccounting>,
    /// Reservations span the queue call and last-use publication. Host access
    /// takes the same gate before reading completion, closing the snapshot race.
    submission_gate: parking_lot::Mutex<()>,
    ownership_range: Option<OwnershipRange>,
    concurrent_families: Option<super::QueueFamilies>,
    generation: crate::device::DeviceGeneration,
    /// `values[queue.index()]` 记录该队列上最后一次登记的时间线值（0 = 从未使用）。
    values: [AtomicU64; MAX_QUEUES],
    native_state: parking_lot::Mutex<Option<super::range_state::RangeStateTable>>,
    /// The bound frame plan alone synchronizes this graph-private resource: no
    /// native state is kept or patched while set (see `enter_plan_private`).
    plan_private: AtomicBool,
    _cpu_memory: Option<SyncCellStorage>,
}
pub(crate) struct SyncCellStorage(crate::raw::allocator::CpuMemoryReservation);
/// Already-admitted CPU storage transferred with a newly bound raw resource.
/// The object and synchronization cell then keep their credits independently.
pub(crate) struct ResourceStorage {
    pub retirement: crate::device::RetireTicket,
    pub object: crate::raw::allocator::CpuMemoryReservation,
    pub synchronization: SyncCellStorage,
}
impl std::fmt::Debug for SyncCellStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _reservation = &self.0;
        f.debug_struct("SyncCellStorage").finish_non_exhaustive()
    }
}
struct AllocationAccounting {
    stats: std::sync::Arc<crate::raw::allocator::AllocatorStats>,
    id: u64,
}
impl std::fmt::Debug for AllocationAccounting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AllocationAccounting")
            .field(&self.id)
            .finish()
    }
}

impl SyncCell {
    pub fn new() -> Self {
        Self {
            accounting: None,
            submission_gate: parking_lot::Mutex::new(()),
            ownership_range: None,
            concurrent_families: None,
            generation: crate::device::DeviceGeneration::INVALID,
            values: std::array::from_fn(|_| AtomicU64::new(0)),
            native_state: parking_lot::Mutex::new(None),
            plan_private: AtomicBool::new(false),
            _cpu_memory: None,
        }
    }

    pub fn for_device(generation: crate::device::DeviceGeneration) -> Self {
        Self {
            generation,
            ..Self::new()
        }
    }
    /// Standalone cells may outlive their original object through recorded
    /// commands. Keep their own Arc capacity charged until the final owner dies.
    pub(crate) fn for_device_budgeted(
        generation: crate::device::DeviceGeneration,
        budget: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<std::sync::Arc<Self>> {
        Ok(Self::for_device(generation).into_reserved_arc(Self::reserve_arc_storage(budget)?))
    }
    pub(crate) fn reserve_arc_storage(
        budget: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<SyncCellStorage> {
        Ok(SyncCellStorage(budget.reserve_cpu_bytes(
            crate::raw::allocator::shared_cpu_bytes::<Self>()?,
            crate::device::BudgetCategory::Live,
        )?))
    }
    pub(crate) fn into_reserved_arc(mut self, storage: SyncCellStorage) -> std::sync::Arc<Self> {
        debug_assert!(self._cpu_memory.is_none());
        self._cpu_memory = Some(storage);
        std::sync::Arc::new(self)
    }
    pub(crate) fn for_allocation(
        generation: crate::device::DeviceGeneration,
        stats: std::sync::Arc<crate::raw::allocator::AllocatorStats>,
        id: u64,
    ) -> Self {
        Self {
            accounting: Some(AllocationAccounting { stats, id }),
            ..Self::for_device(generation)
        }
    }
    /// Bind the physical Vulkan object's domain before sharing this cell.
    pub(crate) fn with_ownership_range(mut self, range: OwnershipRange) -> Self {
        self.ownership_range = Some(range);
        self
    }
    pub fn ownership_range(&self) -> Option<OwnershipRange> {
        self.ownership_range
    }
    pub(crate) fn with_concurrent_families(mut self, families: super::QueueFamilies) -> Self {
        self.concurrent_families = Some(families);
        self
    }
    pub(crate) fn validate_sharing_family(&self, family: u32) -> crate::Result<()> {
        if self
            .concurrent_families
            .is_some_and(|families| !families.as_slice().contains(&family))
        {
            return Err(crate::Error::invalid_usage(
                "queue family is absent from concurrent resource sharing",
            ));
        }
        Ok(())
    }
    /// Preliminary transfer-service admission. Submission rechecks the same
    /// physical table under the resource gate before issuing native work.
    pub(crate) fn validate_queue_range(
        &self,
        family: u32,
        range: OwnershipRange,
    ) -> crate::Result<()> {
        self.validate_family_range(family, range, false)
    }
    /// As [`Self::validate_queue_range`], also accepting ranges released to
    /// `family` whose acquire the caller records before its own access.
    pub(crate) fn validate_incoming_queue_range(
        &self,
        family: u32,
        range: OwnershipRange,
    ) -> crate::Result<()> {
        self.validate_family_range(family, range, true)
    }
    fn validate_family_range(
        &self,
        family: u32,
        range: OwnershipRange,
        incoming: bool,
    ) -> crate::Result<()> {
        self.validate_sharing_family(family)?;
        super::range_state::validate_range(range)?;
        if !self
            .ownership_range
            .is_some_and(|domain| super::range_state::contains(domain, range))
        {
            return Err(crate::Error::invalid_usage(
                "transfer range exceeds physical resource",
            ));
        }
        let state = self.native_state.lock();
        if self.concurrent_families.is_none()
            && state.as_ref().is_some_and(|table| {
                table
                    .cells()
                    .iter()
                    .filter(|cell| super::range_state::intersection(cell.range, range).is_some())
                    .any(|cell| match cell.state.owner {
                        // The caller records the acquire before its access.
                        super::range_state::RangeOwner::Released { transfer, .. }
                            if incoming && transfer.destination_family == family =>
                        {
                            false
                        }
                        owner => owner
                            .admits(super::ResourceSharing::Exclusive, family)
                            .is_err(),
                    })
            })
        {
            return Err(crate::Error::invalid_usage(
                "resource requires a matching queue-family transfer",
            ));
        }
        Ok(())
    }
    pub fn validate_device(
        &self,
        generation: crate::device::DeviceGeneration,
    ) -> crate::Result<()> {
        if generation.is_valid() && self.generation == generation {
            Ok(())
        } else {
            Err(crate::Error::invalid_usage(
                "提交资源来自其他设备或设备代际无效",
            ))
        }
    }

    /// A graph `Create` resource that is neither exported nor shared across
    /// queues is synchronized entirely by its frame plan, which starts from
    /// discarded contents. Its native table is dropped instead of being patched
    /// for every access; a later tracked use rebuilds it conservatively from
    /// the completion points (`ensure_range_state_locked`).
    pub(crate) fn enter_plan_private(&self) {
        self.plan_private.store(true, Ordering::Relaxed);
        if self.native_state.lock().is_some() {
            let _gate = self.lock_submission();
            *self.native_state.lock() = None;
        }
    }
    pub(crate) fn leave_plan_private(&self) {
        self.plan_private.store(false, Ordering::Relaxed);
    }
    pub(crate) fn is_plan_private(&self) -> bool {
        self.plan_private.load(Ordering::Relaxed)
    }

    pub(crate) fn lock_submission(&self) -> parking_lot::MutexGuard<'_, ()> {
        self.submission_gate.lock()
    }
    /// Borrow the same physical resource state used by submission. An import's
    /// initial layout only initializes a previously untracked object.
    pub(crate) fn with_range_state<R>(
        &self,
        initial: super::range_state::NativeState,
        stats: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
        run: impl FnOnce(&super::range_state::RangeStateTable) -> crate::Result<R>,
    ) -> crate::Result<R> {
        let _gate = self.lock_submission();
        let table = self.ensure_range_state_locked(initial, stats)?;
        run(table.as_ref().expect("initialized native state"))
    }
    /// Observe only installed state. Diagnostics must never initialize a newly
    /// imported image from a guessed layout or alter its first-use contract.
    pub(crate) fn with_existing_range_state<R>(
        &self,
        run: impl FnOnce(&super::range_state::RangeStateTable) -> crate::Result<R>,
    ) -> crate::Result<Option<R>> {
        let _gate = self.lock_submission();
        let table = self.native_state.lock();
        table.as_ref().map(run).transpose()
    }
    fn ensure_range_state_locked(
        &self,
        mut initial: super::range_state::NativeState,
        stats: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<parking_lot::MutexGuard<'_, Option<super::range_state::RangeStateTable>>>
    {
        let mut table = self.native_state.lock();
        if table.is_none() {
            let domain = self.ownership_range.ok_or_else(|| {
                crate::Error::invalid_usage("resource has no native synchronization domain")
            })?;
            let summary = self.snapshot();
            if !summary.is_empty() {
                // A raw reference used before native-state installation has no
                // trustworthy layout or access provenance. Never seed it from
                // a later import's stale scalar state.
                for point in summary.points() {
                    initial.writer.points.insert(point);
                }
                for point in summary.points() {
                    initial.writer.add(
                        point.queue,
                        super::range_state::AccessScope {
                            stages: ash::vk::PipelineStageFlags::ALL_COMMANDS,
                            access: ash::vk::AccessFlags::MEMORY_READ
                                | ash::vk::AccessFlags::MEMORY_WRITE,
                            write: true,
                        },
                    );
                }
                initial.layout_known = matches!(domain, OwnershipRange::Buffer { .. });
            }
            let sharing = self.concurrent_families.map_or(
                super::ResourceSharing::Exclusive,
                super::ResourceSharing::Concurrent,
            );
            *table = Some(super::range_state::RangeStateTable::new(
                domain,
                sharing,
                initial,
                Default::default(),
                stats.clone(),
            )?);
        }
        Ok(table)
    }
    pub(crate) fn prepare_range_state(
        &self,
        patches: &[super::range_state::StatePatch],
        queue: QueueId,
        family: u32,
        stats: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<()> {
        let mut table =
            self.ensure_range_state_locked(super::range_state::NativeState::default(), stats)?;
        table
            .as_mut()
            .expect("initialized native state")
            .prepare(patches, queue, family)
    }
    pub(crate) fn publish_range_state(&self, point: SyncPoint) {
        self.native_state
            .lock()
            .as_mut()
            .expect("prepared native state")
            .publish(point);
    }
    pub(crate) fn prepared_range_staged_dependencies(&self) -> crate::sync::StagedDependencies {
        self.native_state
            .lock()
            .as_ref()
            .expect("prepared native state")
            .prepared_staged_dependencies()
    }
    /// Discard the contents of the installed native state, once
    /// `completed` proves every recorded GPU use finished. Returns `false`
    /// (state untouched) while any use may still be executing.
    pub(crate) fn discard_idle_range_state(&self, completed: &CompletedValues) -> bool {
        let _gate = self.lock_submission();
        if !self.is_reached(completed) {
            return false;
        }
        if let Some(table) = self.native_state.lock().as_mut() {
            table.discard_idle_contents();
        }
        true
    }
    pub(crate) fn cancel_range_state(&self) {
        if let Some(table) = self.native_state.lock().as_mut() {
            table.cancel_prepared();
        }
    }
    pub(crate) fn preview_range_state<R>(
        &self,
        initial: super::range_state::NativeState,
        stats: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
        patches: &[super::range_state::StatePatch],
        queue: QueueId,
        family: u32,
        run: impl FnOnce(&super::range_state::RangeStateTable) -> crate::Result<R>,
    ) -> crate::Result<R> {
        let _gate = self.lock_submission();
        let mut table = self.ensure_range_state_locked(initial, stats)?;
        let table = table.as_mut().expect("initialized native state");
        struct Preview<'a>(&'a mut super::range_state::RangeStateTable);
        impl Drop for Preview<'_> {
            fn drop(&mut self) {
                self.0.cancel_prepared();
            }
        }
        let preview = Preview(table);
        preview.0.prepare(patches, queue, family)?;
        run(preview.0)
    }

    pub(crate) fn with_host_buffer_ranges(
        mut self,
        size: u64,
        budget: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<Self> {
        let domain = OwnershipRange::Buffer { offset: 0, size };
        self.ownership_range = Some(domain);
        let sharing = self.concurrent_families.map_or(
            super::ResourceSharing::Exclusive,
            super::ResourceSharing::Concurrent,
        );
        *self.native_state.get_mut() = Some(super::range_state::RangeStateTable::new(
            domain,
            sharing,
            super::range_state::NativeState::default(),
            Default::default(),
            budget.clone(),
        )?);
        Ok(self)
    }
    /// The caller holds lock_submission through this observation and the host
    /// memory access, so no new submit can publish into the granted range.
    pub(crate) fn buffer_range_snapshot(
        &self,
        offset: u64,
        size: u64,
        write: bool,
        completed: &CompletedValues,
    ) -> crate::Result<SyncSet> {
        let state = self.native_state.lock();
        let required = state
            .as_ref()
            .ok_or_else(|| crate::Error::invalid_usage("host buffer native state is missing"))?
            .host_range_snapshot(OwnershipRange::Buffer { offset, size }, write)?;
        let mut remaining = SyncSet::EMPTY;
        for point in required.points() {
            if completed.get(point.queue) < point.value {
                remaining.insert(point);
            }
        }
        Ok(remaining)
    }
    pub(crate) fn prepare_buffer_host_range(
        &self,
        offset: u64,
        size: u64,
        write: bool,
    ) -> crate::Result<()> {
        self.native_state
            .lock()
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("host buffer native state is missing"))?
            .prepare_host(OwnershipRange::Buffer { offset, size }, write)
    }
    pub(crate) fn publish_buffer_host_range(&self) {
        self.native_state
            .lock()
            .as_mut()
            .expect("prepared host buffer state")
            .publish_host();
    }

    /// 登记一次成功提交：把该队列“用到的最晚值”推进到 `point.value`。
    ///
    /// 用 `AcqRel`：写侧以 release 发布本次登记，使回收线程 acquire 后能看到“该资源确实
    /// 被用到这一提交”；读侧 acquire 建立与其他写者的先后序。值只增不减（`fetch_max`）。
    pub fn insert(&self, point: SyncPoint) {
        let _gate = self.lock_submission();
        self.insert_overall(point);
        if let Some(table) = self.native_state.lock().as_mut() {
            table.record_opaque_point(point);
        }
    }
    pub(crate) fn insert_overall(&self, point: SyncPoint) {
        if let Some(accounting) = &self.accounting {
            accounting.stats.record_gpu_use(accounting.id, point);
        }
        self.values[point.queue.index()].fetch_max(point.value, Ordering::AcqRel);
    }

    /// 批量登记一组队列依赖。
    pub fn merge(&self, set: &SyncSet) {
        for point in set.points() {
            self.insert(point);
        }
    }

    /// 取当前登记快照；结果随资源的 `Arc` 一起交给 `Retirement`。
    pub fn snapshot(&self) -> SyncSet {
        let mut out = SyncSet::EMPTY;
        for (index, slot) in self.values.iter().enumerate() {
            let value = slot.load(Ordering::Acquire);
            if value != 0 {
                out.insert(SyncPoint {
                    queue: QueueId::new(index).expect("下标恒小于 MAX_QUEUES"),
                    value,
                });
            }
        }
        out
    }

    /// 所有已登记队列的值是否都已完成。
    pub fn is_reached(&self, completed: &CompletedValues) -> bool {
        self.values.iter().enumerate().all(|(index, slot)| {
            let need = slot.load(Ordering::Acquire);
            need == 0 || need <= completed.get(QueueId::new(index).expect("下标恒小于 MAX_QUEUES"))
        })
    }
}

/// Recording owns the only mutable resource list. Successful command end
/// seals its order; late ownership patches preserve that order by insertion.
pub(crate) struct SubmissionUseCells {
    cells: crate::raw::temp_list::TempList<std::sync::Arc<SyncCell>>,
    seen: crate::collections::token_map::TokenMap<()>,
    sealed: bool,
}
impl SubmissionUseCells {
    pub(crate) fn new(
        max: usize,
        stats: std::sync::Arc<crate::raw::allocator::AllocatorStats>,
        pool: &std::sync::Arc<SyncCell>,
    ) -> crate::Result<Self> {
        let mut result = Self {
            cells: crate::raw::temp_list::TempList::new(max, stats)?,
            seen: Default::default(),
            sealed: false,
        };
        result.insert(pool)?;
        Ok(result)
    }
    pub(crate) fn insert(&mut self, cell: &std::sync::Arc<SyncCell>) -> crate::Result<()> {
        let key = std::sync::Arc::as_ptr(cell) as usize as u64;
        if self.seen.contains_key(&key) {
            return Ok(());
        }
        self.cells.add(cell.clone())?;
        self.seen.insert(key, ());
        if self.sealed {
            self.cells
                .sift_last_by_key(|cell| std::sync::Arc::as_ptr(cell) as usize);
        }
        Ok(())
    }
    pub(crate) fn seal(&mut self) {
        self.cells
            .sort_dedup_by_key(|cell| std::sync::Arc::as_ptr(cell) as usize);
        self.sealed = true;
    }
    pub(crate) fn is_sealed(&self) -> bool {
        self.sealed
    }
    pub(crate) fn as_slice(&self) -> &[std::sync::Arc<SyncCell>] {
        self.cells.as_slice()
    }
    pub(crate) fn ordered(&self) -> crate::Result<OrderedSubmissionUses<'_>> {
        if !self.sealed {
            return Err(crate::Error::invalid_usage(
                "submitted command has not sealed its resource uses",
            ));
        }
        Ok(OrderedSubmissionUses {
            cells: self.cells.as_slice(),
        })
    }
    fn reset_inner(&mut self, pool: &std::sync::Arc<SyncCell>, sealed: bool) {
        self.cells.clear();
        self.seen.clear();
        self.sealed = false;
        self.insert(pool)
            .expect("pool cell capacity was admitted at allocation");
        self.sealed = sealed;
    }
    pub(crate) fn reset(&mut self, pool: &std::sync::Arc<SyncCell>) {
        self.reset_inner(pool, false);
    }
    pub(crate) fn release_completed(&mut self, pool: &std::sync::Arc<SyncCell>) {
        self.reset_inner(pool, self.sealed);
    }
}

/// Only sealed recording and the ordered merge below can construct this view.
#[derive(Clone, Copy)]
pub(crate) struct OrderedSubmissionUses<'a> {
    cells: &'a [std::sync::Arc<SyncCell>],
}
impl<'a> OrderedSubmissionUses<'a> {
    pub(crate) fn as_slice(self) -> &'a [std::sync::Arc<SyncCell>] {
        self.cells
    }
}
impl std::ops::Deref for OrderedSubmissionUses<'_> {
    type Target = [std::sync::Arc<SyncCell>];
    fn deref(&self) -> &Self::Target {
        self.cells
    }
}
pub(crate) struct OwnedOrderedSubmissionUses {
    cells: crate::raw::submit::SubmitScratch<[std::sync::Arc<SyncCell>; 32]>,
}
impl OwnedOrderedSubmissionUses {
    pub(crate) fn ordered(&self) -> OrderedSubmissionUses<'_> {
        OrderedSubmissionUses { cells: &self.cells }
    }
}
impl std::ops::Deref for OwnedOrderedSubmissionUses {
    type Target = [std::sync::Arc<SyncCell>];
    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

/// Merge the sorted, deduplicated cell lists sealed by command recording.
/// Two heap-merge passes count and then collect actual unique owners, in
/// O(total uses * log(command count)) time with bounded/fallible cursor storage.
pub(crate) fn collect_submission_uses(
    sources: &[OrderedSubmissionUses<'_>],
    stats: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
) -> crate::Result<OwnedOrderedSubmissionUses> {
    if sources.len() <= 1 {
        let cells = sources.first().map_or(&[][..], |source| source.cells);
        return Ok(OwnedOrderedSubmissionUses {
            cells: crate::raw::submit::SubmitScratch::try_collect(
                cells.len(),
                stats,
                cells.iter().cloned().map(Ok),
            )?,
        });
    }
    let mut positions = crate::raw::submit::SubmitScratch::<[usize; 8]>::try_collect(
        sources.len(),
        stats,
        (0..sources.len()).map(|_| Ok(0)),
    )?;
    let mut heap = crate::raw::submit::SubmitScratch::<[(usize, usize); 8]>::try_collect(
        sources.len(),
        stats,
        (0..sources.len()).map(|_| Ok((0, 0))),
    )?;
    let mut merged = SubmissionUseMerge {
        sources,
        positions: &mut positions,
        heap: &mut heap,
        heap_len: 0,
    };
    merged.reset();
    let count = merged.by_ref().count();
    merged.reset();
    Ok(OwnedOrderedSubmissionUses {
        cells: crate::raw::submit::SubmitScratch::try_collect(
            count,
            stats,
            merged.map(|cell| Ok(cell.clone())),
        )?,
    })
}
struct SubmissionUseMerge<'a, 'b> {
    sources: &'b [OrderedSubmissionUses<'a>],
    positions: &'b mut [usize],
    heap: &'b mut [(usize, usize)],
    heap_len: usize,
}
impl SubmissionUseMerge<'_, '_> {
    fn reset(&mut self) {
        self.positions.fill(0);
        self.heap_len = 0;
        for (source, cells) in self.sources.iter().enumerate() {
            if let Some(cell) = cells.first() {
                self.heap[self.heap_len] = (std::sync::Arc::as_ptr(cell) as usize, source);
                self.heap_len += 1;
            }
        }
        for index in (0..self.heap_len / 2).rev() {
            self.sift_down(index);
        }
    }
    fn sift_down(&mut self, mut index: usize) {
        while index < self.heap_len / 2 {
            let left = index * 2 + 1;
            let right = left + 1;
            let child = if right < self.heap_len && self.heap[right] < self.heap[left] {
                right
            } else {
                left
            };
            if self.heap[index] <= self.heap[child] {
                break;
            }
            self.heap.swap(index, child);
            index = child;
        }
    }
    fn advance_root(&mut self) {
        let source = self.heap[0].1;
        self.positions[source] += 1;
        if let Some(next) = self.sources[source].get(self.positions[source]) {
            self.heap[0].0 = std::sync::Arc::as_ptr(next) as usize;
        } else {
            self.heap_len -= 1;
            if self.heap_len != 0 {
                self.heap[0] = self.heap[self.heap_len];
            }
        }
        self.sift_down(0);
    }
}
impl<'a> Iterator for SubmissionUseMerge<'a, '_> {
    type Item = &'a std::sync::Arc<SyncCell>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.heap_len == 0 {
            return None;
        }
        let (pointer, source) = self.heap[0];
        let selected = &self.sources[source].as_slice()[self.positions[source]];
        loop {
            self.advance_root();
            if self.heap_len == 0 || self.heap[0].0 != pointer {
                break;
            }
        }
        Some(selected)
    }
}

pub(crate) fn lock_submission_uses<'a>(
    cells: OrderedSubmissionUses<'a>,
    stats: &std::sync::Arc<crate::raw::allocator::AllocatorStats>,
) -> crate::Result<crate::raw::submit::SubmitScratch<[parking_lot::MutexGuard<'a, ()>; 32]>> {
    crate::raw::submit::SubmitScratch::try_collect(
        cells.len(),
        stats,
        cells
            .as_slice()
            .iter()
            .map(|cell| Ok(cell.lock_submission())),
    )
}

impl Default for SyncCell {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource references expose their use cells during recording, before submission.
/// Implementations may traverse dependencies here; successful submission only
/// updates the collected cells and never invokes user code or allocates memory.
pub type UseVisitor<'a> = dyn FnMut(&std::sync::Arc<SyncCell>) -> crate::Result<()> + 'a;
#[inline]
pub(crate) fn visit_cells<'a>(
    visitor: &mut UseVisitor<'_>,
    cells: impl IntoIterator<Item = &'a std::sync::Arc<SyncCell>>,
) -> crate::Result<()> {
    for cell in cells {
        visitor(cell)?;
    }
    Ok(())
}

pub trait GpuUse: Send + Sync {
    fn visit_uses(&self, visitor: &mut UseVisitor<'_>) -> crate::Result<()>;
    /// True for objects without their own use cell: a command that retained
    /// one keeps it alive until the command's completion, even when the
    /// command wrapper is dropped earlier.
    fn held_until_completion(&self) -> bool {
        false
    }
}
impl<T: GpuUse + ?Sized> GpuUse for std::sync::Arc<T> {
    fn visit_uses(&self, visitor: &mut UseVisitor<'_>) -> crate::Result<()> {
        (**self).visit_uses(visitor)
    }
    fn held_until_completion(&self) -> bool {
        (**self).held_until_completion()
    }
}
impl GpuUse for () {
    fn visit_uses(&self, _: &mut UseVisitor<'_>) -> crate::Result<()> {
        Ok(())
    }
}
impl<T: GpuUse + 'static> GpuUse for crate::raw::managed::handle::Managed<T> {
    fn visit_uses(&self, visitor: &mut UseVisitor<'_>) -> crate::Result<()> {
        self.resource().visit_uses(visitor)
    }
}

#[cfg(test)]
mod tests {
    fn sealed(cells: &[std::sync::Arc<super::SyncCell>]) -> super::SubmissionUseCells {
        let stats = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
        let mut uses = super::SubmissionUseCells::new(4096, stats, &cells[0]).unwrap();
        for cell in cells {
            uses.insert(cell).unwrap();
        }
        uses.seal();
        uses
    }
    #[test]
    fn normalized_commands_merge_unique_cells_before_spill_admission() {
        let stats = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
        stats.set_total_budget(1).unwrap();
        let cells: Vec<_> = (0..3)
            .map(|_| std::sync::Arc::new(super::SyncCell::new()))
            .collect();
        let repeated: Vec<_> = (0..1000)
            .map(|index| cells[index % cells.len()].clone())
            .collect();
        let repeated = sealed(&repeated);
        let sources = [repeated.ordered().unwrap(), repeated.ordered().unwrap()];
        let uses = super::collect_submission_uses(&sources, &stats).unwrap();
        assert_eq!(uses.len(), 3);
        let guards = super::lock_submission_uses(uses.ordered(), &stats).unwrap();
        assert_eq!(guards.len(), 3);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        assert!(
            uses.windows(2)
                .all(|p| std::sync::Arc::as_ptr(&p[0]) < std::sync::Arc::as_ptr(&p[1]))
        );
        drop(guards);
        let many: Vec<_> = (0..33)
            .map(|_| std::sync::Arc::new(super::SyncCell::new()))
            .collect();
        let many = sealed(&many);
        assert!(matches!(
            super::collect_submission_uses(
                &[many.ordered().unwrap(), many.ordered().unwrap()],
                &stats
            ),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        stats.set_total_budget(1 << 20).unwrap();
        let uses = super::collect_submission_uses(
            &[many.ordered().unwrap(), many.ordered().unwrap()],
            &stats,
        )
        .unwrap();
        assert_eq!(uses.len(), many.as_slice().len());
        assert!(
            uses.iter()
                .zip(many.as_slice())
                .all(|(a, b)| std::sync::Arc::ptr_eq(a, b))
        );
        drop(uses);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn heap_merge_handles_empty_interleaved_and_repeated_worker_lists() {
        let stats = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
        let mut cells: Vec<_> = (0..257)
            .map(|_| std::sync::Arc::new(super::SyncCell::new()))
            .collect();
        cells.sort_unstable_by_key(|cell| std::sync::Arc::as_ptr(cell) as usize);
        let lanes: Vec<_> = (0..16)
            .map(|lane| {
                sealed(
                    &cells
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| (i + lane) % 7 == 0 || i % 13 == 0)
                        .map(|(_, cell)| cell.clone())
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let empty = super::collect_submission_uses(&[], &stats).unwrap();
        let mut sources: Vec<_> = lanes.iter().map(|lane| lane.ordered().unwrap()).collect();
        sources.push(empty.ordered());
        let actual = super::collect_submission_uses(&sources, &stats).unwrap();
        assert_eq!(actual.len(), cells.len());
        assert!(
            actual
                .iter()
                .zip(&cells)
                .all(|(a, b)| std::sync::Arc::ptr_eq(a, b))
        );
        drop(actual);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        assert!(
            super::collect_submission_uses(&[], &stats)
                .unwrap()
                .is_empty()
        );
        let single = super::collect_submission_uses(&sources[..1], &stats).unwrap();
        assert_eq!(single.len(), lanes[0].as_slice().len());
    }
    #[test]
    fn recording_seal_late_insert_and_resets_preserve_the_only_lock_order() {
        let stats = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
        let mut cells: Vec<_> = (0..4)
            .map(|_| std::sync::Arc::new(super::SyncCell::new()))
            .collect();
        cells.sort_unstable_by_key(|cell| std::sync::Arc::as_ptr(cell) as usize);
        let mut uses = super::SubmissionUseCells::new(3, stats, &cells[2]).unwrap();
        assert!(uses.ordered().is_err());
        uses.insert(&cells[0]).unwrap();
        uses.insert(&cells[0]).unwrap();
        uses.seal();
        uses.insert(&cells[1]).unwrap();
        assert!(
            uses.as_slice()
                .iter()
                .zip(&cells[..3])
                .all(|(a, b)| std::sync::Arc::ptr_eq(a, b))
        );
        assert!(uses.insert(&cells[3]).is_err());
        assert_eq!(uses.ordered().unwrap().len(), 3);
        uses.release_completed(&cells[2]);
        assert!(uses.is_sealed());
        assert_eq!(uses.as_slice().len(), 1);
        uses.reset(&cells[2]);
        assert!(!uses.is_sealed());
        assert!(uses.ordered().is_err());
        uses.release_completed(&cells[2]);
        assert!(!uses.is_sealed());
        uses.seal();
        uses.insert(&cells[3]).unwrap();
        uses.insert(&cells[0]).unwrap();
        assert!(
            uses.as_slice()
                .windows(2)
                .all(|p| std::sync::Arc::as_ptr(&p[0]) < std::sync::Arc::as_ptr(&p[1]))
        );
        assert_eq!(uses.ordered().unwrap().len(), 3);
    }
    #[test]
    fn failed_spill_does_not_publish_membership_or_break_sealed_order() {
        let stats = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
        let cells: Vec<_> = (0..3)
            .map(|_| std::sync::Arc::new(super::SyncCell::new()))
            .collect();
        let mut uses = super::SubmissionUseCells::new(8, stats.clone(), &cells[0]).unwrap();
        uses.insert(&cells[1]).unwrap();
        uses.seal();
        let before = stats.cpu_memory_usage().total();
        stats.set_total_budget(before).unwrap();
        assert!(matches!(
            uses.insert(&cells[2]),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(uses.ordered().unwrap().len(), 2);
        assert_eq!(stats.cpu_memory_usage().total(), before);
        stats.set_total_budget(1 << 20).unwrap();
        uses.insert(&cells[2]).unwrap();
        assert_eq!(uses.ordered().unwrap().len(), 3);
        assert!(
            uses.as_slice()
                .windows(2)
                .all(|p| std::sync::Arc::as_ptr(&p[0]) < std::sync::Arc::as_ptr(&p[1]))
        );
        drop(uses);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    use super::*;

    fn q(index: usize) -> QueueId {
        QueueId::new(index).unwrap()
    }

    #[test]
    fn submission_rejects_foreign_and_uninitialized_use_cells() {
        let first = crate::device::DeviceGeneration::next();
        let second = crate::device::DeviceGeneration::next();
        let cell = SyncCell::for_device(first);
        assert!(cell.validate_device(first).is_ok());
        assert!(cell.validate_device(second).is_err());
        assert!(SyncCell::new().validate_device(first).is_err());
        assert!(
            SyncCell::new()
                .validate_device(crate::device::DeviceGeneration::INVALID)
                .is_err()
        );
        assert!(cell.snapshot().is_empty());
    }

    #[test]
    fn insert_keeps_the_latest_value_per_queue() {
        let cell = SyncCell::new();
        cell.insert(SyncPoint {
            queue: q(0),
            value: 5,
        });
        cell.insert(SyncPoint {
            queue: q(0),
            value: 3,
        });
        cell.insert(SyncPoint {
            queue: q(2),
            value: 7,
        });

        let snapshot = cell.snapshot();
        assert_eq!(snapshot.get(q(0)), 5, "只增不减");
        assert_eq!(snapshot.get(q(1)), 0);
        assert_eq!(snapshot.get(q(2)), 7);
    }

    #[test]
    fn merge_applies_every_queue_dependency() {
        let cell = SyncCell::new();
        let mut set = SyncSet::from_point(SyncPoint {
            queue: q(0),
            value: 4,
        });
        set.insert(SyncPoint {
            queue: q(1),
            value: 2,
        });
        cell.merge(&set);
        assert_eq!(cell.snapshot(), set);
    }

    #[test]
    fn reached_requires_every_registered_queue() {
        let cell = SyncCell::new();
        cell.insert(SyncPoint {
            queue: q(0),
            value: 4,
        });
        cell.insert(SyncPoint {
            queue: q(1),
            value: 2,
        });

        let mut done = CompletedValues::default();
        done.raise(q(0), 10);
        assert!(!cell.is_reached(&done), "队列 1 尚未完成");
        done.raise(q(1), 2);
        assert!(cell.is_reached(&done));
        assert!(SyncCell::new().is_reached(&CompletedValues::default()));
    }

    #[test]
    fn concurrent_inserts_are_serialized_and_lossless() {
        use std::sync::Arc;

        let cell = Arc::new(SyncCell::new());
        std::thread::scope(|scope| {
            for worker in 0..8u64 {
                let cell = Arc::clone(&cell);
                scope.spawn(move || {
                    for value in 1..=1000 {
                        cell.insert(SyncPoint {
                            queue: q((worker % 4) as usize),
                            value,
                        });
                    }
                });
            }
        });

        let snapshot = cell.snapshot();
        for queue in 0..4 {
            assert_eq!(snapshot.get(q(queue)), 1000, "每个队列都有线程写满到 1000");
        }
    }

    #[test]
    fn host_reservation_prevents_new_submits_until_access_is_finished() {
        use std::{
            sync::{Arc, mpsc},
            time::Duration,
        };
        let cell = Arc::new(SyncCell::new());
        let host = cell.lock_submission();
        let initial = cell.snapshot();
        let (started, ready) = mpsc::channel();
        let (published, received) = mpsc::channel();
        let worker_cell = cell.clone();
        let worker = std::thread::spawn(move || {
            let cells = sealed(&[worker_cell]);
            let stats = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
            started.send(()).unwrap();
            let _reservation = lock_submission_uses(cells.ordered().unwrap(), &stats).unwrap();
            cells.as_slice()[0].insert_overall(SyncPoint {
                queue: q(0),
                value: 1,
            });
            published.send(()).unwrap();
        });
        ready.recv_timeout(Duration::from_secs(2)).unwrap();
        assert!(received.recv_timeout(Duration::from_millis(20)).is_err());
        assert_eq!(
            cell.snapshot(),
            initial,
            "host completion snapshot remains valid under its reservation"
        );
        drop(host);
        received.recv_timeout(Duration::from_secs(2)).unwrap();
        worker.join().unwrap();
        assert_eq!(cell.snapshot().get(q(0)), 1);
    }

    #[test]
    fn native_state_query_leaves_untracked_image_available_for_real_import_initialization() {
        use super::super::range_state::{NativeState, RangeOwner};
        let budget = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
        let cell = SyncCell::for_device(crate::device::DeviceGeneration::next())
            .with_ownership_range(OwnershipRange::Image {
                aspects: ash::vk::ImageAspectFlags::COLOR.as_raw(),
                base_mip: 0,
                mip_count: 1,
                base_layer: 0,
                layer_count: 1,
            });
        assert_eq!(cell.with_existing_range_state(|_| Ok(42)).unwrap(), None);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
        let initial = NativeState {
            layout: ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            owner: RangeOwner::Owned(3),
            ..Default::default()
        };
        cell.with_range_state(initial, &budget, |_| Ok(())).unwrap();
        assert_eq!(
            cell.with_existing_range_state(|table| Ok(table.cells()[0].state))
                .unwrap(),
            Some(initial)
        );
    }

    #[test]
    fn standalone_cell_capacity_follows_the_last_recording_owner() {
        let stats = std::sync::Arc::new(crate::raw::allocator::AllocatorStats::default());
        stats.set_total_budget(1).unwrap();
        assert!(matches!(
            SyncCell::for_device_budgeted(crate::device::DeviceGeneration::next(), &stats),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        stats.set_total_budget(u64::MAX).unwrap();
        let owner =
            SyncCell::for_device_budgeted(crate::device::DeviceGeneration::next(), &stats).unwrap();
        let recording = owner.clone();
        let bytes = crate::raw::allocator::shared_cpu_bytes::<SyncCell>().unwrap();
        assert_eq!(stats.cpu_memory_usage().live, bytes);
        drop(owner);
        assert_eq!(stats.cpu_memory_usage().live, bytes);
        drop(recording);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
