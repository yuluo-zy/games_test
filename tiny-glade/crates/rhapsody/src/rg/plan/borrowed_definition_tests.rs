use super::*;
use crate::rg::graph::PassDeclaration;
use crate::rg::resource_info::AccessDeclaration;
use crate::rg::graph::Descriptor;
use crate::rg::graph::GraphDeclaration;
use crate::rg::graph::OriginSource;
use crate::rg::resource::AccessVersion;
use crate::rg::{graph::RenderGraph, resource::{AccessRange, ResourceOrigin}, vulkan::AccessTable};
use crate::rg::resource_registry::BufferDesc;
fn declaration() -> GraphDeclaration {
    let mut graph = RenderGraph::new();
    let descriptor = Descriptor::Buffer(BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    });
    let input = graph.import_unowned("input", descriptor, 11, 0x25).unwrap();
    let output = graph
        .create_raw_resource(
            "output",
            ResourceOrigin {
                descriptor,
                source: OriginSource::Create,
            },
        )
        .unwrap();
    graph
        .subgraph("stable", |scope| {
            scope.input(input, AccessRange::Whole, 0x25)?;
            let mut pass = scope.graph().add_pass("copy");
            pass.add_resource_access(input, 0x25, 3)?;
            pass.add_resource_access(output, 0x34, 4)?;
            pass.render(|_| true);
            scope.output(output, AccessRange::Whole, 0x34)?;
            Ok(())
        })
        .unwrap();
    graph.export(output, 0x25).unwrap();
    graph.build().declaration
}
fn gated_plan(gate: bool, per_pass: bool) -> (CompiledPlan, GraphDeclaration) {
    let mut graph = RenderGraph::new();
    let descriptor = Descriptor::Buffer(BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    });
    let early = graph.import_unowned("early", descriptor, 21, 0x25).unwrap();
    let gated = graph.import_unowned("gated", descriptor, 22, 0x25).unwrap();
    for index in 0..2 {
        let mut pass = graph
            .add_pass(format!("independent-{index}"))
            .with_side_effect();
        pass.add_resource_access(early, 0x34, 0).unwrap();
        pass.render(|_| true);
    }
    for index in 0..2 {
        let mut pass = graph.add_pass(format!("gated-{index}")).with_side_effect();
        pass.add_resource_access(gated, 0x34, 0).unwrap();
        pass.render(|_| true);
    }
    graph.batch_per_pass(per_pass);
    if gate {
        graph.gate_first_use(gated).unwrap();
        // Idempotent: one boundary however often it is requested.
        graph.gate_first_use(gated).unwrap();
    }
    let declaration = graph.build().declaration;
    let definition = GraphDefinition::from_declaration(
        &declaration,
        &AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    (compile_definition(definition).unwrap(), declaration)
}
#[test]
fn gating_a_resource_starts_a_batch_before_its_first_user_and_is_part_of_the_key() {
    let (plain, plain_declaration) = gated_plan(false, false);
    assert_eq!(plain.batches.len(), 1, "no gate: one batch");
    let (gated, declaration) = gated_plan(true, false);
    let ranges = gated
        .batches
        .iter()
        .map(|batch| batch.pass_range.clone())
        .collect::<Vec<_>>();
    assert_eq!(ranges, vec![0..2, 2..4]);
    // The boundary is part of the plan identity: neither the owned key
    // nor the borrowed comparison confuses gated and ungated graphs.
    assert_ne!(gated.definition, plain.definition);
    let context = PlanContext::default();
    assert!(
        gated
            .definition
            .matches_declaration(&declaration, &AccessTable::original(), context)
            .unwrap()
    );
    assert!(
        !gated
            .definition
            .matches_declaration(&plain_declaration, &AccessTable::original(), context)
            .unwrap()
    );
    // Under batch_per_pass the existing boundaries already cover it.
    let (per_pass, _) = gated_plan(true, true);
    assert_eq!(per_pass.batches.len(), 4);
    assert!(
        per_pass
            .batches
            .iter()
            .all(|batch| batch.pass_range.len() == 1)
    );
}
#[test]
fn complete_plan_miss_reasons_distinguish_policy_interface_and_structure() {
    use crate::collections::cache::CacheReason;
    let old = GraphDefinition::from_declaration(
        &declaration(),
        &AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    let mut new = old.clone();
    new.context.device_generation += 1;
    assert_eq!(
        old.differing_key_reason(&new),
        CacheReason::DeviceGeneration
    );
    let mut new = old.clone();
    new.context.policy_signature += 1;
    assert_eq!(old.differing_key_reason(&new), CacheReason::QueuePolicy);
    let mut new = old.clone();
    new.subgraphs[0].interface.implementation_signature += 1;
    assert_eq!(
        old.differing_key_reason(&new),
        CacheReason::ShaderImplementation
    );
    let mut new = old.clone();
    new.subgraphs[0].interface.shader_interface_signature += 1;
    assert_eq!(old.differing_key_reason(&new), CacheReason::ShaderInterface);
    let mut new = old.clone();
    new.passes[0].name = "changed pass".into();
    assert_eq!(old.differing_key_reason(&new), CacheReason::GraphStructure);
}
#[test]
fn borrowed_comparison_matches_the_complete_owned_definition() {
    let graph = declaration();
    let context = PlanContext::default();
    let key = GraphDefinition::from_declaration(&graph, &AccessTable::original(), context)
        .unwrap();
    let changes: &[fn(&mut GraphDeclaration)] = &[
        |g| g.resources[0].name = format!("{}!", g.resources[0].name).into(),
        |g| {
            g.resources[0].descriptor = Descriptor::Buffer(BufferDesc {
                word_at_00: 128,
                word_at_08: 0,
                bytes_at_0c_through_0e: [0; 3],
            })
        },
        |g| {
            g.resources[0].sharing = crate::tracking::ResourceSharing::Concurrent(
                crate::tracking::QueueFamilies::new(&[0, 1]).unwrap(),
            )
        },
        |g| {
            g.resources[0].source = OriginSource::ImportOwned {
                token: 11,
                initial_access: 0x25,
            }
        },
        |g| g.passes[0].name = format!("{}!", g.passes[0].name).into(),
        |g| g.passes[0].queue_class = crate::device::QueueClass::Compute,
        |g| g.passes[0].side_effect = true,
        |g| g.passes[0].accesses[0].synchronization_group += 1,
        |g| g.passes[0].accesses[0].access_code = 0x26,
        |g| {
            g.passes[0].accesses[0].range = AccessRange::Buffer {
                offset: 0,
                size: 32,
            }
        },
        |g| g.passes[0].accesses[0].reads_previous = true,
        |g| g.passes[0].accesses[0].allow_undefined_contents = true,
        |g| {
            g.passes[0].accesses[0].previous_content_read =
                super::super::resource_info::PreviousContentRead::UndefinedAllowed
        },
        |g| g.passes[0].accesses[0].version = AccessVersion::Read(9),
        |g| g.subgraphs[0].interface.implementation_signature += 1,
        |g| g.exports[0].access_code = 0x26,
        |g| {
            g.exports[0].range = AccessRange::Buffer {
                offset: 0,
                size: 32,
            }
        },
        |g| g.batch_per_pass = true,
        |g| g.gated_resources.push(0),
    ];
    for change in changes {
        let mut changed = graph.clone();
        change(&mut changed);
        assert!(
            !key.matches_declaration(&changed, &AccessTable::original(), context)
                .unwrap()
        );
        assert!(
            GraphDefinition::from_declaration(&changed, &AccessTable::original(), context)
                .map_or(true, |owned| owned != key)
        );
    }
    let mut changed = graph.clone();
    changed.resources[0].source = OriginSource::ImportUnowned {
        token: 99,
        initial_access: 0,
    };
    changed.passes[0].graph_id = None;
    for (access, resolved) in changed.passes[0]
        .accesses
        .iter_mut()
        .zip(&key.passes[0].accesses)
    {
        access.version = resolved.version;
    }
    assert_eq!(
        GraphDefinition::from_declaration(&changed, &AccessTable::original(), context)
            .unwrap(),
        key
    );
    assert!(
        key.matches_declaration(&changed, &AccessTable::original(), context)
            .unwrap()
    );
    let mut policy = AccessTable::original();
    policy.0.get_mut(&0x34).unwrap().scope.access ^=
        ash::vk::AccessFlags::INDIRECT_COMMAND_READ;
    assert!(
        !key.matches_declaration(&graph, &policy, context)
            .unwrap()
    );
    let mut policy = AccessTable::original();
    policy.0.get_mut(&0x34).unwrap().scope.write = false;
    assert!(
        !key.matches_declaration(&graph, &policy, context)
            .unwrap()
    );
    for changed_context in [
        PlanContext {
            separate_depth_stencil_layouts: true,
            ..context
        },
        PlanContext {
            device_generation: 1,
            ..context
        },
        PlanContext {
            queue_family: 1,
            ..context
        },
        PlanContext {
            queue_index: 1,
            ..context
        },
        PlanContext {
            policy_signature: 1,
            ..context
        },
        PlanContext {
            queue_domains: Some(
                [super::super::queue_schedule::QueueDomain {
                    queue_index: 0,
                    family: 0,
                }; 3],
            ),
            ..context
        },
    ] {
        assert!(
            !key.matches_declaration(&graph, &AccessTable::original(), changed_context)
                .unwrap()
        );
    }
}
#[test]
fn native_scope_and_layout_changes_invalidate_owned_and_borrowed_plan_keys() {
    let graph = declaration();
    let context = PlanContext::default();
    let original =
        GraphDefinition::from_declaration(&graph, &AccessTable::original(), context)
            .unwrap();
    let changes: &[fn(&mut crate::native::AccessSpec)] = &[
        |spec| spec.scope.stages = ash::vk::PipelineStageFlags::ALL_COMMANDS,
        |spec| spec.scope.access = ash::vk::AccessFlags::MEMORY_WRITE,
        |spec| spec.layout = ash::vk::ImageLayout::GENERAL,
    ];
    for change in changes {
        let mut policy = AccessTable::original();
        change(policy.0.get_mut(&0x34).unwrap());
        let changed = GraphDefinition::from_declaration(&graph, &policy, context).unwrap();
        assert_ne!(original, changed);
        assert_eq!(
            original.differing_key_reason(&changed),
            crate::collections::cache::CacheReason::AccessSemantics
        );
        assert!(
            !original
                .matches_declaration(&graph, &policy, context)
                .unwrap()
        );
    }
}
#[test]
fn matching_import_revalidates_partial_initialization_and_read_modify_write() {
    let descriptor = crate::rg::vulkan::ResourceSpec::Image(
        crate::resource::texture::TextureDesc::new_2d(16, 16, ash::vk::Format::R8G8B8A8_UNORM)
            .mip_levels(2),
    )
    .descriptor();
    let access = |code, mip| AccessDeclaration {
        resource_index: 0,
        synchronization_group: 0,
        access_code: code,
        version: AccessVersion::Automatic,
        reads_previous: false,
        contents_unused: false,
        allow_undefined_contents: false,
        previous_content_read: Default::default(),
        range: AccessRange::Texture {
            aspect_mask: 1,
            base_mip_level: mip,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        },
    };
    for full_write in [false, true] {
        let mut graph = GraphDeclaration::default();
        graph
            .resources
            .push(super::super::graph::ResourceDeclaration {
                sharing: Default::default(),
                name: "image".into(),
                descriptor,
                source: OriginSource::ImportUnowned {
                    token: 1,
                    initial_access: 0x25,
                },
            });
        let mut writes = vec![access(0x34, 0)];
        if full_write {
            writes.push(access(0x34, 1));
        }
        graph.passes.push(PassDeclaration {
            queue_class: crate::device::QueueClass::Graphics,
            graph_id: None,
            name: "initialize".into(),
            accesses: writes,
            side_effect: false,
        });
        graph.passes.push(PassDeclaration {
            queue_class: crate::device::QueueClass::Graphics,
            graph_id: None,
            name: "read".into(),
            accesses: vec![AccessDeclaration {
                range: AccessRange::Whole,
                ..access(0x25, 0)
            }],
            side_effect: true,
        });
        let key = GraphDefinition::from_declaration(
            &graph,
            &AccessTable::original(),
            PlanContext::default(),
        )
        .unwrap();
        graph.resources[0].source = OriginSource::ImportUnowned {
            token: 2,
            initial_access: 0,
        };
        let matched = key.matches_declaration(
            &graph,
            &AccessTable::original(),
            PlanContext::default(),
        );
        assert_eq!(matched.is_ok(), full_write);
        assert_eq!(
            GraphDefinition::from_declaration(
                &graph,
                &AccessTable::original(),
                PlanContext::default()
            )
            .is_ok(),
            full_write
        );
        if full_write {
            assert!(matched.unwrap());
        }
    }
    let mut graph = declaration();
    graph.passes[0].accesses[1].reads_previous = true;
    graph.resources[1].source = OriginSource::ImportUnowned {
        token: 2,
        initial_access: 0x25,
    };
    let key = GraphDefinition::from_declaration(
        &graph,
        &AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    // Buffer imports are initialized independently of image layout codes.
    graph.resources[1].source = OriginSource::ImportUnowned {
        token: 3,
        initial_access: 0,
    };
    assert!(
        key.matches_declaration(&graph, &AccessTable::original(), PlanContext::default())
            .unwrap()
    );
}
#[test]
fn undefined_read_permission_does_not_bypass_cached_ordinary_boundary_validation() {
    let descriptor = super::super::vulkan::ResourceSpec::Image(
        crate::resource::texture::TextureDesc::new_2d(8, 8, ash::vk::Format::R32_SFLOAT),
    )
    .descriptor();
    for permitted in [false, true] {
        let mut graph = RenderGraph::new();
        let h = graph
            .import_unowned("history", descriptor, 9, 0x19)
            .unwrap();
        let mut pass = graph.add_pass("history");
        if permitted {
            pass.add_undefined_content_read_range(h, 0x19, 0, AccessRange::Whole)
                .unwrap();
        } else {
            pass.add_resource_access(h, 0x19, 0).unwrap();
        }
        pass.side_effect();
        pass.render(|_| true);
        let mut raw = graph.build().declaration;
        let context = PlanContext::default();
        let key =
            GraphDefinition::from_declaration(&raw, &AccessTable::original(), context)
                .unwrap();
        raw.resources[0].source = OriginSource::ImportUnowned {
            token: 10,
            initial_access: 0,
        };
        let borrowed = key.matches_declaration(&raw, &AccessTable::original(), context);
        assert_eq!(borrowed.is_ok(), permitted);
        if permitted {
            assert!(borrowed.unwrap());
        }
        assert_eq!(
            GraphDefinition::from_declaration(&raw, &AccessTable::original(), context)
                .is_ok(),
            permitted
        );
    }
}
