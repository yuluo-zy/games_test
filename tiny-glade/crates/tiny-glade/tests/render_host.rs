//! core 的 `GraphicsDevice` 接缝经宿主落到 rhapsody 的冒烟验证；无 Vulkan 设备时跳过（本机可用 lavapipe）。
use country_core::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice};
use rhapsody::raw::{device::GraphicsDevice as RhDevice, init::create_headless_device};
use tiny_glade::render_host::BufferHost;

fn host() -> Option<BufferHost> {
    let device = create_headless_device().ok()?;
    Some(BufferHost::new(RhDevice::from_device(device)))
}

#[test]
fn core_buffer_roundtrip_through_rhapsody() {
    let Some(host) = host() else { return };
    // usage 0x20 = STORAGE_BUFFER，memory 2 = 主机可见。
    let desc = BufferDesc::new(32, 0x20 | 0x2, 2);
    let buffer = host.create_buffer(&desc, "ssbo", Some(&[1, 2, 3, 4])).unwrap();
    assert_eq!(buffer.byte_size, 32);
    host.update_buffer_bytes(&buffer, &[8, 8], &[0..2], 4).unwrap();
    let mapped = host.mapped_slice_mut(&buffer).expect("子分配缓冲持久映射");
    assert_eq!(&mapped[..6], &[1, 2, 3, 4, 8, 8]);
    assert!(host.release(&buffer));
    // 句柄失效后拒绝，且槽位复用时代数不同。
    assert!(host.update_buffer_bytes(&buffer, &[1], &[0..1], 0).is_err());
    let again = host.create_buffer(&desc, "ssbo2", None).unwrap();
    assert_eq!(again.id, buffer.id);
    assert_ne!(again.generation, buffer.generation);
}

#[test]
fn core_texture_creates_through_rhapsody_with_kind_and_format_bytes() {
    use country_core::render::load_texture::boundary::{TextureDesc, TextureDevice};
    use tiny_glade::render_host::TextureHost;
    let Some(device) = create_headless_device().ok() else { return };
    let host = TextureHost::new(RhDevice::from_device(device));
    // 文件尾约定：kind 2 = 2D，format 37 = R8G8B8A8_UNORM。
    let desc = TextureDesc::new([4, 4, 1], 2, 37).sampled();
    let pixels = vec![0x7fu8; 4 * 4 * 4];
    let texture = host.create_texture("t", &desc, &pixels).unwrap();
    assert_eq!((texture.width, texture.height, texture.format), (4, 4, 37));
    let managed = host.get(&texture).expect("live handle");
    assert_eq!(managed.desc.kind as u8, 2);
    assert!(host.release(&texture));
    assert!(host.get(&texture).is_none());
    // 未知 kind 不被猜测。
    assert!(host.create_texture("bad", &TextureDesc::new([1, 1, 1], 9, 37).sampled(), &[]).is_err());
}

