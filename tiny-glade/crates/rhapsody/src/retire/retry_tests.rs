use super::*;
#[test]
fn busy_owner_preserves_payload_then_next_tick_destroys_it_once() {
    let queue = Retirement::new();
    queue.try_enqueue(SyncSet::EMPTY, 7).unwrap();
    assert_eq!(
        queue.collect_limited_retry(&CompletedValues::all(), usize::MAX, usize::MAX, |item| {
            Err(item)
        }),
        CollectionProgress {
            scanned: 1,
            destroyed: 0
        }
    );
    assert_eq!(queue.len(), 1);
    let mut observed = Vec::new();
    assert_eq!(queue.collect(&CompletedValues::all(), &mut observed), 1);
    assert_eq!(observed, [7]);
}
#[test]
fn panicking_destructor_does_not_strand_a_phantom_queue_entry() {
    let queue = Retirement::new();
    queue.try_enqueue(SyncSet::EMPTY, 7).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        queue.collect_limited_retry(&CompletedValues::all(), 1, 1, |_| {
            panic!("injected destructor panic")
        });
    }));
    assert!(result.is_err());
    assert!(queue.is_empty());
}
