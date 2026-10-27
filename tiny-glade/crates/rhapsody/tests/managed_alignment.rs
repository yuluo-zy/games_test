//! Typed store reuse and actual texture view caching.
//! 视图缓存测试需要 Vulkan 设备，无设备时跳过（本机可用 lavapipe）。
mod gpu_test_support;
use ash::vk;
use rhapsody::device::Device;
use rhapsody::resource::managed::resources::ResourceManager;
use rhapsody::resource::texture::{TextureDesc, TextureViewDesc};
use rhapsody::device::selection::create_headless_device;
use std::sync::Arc;

#[test]
fn released_slots_reuse_formal_generations_without_losing_live_values() {
    let store = ResourceManager::new(Default::default()).unwrap();
    let a = store.allocate(1u32).unwrap();
    let b = store.allocate(2u32).unwrap();
    let c = store.allocate(3u32).unwrap();
    let (ka, kc) = (a.key(), c.key());
    drop(a);
    drop(c);
    let d = store.allocate(4u32).unwrap();
    let e = store.allocate(5u32).unwrap();
    let mut reused = [d.key().index(), e.key().index()];
    reused.sort();
    let mut original = [ka.index(), kc.index()];
    original.sort();
    assert_eq!(reused, original);
    assert_ne!(d.key().handle(), ka.handle());
    assert_ne!(e.key().handle(), kc.handle());
    assert!(store.get(ka).is_none());
    assert!(store.get(kc).is_none());
    assert_eq!(*store.get(b.key()).unwrap(), 2);
    assert_eq!(store.len(), 3);
}

#[test]
fn typed_store_survives_concurrent_churn_without_duplicate_live_slots() {
    let store = Arc::new(ResourceManager::new(Default::default()).unwrap());
    let workers: Vec<_> = (0..6u32)
        .map(|t| {
            let store = store.clone();
            std::thread::spawn(move || {
                for round in 0..400u32 {
                    let handles: Vec<_> = (0..4)
                        .map(|i| store.allocate(t * 100_000 + round * 10 + i).unwrap())
                        .collect();
                    let mut slots: Vec<u32> = handles.iter().map(|h| h.key().index()).collect();
                    slots.sort_unstable();
                    slots.dedup();
                    assert_eq!(slots.len(), 4);
                    for h in &handles {
                        assert_eq!(
                            store.get(h.key()).map(|m| *m.resource()),
                            Some(*h.resource())
                        );
                    }
                    drop(handles);
                }
            })
        })
        .collect();
    for w in workers {
        w.join().unwrap();
    }
    // 所有槽位都已归还：再并发取出的槽位互不相同。
    let live: Vec<_> = (0..64u32).map(|i| store.allocate(i).unwrap()).collect();
    let mut slots: Vec<u32> = live.iter().map(|h| h.key().index()).collect();
    slots.sort_unstable();
    slots.dedup();
    assert_eq!(slots.len(), 64);
}

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize("managed texture views", create_headless_device())
}

#[test]
fn texture_view_cache_returns_the_same_view_for_equal_descriptors() {
    let Some(gd) = device() else { return };
    let desc = TextureDesc::new_2d(16, 16, vk::Format::R8G8B8A8_UNORM)
        .sampled()
        .color_attachment();
    let texture = gd.create_texture(&desc, "cache", ()).unwrap();
    assert_eq!(texture.cached_view_count(), 0);
    let default_a = texture.view(None).unwrap();
    let default_b = texture.view(None).unwrap();
    assert_eq!(
        default_a.raw(),
        default_b.raw(),
        "canonical default view is created once"
    );
    assert_eq!(texture.cached_view_count(), 1);

    let view_desc = TextureViewDesc::new(&desc).unwrap();
    let first = texture.view(Some(&view_desc)).unwrap();
    let again = gd.get_vk_image_view(&texture, &view_desc).unwrap();
    assert_eq!(first.raw(), default_a.raw());
    assert!(Arc::ptr_eq(&first.view, &default_a.view));
    assert_eq!(first.raw(), again.raw());
    assert!(Arc::ptr_eq(&first.view, &again.view));
    assert_eq!(texture.cached_view_count(), 1);

    let mut other = view_desc;
    other.layer_count = 1;
    other.view_type = vk::ImageViewType::TYPE_2D;
    let single = texture.view(Some(&other)).unwrap();
    assert_eq!(
        texture.cached_view_count(),
        1,
        "描述符与默认一致（单层 2D）仍命中同一键"
    );
    assert_eq!(single.raw(), first.raw());

    let mut desc2 = TextureDesc::new_2d(16, 16, vk::Format::R8G8B8A8_UNORM).sampled();
    desc2.mip_levels = 3;
    let mipped = gd.create_texture(&desc2, "mips", ()).unwrap();
    let all = mipped
        .view(Some(&TextureViewDesc::new(&desc2).unwrap()))
        .unwrap();
    let mut one = TextureViewDesc::new(&desc2).unwrap();
    one.level_count = 1;
    let one_view = mipped.view(Some(&one)).unwrap();
    assert_ne!(all.raw(), one_view.raw());
    assert_eq!(mipped.cached_view_count(), 2);
    // 无效范围不入缓存。
    let mut bad = TextureViewDesc::new(&desc2).unwrap();
    bad.level_count = 9;
    assert!(mipped.view(Some(&bad)).is_err());
    assert_eq!(mipped.cached_view_count(), 2);
}
