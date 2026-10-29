//! core 的 `GraphicsDevice` 接缝经宿主落到 rhapsody 的冒烟验证；无 Vulkan 设备时跳过（本机可用 lavapipe）。
#[path = "support/mod.rs"]
mod test_support;
use test_support::ShaderTestDriver as _;
use country_core::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice};

use tiny_glade::render_host::BufferHost;

fn graphics_device() -> Option<std::sync::Arc<rhapsody::device::Device>> {
    let result = rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig {
        validation: true,
        ..rhapsody::device::DeviceConfig::headless()
    });
    match result {
        Ok(device) => Some(device),
        Err(error) => {
            assert!(
                !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                "strict tiny host Vulkan initialization failed: {error}"
            );
            eprintln!("SKIP tiny host Vulkan initialization: {error}");
            None
        }
    }
}

fn host() -> Option<BufferHost> {
    let device = graphics_device()?;
    Some(BufferHost::new(device))
}

#[test]
fn core_buffer_roundtrip_through_rhapsody() {
    let Some(host) = host() else { return };
    // usage 0x20 = STORAGE_BUFFER，memory 2 = 主机可见。
    let desc = BufferDesc::new(32, 0x20 | 0x2, 2);
    let buffer = host
        .create_buffer(&desc, "ssbo", Some(&[1, 2, 3, 4]))
        .unwrap();
    assert_eq!(buffer.byte_size, 32);
    host.update_buffer_bytes(&buffer, &[8, 8], &[0..2], 4)
        .unwrap();
    // Owned updates acknowledge admission. Observe only after submission and
    // actual GPU completion, as a production readback consumer must do.
    test_support::until_ready(|| host.poll_pending_uploads()).unwrap();
    let mut actual = [0; 6];
    host.managed_buffer(&buffer).unwrap().read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, [1, 2, 3, 4, 8, 8]);
    assert!(host.release(&buffer));
    // 句柄失效后拒绝，且槽位复用时代数不同。
    assert_eq!(
        host.update_buffer_bytes(&buffer, &[1], &[0..1], 0)
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    let again = host.create_buffer(&desc, "ssbo2", None).unwrap();
    assert_eq!(again.id & 0xffff_ffff, buffer.id & 0xffff_ffff);
    assert_ne!(again.id, buffer.id);
    assert!(host.get_by_id(buffer.id).is_none());
    assert_ne!(again.generation, buffer.generation);
}

#[test]
fn foreign_hosts_and_reused_slots_reject_both_typed_and_id_only_handles() {
    use country_core::render::load_texture::boundary::{TextureDesc, TextureDevice};
    use tiny_glade::render_host::TextureHost;
    let Some(device) = graphics_device() else { return };
    let a = BufferHost::new(device.clone());
    let b = BufferHost::new(device.clone());
    let desc = BufferDesc::new(16, 0x20 | 0x2, 2);
    let first = a.create_buffer(&desc, "first host", None).unwrap();
    let foreign = b.create_buffer(&desc, "second host", None).unwrap();
    assert_eq!(first.id & 0xffff_ffff, foreign.id & 0xffff_ffff);
    assert!(b.managed_buffer(&first).is_none());
    assert!(b.get_by_id(first.id).is_none());
    assert!(!b.release(&first));
    assert!(b.update_buffer_bytes(&first, &[9], &[0..1], 0).is_err());
    assert!(a.release(&first));
    let replacement = a.create_buffer(&desc, "reused slot", None).unwrap();
    assert!(a.get_by_id(first.id).is_none());
    assert!(a.get_by_id(replacement.id).is_some());
    assert_ne!(tiny_glade::render_host::graph_token(first.id),
        tiny_glade::render_host::graph_token(replacement.id));
    let textures_a = TextureHost::new(device.clone());
    let textures_b = TextureHost::new(device);
    let desc = TextureDesc::new([1, 1, 1], 2, 37).sampled();
    let first = textures_a.create_texture("first host", &desc, &[1, 2, 3, 4]).unwrap();
    let second = textures_b.create_texture("second host", &desc, &[5, 6, 7, 8]).unwrap();
    assert_eq!(first.id & 0xffff_ffff, second.id & 0xffff_ffff);
    assert!(textures_b.get(&first).is_none());
    assert!(textures_b.get_by_id(first.id).is_none());
    assert!(!textures_b.release(&first));
    assert!(textures_a.release(&first));
    let replacement = textures_a.create_texture("reused slot", &desc, &[0; 4]).unwrap();
    assert!(textures_a.get_by_id(first.id).is_none());
    assert!(textures_a.get(&replacement).is_some());
    assert!(textures_a.get_by_id(replacement.id & !tiny_glade::render_host::TEXTURE_ID_TAG).is_none());
}

#[test]
fn core_texture_creates_through_rhapsody_with_kind_and_format_bytes() {
    use country_core::render::load_texture::boundary::{TextureDesc, TextureDevice};
    use tiny_glade::render_host::TextureHost;
    let Some(device) = graphics_device() else {
        return;
    };
    let host = TextureHost::new(device);
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
    assert!(
        host.create_texture("bad", &TextureDesc::new([1, 1, 1], 9, 37).sampled(), &[])
            .is_err()
    );
}

#[test]
fn core_compute_pipeline_loads_compiled_bin_and_creates_vulkan_pipeline() {
    use country_core::render::shader::boundary::{ComputePipelineDesc, ShaderBackend};
    use shader_pipeline::{
        compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
        pipeline_desc::ComputePipelineDesc as FileDesc, reflection::ShaderReflection,
        shader_defines::ShaderDefines,
    };
    use tiny_glade::render_host::ShaderHost;
    let Some(device) = graphics_device() else {
        return;
    };
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

    let host = ShaderHost::new(device.clone(), &root).unwrap();
    let foreign_host = ShaderHost::new(device, &root).unwrap();
    let desc = ComputePipelineDesc {
        path: "test/empty".into(),
        entry: "cs_main".into(),
        defines: vec![],
    };
    let loaded = host.load_compiled_compute(&desc).unwrap();
    let program =
        country_core::render::shader::ComputeShaderProgram::try_new("empty", desc.clone(), &host).unwrap();
    let foreign_program = country_core::render::shader::ComputeShaderProgram::try_new(
        "foreign empty", desc, &foreign_host).unwrap();
    let pipeline = program.pipeline;
    drop(loaded);
    // The requested pipeline is registered once; a warm host scans nothing.
    assert_eq!(host.pending_ticket_count(), 1);
    host.prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();
    assert_eq!(host.pending_ticket_count(), 0, "ready tickets leave the registry");
    host.prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();
    assert!(host.compute_pipeline(&pipeline).is_some());
    foreign_host.prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();
    assert!(foreign_host.compute_pipeline(&foreign_program.pipeline).is_some());
    assert_eq!(pipeline.id & 0xffff_ffff, foreign_program.pipeline.id & 0xffff_ffff);
    assert!(foreign_host.compute_entry(&pipeline).is_none());
    assert!(host.compute_entry(&foreign_program.pipeline).is_none());
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn core_render_graph_imports_and_exports_land_in_the_rhapsody_graph() {
    use country_core::render::load_texture::boundary::{TextureDesc, TextureDevice};
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use country_core::rhapsody_types::RenderGraph;
    use rhapsody::rg::RenderGraph as RhGraph;
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{BufferHost, GraphHost, TextureHost};
    let Some(device) = graphics_device() else {
        return;
    };
    let gd = device;
    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd));
    let buffer = buffers
        .create_buffer(&CoreBufferDesc::new(64, 0x20, 1), "b", None)
        .unwrap();
    let texture = textures
        .create_texture(
            "t",
            &TextureDesc::new([4, 4, 1], 2, 37).sampled(),
            &[0u8; 64],
        )
        .unwrap();

    let graph = Rc::new(RefCell::new(RhGraph::new()));
    let mut core = RenderGraph::with_backend(GraphHost::new(graph.clone(), buffers, textures));
    let b = core.import_unowned(buffer.id, 3);
    let t = core.import_unowned(texture.id, 5);
    assert_ne!(b.resource, t.resource);
    core.export(b, 3);
    drop(core);
    let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
    let (declaration, _, _, declaration_error) = built.into_parts();
    assert!(declaration_error.is_none());
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
    let Some(device) = graphics_device() else {
        return;
    };
    let gd = device;
    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd.clone()));
    let shaders = Arc::new(ShaderHost::new(gd, std::env::temp_dir()).unwrap());
    let host = DeviceHost {
        buffers: buffers.clone(),
        textures: textures.clone(),
        shaders,
    };

    let buffer = buffers
        .create_buffer(&CoreBufferDesc::new(64, 0x20, 1), "b", None)
        .unwrap();
    assert_ne!(host.get_vk_buffer(&ManagedHandle(buffer.id)).unwrap(), 0);
    assert!(host.get_vk_buffer(&ManagedHandle(999)).is_none());

    let texture = textures
        .create_texture(
            "t",
            &TextureDesc::new([8, 4, 1], 2, 37).sampled(),
            &[0u8; 8 * 4 * 4],
        )
        .unwrap();
    let handle = ManagedHandle(texture.id);
    let view = host.get_vk_image_view(&handle).unwrap();
    assert_eq!(host.get_vk_image_view(&handle).unwrap(), view, "视图被缓存");
    let desc = host.managed_texture_desc(&handle);
    assert_eq!(
        (desc.extent, desc.kind, desc.format, desc.sampled),
        ([8, 4, 1], 2, 37, true)
    );
}

