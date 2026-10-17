//! Pure production planning, checked against an independent byte interpreter.
use rhapsody::rg::{
    graph::{
        AccessDeclaration, BufferDesc, Descriptor, ExportDeclaration, GraphDeclaration,
        OriginSource, PassDeclaration, ResourceDeclaration,
    },
    plan::{
        CompiledPlan, GraphDefinition, PlanContext, compile_definition, compile_with_subgraphs,
    },
    plan_check::{check_definition, check_plan},
    resource::{AccessRange, AccessVersion},
    vulkan::AccessTable,
};

const READ: u8 = 0x26;
const WRITE: u8 = 0x34;
const SIZE: usize = 16;

fn bytes(start: usize, size: usize) -> AccessRange {
    AccessRange::Buffer {
        offset: start as u64,
        size: size as u64,
    }
}
fn access(resource: usize, code: u8, range: AccessRange) -> AccessDeclaration {
    AccessDeclaration {
        resource_index: resource as u32,
        access_code: code,
        range,
        reads_previous: matches!(code, 0x36 | 0x37),
        ..Default::default()
    }
}
fn graph(resources: usize) -> GraphDeclaration {
    GraphDeclaration {
        resources: (0..resources)
            .map(|index| ResourceDeclaration {
                sharing: rhapsody::resource::ResourceSharing::Exclusive,
                name: format!("buffer-{index}"),
                descriptor: Descriptor::Buffer(BufferDesc {
                    word_at_00: SIZE as u64,
                    word_at_08: 0,
                    bytes_at_0c_through_0e: [0; 3],
                }),
                source: OriginSource::Create,
            })
            .collect(),
        ..Default::default()
    }
}
fn pass(graph: &mut GraphDeclaration, accesses: Vec<AccessDeclaration>, side_effect: bool) {
    graph.passes.push(PassDeclaration {
        queue_class: rhapsody::device::QueueClass::Graphics,
        graph_id: None,
        name: format!("pass-{}", graph.passes.len()).into(),
        accesses,
        side_effect,
    });
}
fn definition(graph: &GraphDeclaration) -> GraphDefinition {
    GraphDefinition::from_declaration(graph, &mut AccessTable::original(), PlanContext::default())
        .unwrap()
}
fn plan(graph: &GraphDeclaration) -> CompiledPlan {
    compile_definition(definition(graph)).unwrap()
}
fn export(graph: &mut GraphDeclaration, resource: usize) {
    graph.exports.push(ExportDeclaration {
        tag: 1,
        resource_index: resource as u32,
        access_code: READ,
        range: AccessRange::Whole,
    });
}

#[test]
fn exact_partial_producers_are_kept_and_dead_work_is_culled() {
    let mut graph = graph(3);
    pass(&mut graph, vec![access(0, WRITE, bytes(0, 8))], false);
    pass(&mut graph, vec![access(0, WRITE, bytes(8, 8))], false);
    pass(
        &mut graph,
        vec![
            access(0, READ, AccessRange::Whole),
            access(1, WRITE, AccessRange::Whole),
        ],
        false,
    );
    pass(
        &mut graph,
        vec![access(2, WRITE, AccessRange::Whole)],
        false,
    );
    pass(&mut graph, Vec::new(), true);
    export(&mut graph, 1);
    let plan = plan(&graph);
    assert_eq!(plan.retained_passes, [0, 1, 2, 4]);
    let mut corrupted = plan.clone();
    corrupted.retained_passes.remove(0);
    assert!(
        check_plan(&plan.definition, &corrupted)
            .unwrap_err()
            .to_string()
            .contains("culled")
    );
}

