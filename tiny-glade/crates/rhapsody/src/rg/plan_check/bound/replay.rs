//! Replay of native patches and barriers against the independent model.
use ash::vk;
use crate::native::AccessScope;
use crate::rg::queue_schedule::QueueDomain;
use crate::rg::sync_plan::BarrierKind;
use crate::rg::sync_plan::BarrierPhase;
use crate::rg::sync_plan::BoundBatchKind;
use crate::rg::sync_plan::BoundPatch;
use crate::rg::sync_plan::BoundPatchAction;
use crate::rg::sync_plan::BoundTransition;
use crate::rg::sync_plan::DependencyRef;
use crate::rg::sync_plan::RangeProgram;
use crate::rg::sync_plan::RangeUse;
use crate::sync::MAX_QUEUES;
use crate::sync::QueueId;
use crate::tracking::ResourceSharing;
use crate::tracking::range_state::NativeRange;
use crate::tracking::range_state::RangeTransfer;
use super::model::History;
use super::model::Ledgers;
use super::model::Ordering;
use super::model::Owner;
use super::model::State;
use super::model::View;
use super::model::discard_alias_contents;
use super::model::memory_barrier;
use super::model::queue;
use super::model::transfer_barrier;
use super::model::visible;
use super::scope::scope_covers;
use super::scope::union;
use super::scope::wait_stages_cover;
use super::shape::overlap;
use super::super::invalid;
use super::super::range_oracle::SpatialLedger;

/// Read-only inputs shared by every replay step.
pub(super) struct Replay<'a> {
    pub(super) program: &'a RangeProgram,
    pub(super) view: &'a View<'a>,
    pub(super) ordering: &'a Ordering,
}

/// One state patch being replayed, with the facts every action needs.
pub(super) struct PatchStep<'a> {
    batch: usize,
    target: QueueId,
    patch: &'a BoundPatch,
    barrier: Option<&'a BoundTransition>,
    domain: QueueDomain,
    sharing: ResourceSharing,
}