#[test]
fn core_passes_execute_on_lavapipe_through_the_rhapsody_session() {
    use country_core::render::rg_helpers::{clear_rg_buffer, rg_buffer_copy_from_slice_u32};
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use country_core::rhapsody_types::RenderGraph;
    use rhapsody::{
        rg::AccessTable, rg::GpuResource, rg::VulkanGraphBackend, rg::VulkanGraphSession,
    };
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{BufferHost, GraphHost, GraphPasses, TextureHost};
    let Some(device) = graphics_device() else {
        return;
    };
    let gd = device.clone();
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let session = Rc::new(RefCell::new(VulkanGraphSession::new(backend)));
    let graph = Rc::new(RefCell::new(session.borrow_mut().begin_frame().unwrap()));

    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd.clone()));
    // usage：STORAGE(0x20) | TRANSFER_DST(0x2)；memory 2 = 主机可见，便于读回。
    let buffer = buffers
        .create_buffer(&CoreBufferDesc::new(16, 0x22, 2), "target", None)
        .unwrap();
    let managed = buffers.get_by_id(buffer.id).unwrap();
    session.borrow_mut()
        .backend
        .bind_import_named(
            "target",
            tiny_glade::render_host::graph_token(buffer.id),
            GpuResource::Buffer(managed.shared()),
        )
        .unwrap();

    let mut core =
        RenderGraph::with_backend(GraphHost::new(graph.clone(), buffers.clone(), textures));
    let imported = core.import_unowned(buffer.id, 0x34);
    let mut passes = GraphPasses::new(graph.clone(), gd)
        .unwrap()
        .with_shared_session(session.clone())
        .unwrap();
    let resource = imported.resource as u32;
    clear_rg_buffer(&mut passes, resource);
    rg_buffer_copy_from_slice_u32(
        &mut passes,
        resource,
        16,
        &[1, 2, 3, 4],
        country_core::render::rg_helpers::rg_boundary::UploadContentVersion::Unknown,
    )
    .unwrap();
    core.export(imported, 0x26); // The host read is an observable graph result.
    drop(core);
    drop(passes);

    let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
    session.borrow_mut().execute_frame(built).unwrap();
    session.borrow_mut().wait_submits_done(10_000_000_000).unwrap();

    let mut out = [0u8; 16];
    managed.read_bytes(0, &mut out).unwrap();
    let words: Vec<u32> = out
        .chunks(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    assert_eq!(words, [1, 2, 3, 4]);
}

/// `out[gid.x] = gid.x + 100`，`local_size = (4,1,1)`，set 0 binding 0 是 storage buffer。
fn compute_words(with_push: bool) -> Vec<u32> {
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
    b.decorate(
        gid,
        Decoration::BuiltIn,
        [rspirv::dr::Operand::BuiltIn(BuiltIn::GlobalInvocationId)],
    );
    let rt_array = b.type_runtime_array(uint);
    b.decorate(
        rt_array,
        Decoration::ArrayStride,
        [rspirv::dr::Operand::LiteralInt32(4)],
    );
    let block = b.type_struct([rt_array]);
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(
        block,
        0,
        Decoration::Offset,
        [rspirv::dr::Operand::LiteralInt32(0)],
    );
    let sb_ptr = b.type_pointer(None, StorageClass::StorageBuffer, block);
    let buf = b.variable(sb_ptr, None, StorageClass::StorageBuffer, None);
    b.decorate(
        buf,
        Decoration::DescriptorSet,
        [rspirv::dr::Operand::LiteralInt32(0)],
    );
    b.decorate(
        buf,
        Decoration::Binding,
        [rspirv::dr::Operand::LiteralInt32(0)],
    );
    let elem_ptr = b.type_pointer(None, StorageClass::StorageBuffer, uint);
    let zero = b.constant_u32(uint, 0);
    let hundred = b.constant_u32(uint, 100);
    let push = if with_push {
        let block = b.type_struct([uint]);
        b.decorate(block, Decoration::Block, []);
        b.member_decorate(
            block,
            0,
            Decoration::Offset,
            [rspirv::dr::Operand::LiteralInt32(0)],
        );
        let pointer = b.type_pointer(None, StorageClass::PushConstant, block);
        let variable = b.variable(pointer, None, StorageClass::PushConstant, None);
        let value_pointer = b.type_pointer(None, StorageClass::PushConstant, uint);
        Some((variable, value_pointer))
    } else {
        None
    };
    let main = b
        .begin_function(void, None, FunctionControl::NONE, fn_ty)
        .unwrap();
    b.begin_block(None).unwrap();
    let id = b.load(uvec3, None, gid, None, []).unwrap();
    let x = b.composite_extract(uint, None, id, [0]).unwrap();
    let ptr = b.access_chain(elem_ptr, None, buf, [zero, x]).unwrap();
    let base = if let Some((variable, pointer)) = push {
        let pointer = b.access_chain(pointer, None, variable, [zero]).unwrap();
        b.load(uint, None, pointer, None, []).unwrap()
    } else {
        hundred
    };
    let value = b.i_add(uint, None, x, base).unwrap();
    b.store(ptr, value, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::GLCompute, main, "cs_main", [gid]);
    b.execution_mode(main, ExecutionMode::LocalSize, [4, 1, 1]);
    b.module().assemble()
}