#[test]
fn dependency_paths_accept_transitive_edges_and_reject_missing_anti_dependencies() {
    let mut graph = graph(1);
    pass(
        &mut graph,
        vec![access(0, WRITE, AccessRange::Whole)],
        false,
    );
    pass(&mut graph, vec![access(0, READ, AccessRange::Whole)], true);
    pass(
        &mut graph,
        vec![access(0, WRITE, AccessRange::Whole)],
        false,
    );
    pass(&mut graph, vec![access(0, READ, AccessRange::Whole)], true);
    let original = plan(&graph);
    let mut transitive = original.clone();
    transitive
        .dependency_edges
        .retain(|edge| *edge != (0, 2) && *edge != (0, 3));
    check_plan(&transitive.definition, &transitive).unwrap();
    let mut missing = original.clone();
    missing.dependency_edges.retain(|edge| *edge != (1, 2));
    assert!(
        check_plan(&missing.definition, &missing)
            .unwrap_err()
            .to_string()
            .contains("dependency path")
    );
    let mut backwards = original.clone();
    backwards.retained_passes.swap(0, 1);
    assert!(check_plan(&backwards.definition, &backwards).is_err());
}

#[test]
fn read_modify_write_keeps_previous_content_and_later_overwrites_keep_anti_dependencies() {
    let mut graph = graph(1);
    pass(
        &mut graph,
        vec![access(0, WRITE, AccessRange::Whole)],
        false,
    );
    pass(&mut graph, vec![access(0, 0x37, AccessRange::Whole)], false);
    pass(&mut graph, vec![access(0, READ, AccessRange::Whole)], true);
    pass(
        &mut graph,
        vec![access(0, WRITE, AccessRange::Whole)],
        false,
    );
    pass(&mut graph, vec![access(0, READ, AccessRange::Whole)], true);
    let compiled = plan(&graph);
    assert!(compiled.definition.passes[1].accesses[0].reads_previous);
    assert_eq!(compiled.retained_passes, [0, 1, 2, 3, 4]);
    let mut corrupt = compiled.clone();
    corrupt.retained_passes.remove(0);
    assert!(
        check_plan(&corrupt.definition, &corrupt)
            .unwrap_err()
            .to_string()
            .contains("culled")
    );
    let mut corrupt = compiled.clone();
    corrupt.dependency_edges.retain(|edge| *edge != (2, 3));
    assert!(
        check_plan(&corrupt.definition, &corrupt)
            .unwrap_err()
            .to_string()
            .contains("dependency path")
    );
    let mut corrupt = compiled.definition.clone();
    corrupt.passes[0].accesses[0].reads_previous = true;
    assert!(
        check_definition(&corrupt)
            .unwrap_err()
            .to_string()
            .contains("initialized source")
    );
    let mut corrupt = compiled.definition;
    corrupt.passes[2].accesses[0].reads_previous = true;
    assert!(check_definition(&corrupt).is_err());
}

#[test]
fn public_definitions_reject_unresolved_versions_unknown_sources_and_invalid_indices() {
    let mut graph = graph(1);
    pass(&mut graph, vec![access(0, WRITE, AccessRange::Whole)], true);
    let valid = definition(&graph);
    let mut bad = valid.clone();
    bad.passes[0].accesses[0].version = AccessVersion::Automatic;
    assert!(check_definition(&bad).is_err());
    assert!(compile_definition(bad).is_err());
    let mut bad = valid.clone();
    bad.passes[0].accesses[0].resource_index = u32::MAX;
    assert!(compile_definition(bad).is_err());
    let mut bad = valid.clone();
    bad.passes[0].accesses[0].access_code = READ;
    bad.passes[0].accesses[0].version = AccessVersion::Read(0);
    bad.access_semantics
        .push(rhapsody::rg::plan::AccessSemantics {
            code: READ,
            mask: 0,
            write: false,
            stages: ash::vk::PipelineStageFlags::TRANSFER,
            access: ash::vk::AccessFlags::TRANSFER_READ,
            layout: ash::vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        });
    assert!(
        check_definition(&bad)
            .unwrap_err()
            .to_string()
            .contains("initialized source")
    );
    let mut bad = valid.clone();
    bad.access_semantics.push(bad.access_semantics[0]);
    assert!(check_definition(&bad).is_err());
    let mut bad = valid.clone();
    bad.passes[0].accesses[0].range = bytes(SIZE, 1);
    assert!(check_definition(&bad).is_err());
    let mut bad = valid.clone();
    bad.passes[0].graph_id = Some(rhapsody::rg::graph::RenderGraph::new().graph_id());
    assert!(check_definition(&bad).is_err());
    let mut bad = valid.clone();
    bad.compiler_version += 1;
    assert!(check_definition(&bad).is_err());
    let mut bad = valid;
    bad.exports.push(ExportDeclaration {
        tag: 1,
        resource_index: u32::MAX,
        access_code: WRITE,
        range: AccessRange::Whole,
    });
    assert!(compile_definition(bad).is_err());
}

