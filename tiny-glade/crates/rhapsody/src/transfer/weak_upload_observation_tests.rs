use super::*;
#[test]
fn status_survives_export_without_retaining_buffer_or_device() {
    let device=Device::new_headless(&crate::device::DeviceConfig::headless())
        .expect("upload observation ownership test requires actual Vulkan");
    let stats=device.allocator_stats().clone();
    let buffer=Arc::new(Buffer::new(device.clone(),&crate::resource::BufferDesc::new_cpu_to_gpu(
        16,vk::BufferUsageFlags::TRANSFER_DST)).unwrap());
    let weak_buffer=Arc::downgrade(&buffer);let weak_device=Arc::downgrade(&device);
    let mut manager=TransferManager::new(device.clone(),TransferConfig::default()).unwrap();
    let ticket=manager.enqueue_upload_owned(buffer.clone(),0,vec![7;16],
        UploadOptions{key:UploadKey(1),epoch:0,priority:UploadPriority::High},UploadMode::OrderedDelta).unwrap();
    let observed=ticket.observe();drop(ticket);
    let deadline=std::time::Instant::now()+std::time::Duration::from_secs(10);
    let point=loop {
        manager.flush_uploads(4).unwrap();
        if let UploadStatus::Submitted(point)=observed.status(){break point;}
        assert!(std::time::Instant::now()<deadline,"upload observation never submitted");
        std::thread::yield_now();
    };
    device.wait(&SyncSet::from_point(point),10_000_000_000).unwrap();
    device.wait_idle_and_collect().unwrap();drop(manager);drop(buffer);
    device.wait_idle_and_collect().unwrap();
    assert!(weak_buffer.upgrade().is_none(),"CPU status observer must not pin the GPU buffer");
    assert!(!observed.owner_alive());assert!(matches!(observed.status(),UploadStatus::Submitted(p) if p==point));
    drop(device);assert!(weak_device.upgrade().is_none(),"status observer must not retain Device");
    assert!(!observed.device_alive());
    let before=stats.cpu_memory_usage().total();drop(observed);
    assert!(stats.cpu_memory_usage().total()<before,"independent Task credit must be released");
}
#[test]
fn pending_cancelled_status_does_not_disappear_with_manager() {
    let device=Device::new_headless(&crate::device::DeviceConfig::headless())
        .expect("upload observation cancellation test requires actual Vulkan");
    let buffer=Arc::new(Buffer::new(device.clone(),&crate::resource::BufferDesc::new_cpu_to_gpu(
        16,vk::BufferUsageFlags::TRANSFER_DST)).unwrap());
    let mut manager=TransferManager::new(device,TransferConfig::default()).unwrap();
    let ticket=manager.enqueue_upload_owned(buffer.clone(),0,vec![1;16],
        UploadOptions{key:UploadKey(1),epoch:0,priority:UploadPriority::High},UploadMode::OrderedDelta).unwrap();
    let observed=ticket.observe();drop(ticket);drop(manager);drop(buffer);
    assert!(matches!(observed.status(),UploadStatus::Cancelled(UploadCancellation::ManagerDropped)));
    assert!(!observed.owner_alive());
}