#[test]
fn core_compute_pass_dispatches_on_lavapipe_with_bound_storage_buffer() {
    run_core_compute_pass_with_heap_parameters(false, false);
}
#[test]
fn custom_compute_push_rebinds_parameter_storage_and_preserves_resource_leases() {
    run_core_compute_pass_with_heap_parameters(true, false);
}
#[test]
fn external_compute_buffers_are_declared_before_recording_and_survive_host_release() {
    run_core_compute_pass_with_heap_parameters(false, true);
    run_core_compute_pass_with_heap_parameters(true, true);
}
fn run_core_compute_pass_with_heap_parameters(with_push: bool, external: bool) {
    use country_core::render::pass_builder::compute_pass_builder::{
        AccessRange, DispatchMode, ExternalBinding, ExternalSlot, RecordedComputePass,
    };
    use country_core::render::pass_builder::pass_resource_access::{
        PassBinding, RecordingPassBuilder, ResourceAccess,
    };
    use country_core::render::pass_builder::shader_uri::ComputePipelineDesc;
    use country_core::render::shader::boundary::ShaderBackend;
    use country_core::render::ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc;
    use country_core::rhapsody_types::RenderGraph;
    use rhapsody::{
        rg::AccessTable, rg::GpuResource, rg::VulkanGraphBackend, rg::VulkanGraphSession,
    };
    use shader_pipeline::{
        compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
        pipeline_desc::ComputePipelineDesc as FileDesc, reflection::Binding,
        reflection::BindingArray, reflection::BindingKind, reflection::BindingLocation,
        reflection::BindingNameHash, reflection::ShaderReflection, shader_defines::ShaderDefines,
    };
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{
        BufferHost, GraphHost, GraphPasses, HostTables, ShaderHost, TextureHost,
    };
    let Some(device) = graphics_device() else {
        return;
    };
    let gd = device.clone();

    // 发行容器：SPIR-V + 存储的反射。
    let compiled = CompiledShaderPipeline {
        stages: vec![CompiledShaderStage {
            stage: ShaderStage::Compute,
            entry_point: "cs_main".into(),
            spirv: compute_words(with_push),
        }],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![vec![(
                0,
                Binding {
                    kind: BindingKind::StorageBuffer,
                    array: BindingArray::Fixed(1),
                    name: "out_data".into(),
                },
            )]],
            uniform_block: None,
            binding_names: vec![("out_data".into(), BindingLocation { set: 0, binding: 0 })],
            binding_name_hashes: vec![(
                BindingNameHash::new("out_data").0,
                BindingLocation { set: 0, binding: 0 },
            )],
            vertex_inputs: vec![],
            local_size: Some([4, 1, 1]),
            push_constant_size: if with_push { 4 } else { 0 },
        },
        trailing_bytes: vec![],
    };
    let root = std::env::temp_dir().join(format!(
        "tg-compute-pass-{with_push}-{external}-{}",
        std::process::id()
    ));
    let file = FileDesc {
        source_path: "test/iota".into(),
        entry_point: "cs_main".into(),
        defines: ShaderDefines::default(),
    };
    let path = root.join(file.compiled_file_path());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, compiled.encode().unwrap()).unwrap();

    let shaders = Arc::new(ShaderHost::new(gd.clone(), &root).unwrap());
    let desc = ComputePipelineDesc {
        path: "test/iota".into(),
        entry: "cs_main".into(),
        defines: vec![],
    };
    let pipeline = shaders
        .create_compute_pipeline(
            "iota",
            Arc::new(shaders.load_compiled_compute(&desc).unwrap()),
        )
        .unwrap();
    shaders.prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();

    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let session = Rc::new(RefCell::new(VulkanGraphSession::new(backend)));
    let graph = Rc::new(RefCell::new(session.borrow_mut().begin_frame().unwrap()));
    let buffers = Arc::new(BufferHost::new(gd.clone()));
    let textures = Arc::new(TextureHost::new(gd.clone()));
    let offset = if external {
        unsafe {
            gd.instance()
                .raw_instance()
                .get_physical_device_properties(gd.physical_device())
        }
        .limits
        .min_storage_buffer_offset_alignment
        .max(16)
    } else {
        0
    };
    let guard = vec![0x5b; (offset + 16) as usize];
    let buffer = buffers
        .create_buffer(
            &CoreBufferDesc::new(guard.len(), 0x22, 2),
            "out",
            Some(&guard),
        )
        .unwrap();
    let managed = buffers.get_by_id(buffer.id).unwrap();
    if !external {
        session.borrow_mut()
            .backend
            .bind_import_named(
                "out",
                tiny_glade::render_host::graph_token(buffer.id),
                GpuResource::Buffer(managed.shared()),
            )
            .unwrap();
    }

    let mut core = RenderGraph::with_backend(GraphHost::new(
        graph.clone(),
        buffers.clone(),
        textures.clone(),
    ));
    let imported = (!external).then(|| core.import_unowned(buffer.id, 0x37));
    let resource = imported.map_or(0, |resource| resource.resource as u32);
    let mut passes = GraphPasses::new(graph.clone(), gd)
        .unwrap()
        .with_shared_session(session.clone())
        .unwrap()
        .with_tables(HostTables {
            buffers: buffers.clone(),
            textures,
            shaders,
        });
    passes
        .add_compute(
            RecordedComputePass {
                name: "iota",
                builder: RecordingPassBuilder {
                    name: "iota",
                    accesses: if external {
                        vec![]
                    } else {
                        vec![ResourceAccess {
                            resource,
                            access: 0x37,
                            group: 0,
                        }]
                    },
                    ..Default::default()
                },
                bindings: if external {
                    vec![]
                } else {
                    vec![PassBinding::Buffer { resource }]
                },
                access_ranges: if external {
                    vec![]
                } else {
                    vec![AccessRange {
                        resource_index: 0,
                        access_index: 0,
                    }]
                },
                inactive_texture_bindings: Vec::new(),
                undefined_texture_bindings: Vec::new(),
                preserving_buffers: Vec::new(),
                preserving_texture_bindings: Vec::new(),
                device_address_reads: Vec::new(),
                external_bindings: if external {
                    vec![ExternalBinding {
                        binding: 0,
                        slot: ExternalSlot::Buffer {
                            vk_buffer: ash::vk::Handle::as_raw(managed.raw()),
                            offset,
                            range: 16,
                            handle: country_core::rhapsody_contract::ManagedHandle(buffer.id),
                            flag: false,
                        },
                    }]
                } else {
                    vec![]
                },
                uniform_binder: None,
                push_constants: if with_push {
                    100_u32.to_le_bytes().to_vec()
                } else {
                    Vec::new()
                },
                program: None,
                dispatch: if with_push {
                    DispatchMode::With(Box::new(|api| {
                        api.push_constants(&100_u32.to_le_bytes());
                        api.push_constants(&100_u32.to_le_bytes());
                        api.push_constants(&200_u32.to_le_bytes());
                        api.push_constants(&200_u32.to_le_bytes());
                        api.dispatch_1d(4);
                    }))
                } else {
                    DispatchMode::Threads { x: 4, y: 1, z: 1 }
                },
            },
            &pipeline,
        )
        .unwrap();
    if let Some(imported) = imported {
        core.export(imported, 0x26);
    } else {
        let borrowed = graph.borrow();
        assert_eq!(borrowed.declaration().resources.len(), 1);
        assert!(borrowed.declaration().passes[0].side_effect);
        assert_eq!(
            borrowed.declaration().passes[0].accesses[0].range,
            rhapsody::rg::AccessRange::Buffer { offset, size: 16 }
        );
        assert!(borrowed.declaration().exports.is_empty());
        assert!(buffers.release(&buffer));
    }
    drop(core);
    drop(passes);
    let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
    session.borrow_mut().execute_frame(built).unwrap();
    session.borrow_mut().wait_submits_done(10_000_000_000).unwrap();

    let mut out = [0u8; 16];
    managed.read_bytes(offset, &mut out).unwrap();
    if external {
        let mut untouched = vec![0; offset as usize];
        managed.read_bytes(0, &mut untouched).unwrap();
        assert!(untouched.iter().all(|byte| *byte == 0x5b));
        assert_eq!(session.borrow_mut().frame_import_count(), 0);
    }
    let words: Vec<u32> = out
        .chunks(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    assert_eq!(
        words,
        if with_push {
            [200, 201, 202, 203]
        } else {
            [100, 101, 102, 103]
        }
    );
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
        b.decorate(
            gl_position,
            Decoration::BuiltIn,
            [Operand::BuiltIn(BuiltIn::Position)],
        );
        let main = b
            .begin_function(void, None, FunctionControl::NONE, fn_ty)
            .unwrap();
        b.begin_block(None).unwrap();
        let v = b.load(vec4, None, position, None, []).unwrap();
        b.store(gl_position, v, None, []).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();
        b.entry_point(
            ExecutionModel::Vertex,
            main,
            "vs_main",
            [position, gl_position],
        );
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
        let main = b
            .begin_function(void, None, FunctionControl::NONE, fn_ty)
            .unwrap();
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
    use rhapsody::pipeline::RasterPipelineOpts;
    use rhapsody::{rg::AttachmentKey, rg::RenderPassKey};
    use shader_pipeline::{
        compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
        reflection::ShaderReflection, reflection::VertexInput, reflection_ty::ScalarType,
        reflection_ty::Type, reflection_ty::VectorType,
    };
    use std::sync::Arc;
    use tiny_glade::render_host::ShaderHost;
    let Some(device) = graphics_device() else {
        return;
    };
    let gd = device;
    let (vs, ps) = raster_words();
    let compiled = CompiledShaderPipeline {
        stages: vec![
            CompiledShaderStage {
                stage: ShaderStage::Vertex,
                entry_point: "vs_main".into(),
                spirv: vs,
            },
            CompiledShaderStage {
                stage: ShaderStage::Fragment,
                entry_point: "ps_main".into(),
                spirv: ps,
            },
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
                    ty: Type::Vector(VectorType {
                        scalar_ty: ScalarType::Float(4),
                        nscalar: 4,
                    }),
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
    let key = RasterPipelineKey {
        handle: pass,
        selector: opts.0,
    };
    let pipeline = host
        .create_raster_pipeline(Arc::new(CoreCompiled(compiled.encode().unwrap())), &key)
        .unwrap();
    host.prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();
    let entry = host.raster_entry(&pipeline).unwrap();
    assert_eq!(entry.pass.color_attachments, 1);
    assert_eq!(
        entry.pipeline.try_get().unwrap().point,
        ash::vk::PipelineBindPoint::GRAPHICS
    );
    // 渲染通道句柄无效 → 报错而不是 panic。
    let bad = RasterPipelineKey {
        handle: 99,
        selector: 0,
    };
    assert!(
        host.create_raster_pipeline(Arc::new(CoreCompiled(compiled.encode().unwrap())), &bad)
            .is_err()
    );
}

#[test]
fn raster_shader_variants_append_the_depth_only_define() {
    use country_core::render::shader::boundary::{RasterPipelineDesc, ShaderBackend};
    use tiny_glade::render_host::ShaderHost;
    let Some(device) = graphics_device() else {
        return;
    };
    let host = ShaderHost::new(device, std::env::temp_dir()).unwrap();
    let desc = RasterPipelineDesc {
        path: "shaders/x".into(),
        entry: "ps_main".into(),
        defines: vec![("A".into(), "1".into())],
    };
    let [v0, v1] = host.raster_shader_variants(&desc);
    assert_eq!(
        v0.defines,
        vec![
            ("A".into(), "1".into()),
            ("SHADER_VARIANT_DEPTH_ONLY".into(), "0".into())
        ]
    );
    assert_eq!(
        v1.defines.last().unwrap(),
        &("SHADER_VARIANT_DEPTH_ONLY".to_owned(), "1".to_owned())
    );
    assert_eq!(
        (v0.path.as_str(), v0.entry.as_str()),
        ("shaders/x", "ps_main")
    );
}

#[test]
fn device_local_sparse_updates_use_the_owned_queue_and_publish_real_completion_points() {
    use ash::vk;
    use rhapsody::transfer::{TransferConfig, TransferManager};
    let Some(device) = graphics_device() else {
        return;
    };
    let host = BufferHost::new(device.clone());
    let capacity = (2 << 20) + 128;
    let desc = BufferDesc::new(
        capacity,
        (vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST).as_raw(),
        1,
    );
    let initial = vec![0x55; capacity];
    let buffer = host
        .create_buffer(&desc, "queued sparse GPU buffer", Some(&initial))
        .unwrap();
    let payload = (0..(1 << 20) + 1024)
        .map(|i| ((i * 31 + 7) % 251) as u8)
        .collect::<Vec<_>>();
    let ranges = [0..512, 256..1024, 2048..payload.len()];
    let idle = device.retirement_stats().idle_waits;
    host.update_buffer_bytes(&buffer, &payload, &ranges, 32).unwrap();
    test_support::until_ready(|| host.poll_pending_uploads()).unwrap();
    let stats = host
        .queued_upload_stats()
        .expect("production GPU-only BufferHost used its queued uploader");
    assert_eq!(stats.queued_jobs, 0);
    assert_eq!(
        stats.submitted_jobs, 3,
        "each requested interval fits one owned chunk and retains its FIFO position"
    );
    assert_eq!(stats.merged_requests, 0,
        "owned deltas preserve individual completion points instead of replacing queued writes");
    let raw = host.get_by_id(buffer.id).unwrap().shared();
    let mut reads = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 1,
            chunk_bytes: capacity as u64,
            max_bytes: 8 << 20,
        },
    )
    .unwrap();
    let read = reads.read_buffer(&raw.clone(), 0, capacity).unwrap();
    read.wait(5_000_000_000).unwrap();
    let completed = device.poll().unwrap();
    let mut count = 0;
    let mut reached = true;
    rhapsody::tracking::GpuUse::visit_uses(raw.as_ref(), &mut |cell| {
        count += 1;
        reached &= cell.snapshot().is_reached(&completed);
        Ok(())
    })
    .unwrap();
    assert!(
        count != 0 && reached,
        "readback completion must include every preceding owned upload point"
    );

    let mut actual = vec![0; capacity];
    read.try_read(&mut actual).unwrap().unwrap();
    let mut expected = initial;
    for range in ranges {
        expected[32 + range.start..32 + range.end].copy_from_slice(&payload[range]);
    }
    assert_eq!(
        actual, expected,
        "holes and untouched prefix/tail must remain intact"
    );
    assert_eq!(device.retirement_stats().idle_waits, idle);
    let submitted = host.queued_upload_stats().unwrap().submitted_jobs;
    assert!(
        host.update_buffer_bytes(&buffer, &payload, &[0..payload.len() + 1], 0)
            .is_err()
    );
    assert_eq!(
        host.queued_upload_stats().unwrap().submitted_jobs,
        submitted,
        "invalid ranges must be rejected before submitting any batch"
    );
    assert!(host.release(&buffer));
}

