//! Current graph capability tests, CPU only. No native ABI field is inferred.
use ash::vk;
use rhapsody::rg::{
    graph::{BuildError, Descriptor, GraphDeclaration, RenderGraph},
    plan::{GraphDefinition, PlanContext, compile_definition, compile_with_subgraphs},
    plan_check::check_plan,
    resource::{AccessRange, ResourceContractError},
    subgraph::SubgraphPlanCache,
    vulkan::{AccessTable, ResourceSpec},
};
const READ: u8 = 0x19;
const WRITE: u8 = 0x34;
fn descriptor() -> Descriptor {
    ResourceSpec::Image(rhapsody::resource::texture::TextureDesc::new_2d(
        16,
        16,
        vk::Format::R32_SFLOAT,
    ))
    .descriptor()
}
fn definition(graph: &GraphDeclaration) -> rhapsody::Result<GraphDefinition> {
    GraphDefinition::from_declaration(graph, &AccessTable::original(), PlanContext::default())
}
fn fixture(initial: u8, unused: bool, later_read: bool) -> GraphDeclaration {
    let mut graph = RenderGraph::new();
    let history = graph
        .import_unowned("history", descriptor(), 1, initial)
        .unwrap();
    let mut pass = graph.add_pass("configured shader");
    if unused {
        pass.add_inactive_resource_read_range(history, READ, 7, AccessRange::Whole)
            .unwrap();
    } else {
        pass.add_resource_access(history, READ, 7).unwrap();
    }
    pass.side_effect();
    pass.render(|_| true);
    if later_read {
        let mut pass = graph.add_pass("later active read");
        pass.add_resource_access(history, READ, 0).unwrap();
        pass.side_effect();
        pass.render(|_| true);
    }
    graph.build().declaration
}
#[test]
fn inactive_binding_is_not_initialization_or_an_exemption_for_active_reads() {
    assert!(definition(&fixture(0, false, false)).is_err());
    let raw = fixture(0, true, false);
    let key = definition(&raw).unwrap();
    let access = key.passes[0].accesses[0];
    assert!(access.contents_unused);
    assert_eq!(
        (
            access.access_code,
            access.synchronization_group,
            access.range
        ),
        (READ, 7, AccessRange::Whole)
    );
    let plan = compile_definition(key).unwrap();
    assert_eq!(plan.retained_passes, [0]);
    assert_ne!(
        plan.info.image_usage_flags[0] & vk::ImageUsageFlags::SAMPLED.as_raw(),
        0
    );
    check_plan(&plan.definition, &plan).unwrap();
    assert!(definition(&fixture(0, true, true)).is_err());
    let mut exported = raw;
    exported
        .exports
        .push(rhapsody::rg::resource_info::ExportDeclaration {
            tag: 0,
            resource_index: 0,
            access_code: READ,
            range: AccessRange::Whole,
        });
    assert!(
        definition(&exported).is_err(),
        "inactive binding must not publish history contents"
    );
}
#[test]
fn active_alias_wins_dedup_in_both_orders_and_writes_are_rejected() {
    for inactive_first in [false, true] {
        let mut graph = RenderGraph::new();
        let history = graph.import_unowned("history", descriptor(), 1, 0).unwrap();
        let mut pass = graph.add_pass("aliased shader binding");
        for inactive in [inactive_first, !inactive_first] {
            if inactive {
                pass.add_inactive_resource_read_range(history, READ, 1, AccessRange::Whole)
                    .unwrap();
            } else {
                pass.add_resource_access_range(history, READ, 1, AccessRange::Whole)
                    .unwrap();
            }
        }
        assert!(matches!(
            pass.add_inactive_resource_read_range(history, WRITE, 1, AccessRange::Whole),
            Err(BuildError::Contract(
                ResourceContractError::WrongAccessMode { .. }
            ))
        ));
        pass.render(|_| true);
        let raw = graph.build().declaration;
        assert_eq!(raw.passes[0].accesses.len(), 1);
        assert!(!raw.passes[0].accesses[0].contents_unused);
        assert!(definition(&raw).is_err());
    }
}
#[test]
fn unused_content_drops_only_data_dependency_and_preserves_native_hazards() {
    let mut graph = RenderGraph::new();
    let history = graph.import_unowned("history", descriptor(), 1, 0).unwrap();
    let mut writer = graph.add_pass("unused prior content");
    writer.add_resource_access(history, WRITE, 0).unwrap();
    writer.render(|_| true);
    let mut reader = graph.add_pass("inactive binding");
    reader
        .add_inactive_resource_read_range(history, READ, 0, AccessRange::Whole)
        .unwrap();
    reader.side_effect();
    reader.render(|_| true);
    let mut overwrite = graph.add_pass("live later write");
    overwrite.add_resource_access(history, WRITE, 0).unwrap();
    overwrite.side_effect();
    overwrite.render(|_| true);
    let plan = compile_definition(definition(&graph.build().declaration).unwrap()).unwrap();
    assert_eq!(
        plan.retained_passes,
        [1, 2],
        "unused producer is not a content dependency"
    );
    assert!(
        plan.dependency_edges.contains(&(1, 2)),
        "native read-to-write hazard must remain"
    );
    check_plan(&plan.definition, &plan).unwrap();
}
#[test]
fn content_use_participates_in_full_and_normalized_cache_identity() {
    let unused_raw = fixture(READ, true, false);
    let active_raw = fixture(READ, false, false);
    let unused = definition(&unused_raw).unwrap();
    let active = definition(&active_raw).unwrap();
    assert_ne!(unused, active);
    let digest = |key: &GraphDefinition| {
        use std::hash::{Hash, Hasher};
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hash);
        hash.finish()
    };
    assert_ne!(digest(&unused), digest(&active));
    // Same resource/lifetime/version, only the configured shader's contents use
    // differs. The normal scope-key copy must retain that bit as well.
    let scoped = |inactive: bool, initial: u8| {
        let mut graph = RenderGraph::new();
        let history = graph
            .import_unowned("history", descriptor(), 1, initial)
            .unwrap();
        graph
            .subgraph("history scope", |scope| {
                if initial != 0 {
                    scope.input(history, AccessRange::Whole, READ)?;
                }
                let mut pass = scope.graph().add_pass("configured shader");
                if inactive {
                    pass.add_inactive_resource_read_range(history, READ, 0, AccessRange::Whole)?;
                } else {
                    pass.add_resource_access(history, READ, 0)?;
                }
                pass.side_effect();
                pass.render(|_| true);
                Ok(())
            })
            .unwrap();
        graph.build().declaration
    };
    let raw = scoped(true, 0);
    let key = definition(&raw).unwrap();
    let mut cache = SubgraphPlanCache::default();
    let plan = compile_with_subgraphs(key.clone(), &mut cache).unwrap();
    check_plan(&plan.definition, &plan).unwrap();
    compile_with_subgraphs(key.clone(), &mut cache).unwrap();
    assert_eq!(cache.stats().hits, 1);
    let mut active = key;
    active.passes[0].accesses[0].contents_unused = false;
    assert!(
        compile_with_subgraphs(active, &mut cache).is_err(),
        "cached inactive scope cannot admit undefined active read"
    );
    let mut cache = SubgraphPlanCache::default();
    compile_with_subgraphs(definition(&scoped(true, READ)).unwrap(), &mut cache).unwrap();
    compile_with_subgraphs(definition(&scoped(false, READ)).unwrap(), &mut cache).unwrap();
    assert_eq!(
        cache.stats().compilations,
        2,
        "normalized cache key includes configured content use"
    );
}

#[test]
fn forged_inactive_writer_and_preserving_read_fail_independent_checks() {
    let mut key = definition(&fixture(READ, true, false)).unwrap();
    key.passes[0].accesses[0].reads_previous = true;
    assert!(rhapsody::rg::plan_check::check_definition(&key).is_err());
    let mut key = definition(&fixture(READ, true, false)).unwrap();
    key.passes[0].accesses[0].access_code = WRITE;
    key.passes[0].accesses[0].version = rhapsody::rg::resource::AccessVersion::Write {
        previous: 0,
        current: 1,
    };
    assert!(rhapsody::rg::plan_check::check_definition(&key).is_err());
}