#[test]
fn core_compute_pipeline_loads_compiled_bin_and_creates_vulkan_pipeline() {
    use country_core::render::shader::boundary::{ComputePipelineDesc, ShaderBackend};
    use rhapsody::shader_pipeline::{
        compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
        pipeline_desc::ComputePipelineDesc as FileDesc,
        reflection::ShaderReflection,
        shader_defines::ShaderDefines,
    };
    use std::sync::Arc;
    use tiny_glade::render_host::ShaderHost;
    let Some(device) = create_headless_device().ok() else { return };
    // 空 compute main（local size 1），入口名 `cs_main`。
    const WORDS: &[u32] = &[
        0x07230203, 0x00010000, 0, 6, 0, 0x00020011, 1, 0x0003000e, 0, 1, 0x0005000f, 5, 4,
        0x6d5f7363, 0x006e6961, 0x00060010, 4, 17, 1, 1, 1, 0x00020013, 1, 0x00030021, 2, 1,
        0x00050036, 1, 4, 0, 2, 0x000200f8, 5, 0x000100fd, 0x00010038,
    ];
    let compiled = CompiledShaderPipeline {
        stages: vec![CompiledShaderStage {
            stage: ShaderStage::Compute,
            entry_point: "cs_main".into(),
            spirv: WORDS.to_vec(),
        }],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![],
            uniform_block: None,
            binding_names: vec![],
            binding_name_hashes: vec![],
            vertex_inputs: vec![],
            local_size: Some([1, 1, 1]),
            push_constant_size: 0,
        },
        trailing_bytes: vec![],
    };
    let root = std::env::temp_dir().join(format!("tg-shader-host-{}", std::process::id()));
    let file = FileDesc {
        source_path: "test/empty".into(),
        entry_point: "cs_main".into(),
        defines: ShaderDefines::default(),
    };
    let path = root.join(file.compiled_file_path());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, compiled.encode().unwrap()).unwrap();

    let host = ShaderHost::new(RhDevice::from_device(device), &root).unwrap();
    let desc = ComputePipelineDesc {
        path: "test/empty".into(),
        entry: "cs_main".into(),
        defines: vec![],
    };
    let loaded = host.load_compiled_compute(&desc).unwrap();
    let pipeline = host.create_compute_pipeline("empty", Arc::new(loaded));
    assert!(host.compute_pipeline(&pipeline).is_some());
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn core_render_graph_imports_and_exports_land_in_the_rhapsody_graph() {
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use country_core::render::load_texture::boundary::{TextureDesc, TextureDevice};
    use country_core::rhapsody_types::RenderGraph;
    use rhapsody::rg::graph::RenderGraph as RhGraph;
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{BufferHost, GraphHost, TextureHost};
    let Some(device) = create_headless_device().ok() else { return };
    let gd = RhDevice::from_device(device);
    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd));
    let buffer = buffers.create_buffer(&CoreBufferDesc::new(64, 0x20, 1), "b", None).unwrap();
    let texture = textures
        .create_texture("t", &TextureDesc::new([4, 4, 1], 2, 37).sampled(), &[0u8; 64])
        .unwrap();

    let graph = Rc::new(RefCell::new(RhGraph::new()));
    let mut core = RenderGraph::with_backend(GraphHost::new(graph.clone(), buffers, textures));
    let b = core.import_unowned(buffer.id, 3);
    let t = core.import_unowned(texture.id, 5);
    assert_ne!(b.resource, t.resource);
    core.export(b, 3);
    drop(core);
    let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
    let (declaration, _, _) = built.into_parts();
    assert_eq!(declaration.resources.len(), 2);
    assert_eq!(declaration.exports.len(), 1);
    assert_eq!(declaration.resources[0].descriptor.kind_code(), 1);
    assert_eq!(declaration.resources[1].descriptor.kind_code(), 0);
}

#[test]
fn compute_pass_device_resolves_buffers_views_and_descs() {
    use country_core::render::load_texture::boundary::{TextureDesc, TextureDevice};
    use country_core::render::pass_builder::compute_pass_builder::ComputePassDevice;
    use country_core::render::pass_builder::pass_resource_access::TextureDevice as PassTextureDevice;
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use country_core::rhapsody_contract::ManagedHandle;
    use std::sync::Arc;
    use tiny_glade::render_host::{BufferHost, DeviceHost, ShaderHost, TextureHost};
    let Some(device) = create_headless_device().ok() else { return };
    let gd = RhDevice::from_device(device);
    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd.clone()));
    let shaders = Arc::new(ShaderHost::new(gd, std::env::temp_dir()).unwrap());
    let host = DeviceHost { buffers: buffers.clone(), textures: textures.clone(), shaders };

    let buffer = buffers.create_buffer(&CoreBufferDesc::new(64, 0x20, 1), "b", None).unwrap();
    assert_ne!(host.get_vk_buffer(&ManagedHandle(buffer.id)).unwrap(), 0);
    assert!(host.get_vk_buffer(&ManagedHandle(999)).is_none());

    let texture = textures
        .create_texture("t", &TextureDesc::new([8, 4, 1], 2, 37).sampled(), &[0u8; 8 * 4 * 4])
        .unwrap();
    let handle = ManagedHandle(texture.id);
    let view = host.get_vk_image_view(&handle).unwrap();
    assert_eq!(host.get_vk_image_view(&handle).unwrap(), view, "视图被缓存");
    let desc = host.managed_texture_desc(&handle);
    assert_eq!((desc.extent, desc.kind, desc.format, desc.sampled), ([8, 4, 1], 2, 37, true));
}

