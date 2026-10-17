//! Explicit cold-path verification of the native binding consumed by recording.
//! This replays independent spatial cells; it never invokes the binder or its
//! production range splitting/state-transition implementation.
use super::{
    invalid,
    range_oracle::{SpatialLedger, contains},
};
use crate::{
    resource::{
        OwnershipRange,
        range_state::{AccessScope, Frontier, NativeState, RangeOwner, RangeTransfer},
    },
    rg::sync_plan::{
        BarrierKind, BarrierPhase, BoundBatch, BoundBatchKind, BoundPatch, BoundPatchAction,
        BoundSynchronization, BoundTransition, BoundWait, DependencyRef, InitialState,
        RangeProgram, RangeUse,
    },
    sync::{MAX_QUEUES, QueueId, SyncSet},
};
use ash::vk;
use std::collections::BTreeSet;

fn stages_cover(provided: vk::PipelineStageFlags, required: vk::PipelineStageFlags) -> bool {
    let mut remaining = required;
    if provided.contains(vk::PipelineStageFlags::ALL_COMMANDS) {
        remaining &= vk::PipelineStageFlags::HOST;
    }
    if provided.contains(vk::PipelineStageFlags::ALL_GRAPHICS) {
        remaining &= !(vk::PipelineStageFlags::DRAW_INDIRECT
            | vk::PipelineStageFlags::VERTEX_INPUT
            | vk::PipelineStageFlags::VERTEX_SHADER
            | vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
            | vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER
            | vk::PipelineStageFlags::GEOMETRY_SHADER
            | vk::PipelineStageFlags::FRAGMENT_SHADER
            | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS
            | vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT);
    }
    provided.contains(remaining)
}
fn scope_covers(provided: AccessScope, required: AccessScope) -> bool {
    let mut access = required.access;
    if provided.access.contains(vk::AccessFlags::MEMORY_READ) {
        access &= !(vk::AccessFlags::INDIRECT_COMMAND_READ
            | vk::AccessFlags::INDEX_READ
            | vk::AccessFlags::VERTEX_ATTRIBUTE_READ
            | vk::AccessFlags::UNIFORM_READ
            | vk::AccessFlags::INPUT_ATTACHMENT_READ
            | vk::AccessFlags::SHADER_READ
            | vk::AccessFlags::COLOR_ATTACHMENT_READ
            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
            | vk::AccessFlags::TRANSFER_READ
            | vk::AccessFlags::HOST_READ);
    }
    if provided.access.contains(vk::AccessFlags::MEMORY_WRITE) {
        access &= !(vk::AccessFlags::SHADER_WRITE
            | vk::AccessFlags::COLOR_ATTACHMENT_WRITE
            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
            | vk::AccessFlags::TRANSFER_WRITE
            | vk::AccessFlags::HOST_WRITE);
    }
    stages_cover(provided.stages, required.stages) && provided.access.contains(access)
}
fn union(a: AccessScope, b: AccessScope) -> AccessScope {
    AccessScope {
        stages: a.stages | b.stages,
        access: a.access | b.access,
        write: a.write || b.write,
    }
}
#[derive(Clone)]
struct History {
    queue: Option<QueueId>,
    dependency: Option<DependencyRef>,
    scope: AccessScope,
    visible: [AccessScope; MAX_QUEUES],
}
fn history(frontier: Frontier) -> Vec<History> {
    let mut result = Vec::new();
    for (index, scope) in frontier.scopes.into_iter().enumerate() {
        let queue = QueueId::new(index).expect("fixed queue domain");
        let value = frontier.points.get(queue);
        if !scope.is_empty() || value != 0 {
            result.push(History {
                queue: Some(queue),
                dependency: (value != 0).then_some(DependencyRef::Point(crate::sync::SyncPoint {
                    queue,
                    value,
                })),
                scope,
                visible: [AccessScope::EMPTY; MAX_QUEUES],
            });
        }
    }
    if !frontier.host.is_empty() {
        result.push(History {
            queue: None,
            dependency: None,
            scope: frontier.host,
            visible: [AccessScope::EMPTY; MAX_QUEUES],
        });
    }
    result
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Owner {
    Unclaimed,
    Owned(u32),
    Released {
        transfer: RangeTransfer,
        dependency: DependencyRef,
    },
}
#[derive(Clone)]
struct State {
    revision: u64,
    layout: vk::ImageLayout,
    known: bool,
    pending_layout: Option<vk::ImageLayout>,
    owner: Owner,
    writer: Vec<History>,
    readers: Vec<History>,
}
impl From<NativeState> for State {
    fn from(state: NativeState) -> Self {
        Self {
            revision: state.revision,
            layout: state.layout,
            known: state.layout_known,
            pending_layout: None,
            owner: match state.owner {
                RangeOwner::Unclaimed => Owner::Unclaimed,
                RangeOwner::Owned(family) => Owner::Owned(family),
                RangeOwner::Released { transfer, point } => Owner::Released {
                    transfer,
                    dependency: DependencyRef::Point(point),
                },
            },
            writer: history(state.writer),
            readers: history(state.readers),
        }
    }
}

struct View<'a> {
    initial: &'a [InitialState],
    batches: &'a [BoundBatch],
    order: &'a [usize],
    transitions: &'a [BoundTransition],
    patches: &'a [BoundPatch],
    waits: &'a [BoundWait],
}
struct Ordering {
    ancestors: Vec<BTreeSet<usize>>,
    points: Vec<SyncSet>,
}
impl Ordering {
    fn new(view: &View<'_>) -> crate::Result<Self> {
        let mut positions = vec![None; view.batches.len()];
        for (position, &batch) in view.order.iter().enumerate() {
            let slot = positions
                .get_mut(batch)
                .ok_or_else(|| invalid("native order names an absent batch"))?;
            if slot.replace(position).is_some() {
                return Err(invalid("native order repeats a batch"));
            }
        }
        if positions.iter().any(Option::is_none) {
            return Err(invalid("native order omits a batch"));
        }
        let mut known = SyncSet::EMPTY;
        for initial in view.initial {
            known.merge(&initial.cell.state.writer.points);
            known.merge(&initial.cell.state.readers.points);
            if let RangeOwner::Released { point, .. } = initial.cell.state.owner {
                if point.value == 0 {
                    return Err(invalid("released native state has a zero point"));
                }
                known.insert(point);
            }
        }
        for wait in view.waits {
            if wait.batch >= view.batches.len() || wait.stages.is_empty() {
                return Err(invalid("native wait has no destination or stages"));
            }
            match wait.dependency {
                DependencyRef::Batch(source)
                    if source >= view.batches.len()
                        || positions[source] >= positions[wait.batch] =>
                {
                    return Err(invalid("native wait refers to an unsubmitted batch"));
                }
                DependencyRef::Point(point) if point.value == 0 => {
                    return Err(invalid("native wait uses an unsubmitted zero point"));
                }
                _ => {}
            }
        }
        let mut result = Self {
            ancestors: vec![BTreeSet::new(); view.batches.len()],
            points: vec![SyncSet::EMPTY; view.batches.len()],
        };
        let mut previous = [None; MAX_QUEUES];
        for &batch in view.order {
            let queue = queue(view.batches[batch].domain.queue_index)?;
            let mut sources = BTreeSet::new();
            sources.extend(previous[queue.index()]);
            for wait in view.waits.iter().filter(|wait| wait.batch == batch) {
                match wait.dependency {
                    DependencyRef::Point(point) => result.points[batch].insert(point),
                    DependencyRef::Batch(source) => {
                        sources.insert(source);
                    }
                }
            }
            for source in sources {
                result.ancestors[batch].insert(source);
                let inherited = result.ancestors[source].clone();
                result.ancestors[batch].extend(inherited);
                let inherited = result.points[source];
                result.points[batch].merge(&inherited);
            }
            result.points[batch].insert(crate::sync::SyncPoint {
                queue,
                value: known.get(queue),
            });
            previous[queue.index()] = Some(batch);
        }
        Ok(result)
    }
    fn covers(&self, provided: DependencyRef, required: DependencyRef) -> bool {
        match (provided, required) {
            (DependencyRef::Point(a), DependencyRef::Point(b)) => {
                a.queue == b.queue && a.value >= b.value
            }
            (DependencyRef::Batch(a), DependencyRef::Batch(b)) => {
                a == b || self.ancestors[a].contains(&b)
            }
            (DependencyRef::Batch(a), DependencyRef::Point(b)) => {
                self.points[a].get(b.queue) >= b.value
            }
            (DependencyRef::Point(_), DependencyRef::Batch(_)) => false,
        }
    }
}
fn queue(index: u32) -> crate::Result<QueueId> {
    usize::try_from(index)
        .ok()
        .and_then(QueueId::new)
        .ok_or_else(|| invalid("native binding queue index exceeds its domain"))
}
fn visible(
    history: &mut History,
    destination: AccessScope,
    batch: usize,
    view: &View<'_>,
    ordering: &Ordering,
) -> crate::Result<()> {
    let queue = queue(view.batches[batch].domain.queue_index)?;
    if scope_covers(history.visible[queue.index()], destination) {
        return Ok(());
    }
    if let (Some(source), Some(dependency)) = (history.queue, history.dependency) {
        if source != queue {
            for wait in view.waits.iter().filter(|wait| wait.batch == batch) {
                if ordering.covers(wait.dependency, dependency)
                    && stages_cover(wait.stages, destination.stages)
                {
                    history.visible[queue.index()] = union(
                        history.visible[queue.index()],
                        AccessScope {
                            stages: wait.stages,
                            access: vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
                            write: true,
                        },
                    );
                    return Ok(());
                }
            }
            return Err(invalid("native access omits a foreign reader/writer wait"));
        }
    } else if history.queue.is_some_and(|source| source != queue) {
        // An externally declared, already-complete scope has no in-flight point.
        return Ok(());
    }
    if history.scope.is_empty() {
        return Ok(());
    }
    Err(invalid(
        "native barrier omits a local reader/writer or host scope",
    ))
}
fn memory_barrier(
    state: &mut State,
    barrier: &BoundTransition,
    target: QueueId,
) -> crate::Result<()> {
    let old = state.pending_layout.unwrap_or(state.layout);
    if !state.known || old != barrier.old_layout {
        return Err(invalid(
            "native barrier old layout does not match its physical range",
        ));
    }
    for history in state.writer.iter_mut().chain(&mut state.readers) {
        if (history.queue == Some(target) || history.queue.is_none())
            && scope_covers(barrier.source, history.scope)
        {
            history.visible[target.index()] =
                union(history.visible[target.index()], barrier.destination);
        }
    }
    state.pending_layout = Some(barrier.new_layout);
    Ok(())
}

