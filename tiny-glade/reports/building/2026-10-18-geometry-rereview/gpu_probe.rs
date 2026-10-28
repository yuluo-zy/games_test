//! 复审诊断：确认稀疏存储销毁后，生产宿主的 GPU 槽是否仍然存活。
use country_core::{
    geometry::instanced_wall::BrickTransformSsbo,
    render::sparse_instance_buffer::SparseInstanceBuffer,
};
use tiny_glade::render_host::BufferHost;

#[test]
fn dropped_sparse_storage_leaves_its_native_buffer_in_the_host() {
    let device =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())
            .expect("需要真实 GPU，不允许跳过");
    // SAFETY: 读取当前已初始化实例和物理设备的只读属性。
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    println!(
        "maxStorageBufferRange={}",
        properties.limits.max_storage_buffer_range
    );
    let host = BufferHost::new(device);
    let storage = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&host).unwrap();
    let buffer = storage.buffer.buffer;
    println!("brick_buffer_bytes={}", buffer.byte_size);
    drop(storage);
    assert!(
        host.managed_buffer(&buffer).is_some(),
        "当前缺陷：存储已经销毁，设备缓冲仍被宿主保留"
    );
    println!("live_after_storage_drop=true");
    // 诊断主动清理，避免把发现的问题留在测试进程里。
    assert!(host.release(&buffer));
}
