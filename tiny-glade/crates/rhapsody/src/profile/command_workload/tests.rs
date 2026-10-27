use super::*;
#[test]
fn command_workload_overflow_is_invalid_and_never_wraps() {
    let mut total = CommandWorkload { direct_draw_api_calls: u64::MAX, ..Default::default() };
    total.merge(CommandWorkload { direct_draw_api_calls: 1, direct_dispatch_api_calls: 7, ..Default::default() });
    assert!(!total.valid);
    assert_eq!(total.direct_draw_api_calls, u64::MAX);
    assert_eq!(total.direct_dispatch_api_calls, 7);
    total.merge(CommandWorkload::default());
    assert!(!total.valid, "a later valid chunk must not repair invalid evidence");
}
#[test]
fn command_workload_counter_overflow_and_invalid_input_are_sticky() {
    let mut total = CommandWorkload { indexed_draw_api_calls: u64::MAX, ..Default::default() };
    total.merge(CommandWorkload { indexed_draw_api_calls: 1, ..Default::default() });
    assert_eq!(total.indexed_draw_api_calls, u64::MAX);
    assert!(!total.valid);
    let mut merged = CommandWorkload::default();
    merged.merge(total);
    assert!(!merged.valid);
    assert!(!CommandWorkload::invalid().valid);
}
#[test]
fn worker_local_workloads_join_in_stable_order_without_shared_counters() {
    let mut work = [2u64, 3, 5, 7, 11, 13, 17, 19, 23];
    let results = crate::rg::parallel::map_chunks(&mut work, |index, calls| {
        let cell = std::cell::Cell::new(CommandWorkload::default());
        cell.set(CommandWorkload { indexed_draw_api_calls: *calls, ..Default::default() });
        (index, cell.get())
    });
    let mut joined = CommandWorkload::default();
    for (expected, (actual, local)) in results.into_iter().enumerate() {
        assert_eq!(actual, expected);
        joined.merge(local);
    }
    assert!(joined.valid);
    assert_eq!(joined.indexed_draw_api_calls, 100);
    assert_eq!(joined.direct_draw_api_calls, 0);
}
