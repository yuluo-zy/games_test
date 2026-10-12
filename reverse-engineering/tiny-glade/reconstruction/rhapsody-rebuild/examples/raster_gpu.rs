//! 真实光栅功能验收：顶点输入映射→反射布局→附件生产→管线缓存→draw→像素读回。
//! 测试 shader 由原锁 rspirv 构建，验收接口能力，不声称原游戏材质或画面相等。
use ash::vk;
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        init::create_headless_device,
        shader::{
            RasterPipelineDesc, ReflectedEntry, ReflectedShader, ShaderLibrary, ShaderModule,
            VulkanPipeline, get_vk_blend_mode, map_vertex_attribute_format,
        },
        texture::{Texture, TextureDesc},
    },
    rg::{
        attachments::{Attachment, AttachmentCache},
        pass_api::{AshCommandBackend, RenderPassApi},
    },
};
use rspirv::{
    binary::Assemble,
    dr::{Builder, Operand},
    spirv,
};
use std::{collections::BTreeMap, ffi::CString, sync::Arc};

fn shader(fragment: bool) -> Vec<u8> {
    let mut builder = Builder::new();
    builder.set_version(1, 0);
    builder.capability(spirv::Capability::Shader);
    builder.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);
    let void = builder.type_void();
    let float = builder.type_float(32);
    let vec2 = builder.type_vector(float, 2);
    let vec4 = builder.type_vector(float, 4);
    let output_pointer = builder.type_pointer(None, spirv::StorageClass::Output, vec4);
    let output = builder.variable(output_pointer, None, spirv::StorageClass::Output, None);
    if fragment {
        builder.decorate(
            output,
            spirv::Decoration::Location,
            [Operand::LiteralInt32(0)],
        );
    } else {
        builder.decorate(
            output,
            spirv::Decoration::BuiltIn,
            [Operand::BuiltIn(spirv::BuiltIn::Position)],
        );
    }
    let input = if fragment {
        None
    } else {
        let pointer = builder.type_pointer(None, spirv::StorageClass::Input, vec2);
        let input = builder.variable(pointer, None, spirv::StorageClass::Input, None);
        builder.decorate(
            input,
            spirv::Decoration::Location,
            [Operand::LiteralInt32(0)],
        );
        Some(input)
    };
    let zero = builder.constant_f32(float, 0.0);
    let one = builder.constant_f32(float, 1.0);
    let color = if fragment {
        let r = builder.constant_f32(float, 0.25);
        let g = builder.constant_f32(float, 0.5);
        let b = builder.constant_f32(float, 0.75);
        Some(builder.constant_composite(vec4, [r, g, b, one]))
    } else {
        None
    };
    let function_type = builder.type_function(void, []);
    let function = builder
        .begin_function(void, None, spirv::FunctionControl::NONE, function_type)
        .unwrap();
    builder.begin_block(None).unwrap();
    let result = match input {
        Some(input) => {
            let xy = builder.load(vec2, None, input, None, []).unwrap();
            let x = builder.composite_extract(float, None, xy, [0]).unwrap();
            let y = builder.composite_extract(float, None, xy, [1]).unwrap();
            builder
                .composite_construct(vec4, None, [x, y, zero, one])
                .unwrap()
        }
        None => color.unwrap(),
    };
    builder.store(output, result, None, []).unwrap();
    builder.ret().unwrap();
    builder.end_function().unwrap();
    let mut interface = vec![output];
    if let Some(input) = input {
        interface.push(input);
    }
    builder.entry_point(
        if fragment {
            spirv::ExecutionModel::Fragment
        } else {
            spirv::ExecutionModel::Vertex
        },
        function,
        "main",
        interface,
    );
    if fragment {
        builder.execution_mode(function, spirv::ExecutionMode::OriginUpperLeft, []);
    }
    builder
        .module()
        .assemble()
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .collect()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let desc = TextureDesc::new_2d(8, 8, vk::Format::R8G8B8A8_UNORM)
        .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC);
    let image = Arc::new(Texture::new(device.clone(), &desc)?);
    let mut attachment = Attachment::color(image.clone(), 1, Some([0.0, 0.0, 0.0, 0.0]));
    attachment.initial_layout = vk::ImageLayout::UNDEFINED;
    let mut attachments = AttachmentCache::new(device.clone());
    let prepared = attachments.prepare(&[attachment], desc.extent_2d(), 1)?;
    let entries: Vec<ReflectedEntry> = serde_json::from_value(serde_json::json!([
        {"entry_point":"main","stage":"Vertex","descriptors":[]},
        {"entry_point":"main","stage":"Fragment","descriptors":[]}
    ]))?;
    let vertex = ShaderModule::from_spirv(device.clone(), &shader(false))?;
    let fragment = ShaderModule::from_spirv(device.clone(), &shader(true))?;
    let mut library = ShaderLibrary::new(device.clone())?;
    library.register(
        "triangle_vertex".into(),
        ReflectedShader {
            module: vertex.clone(),
            entry: entries[0].clone(),
        },
    )?;
    library.register(
        "triangle_fragment".into(),
        ReflectedShader {
            module: fragment.clone(),
            entry: entries[1].clone(),
        },
    )?;
    let program =
        library.prepare_raster_program("triangle_vertex", "triangle_fragment", &BTreeMap::new())?;
    let layout = program.layout.clone();
    let cache = &library.cache;
    let pipeline_desc = |cull_mode| {
        program
            .pipeline_desc(RasterPipelineDesc {
                vertex: vertex.clone(),
                fragment: Some(fragment.clone()),
                entry: CString::new("main").unwrap(),
                fragment_entry: None,
                layout: layout.clone(),
                render_pass: prepared.framebuffer.render_pass.handle,
                render_pass_owner: prepared.framebuffer.render_pass.clone(),
                subpass: 0,
                vertex_bindings: vec![vk::VertexInputBindingDescription {
                    binding: 0,
                    stride: 8,
                    input_rate: vk::VertexInputRate::VERTEX,
                }],
                vertex_attributes: vec![vk::VertexInputAttributeDescription {
                    location: 0,
                    binding: 0,
                    format: map_vertex_attribute_format(4, 8).unwrap(),
                    offset: 0,
                }],
                topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode: vk::PolygonMode::FILL,
                cull_mode,
                front_face: vk::FrontFace::COUNTER_CLOCKWISE,
                samples: vk::SampleCountFlags::TYPE_1,
                depth_test: false,
                depth_write: false,
                depth_compare: vk::CompareOp::ALWAYS,
                blend_attachments: vec![get_vk_blend_mode(0)],
            })
            .unwrap()
    };
    let pipeline =
        VulkanPipeline::create_raster_pipeline(cache, pipeline_desc(vk::CullModeFlags::NONE))?;
    let reused =
        VulkanPipeline::create_raster_pipeline(cache, pipeline_desc(vk::CullModeFlags::NONE))?;
    assert!(Arc::ptr_eq(&pipeline, &reused));
    let changed =
        VulkanPipeline::create_raster_pipeline(cache, pipeline_desc(vk::CullModeFlags::BACK))?;
    assert!(!Arc::ptr_eq(&pipeline, &changed));
    let vertices = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(24, vk::BufferUsageFlags::VERTEX_BUFFER),
    )?);
    let data: Vec<_> = [-1.0f32, -1.0, 3.0, -1.0, -1.0, 3.0]
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    vertices.write_bytes(0, &data)?;
    let readback = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, vk::BufferUsageFlags::TRANSFER_DST),
    )?);
    let pool = VulkanCommandPool::new(device.clone())?;
    let mut command = pool.allocate()?;
    command.begin()?;
    prepared.keep_alive(&mut command);
    command.keep_alive(pipeline.clone());
    command.keep_alive(vertices.clone());
    command.keep_alive(readback.clone());
    {
        let mut backend = unsafe { AshCommandBackend::new(&device.device) };
        let mut api = RenderPassApi::new(&mut backend, command.handle, 0, false);
        api.begin_attachments(&prepared);
        api.set_default_view_and_scissor(desc.extent_2d());
        unsafe {
            pipeline.bind_pipeline(command.handle);
            device
                .device
                .cmd_bind_vertex_buffers(command.handle, 0, &[vertices.handle], &[0]);
            device.device.cmd_draw(command.handle, 3, 1, 0, 0);
        }
        api.end_render_pass();
    }
    let image_barriers = [vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
        .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image.handle)
        .subresource_range(
            vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1),
        )];
    unsafe {
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &image_barriers,
        );
        device.device.cmd_copy_image_to_buffer(
            command.handle,
            image.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback.handle,
            &[vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(desc.extent)],
        );
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(readback.handle)
                .size(256)],
            &[],
        );
    }
    command.end()?;
    cache.invalidate();
    attachments.clear();
    drop(prepared);
    drop(pipeline);
    drop(reused);
    drop(changed);
    drop(image);
    let pending = command.submit()?;
    pending.wait(u64::MAX)?;
    let mut pixels = [0; 256];
    readback.read_bytes(0, &mut pixels)?;
    assert!(
        pixels
            .chunks_exact(4)
            .all(|pixel| (63..=64).contains(&pixel[0])
                && (127..=128).contains(&pixel[1])
                && (191..=192).contains(&pixel[2])
                && pixel[3] == 255)
    );
    println!(
        "{}",
        serde_json::json!({"status":"candidate_raster_gpu_passed","draw_calls":1,"vertices":3,"pixels_verified":64,"pixel":&pixels[..4],"raster_cache_reused":true,"raster_state_change_separates_key":true,"command_owns_after_cache_invalidation":true,"original_game_shader_comparison":false})
    );
    Ok(())
}
