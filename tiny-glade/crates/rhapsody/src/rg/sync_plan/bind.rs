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
    /// Some prerequisite executes on a queue other than `queue`, so it is
    /// reached through a semaphore wait rather than command order.
    fn waits_other_queue(&self, queue: QueueId) -> bool {
        self.points
            .iter()
            .enumerate()
            .any(|(index, point)| point.is_some() && index != queue.index())
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
const MAX_TEMPLATES: usize = 4;
/// Frame-independent view of one imported initial cell. Timeline points are
/// reduced to a per-queue mask: their values only matter for cross-queue waits,
/// and a template is stored only when there are none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InitKey {
    resource: u32,
    range: NativeRange,
    layout: vk::ImageLayout,
    layout_known: bool,
    owner: Option<u32>,
    writer_scopes: [AccessScope; MAX_QUEUES],
    writer_host: AccessScope,
    writer_queues: u8,
    readers_scopes: [AccessScope; MAX_QUEUES],
    readers_host: AccessScope,
    readers_queues: u8,
}
struct Template {
    id: usize,
    _pin: Arc<dyn std::any::Any + Send + Sync>,
    key: Vec<InitKey>,
    aliases: Vec<AliasEdge>,
    /// Frame-independent tables, handed to the planner by reference.
    shared: Arc<SharedBound>,
    patches: Vec<BoundPatch>,
    /// (initial row, revision delta) per patch.
    bases: Vec<(u32, u64)>,
    used: u64,
}
fn shadow_bind_check() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("RHAPSODY_BIND_SHADOW").is_some_and(|v| v == "1"))
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
    /// Index of the `initial` row this cell (or its ancestor) came from.
    initial_slot: u32,
}
impl ShadowCell {
    fn new(resource: u32, cell: RangeCell, initial_slot: u32) -> Self {
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
            initial_slot,
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
/// Per-resource view of the flat shadow array, so a use only visits the cells
/// of its own resource instead of every cell of the frame.
#[derive(Default)]
struct ResourceShadow {
    /// Indices into `shadow.values`, ascending (the order the flat scan used).
    cells: smallvec::SmallVec<[u32; 4]>,
    /// bit0: some alias edge has this resource as `next`; bit1: as `previous`.
    alias_flags: u8,
    /// Every `previous` of an alias edge whose `next` is this resource.
    alias_previous: smallvec::SmallVec<[u32; 2]>,
}
pub struct SynchronizationPlanner {
    bound: BoundSynchronization,
    shadow: Storage<ShadowCell>,
    by_resource: Vec<ResourceShadow>,
    /// `initial` row of each pushed patch (parallel to `bound.patches`).
    patch_slots: Vec<u32>,
    /// Reused every frame so computing the template key allocates nothing.
    key_scratch: Vec<InitKey>,
    templates: Vec<Template>,
    template_clock: u64,
    template_hits: u64,
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
                index: Default::default(),
                shared: None,
            },
            shadow: Storage::new(0, &budget)?,
            by_resource: Vec::new(),
            patch_slots: Vec::new(),
            key_scratch: Vec::new(),
            templates: Vec::new(),
            template_clock: 0,
            template_hits: 0,
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
    /// Approximate bytes held by cached templates (at most `MAX_TEMPLATES`),
    /// kept out of `known_cpu_bytes` because they are reusable cache, not
    /// per-frame working storage, but reported so the bound is visible.
    pub fn template_cpu_bytes(&self) -> usize {
        self.templates
            .iter()
            .map(|template| {
                template.key.len() * size_of::<InitKey>()
                    + template.aliases.len() * size_of::<AliasEdge>()
                    + template.patches.len() * size_of::<BoundPatch>()
                    + template.bases.len() * size_of::<(u32, u64)>()
                    + template.shared.transitions.len() * size_of::<BoundTransition>()
                    + template.shared.batches.len() * size_of::<BoundBatch>()
                    + template.shared.order.len() * size_of::<usize>()
            })
            .sum()
    }
    pub fn begin(&mut self) {
        self.bound.initial.values.clear();
        self.bound.aliases.values.clear();
        self.bound.batches.values.clear();
        self.bound.order.values.clear();
        self.bound.transitions.values.clear();
        self.bound.patches.values.clear();
        self.bound.waits.values.clear();
        self.bound.invalidate_index();
        self.patch_slots.clear();
        self.shadow.values.clear();
        for entry in &mut self.by_resource {
            entry.cells.clear();
            entry.alias_flags = 0;
            entry.alias_previous.clear();
        }
    }
    /// The per-resource index is sized by resource number before `bind`
    /// validates it against the program; refuse numbers no graph can have
    /// instead of allocating a table for them.
    fn check_resource(&self, resource: u32) -> Result<()> {
        if resource as usize >= self.config.max_cells {
            return Err(Error::invalid_usage("native initial resource is absent"));
        }
        Ok(())
    }
    fn resource_shadow(&mut self, resource: u32) -> &mut ResourceShadow {
        let index = resource as usize;
        if self.by_resource.len() <= index {
            self.by_resource.resize_with(index + 1, Default::default);
        }
        &mut self.by_resource[index]
    }
    pub fn add_initial(&mut self, resource: u32, cells: &[RangeCell]) -> Result<()> {
        self.check_resource(resource)?;
        for &cell in cells {
            let slot = self.bound.initial.values.len() as u32;
            self.push_initial(InitialState { resource, cell })?;
            self.push_shadow(ShadowCell::new(resource, cell, slot))?;
        }
        Ok(())
    }
    /// Declare that `next` reuses the backing memory of `previous` in this
    /// frame. Call after [`Self::add_initial`] and before [`Self::bind`].
    pub fn add_alias(&mut self, previous: u32, next: u32) -> Result<()> {
        if previous == next {
            return Err(Error::invalid_usage("a resource cannot alias itself"));
        }
        self.check_resource(previous)?;
        self.check_resource(next)?;
        self.push_alias(AliasEdge { previous, next })?;
        self.resource_shadow(next).alias_flags |= 1;
        self.resource_shadow(next).alias_previous.push(previous);
        self.resource_shadow(previous).alias_flags |= 2;
        Ok(())
    }
    /// Number of bindings served from a cached template (diagnostics/tests).
    pub fn template_hits(&self) -> u64 {
        self.template_hits
    }
    /// [`Self::bind`] with a per-program template cache. `id` identifies the
    /// program and `pin` keeps its allocation alive, so an address can never be
    /// reused by a different program while a template for it exists.
    ///
    /// A template is stored only when the binding is independent of timeline
    /// values: no ownership transfer, no cross-queue wait. A later frame whose
    /// imported state differs from the template only in revision numbers and
    /// timeline points then gets the identical output, with every patch's
    /// `expected_revision` rebased onto its own initial state.
    pub fn bind_cached(
        &mut self,
        program: &RangeProgram,
        id: usize,
        pin: Arc<dyn std::any::Any + Send + Sync>,
        family_domain: impl FnMut(u32) -> Result<QueueDomain>,
    ) -> Result<()> {
        let key_ok = self.template_key();
        let key = std::mem::take(&mut self.key_scratch);
        let result = self.bind_cached_keyed(program, id, pin, family_domain, key_ok, &key);
        self.key_scratch = key;
        result
    }
    fn bind_cached_keyed(
        &mut self,
        program: &RangeProgram,
        id: usize,
        pin: Arc<dyn std::any::Any + Send + Sync>,
        family_domain: impl FnMut(u32) -> Result<QueueDomain>,
        key_ok: bool,
        key: &[InitKey],
    ) -> Result<()> {
        if key_ok {
            let found = self.templates.iter().position(|template| {
                template.id == id
                    && template.key[..] == *key
                    && template.aliases == self.bound.aliases.values[..]
            });
            if let Some(index) = found {
                if self.load_template(index)? {
                    if shadow_bind_check() {
                        let produced = self.bound_fingerprint();
                        self.bound.shared = None;
                        self.bound.batches.values.clear();
                        self.bound.order.values.clear();
                        self.bound.transitions.values.clear();
                        self.bound.patches.values.clear();
                        self.bound.waits.values.clear();
                        self.patch_slots.clear();
                        self.bind_full(program, family_domain)?;
                        assert_eq!(
                            produced,
                            self.bound_fingerprint(),
                            "bind template differs from a full bind"
                        );
                    }
                    return Ok(());
                }
            }
        }
        self.bind_full(program, family_domain)?;
        if key_ok {
            self.save_template(id, pin, key.to_vec());
        }
        Ok(())
    }
    fn bound_fingerprint(&self) -> u64 {
        let text = format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}",
            self.bound.execution_order(),
            self.bound.transitions(),
            self.bound.patches.values,
            self.bound.waits.values,
            self.bound.batches()
        );
        text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3)
        })
    }
    /// Frame-independent projection of the initial state; `None` when a
    /// released ownership makes the binding depend on a pending transfer.
    fn template_key(&mut self) -> bool {
        let mut key = std::mem::take(&mut self.key_scratch);
        key.clear();
        let ok = self.fill_template_key(&mut key);
        self.key_scratch = key;
        ok
    }
    fn fill_template_key(&self, key: &mut Vec<InitKey>) -> bool {
        for initial in &self.bound.initial.values {
            let state = initial.cell.state;
            let owner = match state.owner {
                RangeOwner::Unclaimed => None,
                RangeOwner::Owned(family) => Some(family),
                RangeOwner::Released { .. } => return false,
            };
            let mask = |frontier: &Frontier| {
                frontier
                    .points
                    .points()
                    .fold(0u8, |mask, point| mask | 1 << point.queue.index())
            };
            key.push(InitKey {
                resource: initial.resource,
                range: initial.cell.range,
                layout: state.layout,
                layout_known: state.layout_known,
                owner,
                writer_scopes: state.writer.scopes,
                writer_host: state.writer.host,
                writer_queues: mask(&state.writer),
                readers_scopes: state.readers.scopes,
                readers_host: state.readers.host,
                readers_queues: mask(&state.readers),
            });
        }
        true
    }
    fn save_template(&mut self, id: usize, pin: Arc<dyn std::any::Any + Send + Sync>, key: Vec<InitKey>) {
        let bound = &self.bound;
        let single_queue = bound
            .batches
            .values
            .iter()
            .all(|batch| matches!(batch.kind, BoundBatchKind::Graph { .. }));
        let access_only = bound
            .patches
            .values
            .iter()
            .all(|patch| matches!(patch.action, BoundPatchAction::Access { .. }));
        if !single_queue
            || !access_only
            || !bound.waits.values.is_empty()
            || self.patch_slots.len() != bound.patches.values.len()
        {
            return;
        }
        let mut bases = Vec::with_capacity(bound.patches.values.len());
        for (patch, &slot) in bound.patches.values.iter().zip(&self.patch_slots) {
            let Some(initial) = bound.initial.values.get(slot as usize) else {
                return;
            };
            let Some(delta) = patch.expected_revision.checked_sub(initial.cell.state.revision)
            else {
                return;
            };
            bases.push((slot, delta));
        }
        self.template_clock += 1;
        let template = Template {
            id,
            _pin: pin,
            key,
            aliases: bound.aliases.values.to_vec(),
            shared: Arc::new(SharedBound {
                transitions: bound.transitions.values.to_vec(),
                batches: bound.batches.values.to_vec(),
                order: bound.order.values.to_vec(),
                index: bound.index.clone(),
            }),
            patches: bound.patches.values.to_vec(),
            bases,
            used: self.template_clock,
        };
        if let Some(existing) = self
            .templates
            .iter_mut()
            .find(|old| old.id == id && old.key == template.key && old.aliases == template.aliases)
        {
            *existing = template;
        } else if self.templates.len() < MAX_TEMPLATES {
            self.templates.push(template);
        } else if let Some(oldest) = self.templates.iter_mut().min_by_key(|old| old.used) {
            *oldest = template;
        }
    }
    /// Fills the bound tables from template `index`, rebasing revisions.
    /// Returns false (leaving the tables empty) if a revision would overflow.
    fn load_template(&mut self, index: usize) -> Result<bool> {
        let template = std::mem::take(&mut self.templates[index].patches);
        let result = self.load_template_rows(index, &template);
        self.templates[index].patches = template;
        result
    }
    fn load_template_rows(&mut self, index: usize, patches: &[BoundPatch]) -> Result<bool> {
        let bases = std::mem::take(&mut self.templates[index].bases);
        let mut rebased = true;
        for (patch, &(slot, delta)) in patches.iter().zip(&bases) {
            let revision = self.bound.initial.values[slot as usize]
                .cell
                .state
                .revision
                .checked_add(delta)
                // The patch's own increment must also stay below the reserved
                // maximum, exactly as `ShadowCell::increment` requires.
                .filter(|revision| revision.checked_add(1).is_some_and(|next| next != u64::MAX));
            let Some(revision) = revision else {
                rebased = false;
                break;
            };
            let mut row = *patch;
            row.expected_revision = revision;
            if let Err(error) = self.push_patch(row) {
                self.templates[index].bases = bases;
                return Err(error);
            }
        }
        self.templates[index].bases = bases;
        if !rebased {
            self.bound.patches.values.clear();
            return Ok(false);
        }
        // Shared by reference: no copy of transitions, batches, order or index.
        self.bound.shared = Some(self.templates[index].shared.clone());
        self.template_clock += 1;
        self.templates[index].used = self.template_clock;
        self.template_hits += 1;
        Ok(true)
    }
    pub fn bind(
        &mut self,
        program: &RangeProgram,
        family_domain: impl FnMut(u32) -> Result<QueueDomain>,
    ) -> Result<()> {
        self.bind_full(program, family_domain)
    }
    fn bind_full(
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
            return Err(Error::invalid_usage(
                "native alias edge names an absent resource",
            ));
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
        self.bound.finish_index();
        Ok(())
    }
    /// A scheduled batch dependency. Resource hazards already produced exact
    /// waits; an ordering-only edge waits at the stages where the destination
    /// batch first touches resources the source batch also used (falling back
    /// to every stage the destination batch uses).
    fn static_dependency(&mut self, program: &RangeProgram, from: usize, to: usize) -> Result<()> {
        let (Some(source), Some(target)) = (program.batches.get(from), program.batches.get(to))
        else {
            return Err(Error::invalid_usage(
                "native batch dependency destination absent",
            ));
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
                if x.value >= y.value {
                    a
                } else {
                    b
                }
            }
            (DependencyRef::Batch(_), DependencyRef::Point(_)) => a,
            (DependencyRef::Point(_), DependencyRef::Batch(_)) => b,
            (DependencyRef::Batch(x), DependencyRef::Batch(y)) => {
                if position(x) >= position(y) {
                    a
                } else {
                    b
                }
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
        if let Some(entry) = self.by_resource.get(current.resource as usize) {
            for &previous in &entry.alias_previous {
                let Some(other) = self.by_resource.get(previous as usize) else {
                    continue;
                };
                for &index in &other.cells {
                    let cell = &self.shadow.values[index as usize];
                    writer.merge(cell.writer, later);
                    writer.merge(cell.readers, later);
                }
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
        let alias_flags = self.resource_shadow(access.resource).alias_flags;
        // Fragments split off below are appended past `visit`; like the old
        // flat scan, this use does not revisit them.
        let visit = self.by_resource[access.resource as usize].cells.len();
        let mut covered = false;
        for slot in 0..visit {
            let index = self.by_resource[access.resource as usize].cells[slot] as usize;
            let mut current = self.shadow.values[index];
            let Some(overlap) = range_state::intersection(current.range, access.range) else {
                continue;
            };
            covered = true;
            if !current.layout_known && matches!(current.range, NativeRange::Image { .. }) {
                return Err(Error::unsupported(
                    "opaque image use requires an explicit external native-state declaration",
                ));
            }
            // An alias member's contents are discarded at its first use in a
            // frame; its set returns it only when idle with the state reset
            // (AliasCache::give_back). Never acquire a stale release for it.
            if current.fresh
                && alias_flags != 0
                && (matches!(current.owner, PendingOwner::Released { .. })
                    || !sharing.is_concurrent()
                        && matches!(current.owner, PendingOwner::Owned(owner) if owner != domain.family))
            {
                return Err(Error::invalid_usage(
                    "alias member keeps queue-family ownership from an earlier frame",
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
                    // Discarded contents (UNDEFINED image) have nothing to hand
                    // over: the using family just claims the range, and the
                    // UNDEFINED -> layout transition below runs on its queue.
                    PendingOwner::Owned(_) if Self::contents_discarded(&current) => {
                        current.owner = PendingOwner::Owned(domain.family);
                    }
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
                        self.push_patch_slot(current.initial_slot, BoundPatch {
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
                if alias_flags & 1 != 0 {
                    current = self.discard_alias_contents(current);
                }
            }
            let changes_layout = current.layout != access.layout;
            let changes = access.scope.write || changes_layout;
            self.wait_frontier(current.writer, batch, access.scope.stages, domain)?;
            if changes {
                self.wait_frontier(current.readers, batch, access.scope.stages, domain)?;
            }
            let mut source = current.writer.source(queue).union(if changes {
                current.readers.source(queue)
            } else {
                AccessScope::EMPTY
            });
            // A layout transition after a cross-queue semaphore wait must chain
            // with that wait, which blocks exactly `access.scope.stages`.
            if changes_layout
                && (current.writer.waits_other_queue(queue)
                    || changes && current.readers.waits_other_queue(queue))
            {
                source.stages |= access.scope.stages;
            }
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
            self.push_patch_slot(current.initial_slot, BoundPatch {
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
            // Execution-only source: the acquire (and its layout transition)
            // chains with the release wait at the same stages.
            source: AccessScope {
                stages: transfer.destination.stages,
                ..AccessScope::EMPTY
            },
            destination: transfer.destination,
            old_layout: transfer.old_layout,
            new_layout: transfer.new_layout,
            source_family: transfer.source_family,
            destination_family: transfer.destination_family,
            kind: BarrierKind::Acquire {
                pair: transfer.pair,
            },
        })?;
        self.push_patch_slot(current.initial_slot, BoundPatch {
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
    /// An image range with known UNDEFINED layout holds no defined contents.
    fn contents_discarded(cell: &ShadowCell) -> bool {
        matches!(cell.range, NativeRange::Image { .. })
            && cell.layout_known
            && cell.layout == vk::ImageLayout::UNDEFINED
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
                let own = target.bytes();
                let total = self.known_cpu_bytes();
                let target = &mut self.bound.$field;
                target.grow(capacity, total - own, config, &self.budget)?;
            }
            self.bound.$field.values.push(value);
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
    fn push_patch_slot(&mut self, slot: u32, value: BoundPatch) -> Result<()> {
        self.push_patch(value)?;
        self.patch_slots.push(slot);
        Ok(())
    }
    fn push_shadow(&mut self, value: ShadowCell) -> Result<()> {
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
            let own = target.bytes();
            let total = self.known_cpu_bytes();
            self.shadow
                .grow(capacity, total - own, config, &self.budget)?;
        }
        let target = &mut self.shadow;
        let index = target.values.len() as u32;
        let resource = value.resource;
        target.values.push(value);
        self.resource_shadow(resource).cells.push(index);
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
    fn cross_queue_layout_transition_and_acquire_chain_with_their_precise_waits() {
        let image = NativeRange::Image {
            aspects: vk::ImageAspectFlags::COLOR.as_raw(),
            base_mip: 0,
            mip_count: 1,
            base_layer: 0,
            layer_count: 1,
        };
        let fragment = AccessScope {
            stages: vk::PipelineStageFlags::FRAGMENT_SHADER,
            access: vk::AccessFlags::SHADER_READ,
            write: false,
        };
        let writes = RangeUse {
            resource: 0,
            range: image,
            scope: AccessScope {
                stages: vk::PipelineStageFlags::COMPUTE_SHADER,
                access: vk::AccessFlags::SHADER_WRITE,
                write: true,
            },
            layout: vk::ImageLayout::GENERAL,
            access_code: 0x34,
        };
        let reads = RangeUse {
            scope: fragment,
            layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            access_code: 0x25,
            ..writes
        };
        for (sharing, acquire) in [
            (
                ResourceSharing::Concurrent(crate::resource::QueueFamilies::new(&[0, 1]).unwrap()),
                false,
            ),
            (ResourceSharing::Exclusive, true),
        ] {
            let mut program = program(image, vec![writes, reads], &[queue(1, 1), queue(0, 0)]);
            program.resources[0].sharing = sharing;
            let mut planner = SynchronizationPlanner::new(
                Arc::new(AllocatorStats::default()),
                Default::default(),
            )
            .unwrap();
            planner
                .add_initial(
                    0,
                    &[RangeCell {
                        range: image,
                        state: NativeState::default(),
                    }],
                )
                .unwrap();
            planner
                .bind(&program, |family| Ok(queue(family, family)))
                .unwrap();
            let bound = planner.bound();
            let wait = bound
                .waits()
                .iter()
                .find(|wait| wait.batch == 1)
                .expect("cross-queue wait");
            assert_eq!(wait.stages, vk::PipelineStageFlags::FRAGMENT_SHADER);
            let consumer = bound
                .transitions()
                .iter()
                .filter(|transition| transition.batch == 1)
                .collect::<Vec<_>>();
            assert!(!consumer.is_empty());
            assert_eq!(
                consumer
                    .iter()
                    .any(|transition| matches!(transition.kind, BarrierKind::Acquire { .. })),
                acquire
            );
            for transition in consumer {
                // Every barrier on the consumer queue chains with the wait.
                assert!(
                    transition.source.stages.contains(wait.stages),
                    "{transition:?} does not chain with {wait:?}"
                );
            }
        }
    }
    /// Two exclusive alias members of one backing used on family 0. Member 1
    /// was released to family 1 by an earlier frame whose acquire never ran.
    fn alias_after_failed_release(reset: bool) -> Result<SynchronizationPlanner> {
        use crate::resource::{
            SyncCell,
            range_state::{PatchAction, StatePatch},
        };
        let stats = Arc::new(AllocatorStats::default());
        let range = buffer(0, 64);
        let q1 = QueueId::new(1).unwrap();
        let cell = Arc::new(
            SyncCell::for_device(crate::device::DeviceGeneration::next())
                .with_ownership_range(range),
        );
        cell.with_range_state(NativeState::default(), &stats, |_| Ok(()))
            .unwrap();
        let transfer = RangeTransfer {
            pair: 7,
            source_family: 1,
            destination_family: 0,
            range,
            source: usage(range, true).scope,
            destination: usage(range, false).scope,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::UNDEFINED,
        };
        cell.prepare_range_state(
            &[
                StatePatch {
                    range,
                    expected_revision: 0,
                    action: PatchAction::Access {
                        scope: usage(range, true).scope,
                        layout: vk::ImageLayout::UNDEFINED,
                        queue: q1,
                        family: 1,
                    },
                },
                StatePatch {
                    range,
                    expected_revision: 1,
                    action: PatchAction::Release(transfer),
                },
            ],
            q1,
            1,
            &stats,
        )
        .unwrap();
        let release = SyncPoint {
            queue: q1,
            value: 5,
        };
        cell.publish_range_state(release);
        cell.insert_overall(release);
        let mut completed = crate::sync::CompletedValues::default();
        // Still executing: the member must not be reset (or returned) yet.
        assert!(!cell.discard_idle_range_state(&completed));
        cell.with_range_state(NativeState::default(), &stats, |table| {
            assert!(matches!(
                table.cells()[0].state.owner,
                RangeOwner::Released { .. }
            ));
            Ok(())
        })
        .unwrap();
        if reset {
            completed.raise(q1, 5);
            assert!(cell.discard_idle_range_state(&completed));
        }
        let initial = cell
            .with_range_state(NativeState::default(), &stats, |table| {
                Ok(table.cells().to_vec())
            })
            .unwrap();
        let mut program = program(
            range,
            vec![usage(range, true), usage(range, true)],
            &[queue(0, 0), queue(0, 0)],
        );
        program.resources.push(program.resources[0]);
        program.uses[1].resource = 1;
        program.dependencies = vec![(0, 1)];
        let mut planner = SynchronizationPlanner::new(stats, Default::default()).unwrap();
        planner
            .add_initial(
                0,
                &[RangeCell {
                    range,
                    state: NativeState::default(),
                }],
            )
            .unwrap();
        planner.add_initial(1, &initial).unwrap();
        planner.add_alias(0, 1).unwrap();
        planner.add_alias(1, 0).unwrap();
        planner.bind(&program, |family| Ok(queue(family, family)))?;
        Ok(planner)
    }
    #[test]
    fn alias_member_released_by_a_failed_frame_is_discarded_without_stale_acquire() {
        let planner = alias_after_failed_release(true).unwrap();
        let bound = planner.bound();
        assert!(
            bound
                .batches()
                .iter()
                .all(|batch| matches!(batch.kind, BoundBatchKind::Graph { .. })),
            "no ownership prelude batch"
        );
        assert!(
            bound.transitions().iter().all(|transition| matches!(
                transition.kind,
                BarrierKind::Memory
            ) && transition.source_family
                == vk::QUEUE_FAMILY_IGNORED),
            "no ownership transfer barrier"
        );
        assert!(
            bound
                .patches()
                .iter()
                .all(|patch| matches!(patch.action, BoundPatchAction::Access { .. }))
        );
        // The completed release is neither waited on nor carried forward.
        assert!(
            bound
                .waits()
                .iter()
                .all(|wait| !matches!(wait.dependency, DependencyRef::Point(_)))
        );
        // An unreset stale release is rejected instead of acquired.
        let Err(error) = alias_after_failed_release(false) else {
            panic!("stale release must not bind");
        };
        assert!(error.to_string().contains("alias member"), "{error}");
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

    /// Deterministic synthetic program: `resources` buffers, `passes` passes
    /// of `per_pass` sub-range uses each, spread over `domains` batches.
    fn synthetic(
        resources: u32,
        passes: usize,
        per_pass: usize,
        domains: &[QueueDomain],
    ) -> (RangeProgram, Vec<RangeCell>) {
        const SIZE: u64 = 4096;
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut uses = Vec::new();
        let mut program_passes = Vec::new();
        for pass in 0..passes {
            let first = uses.len();
            for _ in 0..per_pass {
                let resource = (next() % resources as u64) as u32;
                let offset = (next() % 16) * 256;
                let size = (1 + next() % 4) * 256;
                let mut item = usage(buffer(offset, size.min(SIZE - offset)), next() % 3 == 0);
                item.resource = resource;
                uses.push(item);
            }
            program_passes.push(ProgramPass {
                pass,
                uses: first..uses.len(),
            });
        }
        let per_batch = passes.div_ceil(domains.len().max(1)).max(1);
        let batches = (0..passes.div_ceil(per_batch))
            .map(|batch| ProgramBatch {
                domain: domains[batch % domains.len()],
                passes: batch * per_batch..((batch + 1) * per_batch).min(passes),
            })
            .collect();
        let program = RangeProgram {
            resources: (0..resources)
                .map(|_| ResourceDomain {
                    range: buffer(0, SIZE),
                    sharing: ResourceSharing::Exclusive,
                })
                .collect(),
            uses,
            passes: program_passes,
            batches,
            ..Default::default()
        };
        let initial = vec![
            RangeCell {
                range: buffer(0, SIZE),
                state: NativeState::default(),
            };
            1
        ];
        (program, initial)
    }
    fn bind_synthetic(program: &RangeProgram, initial: &[RangeCell]) -> SynchronizationPlanner {
        let mut planner =
            SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
                .unwrap();
        for resource in 0..program.resources.len() as u32 {
            planner.add_initial(resource, initial).unwrap();
        }
        planner
            .bind(program, |family| Ok(queue(family, family)))
            .unwrap();
        planner
    }
    fn fingerprint(bound: &BoundSynchronization) -> u64 {
        let text = format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}",
            bound.execution_order(),
            bound.transitions(),
            bound.patches(),
            bound.waits(),
            bound.batches()
        );
        text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3)
        })
    }
    /// The planner output for a fixed synthetic program must not change when
    /// the lookup structure changes. Values recorded from the scan-everything
    /// implementation.
    #[test]
    fn synthetic_binding_output_is_stable() {
        let one = [queue(0, 0)];
        let two = [queue(0, 0), queue(1, 1)];
        let cases: [(&[QueueDomain], u32, usize, usize); 3] =
            [(&one, 40, 60, 6), (&two, 24, 48, 5), (&two, 120, 160, 12)];
        let mut prints = Vec::new();
        for (domains, resources, passes, per_pass) in cases {
            let (program, initial) = synthetic(resources, passes, per_pass, domains);
            let planner = bind_synthetic(&program, &initial);
            prints.push(fingerprint(planner.bound()));
        }
        eprintln!("FINGERPRINTS {prints:?}");
        assert_eq!(prints, EXPECTED_FINGERPRINTS);
    }
    /// The bucketed accessors must equal the linear filters they replace, in
    /// content and order, for every batch and phase.
    #[test]
    fn phase_index_matches_linear_filter() {
        let one = [queue(0, 0)];
        let two = [queue(0, 0), queue(1, 1)];
        for (domains, resources, passes, per_pass) in
            [(&one[..], 40, 60, 6), (&two[..], 24, 48, 5), (&two[..], 120, 160, 12)]
        {
            let (program, initial) = synthetic(resources, passes, per_pass, domains);
            let planner = bind_synthetic(&program, &initial);
            let bound = planner.bound();
            assert!(bound.index.valid, "synthetic program must index");
            let mut seen = 0;
            for (batch, entry) in bound.batches().iter().enumerate() {
                let mut phases = vec![BarrierPhase::Initial, BarrierPhase::Export];
                if let BoundBatchKind::Graph { passes, .. } = &entry.kind {
                    phases.extend(passes.clone().map(BarrierPhase::BeforePass));
                }
                for phase in phases {
                    let fast: Vec<_> = bound.transitions_for(batch, phase).copied().collect();
                    let slow: Vec<_> = bound
                        .transitions()
                        .iter()
                        .copied()
                        .filter(|t| t.batch == batch && t.phase == phase)
                        .collect();
                    seen += fast.len();
                    assert_eq!(fast, slow, "transitions batch {batch} {phase:?}");
                }
                let fast: Vec<_> = bound.patches_for(batch).copied().collect();
                let slow: Vec<_> = bound.patches().iter().copied().filter(|p| p.batch == batch).collect();
                assert_eq!(fast, slow, "patches batch {batch}");
                let fast: Vec<_> = bound.waits_for(batch).copied().collect();
                let slow: Vec<_> = bound.waits().iter().copied().filter(|w| w.batch == batch).collect();
                assert_eq!(fast, slow, "waits batch {batch}");
            }
            assert_eq!(seen, bound.transitions().len(), "every transition has a slot");
        }
    }
    fn imported(revision: u64, point: u64, layout_known: bool) -> Vec<RangeCell> {
        let mut state = NativeState {
            revision,
            layout_known,
            ..NativeState::default()
        };
        let mut writer = Frontier::EMPTY;
        writer.points.insert(crate::sync::SyncPoint {
            queue: crate::sync::QueueId::new(0).unwrap(),
            value: point,
        });
        state.writer = writer;
        vec![RangeCell {
            range: buffer(0, 4096),
            state,
        }]
    }
    fn bind_with(
        planner: &mut SynchronizationPlanner,
        program: &RangeProgram,
        initial: &[RangeCell],
        cached: bool,
    ) {
        planner.begin();
        for resource in 0..program.resources.len() as u32 {
            planner.add_initial(resource, initial).unwrap();
        }
        let family = |family| Ok(queue(family, family));
        if cached {
            let pin: Arc<dyn std::any::Any + Send + Sync> = Arc::new(());
            planner
                .bind_cached(program, program as *const _ as usize, pin, family)
                .unwrap();
        } else {
            planner.bind(program, family).unwrap();
        }
    }
    /// A frame that differs only in revisions and timeline points must reuse
    /// the template and produce exactly what a full bind produces.
    #[test]
    fn rebased_template_equals_full_binding_across_frames() {
        let one = [queue(0, 0)];
        let (program, _) = synthetic(40, 60, 6, &one);
        let mut cached = SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default()).unwrap();
        let mut fresh = SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default()).unwrap();
        for frame in 0..5u64 {
            let initial = imported(10 * frame + 3, frame + 1, true);
            bind_with(&mut cached, &program, &initial, true);
            bind_with(&mut fresh, &program, &initial, false);
            assert_eq!(fingerprint(cached.bound()), fingerprint(fresh.bound()), "frame {frame}");
            assert_eq!(cached.template_hits(), frame, "frame {frame}");
        }
        // A changed layout is a different key: no hit, still exact.
        let before = cached.template_hits();
        let initial = imported(99, 9, false);
        bind_with(&mut cached, &program, &initial, true);
        bind_with(&mut fresh, &program, &initial, false);
        assert_eq!(cached.template_hits(), before);
        assert_eq!(fingerprint(cached.bound()), fingerprint(fresh.bound()));
    }
    /// A pending ownership release can never be served from a template.
    #[test]
    fn released_owner_is_never_templated() {
        let two = [queue(0, 0), queue(1, 1)];
        let (program, initial) = synthetic(24, 48, 5, &two);
        let mut planner = SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default()).unwrap();
        for _ in 0..3 {
            bind_with(&mut planner, &program, &initial, true);
        }
        // Two families with exclusive resources produce ownership transfers.
        assert_eq!(planner.template_hits(), 0);
    }
    /// Cost of the submission-time replay for the synthetic program: every
    /// resource's table prepares and publishes that resource's patches.
    #[test]
    #[ignore]
    fn replay_cost_baseline() {
        use crate::resource::range_state::{RangeStateTable, StatePatch};
        let one = [queue(0, 0)];
        let (program, initial) = synthetic(200, 160, 19, &one);
        let planner = bind_synthetic(&program, &initial);
        let bound = planner.bound();
        let q0 = crate::sync::QueueId::new(0).unwrap();
        let mut per_resource: Vec<Vec<StatePatch>> = vec![Vec::new(); program.resources.len()];
        for patch in bound.patches() {
            per_resource[patch.resource as usize].push(patch.resolve(|_| None).unwrap());
        }
        let stats = Arc::new(AllocatorStats::default());
        let (mut prepare_ns, mut publish_ns) = (0u128, 0u128);
        let mut samples = Vec::new();
        let rounds: usize = std::env::var("REPLAY_ROUNDS").ok().and_then(|v| v.parse().ok()).unwrap_or(30);
        for _ in 0..rounds {
            let mut tables: Vec<_> = (0..program.resources.len())
                .map(|_| {
                    RangeStateTable::new(
                        buffer(0, 4096),
                        ResourceSharing::Exclusive,
                        NativeState::default(),
                        Default::default(),
                        stats.clone(),
                    )
                    .unwrap()
                })
                .collect();
            // Production tables are long-lived with warm capacity.
            for (table, patches) in tables.iter_mut().zip(&per_resource) {
                if !patches.is_empty() {
                    table.prepare(patches, q0, 0).unwrap();
                    table.cancel_prepared();
                }
            }
            let start = std::time::Instant::now();
            prepare_ns = 0;
            publish_ns = 0;
            for (table, patches) in tables.iter_mut().zip(&per_resource) {
                if patches.is_empty() {
                    continue;
                }
                let t0 = std::time::Instant::now();
                table.prepare(patches, q0, 0).unwrap();
                let t1 = std::time::Instant::now();
                table.publish(crate::sync::SyncPoint { queue: q0, value: 1 });
                prepare_ns += (t1 - t0).as_nanos();
                publish_ns += t1.elapsed().as_nanos();
            }
            samples.push(start.elapsed());
        }
        eprintln!("last round: prepare={prepare_ns}ns publish={publish_ns}ns");
        samples.sort();
        eprintln!(
            "replay patches={} median={:?} p95={:?}",
            bound.patches().len(),
            samples[samples.len() / 2],
            samples[samples.len() * 95 / 100]
        );
    }
    /// A resource number beyond any possible graph is refused up front rather
    /// than sizing the per-resource index for it.
    #[test]
    fn absurd_resource_numbers_are_rejected_without_allocating_an_index() {
        let mut planner =
            SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
                .unwrap();
        let cell = RangeCell {
            range: buffer(0, 16),
            state: NativeState::default(),
        };
        assert!(planner.add_initial(u32::MAX, &[cell]).is_err());
        assert!(planner.add_alias(0, u32::MAX).is_err());
        assert!(planner.by_resource.is_empty());
    }
    /// A template must not be applied when the patch's own revision increment
    /// would exhaust the counter; a full bind rejects that frame.
    #[test]
    fn rebase_that_would_exhaust_a_revision_falls_back_to_the_full_bind_error() {
        let one = [queue(0, 0)];
        let (program, _) = synthetic(4, 6, 3, &one);
        let mut planner =
            SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
                .unwrap();
        bind_with(&mut planner, &program, &imported(1, 1, true), true);
        bind_with(&mut planner, &program, &imported(2, 2, true), true);
        assert_eq!(planner.template_hits(), 1);
        let hits = planner.template_hits();
        planner.begin();
        for resource in 0..program.resources.len() as u32 {
            planner
                .add_initial(resource, &imported(u64::MAX - 2, 3, true))
                .unwrap();
        }
        let pin: Arc<dyn std::any::Any + Send + Sync> = Arc::new(());
        let result = planner.bind_cached(
            &program,
            &program as *const _ as usize,
            pin,
            |family| Ok(queue(family, family)),
        );
        assert!(result.is_err(), "exhausted revision must be an error");
        assert_eq!(planner.template_hits(), hits, "no template served it");
    }
    const EXPECTED_FINGERPRINTS: [u64; 3] = [
        15114495150305685386,
        7488788626014099824,
        8713088647009251552,
    ];
    /// `cargo test -p rhapsody --release --lib bind_scaling -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn bind_scaling_baseline() {
        let one = [queue(0, 0)];
        let (program, initial) = synthetic(200, 160, 19, &one);
        let mut planner =
            SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
                .unwrap();
        let mut samples = Vec::new();
        for _ in 0..30 {
            let start = std::time::Instant::now();
            planner.begin();
            for resource in 0..program.resources.len() as u32 {
                planner.add_initial(resource, &initial).unwrap();
            }
            planner
                .bind(&program, |family| Ok(queue(family, family)))
                .unwrap();
            samples.push(start.elapsed());
        }
        samples.sort();
        let mut cached = Vec::new();
        for frame in 0..30u64 {
            let start = std::time::Instant::now();
            bind_with(&mut planner, &program, &imported(frame * 7, frame + 1, true), true);
            cached.push(start.elapsed());
        }
        cached.sort();
        eprintln!(
            "bind_cached hits={} median={:?}",
            planner.template_hits(),
            cached[cached.len() / 2]
        );
        eprintln!(
            "bind U={} S={} transitions={} median={:?} p95={:?}",
            program.uses.len(),
            planner.shadow.values.len(),
            planner.bound().transitions().len(),
            samples[samples.len() / 2],
            samples[samples.len() * 95 / 100]
        );
    }
}