/// Verify the actual bound native transitions, waits and state patches. This is
/// an explicit validation operation; default frame recording does not allocate
/// this independent diagnostic model.
pub fn check_bound_synchronization(
    program: &RangeProgram,
    bound: &BoundSynchronization,
) -> crate::Result<()> {
    check(
        program,
        View {
            initial: bound.initial_inputs(),
            batches: bound.batches(),
            order: bound.execution_order(),
            transitions: bound.transitions(),
            patches: bound.patches(),
            waits: bound.waits(),
        },
    )
}

fn check(program: &RangeProgram, view: View<'_>) -> crate::Result<()> {
    validate_program_shape(program)?;
    let ordering = Ordering::new(&view)?;
    let mut states = program
        .resources
        .iter()
        .enumerate()
        .map(|(resource, domain)| {
            let initial = view
                .initial
                .iter()
                .filter(|initial| initial.resource as usize == resource);
            let used = program
                .uses
                .iter()
                .chain(&program.exports)
                .any(|usage| usage.resource as usize == resource);
            if initial.clone().next().is_none() && !used {
                return Ok(None);
            }
            SpatialLedger::new(
                domain.range,
                initial.map(|initial| (initial.cell.range, State::from(initial.cell.state))),
            )
            .map(Some)
        })
        .collect::<crate::Result<Vec<_>>>()?;
    if view
        .initial
        .iter()
        .any(|initial| initial.resource as usize >= states.len())
    {
        return Err(invalid(
            "native initial state references an absent resource",
        ));
    }
    let mut originals = vec![None; program.batches.len()];
    let mut families = [None; MAX_QUEUES];
    for (index, batch) in view.batches.iter().enumerate() {
        let queue = queue(batch.domain.queue_index)?;
        if batch.domain.family == u32::MAX {
            return Err(invalid("native batch has no family"));
        }
        if families[queue.index()]
            .replace(batch.domain.family)
            .is_some_and(|family| family != batch.domain.family)
        {
            return Err(invalid("one native queue is assigned multiple families"));
        }
        if let BoundBatchKind::Graph { original, passes } = &batch.kind {
            let expected = program
                .batches
                .get(*original)
                .ok_or_else(|| invalid("native graph batch is absent"))?;
            if originals[*original].is_some()
                || expected.passes != *passes
                || expected.domain != batch.domain
            {
                return Err(invalid(
                    "native graph batches differ from the static program",
                ));
            }
            originals[*original] = Some(index);
        }
    }
    if originals.iter().any(Option::is_none) {
        return Err(invalid("native binding omits a graph batch"));
    }
    for &(source, destination) in &program.dependencies {
        if !ordering.ancestors[originals[destination].expect("checked graph binding")]
            .contains(&originals[source].expect("checked graph binding"))
        {
            return Err(invalid("native binding omits a static batch dependency"));
        }
    }
    for transition in view.transitions {
        validate_operation(
            program,
            &view,
            transition.batch,
            transition.phase,
            transition.resource,
            transition.range,
        )?;
    }
    for patch in view.patches {
        validate_operation(
            program,
            &view,
            patch.batch,
            patch.phase,
            patch.resource,
            patch.range,
        )?;
    }
    let mut consumed = vec![false; view.transitions.len()];
    for &batch in view.order {
        let mut phases = vec![BarrierPhase::Initial];
        match &view.batches[batch].kind {
            BoundBatchKind::Graph { passes, .. } => {
                phases.extend(passes.clone().map(BarrierPhase::BeforePass));
                phases.push(BarrierPhase::Export);
            }
            BoundBatchKind::OwnershipPrelude => {}
        }
        for phase in phases {
            let expected: &[RangeUse] = match phase {
                BarrierPhase::BeforePass(pass) => &program.uses[program.passes[pass].uses.clone()],
                BarrierPhase::Export
                    if matches!(&view.batches[batch].kind, BoundBatchKind::Graph { original, .. }
                    if *original + 1 == program.batches.len()) =>
                {
                    &program.exports
                }
                _ => &[],
            };
            let mut coverage = program
                .resources
                .iter()
                .map(|domain| SpatialLedger::new(domain.range, [(domain.range, 0usize)]))
                .collect::<crate::Result<Vec<_>>>()?;
            for patch in view
                .patches
                .iter()
                .filter(|patch| patch.batch == batch && patch.phase == phase)
            {
                let resource = patch.resource as usize;
                let target = queue(view.batches[batch].domain.queue_index)?;
                let matches = |barrier: &BoundTransition| {
                    barrier.batch == batch
                        && barrier.phase == phase
                        && barrier.resource == patch.resource
                };
                let mut ownership_barrier = None;
                for (index, barrier) in view.transitions.iter().enumerate() {
                    if consumed[index] || !matches(barrier) {
                        continue;
                    }
                    let selected = match patch.action {
                        BoundPatchAction::Access { layout, .. } => {
                            barrier.kind == BarrierKind::Memory
                                && barrier.new_layout == layout
                                && overlap(barrier.range, patch.range)?
                        }
                        BoundPatchAction::Release(transfer) => {
                            barrier.kind
                                == (BarrierKind::Release {
                                    pair: transfer.pair,
                                })
                                && barrier.range == patch.range
                        }
                        BoundPatchAction::Acquire { transfer, .. } => {
                            barrier.kind
                                == (BarrierKind::Acquire {
                                    pair: transfer.pair,
                                })
                                && barrier.range == patch.range
                        }
                    };
                    if !selected {
                        continue;
                    }
                    match barrier.kind {
                        BarrierKind::Memory => {
                            if !matches!((barrier.source_family, barrier.destination_family),
                                (a,b) if a == b && (a == vk::QUEUE_FAMILY_IGNORED || a == view.batches[batch].domain.family))
                            {
                                return Err(invalid(
                                    "ordinary native barrier changes queue ownership",
                                ));
                            }
                            states[resource]
                                .as_mut()
                                .ok_or_else(|| invalid("native barrier has no physical input"))?
                                .update(barrier.range, |old| {
                                    let mut state = old.clone();
                                    memory_barrier(&mut state, barrier, target)?;
                                    Ok(state)
                                })?;
                        }
                        _ => {
                            if ownership_barrier.replace(barrier).is_some() {
                                return Err(invalid("ownership barrier is duplicated"));
                            }
                        }
                    }
                    consumed[index] = true;
                }
                states[resource]
                    .as_mut()
                    .ok_or_else(|| invalid("native patch has no physical input"))?
                    .update(patch.range, |old| {
                        if old.revision != patch.expected_revision {
                            return Err(invalid("native patch revision is stale"));
                        }
                        let mut state = old.clone();
                        let domain = view.batches[batch].domain;
                        let sharing = program.resources[resource].sharing;
                        match patch.action {
                            BoundPatchAction::Access {
                                scope,
                                layout,
                                queue,
                                family,
                            } => {
                                let owned = match state.owner {
                                    Owner::Unclaimed => true,
                                    Owner::Owned(owner) => owner == family,
                                    Owner::Released { .. } => false,
                                };
                                if queue != target
                                    || family != domain.family
                                    || !sharing.allows_family(family)
                                    || !sharing.is_concurrent() && !owned
                                {
                                    return Err(invalid("native access uses an unacquired family"));
                                }
                                let expected_layout = state.pending_layout.unwrap_or(state.layout);
                                if !state.known || expected_layout != layout {
                                    return Err(invalid(
                                        "native access has no matching layout transition",
                                    ));
                                }
                                for writer in &mut state.writer {
                                    visible(writer, scope, batch, &view, &ordering)?;
                                }
                                let changes = scope.write || state.layout != layout;
                                if changes {
                                    for reader in &mut state.readers {
                                        visible(reader, scope, batch, &view, &ordering)?;
                                    }
                                }
                                if !sharing.is_concurrent() {
                                    state.owner = Owner::Owned(family);
                                }
                                let mut next = History {
                                    queue: Some(queue),
                                    dependency: Some(DependencyRef::Batch(batch)),
                                    scope,
                                    visible: [AccessScope::EMPTY; MAX_QUEUES],
                                };
                                if changes && !scope.write {
                                    next.visible[queue.index()] = scope;
                                }
                                if changes {
                                    state.writer = vec![next];
                                    state.readers.clear();
                                } else if let Some(reader) = state
                                    .readers
                                    .iter_mut()
                                    .find(|reader| reader.queue == Some(queue))
                                {
                                    reader.scope = union(reader.scope, scope);
                                    reader.dependency = Some(DependencyRef::Batch(batch));
                                    reader.visible.fill(AccessScope::EMPTY);
                                } else {
                                    state.readers.push(next);
                                }
                                state.layout = layout;
                                state.pending_layout = None;
                            }
                            BoundPatchAction::Release(transfer) => {
                                let barrier = ownership_barrier.ok_or_else(|| {
                                    invalid("native release has no recorded barrier")
                                })?;
                                transfer_barrier(barrier, transfer, false)?;
                                if sharing.is_concurrent()
                                    || transfer.range != patch.range
                                    || state.owner != Owner::Owned(domain.family)
                                    || domain.family != transfer.source_family
                                    || !state.known
                                    || state.layout != transfer.old_layout
                                {
                                    return Err(invalid(
                                        "native release differs from its physical owner or layout",
                                    ));
                                }
                                memory_barrier(&mut state, barrier, target)?;
                                let destination = if transfer.source.stages.is_empty() {
                                    AccessScope {
                                        stages: vk::PipelineStageFlags::ALL_COMMANDS,
                                        ..AccessScope::EMPTY
                                    }
                                } else {
                                    transfer.source
                                };
                                for history in state.writer.iter_mut().chain(&mut state.readers) {
                                    if history.queue == Some(target) || history.queue.is_none() {
                                        if !scope_covers(barrier.source, history.scope) {
                                            return Err(invalid(
                                                "native release omits local reader/writer scope",
                                            ));
                                        }
                                    } else {
                                        visible(history, destination, batch, &view, &ordering)?;
                                    }
                                }
                                state.owner = Owner::Released {
                                    transfer,
                                    dependency: DependencyRef::Batch(batch),
                                };
                                state.layout = transfer.new_layout;
                                state.pending_layout = None;
                            }
                            BoundPatchAction::Acquire { transfer, release } => {
                                let barrier = ownership_barrier.ok_or_else(|| {
                                    invalid("native acquire has no recorded barrier")
                                })?;
                                transfer_barrier(barrier, transfer, true)?;
                                if sharing.is_concurrent()
                                    || transfer.range != patch.range
                                    || domain.family != transfer.destination_family
                                    || state.owner
                                        != (Owner::Released {
                                            transfer,
                                            dependency: release,
                                        })
                                    || !state.known
                                    || state.layout != transfer.new_layout
                                {
                                    return Err(invalid(
                                        "native acquire does not match its exact release",
                                    ));
                                }
                                if !view.waits.iter().filter(|wait| wait.batch == batch).any(
                                    |wait| {
                                        ordering.covers(wait.dependency, release)
                                            && stages_cover(
                                                wait.stages,
                                                transfer.destination.stages,
                                            )
                                    },
                                ) {
                                    return Err(invalid(
                                        "native acquire omits the actual release receipt wait",
                                    ));
                                }
                                let mut checkpoint = History {
                                    queue: Some(target),
                                    dependency: Some(DependencyRef::Batch(batch)),
                                    scope: transfer.destination,
                                    visible: [AccessScope::EMPTY; MAX_QUEUES],
                                };
                                checkpoint.visible[target.index()] = transfer.destination;
                                state.writer = vec![checkpoint];
                                state.readers.clear();
                                state.owner = Owner::Owned(domain.family);
                                state.layout = transfer.new_layout;
                                state.pending_layout = None;
                            }
                        }
                        state.revision = state
                            .revision
                            .checked_add(1)
                            .ok_or_else(|| invalid("native revision overflows"))?;
                        Ok(state)
                    })?;
                if let BoundPatchAction::Access { scope, layout, .. } = patch.action {
                    let mut matching = SpatialLedger::new(
                        program.resources[resource].range,
                        [(program.resources[resource].range, false)],
                    )?;
                    for expected in expected.iter().filter(|usage| {
                        usage.resource == patch.resource
                            && usage.scope == scope
                            && usage.layout == layout
                    }) {
                        matching.update(expected.range, |_| Ok(true))?;
                    }
                    matching.visit(patch.range, |covered| {
                        if *covered {
                            Ok(())
                        } else {
                            Err(invalid(
                                "native patch changes an undeclared access range or scope",
                            ))
                        }
                    })?;
                    coverage[resource].update(patch.range, |count| {
                        if *count == 0 {
                            Ok(1)
                        } else {
                            Err(invalid("native access range is published more than once"))
                        }
                    })?;
                }
            }
            for expected in expected {
                coverage[expected.resource as usize].visit(expected.range, |count| {
                    if *count == 1 {
                        Ok(())
                    } else {
                        Err(invalid("native binding omits a declared range state patch"))
                    }
                })?;
            }
        }
    }
    if consumed.iter().any(|consumed| !consumed) {
        return Err(invalid("native binding contains an unconsumed barrier"));
    }
    for (state, domain) in states.iter().zip(&program.resources) {
        let Some(state) = state else {
            continue;
        };
        state.visit(domain.range, |state| {
            if state.pending_layout.is_none() {
                Ok(())
            } else {
                Err(invalid(
                    "native barrier layout is absent from final state patches",
                ))
            }
        })?;
    }
    Ok(())
}

