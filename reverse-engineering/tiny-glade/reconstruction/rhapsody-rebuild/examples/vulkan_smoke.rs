//! 实际 GPU 验证：构图、屏障、回调命令、时间线提交、读回共同运行。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        init::create_headless_device,
    },
    rg::{
        graph::RenderGraph,
        pass_api::{AshCommandBackend, RenderPassApi},
        vulkan::{AccessSpec, AccessTable, GpuResource, VulkanGraphBackend},
    },
};
use std::sync::Arc;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name =
        unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
    let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
    let dst = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, usage),
    )?);
    let mut table = AccessTable::default();
    table.0.insert(
        40,
        AccessSpec {
            original_mask: 0x1000,
            write: true,
            stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_WRITE,
            layout: vk::ImageLayout::GENERAL,
        },
    );
    table.0.insert(
        4,
        AccessSpec {
            original_mask: 0x800,
            write: false,
            stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_READ,
            layout: vk::ImageLayout::GENERAL,
        },
    );
    table.0.insert(
        55,
        AccessSpec {
            original_mask: 0x800,
            write: false,
            stages: vk::PipelineStageFlags::HOST,
            access: vk::AccessFlags::HOST_READ,
            layout: vk::ImageLayout::GENERAL,
        },
    );
    let mut backend = VulkanGraphBackend::new(device.clone(), table)?;
    let destination = backend.bind_import(GpuResource::Buffer(dst.clone()));
    // 已发布的旧名字不能越过本帧裁剪重新暴露，模拟持久 resolver 上一帧的残留。
    backend
        .bind_import_named("unused", destination, GpuResource::Buffer(dst.clone()))
        .map_err(|error| format!("{error:?}"))?;
    let mut graph = RenderGraph::new();
    let desc = rhapsody::rg::graph::Descriptor::Buffer(rhapsody::rg::graph::BufferDesc {
        word_at_00: 256,
        word_at_08: usage.as_raw(),
        bytes_at_0c_through_0e: [0; 3],
    });
    let source = backend
        .declare(
            &mut graph,
            "source",
            rhapsody::rg::vulkan::ResourceSpec::Buffer(BufferDesc::new_gpu_only(256, usage)),
        )
        .unwrap();
    // 插在活跃资源之间，验证裁剪不会改变后续逻辑资源索引或创建实际 GPU 对象。
    let unused = backend
        .declare(
            &mut graph,
            "unused",
            rhapsody::rg::vulkan::ResourceSpec::Buffer(BufferDesc::new_gpu_only(128, usage)),
        )
        .unwrap();
    let resolver = backend.resolver.clone();
    let destination = graph
        .import_owned("destination", desc, destination, 0)
        .unwrap();
    let mut fill = graph.add_pass("fill");
    fill.add_resource_access(source, 40, 0).unwrap();
    let d = device.clone();
    fill.render(move |handle| {
        let mut backend = unsafe { AshCommandBackend::new(&d.device) };
        let mut api =
            RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(handle), 0, false);
        api.fill_buffer(resolver.buffer("source").unwrap(), None, 0x11223344)
            .is_ok()
    });
    let mut copy = graph.add_pass("copy");
    copy.add_resource_access(source, 4, 0).unwrap();
    copy.add_resource_access(destination, 40, 0).unwrap();
    let d = device.clone();
    let destination_handle = dst.handle;
    let resolver = backend.resolver.clone();
    copy.render(move |handle| {
        let mut backend = unsafe { AshCommandBackend::new(&d.device) };
        let mut api =
            RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(handle), 0, false);
        api.copy_buffer(
            resolver.buffer("source").unwrap(),
            destination_handle,
            &[vk::BufferCopy {
                src_offset: 0,
                dst_offset: 0,
                size: 256,
            }],
        );
        true
    });
    graph.export(destination, 55).unwrap();
    let mut run = backend
        .execute(graph.build())
        .map_err(std::io::Error::other)?;
    run.wait(10_000_000_000).map_err(std::io::Error::other)?;
    assert!(
        run.execution.compiled.snapshot.physical_mapping[unused.raw() as usize]
            .token
            .is_none()
    );
    assert_eq!(run.execution.resources.len(), 2);
    assert!(run.backend.resolver.get("unused").is_none());
    let timeline = run.backend.timeline.completed()?;
    let mut output = [0u8; 256];
    dst.read_bytes(0, &mut output)?;
    let correct = output
        .chunks_exact(4)
        .all(|b| b == 0x11223344u32.to_le_bytes());
    println!(
        "{}",
        serde_json::json!({"status":if correct{"passed"}else{"failed"},"device":name,"bytes":256,"passes":2,"accesses":run.execution.compiled.info.accesses.len(),"timeline":timeline,"receipts":run.execution.receipts.len(),"pattern":"11223344","real_gpu":true,"automatic_resource_creation":true,"unused_resource_culled":true})
    );
    if !correct {
        return Err("GPU读回不一致".into());
    }
    Ok(())
}