impl<'a> Replay<'a> {
    /// The range uses a phase of a batch declares (none for ownership phases).
    fn expected_uses(&self, batch: usize, phase: BarrierPhase) -> &'a [RangeUse] {
        let program = self.program;
        match phase {
            BarrierPhase::BeforePass(pass) => &program.uses[program.passes[pass].uses.clone()],
            BarrierPhase::Export
                if matches!(&self.view.batches[batch].kind, BoundBatchKind::Graph { original, .. }
                if *original + 1 == program.batches.len()) =>
            {
                &program.exports
            }
            _ => &[],
        }
    }

    pub(super) fn replay_phase(
        &self,
        states: &mut Ledgers,
        consumed: &mut [bool],
        batch: usize,
        phase: BarrierPhase,
    ) -> crate::Result<()> {
        let expected = self.expected_uses(batch, phase);
        let mut coverage = self
            .program
            .resources
            .iter()
            .map(|domain| SpatialLedger::new(domain.range, [(domain.range, 0usize)]))
            .collect::<crate::Result<Vec<_>>>()?;
        for patch in self
            .view
            .patches
            .iter()
            .filter(|patch| patch.batch == batch && patch.phase == phase)
        {
            self.apply_patch(states, consumed, batch, phase, patch)?;
            if let BoundPatchAction::Access { scope, layout, .. } = patch.action {
                self.check_declared_access(&mut coverage, expected, patch, scope, layout)?;
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
        Ok(())
    }

    fn apply_patch(
        &self,
        states: &mut Ledgers,
        consumed: &mut [bool],
        batch: usize,
        phase: BarrierPhase,
        patch: &'a BoundPatch,
    ) -> crate::Result<()> {
        let resource = patch.resource as usize;
        let domain = self.view.batches[batch].domain;
        let target = queue(domain.queue_index)?;
        if matches!(patch.action, BoundPatchAction::Access { .. }) {
            discard_alias_contents(states, self.view.aliases, patch)?;
        }
        let barrier = self.consume_barriers(states, consumed, batch, phase, patch, target)?;
        let step = PatchStep {
            batch,
            target,
            patch,
            barrier,
            domain,
            sharing: self.program.resources[resource].sharing,
        };
        states[resource]
            .as_mut()
            .ok_or_else(|| invalid("native patch has no physical input"))?
            .update(patch.range, |old| {
                if old.revision != patch.expected_revision {
                    return Err(invalid("native patch revision is stale"));
                }
                let mut state = old.clone();
                self.apply_action(&mut state, &step)?;
                state.revision = state
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| invalid("native revision overflows"))?;
                Ok(state)
            })
    }

    /// Marks the transitions a patch selects as consumed and applies the ordinary
    /// memory barriers; returns the ownership barrier (release/acquire) if any.
    fn consume_barriers(
        &self,
        states: &mut Ledgers,
        consumed: &mut [bool],
        batch: usize,
        phase: BarrierPhase,
        patch: &BoundPatch,
        target: QueueId,
    ) -> crate::Result<Option<&'a BoundTransition>> {
        let resource = patch.resource as usize;
        let mut ownership_barrier = None;
        for (index, barrier) in self.view.transitions.iter().enumerate() {
            let matches_patch = barrier.batch == batch
                && barrier.phase == phase
                && barrier.resource == patch.resource;
            if consumed[index] || !matches_patch {
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
                    let family = self.view.batches[batch].domain.family;
                    if !matches!((barrier.source_family, barrier.destination_family),
                        (a,b) if a == b && (a == vk::QUEUE_FAMILY_IGNORED || a == family))
                    {
                        return Err(invalid("ordinary native barrier changes queue ownership"));
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
        Ok(ownership_barrier)
    }

    fn apply_action(&self, state: &mut State, step: &PatchStep<'_>) -> crate::Result<()> {
        match step.patch.action {
            BoundPatchAction::Access {
                scope,
                layout,
                queue,
                family,
            } => self.apply_access(state, step, scope, layout, queue, family),
            BoundPatchAction::Release(transfer) => self.apply_release(state, step, transfer),
            BoundPatchAction::Acquire { transfer, release } => {
                self.apply_acquire(state, step, transfer, release)
            }
        }
    }

    fn apply_access(
        &self,
        state: &mut State,
        step: &PatchStep<'_>,
        scope: AccessScope,
        layout: vk::ImageLayout,
        queue: QueueId,
        family: u32,
    ) -> crate::Result<()> {
        let (batch, patch, sharing) = (step.batch, step.patch, step.sharing);
        let owned = match state.owner {
            Owner::Unclaimed => true,
            Owner::Owned(owner) => {
                owner == family
                    // Undefined image contents have no owner to
                    // hand over: any family may claim them.
                    || matches!(patch.range, NativeRange::Image { .. })
                        && state.known
                        && state.layout == vk::ImageLayout::UNDEFINED
            }
            Owner::Released { .. } => false,
        };
        if queue != step.target
            || family != step.domain.family
            || !sharing.allows_family(family)
            || !sharing.is_concurrent() && !owned
        {
            return Err(invalid("native access uses an unacquired family"));
        }
        let expected_layout = state.pending_layout.unwrap_or(state.layout);
        if !state.known || expected_layout != layout {
            return Err(invalid("native access has no matching layout transition"));
        }
        for writer in &mut state.writer {
            visible(writer, scope, batch, self.view, self.ordering)?;
        }
        let changes = scope.write || state.layout != layout;
        if changes {
            for reader in &mut state.readers {
                visible(reader, scope, batch, self.view, self.ordering)?;
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
        Ok(())
    }

    fn apply_release(
        &self,
        state: &mut State,
        step: &PatchStep<'_>,
        transfer: RangeTransfer,
    ) -> crate::Result<()> {
        let (batch, patch, target, domain) = (step.batch, step.patch, step.target, step.domain);
        let barrier = step
            .barrier
            .ok_or_else(|| invalid("native release has no recorded barrier"))?;
        transfer_barrier(barrier, transfer, false)?;
        if step.sharing.is_concurrent()
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
        memory_barrier(state, barrier, target)?;
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
                    return Err(invalid("native release omits local reader/writer scope"));
                }
            } else {
                visible(history, destination, batch, self.view, self.ordering)?;
            }
        }
        state.owner = Owner::Released {
            transfer,
            dependency: DependencyRef::Batch(batch),
        };
        state.layout = transfer.new_layout;
        state.pending_layout = None;
        Ok(())
    }

    fn apply_acquire(
        &self,
        state: &mut State,
        step: &PatchStep<'_>,
        transfer: RangeTransfer,
        release: DependencyRef,
    ) -> crate::Result<()> {
        let (batch, patch, target, domain) = (step.batch, step.patch, step.target, step.domain);
        let barrier = step
            .barrier
            .ok_or_else(|| invalid("native acquire has no recorded barrier"))?;
        transfer_barrier(barrier, transfer, true)?;
        if step.sharing.is_concurrent()
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
            return Err(invalid("native acquire does not match its exact release"));
        }
        if !self
            .view
            .waits
            .iter()
            .filter(|wait| wait.batch == batch)
            .any(|wait| {
                self.ordering.covers(wait.dependency, release)
                    && wait_stages_cover(wait.stages, transfer.destination.stages)
            })
        {
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
        Ok(())
    }

    /// An access patch must lie inside a declared use and be published exactly once.
    fn check_declared_access(
        &self,
        coverage: &mut [SpatialLedger<usize>],
        expected: &[RangeUse],
        patch: &BoundPatch,
        scope: AccessScope,
        layout: vk::ImageLayout,
    ) -> crate::Result<()> {
        let resource = patch.resource as usize;
        let domain = self.program.resources[resource].range;
        let mut matching = SpatialLedger::new(domain, [(domain, false)])?;
        for expected in expected.iter().filter(|usage| {
            usage.resource == patch.resource && usage.scope == scope && usage.layout == layout
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
        })
    }
}
