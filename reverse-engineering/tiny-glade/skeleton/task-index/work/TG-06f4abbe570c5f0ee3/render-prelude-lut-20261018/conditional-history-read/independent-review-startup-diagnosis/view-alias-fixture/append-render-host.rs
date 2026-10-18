/// Graph declaration regression only: GraphPasses needs a real headless device
/// and ShaderHost entry, but this test never submits or dispatches GPU work.
#[test]
fn inactive_compute_views_keep_active_mips_with_interleaved_buffer_binding() {
    use ash::vk;
    use country_core::render::{
        pass_builder::{
            compute_pass_builder::{
                AccessRange as BindingRange, DispatchMode, RecordedComputePass,
            },
            pass_resource_access::{PassBinding, RecordingPassBuilder, ResourceAccess},
        },
        shader::boundary::ShaderBackend,
    };
    use rhapsody::{
        resource::OwnershipRange,
        rg::{
            graph::{BufferDesc, BuildError, Descriptor, OriginSource, RenderGraph},
            plan::{GraphDefinition, PlanContext, compile_definition},
            plan_check::check_plan,
            resource::{AccessRange, ResourceContractError, ResourceOrigin},
            resource_contracts::validate_contracts,
            vulkan::{AccessTable, ResourceSpec},
        },
        shader_pipeline::{
            compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
            reflection::{
                Binding, BindingArray, BindingKind, BindingLocation, BindingNameHash,
                ShaderReflection,
            },
        },
    };
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{BufferHost, GraphPasses, HostTables, ShaderHost, TextureHost};
    let Some(device) = graphics_device() else {
        return;
    };
    // Reuse the existing diagnostic compute package. The two sampled slots are
    // declared for this host-mapping fixture; no texel/shader behavior is tested.
    let slots = [
        (0, BindingKind::StorageBuffer, "out_data"),
        (1, BindingKind::SampledImage, "inactive_mip"),
        (2, BindingKind::SampledImage, "active_mip"),
    ];
    let compiled = CompiledShaderPipeline {
        stages: vec![CompiledShaderStage {
            stage: ShaderStage::Compute,
            entry_point: "cs_main".into(),
            spirv: compute_words(false),
        }],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![
                slots
                    .iter()
                    .map(|&(binding, kind, name)| {
                        (
                            binding,
                            Binding {
                                kind,
                                array: BindingArray::Fixed(1),
                                name: name.into(),
                            },
                        )
                    })
                    .collect(),
            ],
            uniform_block: None,
            binding_names: slots
                .iter()
                .map(|&(binding, _, name)| (name.into(), BindingLocation { set: 0, binding }))
                .collect(),
            binding_name_hashes: slots
                .iter()
                .map(|&(binding, _, name)| {
                    (
                        BindingNameHash::new(name).0,
                        BindingLocation { set: 0, binding },
                    )
                })
                .collect(),
            vertex_inputs: vec![],
            local_size: Some([4, 1, 1]),
            push_constant_size: 0,
        },
        trailing_bytes: vec![],
    };
    let shaders = Arc::new(ShaderHost::new(device.clone(), std::env::temp_dir()).unwrap());
    let pipeline = shaders
        .create_compute_pipeline(
            "inactive view mapping",
            Arc::new(country_core::rhapsody_types::CompiledShaderPipeline(
                compiled.encode().unwrap(),
            )),
        )
        .unwrap();
    let tables = HostTables {
        buffers: Arc::new(BufferHost::new(device.clone())),
        textures: Arc::new(TextureHost::new(device.clone())),
        shaders,
    };
    let mip = |level| AccessRange::Texture {
        aspect_mask: 1,
        base_mip_level: level,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    };
    for inactive_first in [true, false] {
        for initialize_active in [false, true] {
            let graph = Rc::new(RefCell::new(RenderGraph::new()));
            let history = graph
                .borrow_mut()
                .create_raw_resource(
                    "two-mip history",
                    ResourceOrigin {
                        descriptor: ResourceSpec::Image(
                            rhapsody::raw::texture::TextureDesc::new_2d(
                                16,
                                16,
                                vk::Format::R32_SFLOAT,
                            )
                            .mip_levels(2),
                        )
                        .descriptor(),
                        source: OriginSource::Create,
                    },
                )
                .unwrap();
            let output = graph
                .borrow_mut()
                .create_raw_resource(
                    "fixture output",
                    ResourceOrigin {
                        descriptor: Descriptor::Buffer(BufferDesc {
                            word_at_00: 16,
                            word_at_08: 0,
                            bytes_at_0c_through_0e: [0; 3],
                        }),
                        source: OriginSource::Create,
                    },
                )
                .unwrap();
            if initialize_active {
                let mut borrowed = graph.borrow_mut();
                let mut writer = borrowed.add_pass("initialize only active mip");
                writer
                    .add_resource_access_range(history, 0x34, 0, mip(1))
                    .unwrap();
                writer.render(|_| true);
            }
            let levels = if inactive_first { [0, 1] } else { [1, 0] };
            let image = |level| PassBinding::ImageView {
                resource: history.raw(),
                write: false,
                view: country_core::rhapsody_types::TextureViewDesc::new()
                    .aspect_mask(1)
                    .base_mip_level(level)
                    .level_count(Some(1))
                    .layer_count(Some(1)),
            };
            let name_binding = |level| if level == 0 { 1 } else { 2 };
            let mut passes = GraphPasses::new(graph.clone(), device.clone())
                .unwrap()
                .with_tables(tables.clone());
            passes
                .add_compute(
                    RecordedComputePass {
                        name: "two views with a buffer between",
                        builder: RecordingPassBuilder {
                            name: "two views with a buffer between",
                            accesses: vec![
                                ResourceAccess {
                                    resource: history.raw(),
                                    access: 0x19,
                                    group: 1,
                                },
                                ResourceAccess {
                                    resource: output.raw(),
                                    access: 0x37,
                                    group: 0,
                                },
                                ResourceAccess {
                                    resource: history.raw(),
                                    access: 0x19,
                                    group: 1,
                                },
                            ],
                            ..Default::default()
                        },
                        bindings: vec![
                            image(levels[0]),
                            PassBinding::Buffer {
                                resource: output.raw(),
                            },
                            image(levels[1]),
                        ],
                        access_ranges: vec![
                            BindingRange {
                                resource_index: name_binding(levels[0]),
                                access_index: 0,
                            },
                            BindingRange {
                                resource_index: 0,
                                access_index: 1,
                            },
                            BindingRange {
                                resource_index: name_binding(levels[1]),
                                access_index: 2,
                            },
                        ],
                        inactive_texture_bindings: vec![if inactive_first { 0 } else { 2 }],
                        preserving_buffers: vec![],
                        external_bindings: vec![],
                        uniform_binder: None,
                        push_constants: vec![],
                        program: None,
                        dispatch: DispatchMode::Threads { x: 4, y: 1, z: 1 },
                    },
                    &pipeline,
                )
                .unwrap();
            drop(passes);
            graph.borrow_mut().export(output, 0x26).unwrap();
            let (declaration, _, _) = Rc::try_unwrap(graph)
                .ok()
                .unwrap()
                .into_inner()
                .build()
                .into_parts();
            let compute_index = usize::from(initialize_active);
            let accesses = &declaration.passes[compute_index].accesses;
            assert_eq!(accesses.len(), 3);
            for (index, level) in [(0, levels[0]), (2, levels[1])] {
                let access = accesses[index];
                assert_eq!(access.resource_index, history.raw());
                assert_eq!(access.range, mip(level));
                assert_eq!(
                    (access.access_code, access.synchronization_group),
                    (0x19, 1)
                );
                assert_eq!(access.contents_unused, level == 0);
            }
            assert_eq!(accesses[1].resource_index, output.raw());
            assert!(!accesses[1].contents_unused);
            if !initialize_active {
                assert!(
                    matches!(validate_contracts(&declaration, rhapsody::raw::access_table::is_write),
                    Err(BuildError::Contract(ResourceContractError::UninitializedRead { resource, pass }))
                    if resource == history.raw() && pass == 0)
                );
                continue;
            }
            let definition = GraphDefinition::from_declaration(
                &declaration,
                &mut AccessTable::original(),
                PlanContext::default(),
            )
            .unwrap();
            let plan = compile_definition(definition).unwrap();
            assert_eq!(plan.retained_passes, [0, 1]);
            assert_ne!(
                plan.info.image_usage_flags[history.raw() as usize]
                    & vk::ImageUsageFlags::SAMPLED.as_raw(),
                0
            );
            let lifetime = plan.info.lifetimes[history.raw() as usize];
            assert_eq!(
                (
                    lifetime.first_pass.pass_index,
                    lifetime.last_pass.pass_index
                ),
                (0, 1)
            );
            let native_compute = &plan.range_program.passes[1];
            for level in [0, 1] {
                assert!(
                    plan.range_program.uses[native_compute.uses.clone()]
                        .iter()
                        .any(|usage| usage.resource == history.raw()
                            && usage.range
                                == OwnershipRange::Image {
                                    aspects: 1,
                                    base_mip: level,
                                    mip_count: 1,
                                    base_layer: 0,
                                    layer_count: 1
                                })
                );
            }
            check_plan(&plan.definition, &plan).unwrap();
        }
    }
}
