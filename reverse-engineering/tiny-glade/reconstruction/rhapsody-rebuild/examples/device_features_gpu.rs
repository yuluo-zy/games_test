//! 设备地址、真实时间线等待/信号与回收的功能验收。
use ash::vk;
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    command_buffer::VulkanCommandPool,
    device::DeviceQueue,
    dynamic_constants::DynamicConstants,
    init::create_headless_device,
    semaphore::TimelineSemaphore,
};
use std::sync::Arc;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let props = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name = unsafe { std::ffi::CStr::from_ptr(props.device_name.as_ptr()) }.to_string_lossy();
    let alignment = props
        .limits
        .min_uniform_buffer_offset_alignment
        .max(props.limits.min_storage_buffer_offset_alignment)
        .max(4);
    let mut constants = DynamicConstants::new(device.clone(), 4096, alignment)?;
    let a = constants.allocate_with_data(&[1, 2, 3, 4])?;
    let b = constants.allocate_with_data(&[5, 6, 7, 8])?;
    let addresses = if device.capabilities.buffer_device_address {
        let base = a.device_address()?;
        let second = b.device_address()?;
        assert_ne!(base, 0);
        assert_eq!(second - base, b.offset - a.offset);
        Some([base, second])
    } else {
        assert!(a.device_address().is_err());
        None
    };
    constants.reset()?;
    let c = constants.allocate_with_data(&[9, 10, 11, 12])?;
    assert!(c.offset > b.offset);
    drop(a);
    drop(b);
    drop(c);
    constants.reset()?;
    let output = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, vk::BufferUsageFlags::TRANSFER_DST),
    )?);
    let signal = TimelineSemaphore::new(device.clone(), 0)?;
    let primary = DeviceQueue {
        handle: device.queue,
        family: device.queue_family,
        index: 0,
    };
    let pool = VulkanCommandPool::new(device.clone())?;
    let mut first = pool.allocate()?;
    first.begin()?;
    first.end()?;
    let first = first.submit_to(primary, &[], &[(signal.clone(), 1)])?;
    let selected = device.additional_queues.first().copied().unwrap_or(primary);
    let pool2 = VulkanCommandPool::new_for_family(device.clone(), selected.family)?;
    let mut second = pool2.allocate()?;
    second.begin()?;
    second.keep_alive(output.clone());
    unsafe {
        device
            .device
            .cmd_fill_buffer(second.handle, output.handle, 0, 256, 0x55667788);
        let barrier = vk::BufferMemoryBarrier::default()
            .buffer(output.handle)
            .size(vk::WHOLE_SIZE)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ);
        device.device.cmd_pipeline_barrier(
            second.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[barrier],
            &[],
        )
    }
    second.end()?;
    let second = second.submit_to(
        selected,
        &[(signal.clone(), 1, vk::PipelineStageFlags::TRANSFER)],
        &[(signal.clone(), 2)],
    )?;
    signal.wait(2, 10_000_000_000)?;
    second.wait(10_000_000_000)?;
    first.wait(10_000_000_000)?;
    let mut bytes = [0u8; 256];
    output.read_bytes(0, &mut bytes)?;
    assert!(
        bytes
            .chunks_exact(4)
            .all(|b| b == 0x55667788u32.to_le_bytes())
    );
    println!(
        "{}",
        serde_json::json!({"status":"passed","device":name,"buffer_device_address_enabled":device.capabilities.buffer_device_address,"addresses":addresses,"reset_completed":true,"timeline_wait_signal":true,"completed_value":signal.completed()?,"readback_bytes":256,"additional_queues":device.additional_queues.len(),"distinct_queue_tested":selected.handle!=primary.handle})
    );
    Ok(())
}
