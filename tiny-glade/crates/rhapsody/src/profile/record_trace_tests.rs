use super::*;

#[test]
fn off_scope_does_not_activate_or_touch_numbers() {
    assert_eq!(RECORD_ACTIVE.with(std::cell::Cell::get), 0);
    let before = RECORD_NUMBERS.with(std::cell::Cell::get);
    let mut scope = RecordScope::begin(false, 1, 2, 3, "off", false);
    let span = RecordSpan::new(RecordPhase::Finish);
    assert!(span.active.is_none());
    record_count(RecordCount::GroupAttempts, 7);
    let tally = RecordTally::new([
        RecordCount::CoverageAccesses,
        RecordCount::BindingChecks,
    ]);
    tally.add(0);
    tally.add(1);
    assert_eq!(tally.counts[0].get(), 0);
    assert_eq!(tally.counts[1].get(), 0);
    drop(tally);
    scope.complete(false);
    drop(span);
    drop(scope);
    assert_eq!(RECORD_NUMBERS.with(std::cell::Cell::get), before);
    assert_eq!(RECORD_ACTIVE.with(std::cell::Cell::get), 0);
}
#[cfg(feature = "cpu-probe")]
#[test]
fn native_workload_trace_off_is_empty_and_failure_retains_invalid_raw_prefix() {
    let workload = super::CommandWorkload { direct_dispatch_api_calls: 3, ..Default::default() };
    let counter = std::cell::Cell::new(workload);
    let mut off = RecordScope::begin(false, 1, 2, 3, "off", false);
    off.complete_with_workload(true, &counter);
    assert!(off.active.is_none());
    drop(off);
    let mut recorded = RecordScope::begin(true, 1, 2, 3, "duplicate name", false);
    recorded.complete_with_workload(true, &counter);
    assert_eq!(recorded.active.as_ref().unwrap().native_workload, Some(workload));
    drop(recorded);
    let mut failed = RecordScope::begin(true, 1, 3, 4, "duplicate name", false);
    failed.complete_with_workload(false, &counter);
    let failed = failed.active.as_ref().unwrap();
    assert_eq!(failed.callback_ok, Some(false));
    assert_eq!(failed.native_workload.unwrap().direct_dispatch_api_calls, 3);
    assert!(!failed.native_workload.unwrap().valid);
}
#[test]
fn nested_scope_restores_parent_and_discards_foreign_spans() {
    let parent = RecordScope::begin(true, 1, 2, 3, "parent", false);
    record_count(RecordCount::GroupAttempts, 2);
    let child = RecordScope::begin(true, 4, 5, 6, "child", false);
    let foreign = RecordSpan::new(RecordPhase::Finish);
    record_count(RecordCount::GroupAttempts, 7);
    drop(child);
    let before = RECORD_NUMBERS.with(std::cell::Cell::get);
    drop(foreign);
    assert_eq!(RECORD_NUMBERS.with(std::cell::Cell::get), before);
    assert_eq!(before.counts[RecordCount::GroupAttempts as usize], 2);
    assert_eq!(
        RECORD_ACTIVE.with(std::cell::Cell::get),
        parent.active.as_ref().unwrap().id
    );
    drop(parent);
    assert_eq!(RECORD_ACTIVE.with(std::cell::Cell::get), 0);
}
#[test]
fn false_and_unwind_restore_tls_before_next_callback() {
    let mut failed = RecordScope::begin(true, 1, 2, 3, "false", false);
    failed.complete(false);
    assert_eq!(failed.active.as_ref().unwrap().callback_ok, Some(false));
    drop(failed);
    let result = std::panic::catch_unwind(|| {
        let _scope = RecordScope::begin(true, 1, 2, 3, "panic", false);
        let _span = RecordSpan::new(RecordPhase::Finish);
        panic!("test callback panic");
    });
    assert!(result.is_err());
    assert_eq!(RECORD_ACTIVE.with(std::cell::Cell::get), 0);
    assert_eq!(
        RECORD_NUMBERS.with(std::cell::Cell::get),
        RecordNumbers::EMPTY
    );
}
#[test]
fn parallel_workers_keep_distinct_scope_and_tally() {
    let workers: Vec<_> = [3usize, 7]
        .into_iter()
        .map(|count| {
            std::thread::spawn(move || {
                let scope = RecordScope::begin(true, 1, count, count as u64, "parallel", false);
                let tally = RecordTally::new([
                    RecordCount::CoverageAccesses,
                    RecordCount::BindingChecks,
                ]);
                for _ in 0..count {
                    tally.add(0);
                    tally.add(1);
                }
                drop(tally);
                let id = scope.active.as_ref().unwrap().id;
                let numbers = RECORD_NUMBERS.with(std::cell::Cell::get);
                drop(scope);
                assert_eq!(RECORD_ACTIVE.with(std::cell::Cell::get), 0);
                (id, numbers)
            })
        })
        .collect();
    let results: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_ne!(results[0].0, results[1].0);
    for ((_, numbers), expected) in results.into_iter().zip([3, 7]) {
        assert_eq!(
            numbers.counts[RecordCount::CoverageAccesses as usize],
            expected
        );
        assert_eq!(
            numbers.counts[RecordCount::BindingChecks as usize],
            expected
        );
    }
}
#[test]
fn cloned_coverage_iterators_share_local_counts_and_remain_lazy() {
    use crate::rg::{graph::Descriptor, resource::AccessRange};
    use crate::rg::resource_registry::BufferDesc;
    let scope = RecordScope::begin(true, 1, 2, 3, "cloned coverage", false);
    let descriptor = Descriptor::Buffer(BufferDesc {
        word_at_00: 8,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    });
    let writes = [
        (AccessRange::Buffer { offset: 0, size: 4 }, 1u8),
        (AccessRange::Buffer { offset: 4, size: 4 }, 2u8),
    ];
    let semantics = [0u8, 1, 2];
    let tally = RecordTally::new([
        RecordCount::CoverageAccesses,
        RecordCount::BindingChecks,
    ]);
    let ranges = writes
        .iter()
        .filter(|(_, code)| {
            tally.add(0);
            semantics
                .iter()
                .find(|expected| {
                    tally.add(1);
                    **expected == *code
                })
                .is_some()
        })
        .map(|(range, _)| *range);
    assert!(
        crate::rg::resource_contracts::covers_iter(
            ranges,
            AccessRange::Buffer { offset: 0, size: 8 },
            descriptor,
        )
        .unwrap()
    );
    // 首轮 contains 扫描2项，区间并集两轮各再扫描2项；同两Cell累加所有clone。
    assert_eq!(tally.counts[0].get(), 6);
    assert_eq!(tally.counts[1].get(), 15);
    drop(tally);
    let short = RecordTally::new([
        RecordCount::CoverageAccesses,
        RecordCount::BindingChecks,
    ]);
    let ranges = writes
        .iter()
        .filter(|(_, code)| {
            short.add(0);
            semantics
                .iter()
                .find(|expected| {
                    short.add(1);
                    **expected == *code
                })
                .is_some()
        })
        .map(|(range, _)| *range);
    assert!(
        crate::rg::resource_contracts::covers_iter(
            ranges,
            AccessRange::Buffer { offset: 0, size: 4 },
            descriptor,
        )
        .unwrap()
    );
    assert_eq!(
        short.counts[0].get(),
        1,
        "首项包含即可返回，不提前消费剩余范围"
    );
    assert_eq!(short.counts[1].get(), 2);
    drop(short);
    let numbers = RECORD_NUMBERS.with(std::cell::Cell::get);
    assert_eq!(numbers.counts[RecordCount::CoverageAccesses as usize], 7);
    assert_eq!(numbers.counts[RecordCount::BindingChecks as usize], 17);
    drop(scope);
}