#[test]
fn queued_device_local_updates_coalesce_in_call_order_and_submit_on_flush_without_waiting() {
    use ash::vk;
    use rhapsody::transfer::{TransferConfig, TransferManager};
    let Some(device) = graphics_device() else {
        return;
    };
    let host = BufferHost::new(device.clone());
    let capacity = (2 << 20) + 128;
    let desc = BufferDesc::new(
        capacity,
        (vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST).as_raw(),
        1,
    );
    let initial = vec![0x55; capacity];
    let buffer = host
        .create_buffer(&desc, "queued GPU buffer", Some(&initial))
        .unwrap();
    let first = vec![0xA1u8; 4096];
    let second = vec![0xB2u8; 1024];
    let big = (0..(1 << 20) + 513).map(|i| (i % 249) as u8).collect::<Vec<_>>();
    let idle = device.retirement_stats().idle_waits;
    // Overlapping writes in one frame: the later bytes win, the earlier tail survives.
    host.update_buffer_bytes(&buffer, &first, &[0..first.len()], 0).unwrap();
    host.update_buffer_bytes(&buffer, &second, &[0..second.len()], 512).unwrap();
    // A write larger than one staging chunk is split, not rejected.
    host.update_buffer_bytes(&buffer, &big, &[0..big.len()], 8192).unwrap();
    let before = host.queued_upload_stats().expect("queued uploader exists");
    assert!(before.queued_jobs > 0, "nothing is submitted before the flush");
    assert_eq!(before.submitted_jobs, 0);
    test_support::until_ready(|| host.poll_pending_uploads()).unwrap();
    let after = host.queued_upload_stats().unwrap();
    assert_eq!(after.queued_jobs, 0);
    assert!(after.submitted_jobs > 0);
    assert_eq!(device.retirement_stats().idle_waits, idle, "no device idle wait");
    // A second flush with nothing queued is a no-op.
    test_support::until_ready(|| host.poll_pending_uploads()).unwrap();
    assert_eq!(host.queued_upload_stats().unwrap().submitted_jobs, after.submitted_jobs);

    let raw = host.get_by_id(buffer.id).unwrap().shared();
    let mut reads = TransferManager::new(
        device.clone(),
        TransferConfig { slots: 1, chunk_bytes: capacity as u64, max_bytes: 8 << 20 },
    )
    .unwrap();
    let read = reads.read_buffer(&raw.clone(), 0, capacity).unwrap();
    read.wait(5_000_000_000).unwrap();
    let mut actual = vec![0; capacity];
    read.try_read(&mut actual).unwrap().unwrap();
    let mut expected = initial;
    expected[..first.len()].copy_from_slice(&first);
    expected[512..512 + second.len()].copy_from_slice(&second);
    expected[8192..8192 + big.len()].copy_from_slice(&big);
    assert_eq!(actual, expected, "queued writes land exactly as sequential writes would");
    assert!(host.release(&buffer));
}

