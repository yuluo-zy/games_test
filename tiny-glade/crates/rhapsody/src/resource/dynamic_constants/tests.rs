use super::*;
struct CpuPage {
    cell: Arc<crate::tracking::SyncCell>,
    bytes: u64,
}
impl ConstantPage for CpuPage {
    fn use_cell(&self) -> &Arc<crate::tracking::SyncCell> {
        &self.cell
    }
    fn backing_bytes(&self) -> u64 {
        self.bytes
    }
}
fn cpu_page() -> Result<CpuPage> {
    Ok(CpuPage {
        cell: Arc::new(crate::tracking::SyncCell::new()),
        bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES,
    })
}
fn config(pages: usize) -> DynamicConstantsConfig {
    DynamicConstantsConfig {
        max_pages: pages,
        max_backing_bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES * pages as u64,
        max_idle_pages: pages,
    }
}
#[test]
fn frame_banks_learn_capacity_early_and_reuse_only_unreferenced_pages() {
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut ring = PageRing::new(config(2), &budget).unwrap();
    let (first, _) = ring.allocate(16, None, 16, cpu_page).unwrap();
    let first_pointer = Arc::as_ptr(&first);
    ring.begin_frame(2);
    let (second, _) = ring.allocate(16, None, 16, cpu_page).unwrap();
    assert_ne!(first_pointer, Arc::as_ptr(&second));
    ring.begin_frame(2);
    assert!(matches!(ring.allocate(16, None, 16, cpu_page), Err(Error::Timeout)),
        "another bank's free space cannot overwrite a retained frame");
    drop(first);
    let (again, offset) = ring.allocate(16, None, 16, cpu_page).unwrap();
    assert_eq!(Arc::as_ptr(&again), first_pointer);
    assert_eq!(offset, 0);
    assert_eq!(ring.stats().pages, 2);
}

