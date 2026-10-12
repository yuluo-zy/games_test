//! 主图首次回调直接消费真实 CommandState/registry；无需第二counter或第二阶段提交。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::BufferDesc,
        descriptor_set::DescriptorSetState,
        init::create_headless_device,
        managed::{buffer::ManagedBuffer, texture::ManagedTexture},
        shader::{ReflectedEntry, ReflectedShader, ShaderLibrary, ShaderModule},
        texture::{TextureDesc, TextureViewDesc},
    },
    rg::{
        binding::bind_descriptor_set,
        executor::PassContext,
        graph::{RegistryResource, RenderGraph},
        resource::ResourceHandle,
        vulkan::{AccessSpec, AccessTable, ResourceSpec, VulkanGraphBackend},
    },
};
use rspirv::{
    binary::Assemble,
    dr::{Builder, Operand},
    spirv,
};
use std::{cell::RefCell, collections::BTreeMap, ffi::CStr, rc::Rc, sync::Arc};
fn buffer(context: &PassContext<'_>, resource: ResourceHandle) -> ManagedBuffer {
    match context.managed.unwrap().registry.resource(resource.raw()).0 {
        RegistryResource::OwnedBuffer(value) | RegistryResource::UnownedBuffer(value) => {
            value.handle.clone()
        }
        _ => panic!("图资源类型不符"),
    }
}
fn image(context: &PassContext<'_>, resource: ResourceHandle) -> ManagedTexture {
    match context.managed.unwrap().registry.resource(resource.raw()).0 {
        RegistryResource::OwnedTexture(value) | RegistryResource::UnownedTexture(value) => {
            value.handle.clone()
        }
        _ => panic!("图资源类型不符"),
    }
}
fn shader() -> Vec<u8> {
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(spirv::Capability::Shader);
    b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);
    let void = b.type_void();
    let uint = b.type_int(32, 0);
    let int = b.type_int(32, 1);
    let vec2 = b.type_vector(int, 2);
    let vec4 = b.type_vector(uint, 4);
    let block = b.type_struct([uint]);
    b.decorate(block, spirv::Decoration::Block, []);
    b.member_decorate(
        block,
        0,
        spirv::Decoration::Offset,
        [Operand::LiteralInt32(0)],
    );
    let up = b.type_pointer(None, spirv::StorageClass::Uniform, block);
    let ump = b.type_pointer(None, spirv::StorageClass::Uniform, uint);
    let sp = b.type_pointer(None, spirv::StorageClass::StorageBuffer, block);
    let smp = b.type_pointer(None, spirv::StorageClass::StorageBuffer, uint);
    let image = b.type_image(
        uint,
        spirv::Dim::Dim2D,
        0,
        0,
        0,
        1,
        spirv::ImageFormat::Unknown,
        None,
    );
    let ip = b.type_pointer(None, spirv::StorageClass::UniformConstant, image);
    let ubo = b.variable(up, None, spirv::StorageClass::Uniform, None);
    let tex = b.variable(ip, None, spirv::StorageClass::UniformConstant, None);
    let output = b.variable(sp, None, spirv::StorageClass::StorageBuffer, None);
    for (id, binding) in [(ubo, 0), (tex, 1), (output, 2)] {
        b.decorate(
            id,
            spirv::Decoration::DescriptorSet,
            [Operand::LiteralInt32(0)],
        );
        b.decorate(
            id,
            spirv::Decoration::Binding,
            [Operand::LiteralInt32(binding)],
        );
    }
    let zero = b.constant_u32(uint, 0);
    let izero = b.constant_u32(int, 0);
    let coord = b.constant_composite(vec2, [izero, izero]);
    let ft = b.type_function(void, []);
    let f = b
        .begin_function(void, None, spirv::FunctionControl::NONE, ft)
        .unwrap();
    b.begin_block(None).unwrap();
    let ua = b.access_chain(ump, None, ubo, [zero]).unwrap();
    let uv = b.load(uint, None, ua, None, []).unwrap();
    let im = b.load(image, None, tex, None, []).unwrap();
    let pixel = b
        .image_fetch(
            vec4,
            None,
            im,
            coord,
            Some(spirv::ImageOperands::LOD),
            [Operand::IdRef(izero)],
        )
        .unwrap();
    let value = b.composite_extract(uint, None, pixel, [0]).unwrap();
    let sum = b.i_add(uint, None, uv, value).unwrap();
    let dest = b.access_chain(smp, None, output, [zero]).unwrap();
    b.store(dest, sum, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(spirv::ExecutionModel::GLCompute, f, "main", []);
    b.execution_mode(f, spirv::ExecutionMode::LocalSize, [1, 1, 1]);
    b.module()
        .assemble()
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .collect()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--roles") {
        let handle = rhapsody::rg::resource::ResourceHandle::from_raw(71);
        let views: Vec<_> = (0..=1)
            .map(|role| {
                let mut output = [0xaa; 68];
                rhapsody::rg::resource::original_ref_view_binding(
                    &[19; 60],
                    handle,
                    role,
                    &mut output,
                );
                output.to_vec()
            })
            .collect();
        println!(
            "{}",
            serde_json::json!({"views":views,"uniform":rhapsody::rg::resource::original_ref_buffer_binding(handle,true),"other_buffer":rhapsody::rg::resource::original_ref_buffer_binding(handle,false)})
        );
        return Ok(());
    }
    let device = create_headless_device()?;
    let props = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) }.to_string_lossy();
    let mut accesses = AccessTable::default();
    for (code, write, stages, access, layout) in [
        (
            1,
            true,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        ),
        (
            4,
            false,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::AccessFlags::SHADER_READ | vk::AccessFlags::UNIFORM_READ,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        ),
        (
            40,
            true,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::AccessFlags::SHADER_WRITE,
            vk::ImageLayout::GENERAL,
        ),
    ] {
        accesses.0.insert(
            code,
            AccessSpec {
                original_mask: 0x40,
                write,
                stages,
                access,
                layout,
            },
        );
    }
    let mut backend = VulkanGraphBackend::new(device.clone(), accesses)?;
    let mut graph = RenderGraph::new();
    let ubo = backend.declare(
        &mut graph,
        "ubo",
        ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
            16,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
        )),
    )?;
    let tex = backend.declare(
        &mut graph,
        "tex",
        ResourceSpec::Image(
            TextureDesc::new_2d(1, 1, vk::Format::R32_UINT)
                .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST),
        ),
    )?;
    let out = backend.declare(
        &mut graph,
        "out",
        ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
            16,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )),
    )?;
    let mut upload = graph.add_pass("初始资源");
    upload
        .add_resource_access(ubo, 1, 0)?
        .add_resource_access(tex, 1, 0)?;
    let upload_device = device.clone();
    upload.render_with_context(move |context| {
        assert_eq!(context.command_state().tag, 1);
        buffer(&context, ubo)
            .write_bytes(0, &25u32.to_le_bytes())
            .unwrap();
        let tex = image(&context, tex);
        unsafe {
            upload_device.device.cmd_clear_color_image(
                vk::CommandBuffer::from_raw(context.command.handle()),
                tex.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue {
                    uint32: [17, 0, 0, 0],
                },
                &[vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)],
            );
        }
        context.keep_alive(Arc::new(tex));
        true
    });
    let entry: ReflectedEntry = serde_json::from_value(
        serde_json::json!({"entry_point":"main","stage":"GLCompute","descriptors":[
  {"set":0,"binding":0,"name":"ubo","count":1,"descriptor_type":"UniformBuffer"},
  {"set":0,"binding":1,"name":"tex","count":1,"descriptor_type":"SampledImage"},
  {"set":0,"binding":2,"name":"out","count":1,"descriptor_type":"StorageBuffer(ReadWrite)"}]}),
    )?;
    let mut library = ShaderLibrary::new(device.clone())?;
    library.register(
        "Ref消费验证".into(),
        ReflectedShader {
            module: ShaderModule::from_spirv(device.clone(), &shader())?,
            entry: entry.clone(),
        },
    )?;
    let pipeline = library.create_compute_pipeline("Ref消费验证", &BTreeMap::new())?;
    let state = Arc::new(DescriptorSetState::new(pipeline.layout.sets[0].clone(), 4)?);
    let output = Rc::new(RefCell::new(None));
    let captured = output.clone();
    let observed = Rc::new(RefCell::new(None));
    let cb_observed = observed.clone();
    let mut pass = graph.add_pass("主图绑定并dispatch");
    let u = pass.buffer_ref(ubo, 4, 0)?.bind_uniform_buffer(0, 4)?;
    let t = pass.texture_ref(tex, 4, 0)?.into_sampled_rg_texture(
        TextureViewDesc::new(&TextureDesc::new_2d(1, 1, vk::Format::R32_UINT)),
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )?;
    let o = pass.buffer_ref(out, 40, 0)?.bind_storage_buffer(0, 4)?;
    pass.render_bound_with_context(
        BTreeMap::from([
            ("ubo".into(), vec![u]),
            ("tex".into(), vec![t]),
            ("out".into(), vec![o]),
        ]),
        move |context, plan| {
            let writes = context
                .managed
                .unwrap()
                .descriptor_writes(plan, &entry, 0, 0, context.command_state())
                .unwrap();
            let set = unsafe { state.allocate_descriptor_set(&writes) }.unwrap();
            let other = unsafe { state.allocate_descriptor_set(&writes) }.unwrap();
            assert!(!Arc::ptr_eq(&set, &other));
            let raw = vk::CommandBuffer::from_raw(context.command.handle());
            unsafe {
                pipeline.bind_pipeline(raw);
                bind_descriptor_set(raw, &pipeline, 0, &set, &[]).unwrap();
                pipeline.device.device.cmd_dispatch(raw, 1, 1, 1);
            }
            *captured.borrow_mut() = Some(buffer(&context, out));
            *cb_observed.borrow_mut() =
                Some((context.command.sequence().0, context.command.handle()));
            context.keep_alive(set);
            context.keep_alive(other);
            context.keep_alive(pipeline);
            true
        },
    );
    let mut run = backend.execute(graph.build())?;
    let (sequence, handle) = observed.borrow().unwrap();
    assert!(run.backend.submissions.iter().any(|s| {
        s.batches
            .iter()
            .any(|b| b.commands.contains(&vk::CommandBuffer::from_raw(handle)))
    }));
    run.wait(u64::MAX)?;
    let mut bytes = [0; 4];
    output
        .borrow()
        .as_ref()
        .unwrap()
        .read_bytes(0, &mut bytes)?;
    let value = u32::from_le_bytes(bytes);
    assert_eq!(value, 42);
    println!(
        "{}",
        serde_json::json!({"status":"passed","device":name,"storage_output":value,"uniform_input":25,"sampled_image_input":17,
 "managed_command_sequence":sequence,"two_phase_callback_bridge":false,"actual_execution_command_state":true,
 "main_execution_registry_source":true,"descriptor_pool_reused":true,"descriptor_content_cache_reused":false,"original_full_ref_abi":false})
    );
    Ok(())
}