#[test]
fn executable_projection_rejects_lost_accesses_barriers_and_batch_coverage() {
    let mut graph = graph(1);
    pass(
        &mut graph,
        vec![access(0, WRITE, AccessRange::Whole)],
        false,
    );
    pass(&mut graph, vec![access(0, READ, AccessRange::Whole)], true);
    let original = plan(&graph);
    let mut bad = original.clone();
    let access = bad.range_program.passes[1].uses.start;
    bad.range_program.uses[access].scope.stages = ash::vk::PipelineStageFlags::empty();
    assert!(check_plan(&bad.definition, &bad).is_err());
    let mut bad = original.clone();
    std::sync::Arc::make_mut(&mut bad.info).accesses[1].previous_access_index = u32::MAX;
    assert!(check_plan(&bad.definition, &bad).is_err());
    let mut bad = original.clone();
    std::sync::Arc::make_mut(&mut bad.info).lifetimes[0]
        .last_pass
        .pass_index = 0;
    assert!(check_plan(&bad.definition, &bad).is_err());
    let mut bad = original.clone();
    std::sync::Arc::make_mut(&mut bad.info)
        .lifetime_events
        .clear();
    assert!(check_plan(&bad.definition, &bad).is_err());
    let mut bad = original.clone();
    std::sync::Arc::make_mut(&mut bad.info).buffer_usage_flags[0] = 0;
    assert!(check_plan(&bad.definition, &bad).is_err());
    let mut bad = original.clone();
    std::sync::Arc::make_mut(&mut bad.snapshot).pass_order[0].push('!');
    assert!(check_plan(&bad.definition, &bad).is_err());
    let mut bad = original.clone();
    std::sync::Arc::make_mut(&mut bad.snapshot).batches[0]
        .pass_range
        .end -= 1;
    assert!(check_plan(&bad.definition, &bad).is_err());
    let mut bad = original;
    bad.batches[0].pass_range.end -= 1;
    assert!(check_plan(&bad.definition, &bad).is_err());
}

#[test]
fn illegal_image_usage_is_rejected_even_when_the_pass_would_be_dead() {
    let descriptor = rhapsody::rg::vulkan::ResourceSpec::Image(
        rhapsody::raw::texture::TextureDesc::new_2d(8, 8, ash::vk::Format::R8G8B8A8_UNORM),
    )
    .descriptor();
    let mut graph = graph(1);
    graph.resources[0].descriptor = descriptor;
    pass(
        &mut graph,
        vec![access(0, WRITE, AccessRange::Whole)],
        false,
    );
    let mut definition = definition(&graph);
    definition.access_semantics[0].mask = 0x2000;
    assert!(
        check_definition(&definition)
            .unwrap_err()
            .to_string()
            .contains("image access mask")
    );
    assert!(compile_definition(definition).is_err());
}

