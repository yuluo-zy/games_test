//! 后端导入 token 与跨帧 temporal token 必须来自同一个空间：先后各分配一个资源，token 不能撞。
use ash::vk;
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    init::create_headless_device,
};
use rhapsody::rg::{
    graph::Descriptor,
    temporal::TemporalResourceOwner,
    vulkan::{AccessTable, GpuResource, ResourceSpec, TemporalGpuResources, VulkanGraphBackend},
};
use std::sync::Arc;

fn buffer(device: &Arc<rhapsody::raw::device::VulkanDevice>) -> Arc<Buffer> {
    Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER),
        )
        .unwrap(),
    )
}

#[test]
fn temporal_tokens_never_collide_with_backend_import_tokens() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut temporal = TemporalGpuResources::new(device.clone());
    let desc = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let descriptor: Descriptor = temporal.register("history", ResourceSpec::Buffer(desc));

    // 先让后端导入一个外部资源（拿到最小 token），再创建 temporal 资源。
    let imported = backend.bind_import(GpuResource::Buffer(buffer(&device)));
    let created = temporal.create_temporal("history", descriptor).unwrap();
    assert_ne!(imported, created, "两个来源的 token 撞了");

    // 把 temporal 资源导入同一个后端：不能因为 token 已被占用而失败。
    temporal
        .import_into(&mut backend, "history", created)
        .expect("temporal 导入不应因 token 冲突失败");

    // 反方向：temporal 先分配，后端随后 bind_import，也不能撞。
    let second = temporal.create_temporal("history", descriptor).unwrap();
    let later = backend.bind_import(GpuResource::Buffer(buffer(&device)));
    assert_ne!(second, later);
    assert!(later != imported && later != created);
}

/// 导入资源可以按 token 单独释放：仍被在飞帧引用时拒绝，帧完成后立刻放掉，不需要整设备空闲。
#[test]
fn imported_resources_are_released_per_token_once_their_frame_completed() {
    use rhapsody::rg::vulkan::VulkanGraphSession;
    let Ok(device) = create_headless_device() else {
        return;
    };
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let desc = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let descriptor = ResourceSpec::Buffer(desc).descriptor();
    let weak = {
        let buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
        let weak = Arc::downgrade(&buffer);
        let token = session.backend.bind_import(GpuResource::Buffer(buffer));
        let mut graph = session.begin_frame().unwrap();
        let handle = graph
            .import_unowned("imported", descriptor, token, 0x34)
            .unwrap();
        let mut pass = graph.add_pass("touch");
        pass.add_resource_access(handle, 0x34, 0).unwrap();
        pass.render(|_| true);
        let receipt = session.execute_frame(graph.build()).unwrap();

        assert_eq!(session.resource_count(), 1);
        assert!(
            session.release_import(token).is_err(),
            "在飞帧还引用着这个 token，不能释放"
        );
        assert_eq!(session.resource_count(), 1, "拒绝时不能有副作用");

        session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
        assert!(session.release_import(token).unwrap(), "帧完成后应当释放");
        assert_eq!(session.resource_count(), 0);
        assert!(
            !session.release_import(token).unwrap(),
            "重复释放返回 false，不报错"
        );
        weak
    };
    device.wait_idle().unwrap();
    assert!(weak.upgrade().is_none(), "释放后后端不应再持有这个缓冲");
}
