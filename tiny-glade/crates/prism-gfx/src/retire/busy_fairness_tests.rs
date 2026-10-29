use super::*;
#[test]
fn busy_zero_does_not_hide_later_ready_owners_in_the_same_queue() {
    let queue = Retirement::new();
    queue.try_enqueue(SyncSet::EMPTY, 0).unwrap();
    let after = SyncSet::from_point(crate::sync::SyncPoint {
        queue: QueueId::new(0).unwrap(),
        value: 1,
    });
    for item in 1..=3 {
        queue.try_enqueue(after, item).unwrap();
    }
    let mut observed = Vec::new();
    let progress = queue.collect_limited_retry(&CompletedValues::all(), 3, 3, |item| {
        if item == 0 {
            Err(item)
        } else {
            observed.push(item);
            Ok(())
        }
    });
    assert_eq!(
        progress,
        CollectionProgress {
            scanned: 3,
            destroyed: 2
        }
    );
    assert_eq!(observed, [1, 2]);
    assert_eq!(queue.len(), 2);
    queue.collect_limited_retry(&CompletedValues::all(), 3, 3, |item| {
        if item == 0 {
            Err(item)
        } else {
            observed.push(item);
            Ok(())
        }
    });
    assert_eq!(observed, [1, 2, 3]);
    assert_eq!(queue.len(), 1);
    queue.collect(&CompletedValues::all(), &mut observed);
    assert_eq!(observed, [1, 2, 3, 0]);
}
#[test]
fn unwind_restores_previously_busy_nodes() {
    let queue = Retirement::new();
    queue.try_enqueue(SyncSet::EMPTY, 0).unwrap();
    queue.try_enqueue(SyncSet::EMPTY, 1).unwrap();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        queue.collect_limited_retry(&CompletedValues::all(), 8, 8, |item| {
            if item == 0 {
                Err(item)
            } else {
                panic!("later destructor")
            }
        });
    }));
    assert_eq!(queue.len(), 1);
    let mut observed = Vec::new();
    queue.collect(&CompletedValues::all(), &mut observed);
    assert_eq!(observed, [0]);
}
