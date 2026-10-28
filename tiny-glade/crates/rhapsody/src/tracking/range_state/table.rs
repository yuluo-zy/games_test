//! The range-state table that tracks ownership and state across a resource.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::accounting::AllocatorStats;
use crate::native::access_table::AccessScope;
use crate::sync::QueueId;
use crate::sync::StagedDependencies;
use crate::sync::SyncPoint;
use crate::sync::SyncSet;
use crate::tracking::ResourceSharing;
use std::sync::Arc;
use super::frontier::Frontier;
use super::native::NativeRange;
use super::native::NativeState;
use super::ops::coalesce;
use super::ops::contains;
use super::ops::difference;
use super::ops::intersection;
use super::ops::validate_range;
use super::owner::RangeOwner;
use super::patch::PatchAction;
use super::patch::PreparedCell;
use super::patch::RangeCell;
use super::patch::RangeStateConfig;
use super::patch::StatePatch;
use super::patch::apply_preview;
use super::storage::Storage;

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
    prepared_family: Option<u32>,
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
            return Err(Error::exhausted("native range state capacity exceeded"));
        }
        let mut result = Self {
            domain,
            sharing,
            cells,
            scratch,
            config,
            budget,
            prepared_queue: None,
            prepared_family: None,
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
    /// The resource's contents are discarded and all of its earlier GPU work
    /// is known to be complete (caller checked the physical last use). Every
    /// range returns to the unclaimed, `UNDEFINED`, dependency-free state:
    /// a pending ownership release or foreign owner is dropped without any
    /// acquire, and nothing is left to wait on. Revisions keep increasing so
    /// stale recorded patches still fail their preconditions.
    pub fn discard_idle_contents(&mut self) {
        self.cancel_prepared();
        let revision = self
            .cells
            .values
            .iter()
            .map(|cell| cell.state.revision)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        self.cells.values.clear();
        self.cells.values.push(RangeCell {
            range: self.domain,
            state: NativeState {
                revision,
                ..NativeState::default()
            },
        });
    }
    pub fn cancel_prepared(&mut self) {
        self.prepared_queue = None;
        self.prepared_family = None;
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
        self.apply_prepared_patches(patches, queue, family)?;
        self.admit_prepared(queue, family)
    }
    fn apply_prepared_patches(
        &mut self,
        patches: &[StatePatch],
        queue: QueueId,
        family: u32,
    ) -> Result<()> {
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
                // Test overlap on the borrowed range; copying the whole cell
                // (two frontiers) for every non-overlapping neighbour dominated
                // the replay.
                let Some(overlap) = intersection(self.scratch.values[index].cell.range, patch.range)
                else {
                    visited += 1;
                    index += 1;
                    continue;
                };
                visited += 1;
                let current = &self.scratch.values[index];
                if current.cell.state.revision != patch.expected_revision
                    && !(patch.expected_revision == u64::MAX
                        && matches!(patch.action, PatchAction::Unknown { .. }))
                {
                    return Err(Error::invalid_usage(format!(
                        "native range revision changed before submission: domain={:?}, patch={:?}, current={:?}",
                        self.domain, patch, current.cell,
                    )));
                }
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
                let outside = difference(current.cell.range, overlap);
                // Only a split needs the pre-patch cell again; otherwise the
                // cell is updated in place with no copy at all.
                let original = (!outside.is_empty()).then_some(*current);
                let changed = &mut self.scratch.values[index];
                changed.cell.range = overlap;
                apply_preview(changed, patch.action, self.sharing, queue, family)?;
                changed.cell.state.revision = changed
                    .cell
                    .state
                    .revision
                    .checked_add(1)
                    .filter(|revision| *revision != u64::MAX)
                    .ok_or_else(|| Error::unsupported("native range revision exhausted"))?;
                let wanted = self
                    .scratch
                    .values
                    .len()
                    .checked_add(outside.len())
                    .ok_or(Error::OutOfHostMemory)?;
                self.scratch
                    .grow(wanted, self.cells.bytes(), self.config, &self.budget)?;
                // Outside pieces keep the pre-patch state and are not revisited.
                if let Some(original) = original {
                    self.scratch
                        .values
                        .extend(outside.into_iter().map(|range| PreparedCell {
                            cell: RangeCell {
                                range,
                                state: original.cell.state,
                            },
                            ..original
                        }));
                }
                index += 1;
            }
        }
        Ok(())
    }
    // 每个原 prepare 边界仍准入 committed 容量，保持预算失败的发生顺序。
    fn admit_prepared(&mut self, queue: QueueId, family: u32) -> Result<()> {
        self.cells.grow(
            self.scratch.values.len(),
            self.scratch.bytes(),
            self.config,
            &self.budget,
        )?;
        self.prepared_queue = Some(queue);
        self.prepared_family = Some(family);
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
                return Err(Error::invalid_usage("host access requires completed queue ownership acquisition"));
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
        self.prepared_family = None;
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
