//! Same-code/group aliases must retain previous-content dependencies through
//! graph definition, subgraph cache normalization, culling and the plan verifier.
use ash::vk;
use rhapsody::rg::{
    AccessRange, AccessTable, GraphDeclaration, GraphDefinition, PlanContext, RenderGraph,
    ResourceSpec, check_plan, compile_definition, plan::compile_with_subgraphs,
    subgraph::SubgraphPlanCache,
};

fn descriptor() -> rhapsody::rg::Descriptor {
    ResourceSpec::Image(rhapsody::resource::TextureDesc::new_2d_array(
        32,
        32,
        2,
        vk::Format::D32_SFLOAT,
    ))
    .descriptor()
}
fn layer(layer: u32) -> AccessRange {
    AccessRange::Texture {
        aspect_mask: 2,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: layer,
        layer_count: 1,
    }
}
fn definition(raw: &GraphDeclaration) -> rhapsody::Result<GraphDefinition> {
    GraphDefinition::from_declaration(raw, &mut AccessTable::original(), PlanContext::default())
}
fn fixture(preserve: bool, preserving_first: bool, scoped: bool) -> GraphDeclaration {
    let mut graph = RenderGraph::new();
    let texture = graph.import_unowned("depth", descriptor(), 1, 0).unwrap();
    let mut source = graph.add_pass("real initial depth");
    source
        .add_resource_access_range(texture, 0x2f, 1, layer(0))
        .unwrap();
    source.render(|_| true);
    let record = |graph: &mut RenderGraph| {
        let mut update = graph.add_pass("aliased depth output");
        if preserve && preserving_first {
            update
                .add_preserving_resource_access_range(texture, 0x2f, 1, layer(0))
                .unwrap();
        }
        update
            .add_resource_access_range(texture, 0x2f, 1, layer(0))
            .unwrap();
        if preserve && !preserving_first {
            update
                .add_preserving_resource_access_range(texture, 0x2f, 1, layer(0))
                .unwrap();
        }
        update.side_effect();
        update.render(|_| true);
    };
    if scoped {
        graph
            .subgraph("attachment scope", |scope| {
                scope.input(texture, layer(0), 0x17)?;
                record(scope.graph());
                scope.output(texture, layer(0), 0x2f)?;
                Ok(())
            })
            .unwrap();
    } else {
        record(&mut graph);
    }
    graph.build().declaration
}

#[test]
fn preserving_alias_in_either_order_retains_the_real_source_through_compile_and_check() {
    for first in [false, true] {
        let raw = fixture(true, first, false);
        assert_eq!(raw.passes[1].accesses.len(), 1);
        assert!(raw.passes[1].accesses[0].reads_previous);
        let plan = compile_definition(definition(&raw).unwrap()).unwrap();
        assert_eq!(plan.retained_passes, [0, 1]);
        assert!(plan.dependency_edges.contains(&(0, 1)));
        check_plan(&plan.definition, &plan).unwrap();
    }
    let discard = compile_definition(definition(&fixture(false, false, false)).unwrap()).unwrap();
    assert_eq!(discard.retained_passes, [1]);
    check_plan(&discard.definition, &discard).unwrap();

    let mut graph = RenderGraph::new();
    let texture = graph
        .import_unowned("no source", descriptor(), 9, 0)
        .unwrap();
    let mut update = graph.add_pass("discard alias cannot initialize its preserving alias");
    update
        .add_resource_access_range(texture, 0x2f, 1, layer(0))
        .unwrap();
    update
        .add_preserving_resource_access_range(texture, 0x2f, 1, layer(0))
        .unwrap();
    update.side_effect();
    update.render(|_| true);
    assert!(definition(&graph.build().declaration).is_err());

    let mut graph = RenderGraph::new();
    let texture = graph
        .import_unowned("group mismatch", descriptor(), 10, 0x17)
        .unwrap();
    let mut update = graph.add_pass("failed alias is not committed");
    update
        .add_resource_access_range(texture, 0x2f, 4, layer(0))
        .unwrap();
    assert!(
        update
            .add_preserving_resource_access_range(texture, 0x2f, 5, layer(0))
            .is_err()
    );
    update.side_effect();
    update.render(|_| true);
    let raw = graph.build().declaration;
    assert_eq!(raw.passes[0].accesses.len(), 1);
    assert!(!raw.passes[0].accesses[0].reads_previous);
}

