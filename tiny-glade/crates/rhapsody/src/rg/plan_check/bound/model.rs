//! Independent state model: histories, owners, ordering and barrier effects.
use ash::vk;
use crate::native::AccessScope;
use crate::rg::sync_plan::AliasEdge;
use crate::rg::sync_plan::BoundBatch;
use crate::rg::sync_plan::BoundPatch;
use crate::rg::sync_plan::BoundTransition;
use crate::rg::sync_plan::BoundWait;
use crate::rg::sync_plan::DependencyRef;
use crate::rg::sync_plan::InitialState;
use crate::sync::MAX_QUEUES;
use crate::sync::QueueId;
use crate::sync::SyncSet;
use crate::tracking::range_state::Frontier;
use crate::tracking::range_state::NativeState;
use crate::tracking::range_state::RangeOwner;
use crate::tracking::range_state::RangeTransfer;
use std::collections::BTreeSet;
use super::scope::scope_covers;
use super::scope::union;
use super::scope::wait_stages_cover;
use super::super::invalid;
use super::super::range_oracle::SpatialLedger;

#[derive(Clone)]
pub(super) struct History {
    pub(super) queue: Option<QueueId>,
    pub(super) dependency: Option<DependencyRef>,
    pub(super) scope: AccessScope,
    pub(super) visible: [AccessScope; MAX_QUEUES],
}

pub(super) fn history(frontier: Frontier) -> Vec<History> {
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
pub(super) enum Owner {
    Unclaimed,
    Owned(u32),
    Released {
        transfer: RangeTransfer,
        dependency: DependencyRef,
    },
}

#[derive(Clone)]
pub(super) struct State {
    pub(super) revision: u64,
    pub(super) layout: vk::ImageLayout,
    pub(super) known: bool,
    pub(super) pending_layout: Option<vk::ImageLayout>,
    pub(super) owner: Owner,
    pub(super) writer: Vec<History>,
    pub(super) readers: Vec<History>,
    /// No access patch has touched this cell yet in the bound program.
    fresh: bool,
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
            fresh: true,
        }
    }
}

pub(super) struct View<'a> {
    pub(super) initial: &'a [InitialState],
    pub(super) aliases: &'a [AliasEdge],
    pub(super) batches: &'a [BoundBatch],
    pub(super) order: &'a [usize],
    pub(super) transitions: &'a [BoundTransition],
    pub(super) patches: &'a [BoundPatch],
    pub(super) waits: &'a [BoundWait],
}

pub(super) struct Ordering {
    pub(super) ancestors: Vec<BTreeSet<usize>>,
    points: Vec<SyncSet>,
}

impl Ordering {
    pub(super) fn new(view: &View<'_>) -> crate::Result<Self> {
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
                DependencyRef::Point(point) if point.value == 0 || point.value > known.get(point.queue) => {
                    return Err(invalid("native wait uses an unknown or future point"));
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
    pub(super) fn covers(&self, provided: DependencyRef, required: DependencyRef) -> bool {
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

pub(super) type Ledgers = Vec<Option<SpatialLedger<State>>>;

pub(super) fn queue(index: u32) -> crate::Result<QueueId> {
    usize::try_from(index)
        .ok()
        .and_then(QueueId::new)
        .ok_or_else(|| invalid("native binding queue index exceeds its domain"))
}

pub(super) fn visible(
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
                    && wait_stages_cover(wait.stages, destination.stages)
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

pub(super) fn memory_barrier(
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

/// The first access to a cell of an alias member discards its contents: every
/// history of the members it reuses (and its own) must be made visible again,
/// starting from an UNDEFINED layout.
pub(super) fn discard_alias_contents(
    states: &mut [Option<SpatialLedger<State>>],
    aliases: &[AliasEdge],
    patch: &BoundPatch,
) -> crate::Result<()> {
    if !aliases.iter().any(|edge| edge.next == patch.resource) {
        return Ok(());
    }
    let mut inherited = Vec::new();
    for edge in aliases.iter().filter(|edge| edge.next == patch.resource) {
        let previous = states[edge.previous as usize]
            .as_ref()
            .ok_or_else(|| invalid("native alias edge names an absent resource"))?;
        for state in previous.states() {
            inherited.extend(state.writer.iter().chain(&state.readers).cloned());
        }
    }
    states[patch.resource as usize]
        .as_mut()
        .ok_or_else(|| invalid("native patch has no physical input"))?
        .update(patch.range, |old| {
            if !old.fresh {
                return Ok(old.clone());
            }
            let mut state = old.clone();
            let own = std::mem::take(&mut state.readers);
            state.writer.extend(own);
            state.writer.extend(inherited.iter().cloned());
            for history in &mut state.writer {
                history.visible = [AccessScope::EMPTY; MAX_QUEUES];
            }
            state.layout = vk::ImageLayout::UNDEFINED;
            state.known = true;
            state.pending_layout = None;
            state.fresh = false;
            Ok(state)
        })
}

pub(super) fn transfer_barrier(
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
