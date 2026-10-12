//! 帧节流的真实GPU消费链：trim不弹出，第三次begin才等待并排出最老帧。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{buffer::BufferDesc, init::create_headless_device},
    rg::{
        graph::RenderGraph,
        vulkan::{AccessSpec, AccessTable, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::ffi::CStr;

fn frame(
    session: &mut VulkanGraphSession,
    value: u32,
) -> Result<rhapsody::rg::graph::BuiltGraph, Box<dyn std::error::Error>> {
    let mut graph: RenderGraph = session.begin_frame()?;
    let resource = session.backend.declare(
        &mut graph,
        "frame",
        ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
            16,
            vk::BufferUsageFlags::TRANSFER_DST,
        )),
    )?;
    let device = session.backend.device.clone();
    let resolver = session.backend.resolver.clone();
    let mut pass = graph.add_pass("本帧填充");
    pass.add_resource_access(resource, 1, 0)?;
    pass.render(move |handle| {
        unsafe {
            device.device.cmd_fill_buffer(
                vk::CommandBuffer::from_raw(handle),
                resolver.buffer("frame").unwrap(),
                0,
                16,
                value,
            )
        };
        true
    });
    Ok(graph.build())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    let mut access = AccessTable::default();
    access.0.insert(
        1,
        AccessSpec {
            original_mask: 0x40,
            write: true,
            stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_WRITE,
            layout: vk::ImageLayout::GENERAL,
        },
    );
    let mut session = VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access)?);
    let built = frame(&mut session, 16)?;
    let first = session.execute_frame(built)?;
    assert_eq!(session.pending_frame_count(), 1);
    assert!(session.wait_gpu_queue_trim(u64::MAX)?);
    assert_eq!(session.pending_frame_count(), 1);
    let built = frame(&mut session, 17)?;
    let second = session.execute_frame(built)?;
    assert_eq!(session.pending_frame_count(), 2);
    let built = frame(&mut session, 18)?;
    assert_eq!(session.pending_frame_count(), 1);
    let third = session.execute_frame(built)?;
    assert_eq!(session.pending_frame_count(), 2);
    session.wait_frame(second.frame, u64::MAX)?;
    session.wait_frame(third.frame, u64::MAX)?;
    assert_eq!(session.pending_frame_count(), 0);
    let bytes = session.backend.resolver.buffer("frame").unwrap();
    let buffer = session
        .backend
        .resources
        .values()
        .find_map(|resource| match resource {
            rhapsody::rg::vulkan::GpuResource::Buffer(buffer) if buffer.handle == bytes => {
                Some(buffer)
            }
            _ => None,
        })
        .unwrap();
    let mut output = [0; 4];
    buffer.read_bytes(0, &mut output)?;
    assert_eq!(u32::from_le_bytes(output), 18);
    println!(
        "{}",
        serde_json::json!({"status":"passed","device":name,"frames":[first.frame,second.frame,third.frame],"trim_peeks_without_pop":true,"begin_frame_trims_to_one":true,"final_word":18,"original_full_frame_equivalence":false})
    );
    Ok(())
}
