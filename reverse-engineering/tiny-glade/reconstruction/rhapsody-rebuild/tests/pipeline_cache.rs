//! 管线缓存的并发性质：同键并发只创建一次（去重），不同键并行创建且全部成功。
//! 无 Vulkan 设备时跳过；本机可用 lavapipe。
use rhapsody::raw::{
    device::VulkanDevice,
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
fn shader(device: &Arc<VulkanDevice>) -> Arc<ShaderModule> {
    let bytes: Vec<u8> = WORDS.iter().flat_map(|w| w.to_le_bytes()).collect();
    ShaderModule::from_spirv(device.clone(), &bytes).unwrap()
}
fn device() -> Option<Arc<VulkanDevice>> {
    create_headless_device().ok().or_else(|| {
        eprintln!("跳过：无可用 Vulkan 设备");
        None
    })
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
                let (cache, layout, shader, barrier) =
                    (cache.clone(), layout.clone(), shader.clone(), barrier.clone());
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
    assert!(results.iter().all(|p| Arc::ptr_eq(p, &results[0])), "同键必须得到同一个管线");
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
