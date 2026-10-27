use super::*;

#[test]
fn dropped_graph_lease_is_collected_once_and_live_graphs_apply_backpressure() {
    let budget = Arc::new(AllocatorStats::default());
    let mut imports =
        FrameImports::new(FrameImportConfig { max_imports: 2 }, budget.clone()).unwrap();
    let graph = GraphId::next();
    let first = imports.reserve().unwrap();
    let first_graph_owner = first.marker();
    let first_handle = ResourceHandle::scoped(graph, 3);
    imports
        .insert(PhysicalIdentity::Buffer(17), 41, first_handle, first)
        .unwrap();
    assert_eq!(
        imports.find(graph, PhysicalIdentity::Buffer(17)),
        Some(first_handle)
    );
    assert_eq!(
        imports.find(GraphId::next(), PhysicalIdentity::Buffer(17)),
        None
    );
    let second = imports.reserve().unwrap();
    let second_graph_owner = second.marker();
    imports
        .insert(
            PhysicalIdentity::Buffer(18),
            42,
            ResourceHandle::scoped(graph, 4),
            second,
        )
        .unwrap();
    assert!(matches!(imports.reserve(), Err(Error::Exhausted(_))));
    let charged = budget.cpu_memory_usage().total();
    let mut removed = Vec::new();
    imports.collect(None, |token| removed.push(token));
    assert!(removed.is_empty());
    drop(first_graph_owner);
    assert_eq!(budget.cpu_memory_usage().total(), charged);
    imports.collect(None, |token| removed.push(token));
    assert_eq!(removed, [41]);
    assert!(budget.cpu_memory_usage().total() < charged);
    assert_eq!(imports.find(graph, PhysicalIdentity::Buffer(17)), None);
    drop(second_graph_owner);
    imports.collect(None, |token| removed.push(token));
    imports.collect(None, |token| removed.push(token));
    assert_eq!(removed, [41, 42]);
    drop(imports);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}

#[test]
fn physical_identity_keeps_resource_kinds_distinct() {
    let budget = Arc::new(AllocatorStats::default());
    let mut imports = FrameImports::new(FrameImportConfig { max_imports: 2 }, budget).unwrap();
    let graph = GraphId::next();
    let buffer = imports.reserve().unwrap();
    let buffer_owner = buffer.marker();
    imports
        .insert(
            PhysicalIdentity::Buffer(17),
            1,
            ResourceHandle::scoped(graph, 0),
            buffer,
        )
        .unwrap();
    assert_eq!(imports.find(graph, PhysicalIdentity::Image(17)), None);
    let image = imports.reserve().unwrap();
    let image_owner = image.marker();
    imports
        .insert(
            PhysicalIdentity::Image(17),
            2,
            ResourceHandle::scoped(graph, 1),
            image,
        )
        .unwrap();
    assert_eq!(
        imports
            .find(graph, PhysicalIdentity::Buffer(17))
            .unwrap()
            .raw(),
        0
    );
    assert_eq!(
        imports
            .find(graph, PhysicalIdentity::Image(17))
            .unwrap()
            .raw(),
        1
    );
    drop((buffer_owner, image_owner));
    imports.collect(None, |_| {});
    assert_eq!(imports.len(), 0);
}

#[test]
fn recorded_graph_releases_only_its_entries_and_keeps_callback_memory_charged() {
    let budget = Arc::new(AllocatorStats::default());
    let mut imports =
        FrameImports::new(FrameImportConfig { max_imports: 2 }, budget.clone()).unwrap();
    let recorded = GraphId::next();
    let building = GraphId::next();
    let first = imports.reserve().unwrap();
    let pending_owner = first.marker();
    imports.insert(PhysicalIdentity::Buffer(1), 11,
        ResourceHandle::scoped(recorded, 0), first).unwrap();
    let second = imports.reserve().unwrap();
    let building_owner = second.marker();
    imports.insert(PhysicalIdentity::Buffer(1), 12,
        ResourceHandle::scoped(building, 0), second).unwrap();
    let mut removed = Vec::new();
    imports.collect(Some((recorded, &[])), |token| removed.push(token));
    assert_eq!(removed, [11]);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports.find(recorded, PhysicalIdentity::Buffer(1)), None);
    assert!(imports.find(building, PhysicalIdentity::Buffer(1)).is_some());
    assert!(imports.reserve().is_ok());
    drop(imports);
    drop(building_owner);
    assert_ne!(budget.cpu_memory_usage().total(), 0);
    drop(pending_owner);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}

#[test]
fn failed_admission_never_registers_an_import() {
    let budget = Arc::new(AllocatorStats::default());
    let mut imports =
        FrameImports::new(FrameImportConfig { max_imports: 1 }, budget.clone()).unwrap();
    budget.set_total_budget(1).unwrap();
    assert!(matches!(imports.reserve(), Err(Error::OutOfHostMemory)));
    assert_eq!(imports.len(), 0);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}

#[test]
fn an_unsubmitted_graph_outlives_its_session_without_returning_its_lease_credit() {
    let budget = Arc::new(AllocatorStats::default());
    let mut imports =
        FrameImports::new(FrameImportConfig { max_imports: 1 }, budget.clone()).unwrap();
    let lease = imports.reserve().unwrap();
    let graph_owner = lease.marker();
    imports
        .insert(
            PhysicalIdentity::Buffer(1),
            9,
            ResourceHandle::scoped(GraphId::next(), 0),
            lease,
        )
        .unwrap();
    drop(imports);
    assert_ne!(budget.cpu_memory_usage().total(), 0);
    drop(graph_owner);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
