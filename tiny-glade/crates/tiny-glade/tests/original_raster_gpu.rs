//! Explicit GPU validation fixtures, not recovered gameplay geometry. The mesh
//! test uses a tiny diagnostic SPIR-V shader so instance count changes pixels;
//! the resolve test uses the actual packaged original resolve shader.
#[path = "support/mod.rs"]
mod test_support;
use test_support::ShaderTestDriver as _;
use test_support::BridgeTestDriver as _;
use ash::vk::{self, Handle};
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::{
        mesh_library::{AssetMeshLibrary, MeshAsset},
        shader_library::AssetShaderLibrary,
        ssbo_library::{AssetSsboLibrary, SsboType as LibrarySsbo},
        vao_library::AssetVAOLibrary,
    },
    render::{
        mesh::{Mesh, VertexAttributeValues},
        pass_builder::{
            draw_builder::DrawBuilder,
            draw_stream::{DrawKind, DrawPacket, PassHost, RasterPass, RasterState},
            framebuffer_binding::{AttachmentTexture, ColorAttachment},
        },
        shader::boundary::ShaderBackend,
        ssbo::rhapsody_boundary::{
            BufferDesc as CoreBufferDesc, GraphicsDevice as CoreGraphicsDevice,
        },
        ssbo::{StorageElement, TypedStorageBuffer},
    },
    systems::render_loop::draw_passes::{
        d_main_support::{DrawPassCtx, ShaderLookup},
        draw_lists::{DrawItem, DrawLists},
        final_post,
    },
};
use shader_pipeline::{
    compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
    pipeline_desc::RasterPipelineDesc as FileDesc, reflection::ShaderReflection,
    reflection::VertexInput, reflection_ty::ScalarType, reflection_ty::Type,
    reflection_ty::VectorType, shader_defines::ShaderDefines,
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tiny_glade::render_bridge::{CoreGraphFrame, CoreRenderBridge};

/// Explicit diagnostic input, not a claimed original game storage type.
#[repr(transparent)]
#[derive(Clone, Copy, bytemuck::NoUninit)]
struct FixtureIndirectWord(u32);
impl StorageElement for FixtureIndirectWord {
    const NAME: &'static str = "FixtureIndirectWords";
    const BUFFER_LEN: usize = 16;
}
impl LibrarySsbo for FixtureIndirectWord {
    type Storage = TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn CoreGraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        TypedStorageBuffer::from_created_buffer(
            device
                .create_buffer(
                    &CoreBufferDesc::new_gpu_only(64, 0x122).alignment(4),
                    Self::NAME,
                    initial,
                )
                .unwrap(),
        )
    }
}
impl country_core::render::pass_builder::pass_resource_access::SsboType for FixtureIndirectWord {
    fn rg_buffer_hash() -> u64 {
        country_core::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}

fn device() -> Option<Arc<rhapsody::device::Device>> {
    match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
        Ok(device) => Some(device),
        Err(error) => {
            assert!(
                !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                "strict actual GPU initialization: {error}"
            );
            eprintln!("SKIP actual raster GPU: {error}");
            None
        }
    }
}
fn target(
    device: &Arc<rhapsody::device::Device>,
    width: u32,
    height: u32,
) -> Arc<rhapsody::resource::Texture> {
    Arc::new(
        rhapsody::resource::Texture::new(
            device.clone(),
            &rhapsody::resource::TextureDesc::new_2d(width, height, vk::Format::R8G8B8A8_UNORM)
                .usage(
                    vk::ImageUsageFlags::COLOR_ATTACHMENT
                        | vk::ImageUsageFlags::TRANSFER_SRC
                        | vk::ImageUsageFlags::TRANSFER_DST,
                ),
        )
        .unwrap(),
    )
}
fn attachment(id: u32, target: &rhapsody::resource::Texture) -> AttachmentTexture {
    AttachmentTexture {
        id,
        extent: (target.desc.width, target.desc.height),
        format: target.desc.format.as_raw() as u32,
        sample_count: 1,
    }
}
fn clear(
    frame: &mut CoreGraphFrame<'_>,
    id: rhapsody::rg::ResourceHandle,
    image: Arc<rhapsody::resource::Texture>,
) -> rhapsody::Result<()> {
    frame.with_graph(|graph| {
        let mut p = graph.add_pass("fixture background");
        p.add_resource_access(id, 0x34, 0)?;
        p.render_with_result_context(move |ctx| {
            unsafe {
                image.device().raw_device().cmd_clear_color_image(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    image.raw_handle(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    },
                    &[vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .level_count(1)
                        .layer_count(1)],
                );
            }
            Ok(())
        });
        Ok(())
    })
}
fn readback(
    frame: &mut CoreGraphFrame<'_>,
    id: rhapsody::rg::ResourceHandle,
    image: Arc<rhapsody::resource::Texture>,
) -> rhapsody::Result<Arc<rhapsody::resource::Buffer>> {
    let buffer = Arc::new(rhapsody::resource::Buffer::new(
        image.device().clone(),
        &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
            u64::from(image.desc.width * image.desc.height * 4),
            vk::BufferUsageFlags::TRANSFER_DST,
        ),
    )?);
    let output = frame.import_native_buffer(buffer.clone(), 0)?;
    frame.with_graph(|graph| {
        let buffer = buffer.clone();
        let mut p = graph.add_pass("fixture GPU readback");
        p.add_resource_access(id, 0x25, 0)?;
        p.add_resource_access(output, 0x34, 0)?;
        p.render_with_result_context(move |ctx| {
            unsafe {
                image.device().raw_device().cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    image.raw_handle(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    buffer.raw_handle(),
                    &[vk::BufferImageCopy::default()
                        .image_subresource(
                            vk::ImageSubresourceLayers::default()
                                .aspect_mask(vk::ImageAspectFlags::COLOR)
                                .layer_count(1),
                        )
                        .image_extent(image.desc.extent())],
                );
            }
            Ok(())
        });
        graph.export(output, 0x26)?;
        Ok(())
    })?;
    Ok(buffer)
}
fn pixels(
    bridge: &mut CoreRenderBridge,
    built: tiny_glade::render_bridge::BuiltCoreGraph,
    readback: Arc<rhapsody::resource::Buffer>,
) -> Vec<u8> {
    bridge
        .shader_backend()
        .prepare_test_pipelines(Duration::from_secs(10))
        .unwrap();
    let compiled = bridge.compile_built(built).unwrap();
    let receipt = bridge.execute_after_uploads(compiled).unwrap();
    assert!(!receipt.receipts.is_empty());
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let mut bytes = vec![0; readback.desc.size as usize];
    readback.read_bytes(0, &mut bytes).unwrap();
    bytes
}
fn bridge_host_buffer(
    frame: &mut CoreGraphFrame<'_>,
    words: &[u32],
    name: &str,
) -> rhapsody::Result<rhapsody::rg::ResourceHandle> {
    let buffers = frame.hosts().buffers;
    let buffer = buffers.create_buffer(
        &CoreBufferDesc::new_gpu_only(words.len() * 4, 0x122).alignment(4),
        name,
        Some(bytemuck::cast_slice(words)),
    )?;
    let owner = buffers
        .managed_buffer(&buffer)
        .ok_or_else(|| rhapsody::Error::invalid_usage("fixture indirect owner missing"))?;
    frame.import_native_buffer(owner.shared(), 0x34)
}
fn vertex_fixture() -> Vec<u32> {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let fnty = b.type_function(void, vec![]);
    let f = b.type_float(32);
    let v2 = b.type_vector(f, 2);
    let v4 = b.type_vector(f, 4);
    let u = b.type_int(32, 0);
    let inputptr = b.type_pointer(None, StorageClass::Input, v2);
    let input = b.variable(inputptr, None, StorageClass::Input, None);
    b.decorate(input, Decoration::Location, [Operand::LiteralInt32(3)]);
    let iptr = b.type_pointer(None, StorageClass::Input, u);
    let instance = b.variable(iptr, None, StorageClass::Input, None);
    b.decorate(
        instance,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::InstanceIndex)],
    );
    let outptr = b.type_pointer(None, StorageClass::Output, v4);
    let output = b.variable(outptr, None, StorageClass::Output, None);
    b.decorate(
        output,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::Position)],
    );
    let z = b.constant_f32(f, 0.0);
    let o = b.constant_f32(f, 1.0);
    let main = b
        .begin_function(void, None, FunctionControl::NONE, fnty)
        .unwrap();
    b.begin_block(None).unwrap();
    let p = b.load(v2, None, input, None, []).unwrap();
    let x = b.composite_extract(f, None, p, [0]).unwrap();
    let y = b.composite_extract(f, None, p, [1]).unwrap();
    let index = b.load(u, None, instance, None, []).unwrap();
    let shift = b.convert_u_to_f(f, None, index).unwrap();
    let x = b.f_add(f, None, x, shift).unwrap();
    let value = b.composite_construct(v4, None, [x, y, z, o]).unwrap();
    b.store(output, value, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(
        ExecutionModel::Vertex,
        main,
        "vs_main",
        [input, instance, output],
    );
    b.module().assemble()
}
fn fragment_fixture() -> Vec<u32> {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let fn_ty = b.type_function(void, vec![]);
    let f = b.type_float(32);
    let v = b.type_vector(f, 4);
    let ptr = b.type_pointer(None, StorageClass::Output, v);
    let output = b.variable(ptr, None, StorageClass::Output, None);
    b.decorate(output, Decoration::Location, [Operand::LiteralInt32(0)]);
    let z = b.constant_f32(f, 0.0);
    let o = b.constant_f32(f, 1.0);
    let red = b.constant_composite(v, [o, z, z, o]);
    let main = b
        .begin_function(void, None, FunctionControl::NONE, fn_ty)
        .unwrap();
    b.begin_block(None).unwrap();
    b.store(output, red, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::Fragment, main, "ps_main", [output]);
    b.execution_mode(main, ExecutionMode::OriginUpperLeft, []);
    b.module().assemble()
}
#[test]
fn actual_typed_vao_vertex_index_and_instance_count_change_gpu_pixels() {
    let Some(device) = device() else {
        return;
    };
    let root = std::env::temp_dir().join(format!("tg-mesh-actual-{}", std::process::id()));
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = CoreRenderBridge::new(device.clone(), &root, shaders.clone()).unwrap();
    let desc = country_core::render::shader::boundary::RasterPipelineDesc {
        path: "fixture/vao".into(),
        entry: "ps_main".into(),
        defines: vec![],
    };
    let compiled = CompiledShaderPipeline {
        stages: vec![
            CompiledShaderStage {
                stage: ShaderStage::Vertex,
                entry_point: "vs_main".into(),
                spirv: vertex_fixture(),
            },
            CompiledShaderStage {
                stage: ShaderStage::Fragment,
                entry_point: "ps_main".into(),
                spirv: fragment_fixture(),
            },
        ],
        source_dependencies: vec![],
        trailing_bytes: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![],
            uniform_block: None,
            binding_names: vec![],
            binding_name_hashes: vec![],
            local_size: None,
            push_constant_size: 0,
            vertex_inputs: vec![(
                "Vertex_Position".into(),
                VertexInput {
                    location: 3,
                    ty: Type::Vector(VectorType {
                        scalar_ty: ScalarType::Float(4),
                        nscalar: 2,
                    }),
                },
            )],
        },
    };
    for variant in bridge.shader_backend().raster_shader_variants(&desc) {
        let file = FileDesc {
            source_path: variant.path,
            vertex_entry: String::new(),
            fragment_entry: variant.entry,
            defines: ShaderDefines(variant.defines),
        };
        let path = root.join(file.compiled_file_path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, compiled.encode().unwrap()).unwrap();
    }
    let program = country_core::render::shader::RasterShaderProgram::new_draw(
        "fixture vao",
        desc,
        bridge.shader_backend(),
    );
    let shader = shaders.add_raster(program, false);
    let meshes = AssetMeshLibrary::new();
    let mut mesh = Mesh::new("fixture");
    mesh.indices = vec![0, 1, 2];
    mesh.attributes.insert(
        "Vertex_Position".into(),
        VertexAttributeValues::Float2(vec![
            glam::vec2(-0.85, -0.5),
            glam::vec2(-0.15, -0.5),
            glam::vec2(-0.5, 0.5),
        ]),
    );
    let mesh = meshes.add(MeshAsset::new(mesh, None));
    let mut vaos = AssetVAOLibrary::new();
    let vao = vaos
        .add(bridge.hosts().buffers.as_ref(), &meshes, mesh)
        .unwrap();
    let mut world = World::new();
    world.spawn(vao);
    world.insert_resource(vaos);
    let mut lists = DrawLists::new();
    let mut item = DrawItem::new(shader.id().raw(), 0);
    item.vao = vao.id().raw();
    fn unused(
        _: DrawBuilder<'_>,
        _: &country_core::render::pass_builder::draw_list::EntityDrawParams,
        _: &World,
    ) {
    }
    lists.list_mut(0).push((item, unused));
    bridge
        .raster_resources()
        .refresh_draw_vaos(&mut world, &lists)
        .unwrap();
    let target = target(&device, 64, 64);
    let library = AssetSsboLibrary::new();
    let data = [3, 1, 0, 0, 0, 0, 0, 0, 3, 1, 0, 1, 0, 0, 0, 0].map(FixtureIndirectWord);
    library.add_new_with_data::<FixtureIndirectWord>(bridge.hosts().buffers.as_ref(), &data);
    bridge.raster_resources().assets.lock().ssbo_leases = Some(
        library
            .snapshot_graph_leases()
            .into_iter()
            .map(|lease| (lease.id.0.0, lease))
            .collect(),
    );
    // Persistent diagnostic source deliberately leaves both commands live in
    // its tail across frames. Only the actual GPU counter changes2->1->0->2->3
    // (last clamps to capacity2), so a raw max-count substitution draws stale data.
    let mut count_words = vec![0u32; 4 + 2 * 8];
    for command in 0..2 {
        let start = 4 + command * 8;
        count_words[start..start + 5].copy_from_slice(&[3, 1, 0, 0, command as u32]);
    }
    let buffers = bridge.hosts().buffers.clone();
    let original_commands = buffers
        .create_buffer(
            &CoreBufferDesc::new_gpu_only(count_words.len() * 4, 0x122).alignment(4),
            "fixture persistent counted commands",
            Some(bytemuck::cast_slice(&count_words)),
        )
        .unwrap();
    let actual_count = buffers
        .create_buffer(
            &CoreBufferDesc::new_gpu_only(16, 0x122).alignment(4),
            "fixture persistent GPU draw count",
            Some(&[0u8; 16]),
        )
        .unwrap();
    let original_commands = buffers.managed_buffer(&original_commands).unwrap().shared();
    let actual_count = buffers.managed_buffer(&actual_count).unwrap().shared();
    let mut last_indirect_generation = None;
    // Modes 5/7/8 exercise the native indirect packet mapping. Deliberate
    // padding/nonzero offsets catch a swapped drawCount/stride/offset mapping.
    for (instances, first_instance, mode) in [
        (1, None, 3),
        (2, None, 3),
        (1, Some(1), 4),
        (2, None, 5),
        (2, None, 5),
        (2, None, 7),
        (2, None, 8),
        (1, None, 8),
        (0, None, 8),
        (2, None, 8),
        (3, None, 8),
    ] {
        let (output, built) = bridge
            .build_frame(|frame| {
                let id = frame.import_native_texture(target.clone(), 0)?;
                clear(frame, id, target.clone())?;
                let indirect = if mode == 8 {
                    let commands = frame.import_native_buffer(original_commands.clone(), 0)?;
                    let count = frame.import_native_buffer(actual_count.clone(), 0)?;
                    frame.with_graph(|graph| {
                        let mut pass = graph.add_pass("fixture actual GPU indirect count");
                        pass.add_resource_access_range(
                            count,
                            0x34,
                            0,
                            rhapsody::rg::AccessRange::Buffer { offset: 8, size: 8 },
                        )?;
                        let device = device.clone();
                        let count_owner = actual_count.clone();
                        pass.render_with_result_context(move |ctx| {
                            unsafe {
                                device.raw_device().cmd_fill_buffer(
                                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                                    count_owner.raw_handle(),
                                    8,
                                    4,
                                    instances,
                                );
                                // The second packet's real GPU count is zero.
                                // Reusing the first packet's scratch would erase
                                // its live commands and fail the actual pixels.
                                device.raw_device().cmd_fill_buffer(
                                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                                    count_owner.raw_handle(),
                                    12,
                                    4,
                                    0,
                                );
                            }
                            Ok(())
                        });
                        Ok(())
                    })?;
                    Some((commands, Some(count)))
                } else if mode >= 7 {
                    let indexed = mode != 5;
                    let prefix = if indexed { 4 } else { 0 };
                    let mut words = vec![0u32; prefix + 16];
                    for command in 0..2 {
                        let start = prefix + command * 8;
                        let data = if indexed {
                            vec![3, 1, 0, 0, command as u32]
                        } else {
                            vec![3, 1, 0, command as u32]
                        };
                        words[start..start + data.len()].copy_from_slice(&data);
                    }
                    let owned =
                        bridge_host_buffer(frame, &words, "fixture actual indirect commands")?;
                    let count = if mode == 8 {
                        Some(bridge_host_buffer(
                            frame,
                            &[0, 0, instances, 0],
                            "fixture actual indirect count",
                        )?)
                    } else {
                        None
                    };
                    Some((owned, count))
                } else {
                    None
                };
                frame.with_raster(|host| {
                    let mut state = rhapsody::pipeline::RasterPipelineOpts::new();
                    state.set_cull_mode(vk::CullModeFlags::NONE);
                    state.set_depth_test_enable(false);
                    let mut pass = RasterPass::new(
                        "fixture actual VAO",
                        &[ColorAttachment::Texture(attachment(id.raw(), &target))],
                        None,
                        Box::new(host),
                    )
                    .with_viewport([64, 64]);
                    let mut packet = DrawPacket::new(
                        country_core::rhapsody_contract::Handle(
                            country_core::rhapsody_contract::ManagedHandle(shader.id().raw()),
                            std::marker::PhantomData,
                        ),
                        [1, vao.id().raw()],
                        RasterState(state.0),
                    );
                    if let Some((buffer, count)) = indirect {
                        packet.kind = match mode {
                            5 => DrawKind::MultiDrawIndirect {
                                buffer: buffer.raw(),
                                arg_50: 2,
                                arg_58: 32,
                            },
                            7 => DrawKind::IndexedIndirect {
                                buffer: buffer.raw(),
                                arg_48: 16,
                                arg_50: 2,
                                arg_58: 32,
                                first_index_count: None,
                            },
                            8 => DrawKind::IndexedIndirectCount {
                                buffer: buffer.raw(),
                                count_buffer: count.unwrap().raw(),
                                args: [16, 32, 8, 2],
                            },
                            _ => unreachable!(),
                        };
                        let draw = DrawBuilder::new(&mut pass, packet, 0);
                        match mode {
                            5 => {
                                // Public native draw() preserves kind 5; the
                                // host declares its exact indirect owner read.
                                draw.draw();
                            }
                            7 => draw.draw_indexed_indirect_raw(buffer.raw(), 16, 32, 2),
                            8 => {
                                draw.draw_indexed_indirect_count(
                                    buffer.raw(),
                                    count.unwrap().raw(),
                                    [16, 32, 8, 2],
                                );
                                let second = DrawPacket::new(
                                    country_core::rhapsody_contract::Handle(
                                        country_core::rhapsody_contract::ManagedHandle(
                                            shader.id().raw(),
                                        ),
                                        std::marker::PhantomData,
                                    ),
                                    [1, vao.id().raw()],
                                    RasterState(state.0),
                                );
                                DrawBuilder::new(&mut pass, second, 0).draw_indexed_indirect_count(
                                    buffer.raw(),
                                    count.unwrap().raw(),
                                    [16, 32, 12, 2],
                                );
                            }
                            _ => unreachable!(),
                        }
                    } else if mode == 5 {
                        DrawBuilder::new(&mut pass, packet, 0)
                            .multi_draw_indirect::<FixtureIndirectWord>(2, 32);
                    } else if let Some(first) = first_instance {
                        let draw = DrawBuilder::new(&mut pass, packet, 0);
                        draw.draw_multi_instanced(vec![[first, instances]]);
                    } else {
                        let draw = DrawBuilder::new(&mut pass, packet, 0);
                        draw.draw_instanced_w_gl_draw_flag(instances);
                    }
                    pass.render();
                    Ok(())
                })?;
                if mode == 8 {
                    frame.with_graph(|graph| {
                        let scratch: Vec<_> = graph
                            .declaration()
                            .resources
                            .iter()
                            .enumerate()
                            .filter(|(_, resource)| {
                                resource
                                    .name
                                    .starts_with("platform indexed indirect count commands [")
                            })
                            .map(|(index, resource)| (index, resource.name.as_str()))
                            .collect();
                        let adapted = !device
                            .caps()
                            .is_supported(rhapsody::device::Feature::DrawIndirectCount);
                        assert_eq!(scratch.len(), if adapted { 2 } else { 0 });
                        if adapted {
                            assert_ne!(
                                scratch[0].0, scratch[1].0,
                                "each count packet owns a distinct graph resource"
                            );
                            assert_ne!(
                                scratch[0].1, scratch[1].1,
                                "same-frame resource names must be unique"
                            );
                        }
                        Ok(())
                    })?;
                }
                if mode == 5 {
                    let lease = library.snapshot_graph_leases().pop().unwrap();
                    let imported = lease.rg_handle().expect(
                        "native generic raster consumer must lazily import its actual storage",
                    );
                    if let Some(last) = last_indirect_generation {
                        assert_ne!(
                            last, imported.generation,
                            "next frame must reimport instead of leaking cached graph IDs"
                        );
                    }
                    last_indirect_generation = Some(imported.generation);
                }
                library.export_from_rg(&mut frame.resource_graph());
                assert!(
                    library
                        .snapshot_graph_leases()
                        .iter()
                        .all(|lease| lease.rg_handle().is_none())
                );
                readback(frame, id, target.clone())
            })
            .unwrap();
        let bytes = if mode == 8 {
            bridge
                .shader_backend()
                .prepare_test_pipelines(Duration::from_secs(10))
                .unwrap();
            let compiled = bridge.compile_built(built).unwrap();
            let adapted = !device
                .caps()
                .is_supported(rhapsody::device::Feature::DrawIndirectCount);
            assert_eq!(
                compiled
                    .snapshot()
                    .pass_order
                    .iter()
                    .filter(|name| name.as_str() == "platform indexed indirect count adaptation")
                    .count(),
                if adapted { 2 } else { 0 }
            );
            assert!(
                compiled
                    .snapshot()
                    .pass_order
                    .iter()
                    .any(|name| name == "fixture actual GPU indirect count")
            );
            let receipt = bridge.execute_after_uploads(compiled).unwrap();
            assert!(!receipt.receipts.is_empty());
            bridge.wait_submits_done(10_000_000_000).unwrap();
            let mut bytes = vec![0; output.desc.size as usize];
            output.read_bytes(0, &mut bytes).unwrap();
            bytes
        } else {
            pixels(&mut bridge, built, output)
        };
        let pixel = |x: usize| &bytes[(32 * 64 + x) * 4..][..4];
        assert_eq!(
            pixel(16),
            if first_instance.is_some() || instances == 0 {
                [0, 0, 0, 255]
            } else {
                [255, 0, 0, 255]
            }
        );
        assert_eq!(
            pixel(48),
            if instances == 0 || (instances == 1 && first_instance.is_none()) {
                [0, 0, 0, 255]
            } else {
                [255, 0, 0, 255]
            },
            "actual instanceCount/firstInstance must change the second triangle's GPU pixels"
        );
    }
}

