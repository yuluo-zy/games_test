//! Explicit undefined-content reads preserve original native behavior without
//! turning ordinary uninitialized reads into valid initialized contents.
use ash::vk;
use rhapsody::rg::{
    graph::{BuildError, Descriptor, GraphDeclaration, OriginSource, RenderGraph},
    plan::{GraphDefinition, PlanContext, compile_definition, compile_with_subgraphs},
    plan_check::{check_definition, check_plan},
    resource::{AccessRange, ResourceContractError, ResourceOrigin},
    subgraph::SubgraphPlanCache,
    vulkan::{AccessTable, ResourceSpec},
};
const READ: u8 = 0x19;
const WRITE: u8 = 0x34;
fn descriptor() -> Descriptor {
    ResourceSpec::Image(
        rhapsody::raw::texture::TextureDesc::new_2d(16, 16, vk::Format::R32_SFLOAT).mip_levels(2),
    )
    .descriptor()
}
fn definition(graph: &GraphDeclaration) -> rhapsody::Result<GraphDefinition> {
    GraphDefinition::from_declaration(graph, &mut AccessTable::original(), PlanContext::default())
}
fn fixture(initial: Option<u8>, permitted: bool, later_read: bool) -> GraphDeclaration {
    let mut graph = RenderGraph::new();
    let source = initial.map_or(OriginSource::Create, |initial_access| {
        OriginSource::ImportUnowned {
            token: 1,
            initial_access,
        }
    });
    let history = graph
        .create_raw_resource(
            "history",
            ResourceOrigin {
                descriptor: descriptor(),
                source,
            },
        )
        .unwrap();
    let mut reader = graph.add_pass("original undefined history read");
    if permitted {
        reader
            .add_undefined_content_read_range(history, READ, 7, AccessRange::Whole)
            .unwrap();
    } else {
        reader.add_resource_access(history, READ, 7).unwrap();
    }
    reader.side_effect();
    reader.render(|_| true);
    if later_read {
        let mut later = graph.add_pass("ordinary later read");
        later.add_resource_access(history, READ, 0).unwrap();
        later.side_effect();
        later.render(|_| true);
    }
    graph.build().declaration
}
#[test]
fn permission_keeps_real_access_and_lifetime_without_initializing_or_exporting_history() {
    for initial in [None, Some(0)] {
        assert!(definition(&fixture(initial, false, false)).is_err());
        let raw = fixture(initial, true, false);
        let key = definition(&raw).unwrap();
        let access = key.passes[0].accesses[0];
        assert!(access.allow_undefined_contents);
        assert!(!access.contents_unused);
        assert!(
            access.reads_contents(false),
            "this is a real shader content read"
        );
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
        assert!(!plan.info.lifetimes.is_empty());
        assert_ne!(
            plan.info.image_usage_flags[0] & vk::ImageUsageFlags::SAMPLED.as_raw(),
            0
        );
        check_plan(&plan.definition, &plan).unwrap();
        assert!(definition(&fixture(initial, true, true)).is_err());
        let mut exported = raw;
        exported
            .exports
            .push(rhapsody::rg::graph::ExportDeclaration {
                tag: 0,
                resource_index: 0,
                access_code: READ,
                range: AccessRange::Whole,
            });
        assert!(
            definition(&exported).is_err(),
            "permission is not a content producer"
        );
    }
}
#[test]
fn permitted_reads_keep_existing_partial_producers_and_native_read_write_hazards() {
    let mut graph = RenderGraph::new();
    let history = graph
        .create_raw_resource(
            "partially initialized history",
            ResourceOrigin {
                descriptor: descriptor(),
                source: OriginSource::Create,
            },
        )
        .unwrap();
    let mut writer = graph.add_pass("real mip zero producer");
    writer
        .add_resource_access_range(
            history,
            WRITE,
            0,
            AccessRange::Texture {
                aspect_mask: 1,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        )
        .unwrap();
    writer.render(|_| true);
    let mut reader = graph.add_pass("original read of both defined and undefined mips");
    reader
        .add_undefined_content_read_range(history, READ, 0, AccessRange::Whole)
        .unwrap();
    reader.side_effect();
    reader.render(|_| true);
    let mut overwrite = graph.add_pass("subsequent overwrite");
    overwrite.add_resource_access(history, WRITE, 0).unwrap();
    overwrite.side_effect();
    overwrite.render(|_| true);
    let plan = compile_definition(definition(&graph.build().declaration).unwrap()).unwrap();
    assert_eq!(
        plan.retained_passes,
        [0, 1, 2],
        "available content writers remain required"
    );
    assert!(plan.dependency_edges.contains(&(0, 1)));
    assert!(
        plan.dependency_edges.contains(&(1, 2)),
        "native read/write hazard remains"
    );
    check_plan(&plan.definition, &plan).unwrap();
}
#[test]
fn ordinary_alias_tightens_permission_but_inactive_alias_does_not_in_either_order() {
    for permitted_first in [false, true] {
        for ordinary in [false, true] {
            let mut graph = RenderGraph::new();
            let h = graph.import_unowned("history", descriptor(), 1, 0).unwrap();
            let mut pass = graph.add_pass("aliased real history");
            for permitted in [permitted_first, !permitted_first] {
                if permitted {
                    pass.add_undefined_content_read_range(h, READ, 1, AccessRange::Whole)
                        .unwrap();
                } else if ordinary {
                    pass.add_resource_access(h, READ, 1).unwrap();
                } else {
                    pass.add_inactive_resource_read_range(h, READ, 1, AccessRange::Whole)
                        .unwrap();
                }
            }
            pass.side_effect();
            pass.render(|_| true);
            let raw = graph.build().declaration;
            assert_eq!(raw.passes[0].accesses.len(), 1);
            assert!(!raw.passes[0].accesses[0].contents_unused);
            assert_eq!(
                raw.passes[0].accesses[0].allow_undefined_contents,
                !ordinary
            );
            assert_eq!(definition(&raw).is_ok(), !ordinary);
        }
    }
}
#[test]
fn write_permission_and_conflicting_forged_intent_fail_independent_validation() {
    let mut graph = RenderGraph::new();
    let h = graph.import_unowned("history", descriptor(), 1, 0).unwrap();
    let mut pass = graph.add_pass("invalid original contract");
    assert!(matches!(
        pass.add_undefined_content_read_range(h, WRITE, 0, AccessRange::Whole),
        Err(BuildError::Contract(
            ResourceContractError::WrongAccessMode { .. }
        ))
    ));
    let valid = definition(&fixture(Some(READ), true, false)).unwrap();
    for forbidden in [0, 1, 2] {
        let mut forged = valid.clone();
        let a = &mut forged.passes[0].accesses[0];
        match forbidden {
            0 => a.contents_unused = true,
            1 => a.reads_previous = true,
            _ => {
                a.access_code = WRITE;
                a.version = rhapsody::rg::resource::AccessVersion::Write {
                    previous: 0,
                    current: 1,
                };
            }
        }
        assert!(check_definition(&forged).is_err());
        let mut raw = fixture(Some(READ), true, false);
        raw.passes[0].accesses = forged.passes[0].accesses.clone();
        assert!(definition(&raw).is_err());
    }
}
#[test]
fn permission_is_part_of_full_hash_and_normalized_cache_identity() {
    let permitted = definition(&fixture(Some(READ), true, false)).unwrap();
    let ordinary = definition(&fixture(Some(READ), false, false)).unwrap();
    assert_ne!(permitted, ordinary);
    let digest = |key: &GraphDefinition| {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    };
    assert_ne!(digest(&permitted), digest(&ordinary));
    let scoped = |permitted: bool| {
        let mut graph = RenderGraph::new();
        let h = graph
            .import_unowned("history", descriptor(), 1, READ)
            .unwrap();
        graph
            .subgraph("original history scope", |scope| {
                scope.input(h, AccessRange::Whole, READ)?;
                let mut pass = scope.graph().add_pass("history read");
                if permitted {
                    pass.add_undefined_content_read_range(h, READ, 0, AccessRange::Whole)?;
                } else {
                    pass.add_resource_access(h, READ, 0)?;
                }
                pass.side_effect();
                pass.render(|_| true);
                Ok(())
            })
            .unwrap();
        definition(&graph.build().declaration).unwrap()
    };
    let mut cache = SubgraphPlanCache::default();
    let key = scoped(true);
    compile_with_subgraphs(key.clone(), &mut cache).unwrap();
    compile_with_subgraphs(key, &mut cache).unwrap();
    assert_eq!(cache.stats().hits, 1);
    let plan = compile_with_subgraphs(scoped(false), &mut cache).unwrap();
    check_plan(&plan.definition, &plan).unwrap();
    assert_eq!(
        cache.stats().compilations,
        2,
        "normalization must retain this contract bit"
    );
}