#[test]
fn preserving_bit_participates_in_full_key_and_normalized_subgraph_cache() {
    use std::hash::{Hash, Hasher};
    let preserved = definition(&fixture(true, false, true)).unwrap();
    let discarded = definition(&fixture(false, false, true)).unwrap();
    let hash = |definition: &GraphDefinition| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        definition.hash(&mut hasher);
        hasher.finish()
    };
    assert_ne!(preserved, discarded);
    assert_ne!(hash(&preserved), hash(&discarded));
    let mut cache = SubgraphPlanCache::default();
    let first = compile_with_subgraphs(preserved.clone(), &mut cache).unwrap();
    let hit = compile_with_subgraphs(preserved, &mut cache).unwrap();
    assert_eq!(first.retained_passes, [0, 1]);
    assert_eq!(hit.retained_passes, [0, 1]);
    assert_eq!(cache.stats().hits, 1);
    let separate = compile_with_subgraphs(discarded, &mut cache).unwrap();
    assert_eq!(separate.retained_passes, [1]);
    assert_eq!(cache.stats().compilations, 2);
    check_plan(&first.definition, &first).unwrap();
    check_plan(&hit.definition, &hit).unwrap();
    check_plan(&separate.definition, &separate).unwrap();
}

#[test]
fn disjoint_and_overlapping_nonidentical_ranges_do_not_merge_or_share_preserving_flags() {
    let mut graph = RenderGraph::new();
    let texture = graph
        .import_unowned("layers", descriptor(), 3, 0x17)
        .unwrap();
    let mut pass = graph.add_pass("independent layers");
    pass.add_resource_access_range(texture, 0x2f, 1, layer(0))
        .unwrap();
    pass.add_preserving_resource_access_range(texture, 0x2f, 1, layer(1))
        .unwrap();
    pass.add_preserving_resource_access_range(texture, 0x2f, 1, AccessRange::Whole)
        .unwrap();
    pass.side_effect();
    pass.render(|_| true);
    let raw = graph.build().declaration;
    assert_eq!(raw.passes[0].accesses.len(), 3);
    assert!(!raw.passes[0].accesses[0].reads_previous);
    assert!(raw.passes[0].accesses[1].reads_previous);
    assert!(raw.passes[0].accesses[2].reads_previous);
    let plan = compile_definition(definition(&raw).unwrap()).unwrap();
    check_plan(&plan.definition, &plan).unwrap();
}

#[test]
fn active_inactive_and_invalid_preserving_read_keep_contract_rejections_in_both_orders() {
    for inactive_first in [false, true] {
        let mut graph = RenderGraph::new();
        let texture = graph
            .import_unowned("undefined", descriptor(), 4, 0)
            .unwrap();
        let mut pass = graph.add_pass("active alias cannot suppress content requirements");
        if inactive_first {
            pass.add_inactive_resource_read_range(texture, 0x17, 1, layer(0))
                .unwrap();
        }
        pass.add_resource_access_range(texture, 0x17, 1, layer(0))
            .unwrap();
        if !inactive_first {
            pass.add_inactive_resource_read_range(texture, 0x17, 1, layer(0))
                .unwrap();
        }
        pass.side_effect();
        pass.render(|_| true);
        assert!(definition(&graph.build().declaration).is_err());

        let mut graph = RenderGraph::new();
        let texture = graph
            .import_unowned("defined", descriptor(), 5, 0x17)
            .unwrap();
        let mut pass = graph.add_pass("invalid preserving read alias");
        if inactive_first {
            pass.add_inactive_resource_read_range(texture, 0x17, 1, layer(0))
                .unwrap();
        }
        // The low-level preserving adapter does not classify codes; the graph
        // must reject reads_previous on a non-writing access after alias merge.
        pass.add_preserving_resource_access_range(texture, 0x17, 1, layer(0))
            .unwrap();
        if !inactive_first {
            pass.add_inactive_resource_read_range(texture, 0x17, 1, layer(0))
                .unwrap();
        }
        pass.side_effect();
        pass.render(|_| true);
        assert!(definition(&graph.build().declaration).is_err());
    }
    // The two native in-place read-code merges must retain an invalid late
    // reads_previous bit too, rather than laundering it into a valid read.
    for (first, second) in [(0x17, 0x1d), (0x1d, 0x17), (0x1a, 0x02)] {
        let mut graph = RenderGraph::new();
        let buffer = graph
            .import_unowned(
                "merged invalid read",
                ResourceSpec::Buffer(rhapsody::resource::BufferDesc::new(
                    64,
                    vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::INDIRECT_BUFFER,
                    rhapsody::resource::MemoryLocation::GpuOnly,
                ))
                .descriptor(),
                6,
                0x26,
            )
            .unwrap();
        let mut pass = graph.add_pass("native read merge");
        pass.add_resource_access_range(buffer, first, 1, AccessRange::Whole)
            .unwrap();
        pass.add_preserving_resource_access_range(buffer, second, 1, AccessRange::Whole)
            .unwrap();
        pass.side_effect();
        pass.render(|_| true);
        let raw = graph.build().declaration;
        assert!(raw.passes[0].accesses[0].reads_previous);
        assert!(definition(&raw).is_err());
    }
}