struct OriginalShaders<'a>(&'a AssetShaderLibrary);
impl ShaderLookup for OriginalShaders<'_> {
    fn raster_handle_by_name(
        &self,
        name: &str,
    ) -> Option<
        country_core::rhapsody_contract::Handle<country_core::render::shader::RasterShaderProgram>,
    > {
        self.0.get_raster_handle_by_name(name).map(|h| {
            country_core::rhapsody_contract::Handle(
                country_core::rhapsody_contract::ManagedHandle(h.id().raw()),
                std::marker::PhantomData,
            )
        })
    }
}
#[test]
fn original_resolve_uses_target_dimensions_for_different_source_extent() {
    use country_core::render::load_texture::boundary::TextureDevice;
    let Some(device) = device() else {
        return;
    };
    let root = std::env::var_os("TG_COMPILED_SHADERS")
        .map(|path| {
            PathBuf::from(path)
                .parent()
                .expect("TG_COMPILED_SHADERS must be an installation's compiled-shaders directory")
                .to_path_buf()
        })
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        });
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = CoreRenderBridge::new(device.clone(), root, shaders.clone()).unwrap();
    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
    // Original packaged SPIR-V converts FragCoord.xy to integers and uses
    // OpImageFetch: resolve does not stretch a smaller source to a larger
    // target. Keep every sampled coordinate in bounds. The lower half of this
    // larger source is green, so using source.height instead of target.height
    // for transform_image&1 produces the wrong visible pixels.
    let bytes = (0..16)
        .flat_map(|y| {
            (0..32).flat_map(move |x| {
                if y >= 8 {
                    [0, 255, 0, 255]
                } else if y >= 4 {
                    if x < 8 {
                        [255, 255, 0, 255]
                    } else {
                        [0, 255, 255, 255]
                    }
                } else if x < 8 {
                    [255, 0, 0, 255]
                } else {
                    [0, 0, 255, 255]
                }
            })
        })
        .collect::<Vec<_>>();
    let source = bridge
        .hosts()
        .textures
        .create_texture(
            "fixture source",
            &country_core::rhapsody_types::TextureDesc::new_2d([32, 16], 0x2b).sampled(),
            &bytes,
        )
        .unwrap();
    let target = target(&device, 16, 8);
    let mut world = World::new();
    let lookup = OriginalShaders(&shaders);
    for flag in [0, 1] {
        let (output, built) = bridge
            .build_frame(|frame| {
                let source = frame.import_texture(&source, 0x22)?;
                let target_id = frame.import_native_texture(target.clone(), 0)?;
                clear(frame, target_id, target.clone())?;
                frame.with_raster(|host| {
                    let mut make = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
                    let mut ctx = DrawPassCtx::new(&mut world, Duration::ZERO, &mut make, &lookup);
                    final_post::resolve_to_swapchain(
                        &mut ctx,
                        &AttachmentTexture {
                            id: source.raw(),
                            extent: (32, 16),
                            format: 0x2b,
                            sample_count: 1,
                        },
                        &attachment(target_id.raw(), &target),
                        flag,
                    );
                    Ok(())
                })?;
                readback(frame, target_id, target.clone())
            })
            .unwrap();
        let bytes = pixels(&mut bridge, built, output);
        assert_eq!(
            &bytes[(2 * 16 + 2) * 4..][..4],
            if flag == 0 {
                [255, 0, 0, 255]
            } else {
                [255, 255, 0, 255]
            }
        );
        assert_eq!(
            &bytes[(2 * 16 + 13) * 4..][..4],
            if flag == 0 {
                [0, 0, 255, 255]
            } else {
                [0, 255, 255, 255]
            }
        );
    }
}