#[test]
fn core_passes_execute_on_lavapipe_through_the_rhapsody_session() {
    use country_core::render::rg_helpers::{clear_rg_buffer, rg_buffer_copy_from_slice_u32};
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use country_core::rhapsody_types::RenderGraph;
    use rhapsody::rg::vulkan::{AccessTable, GpuResource, VulkanGraphBackend, VulkanGraphSession};
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{BufferHost, GraphHost, GraphPasses, TextureHost};
    let Some(device) = create_headless_device().ok() else { return };
    let gd = RhDevice::from_device(device.clone());
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let graph = Rc::new(RefCell::new(session.begin_frame().unwrap()));

    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd.clone()));
    // usage：STORAGE(0x20) | TRANSFER_DST(0x2)；memory 2 = 主机可见，便于读回。
    let buffer = buffers.create_buffer(&CoreBufferDesc::new(16, 0x22, 2), "target", None).unwrap();
    let managed = buffers.get_by_id(buffer.id).unwrap();
    session
        .backend
        .bind_import_named("target", tiny_glade::render_host::graph_token(buffer.id), GpuResource::Buffer(managed.shared()))
        .unwrap();

    let mut core = RenderGraph::with_backend(GraphHost::new(graph.clone(), buffers.clone(), textures));
    let imported = core.import_unowned(buffer.id, 0x34);
    let mut passes = GraphPasses::new(graph.clone(), gd).unwrap();
    let resource = imported.resource as u32;
    clear_rg_buffer(&mut passes, resource);
    rg_buffer_copy_from_slice_u32(&mut passes, resource, 16, &[1, 2, 3, 4]);
    drop(core);
    drop(passes);

    let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
    session.execute_frame(built).unwrap();
    session.wait_submits_done(10_000_000_000).unwrap();

    let mut out = [0u8; 16];
    managed.read_bytes(0, &mut out).unwrap();
    let words: Vec<u32> = out.chunks(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
    assert_eq!(words, [1, 2, 3, 4]);
}

