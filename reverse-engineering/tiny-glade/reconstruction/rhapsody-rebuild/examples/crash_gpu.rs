//! 正常设备上的诊断链：GPU marker、正常 shader 写入诊断记录、读回与解析。
//! 记录是验证夹具，不使 shader 挂起、设备丢失或产生真实 GPU 崩溃。
use ash::vk;
use rhapsody::raw::{
    binding::DescriptorSetBinding,
    command_buffer::VulkanCommandPool,
    crash::{GpuCrashDiagnostics, has_any_gpu_assertion_failed, parse_gpu_debug_info},
    descriptor_set::{LayoutBinding, LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSetLayout},
    init::create_headless_device,
    shader::{ComputePipelineDesc, PipelineCache, PipelineLayout, ShaderModule, VulkanPipeline},
};
use rspirv::{
    binary::Assemble,
    dr::{Builder, Operand},
    spirv,
};
use std::{collections::BTreeMap, ffi::CStr, io::Read, sync::Arc};
fn shader() -> Vec<u8> {
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(spirv::Capability::Shader);
    b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);
    let void = b.type_void();
    let uint = b.type_int(32, 0);
    let block = b.type_struct([uint; 8]);
    b.decorate(block, spirv::Decoration::Block, []);
    for member in 0..8 {
        b.member_decorate(
            block,
            member,
            spirv::Decoration::Offset,
            [Operand::LiteralInt32(member * 4)],
        );
    }
    let pointer = b.type_pointer(None, spirv::StorageClass::StorageBuffer, block);
    let member_pointer = b.type_pointer(None, spirv::StorageClass::StorageBuffer, uint);
    let output = b.variable(pointer, None, spirv::StorageClass::StorageBuffer, None);
    b.decorate(
        output,
        spirv::Decoration::DescriptorSet,
        [Operand::LiteralInt32(0)],
    );
    b.decorate(
        output,
        spirv::Decoration::Binding,
        [Operand::LiteralInt32(0)],
    );
    let indices: Vec<_> = (0..8).map(|word| b.constant_u32(uint, word)).collect();
    let values: Vec<_> = [0, 1, 0, 0, (3 << 24) | 77, 0x1234, 7, 8]
        .into_iter()
        .map(|word| b.constant_u32(uint, word))
        .collect();
    let function_type = b.type_function(void, []);
    let function = b
        .begin_function(void, None, spirv::FunctionControl::NONE, function_type)
        .unwrap();
    b.begin_block(None).unwrap();
    for (index, value) in indices.into_iter().zip(values) {
        let address = b
            .access_chain(member_pointer, None, output, [index])
            .unwrap();
        b.store(address, value, None, []).unwrap();
    }
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(spirv::ExecutionModel::GLCompute, function, "main", []);
    b.execution_mode(function, spirv::ExecutionMode::LocalSize, [1, 1, 1]);
    b.module()
        .assemble()
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .collect()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--parse") {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        let cases: Vec<Vec<u32>> = serde_json::from_str(&input)?;
        let output:Vec<_>=cases.iter().map(|words|serde_json::json!({"has_assertion":has_any_gpu_assertion_failed(Some(words)).ok(),"log":parse_gpu_debug_info(words,&BTreeMap::new()).ok()})).collect();
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    let diagnostics = GpuCrashDiagnostics::new(device.clone(), 4)?;
    let names = BTreeMap::from([(0x1234, "验证shader.hlsl".into())]);
    let before = diagnostics.log_gpu_crash_info(&names)?;
    assert!(before.initial_sentinel_observed);
    assert!(!before.assertions_failed);
    let plan = LayoutPlan {
        flags: 0,
        binding_flags: vec![0],
        bindings: vec![LayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER.as_raw(),
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE.as_raw(),
            immutable_sampler: None,
        }],
    };
    let set_layout = VulkanDescriptorSetLayout::new(device.clone(), &plan)?;
    let descriptor_pool = VulkanDescriptorPool::new(device.clone(), &plan, 1)?;
    let mut set = descriptor_pool.allocate(set_layout.clone())?;
    let binding = DescriptorSetBinding::Buffer {
        ty: vk::DescriptorType::STORAGE_BUFFER,
        info: vk::DescriptorBufferInfo::default()
            .buffer(diagnostics.debug_buffer.handle)
            .range(32),
        owner: diagnostics.debug_buffer.clone(),
    };
    unsafe { set.update(0, 0, &binding) }?;
    let set = Arc::new(set);
    let layout = PipelineLayout::new(device.clone(), vec![set_layout], vec![])?;
    let shader = ShaderModule::from_spirv(device.clone(), &shader())?;
    let cache = PipelineCache::new(device.clone(), &[])?;
    let pipeline = VulkanPipeline::create_compute_pipeline(
        &cache,
        ComputePipelineDesc::new(shader, layout.clone()),
    )?;
    let pool = VulkanCommandPool::new(device.clone())?;
    let mut command = pool.allocate()?;
    assert!(
        diagnostics
            .record_crash_marker(&mut command, "未begin")
            .is_err()
    );
    command.begin()?;
    let first = diagnostics.record_crash_marker(&mut command, "调试shader之前")?;
    assert_eq!(first, 0);
    command.keep_alive(pipeline.clone());
    command.keep_alive(set.clone());
    diagnostics.keep_debug_buffer(&mut command);
    unsafe {
        let marker = [vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
            .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .buffer(diagnostics.marker_buffer.handle)
            .size(4)];
        pipeline.bind_pipeline(command.handle);
        device.device.cmd_bind_descriptor_sets(
            command.handle,
            vk::PipelineBindPoint::COMPUTE,
            layout.handle,
            0,
            &[set.handle],
            &[],
        );
        device.device.cmd_dispatch(command.handle, 1, 1, 1);
        // consumer 决定 marker 所代表的进度边界；原 marker 叶入口本身只写 4 字节。
        // 这里同时保护连续 marker 的 WAW，并让计算执行先于后置 marker。
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &marker,
            &[],
        );
    }
    let second = diagnostics.record_crash_marker(&mut command, "调试shader之后")?;
    assert_eq!(second, 1);
    diagnostics.make_host_readable(&mut command)?;
    command.end()?;
    let pending = command.submit()?;
    pending.wait(u64::MAX)?;
    let after = diagnostics.log_gpu_crash_info(&names)?;
    assert_eq!(after.observed_marker, 1);
    assert!(after.assertions_failed);
    assert_eq!(after.debug.records.len(), 1);
    assert_eq!(after.debug.records[0].message, "assertion failed (7 > 8)");
    assert_eq!(after.debug.records[0].source_line, 77);
    assert_eq!(after.debug.records[0].source_id, 0x1234);
    let mut not_submitted = pool.allocate()?;
    not_submitted.begin()?;
    let recorded = diagnostics.record_crash_marker(&mut not_submitted, "只录制，未提交")?;
    not_submitted.end()?;
    let unexecuted = diagnostics.log_gpu_crash_info(&names)?;
    assert_eq!(recorded, 2);
    assert_eq!(unexecuted.observed_marker, 1);
    assert!(diagnostics.require_vendor_crash_dump().is_err());
    println!(
        "{}",
        serde_json::json!({"status":"passed","device":name,"before":before,"after":after,"host_recorded_unsubmitted":recorded,"gpu_after_unsubmitted":unexecuted.observed_marker,"synthetic_debug_payload":true,"vendor_dump_supported":false,"device_lost_triggered":false,"original_full_equivalence":false})
    );
    Ok(())
}
