//! Full binding of a range program, including static dependencies.
use crate::Result;
use ash::vk;
use crate::rg::sync_plan::BoundBatch;
use crate::Error;
use crate::rg::queue_schedule::QueueDomain;
use crate::rg::sync_plan::BarrierPhase;
use crate::rg::sync_plan::BoundBatchKind;
use crate::rg::sync_plan::DependencyRef;
use crate::rg::sync_plan::ProgramBatch;
use crate::rg::sync_plan::RangeProgram;
use crate::tracking::range_state;
use super::SynchronizationPlanner;

impl SynchronizationPlanner {
    pub fn bind(
        &mut self,
        program: &RangeProgram,
        family_domain: impl FnMut(u32) -> Result<QueueDomain>,
    ) -> Result<()> {
        self.start_binding()?;
        self.bind_full(program, family_domain)
    }

    pub(super) fn start_binding(&mut self) -> Result<()> {
        if !std::mem::take(&mut self.ready) {
            return Err(Error::invalid_usage("synchronization binding requires begin for the next frame"));
        }
        Ok(())
    }

    pub(super) fn bind_full(
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
                return Err(Error::invalid_usage(format!(
                    "native initial range exceeds declaration: resource={} initial={:?} declared={:?}",
                    initial.resource, initial.cell.range, domain.range,
                )));
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
    pub(super) fn later(order: &[usize], a: DependencyRef, b: DependencyRef) -> DependencyRef {
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
}