#[test]
fn production_ring_requires_every_queue_and_cpu_or_weak_owner_before_reset() {
    use crate::sync::{QueueId, SyncPoint};
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut ring = PageRing::new(config(1), &budget).unwrap();
    let (first, _) = ring
        .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
        .unwrap();
    let pointer = Arc::as_ptr(&first);
    let cell = first.cell.clone();
    let queue0 = QueueId::new(0).unwrap();
    let queue1 = QueueId::new(1).unwrap();
    cell.insert(SyncPoint {
        queue: queue0,
        value: 7,
    });
    cell.insert(SyncPoint {
        queue: queue1,
        value: 9,
    });
    ring.completed.raise(queue0, 7);
    ring.completed.raise(queue1, 9);
    assert!(
        matches!(ring.allocate(1, None, 16, cpu_page), Err(Error::Timeout)),
        "an unsubmitted CPU lease still owns the page"
    );
    let weak = Arc::downgrade(&first);
    drop(first);
    assert!(
        matches!(ring.allocate(1, None, 16, cpu_page), Err(Error::Timeout)),
        "Weak cannot resurrect an old allocation after reset"
    );
    drop(weak);
    ring.completed = Default::default();
    ring.completed.raise(queue0, 7);
    assert!(
        matches!(ring.allocate(1, None, 16, cpu_page), Err(Error::Timeout)),
        "the independent queue remains pending"
    );
    ring.completed.raise(queue1, 9);
    for _ in 0..1000 {
        let (page, offset) = ring
            .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
            .unwrap();
        assert_eq!(offset, 0);
        assert_eq!(Arc::as_ptr(&page), pointer);
        assert_eq!(
            page.cell.snapshot().get(queue1),
            9,
            "reuse does not erase real completion history"
        );
        drop(page);
    }
    assert_eq!(ring.stats().page_creations, 1);
    assert_eq!(ring.stats().page_reuses, 1000);
}
#[test]
fn production_ring_page_used_by_a_submission_is_sealed_even_after_completion() {
    use crate::sync::{QueueId, SyncPoint};
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut ring = PageRing::new(config(2), &budget).unwrap();
    let queue = QueueId::new(0).unwrap();
    let (first, first_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
    let (second, second_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
    assert_eq!(first_offset, 0);
    assert_eq!(second_offset, 64, "unused page keeps appending");
    assert!(Arc::ptr_eq(&first, &second));
    // A submission uses the page and the GPU finishes before the next allocation.
    first.cell.insert(SyncPoint { queue, value: 1 });
    ring.completed.raise(queue, 1);
    let cell = first.cell.clone();
    drop((first, second));
    // Without sealing, completion timing would decide between offset 128 here and a restart.
    let (third, third_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
    assert_eq!(third_offset, 0, "a used page restarts instead of appending");
    assert!(Arc::ptr_eq(&third.cell, &cell), "the idle used page is reset in place");
    assert_eq!(ring.stats().page_reuses, 1);
    // The reset page is appendable again until its next use.
    let (_, fourth_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
    assert_eq!(fourth_offset, 64);
    // Still in flight: a second page is created at offset 0 instead.
    cell.insert(SyncPoint { queue, value: 2 });
    let (fifth, fifth_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
    assert_eq!(fifth_offset, 0);
    assert!(!Arc::ptr_eq(&fifth.cell, &cell));
    assert_eq!(ring.stats().page_creations, 2);
}
#[test]
fn production_ring_caps_actual_bytes_and_preserves_oom_or_device_loss() {
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut ring = PageRing::<CpuPage>::new(config(1), &budget).unwrap();
    for error in [
        Error::OutOfHostMemory,
        Error::OutOfDeviceMemory,
        Error::DeviceLost,
    ] {
        let kind = error.kind();
        assert_eq!(
            ring.allocate(1, None, 16, || Err(error))
                .err()
                .unwrap()
                .kind(),
            kind
        );
        assert_eq!(ring.stats().pages, 0);
    }
    assert!(matches!(
        ring.allocate(1, None, 16, || Ok(CpuPage {
            cell: Arc::new(crate::tracking::SyncCell::new()),
            bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES + 1
        })),
        Err(Error::Timeout)
    ));
    assert_eq!(ring.stats().pages, 0);
    assert_eq!(ring.stats().backing_bytes, 0);
    assert!(ring.allocate(1, None, 16, cpu_page).is_ok());
}
#[test]
fn production_ring_pressure_trims_only_idle_pages_and_keeps_metadata_accounted() {
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut ring = PageRing::new(config(2), &budget).unwrap();
    let metadata = ring.stats().metadata_bytes as u64;
    let (first, _) = ring
        .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
        .unwrap();
    let (second, _) = ring
        .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
        .unwrap();
    assert_eq!(ring.trim_idle(0, false), 0);
    let cell = first.cell.clone();
    cell.insert(crate::sync::SyncPoint {
        queue: crate::sync::QueueId::new(0).unwrap(),
        value: 1,
    });
    drop(first);
    drop(second);
    assert_eq!(ring.trim_idle(0, false), 1);
    assert_eq!(
        ring.stats().backing_bytes,
        DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES
    );
    ring.completed
        .raise(crate::sync::QueueId::new(0).unwrap(), 1);
    assert_eq!(ring.trim_idle(0, false), 1);
    assert_eq!(ring.stats().backing_bytes, 0);
    assert_eq!(budget.cpu_memory_usage().live, metadata);
    drop(ring);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}

#[test]
fn constant_pages_use_real_concurrent_sharing_only_for_distinct_configured_families() {
    use crate::device::PhysicalQueue;
    let aliased = [
        PhysicalQueue {
            family: 2,
            index: 0,
        },
        PhysicalQueue {
            family: 2,
            index: 1,
        },
    ];
    assert_eq!(
        constant_page_sharing(&aliased).unwrap(),
        crate::tracking::ResourceSharing::Exclusive
    );
    let distinct = [
        aliased[0],
        PhysicalQueue {
            family: 4,
            index: 0,
        },
        aliased[1],
    ];
    let sharing = constant_page_sharing(&distinct).unwrap();
    assert!(sharing.is_concurrent());
    assert!(sharing.allows_family(2));
    assert!(sharing.allows_family(4));
    assert!(!sharing.allows_family(3));
}

#[test]
fn constant_pages_support_uniform_and_bindless_storage_bindings() {
    assert!(DYNAMIC_CONSTANTS_USAGE.contains(vk::BufferUsageFlags::UNIFORM_BUFFER));
    assert!(DYNAMIC_CONSTANTS_USAGE.contains(vk::BufferUsageFlags::STORAGE_BUFFER));
    assert_eq!(constant_page_usage(true), DYNAMIC_CONSTANTS_USAGE);
    let no_address = constant_page_usage(false);
    assert!(
        no_address.contains(
            vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER
        )
    );
    assert!(!no_address.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS));
}

#[test]
fn invalid_allocation_requests_fail_before_page_creation() {
    for (size, alignment) in [
        (DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES + 1, None),
        (1, Some(0)),
        (1, Some(3)),
    ] {
        assert_eq!(
            allocation_offset(0, size, alignment, 256)
                .unwrap_err()
                .kind(),
            crate::error::ErrorKind::InvalidUsage,
        );
    }
    assert!(allocation_offset(u64::MAX, 1, None, 256).is_err());
}

#[test]
fn native_empty_chunks_keep_real_pages_and_the_aligned_cursor() {
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut ring = PageRing::new(config(1), &budget).unwrap();
    let (empty, offset) = ring.allocate(0, None, 16, cpu_page).unwrap();
    assert_eq!(offset, 0);
    assert_eq!(ring.pages[0].cursor, 0);
    let (payload, payload_offset) = ring.allocate(17, None, 16, cpu_page).unwrap();
    assert!(Arc::ptr_eq(&empty, &payload));
    assert_eq!(payload_offset, 0);
    let (aligned_empty, aligned_offset) = ring.allocate(0, Some(64), 16, cpu_page).unwrap();
    assert!(Arc::ptr_eq(&empty, &aligned_empty));
    assert_eq!(aligned_offset, 64);
    assert_eq!(ring.pages[0].cursor, 64);
    let (next, next_offset) = ring.allocate(1, None, 16, cpu_page).unwrap();
    assert!(Arc::ptr_eq(&empty, &next));
    assert_eq!(next_offset, 64);
    assert_eq!(ring.stats().page_creations, 1);
    // A retained empty chunk is still a genuine CPU owner of this page.
    drop(payload);
    drop(aligned_empty);
    drop(next);
    assert!(matches!(
        ring.allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page),
        Err(Error::Timeout)
    ));
    drop(empty);
    assert!(
        ring.allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
            .is_ok()
    );
    assert_eq!(
        allocation_offset(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, 0, None, 16).unwrap(),
        Some(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
    );
}

#[test]
fn page_boundary_and_device_alignment_determine_placement() {
    assert_eq!(allocation_offset(1, 16, None, 256).unwrap(), Some(256));
    assert_eq!(
        allocation_offset(0, DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 256).unwrap(),
        Some(0)
    );
    assert_eq!(
        allocation_offset(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES - 16, 32, None, 16).unwrap(),
        None
    );
}
