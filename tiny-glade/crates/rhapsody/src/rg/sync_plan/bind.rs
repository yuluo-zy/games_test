use super::*;
use crate::{
    raw::allocator::{AllocatorStats, CpuMemoryReservation},
    resource::range_state::{self, Frontier, RangeOwner, RangeStateConfig, Storage},
    sync::MAX_QUEUES,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct SynchronizationConfig {
    pub max_cells: usize,
    pub max_rows: usize,
    pub max_batches: usize,
    pub max_cpu_bytes: usize,
}
impl Default for SynchronizationConfig {
    fn default() -> Self {
        Self {
            max_cells: 65_536,
            max_rows: 262_144,
            max_batches: 4096,
            max_cpu_bytes: 64 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PendingFrontier {
    scope: [AccessScope; MAX_QUEUES],
    host: AccessScope,
    points: [Option<DependencyRef>; MAX_QUEUES],
}
impl PendingFrontier {
    const EMPTY: Self = Self {
        scope: [AccessScope::EMPTY; MAX_QUEUES],
        host: AccessScope::EMPTY,
        points: [None; MAX_QUEUES],
    };
    fn from_native(native: Frontier) -> Self {
        let mut result = Self {
            scope: native.scopes,
            host: native.host,
            ..Self::EMPTY
        };
        for point in native.points.points() {
            result.points[point.queue.index()] = Some(DependencyRef::Point(point));
        }
        result
    }
    fn source(self, queue: QueueId) -> AccessScope {
        self.scope[queue.index()].union(self.host)
    }
    fn add(&mut self, queue: QueueId, scope: AccessScope, batch: usize) {
        self.scope[queue.index()] = self.scope[queue.index()].union(scope);
        self.points[queue.index()] = Some(DependencyRef::Batch(batch));
    }
    /// Union of two frontiers. Per queue, the dependency that executes later
    /// covers the earlier one (`later` decides using the actual binding order).
    fn merge(
        &mut self,
        other: Self,
        later: impl Fn(DependencyRef, DependencyRef) -> DependencyRef,
    ) {
        for queue in 0..MAX_QUEUES {
            self.scope[queue] = self.scope[queue].union(other.scope[queue]);
            self.points[queue] = match (self.points[queue], other.points[queue]) {
                (Some(a), Some(b)) => Some(later(a, b)),
                (a, b) => a.or(b),
            };
        }
        self.host = self.host.union(other.host);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingOwner {
    Unclaimed,
    Owned(u32),
    Released {
        transfer: RangeTransfer,
        release: DependencyRef,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ShadowCell {
    resource: u32,
    range: NativeRange,
    layout: vk::ImageLayout,
    layout_known: bool,
    writer: PendingFrontier,
    readers: PendingFrontier,
    owner: PendingOwner,
    revision: u64,
    /// Not yet touched by this binding; alias edges apply on the first touch.
    fresh: bool,
}
impl ShadowCell {
    fn new(resource: u32, cell: RangeCell) -> Self {
        let state = cell.state;
        Self {
            resource,
            range: cell.range,
            layout: state.layout,
            layout_known: state.layout_known,
            writer: PendingFrontier::from_native(state.writer),
            readers: PendingFrontier::from_native(state.readers),
            owner: match state.owner {
                RangeOwner::Unclaimed => PendingOwner::Unclaimed,
                RangeOwner::Owned(family) => PendingOwner::Owned(family),
                RangeOwner::Released { transfer, point } => PendingOwner::Released {
                    transfer,
                    release: DependencyRef::Point(point),
                },
            },
            revision: state.revision,
            fresh: true,
        }
    }
    fn increment(&mut self) -> Result<()> {
        self.revision = self
            .revision
            .checked_add(1)
            .filter(|revision| *revision != u64::MAX)
            .ok_or_else(|| Error::unsupported("native range revision exhausted"))?;
        Ok(())
    }
}
/// CPU-only binding scratch and the read-only native projection share one
/// bounded owner. GPU completion never extends this CPU lease: commands copy
/// only their admitted StatePatches and retain the physical raw resources.
/// An outstanding diagnostic borrow prevents a caller from beginning a frame.
pub struct SynchronizationPlanner {
    bound: BoundSynchronization,
    shadow: Storage<ShadowCell>,
    budget: Arc<AllocatorStats>,
    config: SynchronizationConfig,
    next_pair: u64,
    owner_bytes: usize,
    _owner_memory: Option<CpuMemoryReservation>,
}
impl SynchronizationPlanner {
    pub fn new(budget: Arc<AllocatorStats>, config: SynchronizationConfig) -> Result<Self> {
        if config.max_cells == 0
            || config.max_cells > 65_536
            || config.max_rows == 0
            || config.max_batches == 0
            || config.max_batches > 65_536
            || config.max_cpu_bytes == 0
        {
            return Err(Error::invalid_usage(
                "invalid synchronization storage bounds",
            ));
        }
        Ok(Self {
            bound: BoundSynchronization {
                initial: Storage::new(0, &budget)?,
                aliases: Storage::new(0, &budget)?,
                batches: Storage::new(0, &budget)?,
                order: Storage::new(0, &budget)?,
                transitions: Storage::new(0, &budget)?,
                patches: Storage::new(0, &budget)?,
                waits: Storage::new(0, &budget)?,
            },
            shadow: Storage::new(0, &budget)?,
            budget,
            config,
            next_pair: 1,
            owner_bytes: 0,
            _owner_memory: None,
        })
    }
    /// Production shared owner. Its actual Arc envelope is charged separately
    /// from the reusable arrays and follows the last CPU recording owner.
    pub fn new_shared(
        budget: Arc<AllocatorStats>,
        config: SynchronizationConfig,
    ) -> Result<Arc<Self>> {
        let bytes = crate::raw::allocator::shared_cpu_bytes::<Self>()?;
        if bytes > config.max_cpu_bytes as u64 {
            return Err(Error::Timeout);
        }
        let memory = budget.reserve_cpu_bytes(bytes, crate::device::BudgetCategory::Live)?;
        let mut planner = Self::new(budget, config)?;
        planner.owner_bytes = bytes as usize;
        planner._owner_memory = Some(memory);
        Ok(Arc::new(planner))
    }
    pub fn bound(&self) -> &BoundSynchronization {
        &self.bound
    }
    pub fn known_cpu_bytes(&self) -> usize {
        self.owner_bytes + self.bound.bytes() + self.shadow.bytes()
    }
    pub fn begin(&mut self) {
        self.bound.initial.values.clear();
        self.bound.aliases.values.clear();
        self.bound.batches.values.clear();
        self.bound.order.values.clear();
        self.bound.transitions.values.clear();
        self.bound.patches.values.clear();
        self.bound.waits.values.clear();
        self.shadow.values.clear();
    }
    pub fn add_initial(&mut self, resource: u32, cells: &[RangeCell]) -> Result<()> {
        for &cell in cells {
            self.push_initial(InitialState { resource, cell })?;
            self.push_shadow(ShadowCell::new(resource, cell))?;
        }
        Ok(())
    }
    /// Declare that `next` reuses the backing memory of `previous` in this
    /// frame. Call after [`Self::add_initial`] and before [`Self::bind`].
    pub fn add_alias(&mut self, previous: u32, next: u32) -> Result<()> {
        if previous == next {
            return Err(Error::invalid_usage("a resource cannot alias itself"));
        }
        self.push_alias(AliasEdge { previous, next })
    }
    pub fn bind(
        &mut self,
        program: &RangeProgram,
        mut family_domain: impl FnMut(u32) -> Result<QueueDomain>,
    ) -> Result<()> {
        for initial in &self.bound.initial.values {
            let domain = program
                .resources
                .get(initial.resource as usize)
                .ok_or_else(|| Error::invalid_usage("native initial resource is absent"))?;
            if !range_state::contains(domain.range, initial.cell.range) {
                return Err(Error::invalid_usage(
                    "native initial range exceeds declaration",
                ));
            }
        }
        if self.bound.aliases.values.iter().any(|edge| {
            edge.previous as usize >= program.resources.len()
                || edge.next as usize >= program.resources.len()
        }) {
            return Err(Error::invalid_usage("native alias edge names an absent resource"));
        }
        for batch in &program.batches {
            self.push_batch(BoundBatch {
                domain: batch.domain,
                kind: BoundBatchKind::Graph {
                    original: self.bound.batches.values.len(),
                    passes: batch.passes.clone(),
                },
            })?;
        }
        for (batch, entry) in program.batches.iter().enumerate() {
            for pass in entry.passes.clone() {
                let pass = program
                    .passes
                    .get(pass)
                    .ok_or_else(|| Error::invalid_usage("native scheduled pass is absent"))?;
                for access in &program.uses[pass.uses.clone()] {
                    self.bind_use(
                        *access,
                        batch,
                        BarrierPhase::BeforePass(pass.pass),
                        program,
                        &mut family_domain,
                    )?;
                }
            }
            if batch + 1 == program.batches.len() {
                for access in &program.exports {
                    self.bind_use(
                        *access,
                        batch,
                        BarrierPhase::Export,
                        program,
                        &mut family_domain,
                    )?;
                }
            }
            self.push_order(batch)?;
        }
        for &(from, to) in &program.dependencies {
            self.static_dependency(program, from, to)?;
        }
        Ok(())
    }
    /// A scheduled batch dependency. Resource hazards already produced exact
    /// waits; an ordering-only edge waits at the stages where the destination
    /// batch first touches resources the source batch also used (falling back
    /// to every stage the destination batch uses).
    fn static_dependency(&mut self, program: &RangeProgram, from: usize, to: usize) -> Result<()> {
        let (Some(source), Some(target)) = (program.batches.get(from), program.batches.get(to))
        else {
            return Err(Error::invalid_usage("native batch dependency destination absent"));
        };
        let domain = self.bound.batches.values[to].domain;
        let dependency = DependencyRef::Batch(from);
        if source.domain.queue_index == domain.queue_index
            || self
                .bound
                .waits
                .values
                .iter()
                .any(|wait| wait.batch == to && wait.dependency == dependency)
        {
            return Ok(());
        }
        let uses = |batch: &ProgramBatch| {
            program.passes[batch.passes.clone()]
                .iter()
                .flat_map(|pass| &program.uses[pass.uses.clone()])
        };
        let mut shared = vk::PipelineStageFlags::empty();
        let mut all = vk::PipelineStageFlags::empty();
        for usage in uses(target) {
            all |= usage.scope.stages;
            if uses(source).any(|earlier| earlier.resource == usage.resource) {
                shared |= usage.scope.stages;
            }
        }
        let stages = if !shared.is_empty() {
            shared
        } else if !all.is_empty() {
            all
        } else {
            vk::PipelineStageFlags::ALL_COMMANDS
        };
        self.wait(to, dependency, stages, domain)
    }
    /// The dependency that executes later on one queue. Bound batches execute
    /// in `order`; a batch not yet ordered is the one being bound now.
    fn later(order: &[usize], a: DependencyRef, b: DependencyRef) -> DependencyRef {
        let position = |batch: usize| {
            order
                .iter()
                .position(|&ordered| ordered == batch)
                .unwrap_or(usize::MAX)
        };
        match (a, b) {
            (DependencyRef::Point(x), DependencyRef::Point(y)) => {
                if x.value >= y.value { a } else { b }
            }
            (DependencyRef::Batch(_), DependencyRef::Point(_)) => a,
            (DependencyRef::Point(_), DependencyRef::Batch(_)) => b,
            (DependencyRef::Batch(x), DependencyRef::Batch(y)) => {
                if position(x) >= position(y) { a } else { b }
            }
        }
    }
    /// First touch of an alias member: its contents are discarded and every
    /// access to the members it reuses becomes its synthetic previous writer.
    fn discard_alias_contents(&self, mut current: ShadowCell) -> ShadowCell {
        let order = &self.bound.order.values;
        let later = |a, b| Self::later(order, a, b);
        let mut writer = current.writer;
        writer.merge(current.readers, later);
        for edge in self
            .bound
            .aliases
            .values
            .iter()
            .filter(|edge| edge.next == current.resource)
        {
            for cell in self
                .shadow
                .values
                .iter()
                .filter(|cell| cell.resource == edge.previous)
            {
                writer.merge(cell.writer, later);
                writer.merge(cell.readers, later);
            }
        }
        current.writer = writer;
        current.readers = PendingFrontier::EMPTY;
        current.layout = vk::ImageLayout::UNDEFINED;
        current.layout_known = true;
        current
    }
    fn bind_use(
        &mut self,
        access: RangeUse,
        batch: usize,
        phase: BarrierPhase,
        program: &RangeProgram,
        family_domain: &mut impl FnMut(u32) -> Result<QueueDomain>,
    ) -> Result<()> {
        let domain = self.bound.batches.values[batch].domain;
        let queue = queue_id(domain)?;
        let sharing = program.resources[access.resource as usize].sharing;
        if !sharing.allows_family(domain.family) {
            return Err(Error::invalid_usage(
                "native use family absent from sharing list",
            ));
        }
        let before = self.shadow.values.len();
        let mut covered = false;
        for index in 0..before {
            let mut current = self.shadow.values[index];
            if current.resource != access.resource {
                continue;
            }
            let Some(overlap) = range_state::intersection(current.range, access.range) else {
                continue;
            };
            covered = true;
            if !current.layout_known && matches!(current.range, NativeRange::Image { .. }) {
                return Err(Error::unsupported(
                    "opaque image use requires an explicit external native-state declaration",
                ));
            }
            // A prior failed acquire is completed for exactly its original
            // native pair before any splitting of that released range.
            if let PendingOwner::Released { transfer, release } = current.owner {
                if current.range != transfer.range {
                    return Err(Error::invalid_usage(
                        "released range lost its exact native pair coverage",
                    ));
                }
                if domain.family == transfer.destination_family {
                    current = self.acquire(current, transfer, release, batch, phase)?;
                } else {
                    let target = family_domain(transfer.destination_family)?;
                    let auxiliary = self.push_batch(BoundBatch {
                        domain: target,
                        kind: BoundBatchKind::OwnershipPrelude,
                    })?;
                    current =
                        self.acquire(current, transfer, release, auxiliary, BarrierPhase::Initial)?;
                    self.push_order(auxiliary)?;
                }
            }
            let outside = range_state::difference(current.range, overlap);
            for range in outside {
                self.push_shadow(ShadowCell { range, ..current })?;
            }
            current.range = overlap;
            if !sharing.is_concurrent() {
                match current.owner {
                    PendingOwner::Unclaimed => current.owner = PendingOwner::Owned(domain.family),
                    PendingOwner::Owned(owner) if owner == domain.family => {}
                    PendingOwner::Owned(owner) => {
                        let source = family_domain(owner)?;
                        let auxiliary = self.push_batch(BoundBatch {
                            domain: source,
                            kind: BoundBatchKind::OwnershipPrelude,
                        })?;
                        let transfer = RangeTransfer {
                            pair: self.pair()?,
                            source_family: owner,
                            destination_family: domain.family,
                            range: overlap,
                            source: current
                                .writer
                                .source(queue_id(source)?)
                                .union(current.readers.source(queue_id(source)?)),
                            destination: access.scope,
                            old_layout: current.layout,
                            new_layout: access.layout,
                        };
                        self.wait_frontier(
                            current.writer,
                            auxiliary,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            source,
                        )?;
                        self.wait_frontier(
                            current.readers,
                            auxiliary,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            source,
                        )?;
                        self.push_transition(BoundTransition {
                            batch: auxiliary,
                            phase: BarrierPhase::Initial,
                            resource: access.resource,
                            range: overlap,
                            source: transfer.source,
                            destination: AccessScope::EMPTY,
                            old_layout: transfer.old_layout,
                            new_layout: transfer.new_layout,
                            source_family: owner,
                            destination_family: domain.family,
                            kind: BarrierKind::Release {
                                pair: transfer.pair,
                            },
                        })?;
                        self.push_patch(BoundPatch {
                            batch: auxiliary,
                            phase: BarrierPhase::Initial,
                            resource: access.resource,
                            range: overlap,
                            expected_revision: current.revision,
                            action: BoundPatchAction::Release(transfer),
                        })?;
                        current.increment()?;
                        current.owner = PendingOwner::Released {
                            transfer,
                            release: DependencyRef::Batch(auxiliary),
                        };
                        current.layout = transfer.new_layout;
                        self.push_order(auxiliary)?;
                        current = self.acquire(
                            current,
                            transfer,
                            DependencyRef::Batch(auxiliary),
                            batch,
                            phase,
                        )?;
                    }
                    PendingOwner::Released { .. } => {
                        unreachable!("released ranges recovered before split")
                    }
                }
            }
            // Ownership recovery above keeps the stale physical layout; the
            // first access of an alias member then discards the contents.
            if current.fresh {
                current.fresh = false;
                if self
                    .bound
                    .aliases
                    .values
                    .iter()
                    .any(|edge| edge.next == access.resource)
                {
                    current = self.discard_alias_contents(current);
                }
            }
            let changes_layout = current.layout != access.layout;
            let changes = access.scope.write || changes_layout;
            self.wait_frontier(current.writer, batch, access.scope.stages, domain)?;
            if changes {
                self.wait_frontier(current.readers, batch, access.scope.stages, domain)?;
            }
            let source = current.writer.source(queue).union(if changes {
                current.readers.source(queue)
            } else {
                AccessScope::EMPTY
            });
            if !source.is_empty() || changes_layout {
                self.push_transition(BoundTransition {
                    batch,
                    phase,
                    resource: access.resource,
                    range: overlap,
                    source,
                    destination: access.scope,
                    old_layout: current.layout,
                    new_layout: access.layout,
                    source_family: vk::QUEUE_FAMILY_IGNORED,
                    destination_family: vk::QUEUE_FAMILY_IGNORED,
                    kind: BarrierKind::Memory,
                })?;
            }
            self.push_patch(BoundPatch {
                batch,
                phase,
                resource: access.resource,
                range: overlap,
                expected_revision: current.revision,
                action: BoundPatchAction::Access {
                    scope: access.scope,
                    layout: access.layout,
                    queue,
                    family: domain.family,
                },
            })?;
            current.increment()?;
            if changes {
                current.writer = PendingFrontier::EMPTY;
                current.writer.add(queue, access.scope, batch);
                current.readers = PendingFrontier::EMPTY;
            } else {
                current.readers.add(queue, access.scope, batch);
            }
            current.layout = access.layout;
            current.layout_known = true;
            self.shadow.values[index] = current;
        }
        if !covered {
            return Err(Error::invalid_usage(
                "native resource has no physical initial state",
            ));
        }
        Ok(())
    }
    fn acquire(
        &mut self,
        mut current: ShadowCell,
        transfer: RangeTransfer,
        release: DependencyRef,
        batch: usize,
        phase: BarrierPhase,
    ) -> Result<ShadowCell> {
        let domain = self.bound.batches.values[batch].domain;
        if domain.family != transfer.destination_family {
            return Err(Error::invalid_usage(
                "ownership acquire queue family mismatch",
            ));
        }
        self.wait(batch, release, transfer.destination.stages, domain)?;
        self.push_transition(BoundTransition {
            batch,
            phase,
            resource: current.resource,
            range: transfer.range,
            source: AccessScope::EMPTY,
            destination: transfer.destination,
            old_layout: transfer.old_layout,
            new_layout: transfer.new_layout,
            source_family: transfer.source_family,
            destination_family: transfer.destination_family,
            kind: BarrierKind::Acquire {
                pair: transfer.pair,
            },
        })?;
        self.push_patch(BoundPatch {
            batch,
            phase,
            resource: current.resource,
            range: transfer.range,
            expected_revision: current.revision,
            action: BoundPatchAction::Acquire { transfer, release },
        })?;
        current.increment()?;
        current.owner = PendingOwner::Owned(domain.family);
        current.layout = transfer.new_layout;
        current.layout_known = true;
        current.writer = PendingFrontier::EMPTY;
        current.readers = PendingFrontier::EMPTY;
        current
            .writer
            .add(queue_id(domain)?, transfer.destination, batch);
        Ok(current)
    }
    fn pair(&mut self) -> Result<u64> {
        let pair = self.next_pair;
        self.next_pair = self
            .next_pair
            .checked_add(1)
            .ok_or_else(|| Error::unsupported("ownership pair identity exhausted"))?;
        Ok(pair)
    }
    fn wait_frontier(
        &mut self,
        frontier: PendingFrontier,
        batch: usize,
        stages: vk::PipelineStageFlags,
        domain: QueueDomain,
    ) -> Result<()> {
        for dependency in frontier.points.into_iter().flatten() {
            self.wait(batch, dependency, stages, domain)?;
        }
        Ok(())
    }
    fn wait(
        &mut self,
        batch: usize,
        dependency: DependencyRef,
        stages: vk::PipelineStageFlags,
        domain: QueueDomain,
    ) -> Result<()> {
        let source = match dependency {
            DependencyRef::Point(point) => point.queue.index(),
            DependencyRef::Batch(source) => {
                if source == batch {
                    return Ok(());
                }
                self.bound
                    .batches
                    .values
                    .get(source)
                    .ok_or_else(|| Error::invalid_usage("native wait refers to an absent batch"))?
                    .domain
                    .queue_index as usize
            }
        };
        if source == domain.queue_index as usize {
            return Ok(());
        }
        if let Some(wait) = self
            .bound
            .waits
            .values
            .iter_mut()
            .find(|wait| wait.batch == batch && wait.dependency == dependency)
        {
            wait.stages |= stages;
            return Ok(());
        }
        self.push_wait(BoundWait {
            batch,
            dependency,
            stages: if stages.is_empty() {
                vk::PipelineStageFlags::ALL_COMMANDS
            } else {
                stages
            },
        })
    }
}
fn queue_id(domain: QueueDomain) -> Result<QueueId> {
    QueueId::new(domain.queue_index as usize)
        .ok_or_else(|| Error::invalid_usage("native queue index exceeds timeline capacity"))
}
impl BoundSynchronization {
    fn bytes(&self) -> usize {
        self.initial.bytes()
            + self.aliases.bytes()
            + self.batches.bytes()
            + self.order.bytes()
            + self.transitions.bytes()
            + self.patches.bytes()
            + self.waits.bytes()
    }
}
macro_rules! push {
    ($name:ident,$field:ident,$ty:ty,$limit:ident) => {
        fn $name(&mut self, value: $ty) -> Result<()> {
            let total = self.known_cpu_bytes();
            let target = &mut self.bound.$field;
            let config = RangeStateConfig {
                max_cells: self.config.$limit,
                max_cpu_bytes: self.config.max_cpu_bytes,
            };
            let needed = target
                .values
                .len()
                .checked_add(1)
                .ok_or(Error::OutOfHostMemory)?;
            let capacity = needed
                .max(target.values.capacity().saturating_mul(2))
                .min(config.max_cells);
            if needed > config.max_cells {
                return Err(Error::Timeout);
            }
            if needed > target.values.capacity() {
                target.grow(capacity, total - target.bytes(), config, &self.budget)?;
            }
            target.values.push(value);
            Ok(())
        }
    };
}
impl SynchronizationPlanner {
    push!(push_initial, initial, InitialState, max_cells);
    push!(push_alias, aliases, AliasEdge, max_cells);
    push!(push_batch_row, batches, BoundBatch, max_batches);
    fn push_batch(&mut self, value: BoundBatch) -> Result<usize> {
        let index = self.bound.batches.values.len();
        self.push_batch_row(value)?;
        Ok(index)
    }
    push!(push_order, order, usize, max_batches);
    push!(push_transition, transitions, BoundTransition, max_rows);
    push!(push_patch, patches, BoundPatch, max_rows);
    push!(push_wait, waits, BoundWait, max_rows);
    fn push_shadow(&mut self, value: ShadowCell) -> Result<()> {
        let total = self.known_cpu_bytes();
        let target = &mut self.shadow;
        let config = RangeStateConfig {
            max_cells: self.config.max_cells,
            max_cpu_bytes: self.config.max_cpu_bytes,
        };
        let needed = target
            .values
            .len()
            .checked_add(1)
            .ok_or(Error::OutOfHostMemory)?;
        if needed > config.max_cells {
            return Err(Error::Timeout);
        }
        let capacity = needed
            .max(target.values.capacity().saturating_mul(2))
            .min(config.max_cells);
        if needed > target.values.capacity() {
            target.grow(capacity, total - target.bytes(), config, &self.budget)?;
        }
        target.values.push(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn buffer(offset: u64, size: u64) -> NativeRange {
        NativeRange::Buffer { offset, size }
    }
    fn queue(index: u32, family: u32) -> QueueDomain {
        QueueDomain {
            queue_index: index,
            family,
        }
    }
    fn usage(range: NativeRange, write: bool) -> RangeUse {
        RangeUse {
            resource: 0,
            range,
            scope: AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: if write {
                    vk::AccessFlags::TRANSFER_WRITE
                } else {
                    vk::AccessFlags::TRANSFER_READ
                },
                write,
            },
            layout: vk::ImageLayout::UNDEFINED,
            access_code: if write { 0x34 } else { 0x25 },
        }
    }
    fn program(domain: NativeRange, uses: Vec<RangeUse>, domains: &[QueueDomain]) -> RangeProgram {
        let count = uses.len();
        RangeProgram {
            resources: vec![ResourceDomain {
                range: domain,
                sharing: ResourceSharing::Exclusive,
            }],
            uses,
            passes: (0..count)
                .map(|pass| ProgramPass {
                    pass,
                    uses: pass..pass + 1,
                })
                .collect(),
            batches: domains
                .iter()
                .enumerate()
                .map(|(pass, &domain)| ProgramBatch {
                    domain,
                    passes: pass..pass + 1,
                })
                .collect(),
            ..Default::default()
        }
    }
    #[test]
    fn disjoint_unclaimed_ranges_need_no_cross_queue_or_ownership_wait() {
        let stats = Arc::new(AllocatorStats::default());
        let program = program(
            buffer(0, 16),
            vec![usage(buffer(0, 8), true), usage(buffer(8, 8), true)],
            &[queue(0, 0), queue(1, 1)],
        );
        let mut planner = SynchronizationPlanner::new(stats, Default::default()).unwrap();
        planner
            .add_initial(
                0,
                &[RangeCell {
                    range: buffer(0, 16),
                    state: NativeState::default(),
                }],
            )
            .unwrap();
        planner
            .bind(&program, |family| Ok(queue(family, family)))
            .unwrap();
        assert_eq!(planner.bound().execution_order(), &[0, 1]);
        assert!(planner.bound().waits().is_empty());
        assert!(planner.bound().transitions().is_empty());
        assert_eq!(planner.bound().patches()[0].range, buffer(0, 8));
        assert_eq!(planner.bound().patches()[1].range, buffer(8, 8));
    }
    #[test]
    fn exclusive_transfer_is_exact_and_acquire_refers_to_actual_release_batch() {
        let program = program(
            buffer(0, 16),
            vec![usage(buffer(8, 8), true)],
            &[queue(1, 1)],
        );
        let mut planner =
            SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
                .unwrap();
        planner
            .add_initial(
                0,
                &[RangeCell {
                    range: buffer(0, 16),
                    state: NativeState {
                        owner: RangeOwner::Owned(0),
                        ..NativeState::default()
                    },
                }],
            )
            .unwrap();
        planner
            .bind(&program, |family| Ok(queue(family, family)))
            .unwrap();
        assert_eq!(planner.bound().execution_order(), &[1, 0]);
        let transitions = planner.bound().transitions();
        assert_eq!(transitions[0].range, buffer(8, 8));
        assert_eq!(transitions[1].range, buffer(8, 8));
        assert!(matches!(
            planner.bound().patches()[1].action,
            BoundPatchAction::Acquire {
                release: DependencyRef::Batch(1),
                ..
            }
        ));
        assert!(planner.bound().patches()[1].resolve(|_| None).is_err());
        let real = SyncPoint {
            queue: QueueId::new(0).unwrap(),
            value: 17,
        };
        assert!(
            matches!(planner.bound().patches()[1].resolve(|_|Some(real)).unwrap().action,PatchAction::Acquire {release_point,..} if release_point==real)
        );
    }
    #[test]
    fn released_recovery_first_acquires_the_original_destination_then_moves_again() {
        let range = buffer(0, 16);
        let transfer = RangeTransfer {
            pair: 31,
            source_family: 0,
            destination_family: 1,
            range,
            source: usage(range, true).scope,
            destination: usage(range, false).scope,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::UNDEFINED,
        };
        let original = SyncPoint {
            queue: QueueId::new(0).unwrap(),
            value: 9,
        };
        let program = program(range, vec![usage(buffer(8, 8), true)], &[queue(2, 2)]);
        let mut planner =
            SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
                .unwrap();
        planner
            .add_initial(
                0,
                &[RangeCell {
                    range,
                    state: NativeState {
                        owner: RangeOwner::Released {
                            transfer,
                            point: original,
                        },
                        revision: 4,
                        ..NativeState::default()
                    },
                }],
            )
            .unwrap();
        planner
            .bind(&program, |family| Ok(queue(family, family)))
            .unwrap();
        assert_eq!(planner.bound().execution_order(), &[1, 2, 0]);
        let first = planner.bound().patches()[0];
        assert_eq!(first.range, range);
        assert_eq!(first.expected_revision, 4);
        assert!(
            matches!(first.action,BoundPatchAction::Acquire {release:DependencyRef::Point(point),..} if point==original)
        );
        assert!(
            matches!(planner.bound().patches()[1].action,BoundPatchAction::Release(pair) if pair.range==buffer(8,8)&&pair.source_family==1&&pair.destination_family==2)
        );
    }
    #[test]
    fn shared_planner_envelope_keeps_credit_until_its_last_cpu_owner() {
        let stats = Arc::new(AllocatorStats::default());
        let planner =
            SynchronizationPlanner::new_shared(stats.clone(), Default::default()).unwrap();
        let expected = crate::raw::allocator::shared_cpu_bytes::<SynchronizationPlanner>().unwrap();
        assert_eq!(stats.cpu_memory_usage().total(), expected);
        let observer = planner.clone();
        drop(planner);
        assert_eq!(stats.cpu_memory_usage().total(), expected);
        drop(observer);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        assert!(matches!(
            SynchronizationPlanner::new_shared(
                stats.clone(),
                SynchronizationConfig {
                    max_cpu_bytes: 1,
                    ..Default::default()
                }
            ),
            Err(Error::Timeout)
        ));
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn warmed_binding_reuses_all_bounded_capacity() {
        let stats = Arc::new(AllocatorStats::default());
        let program = program(
            buffer(0, 16),
            vec![usage(buffer(0, 16), true), usage(buffer(0, 16), false)],
            &[queue(0, 0), queue(0, 0)],
        );
        let mut planner = SynchronizationPlanner::new(stats.clone(), Default::default()).unwrap();
        let initial = [RangeCell {
            range: buffer(0, 16),
            state: NativeState::default(),
        }];
        let mut bytes = 0;
        for frame in 0..1000 {
            planner.begin();
            planner.add_initial(0, &initial).unwrap();
            planner
                .bind(&program, |family| Ok(queue(0, family)))
                .unwrap();
            if frame == 0 {
                bytes = planner.known_cpu_bytes();
            } else {
                assert_eq!(planner.known_cpu_bytes(), bytes);
            }
        }
        drop(planner);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
