//! Compilation entry points and plan finishing.
use std::collections::BTreeSet;
use std::collections::VecDeque;
use super::analysis::PassAnalysis;
use super::analysis::SemanticTable;
use super::analysis::add_pass_producers;
use super::analysis::analyze_passes;
use super::analysis::compose_analysis;
use super::compiled::CompiledPlan;
use super::definition::GraphDefinition;
use super::ordering::ordering_edges;
use super::ranges::ranges_overlap;
use super::resource::ResourceDefinition;
use super::super::compiled::BatchCompiler;
use super::super::resource_info::ResourceInfo;

/// Produces immutable symbolic work before asking the backend for any allocation.
pub fn compile_definition(definition: GraphDefinition) -> crate::Result<CompiledPlan> {
    super::super::plan_check::check_definition(&definition)?;
    let analysis = analyze_passes(&definition.passes, &definition.access_semantics);
    finish_plan(definition, analysis)
}

/// Reuses stable local analyses, then coordinates all cross-scope hazards and
/// rebuilds global liveness, symbolic barriers, batches and resource lifetimes.
pub fn compile_with_subgraphs(
    definition: GraphDefinition,
    cache: &mut super::super::subgraph::SubgraphPlanCache,
) -> crate::Result<CompiledPlan> {
    if definition.subgraphs.is_empty() || !cache.is_enabled() {
        return compile_definition(definition);
    }
    super::super::plan_check::check_definition(&definition)?;
    let mut analysis = PassAnalysis {
        dependencies: vec![BTreeSet::new(); definition.passes.len()],
        producers: Default::default(),
    };
    let table = SemanticTable::new(&definition.access_semantics);
    let mut next = 0;
    for scope in &definition.subgraphs {
        while next < scope.pass_range.start {
            add_pass_producers(&mut analysis, next, &definition.passes[next], &table);
            next += 1;
        }
        let local = cache.analysis(&definition, scope)?;
        compose_analysis(
            &definition,
            &table,
            local.range.clone(),
            &local.analysis,
            Some(&local.resources),
            &mut analysis,
        );
        next = local.range.end;
    }
    while next < definition.passes.len() {
        add_pass_producers(&mut analysis, next, &definition.passes[next], &table);
        next += 1;
    }
    finish_plan(definition, analysis)
}

pub(super) fn finish_plan(definition: GraphDefinition, analysis: PassAnalysis) -> crate::Result<CompiledPlan> {
    let count = definition.passes.len();
    let PassAnalysis {
        dependencies: data_dependencies,
        producers,
    } = analysis;
    let mut needed = vec![false; count];
    let mut pending = VecDeque::new();
    for (i, p) in definition.passes.iter().enumerate() {
        if p.side_effect {
            pending.push_back(i);
        }
    }
    for export in &definition.exports {
        if let Some(writers) = producers.get(&export.resource_index) {
            for &(producer, range) in writers {
                if ranges_overlap(export.range, range) {
                    pending.push_back(producer);
                }
            }
        }
    }
    while let Some(pass) = pending.pop_front() {
        if std::mem::replace(&mut needed[pass], true) {
            continue;
        }
        pending.extend(data_dependencies[pass].iter().copied());
    }
    let retained_passes = (0..count).filter(|&i| needed[i]).collect::<Vec<_>>();
    // Even two reads cannot overlap while asking for different native image
    // layouts. This is a storage constraint rather than a content producer edge.
    let dependency_edges = ordering_edges(&definition, &retained_passes)?;
    super::super::queue_schedule::validate_resource_queue_contracts(
        &definition,
        &retained_passes,
        &dependency_edges,
    )?;
    let origins = definition
        .resources
        .iter()
        .map(ResourceDefinition::info_origin)
        .collect::<Vec<_>>();
    let pass_slices = retained_passes
        .iter()
        .map(|&i| definition.passes[i].accesses.as_slice())
        .collect::<Vec<_>>();
    let mut info = ResourceInfo::default();
    let table = SemanticTable::new(&definition.access_semantics);
    info.calculate(&origins, &definition.exports, &pass_slices, |code| {
        table
            .get(code)
            .map_or(ash::vk::AccessFlags::empty(), |semantic| {
                semantic.declared_access
            })
    })?;
    let mut batch_compiler = BatchCompiler::new();
    if definition.batch_per_pass {
        for i in 0..retained_passes.len() {
            batch_compiler.flush_batch(i);
        }
    }
    // A gated resource's first retained user starts a batch (see
    // `RenderGraph::gate_first_use`); `flush_batch` ignores boundaries already
    // present and the one at pass 0.
    let mut gates = definition
        .gated_resources
        .iter()
        .filter_map(|&resource| {
            retained_passes.iter().position(|&pass| {
                definition.passes[pass]
                    .accesses
                    .iter()
                    .any(|access| access.resource_index == resource)
            })
        })
        .collect::<Vec<_>>();
    gates.sort_unstable();
    for boundary in gates {
        // Skip boundaries that `batch_per_pass` already produced.
        if boundary > batch_compiler.current_range.start {
            batch_compiler.flush_batch(boundary);
        }
    }
    let base_batches = batch_compiler.finish(retained_passes.len());
    let (batches, queue_schedule) = super::super::queue_schedule::plan_queue_batches(
        &definition.passes,
        &retained_passes,
        &base_batches,
        &dependency_edges,
        &definition.exports,
        &definition.context,
    )?;
    let snapshot = std::sync::Arc::new(super::super::graph::SnapshotStatic::from_plan(
        &definition,
        &retained_passes,
        &info,
        &batches,
    ));
    let range_program = super::super::sync_plan::RangeProgram::compile(
        &definition,
        &retained_passes,
        &batches,
        &queue_schedule,
    )?;
    let plan = CompiledPlan {
        range_program,
        definition,
        retained_passes,
        dependency_edges,
        info: std::sync::Arc::new(info),
        snapshot,
        batches,
        queue_schedule,
    };
    // Independent verification of the compiler's own output (not input
    // validation, which check_definition performs unconditionally).
    if super::super::validation_enabled() {
        super::super::plan_check::check_plan(&plan.definition, &plan)?;
    }
    Ok(plan)
}
