//! Deferred destruction: objects the GPU may still use wait on the
//! [`SyncSet`] they depend on and are handed back once it completes.
//!
//! Entries live in per-queue FIFOs ordered by timeline value, each guarded by
//! its own lock. An entry is filed under one queue it depends on; when that
//! queue reaches its value the entry either completes or moves to the next
//! queue it still waits for (at most once per queue). Collection therefore
//! pops exactly the reached prefix of each FIFO, in O(reached), and reached
//! entries can never starve behind unreached ones.
use super::{CompletedValues, MAX_QUEUES, QueueId, SyncSet};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Entry<T> {
    /// Value on the FIFO's queue this entry waits for.
    value: u64,
    after: SyncSet,
    item: T,
}

pub struct Retirement<T> {
    queues: [Mutex<VecDeque<Entry<T>>>; MAX_QUEUES],
    len: AtomicUsize,
    /// Rotates the first queue visited by bounded collection.
    cursor: AtomicUsize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CollectionProgress {
    /// Entries removed from a FIFO (completed or moved to another queue).
    pub scanned: usize,
    pub destroyed: usize,
}

impl<T> Default for Retirement<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// The queue an entry waits on next: the first queue whose value is not yet
/// reached by `completed`, or `None` when the entry is complete.
fn pending_queue(after: &SyncSet, completed: &CompletedValues) -> Option<(usize, u64)> {
    after
        .points()
        .find(|point| point.value > completed.get(point.queue))
        .map(|point| (point.queue.index(), point.value))
}

impl<T> Retirement<T> {
    pub fn new() -> Self {
        Self {
            queues: std::array::from_fn(|_| Mutex::new(VecDeque::new())),
            len: AtomicUsize::new(0),
            cursor: AtomicUsize::new(0),
        }
    }

    /// Hand `item` back when `after` is empty or already reached; otherwise
    /// queue it. Returns `Err(item)` only when queue storage cannot grow.
    pub fn retire(
        &self,
        after: SyncSet,
        item: T,
        completed: &CompletedValues,
    ) -> Result<Option<T>, T> {
        if after.is_reached(completed) {
            return Ok(Some(item));
        }
        self.try_enqueue(after, item).map(|()| None)
    }

    /// Queue an object even when its dependency has already completed, so
    /// dropping a submitted owner does not bypass the maintenance budget.
    /// Storage growth is fallible: on host OOM the item is returned.
    pub fn try_enqueue(&self, after: SyncSet, item: T) -> Result<(), T> {
        let (queue, value) = after
            .points()
            .next()
            .map_or((0, 0), |point| (point.queue.index(), point.value));
        self.insert(queue, Entry { value, after, item })
            .map_err(|entry| entry.item)?;
        self.len.fetch_add(1, Ordering::AcqRel);
        Ok(())
    }

    fn insert(&self, queue: usize, entry: Entry<T>) -> Result<(), Entry<T>> {
        let mut fifo = self.queues[queue].lock();
        if fifo.try_reserve(1).is_err() {
            return Err(entry);
        }
        // Retirements arrive almost in value order; search from the back.
        let at = fifo.partition_point(|queued| queued.value <= entry.value);
        fifo.insert(at, entry);
        Ok(())
    }

    /// Pop the next reached entry of `queue`, without holding the lock after.
    fn pop_reached(&self, queue: usize, completed: &CompletedValues) -> Option<Entry<T>> {
        let done = QueueId::new(queue).map_or(0, |id| completed.get(id));
        let mut fifo = self.queues[queue].lock();
        if fifo.front().is_some_and(|entry| entry.value <= done) {
            fifo.pop_front()
        } else {
            None
        }
    }

    /// Remove up to `max_destroys` completed entries, inspecting at most
    /// `max_scans` reached entries. `ready` runs without any retirement lock
    /// held, so it may destroy the object directly.
    pub fn collect_limited(
        &self,
        completed: &CompletedValues,
        max_scans: usize,
        max_destroys: usize,
        mut ready: impl FnMut(T),
    ) -> CollectionProgress {
        let mut progress = CollectionProgress::default();
        if self.is_empty() {
            return progress;
        }
        let start = self.cursor.fetch_add(1, Ordering::Relaxed);
        for offset in 0..MAX_QUEUES {
            let queue = (start + offset) % MAX_QUEUES;
            while progress.scanned < max_scans && progress.destroyed < max_destroys {
                let Some(mut entry) = self.pop_reached(queue, completed) else {
                    break;
                };
                progress.scanned += 1;
                match pending_queue(&entry.after, completed) {
                    None => {
                        self.len.fetch_sub(1, Ordering::AcqRel);
                        progress.destroyed += 1;
                        ready(entry.item);
                    }
                    Some((next, value)) => {
                        entry.value = value;
                        if let Err(entry) = self.insert(next, entry) {
                            // No room to move it: keep it reached at the head of
                            // its FIFO and end this pass instead of spinning.
                            self.queues[queue].lock().push_front(entry);
                            return progress;
                        }
                    }
                }
            }
        }
        progress
    }

    /// Move every completed object into `ready`; returns how many moved.
    pub fn collect(&self, completed: &CompletedValues, ready: &mut Vec<T>) -> usize {
        self.collect_limited(completed, usize::MAX, usize::MAX, |item| ready.push(item))
            .destroyed
    }

    /// Remove every object unconditionally (after `vkDeviceWaitIdle` or loss).
    pub fn drain_all(&self, ready: &mut Vec<T>) -> usize {
        self.collect_limited(&CompletedValues::all(), usize::MAX, usize::MAX, |item| {
            ready.push(item)
        })
        .destroyed
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::Acquire)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
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
}
