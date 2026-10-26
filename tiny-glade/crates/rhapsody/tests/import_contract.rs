//! 实际设备上的导入边界；缺少 Vulkan 时明确跳过，CPU 描述校验另有库内测试。
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    device::Device,
    init::create_headless_device,
    texture::{Texture, TextureDesc},
};
use rhapsody::rg::vulkan::{
    AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize("import contract device", create_headless_device())
}

#[test]
fn foreign_device_imports_fail_without_replacing_bindings() {
    let Some(current) = device() else { return };
    let foreign = create_headless_device().expect("second logical device");
    assert_ne!(current.generation(), foreign.generation());
    let mut backend = VulkanGraphBackend::new(current.clone(), AccessTable::original()).unwrap();
    let desc = BufferDesc::new_gpu_only(64, vk::BufferUsageFlags::TRANSFER_DST);
    let local = Arc::new(Buffer::new(current.clone(), &desc).unwrap());
    let token = (1u64 << 62) | 17;
    backend
        .bind_import_named("local", token, GpuResource::Buffer(local.clone()))
        .unwrap();
    let foreign_buffer =
        GpuResource::Buffer(Arc::new(Buffer::new(foreign.clone(), &desc).unwrap()));
    let mut image_desc = TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM);
    image_desc.usage = vk::ImageUsageFlags::TRANSFER_DST;
    let foreign_image = GpuResource::Image(Arc::new(
        Texture::new(foreign.clone(), &image_desc).unwrap(),
    ));
    for resource in [foreign_buffer, foreign_image] {
        let errors = [
            backend.bind_import(resource.clone()).unwrap_err(),
            backend
                .bind_import_owned_by(resource.clone(), current.queue_family_index())
                .unwrap_err(),
            backend
                .bind_import_named("local", token, resource.clone())
                .unwrap_err(),
            backend
                .bind_import_named_owned_by(
                    "foreign",
                    token + 1,
                    resource,
                    current.queue_family_index(),
                )
                .unwrap_err(),
        ];
        for error in errors {
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        }
        assert_eq!(backend.resources.len(), 1);
        assert!(
            matches!(&backend.resources[&token], GpuResource::Buffer(actual) if Arc::ptr_eq(actual, &local))
        );
    }
}

#[test]
fn explicit_import_domain_cannot_replace_generated_resources() {
    let Some(device) = device() else { return; };
    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let desc = BufferDesc::new_gpu_only(64, vk::BufferUsageFlags::TRANSFER_DST);
    let internal = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
    let external = Arc::new(Buffer::new(device, &desc).unwrap());
    let generated = backend.bind_import(GpuResource::Buffer(internal.clone())).unwrap();
    for token in [0, generated, (1u64 << 62) - 1, u64::MAX] {
        assert_eq!(backend.bind_import_named("external", token, GpuResource::Buffer(external.clone())).unwrap_err().kind(), rhapsody::ErrorKind::InvalidUsage);
    }
    let host = (1u64 << 62) | generated;
    backend.bind_import_named("external", host, GpuResource::Buffer(external.clone())).unwrap();
    let next = backend.bind_import(GpuResource::Buffer(internal.clone())).unwrap();
    assert!(next < (1u64 << 62));
    assert_ne!(generated, next);
    assert!(matches!(&backend.resources[&generated], GpuResource::Buffer(actual) if Arc::ptr_eq(actual, &internal)));
    assert!(matches!(&backend.resources[&host], GpuResource::Buffer(actual) if Arc::ptr_eq(actual, &external)));
}

#[test]
fn invalid_import_is_rejected_before_recording_and_next_frame_remains_usable() {
    let Some(device) = device() else { return };
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let desc = BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST);
    let buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(buffer.clone()))
        .unwrap();
    let descriptor = ResourceSpec::Buffer(desc).descriptor();
    let bad_descriptor = ResourceSpec::Buffer(BufferDesc { size: 128, ..desc }).descriptor();
    for (declared, imported) in [(bad_descriptor, token), (descriptor, 0)] {
        let recorded = Arc::new(AtomicBool::new(false));
        let retired = Arc::new(AtomicBool::new(false));
        let mut graph = session.begin_frame().unwrap();
        let handle = graph
            .import_unowned("readback", declared, imported, 0)
            .unwrap();
        let mut pass = graph.add_pass("must not record").with_side_effect();
        pass.add_resource_access(handle, 0x34, 0).unwrap();
        let flag = recorded.clone();
        pass.render(move |_| {
            flag.store(true, Ordering::SeqCst);
            true
        });
        let flag = retired.clone();
        graph.on_retire(move || flag.store(true, Ordering::SeqCst));
        let submitted = device.timelines().submitted_set();
        assert_eq!(
            session.execute_frame(graph.build()).unwrap_err().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
        assert!(!recorded.load(Ordering::SeqCst));
        assert!(retired.load(Ordering::SeqCst));
        assert_eq!(device.timelines().submitted_set(), submitted);
        assert_eq!(session.pending_frame_count(), 0);
        assert_eq!(session.command_count(), 0);
        assert!(!session.is_poisoned());
    }
    let mut graph = session.begin_frame().unwrap();
    let handle = graph
        .import_unowned("readback", descriptor, token, 0)
        .unwrap();
    let mut pass = graph.add_pass("fill").with_side_effect();
    pass.add_resource_access(handle, 0x34, 0).unwrap();
    let dev = device.clone();
    let target = buffer.clone();
    pass.render(move |context| {
        unsafe {
            dev.raw_device().cmd_fill_buffer(
                vk::CommandBuffer::from_raw(context),
                target.raw_handle(),
                0,
                64,
                0x12345678,
            )
        };
        true
    });
    graph.export(handle, 0x26).unwrap();
    let receipt = session.execute_frame(graph.build()).unwrap();
    session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
    let mut bytes = [0; 64];
    // 该缓冲只有本次图写入，上面已等待覆盖它的提交完成。
    unsafe { buffer.read_bytes_after_completion(0, &mut bytes) }.unwrap();
    assert_eq!(bytes.as_slice(), 0x12345678u32.to_ne_bytes().repeat(16));
}
