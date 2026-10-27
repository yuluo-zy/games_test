use super::*;
fn snapshot(resources: usize) -> graph::Snapshot {
    let shared = Arc::new(graph::SnapshotStatic {
        pass_order: Vec::new(),
        pass_ranges: Vec::new(),
        accesses: Vec::new(),
        lifetimes: Vec::new(),
        image_usage: Vec::new(),
        buffer_usage: Vec::new(),
        lifetime_events: Vec::new(),
        batches: Vec::new(),
        imports: Vec::new(),
        exports: Vec::new(),
    });
    graph::Snapshot {
        shared,
        physical_mapping: (0..resources)
            .map(|logical| graph::PhysicalMapping {
                logical: logical as u32,
                physical: logical as u16,
                reused: false,
                initial_access: 0x26,
                payload_tag: 0,
                token: Some(11 + logical as u64),
            })
            .collect(),
        pool_hits: vec![],
    }
}
fn config() -> ReceiptStorageConfig {
    ReceiptStorageConfig {
        max_receipts: 2,
        max_total_bytes: 256 << 10,
        max_cached_bytes: 128 << 10,
    }
}
#[test]
fn cancellation_returns_storage_and_statistics_do_not_reclassify_weakly_pinned_slots() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = ReceiptStorage::new(ReceiptStorageConfig { max_receipts: 1, ..config() }, budget.clone()).unwrap();
    let source = snapshot(2);
    let prepared = pool.prepare(&source, 3).unwrap();
    assert_eq!(pool.statistics().borrowed, 1);
    drop(prepared);
    assert_eq!(pool.statistics().borrowed, 0);
    let prepared = pool.prepare(&source, 3).unwrap();
    let published = prepared.publish(&[]);
    let weak = Arc::downgrade(&published);
    drop(published);
    assert!(matches!(pool.prepare(&source, 3), Err(Error::Timeout)));
    assert_eq!(pool.trim_idle(0), 0);
    drop(weak);
    let usage = budget.cpu_memory_usage();
    assert_eq!(pool.statistics().borrowed, 0);
    assert!(pool.statistics().cached_bytes > 0);
    assert_eq!(budget.cpu_memory_usage().live, usage.live);
    assert_eq!(budget.cpu_memory_usage().cache, usage.cache);
    assert!(pool.trim_idle(0) > 0);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
#[test]
fn retained_clones_bound_reuse_and_own_bytes_after_session_pool_drop() {
    let stats = Arc::new(AllocatorStats::default());
    let pool = ReceiptStorage::new(config(), stats.clone()).unwrap();
    let mut source = snapshot(12);
    let index = pool.prepare(&source, 16).unwrap();
    let point = crate::sync::SyncPoint {
        queue: crate::sync::QueueId::new(0).unwrap(),
        value: 1,
    };
    let first = index.publish(&[point]);
    let cloned = first.clone();
    source.physical_mapping[0].token = Some(22);
    let index = pool.prepare(&source, 16).unwrap();
    let second = index.publish(&[point]);
    assert_eq!(first.data.snapshot.physical_mapping[0].token, Some(11));
    assert_eq!(second.data.snapshot.physical_mapping[0].token, Some(22));
    assert!(matches!(pool.prepare(&source, 16), Err(Error::Timeout)));
    drop(first);
    assert!(matches!(pool.prepare(&source, 16), Err(Error::Timeout)));
    assert_eq!(pool.trim_idle(0), 0);
    let pointer = cloned.data.snapshot.physical_mapping.as_ptr();
    drop(cloned);
    for _ in 0..1000 {
        let index = pool.prepare(&source, 16).unwrap();
        let reused = index.publish(&[point]);
        assert_eq!(reused.data.snapshot.physical_mapping.as_ptr(), pointer);
        assert_eq!(reused.data.receipts, [point]);
    }
    assert_eq!(pool.statistics().capacity_growths, 2);
    assert!(pool.trim_idle(0) > 0);
    let retained_bytes = second.bytes as u64;
    drop(pool);
    assert_eq!(stats.cpu_memory_usage().total(), retained_bytes);
    drop(second);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn local_receipt_capacity_backpressure_and_host_budget_errors_are_distinct() {
    let stats = Arc::new(AllocatorStats::default());
    let pool = ReceiptStorage::new(config(), stats.clone()).unwrap();
    let source = snapshot(1);
    assert!(matches!(
        pool.prepare(&source, 100_000),
        Err(Error::Timeout)
    ));
    assert_eq!(pool.statistics().slots, 0);
    stats
        .set_total_budget(stats.cpu_memory_usage().total() + 1)
        .unwrap();
    assert!(matches!(
        pool.prepare(&source, 1),
        Err(Error::OutOfHostMemory)
    ));
    stats.set_total_budget(0).unwrap();
    let index = pool.prepare(&source, 1).unwrap();
    drop(index.publish(&[]));
    drop(pool);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
