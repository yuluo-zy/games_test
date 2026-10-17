//! 后端导入 token 与跨帧 temporal token 必须来自同一个空间：先后各分配一个资源，token 不能撞。
mod gpu_test_support;
use ash::vk;
use rhapsody::raw::{
    buffer::{Buffer, BufferDesc},
    init::create_headless_device,
};
use rhapsody::rg::vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend};
use std::sync::Arc;

fn buffer(device: &Arc<rhapsody::raw::device::Device>) -> Arc<Buffer> {
    Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER),
        )
        .unwrap(),
    )
}

#[test]
fn named_and_external_resources_coexist_without_identity_collisions() {
    use rhapsody::rg::vulkan::VulkanGraphSession;
    let Some(device) =
        gpu_test_support::initialize("named/external token identity", create_headless_device())
    else {
        return;
    };
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let imported = session
        .backend
        .bind_import(GpuResource::Buffer(buffer(&device)))
        .unwrap();
    let mut graph = session.begin_frame().unwrap();
    let desc = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let named = session.named_buffer(&mut graph, "history", desc).unwrap();
    assert!(!named.history_valid);
    let external = graph
        .import_unowned(
            "external",
            ResourceSpec::Buffer(desc).descriptor(),
            imported,
            0x34,
        )
        .unwrap();
    let mut pass = graph.add_pass("distinct objects").with_side_effect();
    pass.add_resource_access(named.buffer.raw(), 0x34, 0)
        .unwrap();
    pass.add_resource_access(external, 0x34, 0).unwrap();
    pass.render(|_| true);
    let receipt = session.execute_frame(graph.build()).unwrap();
    session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
    assert_eq!(session.named_resource_count(), 1);
    assert_eq!(session.resource_count(), 2);
}

/// 导入资源可以按 token 单独释放：仍被在飞帧引用时拒绝，帧完成后立刻放掉，不需要整设备空闲。
#[test]
fn imported_resources_are_released_per_token_once_their_frame_completed() {
    use rhapsody::rg::vulkan::VulkanGraphSession;
    let Some(device) =
        gpu_test_support::initialize("named/external token identity", create_headless_device())
    else {
        return;
    };
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let desc = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let descriptor = ResourceSpec::Buffer(desc).descriptor();
    let weak = {
        let buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
        let weak = Arc::downgrade(&buffer);
        let token = session
            .backend
            .bind_import(GpuResource::Buffer(buffer))
            .unwrap();
        let mut graph = session.begin_frame().unwrap();
        let handle = graph
            .import_unowned("imported", descriptor, token, 0x34)
            .unwrap();
        let mut pass = graph.add_pass("touch").with_side_effect();
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
