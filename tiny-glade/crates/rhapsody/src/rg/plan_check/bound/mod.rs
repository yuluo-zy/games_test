//! Explicit cold-path verification of the native binding consumed by recording.
//! This replays independent spatial cells; it never invokes the binder or its
//! production range splitting/state-transition implementation.
use super::invalid;
use crate::rg::sync_plan::{BoundSynchronization, RangeProgram};
mod scope;
mod model;
mod shape;
mod replay;
use model::{View, Ordering};
use shape::{initial_states, match_graph_batches, check_static_dependencies, check_alias_edges, batch_phases, check_no_pending_layout, validate_operation, validate_program_shape};
use replay::Replay;

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
            aliases: bound.aliases(),
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
    let mut states = initial_states(program, &view)?;
    let originals = match_graph_batches(program, &view)?;
    check_static_dependencies(program, &ordering, &originals)?;
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
    check_alias_edges(&states, view.aliases)?;
    let replay = Replay {
        program,
        view: &view,
        ordering: &ordering,
    };
    let mut consumed = vec![false; view.transitions.len()];
    for &batch in view.order {
        for phase in batch_phases(&view.batches[batch]) {
            replay.replay_phase(&mut states, &mut consumed, batch, phase)?;
        }
    }
    if consumed.iter().any(|consumed| !consumed) {
        return Err(invalid("native binding contains an unconsumed barrier"));
    }
    check_no_pending_layout(&states, program)
}

#[cfg(test)]
mod tests;
