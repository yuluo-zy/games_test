//! 独立消费真正新编译/新反射打包资产；不读取原.bin或catalog metadata。
//! CLI: <fresh fill.bin> <fresh raster variant0.bin> <fresh raster variant1.bin>。
use ash::vk;
use rhapsody::{
    raw::{buffer::{Buffer, BufferDesc}, descriptor_set::DescriptorSetState,
        init::create_headless_device, sampler::Sampler,
        shader::{ShaderLibrary, stored_pipeline_entry}},
    rg::{binding::{PassResourceSlot, ResolvedBinding, bind_descriptor_set, create_descriptor_writes},
        graph::{AccessDeclaration, PassDeclaration}, resource::ResourceHandle},
    shader_pipeline::{compiled::{CompiledShaderPipeline, ShaderStage}, reflection::BindingKind},
};
use std::{collections::BTreeMap, sync::Arc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = std::env::args().skip(1).collect::<Vec<_>>();
    if paths.len() != 3 { return Err("需要新fill及两个raster variant容器路径".into()); }
    let bytes = std::fs::read(&paths[0])?;
    let compiled = CompiledShaderPipeline::decode(&bytes)?;
    assert_eq!(compiled.stages.len(), 1);
    assert_eq!(compiled.stages[0].stage, ShaderStage::Compute);
    assert_eq!(compiled.reflection.push_constant_size, 8);
    let entry = stored_pipeline_entry(&compiled, 0)?;
    assert_eq!(entry.descriptors.len(), 1);
    assert_eq!(entry.descriptors[0].descriptor_type, "StorageBuffer(ReadWrite)");
    let descriptor = &entry.descriptors[0];
    let local = compiled.reflection.local_size.ok_or("缺少compute工作组尺寸")?;
    assert!(local[0] != 0 && local[1] == 1 && local[2] == 1);
    let device = create_headless_device()?;
    let output = Arc::new(Buffer::new(device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, vk::BufferUsageFlags::STORAGE_BUFFER))?);
    let mut library = ShaderLibrary::new(device.clone())?;
    library.register_compiled_compute("fresh_fill".into(), &bytes)?;
    let pipeline = library.create_compute_pipeline("fresh_fill", &BTreeMap::new())?;
    assert_eq!(pipeline.layout.push_constants[0].stage_flags, vk::ShaderStageFlags::COMPUTE);
    let state = DescriptorSetState::new(pipeline.layout.sets[descriptor.set as usize].clone(), 2)?;
    let pass = PassDeclaration {name:"fresh_fill".into(), accesses:vec![AccessDeclaration {
        resource_index:1, synchronization_group:0, access_code:4,
    }]};
    let slots = BTreeMap::from([(descriptor.name.clone(), vec![PassResourceSlot::StorageBuffer {
        resource:ResourceHandle::from_raw(1), offset:0, size:256,
    }])]);
    let writes = create_descriptor_writes(&entry, descriptor.set, &pass, &slots, |_| Some(ResolvedBinding::Buffer {
        buffer:output.handle, owner:output.clone(),
    })).map_err(|err| format!("新编译资产绑定失败: {err:?}"))?;
    let set = unsafe {state.allocate_descriptor_set(&writes)}?;
    let pool = unsafe {device.device.create_command_pool(&vk::CommandPoolCreateInfo::default().queue_family_index(device.queue_family), None)}?;
    let command = unsafe {device.device.allocate_command_buffers(&vk::CommandBufferAllocateInfo::default()
        .command_pool(pool).level(vk::CommandBufferLevel::PRIMARY).command_buffer_count(1))}?[0];
    let value = 0xc001_d00d_u32;
    unsafe {
        device.device.begin_command_buffer(command, &vk::CommandBufferBeginInfo::default())?;
        pipeline.bind_pipeline(command);
        bind_descriptor_set(command, &pipeline, descriptor.set, &set, &[])?;
        let constants = [value,64].iter().flat_map(|word|word.to_le_bytes()).collect::<Vec<_>>();
        device.device.cmd_push_constants(command, pipeline.layout.handle, vk::ShaderStageFlags::COMPUTE, 0, &constants);
        device.device.cmd_dispatch(command,64u32.div_ceil(local[0]),1,1);
        let barriers = [vk::BufferMemoryBarrier::default().src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ).buffer(output.handle).offset(0).size(256)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED).dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)];
        device.device.cmd_pipeline_barrier(command,vk::PipelineStageFlags::COMPUTE_SHADER,vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),&[],&barriers,&[]);
        device.device.end_command_buffer(command)?;
        let _guard = device.queue_lock.lock();
        device.note_submit();
        device.device.queue_submit(device.queue,&[vk::SubmitInfo::default().command_buffers(&[command])],vk::Fence::null())?;
        device.device.queue_wait_idle(device.queue)?;
        device.device.destroy_command_pool(pool,None);
    }
    let mut readback = [0u8;256];
    output.read_bytes(0,&mut readback)?;
    assert!(readback.chunks_exact(4).all(|word|u32::from_le_bytes(word.try_into().unwrap())==value));
    let mut raster_facts = Vec::new();
    for (variant,path) in paths[1..].iter().enumerate() {
        let raster_bytes = std::fs::read(path)?;
        let raster = CompiledShaderPipeline::decode(&raster_bytes)?;
        let mut samplers = BTreeMap::new();
        for bindings in raster.reflection.effective_descriptor_sets() {
            for binding in bindings.into_values().filter(|binding|binding.kind==BindingKind::Sampler) {
                // 这里只验证immutable sampler真实拥有/布局，未冒称游戏builtin sampler策略。
                samplers.entry(binding.name.clone()).or_insert(Arc::new(Sampler::new(device.clone(),
                    &vk::SamplerCreateInfo::default().mag_filter(vk::Filter::NEAREST).min_filter(vk::Filter::NEAREST))?));
            }
        }
        let vs = format!("fresh_vs_{variant}");
        let ps = format!("fresh_ps_{variant}");
        library.register_compiled_raster(vs.clone(),ps.clone(),&raster_bytes)?;
        let program = library.prepare_raster_program(&vs,&ps,&samplers)?;
        assert!(program.layout.sets.iter().flat_map(|set|&set.plan.bindings)
            .all(|binding|binding.stage_flags==vk::ShaderStageFlags::ALL_GRAPHICS.as_raw()));
        assert!(program.layout.push_constants.iter().all(|range|range.stage_flags==vk::ShaderStageFlags::ALL_GRAPHICS));
        assert!(library.register_compiled_raster(vs.clone(),ps.clone(),&bytes).is_err());
        let retained = library.prepare_raster_program(&vs,&ps,&samplers)?;
        assert!(Arc::ptr_eq(&program.layout,&retained.layout));
        raster_facts.push(serde_json::json!({"file":path,"variant":variant,"set_count":program.layout.sets.len(),
            "bindings":program.layout.sets.iter().map(|set|set.plan.bindings.len()).sum::<usize>(),
            "push_constant_size":raster.reflection.push_constant_size,"actual_layout_created":true,"failed_registration_preserves_layout":true}));
    }
    let properties = unsafe {device.instance.instance.get_physical_device_properties(device.physical)};
    let name = unsafe {std::ffi::CStr::from_ptr(properties.device_name.as_ptr())}.to_string_lossy();
    println!("{}",serde_json::json!({"status":"fresh_compilation_runtime_consumer_verified","device":name,
        "compute_file":paths[0],"resource_name":descriptor.name,"set":descriptor.set,"binding":descriptor.binding,
        "fill_words":64,"fill_value":value,"stored_metadata_consumed":true,"original_metadata_injected":false,
        "raster_variants":raster_facts,"raster_draw":false,"original_exe_executed":false}));
    Ok(())
}