/// `out[gid.x] = gid.x + 100`，`local_size = (4,1,1)`，set 0 binding 0 是 storage buffer。
fn compute_words() -> Vec<u32> {
    use rspirv::binary::Assemble;
    use rspirv::dr::Builder;
    use rspirv::spirv::{
        AddressingModel, BuiltIn, Capability, Decoration, ExecutionMode, ExecutionModel,
        FunctionControl, MemoryModel, StorageClass,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let fn_ty = b.type_function(void, vec![]);
    let uint = b.type_int(32, 0);
    let uvec3 = b.type_vector(uint, 3);
    let in_ptr = b.type_pointer(None, StorageClass::Input, uvec3);
    let gid = b.variable(in_ptr, None, StorageClass::Input, None);
    b.decorate(gid, Decoration::BuiltIn, [rspirv::dr::Operand::BuiltIn(BuiltIn::GlobalInvocationId)]);
    let rt_array = b.type_runtime_array(uint);
    b.decorate(rt_array, Decoration::ArrayStride, [rspirv::dr::Operand::LiteralInt32(4)]);
    let block = b.type_struct([rt_array]);
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(block, 0, Decoration::Offset, [rspirv::dr::Operand::LiteralInt32(0)]);
    let sb_ptr = b.type_pointer(None, StorageClass::StorageBuffer, block);
    let buf = b.variable(sb_ptr, None, StorageClass::StorageBuffer, None);
    b.decorate(buf, Decoration::DescriptorSet, [rspirv::dr::Operand::LiteralInt32(0)]);
    b.decorate(buf, Decoration::Binding, [rspirv::dr::Operand::LiteralInt32(0)]);
    let elem_ptr = b.type_pointer(None, StorageClass::StorageBuffer, uint);
    let zero = b.constant_u32(uint, 0);
    let hundred = b.constant_u32(uint, 100);
    let main = b.begin_function(void, None, FunctionControl::NONE, fn_ty).unwrap();
    b.begin_block(None).unwrap();
    let id = b.load(uvec3, None, gid, None, []).unwrap();
    let x = b.composite_extract(uint, None, id, [0]).unwrap();
    let ptr = b.access_chain(elem_ptr, None, buf, [zero, x]).unwrap();
    let value = b.i_add(uint, None, x, hundred).unwrap();
    b.store(ptr, value, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::GLCompute, main, "cs_main", [gid]);
    b.execution_mode(main, ExecutionMode::LocalSize, [4, 1, 1]);
    b.module().assemble()
}

#[test]
fn core_compute_pass_dispatches_on_lavapipe_with_bound_storage_buffer() {
    use country_core::render::pass_builder::compute_pass_builder::{
        AccessRange, DispatchMode, RecordedComputePass,
    };
    use country_core::render::pass_builder::pass_resource_access::{
        PassBinding, RecordingPassBuilder, ResourceAccess,
    };
    use country_core::render::pass_builder::shader_uri::ComputePipelineDesc;
    use country_core::render::shader::boundary::ShaderBackend;
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use country_core::rhapsody_types::RenderGraph;
    use rhapsody::rg::vulkan::{AccessTable, GpuResource, VulkanGraphBackend, VulkanGraphSession};
    use rhapsody::shader_pipeline::{
        compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
        pipeline_desc::ComputePipelineDesc as FileDesc,
        reflection::{Binding, BindingArray, BindingKind, BindingLocation, BindingNameHash, ShaderReflection},
        shader_defines::ShaderDefines,
    };
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{BufferHost, GraphHost, GraphPasses, HostTables, ShaderHost, TextureHost};
    let Some(device) = create_headless_device().ok() else { return };
    let gd = RhDevice::from_device(device.clone());

    // 发行容器：SPIR-V + 存储的反射。
    let compiled = CompiledShaderPipeline {
        stages: vec![CompiledShaderStage {
            stage: ShaderStage::Compute,
            entry_point: "cs_main".into(),
            spirv: compute_words(),
        }],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![vec![(
                0,
                Binding { kind: BindingKind::StorageBuffer, array: BindingArray::Fixed(1), name: "out_data".into() },
            )]],
            uniform_block: None,
            binding_names: vec![("out_data".into(), BindingLocation { set: 0, binding: 0 })],
            binding_name_hashes: vec![(BindingNameHash::new("out_data").0, BindingLocation { set: 0, binding: 0 })],
            vertex_inputs: vec![],
            local_size: Some([4, 1, 1]),
            push_constant_size: 0,
        },
        trailing_bytes: vec![],
    };
    let root = std::env::temp_dir().join(format!("tg-compute-pass-{}", std::process::id()));
    let file = FileDesc { source_path: "test/iota".into(), entry_point: "cs_main".into(), defines: ShaderDefines::default() };
    let path = root.join(file.compiled_file_path());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, compiled.encode().unwrap()).unwrap();

    let shaders = Arc::new(ShaderHost::new(gd.clone(), &root).unwrap());
    let desc = ComputePipelineDesc { path: "test/iota".into(), entry: "cs_main".into(), defines: vec![] };
    let pipeline = shaders.create_compute_pipeline("iota", Arc::new(shaders.load_compiled_compute(&desc).unwrap()));

    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let graph = Rc::new(RefCell::new(session.begin_frame().unwrap()));
    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd.clone()));
    let buffer = buffers.create_buffer(&CoreBufferDesc::new(16, 0x22, 2), "out", None).unwrap();
    let managed = buffers.get_by_id(buffer.id).unwrap();
    session.backend.bind_import_named("out", tiny_glade::render_host::graph_token(buffer.id), GpuResource::Buffer(managed.shared())).unwrap();

    let mut core = RenderGraph::with_backend(GraphHost::new(graph.clone(), buffers.clone(), textures.clone()));
    let imported = core.import_unowned(buffer.id, 0x37);
    let resource = imported.resource as u32;
    let mut passes = GraphPasses::new(graph.clone(), gd)
        .unwrap()
        .with_tables(HostTables { buffers, textures, shaders });
    passes.add_compute(
        RecordedComputePass {
            name: "iota",
            builder: RecordingPassBuilder {
                name: "iota",
                accesses: vec![ResourceAccess { resource, access: 0x37, group: 0 }],
                ..Default::default()
            },
            bindings: vec![PassBinding::Buffer { resource }],
            access_ranges: vec![AccessRange { resource_index: 0, access_index: 0 }],
            external_bindings: vec![],
            uniform_binder: None,
            push_constants: vec![],
            program: None,
            dispatch: DispatchMode::Threads { x: 4, y: 1, z: 1 },
        },
        &pipeline,
    );
    drop(core);
    drop(passes);
    let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
    session.execute_frame(built).unwrap();
    session.wait_submits_done(10_000_000_000).unwrap();

    let mut out = [0u8; 16];
    managed.read_bytes(0, &mut out).unwrap();
    let words: Vec<u32> = out.chunks(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
    assert_eq!(words, [100, 101, 102, 103]);
    std::fs::remove_dir_all(root).ok();
}