#[test]
fn queue_pressure_flush_in_the_middle_of_a_frame_never_cancels_earlier_writes() {
    use ash::vk;
    use rhapsody::transfer::{TransferConfig, TransferManager};
    let Some(device) = graphics_device() else {
        return;
    };
    let host = BufferHost::new(device.clone());
    let desc = BufferDesc::new(
        256,
        (vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST).as_raw(),
        1,
    );
    // More device-local buffers than the queue admits jobs (64) but fewer than its
    // key limit (128): enqueue is rejected part-way through the frame, the retry
    // succeeds after a flush, and the epoch has moved on by then. 100 distinct
    // buffers, so
    // that retry path is exercised (more would hit the key limit and fall back to a waited write).
    let buffers: Vec<_> = (0..100)
        .map(|index| host.create_buffer(&desc, &format!("pressure {index}"), Some(&[0u8; 256])).unwrap())
        .collect();
    for (index, buffer) in buffers.iter().enumerate() {
        let first = [index as u8 | 1; 16];
        host.update_buffer_bytes(buffer, &first, &[0..16], 0).unwrap();
    }
    // Same frame, same buffers, a different non-overlapping range: it must add to,
    // not replace, the first write that may already have been flushed or queued.
    // Start with the 65th buffer: its first write is the one that was retried right
    // after the mid-frame flush, so it is still queued when this second write arrives.
    for index in (64..buffers.len()).chain(0..64) {
        let second = [index as u8 | 2; 16];
        host.update_buffer_bytes(&buffers[index], &second, &[0..16], 64).unwrap();
    }
    test_support::until_ready(|| host.poll_pending_uploads()).unwrap();
    let mut reads = TransferManager::new(
        device.clone(),
        TransferConfig { slots: 1, chunk_bytes: 256, max_bytes: 8 << 20 },
    )
    .unwrap();
    for (index, buffer) in buffers.iter().enumerate() {
        let raw = host.get_by_id(buffer.id).unwrap().shared();
        let read = reads.read_buffer(&raw, 0, 256).unwrap();
        read.wait(5_000_000_000).unwrap();
        let mut actual = [0u8; 256];
        read.try_read(&mut actual).unwrap().unwrap();
        assert_eq!(&actual[..16], &[index as u8 | 1; 16], "first write lost for buffer {index}");
        assert_eq!(&actual[64..80], &[index as u8 | 2; 16], "second write lost for buffer {index}");
    }
    for buffer in &buffers {
        host.release(buffer);
    }
}

#[test]
fn versioned_country_upload_uses_real_session_subgraph_and_result_caches() {
    use country_core::render::rg_helpers::{
        rg_boundary::UploadContentVersion, rg_buffer_copy_from_slice_u32,
    };
    use rhapsody::{resource::Buffer as RawBuffer, resource::BufferDesc as RawDesc, rg::AccessTable, rg::GpuResource, rg::ResourceSpec, rg::VulkanGraphBackend, rg::VulkanGraphSession};
    use rhapsody::collections::CacheReason;
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::GraphPasses;
    let Some(device) = graphics_device() else {
        return;
    };
    let run = |enabled| {
        let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
        let session = Rc::new(RefCell::new(VulkanGraphSession::new(backend)));
        session.borrow_mut().set_result_cache_enabled(enabled);
        session.borrow_mut().chain.set_subgraph_cache_enabled(enabled);
        let target = Arc::new(
            RawBuffer::new(
                device.clone(),
                &RawDesc::new_gpu_to_cpu(16, ash::vk::BufferUsageFlags::TRANSFER_DST),
            )
            .unwrap(),
        );
        let token = session.borrow_mut()
            .backend
            .bind_import(GpuResource::Buffer(target.clone()))
            .unwrap();
        let inputs = [
            (1, [1, 2, 3, 4]),
            (1, [1, 2, 3, 4]),
            (2, [5, 6, 7, 8]),
            (2, [5, 6, 7, 8]),
            (2, [9, 10, 11, 12]),
            (2, [9, 10, 11, 12]),
        ];
        let mut outputs = Vec::new();
        for (frame, (version, words)) in inputs.into_iter().enumerate() {
            let graph = Rc::new(RefCell::new(session.borrow_mut().begin_frame().unwrap()));
            let id = graph.borrow().graph_id();
            let output = graph
                .borrow_mut()
                .import_unowned(
                    "country-upload-target",
                    ResourceSpec::Buffer(target.desc).descriptor(),
                    token,
                    0x26,
                )
                .unwrap();
            {
                let mut passes = GraphPasses::new(graph.clone(), device.clone())
                    .unwrap()
                    .with_shared_session(session.clone())
                    .unwrap();
                rg_buffer_copy_from_slice_u32(
                    &mut passes,
                    output.raw(),
                    16,
                    &words,
                    UploadContentVersion::Complete {
                        source: 0x4350_5555,
                        version,
                    },
                )
                .unwrap();
            }
            assert_eq!(
                graph.borrow().graph_id(),
                id,
                "host cache must keep the frame's graph scope"
            );
            // An unrelated changing pass forces a whole-plan miss while the
            // stable upload scopes remain eligible for subgraph reuse.
            graph
                .borrow_mut()
                .add_pass(format!("unrelated-frame-{frame}"))
                .with_side_effect()
                .render(|_| true);
            graph.borrow_mut().export(output, 0x26).unwrap();
            session.borrow_mut()
                .execute_frame(Rc::try_unwrap(graph).ok().unwrap().into_inner().build())
                .unwrap();
            session.borrow_mut().wait_submits_done(10_000_000_000).unwrap();
            let mut bytes = [0; 16];
            target.read_bytes(0, &mut bytes).unwrap();
            let actual: Vec<_> = bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|word| u32::from_ne_bytes(*word))
                .collect();
            assert_eq!(actual, words);
            outputs.push(actual);
        }
        let cached = session.borrow_mut().result_cache_stats();
        let subgraphs = session.borrow_mut().chain.subgraph_cache_stats();
        // The old/default caller explicitly declares unknown content. Even a
        // byte-identical upload must stage again and publish no cache request.
        let graph = Rc::new(RefCell::new(session.borrow_mut().begin_frame().unwrap()));
        let output = graph
            .borrow_mut()
            .import_unowned(
                "country-unknown-target",
                ResourceSpec::Buffer(target.desc).descriptor(),
                token,
                0x26,
            )
            .unwrap();
        {
            let mut passes = GraphPasses::new(graph.clone(), device.clone())
                .unwrap()
                .with_shared_session(session.clone())
                .unwrap();
            rg_buffer_copy_from_slice_u32(
                &mut passes,
                output.raw(),
                16,
                &[13, 14, 15, 16],
                UploadContentVersion::Unknown,
            )
            .unwrap();
        }
        graph.borrow_mut().export(output, 0x26).unwrap();
        session.borrow_mut()
            .execute_frame(Rc::try_unwrap(graph).ok().unwrap().into_inner().build())
            .unwrap();
        session.borrow_mut().wait_submits_done(10_000_000_000).unwrap();
        let mut bytes = [0; 16];
        target.read_bytes(0, &mut bytes).unwrap();
        assert_eq!(
            bytes,
            [13_u32, 14, 15, 16]
                .map(u32::to_ne_bytes)
                .concat()
                .as_slice()
        );
        assert_eq!(session.borrow_mut().result_cache_stats().misses, cached.misses);
        assert_eq!(session.borrow_mut().result_cache_stats().hits, cached.hits);
        session.borrow_mut().clear_after_completion().unwrap();
        (outputs, cached, subgraphs)
    };
    let (baseline, off, _) = run(false);
    let (cached, on, subgraphs) = run(true);
    assert_eq!(cached, baseline);
    assert_eq!(off.hits, 0);
    assert_eq!(off.bypassed, 6);
    assert_eq!(on.hits, 3);
    assert_eq!(on.misses, 3);
    assert_eq!(on.invalidations, 1);
    assert_eq!(
        on.diagnostics.misses(CacheReason::Parameters),
        1,
        "same version with changed bytes must miss safely"
    );
    assert_eq!(
        on.diagnostics.invalidations(CacheReason::DependencyVersion),
        1
    );
    assert!(
        subgraphs.hits >= 3,
        "independent upload scopes must reuse their formal analyses"
    );
}