fn transfer_barrier(
    barrier: &BoundTransition,
    transfer: RangeTransfer,
    acquire: bool,
) -> crate::Result<()> {
    if transfer.pair == 0
        || transfer.source_family == u32::MAX
        || transfer.destination_family == u32::MAX
        || transfer.source_family == transfer.destination_family
        || barrier.source_family != transfer.source_family
        || barrier.destination_family != transfer.destination_family
        || barrier.old_layout != transfer.old_layout
        || barrier.new_layout != transfer.new_layout
        || barrier.range != transfer.range
        || if acquire {
            !scope_covers(barrier.destination, transfer.destination)
        } else {
            !scope_covers(barrier.source, transfer.source)
        }
    {
        return Err(invalid(
            "native release/acquire barriers disagree on the transfer pair",
        ));
    }
    Ok(())
}
fn validate_operation(
    program: &RangeProgram,
    view: &View<'_>,
    batch: usize,
    phase: BarrierPhase,
    resource: u32,
    range: OwnershipRange,
) -> crate::Result<()> {
    let domain = program
        .resources
        .get(resource as usize)
        .ok_or_else(|| invalid("native operation references an absent resource"))?;
    if !contains(domain.range, range)? {
        return Err(invalid("native operation exceeds resource bounds"));
    }
    match (view.batches.get(batch).map(|batch| &batch.kind), phase) {
        (Some(BoundBatchKind::OwnershipPrelude), BarrierPhase::Initial) => Ok(()),
        (Some(BoundBatchKind::Graph { passes, .. }), BarrierPhase::BeforePass(pass))
            if passes.contains(&pass) =>
        {
            Ok(())
        }
        (Some(BoundBatchKind::Graph { .. }), BarrierPhase::Initial | BarrierPhase::Export) => {
            Ok(())
        }
        _ => Err(invalid("native operation is assigned to an absent phase")),
    }
}
fn overlap(a: OwnershipRange, b: OwnershipRange) -> crate::Result<bool> {
    super::range_program::ranges_overlap(a, b)
}