#[test]
fn texture_provenance_tracks_aspects_mips_layers_and_partial_overwrites() {
    use ash::vk;
    let descriptor = rhapsody::rg::vulkan::ResourceSpec::Image(
        rhapsody::raw::texture::TextureDesc::new_2d_array(16, 16, 2, vk::Format::D24_UNORM_S8_UINT)
            .mip_levels(2),
    )
    .descriptor();
    let mut graph = graph(1);
    graph.resources[0].descriptor = descriptor;
    for aspect_mask in [
        vk::ImageAspectFlags::DEPTH.as_raw(),
        vk::ImageAspectFlags::STENCIL.as_raw(),
    ] {
        for base_mip_level in 0..2 {
            for base_array_layer in 0..2 {
                pass(
                    &mut graph,
                    vec![access(
                        0,
                        WRITE,
                        AccessRange::Texture {
                            aspect_mask,
                            base_mip_level,
                            level_count: 1,
                            base_array_layer,
                            layer_count: 1,
                        },
                    )],
                    false,
                );
            }
        }
    }
    // Replace one cell; its old writer becomes dead while all other cells remain.
    pass(
        &mut graph,
        vec![access(
            0,
            WRITE,
            AccessRange::Texture {
                aspect_mask: vk::ImageAspectFlags::DEPTH.as_raw(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        )],
        false,
    );
    graph.exports.push(ExportDeclaration {
        tag: 0,
        resource_index: 0,
        access_code: 0x25,
        range: AccessRange::Whole,
    });
    let compiled = plan(&graph);
    assert_eq!(compiled.retained_passes, (1..9).collect::<Vec<_>>());
    let mut corrupted = compiled.definition.clone();
    // Leave depth mip 1 / layer 1 undefined by redirecting its writer elsewhere.
    corrupted.passes[3].accesses[0].range = corrupted.passes[0].accesses[0].range;
    assert!(
        check_definition(&corrupted)
            .unwrap_err()
            .to_string()
            .contains("initialized source")
    );
    let mut corrupted = compiled.clone();
    corrupted.retained_passes.retain(|&index| index != 6);
    assert!(
        check_plan(&corrupted.definition, &corrupted)
            .unwrap_err()
            .to_string()
            .contains("culled")
    );
}

#[derive(Clone, Debug)]
enum Op {
    Add {
        resource: usize,
        start: usize,
        len: usize,
        value: u8,
    },
    Fill {
        resource: usize,
        start: usize,
        len: usize,
        value: u8,
    },
    Copy {
        source: usize,
        target: usize,
        read: usize,
        write: usize,
        len: usize,
    },
    Observe {
        resource: usize,
        start: usize,
        len: usize,
    },
}
fn append(graph: &mut GraphDeclaration, operations: &mut Vec<Op>, op: Op) {
    match op {
        Op::Fill {
            resource,
            start,
            len,
            ..
        } => pass(
            graph,
            vec![access(resource, WRITE, bytes(start, len))],
            false,
        ),
        Op::Copy {
            source,
            target,
            read,
            write,
            len,
        } => pass(
            graph,
            vec![
                access(source, READ, bytes(read, len)),
                access(target, WRITE, bytes(write, len)),
            ],
            false,
        ),
        Op::Observe {
            resource,
            start,
            len,
        } => pass(graph, vec![access(resource, READ, bytes(start, len))], true),
        Op::Add {
            resource,
            start,
            len,
            ..
        } => pass(
            graph,
            vec![access(resource, 0x37, bytes(start, len))],
            false,
        ),
    }
    operations.push(op);
}
fn interpret(
    operations: &[Op],
    schedule: impl Iterator<Item = usize>,
) -> ([u8; SIZE], Vec<(usize, Vec<u8>)>) {
    let mut memory = [[0u8; SIZE]; 3];
    let mut observed = Vec::new();
    for index in schedule {
        match operations[index] {
            Op::Fill {
                resource,
                start,
                len,
                value,
            } => memory[resource][start..start + len].fill(value),
            Op::Copy {
                source,
                target,
                read,
                write,
                len,
            } => {
                let input = memory[source][read..read + len].to_vec();
                memory[target][write..write + len].copy_from_slice(&input);
            }
            Op::Observe {
                resource,
                start,
                len,
            } => observed.push((index, memory[resource][start..start + len].to_vec())),
            Op::Add {
                resource,
                start,
                len,
                value,
            } => {
                for byte in &mut memory[resource][start..start + len] {
                    *byte = byte.wrapping_add(value);
                }
            }
        }
    }
    (memory[0], observed)
}
fn random(state: &mut u64, limit: usize) -> usize {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state % limit as u64) as usize
}

#[test]
fn randomized_partial_range_graphs_match_conservative_execution_outputs() {
    use rhapsody::rg::subgraph::{
        SubgraphDeclaration, SubgraphInterface, SubgraphPlanCache, SubgraphPort,
    };
    let mut total_culled = 0;
    let mut partial_writes = 0;
    for seed in 1..=256 {
        let mut state = seed * 0x9e37_79b9;
        let mut graph = graph(3);
        graph.batch_per_pass = seed % 2 == 0;
        let mut operations = Vec::new();
        for resource in 0..3 {
            append(
                &mut graph,
                &mut operations,
                Op::Fill {
                    resource,
                    start: 0,
                    len: SIZE,
                    value: (resource + seed as usize) as u8,
                },
            );
        }
        for _ in 0..24 {
            let resource = random(&mut state, 3);
            let start = random(&mut state, SIZE);
            let len = 1 + random(&mut state, SIZE - start);
            let op = match random(&mut state, 5) {
                0 | 1 => {
                    partial_writes += usize::from(len != SIZE);
                    Op::Fill {
                        resource,
                        start,
                        len,
                        value: random(&mut state, 256) as u8,
                    }
                }
                2 => Op::Copy {
                    source: resource,
                    target: (resource + 1 + random(&mut state, 2)) % 3,
                    read: start,
                    write: random(&mut state, SIZE - len + 1),
                    len,
                },
                3 => Op::Add {
                    resource,
                    start,
                    len,
                    value: random(&mut state, 256) as u8,
                },
                _ => Op::Observe {
                    resource,
                    start,
                    len,
                },
            };
            append(&mut graph, &mut operations, op);
        }
        export(&mut graph, 0);
        let compiled = plan(&graph);
        total_culled += operations.len() - compiled.retained_passes.len();
        assert_eq!(
            interpret(&operations, 0..operations.len()),
            interpret(&operations, compiled.retained_passes.iter().copied()),
            "seed {seed}\n{}",
            compiled.dump(),
        );
        let mut definition = compiled.definition.clone();
        let version_at = |resource, boundary| {
            definition.passes[..boundary]
                .iter()
                .flat_map(|pass| &pass.accesses)
                .filter(|access| access.resource_index == resource)
                .filter_map(|access| match access.version {
                    AccessVersion::Write { current, .. } => Some(current),
                    _ => None,
                })
                .next_back()
                .unwrap_or(0)
        };
        let scopes = (0..operations.len())
            .step_by(3)
            .map(|start| {
                let end = (start + 3).min(operations.len());
                SubgraphDeclaration {
                    name: format!("scope-{start}"),
                    pass_range: start..end,
                    interface: SubgraphInterface {
                        inputs: (0..3)
                            .map(|resource| SubgraphPort {
                                resource,
                                range: AccessRange::Whole,
                                access_code: READ,
                                version: version_at(resource, start),
                            })
                            .collect(),
                        outputs: (0..3)
                            .map(|resource| SubgraphPort {
                                resource,
                                range: AccessRange::Whole,
                                access_code: WRITE,
                                version: version_at(resource, end),
                            })
                            .collect(),
                        ..Default::default()
                    },
                }
            })
            .collect();
        definition.subgraphs = scopes;
        let mut cache = SubgraphPlanCache::default();
        let composed = compile_with_subgraphs(definition.clone(), &mut cache).unwrap();
        assert_eq!(
            composed.retained_passes, compiled.retained_passes,
            "seed {seed}"
        );
        assert_eq!(
            composed.dependency_edges, compiled.dependency_edges,
            "seed {seed}"
        );
        assert_eq!(
            composed.range_program, compiled.range_program,
            "seed {seed}"
        );
        let previous_hits = cache.stats().hits;
        let reused = compile_with_subgraphs(definition.clone(), &mut cache).unwrap();
        assert_eq!(
            cache.stats().hits - previous_hits,
            definition.subgraphs.len() as u64
        );
        assert_eq!(
            interpret(&operations, 0..operations.len()),
            interpret(&operations, reused.retained_passes.iter().copied())
        );
    }
    assert!(
        total_culled > 256,
        "random fixtures must exercise real culling"
    );
    assert!(
        partial_writes > 2000,
        "random fixtures must exercise partial writes"
    );
}