fn raster_words() -> (Vec<u32>, Vec<u32>) {
    use rspirv::binary::Assemble;
    use rspirv::dr::{Builder, Operand};
    use rspirv::spirv::{
        AddressingModel, BuiltIn, Capability, Decoration, ExecutionMode, ExecutionModel,
        FunctionControl, MemoryModel, StorageClass,
    };
    // 顶点：`gl_Position = position`（location 0 的 vec4）。
    let vertex = {
        let mut b = Builder::new();
        b.set_version(1, 3);
        b.capability(Capability::Shader);
        b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
        let void = b.type_void();
        let fn_ty = b.type_function(void, vec![]);
        let float = b.type_float(32);
        let vec4 = b.type_vector(float, 4);
        let in_ptr = b.type_pointer(None, StorageClass::Input, vec4);
        let out_ptr = b.type_pointer(None, StorageClass::Output, vec4);
        let position = b.variable(in_ptr, None, StorageClass::Input, None);
        b.decorate(position, Decoration::Location, [Operand::LiteralInt32(0)]);
        let gl_position = b.variable(out_ptr, None, StorageClass::Output, None);
        b.decorate(gl_position, Decoration::BuiltIn, [Operand::BuiltIn(BuiltIn::Position)]);
        let main = b.begin_function(void, None, FunctionControl::NONE, fn_ty).unwrap();
        b.begin_block(None).unwrap();
        let v = b.load(vec4, None, position, None, []).unwrap();
        b.store(gl_position, v, None, []).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();
        b.entry_point(ExecutionModel::Vertex, main, "vs_main", [position, gl_position]);
        b.module().assemble()
    };
    // 片元：输出常量颜色到 location 0。
    let fragment = {
        let mut b = Builder::new();
        b.set_version(1, 3);
        b.capability(Capability::Shader);
        b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
        let void = b.type_void();
        let fn_ty = b.type_function(void, vec![]);
        let float = b.type_float(32);
        let vec4 = b.type_vector(float, 4);
        let out_ptr = b.type_pointer(None, StorageClass::Output, vec4);
        let color = b.variable(out_ptr, None, StorageClass::Output, None);
        b.decorate(color, Decoration::Location, [Operand::LiteralInt32(0)]);
        let one = b.constant_f32(float, 1.0);
        let zero = b.constant_f32(float, 0.0);
        let value = b.constant_composite(vec4, [one, zero, zero, one]);
        let main = b.begin_function(void, None, FunctionControl::NONE, fn_ty).unwrap();
        b.begin_block(None).unwrap();
        b.store(color, value, None, []).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();
        b.entry_point(ExecutionModel::Fragment, main, "ps_main", [color]);
        b.execution_mode(main, ExecutionMode::OriginUpperLeft, []);
        b.module().assemble()
    };
    (vertex, fragment)
}

