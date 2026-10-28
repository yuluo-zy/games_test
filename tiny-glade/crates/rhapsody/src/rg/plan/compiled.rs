//! Compiled plan produced from a graph definition.
use super::definition::GraphDefinition;
use super::resource::AccessSemantics;
use super::super::compiled::Batch;
use super::super::resource_info::AccessDeclaration;
use super::super::resource_info::ExportDeclaration;
use super::super::resource_info::ResourceInfo;

#[derive(Clone, Debug)]
pub struct CompiledPlan {
    pub range_program: super::super::sync_plan::RangeProgram,
    pub definition: GraphDefinition,
    /// Original declaration indices. Frame callbacks use this exact mapping.
    pub retained_passes: Vec<usize>,
    /// Original indices, including storage anti-dependencies between versions.
    pub dependency_edges: Vec<(usize, usize)>,
    pub info: std::sync::Arc<ResourceInfo>,
    pub snapshot: std::sync::Arc<super::super::graph::SnapshotStatic>,
    pub batches: Vec<Batch>,
    pub queue_schedule: super::super::queue_schedule::QueueSchedule,
}

impl CompiledPlan {
    pub fn estimated_bytes(&self) -> usize {
        let d = &self.definition;
        std::mem::size_of::<Self>()
            .saturating_add(self.snapshot.estimated_bytes())
            .saturating_add(self.range_program.estimated_bytes())
            .saturating_add(
                d.resources
                    .iter()
                    .map(|r| std::mem::size_of_val(r) + r.name.len())
                    .sum::<usize>(),
            )
            .saturating_add(
                d.passes
                    .iter()
                    .map(|p| {
                        std::mem::size_of_val(p)
                            + p.name.len()
                            + p.accesses.len() * std::mem::size_of::<AccessDeclaration>()
                    })
                    .sum::<usize>(),
            )
            .saturating_add(
                self.info.accesses.len()
                    * std::mem::size_of::<crate::rg::resource_info::ResourceAccessRecord>(),
            )
            .saturating_add(
                self.info.lifetimes.len()
                    * std::mem::size_of::<crate::rg::resource_info::ResourceLifetimeRecord>(),
            )
            .saturating_add(self.dependency_edges.len() * std::mem::size_of::<(usize, usize)>())
            .saturating_add(
                self.queue_schedule.domains.len()
                    * std::mem::size_of::<super::super::queue_schedule::QueueDomain>(),
            )
            .saturating_add(
                self.queue_schedule
                    .dependencies
                    .iter()
                    .map(|dependencies| {
                        dependencies.len() * std::mem::size_of::<usize>()
                            + std::mem::size_of::<Vec<usize>>()
                    })
                    .sum::<usize>(),
            )
            .saturating_add(d.exports.len() * std::mem::size_of::<ExportDeclaration>())
            .saturating_add(d.access_semantics.len() * std::mem::size_of::<AccessSemantics>())
            .saturating_add(self.retained_passes.len() * std::mem::size_of::<usize>())
            .saturating_add(self.batches.len() * std::mem::size_of::<Batch>())
            .saturating_add(
                d.subgraphs
                    .iter()
                    .map(|scope| {
                        std::mem::size_of::<super::super::subgraph::SubgraphDeclaration>()
                            + scope.name.len()
                            + (scope.interface.inputs.len() + scope.interface.outputs.len())
                                * std::mem::size_of::<super::super::subgraph::SubgraphPort>()
                    })
                    .sum::<usize>(),
            )
    }
    pub fn dump(&self) -> String {
        use std::fmt::Write;
        let mut out = format!(
            "compiler={} passes={}/{} resources={}\n",
            self.definition.compiler_version,
            self.retained_passes.len(),
            self.definition.passes.len(),
            self.definition.resources.len()
        );
        for (scheduled, original) in self.retained_passes.iter().enumerate() {
            let p = &self.definition.passes[*original];
            let _ = writeln!(
                out,
                "pass {scheduled} original={original} {:?} side_effect={}",
                p.name, p.side_effect
            );
            for a in &p.accesses {
                let _ = writeln!(
                    out,
                    "  r{} {:?} {:?} access={:#x}",
                    a.resource_index, a.version, a.range, a.access_code
                );
            }
        }
        for (from, to) in &self.dependency_edges {
            let _ = writeln!(out, "dependency {from} -> {to}");
        }
        out
    }
}
