//! Consumes the one bound native range plan. This recorder schedules callbacks
//! and command boundaries; it never reconstructs synchronization from access codes.
use super::sync_plan::{BarrierPhase, BoundTransition, SynchronizationPlanner};
use crate::command::recording::{Command, CommandState};
use std::sync::Arc;
#[derive(Clone, Copy, Debug)]
pub struct Pass {
    pub callback_present: bool,
}
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
            .transitions_for(self.batch, phase)
            .copied()
    }
    fn emit(
        &self,
        phase: BarrierPhase,
        command: u64,
        out: &mut Outcome,
        boundary: &mut impl FnMut(RecordAction) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let mut count = 0;
        for transition in self.phase(phase) {
            boundary(RecordAction::Transition {
                command,
                transition,
            })?;
            count += 1;
        }
        out.barrier_transitions += count;
        out.barrier_commands += u32::from(count != 0);
        Ok(())
    }
    /// Emit the batch's initial barriers and collect every pass's barriers
    /// into `planned` (flat), with `ranges[i]` selecting pass `i`'s entries.
    /// Both buffers are cleared first and reuse their capacity.
    pub fn plan_chunked_batch(
        &mut self,
        passes: &mut [Pass],
        commands: &[Command],
        chunk_of_pass: &[usize],
        planned: &mut Vec<BoundTransition>,
        ranges: &mut Vec<std::ops::Range<usize>>,
        mut boundary: impl FnMut(RecordAction) -> crate::Result<()>,
    ) -> crate::Result<Outcome> {
        let mut out = Outcome::default();
        planned.clear();
        ranges.clear();
        if chunk_of_pass.len() != passes.len() {
            return Err(crate::Error::invalid_usage("chunk plan does not cover its passes"));
        }
        let Some(initial) = self.commands.last() else {
            return Err(crate::Error::invalid_usage("batch initial command is missing"));
        };
        self.emit(
            BarrierPhase::Initial,
            initial.handle,
            &mut out,
            &mut boundary,
        )?;
        for (index, pass) in passes.iter_mut().enumerate() {
            if commands.get(chunk_of_pass[index]).is_none()
                || !std::mem::take(&mut pass.callback_present)
            {
                return Err(crate::Error::invalid_usage("chunk command or callback is missing"));
            }
            let start = planned.len();
            planned.extend(self.phase(BarrierPhase::BeforePass(self.first_pass + index)));
            let count = planned.len() - start;
            out.barrier_commands += u32::from(count != 0);
            out.barrier_transitions += count as u32;
            out.consumed_passes += 1;
            ranges.push(start..planned.len());
        }
        Ok(out)
    }
    pub fn finish_chunked_batch(
        &mut self,
        out: &mut Outcome,
        commands: impl IntoIterator<Item = Command>,
        final_batch: bool,
        mut boundary: impl FnMut(RecordAction) -> crate::Result<()>,
    ) -> crate::Result<()> {
        self.finish(out, commands, final_batch, &mut boundary)
    }
    fn finish(
        &mut self,
        out: &mut Outcome,
        commands: impl IntoIterator<Item = Command>,
        final_batch: bool,
        boundary: &mut impl FnMut(RecordAction) -> crate::Result<()>,
    ) -> crate::Result<()> {
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
            return Err(crate::Error::invalid_usage("batch commands are empty"));
        }
        for command in &mut self.commands[..len - 1] {
            boundary(RecordAction::End(command.handle))?;
            command.state = CommandState::Recorded;
        }
        if final_batch {
            self.emit(
                BarrierPhase::Export,
                self.commands[len - 1].handle,
                out,
                boundary,
            )?;
        }
        Ok(())
    }
}
