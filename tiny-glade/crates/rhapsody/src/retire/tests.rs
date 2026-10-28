use super::*;
use crate::sync::{QueueId, SyncPoint};

fn point(queue: usize, value: u64) -> SyncSet {
    SyncSet::from_point(SyncPoint {
        queue: QueueId::new(queue).unwrap(),
        value,
    })
}

#[test]
fn unused_or_completed_items_come_back_immediately() {
    let retirement = Retirement::new();
    let done = CompletedValues::default();
    assert_eq!(
        retirement.retire(SyncSet::EMPTY, "never submitted", &done),
        Ok(Some("never submitted"))
    );
    let mut done = done;
    done.raise(QueueId::new(0).unwrap(), 3);
    assert_eq!(
        retirement.retire(point(0, 3), "finished", &done),
        Ok(Some("finished"))
    );
    assert!(retirement.is_empty());
}

#[test]
fn items_wait_for_every_queue_they_used() {
    let retirement = Retirement::new();
    let mut done = CompletedValues::default();
    let mut both = point(0, 2);
    both.merge(&point(1, 5));
    assert!(
        retirement
            .retire(both, "cross-queue", &done)
            .unwrap()
            .is_none()
    );
    assert!(
        retirement
            .retire(point(0, 1), "graphics", &done)
            .unwrap()
            .is_none()
    );

    let mut ready = Vec::new();
    done.raise(QueueId::new(0).unwrap(), 2);
    assert_eq!(retirement.collect(&done, &mut ready), 1);
    assert_eq!(ready, ["graphics"], "队列 1 未完成时跨队列对象必须保留");

    done.raise(QueueId::new(1).unwrap(), 5);
    assert_eq!(retirement.collect(&done, &mut ready), 1);
    assert_eq!(ready, ["graphics", "cross-queue"]);
    assert!(retirement.is_empty());
}

#[test]
fn drain_all_releases_everything() {
    let retirement = Retirement::new();
    let done = CompletedValues::default();
    for value in 1..=3 {
        assert!(
            retirement
                .retire(point(0, value), value, &done)
                .unwrap()
                .is_none()
        );
    }
    let mut ready = Vec::new();
    assert_eq!(retirement.drain_all(&mut ready), 3);
    ready.sort_unstable();
    assert_eq!(ready, [1, 2, 3]);
    assert!(retirement.is_empty());
}

#[test]
fn bounded_scans_progress_past_pending_entries_and_inspect_swapped_tail() {
    let retirement = Retirement::new();
    for value in 0..7 {
        retirement
            .try_enqueue(point(0, if value < 4 { 9 } else { 1 }), value)
            .ok()
            .unwrap();
    }
    let mut done = CompletedValues::default();
    done.raise(QueueId::new(0).unwrap(), 1);
    let mut ready = Vec::new();
    for _ in 0..7 {
        let progress = retirement.collect_limited(&done, 1, 1, |item| ready.push(item));
        assert!(progress.scanned <= 1 && progress.destroyed <= 1);
    }
    ready.sort_unstable();
    assert_eq!(ready, [4, 5, 6]);
    assert_eq!(retirement.len(), 4);
    done.raise(QueueId::new(0).unwrap(), 9);
    assert_eq!(retirement.collect(&done, &mut ready), 4);
    assert!(retirement.is_empty());
    retirement.try_enqueue(point(0, 1), 8).ok().unwrap();
    assert_eq!(
        retirement.collect_limited(&done, 0, 1, |_| panic!("zero scans")),
        CollectionProgress::default()
    );
    assert_eq!(
        retirement.collect_limited(&done, 1, 0, |_| panic!("zero destroys")),
        CollectionProgress::default()
    );
    let mut final_ready = Vec::new();
    retirement.collect_limited(&done, 1, 1, |item| final_ready.push(item));
    assert_eq!(final_ready, [8]);
}

#[test]
fn every_three_queue_completion_permutation_obeys_small_budgets() {
    const ORDERS: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for order in ORDERS {
        for scans in 1..=7 {
            for destroys in 1..=scans.min(3) {
                let retirement = Retirement::new();
                let mut expected = Vec::new();
                for item in 0..23 {
                    let mut after = SyncSet::EMPTY;
                    for queue in 0..3 {
                        after.merge(&point(queue, (item % 3 + 1) as u64));
                    }
                    expected.push(after);
                    retirement.try_enqueue(after, item).ok().unwrap();
                }
                let mut released = [false; 23];
                let mut done = CompletedValues::default();
                for value in 1..=3 {
                    for queue in order {
                        done.raise(QueueId::new(queue).unwrap(), value);
                        for _ in 0..23 {
                            let progress =
                                retirement.collect_limited(&done, scans, destroys, |item| {
                                    assert!(expected[item].is_reached(&done));
                                    assert!(!released[item], "object was reclaimed twice");
                                    released[item] = true;
                                });
                            assert!(progress.scanned <= scans);
                            assert!(progress.destroyed <= destroys);
                        }
                    }
                }
                assert!(retirement.is_empty(), "completion order {order:?}");
                assert!(released.into_iter().all(|item| item));
            }
        }
    }
}

