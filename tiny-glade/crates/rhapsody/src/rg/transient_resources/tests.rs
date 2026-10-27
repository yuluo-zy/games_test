use super::*;

#[test]
fn cached_unaliased_edges_reuse_accounted_storage_and_trim() {
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut prepared = PreparedTransients::default();
    let edges = [(0, 1), (1, 2), (2, 3)];
    prepared.prepare_unaliased(&edges, &budget).unwrap();
    let address = prepared.plan.dependency_edges.as_ptr();
    let bytes = (prepared.plan.dependency_edges.capacity()
        * std::mem::size_of::<(usize, usize)>()) as u64;
    budget.set_total_budget(bytes).unwrap();
    for _ in 0..1000 {
        prepared.prepare_unaliased(&edges, &budget).unwrap();
        assert_eq!(prepared.plan.dependency_edges.as_ptr(), address);
        assert_eq!(prepared.plan.dependency_edges, edges);
        assert_eq!(budget.cpu_memory_usage().cache, bytes);
    }
    prepared.trim_unaliased();
    assert_eq!(prepared.plan.dependency_edges.capacity(), 0);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}

#[test]
fn unaliased_edge_growth_oom_preserves_the_existing_credit_and_plan() {
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut prepared = PreparedTransients::default();
    prepared.prepare_unaliased(&[(0, 1)], &budget).unwrap();
    let address = prepared.plan.dependency_edges.as_ptr();
    let bytes = budget.cpu_memory_usage().total();
    budget.set_total_budget(bytes).unwrap();
    assert!(matches!(
        prepared.prepare_unaliased(&[(0, 1), (1, 2)], &budget),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(prepared.plan.dependency_edges, [(0, 1)]);
    assert_eq!(prepared.plan.dependency_edges.as_ptr(), address);
    assert_eq!(budget.cpu_memory_usage().total(), bytes);
    drop(prepared);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}
