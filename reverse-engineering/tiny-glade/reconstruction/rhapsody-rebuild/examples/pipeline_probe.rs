//! 实际 Vulkan 子系统冒烟：常量页→descriptor→计算管线→GPU 提交→timestamp。
//! 使用最小空计算着色器验证接线，不等同原游戏 shader/画面对照。
use ash::vk;
use rhapsody::raw::{
    descriptor_set::{LayoutBinding, LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSetLayout},
    dynamic_constants::DynamicConstants,
    init::create_headless_device,
    profiler::GpuProfiler,
    shader::{ComputePipelineDesc, PipelineCache, PipelineLayout, ShaderModule, VulkanPipeline},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let queues = unsafe {
        device
            .instance
            .instance
            .get_physical_device_queue_family_properties(device.physical)
    };
    let alignment = properties
        .limits
        .min_uniform_buffer_offset_alignment
        .max(properties.limits.min_storage_buffer_offset_alignment)
        .max(1);
    let mut constants = DynamicConstants::new(device.clone(), 4096, alignment)?;
    let chunk = constants.allocate_with_data(&[1, 2, 3, 4])?;
    let (_, storage) = chunk.into_uniform_and_storage_buffer_binding();
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
    let pool = VulkanDescriptorPool::new(device.clone(), &plan, 2)?;
    let mut set = pool.allocate(set_layout.clone())?;
    unsafe {
        set.update(0, 0, &storage)?;
    }
    let layout = PipelineLayout::new(device.clone(), vec![set_layout], vec![])?;
    // SPIR-V 1.0 空 compute main，local size 为 1；仅探测真实管线创建和分派。
    let words: &[u32] = &[
        0x07230203, 0x00010000, 0, 6, 0, 0x00020011, 1, 0x0003000e, 0, 1, 0x0005000f, 5, 4,
        0x6e69616d, 0, 0x00060010, 4, 17, 1, 1, 1, 0x00020013, 1, 0x00030021, 2, 1, 0x00050036, 1,
        4, 0, 2, 0x000200f8, 5, 0x000100fd, 0x00010038,
    ];
    let bytes: Vec<_> = words.iter().flat_map(|v| v.to_le_bytes()).collect();
    let shader = ShaderModule::from_spirv(device.clone(), &bytes)?;
    let cache = PipelineCache::new(device.clone(), &[])?;
    let pipeline = VulkanPipeline::create_compute_pipeline(
        &cache,
        ComputePipelineDesc::new(shader, layout.clone()),
    )?;
    let profiler = GpuProfiler::new(
        device.clone(),
        1,
        properties.limits.timestamp_period as f64,
        queues[device.queue_family as usize].timestamp_valid_bits,
    )?;
    let info = vk::CommandPoolCreateInfo::default().queue_family_index(device.queue_family);
    let command_pool = unsafe { device.device.create_command_pool(&info, None) }?;
    let info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .command_buffer_count(1)
        .level(vk::CommandBufferLevel::PRIMARY);
    let cb = unsafe { device.device.allocate_command_buffers(&info) }?[0];
    unsafe {
        device
            .device
            .begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
        profiler.reset(cb);
        let span = profiler.begin(cb, "empty_compute")?;
        pipeline.bind_pipeline(cb);
        rhapsody::raw::binding::bind_persistent_descriptor_set(
            &device,
            cb,
            vk::PipelineBindPoint::COMPUTE,
            layout.handle,
            0,
            &[set.handle],
            &[],
        );
        device.device.cmd_dispatch(cb, 1, 1, 1);
        profiler.end(cb, span);
        device.device.end_command_buffer(cb)?;
        let cbs = [cb];
        let submits = [vk::SubmitInfo::default().command_buffers(&cbs)];
        {
            let _lock = device.queue_lock.lock().unwrap();
            device
                .device
                .queue_submit(device.queue, &submits, vk::Fence::null())?;
            device.device.queue_wait_idle(device.queue)?;
        }
        device.device.destroy_command_pool(command_pool, None);
    }
    let timings = profiler.results()?;
    let cache_bytes = cache.data()?;
    let device_name =
        unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    println!(
        "{}",
        serde_json::json!({"status":"candidate_live_vulkan_passed","device_name":device_name,"vendor_id":properties.vendor_id,"device_id":properties.device_id,"driver_version":properties.driver_version,"pipeline":"compute","descriptor_write":true,"constant_pages":constants.page_count(),"gpu_dispatches":1,"timestamp_ms":timings[0].milliseconds,"pipeline_cache_bytes":cache_bytes.len(),"original_game_comparison":false})
    );
    Ok(())
}
