//! 管线缓存的并发性质：同键并发只创建一次（去重），不同键并行创建且全部成功。
//! 无 Vulkan 设备时跳过；本机可用 lavapipe。
mod gpu_test_support;
use rhapsody::raw::{
    device::Device,
    init::create_headless_device,
    shader::{ComputePipelineDesc, PipelineCache, PipelineLayout, ShaderModule, VulkanPipeline},
};
use std::sync::{Arc, Barrier};

/// SPIR-V 1.0 空 compute main（local size 1）。
const WORDS: &[u32] = &[
    0x07230203, 0x00010000, 0, 6, 0, 0x00020011, 1, 0x0003000e, 0, 1, 0x0005000f, 5, 4, 0x6e69616d,
    0, 0x00060010, 4, 17, 1, 1, 1, 0x00020013, 1, 0x00030021, 2, 1, 0x00050036, 1, 4, 0, 2,
    0x000200f8, 5, 0x000100fd, 0x00010038,
];
fn shader(device: &Arc<Device>) -> Arc<ShaderModule> {
    let bytes: Vec<u8> = WORDS.iter().flat_map(|w| w.to_le_bytes()).collect();
    ShaderModule::from_spirv(device.clone(), &bytes).unwrap()
}
fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize("concurrent pipeline cache device", create_headless_device())
}

#[test]
fn concurrent_requests_for_the_same_key_create_exactly_one_pipeline() {
    let Some(device) = device() else { return };
    let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let shader = shader(&device);
    let threads = 8;
    let barrier = Arc::new(Barrier::new(threads));
    let results: Vec<_> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                let (cache, layout, shader, barrier) = (
                    cache.clone(),
                    layout.clone(),
                    shader.clone(),
                    barrier.clone(),
                );
                scope.spawn(move || {
                    barrier.wait();
                    VulkanPipeline::create_compute_pipeline(
                        &cache,
                        ComputePipelineDesc::new(shader, layout),
                    )
                    .unwrap()
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    assert_eq!(cache.cached_compute_count(), 1);
    assert!(
        results.iter().all(|p| Arc::ptr_eq(p, &results[0])),
        "同键必须得到同一个管线"
    );
}

#[test]
fn distinct_keys_created_in_parallel_all_succeed_and_are_cached() {
    let Some(device) = device() else { return };
    let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let count = 24;
    let shaders: Vec<_> = (0..count).map(|_| shader(&device)).collect();
    let pipelines: Vec<_> = std::thread::scope(|scope| {
        let handles: Vec<_> = shaders
            .iter()
            .map(|shader| {
                let (cache, layout, shader) = (cache.clone(), layout.clone(), shader.clone());
                scope.spawn(move || {
                    VulkanPipeline::create_compute_pipeline(
                        &cache,
                        ComputePipelineDesc::new(shader, layout),
                    )
                    .unwrap()
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    assert_eq!(cache.cached_compute_count(), count);
    // 再次请求命中缓存，返回同一对象。
    for (shader, first) in shaders.iter().zip(&pipelines) {
        let again = VulkanPipeline::create_compute_pipeline(
            &cache,
            ComputePipelineDesc::new(shader.clone(), layout.clone()),
        )
        .unwrap();
        assert!(Arc::ptr_eq(&again, first));
    }
    // 缓存数据在并发创建后仍可读取。
    assert!(cache.data().is_ok());
}

#[test]
fn retained_driver_data_keeps_its_real_cpu_budget_after_cache_destruction() {
    let Some(device) = device() else { return };
    let stats = device.allocator_stats().clone();
    let cache = PipelineCache::new(device.clone(), &[]).unwrap();
    let before = stats.cpu_memory_usage().total();
    let data = cache.data().unwrap();
    assert!(!data.is_empty());
    let retained = stats.cpu_memory_usage().total();
    assert!(retained >= before + data.len() as u64);
    stats
        .set_total_budget(stats.budget_usage().total())
        .unwrap();
    assert!(matches!(
        cache.data(),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), retained);
    drop(cache);
    drop(device);
    assert!(stats.cpu_memory_usage().total() >= data.len() as u64);
    drop(data);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn disk_cache_admission_failure_preserves_the_previous_file_and_error_category() {
    use rhapsody::pipeline::{DiskCacheStatus, PersistentPipelineCache};
    let Some(device) = device() else { return };
    let path = std::env::temp_dir().join(format!(
        "rhapsody-cache-budget-{}-{}.bin",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let disk = PersistentPipelineCache::open(device.clone(), &path, 8).unwrap();
    assert_eq!(disk.status, DiskCacheStatus::Missing);
    disk.save().unwrap();
    let original = std::fs::read(&path).unwrap();
    let baseline = device.allocator_stats().cpu_memory_usage().total();
    // Leave one byte available when the otherwise empty device has no charged
    // memory. The disk envelope and driver output both need more than one byte.
    device
        .allocator_stats()
        .set_total_budget(device.allocator_stats().budget_usage().total().max(1))
        .unwrap();
    assert!(matches!(disk.save(), Err(rhapsody::Error::OutOfHostMemory)));
    assert!(matches!(
        PersistentPipelineCache::open(device.clone(), &path, 8),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total(),
        baseline
    );
    assert_eq!(std::fs::read(&path).unwrap(), original);
    device.allocator_stats().set_total_budget(u64::MAX).unwrap();
    let loaded = PersistentPipelineCache::open(device, &path, 8).unwrap();
    assert_eq!(loaded.status, DiskCacheStatus::Loaded);
    std::fs::remove_file(path).unwrap();
}