fn external_compute_image_words() -> Vec<u32> {
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
    let function = b.type_function(void, vec![]);
    let uint = b.type_int(32, 0);
    let uvec2 = b.type_vector(uint, 2);
    let uvec3 = b.type_vector(uint, 3);
    let float = b.type_float(32);
    let vec4 = b.type_vector(float, 4);
    let zero = b.constant_u32(uint, 0);
    let gid_ptr = b.type_pointer(None, StorageClass::Input, uvec3);
    let gid = b.variable(gid_ptr, None, StorageClass::Input, None);
    b.decorate(
        gid,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::GlobalInvocationId)],
    );
    let source_ty = b.type_image(float, Dim::Dim2D, 0, 0, 0, 1, ImageFormat::Unknown, None);
    let target_ty = b.type_image(float, Dim::Dim2D, 0, 0, 0, 2, ImageFormat::Rgba8, None);
    let source_ptr = b.type_pointer(None, StorageClass::UniformConstant, source_ty);
    let target_ptr = b.type_pointer(None, StorageClass::UniformConstant, target_ty);
    let source = b.variable(source_ptr, None, StorageClass::UniformConstant, None);
    let target = b.variable(target_ptr, None, StorageClass::UniformConstant, None);
    for (variable, binding) in [(source, 0), (target, 1)] {
        b.decorate(
            variable,
            Decoration::DescriptorSet,
            [Operand::LiteralInt32(0)],
        );
        b.decorate(
            variable,
            Decoration::Binding,
            [Operand::LiteralInt32(binding)],
        );
    }
    let main = b
        .begin_function(void, None, FunctionControl::NONE, function)
        .unwrap();
    b.begin_block(None).unwrap();
    let id = b.load(uvec3, None, gid, None, []).unwrap();
    let x = b.composite_extract(uint, None, id, [0]).unwrap();
    let y = b.composite_extract(uint, None, id, [1]).unwrap();
    let coord = b.composite_construct(uvec2, None, [x, y]).unwrap();
    let source = b.load(source_ty, None, source, None, []).unwrap();
    let target = b.load(target_ty, None, target, None, []).unwrap();
    let color = b
        .image_fetch(
            vec4,
            None,
            source,
            coord,
            Some(ImageOperands::LOD),
            [Operand::IdRef(zero)],
        )
        .unwrap();
    b.image_write(target, coord, color, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::GLCompute, main, "cs_main", [gid]);
    b.execution_mode(main, ExecutionMode::LocalSize, [4, 4, 1]);
    b.module().assemble()
}

