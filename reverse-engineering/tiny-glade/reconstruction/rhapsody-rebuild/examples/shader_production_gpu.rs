//! 功能验收：反射→布局→状态分配/复用→管线选择→描述符读取/写入→实际 GPU 读回。
//! 一条为独立测试 shader，另一条直接执行原发行 fill_buffer shader；均不手填布局。
use ash::vk;
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        descriptor_set::DescriptorSetState,
        init::create_headless_device,
        shader::{ReflectedEntry, ReflectedShader, ShaderLibrary, ShaderModule},
    },
    rg::{
        binding::{
            PassResourceSlot, ResolvedBinding, bind_descriptor_set, create_descriptor_writes,
        },
        graph::{AccessDeclaration, PassDeclaration},
        resource::ResourceHandle,
    },
};
use rspirv::{
    binary::Assemble,
    dr::{Builder, Operand},
    spirv,
};
use std::{collections::BTreeMap, sync::Arc};

fn arithmetic_shader() -> Vec<u8> {
    let mut builder = Builder::new();
    builder.set_version(1, 3);
    builder.capability(spirv::Capability::Shader);
    builder.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);
    let void = builder.type_void();
    let uint = builder.type_int(32, 0);
    let block = builder.type_struct([uint]);
    builder.decorate(block, spirv::Decoration::Block, []);
    builder.member_decorate(
        block,
        0,
        spirv::Decoration::Offset,
        [Operand::LiteralInt32(0)],
    );
    let pointer = builder.type_pointer(None, spirv::StorageClass::StorageBuffer, block);
    let member_pointer = builder.type_pointer(None, spirv::StorageClass::StorageBuffer, uint);
    let input = builder.variable(pointer, None, spirv::StorageClass::StorageBuffer, None);
    let output = builder.variable(pointer, None, spirv::StorageClass::StorageBuffer, None);
    for (id, binding) in [(input, 3), (output, 9)] {
        builder.decorate(
            id,
            spirv::Decoration::DescriptorSet,
            [Operand::LiteralInt32(2)],
        );
        builder.decorate(
            id,
            spirv::Decoration::Binding,
            [Operand::LiteralInt32(binding)],
        );
    }
    let zero = builder.constant_u32(uint, 0);
    let increment = builder.constant_u32(uint, 17);
    let function_type = builder.type_function(void, []);
    let function = builder
        .begin_function(void, None, spirv::FunctionControl::NONE, function_type)
        .unwrap();
    builder.begin_block(None).unwrap();
    let address = builder
        .access_chain(member_pointer, None, input, [zero])
        .unwrap();
    let value = builder.load(uint, None, address, None, []).unwrap();
    let result = builder.i_add(uint, None, value, increment).unwrap();
    let address = builder
        .access_chain(member_pointer, None, output, [zero])
        .unwrap();
    builder.store(address, result, None, []).unwrap();
    builder.ret().unwrap();
    builder.end_function().unwrap();
    builder.entry_point(spirv::ExecutionModel::GLCompute, function, "main", []);
    builder.execution_mode(function, spirv::ExecutionMode::LocalSize, [1, 1, 1]);
    builder
        .module()
        .assemble()
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--mapping") {
        let blends = (0u8..=255)
            .map(|flags| {
                let state = rhapsody::raw::shader::get_vk_blend_mode(flags);
                [
                    state.blend_enable,
                    state.src_color_blend_factor.as_raw() as u32,
                    state.dst_color_blend_factor.as_raw() as u32,
                    state.color_blend_op.as_raw() as u32,
                    state.src_alpha_blend_factor.as_raw() as u32,
                    state.dst_alpha_blend_factor.as_raw() as u32,
                    state.alpha_blend_op.as_raw() as u32,
                    state.color_write_mask.as_raw(),
                ]
            })
            .collect::<Vec<_>>();
        let vertices = (0..=5).flat_map(|tag| [0, 4, 6, 8, 12, 16, 20].map(move |bytes| {
            serde_json::json!({"tag":tag,"bytes":bytes,"format":rhapsody::raw::shader::map_vertex_attribute_format(tag, bytes).ok().map(|format|format.as_raw())})
        })).collect::<Vec<_>>();
        println!("{}", serde_json::json!({"blend":blends,"vertex":vertices}));
        return Ok(());
    }
    let device = create_headless_device()?;
    let input = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(4, vk::BufferUsageFlags::STORAGE_BUFFER),
    )?);
    input.write_bytes(0, &25u32.to_le_bytes())?;
    let output = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, vk::BufferUsageFlags::STORAGE_BUFFER),
    )?);
    let entry: ReflectedEntry = serde_json::from_value(
        serde_json::json!({"entry_point":"main","stage":"GLCompute","descriptors":[{"set":2,"binding":3,"name":"input","count":1,"descriptor_type":"StorageBuffer(ReadWrite)"},{"set":2,"binding":9,"name":"output","count":1,"descriptor_type":"StorageBuffer(ReadWrite)"}]}),
    )?;
    let mut library = ShaderLibrary::new(device.clone())?;
    library.register(
        "arithmetic".into(),
        ReflectedShader {
            module: ShaderModule::from_spirv(device.clone(), &arithmetic_shader())?,
            entry: entry.clone(),
        },
    )?;
    let pipeline = library.create_compute_pipeline("arithmetic", &BTreeMap::new())?;
    let again = library.create_compute_pipeline("arithmetic", &BTreeMap::new())?;
    assert!(Arc::ptr_eq(&pipeline, &again));
    assert_eq!(pipeline.layout.sets.len(), 3);
    let state = DescriptorSetState::new(pipeline.layout.sets[2].clone(), 2)?;
    let pass = PassDeclaration {
        name: "arithmetic".into(),
        accesses: vec![
            AccessDeclaration {
                resource_index: 0,
                synchronization_group: 0,
                access_code: 4,
            },
            AccessDeclaration {
                resource_index: 1,
                synchronization_group: 0,
                access_code: 40,
            },
        ],
    };
    let slots = BTreeMap::from([
        (
            "input".into(),
            vec![PassResourceSlot::StorageBuffer {
                resource: ResourceHandle::from_raw(0),
                offset: 0,
                size: 4,
            }],
        ),
        (
            "output".into(),
            vec![PassResourceSlot::StorageBuffer {
                resource: ResourceHandle::from_raw(1),
                offset: 0,
                size: 4,
            }],
        ),
    ]);
    let writes = create_descriptor_writes(&entry, 2, &pass, &slots, |handle| {
        let buffer = if handle.raw() == 0 {
            input.clone()
        } else {
            output.clone()
        };
        Some(ResolvedBinding::Buffer {
            buffer: buffer.handle,
            owner: buffer,
        })
    })
    .map_err(|error| format!("绑定失败：{error:?}"))?;
    let set = unsafe { state.allocate_descriptor_set(&writes) }?;
    let reused = unsafe { state.allocate_descriptor_set(&writes) }?;
    assert!(!Arc::ptr_eq(&set, &reused));
    let pool = unsafe {
        device.device.create_command_pool(
            &vk::CommandPoolCreateInfo::default().queue_family_index(device.queue_family),
            None,
        )
    }?;
    let cb = unsafe {
        device.device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1),
        )
    }?[0];
    unsafe {
        device
            .device
            .begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
        pipeline.bind_pipeline(cb);
        bind_descriptor_set(cb, &pipeline, 2, &set, &[])?;
        device.device.cmd_dispatch(cb, 1, 1, 1);
        let barriers = [vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)
            .buffer(output.handle)
            .offset(0)
            .size(4)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)];
        device.device.cmd_pipeline_barrier(
            cb,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &barriers,
            &[],
        );
        device.device.end_command_buffer(cb)?;
        {
            let _guard = device.queue_lock.lock().unwrap();
            device.device.queue_submit(
                device.queue,
                &[vk::SubmitInfo::default().command_buffers(&[cb])],
                vk::Fence::null(),
            )?;
            device.device.queue_wait_idle(device.queue)?;
        }
        device.device.destroy_command_pool(pool, None);
    }
    let mut readback = [0; 4];
    output.read_bytes(0, &mut readback)?;
    let result = u32::from_le_bytes(readback);
    assert_eq!(result, 42);
    state.invalidate();
    assert_eq!(state.cached_set_count(), 0);
    // 失效不破坏仍被调用者持有的旧集合；新集合不能误命中旧缓存。
    let replacement = unsafe { state.allocate_descriptor_set(&writes) }?;
    assert!(!Arc::ptr_eq(&replacement, &set));

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let catalog: serde_json::Value = serde_json::from_slice(&std::fs::read(root.join(
        "evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/catalog.json",
    ))?)?;
    let file = catalog["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|file| file["file"] == "_fill_buffer.cs.hlsl_cs_main_0.bin")
        .unwrap();
    let bytes = std::fs::read(
        "D:/game/ljxsj_92385/Tiny Glade/compiled-shaders/_fill_buffer.cs.hlsl_cs_main_0.bin",
    )?;
    let original = ReflectedShader::from_compiled_container(
        device.clone(),
        &bytes,
        &file["modules"][0],
        "cs_main",
    )?;
    let original_entry = original.entry.clone();
    library.register("original_fill_buffer".into(), original)?;
    let original_pipeline =
        library.create_compute_pipeline("original_fill_buffer", &BTreeMap::new())?;
    let original_state = DescriptorSetState::new(original_pipeline.layout.sets[0].clone(), 2)?;
    let slots = BTreeMap::from([(
        "__alias_output".into(),
        vec![PassResourceSlot::StorageBuffer {
            resource: ResourceHandle::from_raw(1),
            offset: 0,
            size: 256,
        }],
    )]);
    let writes = create_descriptor_writes(&original_entry, 0, &pass, &slots, |_| {
        Some(ResolvedBinding::Buffer {
            buffer: output.handle,
            owner: output.clone(),
        })
    })
    .map_err(|error| format!("原 shader 绑定失败：{error:?}"))?;
    let original_set = unsafe { original_state.allocate_descriptor_set(&writes) }?;
    let pool = unsafe {
        device.device.create_command_pool(
            &vk::CommandPoolCreateInfo::default().queue_family_index(device.queue_family),
            None,
        )
    }?;
    let cb = unsafe {
        device.device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1),
        )
    }?[0];
    unsafe {
        device
            .device
            .begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
        original_pipeline.bind_pipeline(cb);
        bind_descriptor_set(cb, &original_pipeline, 0, &original_set, &[])?;
        let constants: Vec<_> = [0xabcdu32, 64]
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect();
        device.device.cmd_push_constants(
            cb,
            original_pipeline.layout.handle,
            vk::ShaderStageFlags::COMPUTE,
            0,
            &constants,
        );
        device.device.cmd_dispatch(cb, 1, 1, 1);
        let barriers = [vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)
            .buffer(output.handle)
            .offset(0)
            .size(256)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)];
        device.device.cmd_pipeline_barrier(
            cb,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &barriers,
            &[],
        );
        device.device.end_command_buffer(cb)?;
        {
            let _guard = device.queue_lock.lock().unwrap();
            device.device.queue_submit(
                device.queue,
                &[vk::SubmitInfo::default().command_buffers(&[cb])],
                vk::Fence::null(),
            )?;
            device.device.queue_wait_idle(device.queue)?;
        }
        device.device.destroy_command_pool(pool, None);
    }
    let mut filled = [0; 256];
    output.read_bytes(0, &mut filled)?;
    assert!(
        filled
            .chunks_exact(4)
            .all(|word| u32::from_le_bytes(word.try_into().unwrap()) == 0xabcd)
    );
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name =
        unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    println!(
        "{}",
        serde_json::json!({"status":"candidate_shader_production_gpu_passed","device":name,"arithmetic_input":25,"arithmetic_result":result,"sparse_set":2,"sparse_bindings":[3,9],"descriptor_cache_reused":false,"descriptor_pool_reused":true,"descriptor_invalidation_validated":true,"pipeline_cache_reused":true,"original_shader":"_fill_buffer.cs.hlsl_cs_main_0.bin","original_sparse_binding":13,"original_fill_words":64,"original_fill_value":43981,"original_full_rhapsody_comparison":false})
    );
    Ok(())
}
