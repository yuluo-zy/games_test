//! Public subgraph construction and the production symbolic cache; CPU only.
use rhapsody::rg::{
    graph::{BufferDesc, Descriptor, RenderGraph},
    plan::{GraphDefinition, PlanContext, compile_definition, compile_with_subgraphs},
    resource::AccessRange,
    subgraph::{SubgraphPlanCache, SubgraphQueueConstraint},
    vulkan::AccessTable,
};
const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
fn descriptor() -> Descriptor {
    Descriptor::Buffer(BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    })
}
fn fixture(prefix: bool, earlier_versions: usize, implementation: u64) -> GraphDefinition {
    let mut graph = RenderGraph::new();
    if prefix {
        graph
            .import_unowned("unrelated resource", descriptor(), 5, READ)
            .unwrap();
    }
    let input = graph
        .import_unowned(format!("input {prefix}"), descriptor(), 7, READ)
        .unwrap();
    let output = graph
        .create_buffer(
            format!("output {prefix}"),
            match descriptor() {
                Descriptor::Buffer(desc) => desc,
                _ => unreachable!(),
            },
        )
        .unwrap();
    for _ in 0..earlier_versions {
        let mut pass = graph.add_pass("parent write");
        pass.add_resource_access(input, WRITE, 0).unwrap();
        pass.render(|_| true);
    }
    graph
        .subgraph(format!("scope {prefix}"), |subgraph| {
            subgraph.interface_mut().implementation_signature = implementation;
            subgraph.input(input, AccessRange::Whole, READ)?;
            let mut pass = subgraph.graph().add_pass(format!("local pass {prefix}"));
            pass.add_resource_access(input, READ, 0)?;
            pass.add_resource_access(output.raw(), WRITE, 0)?;
            pass.render(|_| true);
            subgraph.output(output.raw(), AccessRange::Whole, WRITE)?;
            Ok(())
        })
        .unwrap();
    graph.export(output.raw(), READ).unwrap();
    GraphDefinition::from_declaration(
        &graph.build().declaration,
        &mut AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap()
}

#[test]
fn normalized_keys_reuse_shifted_parent_indices_names_and_version_bases() {
    let mut cache = SubgraphPlanCache::default();
    compile_with_subgraphs(fixture(false, 0, 1), &mut cache).unwrap();
    assert_eq!(cache.stats().compilations, 1);
    let shifted = fixture(true, 2, 1);
    let compiled = compile_with_subgraphs(shifted.clone(), &mut cache).unwrap();
    assert_eq!(
        cache.stats().compilations,
        1,
        "parent indices and global version offsets are dynamic local bindings"
    );
    assert_eq!(cache.stats().hits, 1);
    let reference = compile_definition(shifted).unwrap();
    assert_eq!(compiled.retained_passes, reference.retained_passes);
    assert_eq!(compiled.dependency_edges, reference.dependency_edges);
    assert_eq!(compiled.range_program, reference.range_program);

    let mut changed = fixture(false, 0, 1);
    changed.context.device_generation = 2;
    compile_with_subgraphs(changed, &mut cache).unwrap();
    assert_eq!(cache.stats().compilations, 2);
    compile_with_subgraphs(fixture(false, 0, 2), &mut cache).unwrap();
    assert_eq!(cache.stats().compilations, 3);
    let mut changed = fixture(false, 0, 1);
    let Descriptor::Buffer(mut desc) = changed.resources[0].descriptor else {
        unreachable!()
    };
    desc.word_at_00 = 128;
    changed.resources[0].descriptor = Descriptor::Buffer(desc);
    compile_with_subgraphs(changed, &mut cache).unwrap();
    assert_eq!(cache.stats().compilations, 4);
}

#[test]
fn native_stages_access_layout_and_layout_coupling_invalidate_local_cache_keys() {
    use rhapsody::rg::cache::CacheReason;
    let baseline = fixture(false, 0, 1);
    let mut cache = SubgraphPlanCache::default();
    compile_with_subgraphs(baseline.clone(), &mut cache).unwrap();
    for index in 0..3 {
        let mut changed = baseline.clone();
        let semantics = changed
            .access_semantics
            .iter_mut()
            .find(|entry| entry.code == WRITE)
            .unwrap();
        match index {
            0 => semantics.scope.stages = ash::vk::PipelineStageFlags::ALL_COMMANDS,
            1 => semantics.scope.access = ash::vk::AccessFlags::MEMORY_WRITE,
            _ => semantics.layout = ash::vk::ImageLayout::GENERAL,
        }
        compile_with_subgraphs(changed, &mut cache).unwrap();
        assert_eq!(cache.stats().compilations, index + 2);
    }
    assert_eq!(
        cache
            .stats()
            .diagnostics
            .misses(CacheReason::AccessSemantics),
        3
    );
    let mut changed = baseline;
    changed.context.separate_depth_stencil_layouts = true;
    compile_with_subgraphs(changed, &mut cache).unwrap();
    assert_eq!(cache.stats().compilations, 5);
    assert_eq!(
        cache.stats().diagnostics.misses(CacheReason::QueuePolicy),
        1
    );
}

#[test]
fn boundary_contracts_reject_missing_ports_stale_versions_wrong_queues_and_overlaps() {
    let original = fixture(false, 0, 1);
    let mut invalid = original.clone();
    invalid.subgraphs[0].interface.inputs.clear();
    assert!(compile_definition(invalid).is_err());
    let mut invalid = original.clone();
    invalid.subgraphs[0].interface.outputs.clear();
    assert!(compile_with_subgraphs(invalid, &mut SubgraphPlanCache::default()).is_err());
    let mut invalid = original.clone();
    invalid.subgraphs[0].interface.inputs[0].version += 1;
    assert!(compile_definition(invalid).is_err());
    let mut invalid = original.clone();
    invalid.subgraphs[0].interface.queue = SubgraphQueueConstraint::Exact {
        family: 1,
        index: 0,
    };
    assert_eq!(
        compile_definition(invalid).unwrap_err().kind(),
        rhapsody::ErrorKind::Unsupported
    );
    let mut invalid = original.clone();
    invalid.subgraphs.push(invalid.subgraphs[0].clone());
    assert!(compile_definition(invalid).is_err());
    let mut invalid = original;
    invalid.subgraphs[0].pass_range.end = usize::MAX;
    assert!(compile_definition(invalid).is_err());
}

#[test]
fn local_cache_is_bounded_and_disabled_or_zero_budget_does_not_retain_entries() {
    let mut cache = SubgraphPlanCache::new(1, 1024 * 1024);
    for implementation in 0..4 {
        compile_with_subgraphs(fixture(false, 0, implementation), &mut cache).unwrap();
    }
    assert_eq!(cache.stats().compilations, 4);
    assert_eq!(cache.stats().evictions, 3);
    assert!(cache.stats().bytes <= 1024 * 1024);
    cache.set_enabled(false);
    assert_eq!(cache.stats().bytes, 0);
    for _ in 0..2 {
        compile_with_subgraphs(fixture(false, 0, 3), &mut cache).unwrap();
    }
    assert_eq!(
        cache.stats().compilations,
        4,
        "disabled mode uses the full-graph compiler without local key construction"
    );
    let mut cache = SubgraphPlanCache::new(0, 0);
    for _ in 0..2 {
        compile_with_subgraphs(fixture(false, 0, 3), &mut cache).unwrap();
    }
    assert_eq!(cache.stats().bytes, 0);
    assert_eq!(cache.stats().compilations, 2);
}

#[test]
fn graph_builder_checks_foreign_handles_and_rolls_back_failed_or_nested_scopes() {
    let mut foreign = RenderGraph::new();
    let handle = foreign
        .import_unowned("foreign", descriptor(), 4, READ)
        .unwrap();
    let mut graph = RenderGraph::new();
    graph
        .import_unowned("local", descriptor(), 5, READ)
        .unwrap();
    let error = graph.subgraph("foreign", |scope| {
        scope.input(handle, AccessRange::Whole, READ)
    });
    assert!(error.is_err());
    let error: rhapsody::Result<()> = graph.subgraph("failed", |scope| {
        scope.graph().add_pass("rolled back").render(|_| true);
        Err(rhapsody::Error::invalid_usage("injected builder failure"))
    });
    assert!(error.is_err());
    let error: rhapsody::Result<()> = graph.subgraph("outer", |scope| {
        scope.graph().subgraph("inner", |nested| {
            nested.graph().add_pass("nested").render(|_| true);
            Ok(())
        })
    });
    assert!(error.is_err());
    let declaration = graph.build().declaration;
    assert!(declaration.passes.is_empty());
    assert!(declaration.subgraphs.is_empty());
}

#[test]
fn failed_scopes_restore_exports_and_batch_policy_without_reusing_escaped_resource_ids() {
    let mut graph = RenderGraph::new();
    let mut escaped = None;
    let retired = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let callback = retired.clone();
    let result: rhapsody::Result<()> = graph.subgraph("failed", |scope| {
        let Descriptor::Buffer(desc) = descriptor() else {
            unreachable!()
        };
        let output = scope.graph().create_buffer("escaped", desc)?;
        escaped = Some(output.raw());
        scope.graph().export(output.raw(), READ)?;
        scope.graph().batch_per_pass(true);
        scope.graph().on_retire(move || {
            callback.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        Err(rhapsody::Error::invalid_usage(
            "injected failure after export",
        ))
    });
    assert!(result.is_err());
    let Descriptor::Buffer(desc) = descriptor() else {
        unreachable!()
    };
    let later = graph.create_buffer("later", desc).unwrap();
    let escaped = escaped.unwrap();
    assert_ne!(
        escaped.raw(),
        later.raw().raw(),
        "a failed closure's escaped handle must not alias a later allocation"
    );
    let mut pass = graph.add_pass("use retained declaration");
    pass.add_resource_access(escaped, WRITE, 0).unwrap();
    pass.render(|_| true);
    let (declaration, _, callbacks) = graph.build().into_parts();
    assert!(declaration.exports.is_empty());
    assert!(!declaration.batch_per_pass);
    assert_eq!(declaration.resources.len(), 2);
    assert_eq!(
        declaration.passes[0].accesses[0].resource_index,
        escaped.raw()
    );
    assert_ne!(
        declaration.passes[0].accesses[0].resource_index,
        later.raw().raw()
    );
    let definition = GraphDefinition::from_declaration(
        &declaration,
        &mut AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    let compiled = compile_definition(definition).unwrap();
    assert!(
        compiled.retained_passes.is_empty(),
        "unused declarations and rolled-back work must not allocate or execute"
    );
    assert_eq!(callbacks.len(), 1, "retirement callbacks remain frame-wide");
    for callback in callbacks {
        callback();
    }
    assert_eq!(retired.load(std::sync::atomic::Ordering::SeqCst), 1);
}