fn validate_program_shape(program: &RangeProgram) -> crate::Result<()> {
    for resource in &program.resources {
        if !contains(resource.range, resource.range)? {
            return Err(invalid("native resource domain is invalid"));
        }
    }
    let mut cursor = 0;
    for (index, pass) in program.passes.iter().enumerate() {
        if pass.pass != index
            || pass.uses.start != cursor
            || pass.uses.end < cursor
            || pass.uses.end > program.uses.len()
        {
            return Err(invalid("native program pass slices are invalid"));
        }
        cursor = pass.uses.end;
    }
    if cursor != program.uses.len() {
        return Err(invalid("native program has unscheduled uses"));
    }
    let mut cursor = 0;
    for batch in &program.batches {
        queue(batch.domain.queue_index)?;
        if batch.passes.start != cursor
            || batch.passes.end < cursor
            || batch.passes.end > program.passes.len()
            || batch.domain.family == u32::MAX
        {
            return Err(invalid("native program batches are invalid"));
        }
        cursor = batch.passes.end;
    }
    if cursor != program.passes.len() {
        return Err(invalid("native program omits scheduled passes"));
    }
    for usage in program.uses.iter().chain(&program.exports) {
        if matches!(usage.range, OwnershipRange::Image { .. })
            && matches!(
                usage.layout,
                vk::ImageLayout::UNDEFINED | vk::ImageLayout::PREINITIALIZED
            )
        {
            return Err(invalid(
                "native image use has an initial-only target layout",
            ));
        }
        let domain = program
            .resources
            .get(usage.resource as usize)
            .ok_or_else(|| invalid("native program use has no resource"))?;
        if !contains(domain.range, usage.range)? {
            return Err(invalid("native program use leaves its domain"));
        }
    }
    for &(source, destination) in &program.dependencies {
        if source >= destination || destination >= program.batches.len() {
            return Err(invalid("native program dependency is invalid"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        raw::allocator::AllocatorStats,
        resource::{QueueFamilies, ResourceSharing, range_state::RangeCell},
        rg::{
            queue_schedule::QueueDomain,
            sync_plan::{
                ProgramBatch, ProgramPass, ResourceDomain, SynchronizationConfig,
                SynchronizationPlanner,
            },
        },
        sync::SyncPoint,
    };
    use std::sync::Arc;

    #[derive(Clone)]
    struct Capture {
        initial: Vec<InitialState>,
        batches: Vec<BoundBatch>,
        order: Vec<usize>,
        transitions: Vec<BoundTransition>,
        patches: Vec<BoundPatch>,
        waits: Vec<BoundWait>,
    }
    impl Capture {
        fn check(&self, program: &RangeProgram) -> crate::Result<()> {
            check(
                program,
                View {
                    initial: &self.initial,
                    batches: &self.batches,
                    order: &self.order,
                    transitions: &self.transitions,
                    patches: &self.patches,
                    waits: &self.waits,
                },
            )
        }
    }
    fn capture(program: &RangeProgram, initial: &[RangeCell]) -> Capture {
        let mut planner = SynchronizationPlanner::new(
            Arc::new(AllocatorStats::default()),
            SynchronizationConfig::default(),
        )
        .unwrap();
        planner.add_initial(0, initial).unwrap();
        planner
            .bind(program, |family| {
                Ok(QueueDomain {
                    queue_index: family,
                    family,
                })
            })
            .unwrap();
        check_bound_synchronization(program, planner.bound()).unwrap();
        let bound = planner.bound();
        Capture {
            initial: bound.initial_inputs().to_vec(),
            batches: bound.batches().to_vec(),
            order: bound.execution_order().to_vec(),
            transitions: bound.transitions().to_vec(),
            patches: bound.patches().to_vec(),
            waits: bound.waits().to_vec(),
        }
    }
    fn point(queue: usize, value: u64) -> SyncPoint {
        SyncPoint {
            queue: QueueId::new(queue).unwrap(),
            value,
        }
    }
    fn scope(stages: vk::PipelineStageFlags, write: bool) -> AccessScope {
        AccessScope {
            stages,
            access: if write {
                vk::AccessFlags::SHADER_WRITE
            } else {
                vk::AccessFlags::SHADER_READ
            },
            write,
        }
    }
    fn program(
        range: OwnershipRange,
        sharing: ResourceSharing,
        entries: &[(u32, AccessScope, vk::ImageLayout)],
    ) -> RangeProgram {
        RangeProgram {
            resources: vec![ResourceDomain { range, sharing }],
            uses: entries
                .iter()
                .map(|(_, scope, layout)| RangeUse {
                    resource: 0,
                    range,
                    scope: *scope,
                    layout: *layout,
                    access_code: 0x22,
                })
                .collect(),
            passes: (0..entries.len())
                .map(|pass| ProgramPass {
                    pass,
                    uses: pass..pass + 1,
                })
                .collect(),
            batches: entries
                .iter()
                .enumerate()
                .map(|(index, (queue, _, _))| ProgramBatch {
                    domain: QueueDomain {
                        queue_index: *queue,
                        family: *queue,
                    },
                    passes: index..index + 1,
                })
                .collect(),
            ..RangeProgram::default()
        }
    }
    fn initial(range: OwnershipRange, queue: usize, layout: vk::ImageLayout) -> RangeCell {
        let mut state = NativeState {
            layout,
            owner: RangeOwner::Owned(queue as u32),
            ..NativeState::default()
        };
        state.writer.scopes[queue] = scope(vk::PipelineStageFlags::COMPUTE_SHADER, true);
        state.writer.points.insert(point(queue, 9));
        RangeCell { range, state }
    }
    #[test]
    fn local_reader_stage_union_is_required_and_previous_visibility_can_remove_redundant_barriers()
    {
        let range = OwnershipRange::Buffer {
            offset: 0,
            size: 16,
        };
        let make = |first| {
            program(
                range,
                ResourceSharing::Exclusive,
                &[
                    (0, scope(first, false), vk::ImageLayout::UNDEFINED),
                    (
                        0,
                        scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
                        vk::ImageLayout::UNDEFINED,
                    ),
                    (
                        0,
                        scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
                        vk::ImageLayout::UNDEFINED,
                    ),
                ],
            )
        };
        let graph = make(vk::PipelineStageFlags::VERTEX_SHADER);
        let baseline = capture(&graph, &[initial(range, 0, vk::ImageLayout::UNDEFINED)]);
        let mut broken = baseline.clone();
        let last = broken
            .transitions
            .iter_mut()
            .find(|barrier| barrier.phase == BarrierPhase::BeforePass(2))
            .unwrap();
        last.source.stages &= !vk::PipelineStageFlags::FRAGMENT_SHADER;
        assert!(broken.check(&graph).is_err());

        let graph = make(vk::PipelineStageFlags::ALL_COMMANDS);
        let mut optimized = capture(&graph, &[initial(range, 0, vk::ImageLayout::UNDEFINED)]);
        optimized
            .transitions
            .retain(|barrier| barrier.phase != BarrierPhase::BeforePass(1));
        optimized.check(&graph).unwrap();
    }
    #[test]
    fn every_foreign_reader_wait_and_its_destination_stages_are_checked() {
        let range = OwnershipRange::Buffer {
            offset: 0,
            size: 16,
        };
        let graph = program(
            range,
            ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1, 2]).unwrap()),
            &[
                (
                    1,
                    scope(vk::PipelineStageFlags::VERTEX_SHADER, false),
                    vk::ImageLayout::UNDEFINED,
                ),
                (
                    2,
                    scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
                    vk::ImageLayout::UNDEFINED,
                ),
                (
                    0,
                    scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
                    vk::ImageLayout::UNDEFINED,
                ),
            ],
        );
        let source = capture(&graph, &[initial(range, 0, vk::ImageLayout::UNDEFINED)]);
        for reader in [0, 1] {
            let mut broken = source.clone();
            broken.waits.retain(|wait| {
                !(wait.batch == 2 && wait.dependency == DependencyRef::Batch(reader))
            });
            assert!(broken.check(&graph).is_err());
        }
        let mut broken = source.clone();
        broken
            .waits
            .iter_mut()
            .filter(|wait| wait.batch == 2)
            .for_each(|wait| wait.stages = vk::PipelineStageFlags::FRAGMENT_SHADER);
        assert!(broken.check(&graph).is_err());
        let mut broken = source;
        broken.order.swap(0, 2);
        assert!(broken.check(&graph).is_err());
    }
    #[test]
    fn mixed_mip_layouts_missing_patch_ranges_and_stale_revisions_are_rejected() {
        let whole = OwnershipRange::Image {
            aspects: 1,
            base_mip: 0,
            mip_count: 2,
            base_layer: 0,
            layer_count: 2,
        };
        let graph = program(
            whole,
            ResourceSharing::Exclusive,
            &[(
                0,
                scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            )],
        );
        let mip = |base_mip| OwnershipRange::Image {
            base_mip,
            mip_count: 1,
            aspects: 1,
            base_layer: 0,
            layer_count: 2,
        };
        let capture = capture(
            &graph,
            &[
                initial(mip(0), 0, vk::ImageLayout::GENERAL),
                initial(mip(1), 0, vk::ImageLayout::TRANSFER_SRC_OPTIMAL),
            ],
        );
        assert_eq!(capture.transitions.len(), 2);
        let mut broken = capture.clone();
        broken.transitions[0].old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
        assert!(broken.check(&graph).is_err());
        let mut broken = capture.clone();
        broken.patches.pop();
        assert!(broken.check(&graph).is_err());
        let mut broken = capture;
        broken.patches[1].expected_revision += 1;
        assert!(broken.check(&graph).is_err());
    }
    #[test]
    fn failed_acquire_recovery_requires_original_pair_before_releasing_to_a_new_family() {
        let range = OwnershipRange::Buffer {
            offset: 0,
            size: 16,
        };
        let graph = program(
            range,
            ResourceSharing::Exclusive,
            &[(
                2,
                scope(vk::PipelineStageFlags::COMPUTE_SHADER, false),
                vk::ImageLayout::UNDEFINED,
            )],
        );
        let transfer = RangeTransfer {
            pair: 17,
            source_family: 0,
            destination_family: 1,
            range,
            source: scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
            destination: scope(vk::PipelineStageFlags::COMPUTE_SHADER, false),
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::UNDEFINED,
        };
        let mut released = initial(range, 0, vk::ImageLayout::UNDEFINED);
        released.state.owner = RangeOwner::Released {
            transfer,
            point: point(0, 10),
        };
        released.state.revision = 7;
        let source = capture(&graph, &[released]);
        assert!(source.batches.len() >= 3);
        assert_ne!(source.order[0], 0);
        let mut broken = source.clone();
        broken
            .transitions
            .retain(|barrier| barrier.kind != BarrierKind::Acquire { pair: 17 });
        assert!(broken.check(&graph).is_err());
        let mut broken = source.clone();
        let acquire = broken
            .patches
            .iter_mut()
            .find(|patch| {
                patch.batch == 0 && matches!(patch.action, BoundPatchAction::Acquire { .. })
            })
            .unwrap();
        let BoundPatchAction::Acquire { transfer, .. } = acquire.action else {
            unreachable!()
        };
        acquire.action = BoundPatchAction::Acquire {
            transfer,
            release: DependencyRef::Point(point(0, 10)),
        };
        assert!(broken.check(&graph).is_err());
        let mut broken = source;
        broken
            .waits
            .retain(|wait| wait.dependency != DependencyRef::Point(point(0, 10)));
        assert!(broken.check(&graph).is_err());
    }
}
