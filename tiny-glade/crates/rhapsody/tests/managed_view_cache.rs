//! Actual native view ownership, bounded cache admission and in-flight eviction.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::{
        command_buffer::VulkanCommandPool,
        managed::texture::MAX_CACHED_TEXTURE_VIEWS,
        semaphore::TimelineSemaphore,
        texture::{TextureDesc, TextureViewDesc},
    },
};
use std::sync::Arc;
fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "bounded managed view cache",
        Device::new_headless(&DeviceConfig::headless()),
    )
}
struct Release(Arc<TimelineSemaphore>, bool);
impl Release {
    fn signal(&mut self) -> rhapsody::Result<()> {
        if !self.1 {
            unsafe {
                self.0.device.device.signal_semaphore(
                    &vk::SemaphoreSignalInfo::default()
                        .semaphore(self.0.handle)
                        .value(1),
                )
            }?;
            self.1 = true;
        }
        Ok(())
    }
}
impl Drop for Release {
    fn drop(&mut self) {
        let _ = self.signal();
    }
}
#[test]
fn view_cache_rejects_held_and_pending_views_then_reuses_completed_capacity() {
    let Some(device) = device() else { return };
    let mut desc = TextureDesc::new_2d_array(16, 16, 16, vk::Format::R8G8B8A8_UNORM).sampled();
    desc.mip_levels = 2;
    let texture = device.create_texture(&desc, "bounded views", ()).unwrap();
    let cache_before = device.allocator_stats.cpu_memory_usage().cache;
    let mut descriptors = Vec::new();
    for mip in 0..2 {
        for first in 0..16 {
            for count in 1..=16 - first {
                let mut view = TextureViewDesc::new(&desc).unwrap();
                view.base_mip_level = mip;
                view.level_count = 1;
                view.base_layer = first;
                view.layer_count = count;
                descriptors.push(view);
            }
        }
    }
    assert!(descriptors.len() > MAX_CACHED_TEXTURE_VIEWS);
    let mut held = Vec::new();
    for descriptor in &descriptors[..MAX_CACHED_TEXTURE_VIEWS] {
        held.push(texture.view(Some(descriptor)).unwrap());
    }
    let observers: Vec<_> = held.iter().map(|view| Arc::downgrade(&view.view)).collect();
    let cache_full = device.allocator_stats.cpu_memory_usage().cache;
    assert!(cache_full > cache_before);
    let extra = &descriptors[MAX_CACHED_TEXTURE_VIEWS];
    assert!(matches!(
        texture.view(Some(extra)),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(texture.cached_view_count(), MAX_CACHED_TEXTURE_VIEWS);
    let hit = texture.view(Some(&descriptors[0])).unwrap();
    assert!(Arc::ptr_eq(&hit.view, &held[0].view));
    drop(hit);

    let blocker = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let mut release = Release(blocker.clone(), false);
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    // Use actual SyncCells and command owners. No native image operation is
    // needed to test that the real receipt protects every retained view.
    for view in &held {
        command.retain_opaque(view.view.clone()).unwrap();
    }
    command.end().unwrap();
    let pending = command
        .submit_to(
            rhapsody::raw::device::DeviceQueue {
                handle: device.queue,
                family: device.queue_family,
                index: 0,
            },
            &[(blocker, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    drop(held);
    assert!(!pending.is_complete().unwrap());
    let point = pending.point;
    drop(pending);
    // The cache is now the only CPU owner. Drop every observer too, so the
    // next rejection independently exercises last_use rather than Arc counts.
    assert!(observers.iter().all(|view| view.strong_count() == 1));
    drop(observers);
    assert!(!device.timelines.is_reached(&point.into()).unwrap());
    assert!(matches!(
        texture.view(Some(extra)),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(texture.cached_view_count(), MAX_CACHED_TEXTURE_VIEWS);
    release.signal().unwrap();
    device
        .timelines
        .wait(&point.into(), 10_000_000_000)
        .unwrap();
    pool.trim_cpu_scratch();
    device.try_collect_all_completed().unwrap();
    let admitted = texture.view(Some(extra)).unwrap();
    assert_eq!(texture.cached_view_count(), MAX_CACHED_TEXTURE_VIEWS);
    assert_eq!(device.allocator_stats.cpu_memory_usage().cache, cache_full);
    drop(admitted);
    drop(texture);
    device.try_collect_all_completed().unwrap();
    assert_eq!(
        device.allocator_stats.cpu_memory_usage().cache,
        cache_before
    );
}
#[test]
fn metadata_budget_rejects_view_before_cache_or_native_publication() {
    let Some(device) = device() else { return };
    let desc = TextureDesc::new_2d(4, 4, vk::Format::R8G8B8A8_UNORM).sampled();
    let texture = device.create_texture(&desc, "view quota", ()).unwrap();
    let before = device.allocator_stats.cpu_memory_usage();
    let total = device.allocator_stats.cpu_memory_usage().total();
    device.allocator_stats.set_host_budget(total + 1).unwrap();
    assert!(matches!(
        texture.view(None),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(texture.cached_view_count(), 0);
    assert_eq!(device.allocator_stats.cpu_memory_usage(), before);
    device.allocator_stats.set_total_budget(u64::MAX).unwrap();
    let canonical = texture.view(None).unwrap();
    let explicit = texture
        .view(Some(&TextureViewDesc::new(&desc).unwrap()))
        .unwrap();
    assert!(Arc::ptr_eq(&canonical.view, &explicit.view));
    assert_eq!(texture.cached_view_count(), 1);
}