#[test]
fn seeded_random_four_queue_completion_reclaims_exactly_once() {
    fn random(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }
    for mut seed in [1, 0x519a_423e, 0x7521_f1f4_3691_ab35] {
        let retirement = Retirement::new();
        let mut expected = Vec::new();
        for item in 0..97 {
            let mut after = SyncSet::EMPTY;
            for queue in 0..4 {
                after.merge(&point(queue, random(&mut seed) % 8 + 1));
            }
            expected.push(after);
            retirement.try_enqueue(after, item).ok().unwrap();
        }
        let mut done = CompletedValues::default();
        let mut released = [false; 97];
        for tick in 0..1200 {
            let queue = QueueId::new((random(&mut seed) % 4) as usize).unwrap();
            done.raise(queue, (done.get(queue) + 1).min(8));
            let scans = (random(&mut seed) % 11 + 1) as usize;
            let destroys = (random(&mut seed) % scans as u64 + 1) as usize;
            let progress = retirement.collect_limited(&done, scans, destroys, |item| {
                assert!(expected[item].is_reached(&done));
                assert!(!released[item], "object was reclaimed twice at tick {tick}");
                released[item] = true;
            });
            assert!(progress.scanned <= scans && progress.destroyed <= destroys);
            if retirement.is_empty() {
                break;
            }
        }
        assert!(retirement.is_empty(), "completed entries must not starve");
        assert!(released.into_iter().all(|item| item));
    }
}

#[test]
fn reserved_queue_move_never_requests_more_storage() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let retirement = Retirement::new();
    let mut after = point(0, 5);
    after.merge(&point(1, 100));
    let ticket = RetirementTicket::reserve(&stats).unwrap();
    let charged = stats.cpu_memory_usage().total();
    assert!(charged > 0);
    retirement.enqueue_reserved(after, "cross-queue", ticket);
    stats.set_host_budget(charged).unwrap(); // No further reservation fits.
    let mut done = CompletedValues::default();
    done.raise(QueueId::new(0).unwrap(), 5);
    let progress = retirement.collect_limited(&done, 8, 8, |_| panic!("early destruction"));
    assert_eq!(
        progress,
        CollectionProgress {
            scanned: 1,
            destroyed: 0
        }
    );
    assert_eq!(retirement.len(), 1);
    assert_eq!(stats.cpu_memory_usage().total(), charged);
    assert!(RetirementTicket::<&str>::reserve(&stats).is_err());
    done.raise(QueueId::new(1).unwrap(), 100);
    let mut ready = Vec::new();
    assert_eq!(retirement.collect(&done, &mut ready), 1);
    assert_eq!(ready, ["cross-queue"]);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    let ticket = RetirementTicket::<&str>::reserve(&stats).unwrap();
    drop(ticket); // Cancelled creation returns node storage and its credit.
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

#[test]
fn injected_enqueue_storage_failure_returns_original_item_without_counting_it() {
    let retirement = Retirement::new();
    retirement
        .fail_next_insert_queue
        .store(1, Ordering::Relaxed);
    assert_eq!(retirement.try_enqueue(point(1, 12), "owned"), Err("owned"));
    assert!(retirement.is_empty());
    assert!(retirement.queues[1].lock().is_empty());
    retirement.try_enqueue(point(1, 12), "retry").ok().unwrap();
    assert_eq!(retirement.len(), 1);
    let mut ready = Vec::new();
    assert_eq!(retirement.drain_all(&mut ready), 1);
    assert_eq!(ready, ["retry"]);
    assert!(retirement.is_empty());
}

#[test]
fn collection_touches_only_the_reached_prefix() {
    let retirement = Retirement::new();
    for value in 1..=1000u64 {
        retirement.try_enqueue(point(0, value), value).ok().unwrap();
    }
    let mut done = CompletedValues::default();
    done.raise(QueueId::new(0).unwrap(), 10);
    let mut ready = Vec::new();
    let progress = retirement.collect_limited(&done, 4096, 4096, |item| ready.push(item));
    assert_eq!(
        progress.scanned, 10,
        "unreached entries are never inspected"
    );
    assert_eq!(ready, (1..=10).collect::<Vec<_>>());
    // A multi-queue entry moves once to the queue it still waits for.
    let mut both = point(0, 5);
    both.merge(&point(1, 3));
    retirement.try_enqueue(both, 0).ok().unwrap();
    let mut ready = Vec::new();
    assert_eq!(retirement.collect(&done, &mut ready), 0);
    done.raise(QueueId::new(1).unwrap(), 3);
    assert_eq!(retirement.collect(&done, &mut ready), 1);
    assert_eq!(ready, [0]);
    assert_eq!(retirement.len(), 990);
}
