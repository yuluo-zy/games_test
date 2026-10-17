//! Consumes the one bound native range plan. This recorder schedules callbacks
//! and command boundaries; it never reconstructs synchronization from access codes.
use super::sync_plan::{BarrierPhase, BoundTransition, SynchronizationPlanner};
pub use crate::command::recording::{Command, CommandState};
use std::sync::Arc;
#[derive(Clone, Copy, Debug)]
pub struct Pass {
    pub callback_present: bool,
}
pub type PlannedTransition = BoundTransition;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    pub barrier_commands: u32,
    pub barrier_transitions: u32,
    pub panic: bool,
    pub consumed_passes: usize,
}
pub struct Recorder {
    pub synchronization: Arc<SynchronizationPlanner>,
    pub batch: usize,
    pub first_pass: usize,
    pub commands: smallvec::SmallVec<[Command; 8]>,
}
#[derive(Clone, Copy, Debug)]
pub enum RecordAction {
    Pass(usize),
    End(u64),
    Transition {
        command: u64,
        transition: BoundTransition,
    },
}
impl Recorder {
    fn phase(&self, phase: BarrierPhase) -> impl Iterator<Item = BoundTransition> + '_ {
        self.synchronization
            .bound()
            .transitions()
            .iter()
            .copied()
            .filter(move |transition| transition.batch == self.batch && transition.phase == phase)
    }
    fn emit(
        &self,
        phase: BarrierPhase,
        command: u64,
        out: &mut Outcome,
        boundary: &mut impl FnMut(RecordAction) -> bool,
    ) -> bool {
        let mut count = 0;
        for transition in self.phase(phase) {
            if !boundary(RecordAction::Transition {
                command,
                transition,
            }) {
                return false;
            }
            count += 1;
        }
        out.barrier_transitions += count;
        out.barrier_commands += u32::from(count != 0);
        true
    }
    pub fn record_batch(
        &mut self,
        passes: &mut [Pass],
        final_batch: bool,
        command: Command,
        mut boundary: impl FnMut(RecordAction) -> bool,
    ) -> Outcome {
        let mut out = Outcome::default();
        let Some(initial) = self.commands.last() else {
            out.panic = true;
            return out;
        };
        if !self.emit(
            BarrierPhase::Initial,
            initial.handle,
            &mut out,
            &mut boundary,
        ) {
            out.panic = true;
            return out;
        }
        for (index, pass) in passes.iter_mut().enumerate() {
            if !self.emit(
                BarrierPhase::BeforePass(self.first_pass + index),
                command.handle,
                &mut out,
                &mut boundary,
            ) || !std::mem::take(&mut pass.callback_present)
            {
                out.panic = true;
                return out;
            }
            out.consumed_passes += 1;
            if !boundary(RecordAction::Pass(index)) {
                out.panic = true;
                return out;
            }
        }
        if !self.finish(
            &mut out,
            std::iter::once(command),
            final_batch,
            &mut boundary,
        ) {
            out.panic = true;
        }
        out
    }
    pub fn plan_chunked_batch(
        &mut self,
        passes: &mut [Pass],
        commands: &[Command],
        chunk_of_pass: &[usize],
        mut boundary: impl FnMut(RecordAction) -> bool,
    ) -> (Vec<Vec<PlannedTransition>>, Outcome) {
        let mut out = Outcome::default();
        let mut plan = Vec::with_capacity(passes.len());
        if chunk_of_pass.len() != passes.len() {
            out.panic = true;
            return (plan, out);
        }
        let Some(initial) = self.commands.last() else {
            out.panic = true;
            return (plan, out);
        };
        if !self.emit(
            BarrierPhase::Initial,
            initial.handle,
            &mut out,
            &mut boundary,
        ) {
            out.panic = true;
            return (plan, out);
        }
        for (index, pass) in passes.iter_mut().enumerate() {
            if commands.get(chunk_of_pass[index]).is_none()
                || !std::mem::take(&mut pass.callback_present)
            {
                out.panic = true;
                return (plan, out);
            }
            let transitions = self
                .phase(BarrierPhase::BeforePass(self.first_pass + index))
                .collect::<Vec<_>>();
            out.barrier_commands += u32::from(!transitions.is_empty());
            out.barrier_transitions += transitions.len() as u32;
            out.consumed_passes += 1;
            plan.push(transitions);
        }
        (plan, out)
    }
    pub fn finish_chunked_batch(
        &mut self,
        out: &mut Outcome,
        commands: Vec<Command>,
        final_batch: bool,
        mut boundary: impl FnMut(RecordAction) -> bool,
    ) {
        if !self.finish(out, commands, final_batch, &mut boundary) {
            out.panic = true;
        }
    }
    fn finish(
        &mut self,
        out: &mut Outcome,
        commands: impl IntoIterator<Item = Command>,
        final_batch: bool,
        boundary: &mut impl FnMut(RecordAction) -> bool,
    ) -> bool {
        for command in commands {
            if self
                .commands
                .last()
                .is_none_or(|last| last.handle != command.handle)
            {
                self.commands.push(command);
            }
        }
        let len = self.commands.len();
        if len == 0 {
            return false;
        }
        for command in &mut self.commands[..len - 1] {
            if !boundary(RecordAction::End(command.handle)) {
                return false;
            }
            command.state = CommandState::Recorded;
        }
        !final_batch
            || self.emit(
                BarrierPhase::Export,
                self.commands[len - 1].handle,
                out,
                boundary,
            )
    }
}
