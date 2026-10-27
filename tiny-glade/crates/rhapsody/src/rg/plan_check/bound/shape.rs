//! Structural checks of the program and the native binding before replay.
use ash::vk;
use crate::rg::sync_plan::AliasEdge;
use crate::rg::sync_plan::BarrierPhase;
use crate::rg::sync_plan::BoundBatch;
use crate::rg::sync_plan::BoundBatchKind;
use crate::rg::sync_plan::RangeProgram;
use crate::sync::MAX_QUEUES;
use crate::tracking::OwnershipRange;
use super::model::Ledgers;
use super::model::Ordering;
use super::model::State;
use super::model::View;
use super::model::queue;
use super::super::invalid;
use super::super::range_oracle::SpatialLedger;
use super::super::range_oracle::contains;

/// One ledger per resource that is initialised or used; absent otherwise.
pub(super) fn initial_states(program: &RangeProgram, view: &View<'_>) -> crate::Result<Ledgers> {
    let states = program
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
    Ok(states)
}

/// Native index of every static graph batch, after checking queue families and
/// that the bound graph batches replicate the static program exactly.
pub(super) fn match_graph_batches(program: &RangeProgram, view: &View<'_>) -> crate::Result<Vec<usize>> {
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
    originals
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| invalid("native binding omits a graph batch"))
}

pub(super) fn check_static_dependencies(
    program: &RangeProgram,
    ordering: &Ordering,
    originals: &[usize],
) -> crate::Result<()> {
    for &(source, destination) in &program.dependencies {
        if !ordering.ancestors[originals[destination]].contains(&originals[source]) {
            return Err(invalid("native binding omits a static batch dependency"));
        }
    }
    Ok(())
}

pub(super) fn check_alias_edges(states: &Ledgers, aliases: &[AliasEdge]) -> crate::Result<()> {
    for edge in aliases {
        if states
            .get(edge.previous as usize)
            .is_none_or(Option::is_none)
            || states.get(edge.next as usize).is_none_or(Option::is_none)
            || edge.previous == edge.next
        {
            return Err(invalid("native alias edge names an absent resource"));
        }
    }
    Ok(())
}

/// The barrier phases a batch is replayed through, in recording order.
pub(super) fn batch_phases(batch: &BoundBatch) -> Vec<BarrierPhase> {
    let mut phases = vec![BarrierPhase::Initial];
    match &batch.kind {
        BoundBatchKind::Graph { passes, .. } => {
            phases.extend(passes.clone().map(BarrierPhase::BeforePass));
            phases.push(BarrierPhase::Export);
        }
        BoundBatchKind::OwnershipPrelude => {}
    }
    phases
}

pub(super) fn check_no_pending_layout(states: &Ledgers, program: &RangeProgram) -> crate::Result<()> {
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

pub(super) fn validate_operation(
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

pub(super) fn overlap(a: OwnershipRange, b: OwnershipRange) -> crate::Result<bool> {
    super::super::range_program::ranges_overlap(a, b)
}

pub(super) fn validate_program_shape(program: &RangeProgram) -> crate::Result<()> {
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
