//! Native per-range state, prepared under the existing submission gate and
//! published only with the existing device timeline's successful SyncPoint.
//! This module records no independent completion counter and issues no Vulkan.
use super::{OwnershipRange, ResourceSharing};
use crate::{
    Error, Result,
    device::BudgetCategory,
    raw::allocator::{AllocatorStats, CpuMemoryReservation},
    sync::{MAX_QUEUES, QueueId, StagedDependencies, SyncPoint, SyncSet},
};
use ash::vk;
use smallvec::SmallVec;
use std::sync::Arc;

pub type NativeRange = OwnershipRange;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccessScope {
    pub stages: vk::PipelineStageFlags,
    pub access: vk::AccessFlags,
    pub write: bool,
}
impl AccessScope {
    pub const EMPTY: Self = Self {
        stages: vk::PipelineStageFlags::empty(),
        access: vk::AccessFlags::empty(),
        write: false,
    };
    pub fn union(self, other: Self) -> Self {
        Self {
            stages: self.stages | other.stages,
            access: self.access | other.access,
            write: self.write || other.write,
        }
    }
    pub fn is_empty(self) -> bool {
        self.stages.is_empty() && self.access.is_empty()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frontier {
    pub scopes: [AccessScope; MAX_QUEUES],
    pub points: SyncSet,
    /// Host accesses complete synchronously and have no device timeline point.
    pub host: AccessScope,
}
impl Default for Frontier {
    fn default() -> Self {
        Self::EMPTY
    }
}
impl Frontier {
    pub const EMPTY: Self = Self {
        scopes: [AccessScope::EMPTY; MAX_QUEUES],
        points: SyncSet::EMPTY,
        host: AccessScope::EMPTY,
    };
    pub fn combined(&self) -> AccessScope {
        self.scopes
            .iter()
            .copied()
            .fold(self.host, AccessScope::union)
    }
    pub fn for_queue(&self, queue: QueueId) -> AccessScope {
        self.scopes[queue.index()].union(self.host)
    }
    pub fn add(&mut self, queue: QueueId, scope: AccessScope) {
        self.scopes[queue.index()] = self.scopes[queue.index()].union(scope);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeTransfer {
    pub pair: u64,
    pub source_family: u32,
    pub destination_family: u32,
    pub range: NativeRange,
    pub source: AccessScope,
    pub destination: AccessScope,
    pub old_layout: vk::ImageLayout,
    pub new_layout: vk::ImageLayout,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RangeOwner {
    #[default]
    Unclaimed,
    Owned(u32),
    Released {
        transfer: RangeTransfer,
        point: SyncPoint,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeState {
    pub layout: vk::ImageLayout,
    pub layout_known: bool,
    pub writer: Frontier,
    pub readers: Frontier,
    pub owner: RangeOwner,
    pub revision: u64,
}
impl Default for NativeState {
    fn default() -> Self {
        Self {
            layout: vk::ImageLayout::UNDEFINED,
            layout_known: true,
            writer: Frontier::EMPTY,
            readers: Frontier::EMPTY,
            owner: RangeOwner::Unclaimed,
            revision: 0,
        }
    }
}
impl NativeState {
    pub fn dependencies(&self, write_or_layout_change: bool) -> SyncSet {
        let mut set = self.writer.points;
        if write_or_layout_change {
            set.merge(&self.readers.points);
        }
        if let RangeOwner::Released { point, .. } = self.owner {
            set.insert(point);
        }
        set
    }
    pub fn source_scope(&self, queue: QueueId, write_or_layout_change: bool) -> AccessScope {
        let scope = self.writer.for_queue(queue);
        if write_or_layout_change {
            scope.union(self.readers.for_queue(queue))
        } else {
            scope
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeCell {
    pub range: NativeRange,
    pub state: NativeState,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchAction {
    Access {
        scope: AccessScope,
        layout: vk::ImageLayout,
        queue: QueueId,
        family: u32,
    },
    Release(RangeTransfer),
    Acquire {
        transfer: RangeTransfer,
        release_point: SyncPoint,
    },
    /// An opaque native use cannot assert that an image kept its old layout.
    /// Its lifetime and completion are still recorded on the physical resource.
    Unknown {
        queue: QueueId,
        family: u32,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatePatch {
    pub range: NativeRange,
    pub expected_revision: u64,
    pub action: PatchAction,
}
#[derive(Clone, Copy, Debug)]
pub struct RangeStateConfig {
    pub max_cells: usize,
    pub max_cpu_bytes: usize,
}
impl Default for RangeStateConfig {
    fn default() -> Self {
        Self {
            max_cells: 4096,
            max_cpu_bytes: 16 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct PreparedCell {
    cell: RangeCell,
    writer: bool,
    readers: bool,
    release: Option<RangeTransfer>,
}
impl From<RangeCell> for PreparedCell {
    fn from(cell: RangeCell) -> Self {
        Self {
            cell,
            writer: false,
            readers: false,
            release: None,
        }
    }
}
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
            return Err(Error::Timeout);
        }
        // The old ticket stays charged until its actual old backing is freed.
        let mut next = Self::new(capacity, budget)?;
        next.values.append(&mut self.values);
        std::mem::swap(self, &mut next);
        Ok(())
    }
}
/// Caller synchronizes this table using SyncCell::lock_submission. `prepare`
/// leaves committed state unchanged on failure; `publish` cannot allocate.
pub struct RangeStateTable {
    domain: NativeRange,
    sharing: ResourceSharing,
    cells: Storage<RangeCell>,
    scratch: Storage<PreparedCell>,
    config: RangeStateConfig,
    budget: Arc<AllocatorStats>,
    prepared_queue: Option<QueueId>,
    prepared_host: bool,
    prepared_dependencies: StagedDependencies,
}
impl std::fmt::Debug for RangeStateTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RangeStateTable")
            .field("domain", &self.domain)
            .field("cells", &self.cells.values)
            .finish()
    }
}
impl RangeStateTable {
    pub fn new(
        domain: NativeRange,
        sharing: ResourceSharing,
        initial: NativeState,
        config: RangeStateConfig,
        budget: Arc<AllocatorStats>,
    ) -> Result<Self> {
        validate_range(domain)?;
        if matches!(initial.owner, RangeOwner::Released { point, .. } if point.value == 0) {
            return Err(Error::invalid_usage(
                "ownership release requires a real submitted point",
            ));
        }
        if config.max_cells == 0 || config.max_cells > 65_536 {
            return Err(Error::invalid_usage("invalid native range cell limit"));
        }
        let cells = Storage::new(1, &budget)?;
        let scratch = Storage::new(1, &budget)?;
        if cells.bytes() + scratch.bytes() > config.max_cpu_bytes {
            return Err(Error::Timeout);
        }
        let mut result = Self {
            domain,
            sharing,
            cells,
            scratch,
            config,
            budget,
            prepared_queue: None,
            prepared_host: false,
            prepared_dependencies: StagedDependencies::EMPTY,
        };
        result.cells.values.push(RangeCell {
            range: domain,
            state: initial,
        });
        Ok(result)
    }
    pub fn domain(&self) -> NativeRange {
        self.domain
    }
    pub fn cells(&self) -> &[RangeCell] {
        &self.cells.values
    }
    /// Recording preview for helpers that append further Access patches to the
    /// same command. Pending work has scopes/revisions but no fabricated GPU
    /// completion point. Ownership transitions require the bound batch protocol.
    pub fn prepared_cells(&self) -> impl Iterator<Item = RangeCell> + '_ {
        self.scratch.values.iter().map(|prepared| prepared.cell)
    }
    pub fn known_cpu_bytes(&self) -> usize {
        self.cells.bytes() + self.scratch.bytes()
    }
    /// A successful opaque submission through the public whole-resource escape
    /// records a conservative checkpoint. Exhaustion poisons revisions with the
    /// reserved maximum stamp, so subsequent typed preparation fails closed.
    pub fn record_opaque_point(&mut self, point: SyncPoint) {
        self.cancel_prepared();
        for cell in &mut self.cells.values {
            // This public escape has no proof that the new queue waited the
            // old queues. Preserve every previous dependency conservatively.
            let mut dependencies = cell.state.dependencies(true);
            dependencies.insert(point);
            let host = cell.state.writer.host.union(cell.state.readers.host);
            cell.state.writer = Frontier::EMPTY;
            cell.state.writer.host = host;
            cell.state.writer.points = dependencies;
            for pending in dependencies.points() {
                cell.state.writer.add(
                    pending.queue,
                    AccessScope {
                        stages: vk::PipelineStageFlags::ALL_COMMANDS,
                        access: vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
                        write: true,
                    },
                );
            }
            cell.state.readers = Frontier::EMPTY;
            cell.state.layout_known = matches!(cell.range, NativeRange::Buffer { .. });
            cell.state.revision = cell.state.revision.saturating_add(1);
        }
    }
    pub fn cancel_prepared(&mut self) {
        self.prepared_queue = None;
        self.prepared_host = false;
        self.prepared_dependencies = StagedDependencies::EMPTY;
        self.scratch.values.clear();
    }
    /// Ordered patches may include accesses followed by a release in the same
    /// native call. An unsubmitted release is represented explicitly, never by a
    /// made-up timeline point. The successful real point is filled in publish.
    pub fn prepare(&mut self, patches: &[StatePatch], queue: QueueId, family: u32) -> Result<()> {
        self.cancel_prepared();
        self.scratch.grow(
            self.cells.values.len(),
            self.cells.bytes(),
            self.config,
            &self.budget,
        )?;
        self.scratch
            .values
            .extend(self.cells.values.iter().copied().map(PreparedCell::from));
        for patch in patches {
            validate_range(patch.range)?;
            if !contains(self.domain, patch.range) {
                return Err(Error::invalid_usage(
                    "state patch exceeds physical resource",
                ));
            }
            if matches!(patch.action, PatchAction::Release(transfer) | PatchAction::Acquire { transfer, .. } if transfer.range != patch.range)
            {
                return Err(Error::invalid_usage(
                    "ownership patch must exactly match its native transfer range",
                ));
            }
            let mut index = 0;
            let before = self.scratch.values.len();
            // New outside/intersection pieces are appended. Only original cells
            // can be visited by this patch, so no cell is incremented twice.
            let mut visited = 0;
            while visited < before {
                let current = self.scratch.values[index];
                visited += 1;
                let Some(overlap) = intersection(current.cell.range, patch.range) else {
                    index += 1;
                    continue;
                };
                if current.cell.state.revision != patch.expected_revision
                    && !(patch.expected_revision == u64::MAX
                        && matches!(patch.action, PatchAction::Unknown { .. }))
                {
                    return Err(Error::Timeout);
                }
                let mut changed = current;
                changed.cell.range = overlap;
                // Declared accesses know the first stage that touches the
                // range on this queue; opaque uses and releases do not.
                let (dependencies, stages) = match patch.action {
                    PatchAction::Access { scope, layout, .. } => (
                        current.cell.state.dependencies(
                            scope.write
                                || !current.cell.state.layout_known
                                || current.cell.state.layout != layout,
                        ),
                        scope.stages,
                    ),
                    PatchAction::Release(_) | PatchAction::Unknown { .. } => (
                        current.cell.state.dependencies(true),
                        vk::PipelineStageFlags::ALL_COMMANDS,
                    ),
                    PatchAction::Acquire {
                        release_point,
                        transfer,
                    } => (
                        SyncSet::from_point(release_point),
                        transfer.destination.stages,
                    ),
                };
                self.prepared_dependencies.merge_set(&dependencies, stages);
                apply_preview(&mut changed, patch.action, self.sharing, queue, family)?;
                changed.cell.state.revision = current
                    .cell
                    .state
                    .revision
                    .checked_add(1)
                    .filter(|revision| *revision != u64::MAX)
                    .ok_or_else(|| Error::unsupported("native range revision exhausted"))?;
                let outside = difference(current.cell.range, overlap);
                let wanted = self
                    .scratch
                    .values
                    .len()
                    .checked_add(outside.len())
                    .ok_or(Error::OutOfHostMemory)?;
                self.scratch
                    .grow(wanted, self.cells.bytes(), self.config, &self.budget)?;
                // Keep the intersection in-place; outside pieces are not revisited.
                self.scratch.values[index] = changed;
                self.scratch
                    .values
                    .extend(outside.into_iter().map(|range| PreparedCell {
                        cell: RangeCell {
                            range,
                            state: current.cell.state,
                        },
                        ..current
                    }));
                index += 1;
            }
        }
        self.cells.grow(
            self.scratch.values.len(),
            self.scratch.bytes(),
            self.config,
            &self.budget,
        )?;
        self.prepared_queue = Some(queue);
        Ok(())
    }
    /// External, already-submitted prerequisites observed while preparing this
    /// batch under its gate. Pending accesses in the same native batch introduce
    /// no invented point and rely on the emitted command-order dependencies.
    pub fn prepared_dependencies(&self) -> SyncSet {
        self.prepared_dependencies.points()
    }
    /// The same prerequisites with, per source queue, the destination stages
    /// of the declared accesses that need them (`ALL_COMMANDS` for opaque
    /// uses and releases).
    pub fn prepared_staged_dependencies(&self) -> StagedDependencies {
        self.prepared_dependencies
    }
    /// Admit all splits before a mapped operation. Caller holds the same gate
    /// used by GPU submission, waits the overlapping dependencies and performs
    /// invalidate/copy/flush. Once a write copied any byte, publish even if flush
    /// fails: a recorded GPU precondition must not accept the old host revision.
    pub fn prepare_host(&mut self, range: NativeRange, write: bool) -> Result<()> {
        self.cancel_prepared();
        validate_range(range)?;
        if !matches!(range, NativeRange::Buffer { .. }) || !contains(self.domain, range) {
            return Err(Error::invalid_usage("mapped host range exceeds its buffer"));
        }
        self.scratch.grow(
            self.cells.values.len(),
            self.cells.bytes(),
            self.config,
            &self.budget,
        )?;
        self.scratch
            .values
            .extend(self.cells.values.iter().copied().map(PreparedCell::from));
        let before = self.scratch.values.len();
        for index in 0..before {
            let current = self.scratch.values[index];
            let Some(overlap) = intersection(current.cell.range, range) else {
                continue;
            };
            if matches!(current.cell.state.owner, RangeOwner::Released { .. }) {
                return Err(Error::Timeout);
            }
            let mut changed = current;
            changed.cell.range = overlap;
            changed.cell.state.revision = changed
                .cell
                .state
                .revision
                .checked_add(1)
                .ok_or_else(|| Error::unsupported("native range revision exhausted"))?;
            let scope = AccessScope {
                stages: vk::PipelineStageFlags::HOST,
                access: if write {
                    vk::AccessFlags::HOST_WRITE
                } else {
                    vk::AccessFlags::HOST_READ
                },
                write,
            };
            if write {
                changed.cell.state.writer = Frontier {
                    host: scope,
                    ..Frontier::EMPTY
                };
                changed.cell.state.readers = Frontier::EMPTY;
            } else {
                changed.cell.state.readers.host = changed.cell.state.readers.host.union(scope);
            }
            let outside = difference(current.cell.range, overlap);
            self.scratch.grow(
                self.scratch.values.len() + outside.len(),
                self.cells.bytes(),
                self.config,
                &self.budget,
            )?;
            self.scratch.values[index] = changed;
            self.scratch
                .values
                .extend(outside.into_iter().map(|range| PreparedCell {
                    cell: RangeCell {
                        range,
                        state: current.cell.state,
                    },
                    ..current
                }));
        }
        self.cells.grow(
            self.scratch.values.len(),
            self.scratch.bytes(),
            self.config,
            &self.budget,
        )?;
        self.prepared_host = true;
        Ok(())
    }
    pub fn publish_host(&mut self) {
        assert!(
            std::mem::take(&mut self.prepared_host),
            "host access must be admitted before publication"
        );
        self.cells.values.clear();
        self.cells
            .values
            .extend(self.scratch.values.iter().map(|prepared| prepared.cell));
        coalesce(&mut self.cells.values);
        self.scratch.values.clear();
    }
    /// Exact overlapping completion projection; it never replaces the whole
    /// SyncCell retirement upper bound.
    pub fn host_range_snapshot(&self, range: NativeRange, write: bool) -> Result<SyncSet> {
        validate_range(range)?;
        if !contains(self.domain, range) {
            return Err(Error::invalid_usage("host range exceeds buffer"));
        }
        let mut result = SyncSet::EMPTY;
        for cell in &self.cells.values {
            if intersection(cell.range, range).is_some() {
                result.merge(&cell.state.dependencies(write));
            }
        }
        Ok(result)
    }
    pub fn publish(&mut self, point: SyncPoint) {
        assert_ne!(
            point.value, 0,
            "successful submissions never publish the empty timeline sentinel"
        );
        assert_eq!(
            self.prepared_queue.take(),
            Some(point.queue),
            "only a successful prepared native submit can publish"
        );
        self.cells.values.clear();
        for prepared in &self.scratch.values {
            let mut cell = prepared.cell;
            if prepared.writer {
                cell.state.writer.points.insert(point);
            }
            if prepared.readers {
                cell.state.readers.points.insert(point);
            }
            if let Some(transfer) = prepared.release {
                cell.state.owner = RangeOwner::Released { transfer, point };
            }
            self.cells.values.push(cell);
        }
        coalesce(&mut self.cells.values);
        self.scratch.values.clear();
    }
}
fn apply_preview(
    prepared: &mut PreparedCell,
    action: PatchAction,
    sharing: ResourceSharing,
    queue: QueueId,
    family: u32,
) -> Result<()> {
    let state = &mut prepared.cell.state;
    match action {
        PatchAction::Access {
            scope,
            layout,
            queue: expected_queue,
            family: expected_family,
        } => {
            if queue != expected_queue || family != expected_family || prepared.release.is_some() {
                return Err(Error::invalid_usage(
                    "range access uses another queue or a pending release",
                ));
            }
            match sharing {
                ResourceSharing::Concurrent(families) => {
                    if !families.as_slice().contains(&family) {
                        return Err(Error::invalid_usage("concurrent access family is absent"));
                    }
                }
                ResourceSharing::Exclusive => match state.owner {
                    RangeOwner::Unclaimed => state.owner = RangeOwner::Owned(family),
                    RangeOwner::Owned(owner) if owner == family => {}
                    _ => {
                        return Err(Error::invalid_usage(
                            "range access needs a matching ownership acquire",
                        ));
                    }
                },
            }
            // A layout conversion is a visibility checkpoint even for a read.
            // Later readers on another VkQueue must wait its actual point.
            if scope.write || !state.layout_known || state.layout != layout {
                state.writer = Frontier::EMPTY;
                state.writer.add(queue, scope);
                state.readers = Frontier::EMPTY;
                prepared.writer = true;
                prepared.readers = false;
            } else {
                state.readers.add(queue, scope);
                prepared.readers = true;
            }
            state.layout = layout;
            state.layout_known = true;
        }
        PatchAction::Release(transfer) => {
            validate_transfer(transfer)?;
            if sharing.is_concurrent()
                || family != transfer.source_family
                || prepared.release.is_some()
                || state.owner != RangeOwner::Owned(family)
                || !contains(transfer.range, prepared.cell.range)
                || !state.layout_known
                || state.layout != transfer.old_layout
            {
                return Err(Error::invalid_usage(
                    "release range, owner or old layout does not match",
                ));
            }
            prepared.release = Some(transfer);
            state.layout = transfer.new_layout;
        }
        PatchAction::Acquire {
            transfer,
            release_point,
        } => {
            validate_transfer(transfer)?;
            if sharing.is_concurrent()
                || family != transfer.destination_family
                || prepared.release.is_some()
                || state.owner
                    != (RangeOwner::Released {
                        transfer,
                        point: release_point,
                    })
                || !contains(transfer.range, prepared.cell.range)
            {
                return Err(Error::invalid_usage(
                    "acquire does not match the actual released range and point",
                ));
            }
            state.owner = RangeOwner::Owned(family);
            state.layout = transfer.new_layout;
            state.layout_known = true;
            // Acquire checkpoints visibility but does not claim logical contents
            // were initialized; temporal/version validity lives in the graph.
            state.writer = Frontier::EMPTY;
            state.readers = Frontier::EMPTY;
            state.writer.add(queue, transfer.destination);
            prepared.writer = true;
            prepared.readers = false;
        }
        PatchAction::Unknown {
            queue: expected_queue,
            family: expected_family,
        } => {
            if queue != expected_queue
                || family != expected_family
                || prepared.release.is_some()
                || !sharing.allows_family(family)
            {
                return Err(Error::invalid_usage(
                    "opaque access uses another queue or pending release",
                ));
            }
            if !sharing.is_concurrent() {
                match state.owner {
                    RangeOwner::Unclaimed => state.owner = RangeOwner::Owned(family),
                    RangeOwner::Owned(owner) if owner == family => {}
                    _ => {
                        return Err(Error::invalid_usage(
                            "opaque access requires ownership acquire",
                        ));
                    }
                }
            }
            state.writer = Frontier::EMPTY;
            state.writer.add(
                queue,
                AccessScope {
                    stages: vk::PipelineStageFlags::ALL_COMMANDS,
                    access: vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
                    write: true,
                },
            );
            state.readers = Frontier::EMPTY;
            state.layout_known = matches!(prepared.cell.range, NativeRange::Buffer { .. });
            prepared.writer = true;
            prepared.readers = false;
        }
    }
    Ok(())
}
pub fn validate_transfer(transfer: RangeTransfer) -> Result<()> {
    validate_range(transfer.range)?;
    if transfer.pair == 0
        || transfer.source_family == transfer.destination_family
        || transfer.source_family == u32::MAX
        || transfer.destination_family == u32::MAX
    {
        return Err(Error::invalid_usage("invalid native range ownership pair"));
    }
    Ok(())
}
pub fn validate_range(range: NativeRange) -> Result<()> {
    let valid = match range {
        NativeRange::Buffer { offset, size } => size != 0 && offset.checked_add(size).is_some(),
        NativeRange::Image {
            aspects,
            base_mip,
            mip_count,
            base_layer,
            layer_count,
        } => {
            aspects != 0
                && mip_count != 0
                && layer_count != 0
                && base_mip.checked_add(mip_count).is_some()
                && base_layer.checked_add(layer_count).is_some()
        }
    };
    if valid {
        Ok(())
    } else {
        Err(Error::invalid_usage("invalid native synchronization range"))
    }
}
pub fn contains(outer: NativeRange, inner: NativeRange) -> bool {
    intersection(outer, inner) == Some(inner)
}
pub fn intersection(a: NativeRange, b: NativeRange) -> Option<NativeRange> {
    match (a, b) {
        (
            NativeRange::Buffer { offset: a, size: n },
            NativeRange::Buffer { offset: b, size: m },
        ) => {
            let start = a.max(b);
            let end = a.checked_add(n)?.min(b.checked_add(m)?);
            (start < end).then(|| NativeRange::Buffer {
                offset: start,
                size: end - start,
            })
        }
        (
            NativeRange::Image {
                aspects: a,
                base_mip: am,
                mip_count: an,
                base_layer: al,
                layer_count: ac,
            },
            NativeRange::Image {
                aspects: b,
                base_mip: bm,
                mip_count: bn,
                base_layer: bl,
                layer_count: bc,
            },
        ) => {
            let aspects = a & b;
            let mip = am.max(bm);
            let end_mip = am.checked_add(an)?.min(bm.checked_add(bn)?);
            let layer = al.max(bl);
            let end_layer = al.checked_add(ac)?.min(bl.checked_add(bc)?);
            (aspects != 0 && mip < end_mip && layer < end_layer).then(|| NativeRange::Image {
                aspects,
                base_mip: mip,
                mip_count: end_mip - mip,
                base_layer: layer,
                layer_count: end_layer - layer,
            })
        }
        _ => None,
    }
}
/// Slab subtraction preserves exact union and never fills a hole.
pub fn difference(whole: NativeRange, cut: NativeRange) -> SmallVec<[NativeRange; 5]> {
    let mut pieces = SmallVec::new();
    let Some(overlap) = intersection(whole, cut) else {
        pieces.push(whole);
        return pieces;
    };
    match (whole, overlap) {
        (
            NativeRange::Buffer { offset, size },
            NativeRange::Buffer {
                offset: start,
                size: bytes,
            },
        ) => {
            if offset < start {
                pieces.push(NativeRange::Buffer {
                    offset,
                    size: start - offset,
                });
            }
            if start + bytes < offset + size {
                pieces.push(NativeRange::Buffer {
                    offset: start + bytes,
                    size: offset + size - start - bytes,
                });
            }
        }
        (
            NativeRange::Image {
                aspects,
                base_mip,
                mip_count,
                base_layer,
                layer_count,
            },
            NativeRange::Image {
                aspects: mask,
                base_mip: mip,
                mip_count: mips,
                base_layer: layer,
                layer_count: layers,
            },
        ) => {
            let image =
                |aspects, base_mip, mip_count, base_layer, layer_count| NativeRange::Image {
                    aspects,
                    base_mip,
                    mip_count,
                    base_layer,
                    layer_count,
                };
            if aspects & !mask != 0 {
                pieces.push(image(
                    aspects & !mask,
                    base_mip,
                    mip_count,
                    base_layer,
                    layer_count,
                ));
            }
            if base_mip < mip {
                pieces.push(image(
                    mask,
                    base_mip,
                    mip - base_mip,
                    base_layer,
                    layer_count,
                ));
            }
            if mip + mips < base_mip + mip_count {
                pieces.push(image(
                    mask,
                    mip + mips,
                    base_mip + mip_count - mip - mips,
                    base_layer,
                    layer_count,
                ));
            }
            if base_layer < layer {
                pieces.push(image(mask, mip, mips, base_layer, layer - base_layer));
            }
            if layer + layers < base_layer + layer_count {
                pieces.push(image(
                    mask,
                    mip,
                    mips,
                    layer + layers,
                    base_layer + layer_count - layer - layers,
                ));
            }
        }
        _ => unreachable!("intersection kind"),
    }
    pieces
}
fn key(range: NativeRange) -> (u32, u64, u64, u64, u64) {
    match range {
        NativeRange::Buffer { offset, size } => (0, offset, size, 0, 0),
        NativeRange::Image {
            aspects,
            base_mip,
            mip_count,
            base_layer,
            layer_count,
        } => (
            aspects,
            u64::from(base_mip),
            u64::from(base_layer),
            u64::from(mip_count),
            u64::from(layer_count),
        ),
    }
}
fn adjacent(a: NativeRange, b: NativeRange) -> Option<NativeRange> {
    match (a, b) {
        (
            NativeRange::Buffer { offset, size },
            NativeRange::Buffer {
                offset: next,
                size: n,
            },
        ) if offset + size == next => Some(NativeRange::Buffer {
            offset,
            size: size + n,
        }),
        (
            NativeRange::Image {
                aspects: a,
                base_mip: am,
                mip_count: an,
                base_layer: al,
                layer_count: ac,
            },
            NativeRange::Image {
                aspects: b,
                base_mip: bm,
                mip_count: bn,
                base_layer: bl,
                layer_count: bc,
            },
        ) => {
            if a == b && am == bm && an == bn && al + ac == bl {
                Some(NativeRange::Image {
                    aspects: a,
                    base_mip: am,
                    mip_count: an,
                    base_layer: al,
                    layer_count: ac + bc,
                })
            } else if a == b && al == bl && ac == bc && am + an == bm {
                Some(NativeRange::Image {
                    aspects: a,
                    base_mip: am,
                    mip_count: an + bn,
                    base_layer: al,
                    layer_count: ac,
                })
            } else if a & b == 0 && am == bm && an == bn && al == bl && ac == bc {
                Some(NativeRange::Image {
                    aspects: a | b,
                    base_mip: am,
                    mip_count: an,
                    base_layer: al,
                    layer_count: ac,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
fn coalesce(cells: &mut Vec<RangeCell>) {
    loop {
        cells.sort_unstable_by_key(|cell| key(cell.range));
        let before = cells.len();
        let mut written = 0usize;
        for index in 0..before {
            let next = cells[index];
            if written != 0
                && cells[written - 1].state == next.state
                && let Some(range) = adjacent(cells[written - 1].range, next.range)
            {
                cells[written - 1].range = range;
            } else {
                cells[written] = next;
                written += 1;
            }
        }
        cells.truncate(written);
        if before == written {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn q(index: usize) -> QueueId {
        QueueId::new(index).unwrap()
    }
    fn point(queue: usize, value: u64) -> SyncPoint {
        SyncPoint {
            queue: q(queue),
            value,
        }
    }
    fn read() -> AccessScope {
        AccessScope {
            stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_READ,
            write: false,
        }
    }
    fn write() -> AccessScope {
        AccessScope {
            stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_WRITE,
            write: true,
        }
    }
    fn bytes(offset: u64, size: u64) -> NativeRange {
        NativeRange::Buffer { offset, size }
    }
    fn table(domain: NativeRange, state: NativeState) -> RangeStateTable {
        RangeStateTable::new(
            domain,
            ResourceSharing::Exclusive,
            state,
            RangeStateConfig::default(),
            Arc::new(AllocatorStats::default()),
        )
        .unwrap()
    }
    fn access(
        range: NativeRange,
        revision: u64,
        scope: AccessScope,
        layout: vk::ImageLayout,
        queue: usize,
    ) -> StatePatch {
        StatePatch {
            range,
            expected_revision: revision,
            action: PatchAction::Access {
                scope,
                layout,
                queue: q(queue),
                family: 0,
            },
        }
    }
    #[test]
    fn opaque_public_points_preserve_all_older_unordered_queues() {
        let mut table = table(bytes(0, 16), NativeState::default());
        table.record_opaque_point(point(0, 7));
        table.record_opaque_point(point(1, 9));
        let dependencies = table.host_range_snapshot(bytes(0, 16), true).unwrap();
        assert_eq!(dependencies.get(q(0)), 7);
        assert_eq!(dependencies.get(q(1)), 9);
        assert_eq!(table.cells()[0].state.revision, 2);
    }
    #[test]
    fn disjoint_intersections_never_evaluate_negative_extents() {
        assert_eq!(intersection(bytes(0, 4), bytes(8, 4)), None);
        assert_eq!(intersection(bytes(8, 4), bytes(0, 4)), None);
        assert_eq!(intersection(bytes(0, 4), bytes(4, 4)), None);
        let image = |mip, layer| NativeRange::Image {
            aspects: 1,
            base_mip: mip,
            mip_count: 1,
            base_layer: layer,
            layer_count: 1,
        };
        assert_eq!(intersection(image(0, 0), image(2, 0)), None);
        assert_eq!(intersection(image(2, 0), image(0, 0)), None);
        assert_eq!(intersection(image(0, 0), image(0, 2)), None);
        assert_eq!(intersection(image(0, 2), image(0, 0)), None);
        assert_eq!(
            difference(image(0, 0), image(0, 2)).as_slice(),
            &[image(0, 0)]
        );
    }
    #[test]
    fn disjoint_versions_and_late_failure_are_transactional() {
        let mut table = table(bytes(0, 16), NativeState::default());
        table
            .prepare(
                &[access(
                    bytes(0, 8),
                    0,
                    write(),
                    vk::ImageLayout::UNDEFINED,
                    0,
                )],
                q(0),
                0,
            )
            .unwrap();
        assert_eq!(table.cells().len(), 1);
        table.publish(point(0, 1));
        assert_eq!(table.cells().len(), 2);
        assert!(
            table
                .prepare(
                    &[
                        access(bytes(8, 8), 0, write(), vk::ImageLayout::UNDEFINED, 0),
                        access(bytes(0, 8), 0, read(), vk::ImageLayout::UNDEFINED, 0)
                    ],
                    q(0),
                    0
                )
                .is_err()
        );
        assert_eq!(
            table
                .cells()
                .iter()
                .find(|cell| cell.range == bytes(8, 8))
                .unwrap()
                .state
                .revision,
            0
        );
        table
            .prepare(
                &[access(
                    bytes(8, 8),
                    0,
                    write(),
                    vk::ImageLayout::UNDEFINED,
                    0,
                )],
                q(0),
                0,
            )
            .unwrap();
        table.publish(point(0, 2));
        assert_eq!(
            table
                .host_range_snapshot(bytes(0, 8), false)
                .unwrap()
                .get(q(0)),
            1
        );
        assert_eq!(
            table
                .host_range_snapshot(bytes(8, 8), false)
                .unwrap()
                .get(q(0)),
            2
        );
    }
    #[test]
    fn read_layout_conversion_is_a_checkpoint_for_another_queue() {
        let domain = NativeRange::Image {
            aspects: 1,
            base_mip: 0,
            mip_count: 1,
            base_layer: 0,
            layer_count: 1,
        };
        let mut table = table(domain, NativeState::default());
        table
            .prepare(
                &[access(
                    domain,
                    0,
                    read(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    0,
                )],
                q(0),
                0,
            )
            .unwrap();
        table.publish(point(0, 7));
        let state = table.cells()[0].state;
        assert_eq!(state.dependencies(false).get(q(0)), 7);
        assert!(state.source_scope(q(1), false).is_empty());
        table
            .prepare(
                &[access(
                    domain,
                    1,
                    read(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    1,
                )],
                q(1),
                0,
            )
            .unwrap();
        table.publish(point(1, 9));
        assert_eq!(table.cells()[0].state.dependencies(false).get(q(0)), 7);
        assert_eq!(table.cells()[0].state.dependencies(true).get(q(1)), 9);
    }
    #[test]
    fn acquire_checkpoints_a_real_release_and_partial_pair_is_rejected() {
        let range = bytes(0, 16);
        let transfer = RangeTransfer {
            pair: 1,
            source_family: 0,
            destination_family: 1,
            range,
            source: write(),
            destination: read(),
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::UNDEFINED,
        };
        let mut table = table(
            range,
            NativeState {
                owner: RangeOwner::Owned(0),
                ..NativeState::default()
            },
        );
        assert!(
            table
                .prepare(
                    &[StatePatch {
                        range: bytes(0, 8),
                        expected_revision: 0,
                        action: PatchAction::Release(transfer)
                    }],
                    q(0),
                    0
                )
                .is_err()
        );
        table
            .prepare(
                &[StatePatch {
                    range,
                    expected_revision: 0,
                    action: PatchAction::Release(transfer),
                }],
                q(0),
                0,
            )
            .unwrap();
        table.publish(point(0, 2));
        table
            .prepare(
                &[StatePatch {
                    range,
                    expected_revision: 1,
                    action: PatchAction::Acquire {
                        transfer,
                        release_point: point(0, 2),
                    },
                }],
                q(1),
                1,
            )
            .unwrap();
        table.publish(point(1, 3));
        assert_eq!(table.cells()[0].state.dependencies(false).get(q(1)), 3);
        assert!(!table.cells()[0].state.writer.combined().write);
    }
    #[test]
    fn host_write_advances_only_its_range_without_a_gpu_point() {
        let mut table = table(bytes(0, 16), NativeState::default());
        table.prepare_host(bytes(4, 4), true).unwrap();
        table.publish_host();
        let state = table
            .cells()
            .iter()
            .find(|cell| cell.range == bytes(4, 4))
            .unwrap()
            .state;
        assert_eq!(state.revision, 1);
        assert!(state.writer.points.is_empty());
        assert_eq!(
            state.source_scope(q(1), false).access,
            vk::AccessFlags::HOST_WRITE
        );
        assert!(
            table
                .prepare(
                    &[access(
                        bytes(4, 4),
                        0,
                        read(),
                        vk::ImageLayout::UNDEFINED,
                        0
                    )],
                    q(0),
                    0
                )
                .is_err()
        );
    }
    #[test]
    fn split_limit_failure_keeps_committed_state_and_credit() {
        let stats = Arc::new(AllocatorStats::default());
        let mut table = RangeStateTable::new(
            bytes(0, 16),
            ResourceSharing::Exclusive,
            NativeState::default(),
            RangeStateConfig {
                max_cells: 2,
                max_cpu_bytes: 1 << 20,
            },
            stats.clone(),
        )
        .unwrap();
        assert!(matches!(
            table.prepare(
                &[access(
                    bytes(4, 4),
                    0,
                    write(),
                    vk::ImageLayout::UNDEFINED,
                    0
                )],
                q(0),
                0
            ),
            Err(Error::Timeout)
        ));
        assert_eq!(
            table.cells(),
            &[RangeCell {
                range: bytes(0, 16),
                state: NativeState::default()
            }]
        );
        drop(table);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