#[test]
fn external_compute_sampled_and_storage_images_have_declared_views_and_real_pixels() {
    use ash::vk;
    use country_core::{
        render::{
            load_texture::boundary::{TextureDesc as CoreTextureDesc, TextureDevice},
            pass_builder::compute_pass_builder::{
                DispatchMode, ExternalBinding, ExternalSlot, RecordedComputePass,
            },
            shader::boundary::ShaderBackend,
        },
        rhapsody_contract::ManagedHandle,
    };
    use rhapsody::{command::buffer_access, command::image_access, resource::Buffer as RawBuffer, resource::BufferDesc as RawBufferDesc, rg::AccessRange, rg::AccessTable, rg::VulkanGraphBackend, rg::VulkanGraphSession};
    use rhapsody::tracking::OwnershipRange;
    use rhapsody::transfer::with_setup_command;
    use rhapsody::native::AccessScope;
    use shader_pipeline::{
        compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
        pipeline_desc::ComputePipelineDesc as FileDesc, reflection::Binding,
        reflection::BindingArray, reflection::BindingKind, reflection::BindingLocation,
        reflection::BindingNameHash, reflection::ShaderReflection, shader_defines::ShaderDefines,
    };
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{BufferHost, GraphPasses, HostTables, ShaderHost, TextureHost};
    let Some(device) = graphics_device() else {
        return;
    };
    let reflection_bindings = [
        (0, BindingKind::SampledImage, "external_source"),
        (1, BindingKind::StorageImage, "external_target"),
    ];
    let compiled = CompiledShaderPipeline {
        stages: vec![CompiledShaderStage {
            stage: ShaderStage::Compute,
            entry_point: "cs_main".into(),
            spirv: external_compute_image_words(),
        }],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![
                reflection_bindings
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
            binding_names: reflection_bindings
                .iter()
                .map(|&(binding, _, name)| (name.into(), BindingLocation { set: 0, binding }))
                .collect(),
            binding_name_hashes: reflection_bindings
                .iter()
                .map(|&(binding, _, name)| {
                    (
                        BindingNameHash::new(name).0,
                        BindingLocation { set: 0, binding },
                    )
                })
                .collect(),
            vertex_inputs: vec![],
            local_size: Some([4, 4, 1]),
            push_constant_size: 0,
        },
        trailing_bytes: vec![],
    };
    let root =
        std::env::temp_dir().join(format!("tg-compute-external-images-{}", std::process::id()));
    let file = FileDesc {
        source_path: "test/external-image".into(),
        entry_point: "cs_main".into(),
        defines: ShaderDefines::default(),
    };
    let path = root.join(file.compiled_file_path());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, compiled.encode().unwrap()).unwrap();
    let shaders = Arc::new(ShaderHost::new(device.clone(), &root).unwrap());
    let desc = country_core::render::pass_builder::shader_uri::ComputePipelineDesc {
        path: "test/external-image".into(),
        entry: "cs_main".into(),
        defines: vec![],
    };
    let pipeline = shaders
        .create_compute_pipeline(
            "external-image",
            Arc::new(shaders.load_compiled_compute(&desc).unwrap()),
        )
        .unwrap();
    shaders.prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();
    let textures =
        Arc::new(TextureHost::new(device.clone()).with_extra_usage(
            vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST,
        ));
    let buffers = Arc::new(BufferHost::new(device.clone()));
    let texture_desc = CoreTextureDesc {
        extent: [4, 4, 1],
        kind: 2,
        format: vk::Format::R8G8B8A8_UNORM.as_raw() as u32,
        sampled: true,
        mip_levels: 1,
        array_elements: 1,
        ..Default::default()
    };
    let expected = (0..16)
        .flat_map(|index| {
            if index % 2 == 0 {
                [0, 255, 0, 255]
            } else {
                [255, 0, 255, 255]
            }
        })
        .collect::<Vec<u8>>();
    let source = textures
        .create_texture("external sampled", &texture_desc, &expected)
        .unwrap();
    let target = textures
        .create_texture(
            "external storage",
            &CoreTextureDesc {
                storage: true,
                ..texture_desc
            },
            &[0; 64],
        )
        .unwrap();
    let target_owner = textures.get_by_id(target.id).unwrap().shared();
    let source_view = textures.default_view(source.id).unwrap();
    let target_view = textures.default_view(target.id).unwrap();
    let session = Rc::new(RefCell::new(VulkanGraphSession::new(
        VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap(),
    )));
    let graph = Rc::new(RefCell::new(session.borrow_mut().begin_frame().unwrap()));
    let mut passes = GraphPasses::new(graph.clone(), device.clone())
        .unwrap()
        .with_shared_session(session.clone())
        .unwrap()
        .with_tables(HostTables {
            buffers,
            textures: textures.clone(),
            shaders,
        });
    passes
        .add_compute(
            RecordedComputePass {
                name: "external sampled to storage",
                builder: Default::default(),
                bindings: vec![],
                access_ranges: vec![],
                inactive_texture_bindings: Vec::new(),
                undefined_texture_bindings: Vec::new(),
                preserving_buffers: Vec::new(),
                preserving_texture_bindings: Vec::new(),
                device_address_reads: Vec::new(),
                external_bindings: vec![
                    ExternalBinding {
                        binding: 0,
                        slot: ExternalSlot::Image {
                            sampler: 0,
                            image_view: source_view,
                            layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL.as_raw() as u32,
                            handle: ManagedHandle(source.id),
                            flag: false,
                        },
                    },
                    ExternalBinding {
                        binding: 1,
                        slot: ExternalSlot::Image {
                            sampler: 0,
                            image_view: target_view,
                            layout: vk::ImageLayout::GENERAL.as_raw() as u32,
                            handle: ManagedHandle(target.id),
                            flag: false,
                        },
                    },
                ],
                uniform_binder: None,
                push_constants: vec![],
                dispatch: DispatchMode::Threads { x: 4, y: 4, z: 1 },
                program: None,
            },
            &pipeline,
        )
        .unwrap();
    {
        let borrowed = graph.borrow();
        let pass = &borrowed.declaration().passes[0];
        assert!(pass.side_effect);
        assert_eq!(pass.accesses.len(), 2);
        assert_eq!(
            pass.accesses
                .iter()
                .map(|access| access.access_code)
                .collect::<Vec<_>>(),
            [0x22, 0x37]
        );
        assert!(pass.accesses.iter().all(|access| access.range
            == AccessRange::Texture {
                aspect_mask: vk::ImageAspectFlags::COLOR.as_raw(),
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1
            }));
        assert!(borrowed.declaration().exports.is_empty());
    }
    assert!(textures.release(&source));
    assert!(textures.release(&target));
    drop(passes);
    let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
    session.borrow_mut().execute_frame(built).unwrap();
    session.borrow_mut().wait_submits_done(10_000_000_000).unwrap();
    assert_eq!(session.borrow_mut().frame_import_count(), 0);
    let readback = Arc::new(
        RawBuffer::new(
            device.clone(),
            &RawBufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let queue = device
        .queue(rhapsody::device::QueueClass::Graphics)
        .unwrap();
    with_setup_command(&device, &[], |command| unsafe {
        image_access(
            command,
            &target_owner.clone(),
            queue,
            OwnershipRange::Image {
                aspects: vk::ImageAspectFlags::COLOR.as_raw(),
                base_mip: 0,
                mip_count: 1,
                base_layer: 0,
                layer_count: 1,
            },
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ,
                write: false,
            },
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        )?;
        buffer_access(
            command,
            &readback.clone(),
            queue,
            0,
            64,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_WRITE,
                write: true,
            },
        )?;
        device.raw_device().cmd_copy_image_to_buffer(
            command.raw_handle(),
            target_owner.raw_handle(),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback.raw_handle(),
            &[vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width: 4,
                    height: 4,
                    depth: 1,
                })],
        );
        buffer_access(
            command,
            &readback.clone(),
            queue,
            0,
            64,
            AccessScope {
                stages: vk::PipelineStageFlags::HOST,
                access: vk::AccessFlags::HOST_READ,
                write: false,
            },
        )
    })
    .and_then(|setup| setup.wait(rhapsody::transfer::SETUP_WAIT_TIMEOUT_NS))
    .unwrap();
    let mut actual = vec![0; 64];
    readback.read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, expected);
    std::fs::remove_dir_all(root).ok();
}

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
    use rhapsody::{rg::AccessRange, rg::AccessTable, rg::BufferDesc, rg::BuildError, rg::Descriptor, rg::GraphDefinition, rg::OriginSource, rg::PlanContext, rg::RenderGraph, rg::ResourceContractError, rg::ResourceOrigin, rg::ResourceSpec, rg::check_plan, rg::compile_definition, rg::validate_contracts};
    use rhapsody::tracking::OwnershipRange;
    use shader_pipeline::{
        compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
        reflection::Binding, reflection::BindingArray, reflection::BindingKind,
        reflection::BindingLocation, reflection::BindingNameHash, reflection::ShaderReflection,
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
                            rhapsody::resource::TextureDesc::new_2d(16, 16, vk::Format::R32_SFLOAT)
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
                        undefined_texture_bindings: Vec::new(),
                        preserving_buffers: vec![],
                        preserving_texture_bindings: Vec::new(),
                        device_address_reads: Vec::new(),
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
            let (declaration, _, _, declaration_error) = Rc::try_unwrap(graph)
                .ok()
                .unwrap()
                .into_inner()
                .build()
                .into_parts();
            assert!(declaration_error.is_none());
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
                    matches!(validate_contracts(&declaration, rhapsody::native::is_write),
                    Err(BuildError::Contract(ResourceContractError::UninitializedRead { resource, pass }))
                    if resource == history.raw() && pass == 0)
                );
                continue;
            }
            let definition = GraphDefinition::from_declaration(
                &declaration,
                &AccessTable::original(),
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
                (lifetime.first_pass, lifetime.last_pass),
                (Some(0), Some(1))
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

/// Actual GraphPasses metadata/owner lifecycle regression. Append to the
/// existing tests/render_host.rs to reuse graphics_device() and compute_words().
/// The diagnostic shader writes out_data; it deliberately does not dereference
/// BDA inputs. Passing this is NOT an original terrain shader result.
#[test]
fn device_address_chunks_keep_same_page_ranges_until_cancel_or_real_completion() {
    use country_core::render::{
        pass_builder::{
            compute_pass_builder::{AccessRange, DispatchMode, RecordedComputePass},
            pass_resource_access::{PassBinding, RecordingPassBuilder, ResourceAccess},
            shader_uri::ComputePipelineDesc,
        },
        shader::boundary::ShaderBackend,
    };
    use rhapsody::{resource::DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES as PAGE, resource::DynamicConstants, resource::DynamicConstantsConfig, rg::AccessRange as NativeRange, rg::AccessTable, rg::RenderGraph as NativeGraph, rg::VulkanGraphBackend, rg::VulkanGraphSession};
    use rhapsody::tracking::GpuUse;
    use shader_pipeline::{
        compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
        pipeline_desc::ComputePipelineDesc as FileDesc, reflection::Binding,
        reflection::BindingArray, reflection::BindingKind, reflection::BindingLocation,
        reflection::BindingNameHash, reflection::ShaderReflection, shader_defines::ShaderDefines,
    };
    use std::{cell::RefCell, rc::Rc, sync::Arc};
    use tiny_glade::render_host::{GraphPasses, HostTables, ShaderHost, TextureHost};
    let Some(device) = graphics_device() else {
        return;
    };
    assert!(
        device.capabilities().buffer_device_address,
        "this GPU regression requires the real BDA feature"
    );
    let root = tempfile::tempdir().unwrap();
    let file = FileDesc {
        source_path: "test/bda-metadata-owner".into(),
        entry_point: "cs_main".into(),
        defines: ShaderDefines::default(),
    };
    let location = BindingLocation { set: 0, binding: 0 };
    let compiled = CompiledShaderPipeline {
        stages: vec![CompiledShaderStage {
            stage: ShaderStage::Compute,
            entry_point: "cs_main".into(),
            spirv: compute_words(false),
        }],
        source_dependencies: vec![],
        reflection: ShaderReflection {
            descriptor_sets: vec![vec![(
                0,
                Binding {
                    kind: BindingKind::StorageBuffer,
                    array: BindingArray::Fixed(1),
                    name: "out_data".into(),
                },
            )]],
            uniform_block: None,
            binding_names: vec![("out_data".into(), location)],
            binding_name_hashes: vec![(BindingNameHash::new("out_data").0, location)],
            vertex_inputs: vec![],
            local_size: Some([4, 1, 1]),
            push_constant_size: 0,
        },
        trailing_bytes: vec![],
    };
    let path = root.path().join(file.compiled_file_path());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, compiled.encode().unwrap()).unwrap();
    let shaders = Arc::new(ShaderHost::new(device.clone(), root.path()).unwrap());
    let desc = ComputePipelineDesc {
        path: file.source_path,
        entry: file.entry_point,
        defines: vec![],
    };
    let pipeline = shaders
        .create_compute_pipeline(
            "BDA metadata diagnostic",
            Arc::new(shaders.load_compiled_compute(&desc).unwrap()),
        )
        .unwrap();
    shaders.prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();
    let buffers = Arc::new(BufferHost::new(device.clone()));
    let textures = Arc::new(TextureHost::new(device.clone()));
    let session = Rc::new(RefCell::new(VulkanGraphSession::new(
        VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap(),
    )));

    for submit in [false, true] {
        let idle_waits = device.retirement_stats().idle_waits;
        // Bound to one full page: the next allocation can succeed only after
        // the actual imported ranges have been released, not by adding a page.
        let mut constants = DynamicConstants::with_config(
            device.clone(),
            DynamicConstantsConfig {
                max_pages: 1,
                max_backing_bytes: PAGE,
                max_idle_pages: 1,
            },
        )
        .unwrap();
        let empty = constants.allocate(0).expect("original size0 allocation");
        assert_eq!(empty.size, 0);
        assert_ne!(empty.device_address().unwrap(), 0);
        assert_eq!(constants.page_count(), 1);
        let first = constants.allocate(16).unwrap();
        let second = constants.allocate(32).unwrap();
        assert!(Arc::ptr_eq(&empty.buffer, &first.buffer));
        assert!(Arc::ptr_eq(&first.buffer, &second.buffer));
        assert_eq!(
            empty.offset, first.offset,
            "size0 did not consume page bytes"
        );
        let first_range = NativeRange::Buffer {
            offset: first.offset,
            size: first.size,
        };
        let second_range = NativeRange::Buffer {
            offset: second.offset,
            size: second.size,
        };
        let first_bytes = [0x31; 16];
        let second_bytes = [0xa7; 32];
        first.write_bytes(&first_bytes).unwrap();
        second.write_bytes(&second_bytes).unwrap();
        let first_offset = first.offset;
        let second_offset = second.offset;
        let page = Arc::downgrade(&first.buffer);
        // Exhaust the exact remaining page without declaring a fake read.
        let aligned_after = constants.allocate(0).unwrap();
        let tail = constants.allocate(PAGE - aligned_after.offset).unwrap();
        drop(aligned_after);
        drop(tail);
        drop(empty); // zero read must never appear in metadata

        let target = buffers
            .create_buffer(
                &BufferDesc::new(16, 0x22, 2),
                "BDA metadata diagnostic output",
                Some(&[0; 16]),
            )
            .unwrap();
        let managed = buffers.managed_buffer(&target).unwrap();
        let graph = Rc::new(RefCell::new(session.borrow_mut().begin_frame().unwrap()));
        let output = session.borrow_mut()
            .import_frame_buffer(&mut graph.borrow_mut(), managed.shared(), 0x37)
            .unwrap();
        let mut passes = GraphPasses::new(graph.clone(), device.clone())
            .unwrap()
            .with_shared_session(session.clone())
            .unwrap()
            .with_tables(HostTables {
                buffers: buffers.clone(),
                textures: textures.clone(),
                shaders: shaders.clone(),
            });
        passes
            .add_compute(
                RecordedComputePass {
                    name: "BDA metadata diagnostic",
                    builder: RecordingPassBuilder {
                        name: "BDA metadata diagnostic",
                        accesses: vec![ResourceAccess {
                            resource: output.raw(),
                            access: 0x37,
                            group: 0,
                        }],
                        ..Default::default()
                    },
                    bindings: vec![PassBinding::Buffer {
                        resource: output.raw(),
                    }],
                    access_ranges: vec![AccessRange {
                        resource_index: 0,
                        access_index: 0,
                    }],
                    inactive_texture_bindings: vec![],
                    undefined_texture_bindings: Vec::new(),
                    preserving_buffers: vec![],
                    preserving_texture_bindings: vec![],
                    external_bindings: vec![],
                    device_address_reads: vec![first, second],
                    uniform_binder: None,
                    push_constants: vec![],
                    program: None,
                    dispatch: DispatchMode::Threads { x: 4, y: 1, z: 1 },
                },
                &pipeline,
            )
            .unwrap();
        // CPU chunk/RecordedComputePass owners are gone here. Graph imports own
        // the two exact ranges of ONE physical page; zero had no readable range.
        drop(passes);
        let bda = graph.borrow().declaration().passes[0]
            .accesses
            .iter()
            .filter(|a| a.access_code == 0x1a)
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(bda.len(), 2);
        assert_eq!(bda[0].resource_index, bda[1].resource_index);
        assert_eq!([bda[0].range, bda[1].range], [first_range, second_range]);
        assert!(bda.iter().all(|a| a.synchronization_group == 1));
        graph.borrow_mut().export(output, 0x26).unwrap();
        let built = Rc::try_unwrap(graph).ok().unwrap().into_inner().build();
        assert_eq!(constants.stats().referenced_pages, 1);
        // Drop the Weak before admission: Weak itself intentionally blocks page
        // reset in the existing allocator, so it must not create a false pass.
        let page_owner = page.upgrade().unwrap();
        let page_ptr = Arc::as_ptr(&page_owner) as usize;
        drop(page_owner);
        drop(page);
        // No external Weak remains after this point.
        assert!(
            matches!(constants.allocate(16), Err(rhapsody::Error::Timeout)),
            "an unsubmitted graph must retain its constants page"
        );
        if submit {
            let receipt = session.borrow_mut().execute_frame(built).unwrap();
            assert!(
                !receipt.receipts.is_empty(),
                "actual Vulkan submission required"
            );
            session.borrow_mut().wait_submits_done(10_000_000_000).unwrap();
            let mut bytes = [0; 16];
            managed.read_bytes(0, &mut bytes).unwrap();
            assert_eq!(
                bytes,
                [100_u32, 101, 102, 103].map(u32::to_ne_bytes).concat()[..]
            );
            // Re-obtain the page through the allocator only after the session
            // has released owners. The still-unwritten tail means this forces
            // reset rather than append; bytes are observed before overwriting.
        } else {
            session.borrow_mut().cancel_frame(built).unwrap();
        }
        assert_eq!(session.borrow_mut().frame_import_count(), 0);
        let reused = constants
            .allocate(16)
            .unwrap_or_else(|error| {
                panic!("cancel/completion released imported page (submit={submit}): {error:?}")
            });
        assert_eq!(Arc::as_ptr(&reused.buffer) as usize, page_ptr);
        assert_eq!(reused.offset, 0);
        assert_eq!(constants.stats().page_reuses, 1);
        assert_eq!(device.retirement_stats().idle_waits, idle_waits);
        // Reset does not clear contents: verify both exact CPU uploads after
        // cancellation/completion and before writing the reused range.
        let mut actual_first = [0; 16];
        let mut actual_second = [0; 32];
        reused
            .buffer
            .read_bytes(first_offset, &mut actual_first)
            .unwrap();
        reused
            .buffer
            .read_bytes(second_offset, &mut actual_second)
            .unwrap();
        assert_eq!(actual_first, first_bytes);
        assert_eq!(actual_second, second_bytes);
        if submit {
            let mut uses = 0;
            reused
                .buffer
                .visit_uses(&mut |cell| {
                    uses += usize::from(!cell.snapshot().is_empty());
                    assert!(cell.snapshot().is_reached(&device.poll().unwrap()));
                    Ok(())
                })
                .unwrap();
            assert_ne!(
                uses, 0,
                "metadata participated in the real command owner protocol"
            );
        }
        drop(reused);
        assert_eq!(constants.trim_idle(0).unwrap(), 1);
        assert_eq!(constants.page_count(), 0);
        assert!(buffers.release(&target));
    }
}