#[test]
fn core_raster_pipeline_creates_from_key_handle_and_opts_bitfield() {
    use country_core::render::shader::boundary::{
        CompiledShaderPipeline as CoreCompiled, RasterPipelineKey, ShaderBackend,
    };
    use rhapsody::raw::render_pass::{AttachmentKey, RenderPassKey};
    use rhapsody::raw::shader::RasterPipelineOpts;
    use rhapsody::shader_pipeline::{
        compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
        reflection::{ShaderReflection, VertexInput},
        reflection_ty::{ScalarType, Type, VectorType},
    };
    use std::sync::Arc;
    use tiny_glade::render_host::ShaderHost;
    let Some(device) = create_headless_device().ok() else { return };
    let gd = RhDevice::from_device(device);
    let (vs, ps) = raster_words();
    let compiled = CompiledShaderPipeline {
        stages: vec![
            CompiledShaderStage { stage: ShaderStage::Vertex, entry_point: "vs_main".into(), spirv: vs },
            CompiledShaderStage { stage: ShaderStage::Fragment, entry_point: "ps_main".into(), spirv: ps },
        ],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![],
            uniform_block: None,
            binding_names: vec![],
            binding_name_hashes: vec![],
            vertex_inputs: vec![(
                "position".into(),
                VertexInput {
                    location: 0,
                    ty: Type::Vector(VectorType { scalar_ty: ScalarType::Float(32), nscalar: 4 }),
                },
            )],
            local_size: None,
            push_constant_size: 0,
        },
        trailing_bytes: vec![],
    };
    let root = std::env::temp_dir().join(format!("tg-raster-{}", std::process::id()));
    let host = ShaderHost::new(gd.clone(), &root).unwrap();
    // 一个 RGBA8 颜色附件（clear → store）。
    let pass = host
        .passes()
        .create(
            &gd,
            &RenderPassKey {
                attachments: vec![AttachmentKey {
                    format: 37,
                    samples: 1,
                    load: 1,
                    store: 0,
                    stencil_load: 2,
                    stencil_store: 1,
                    initial_layout: 0,
                    final_layout: 2,
                    subpass_layout: 2,
                    depth_stencil: false,
                }],
            },
        )
        .unwrap();
    let mut opts = RasterPipelineOpts::new();
    opts.set_cull_mode(ash::vk::CullModeFlags::BACK);
    opts.set_blend_mode(2);
    let key = RasterPipelineKey { handle: pass, selector: opts.0 };
    let pipeline = host
        .create_raster_pipeline(Arc::new(CoreCompiled(compiled.encode().unwrap())), &key)
        .unwrap();
    let entry = host.raster_entry(&pipeline).unwrap();
    assert_eq!(entry.pass.color_attachments, 1);
    assert_eq!(entry.pipeline.point, ash::vk::PipelineBindPoint::GRAPHICS);
    // 渲染通道句柄无效 → 报错而不是 panic。
    let bad = RasterPipelineKey { handle: 99, selector: 0 };
    assert!(host.create_raster_pipeline(Arc::new(CoreCompiled(compiled.encode().unwrap())), &bad).is_err());
}

#[test]
fn raster_shader_variants_append_the_depth_only_define() {
    use country_core::render::shader::boundary::{RasterPipelineDesc, ShaderBackend};
    use tiny_glade::render_host::ShaderHost;
    let Some(device) = create_headless_device().ok() else { return };
    let host = ShaderHost::new(RhDevice::from_device(device), std::env::temp_dir()).unwrap();
    let desc = RasterPipelineDesc {
        path: "shaders/x".into(),
        entry: "ps_main".into(),
        defines: vec![("A".into(), "1".into())],
    };
    let [v0, v1] = host.raster_shader_variants(&desc);
    assert_eq!(v0.defines, vec![("A".into(), "1".into()), ("SHADER_VARIANT_DEPTH_ONLY".into(), "0".into())]);
    assert_eq!(v1.defines.last().unwrap(), &("SHADER_VARIANT_DEPTH_ONLY".to_owned(), "1".to_owned()));
    assert_eq!((v0.path.as_str(), v0.entry.as_str()), ("shaders/x", "ps_main"));
}
