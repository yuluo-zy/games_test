//! 命令缓冲销毁桶语义：重复 seq / 缺桶失败、缺桶回退桶、SequentialDisposal 乱序窗口。
use rhapsody::raw::disposal::{CommandBufferDisposalBuckets, DisposalBucket, SequentialDisposal};
use rhapsody::raw::managed::tracking::Failure;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn panic_message(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let err = std::panic::catch_unwind(f).unwrap_err();
    err.downcast_ref::<String>()
        .cloned()
        .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default()
}

#[test]
fn try_variants_keep_failure_contract() {
    let buckets = CommandBufferDisposalBuckets::default();
    assert_eq!(buckets.begin_command_bucket(1), Ok(()));
    assert_eq!(buckets.begin_command_bucket(1), Err(Failure::DuplicateBucket));
    assert!(matches!(buckets.try_remove(2), Err(Failure::MissingBucket)));
    assert_eq!(buckets.drain_completed_sequence(2), Err(Failure::MissingBucket));
}

#[test]
fn dispose_cb_resource_hits_bucket_or_falls_back() {
    struct Count(Arc<AtomicUsize>);
    impl Drop for Count {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let dropped = Arc::new(AtomicUsize::new(0));
    let buckets = CommandBufferDisposalBuckets::default();
    buckets.begin(1);
    let mut fallback = DisposalBucket::new();
    buckets.dispose_cb_resource(1, Count(dropped.clone()), &mut fallback);
    buckets.dispose_cb_resource(2, Count(dropped.clone()), &mut fallback); // 缺桶
    assert_eq!((buckets.pending(), fallback.len()), (1, 1));
    assert_eq!(dropped.load(Ordering::SeqCst), 0);
    assert_eq!(CommandBufferDisposalBuckets::empty_bucket(fallback), 1);
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    assert_eq!(buckets.remove(1).dispose(), 1);
    assert_eq!(dropped.load(Ordering::SeqCst), 2);
}

#[test]
fn sequential_disposal_window_retires_in_order() {
    let mut order = vec![];
    let mut sequential = SequentialDisposal::new();
    sequential.dispose_in_order(3, |s| order.push(s));
    sequential.dispose_in_order(5, |s| order.push(s));
    assert!(order.is_empty());
    assert_eq!(sequential.waiting(), 2);
    sequential.dispose_in_order(1, |s| order.push(s));
    assert_eq!(order, [1]);
    sequential.dispose_in_order(2, |s| order.push(s));
    assert_eq!(order, [1, 2, 3]);
    sequential.dispose_in_order(4, |s| order.push(s));
    assert_eq!(order, [1, 2, 3, 4, 5]);
    assert_eq!(sequential.last_retired(), 5);
    // seq <= last_retired 断言；窗口内重复同一 seq 也 panic。
    let msg = panic_message(std::panic::AssertUnwindSafe(|| sequential.dispose_in_order(5, |_| {})));
    assert_eq!(msg, "assertion failed: seq > self.last_retired");
    let mut sequential = SequentialDisposal::new();
    sequential.dispose_in_order(4, |_| {});
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sequential.dispose_in_order(4, |_| {}))).is_err());
}
