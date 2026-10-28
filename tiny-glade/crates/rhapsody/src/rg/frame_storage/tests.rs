use super::*;
fn plan(resources: usize, passes: usize) -> CompiledPlan {
    let mut graph = graph::RenderGraph::new();
    let resources = (0..resources)
        .map(|index| {
            graph
                .import_unowned(
                    format!("resource{index}"),
                    graph::Descriptor::Buffer(crate::rg::resource_registry::BufferDesc {
                        word_at_00: 64,
                        word_at_08: 0,
                        bytes_at_0c_through_0e: [0; 3],
                    }),
                    index as u64 + 1,
                    0x25,
                )
                .unwrap()
        })
        .collect::<Vec<_>>();
    for index in 0..passes {
        let mut pass = graph.add_pass(format!("pass{index}"));
        pass.side_effect();
        for resource in &resources {
            pass.add_resource_access(*resource, 0x25, 0).unwrap();
        }
        pass.render_with_result_context(|_| Ok(()));
    }
    if let Some(resource) = resources.first() {
        graph.export(*resource, 0x25).unwrap();
    }
    let (declaration, _, _, declaration_error) = graph.build().into_parts();
    assert!(declaration_error.is_none());
    super::super::plan::compile_definition(
        super::super::plan::GraphDefinition::from_declaration(
            &declaration,
            &super::super::vulkan::AccessTable::original(),
            Default::default(),
        )
        .unwrap(),
    )
    .unwrap()
}
fn config() -> FrameStorageConfig {
    FrameStorageConfig {
        max_frames: 2,
        max_total_bytes: 256 << 10,
        max_cached_bytes: 128 << 10,
    }
}
#[test]
fn real_tables_reuse_beyond_inline_sizes_and_return_only_after_cpu_lease_drop() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = FrameStoragePool::new(config(), budget.clone()).unwrap();
    let plan = plan(32, 12);
    assert_eq!(plan.info.pass_access_ranges.len(), 12);
    assert_eq!(plan.info.lifetimes.len(), 32);
    let mut first = pool.checkout(&plan).unwrap();
    let tables = first.take_tables();
    let pointers = (
        tables.registry.resources.as_ptr(),
        tables.physical_mapping.as_ptr(),
    );
    let actual = tables.capacity_bytes();
    assert!(actual > 0);
    first.return_tables(tables);
    let second = pool.checkout(&plan).unwrap();
    assert!(matches!(pool.checkout(&plan), Err(Error::Timeout)));
    assert_eq!(pool.trim_idle(0), 0);
    assert_eq!(pool.stats().checked_out, 2);
    drop(first);
    for _ in 0..1000 {
        let mut reused = pool.checkout(&plan).unwrap();
        let tables = reused.take_tables();
        assert_eq!(
            (
                tables.registry.resources.as_ptr(),
                tables.physical_mapping.as_ptr()
            ),
            pointers
        );
        reused.return_tables(tables);
    }
    assert_eq!(pool.stats().capacity_growths, 2);
    assert_eq!(pool.stats().checked_out, 1);
    let metadata = pool.stats().metadata_bytes;
    assert_eq!(budget.cpu_memory_usage().live, (actual + metadata) as u64);
    assert_eq!(budget.cpu_memory_usage().cache, actual as u64);
    assert_eq!(pool.trim_idle(0), actual);
    assert_eq!(budget.cpu_memory_usage().cache, 0);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().live, (actual + metadata) as u64);
    drop(second);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
    assert!(budget.report().allocations.is_empty());
}
#[test]
fn quota_and_local_capacity_failures_are_unpublished_and_preserve_categories() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = FrameStoragePool::new(config(), budget.clone()).unwrap();
    let metadata = pool.stats().metadata_bytes as u64;
    budget.set_total_budget(metadata + 1).unwrap();
    assert!(matches!(
        pool.checkout(&plan(32, 12)),
        Err(Error::OutOfHostMemory)
    ));
    assert_eq!(pool.stats().checked_out, 0);
    assert_eq!(budget.cpu_memory_usage().total(), metadata);
    drop(pool);
    assert_eq!(budget.cpu_memory_usage().total(), 0);

    let pool = FrameStoragePool::new(
        FrameStorageConfig {
            max_frames: 1,
            max_total_bytes: 4096,
            max_cached_bytes: 4096,
        },
        Arc::new(AllocatorStats::default()),
    )
    .unwrap();
    assert!(matches!(pool.checkout(&plan(128, 16)), Err(Error::Timeout)));
    assert_eq!(pool.stats().checked_out, 0);
    assert_eq!(pool.stats().capacity_stalls, 1);
}
#[test]
fn shrinking_tables_and_idle_eviction_credit_only_released_capacity() {
    let budget = Arc::new(AllocatorStats::default());
    let pool = FrameStoragePool::new(config(), budget.clone()).unwrap();
    let mut lease = pool.checkout(&plan(16, 9)).unwrap();
    let mut tables = lease.take_tables();
    let original = tables.capacity_bytes();
    // Release the actual allocation before reducing its capacity ticket.
    drop(std::mem::take(&mut tables.registry.backing_resources));
    let retained = tables.capacity_bytes();
    lease.return_tables(tables);
    assert!(retained < original);
    assert_eq!(pool.stats().live_bytes, retained);
    drop(lease);
    assert_eq!(pool.stats().cached_bytes, retained);
    assert_eq!(pool.trim_idle(0), retained);
    assert_eq!(
        budget.cpu_memory_usage().total(),
        pool.stats().metadata_bytes as u64
    );
}
