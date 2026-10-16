//! managed 域补充测试：空闲链表无锁复用、纹理视图缓存。
//! 视图缓存测试需要 Vulkan 设备，无设备时跳过（本机可用 lavapipe）。
use ash::vk;
use rhapsody::raw::{
    device::GraphicsDevice,
    init::create_headless_device,
    managed::resources::ResourceManager,
    texture::{TextureDesc, TextureViewDesc},
};
use std::sync::Arc;

#[test]
fn free_list_is_lifo_and_reuses_the_most_recently_released_slot() {
    let store = ResourceManager::new();
    let a = store.allocate(1u32);
    let b = store.allocate(2u32);
    let c = store.allocate(3u32);
    let (ia, ib, ic) = (a.key().index, b.key().index, c.key().index);
    assert_eq!((ia, ib, ic), (0, 1, 2), "空表时追加新槽");
    drop(a);
    drop(c);
    store.take_out_dropped_waste();
    // 原 allocate：CAS 弹出头结点；dispose 先释放 a 再释放 c，所以头是 c（LIFO）。
    let d = store.allocate(4u32);
    let e = store.allocate(5u32);
    assert_eq!((d.key().index, e.key().index), (ic, ia));
    // 复用槽的校验和已递增，旧键失效。
    assert_eq!(d.key().resource_checksum, 1);
    assert!(store.get(b.key()).is_some());
    // 链空后继续追加。
    let f = store.allocate(6u32);
    assert_eq!(f.key().index, 3);
}

#[test]
fn free_list_survives_concurrent_churn_without_duplicate_live_slots() {
    let store = Arc::new(ResourceManager::new());
    let workers: Vec<_> = (0..6u32)
        .map(|t| {
            let store = store.clone();
            std::thread::spawn(move || {
                for round in 0..400u32 {
                    let handles: Vec<_> = (0..4)
                        .map(|i| store.allocate(t * 100_000 + round * 10 + i))
                        .collect();
                    let mut slots: Vec<u32> = handles.iter().map(|h| h.key().index).collect();
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
                    store.take_out_dropped_waste();
                }
            })
        })
        .collect();
    for w in workers {
        w.join().unwrap();
    }
    store.take_out_dropped_waste();
    // 所有槽位都已归还：再并发取出的槽位互不相同。
    let live: Vec<_> = (0..64u32).map(|i| store.allocate(i)).collect();
    let mut slots: Vec<u32> = live.iter().map(|h| h.key().index).collect();
    slots.sort_unstable();
    slots.dedup();
    assert_eq!(slots.len(), 64);
}

fn device() -> Option<Arc<GraphicsDevice>> {
    create_headless_device()
        .ok()
        .map(GraphicsDevice::from_device)
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
        "默认视图只建一次（+0x70 缓存）"
    );
    assert_eq!(texture.cached_view_count(), 0, "默认视图不占缓存表");

    let view_desc = TextureViewDesc::new(&desc);
    let first = texture.view(Some(&view_desc)).unwrap();
    let again = gd.get_vk_image_view(&texture, &view_desc).unwrap();
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
    let all = mipped.view(Some(&TextureViewDesc::new(&desc2))).unwrap();
    let mut one = TextureViewDesc::new(&desc2);
    one.level_count = 1;
    let one_view = mipped.view(Some(&one)).unwrap();
    assert_ne!(all.raw(), one_view.raw());
    assert_eq!(mipped.cached_view_count(), 2);
    // 无效范围不入缓存。
    let mut bad = TextureViewDesc::new(&desc2);
    bad.level_count = 9;
    assert!(mipped.view(Some(&bad)).is_err());
    assert_eq!(mipped.cached_view_count(), 2);
}
