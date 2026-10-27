use super::*;
use crate::bindless::residency::MAX_RESIDENT_KEYS;
use crate::resource::buffer::Buffer;
use ash::vk::Handle;

#[test]
fn page_index_reuses_live_leases_without_retaining_retired_ranges_or_itself() {
    let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(
                !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                "{error}"
            );
            eprintln!("SKIP descriptor cache GPU initialization: {error}");
            return;
        }
    };
    let heap = BindlessHeap::new(
        device.clone(),
        BindlessConfig {
            sampled_images: 4,
            storage_images: 4,
            storage_buffers: 4,
            samplers: 4,
        },
    )
    .unwrap();
    heap.set_residency(true);
    let buffer = Arc::new(
        Buffer::new(
            device,
            &crate::resource::buffer::BufferDesc::new_cpu_to_gpu(
                8192 * 256,
                vk::BufferUsageFlags::STORAGE_BUFFER,
            ),
        )
        .unwrap(),
    );
    // Thousands of independent ranges of one still-live allocation exposed
    // the old cache's mistaken resource strong-count liveness test.
    for range in 0..8192u64 {
        let offset = range * 256;
        let key =
            parameters::ResourceKey::Buffer(buffer.raw_handle().as_raw(), offset, 4, false);
        let first = heap
            .resident_lease(key, || heap.storage_buffer(buffer.clone(), offset, 4))
            .unwrap();
        let second = heap
            .resident_lease(key, || panic!("live lease must be reused"))
            .unwrap();
        assert!(Arc::ptr_eq(&first, &second));
        drop(first);
        assert_eq!(heap.stats().live, 1);
        drop(second);
        assert_eq!(heap.stats().live, 0);
        assert!(heap.resident.lock().as_ref().unwrap().leases.len() <= MAX_RESIDENT_KEYS);
    }
    let weak = Arc::downgrade(&heap);
    drop(heap);
    assert!(
        weak.upgrade().is_none(),
        "an index cannot form a heap/lease ownership cycle"
    );
}

#[test]
fn slots_reuse_only_after_release_and_stale_generations_never_match() {
    let mut slots = Slots::new([1; 4]).unwrap();
    let first = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
    assert!(matches!(slots.allocate(DescriptorKind::StorageBuffer), Err(Error::Exhausted(_))));
    slots.release(first);
    let second = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
    assert_eq!(first.index, second.index);
    assert_ne!(first.generation, second.generation);
    assert!(!slots.valid(first));
    assert!(slots.valid(second));
}
#[test]
fn table_statistics_distinguish_live_free_and_exhausted_kinds() {
    let mut slots = Slots::new([2, 1, 2, 1]).unwrap();
    let first = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
    let second = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
    assert!(slots.allocate(DescriptorKind::StorageBuffer).is_err());
    assert_eq!(slots.table_stats(DescriptorKind::StorageBuffer).peak_live, 2);
    slots.release(first);
    slots.release(second);
    slots.entries[DescriptorKind::Sampler as usize][0].generation = u64::MAX;
    assert!(slots.allocate(DescriptorKind::Sampler).is_err());
    for kind in DescriptorKind::ALL {
        let stats = slots.table_stats(kind);
        assert_eq!(stats.capacity, stats.live + stats.free + stats.exhausted);
    }
    assert_eq!(slots.table_stats(DescriptorKind::StorageBuffer).free, 2);
}

#[test]
fn sampled_occupancy_uses_current_slots_and_preserves_exhausted_capacity() {
    let mut a = Slots::new([2, 1, 4, 2]).unwrap();
    let historical: Vec<_> = (0..4).map(|_| a.allocate(DescriptorKind::StorageBuffer).unwrap()).collect();
    for index in historical { a.release(index); }
    let _image = a.allocate(DescriptorKind::SampledImage).unwrap();
    let mut b = Slots::new([1; 4]).unwrap();
    b.entries[DescriptorKind::Sampler as usize][0].generation = u64::MAX;
    assert!(b.allocate(DescriptorKind::Sampler).is_err());
    let _buffer = b.allocate(DescriptorKind::StorageBuffer).unwrap();
    let mut sampled = BindlessOccupancy::default();
    sampled.include(a.statistics()); sampled.include(b.statistics());
    assert_eq!(a.statistics().peak_live, 4);
    assert_eq!(sampled.live, 2);
    assert_eq!(sampled.capacity, 13);
    assert_eq!(sampled.free, 10);
    for table in sampled.tables { assert_eq!(table.capacity, table.live + table.free + table.exhausted); }
    assert_eq!(sampled.table(DescriptorKind::Sampler).exhausted, 1);
}
