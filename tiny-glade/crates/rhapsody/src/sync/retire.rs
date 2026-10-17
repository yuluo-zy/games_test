//! 延迟回收：GPU 可能仍在使用的对象挂在它依赖的 [`SyncSet`] 上，完成后交还调用方销毁。
//!
//! 销毁本身不在这里发生：[`Retirement::collect`] 把到期对象移到调用方提供的缓冲里，
//! 调用方在释放锁之后再 drop，避免持锁执行 `vkDestroy*`。
use super::{CompletedValues, SyncSet};
use std::time::Instant;

pub struct Retirement<T> {
    entries: Vec<(SyncSet, T, Instant)>,
    scan_cursor: usize,
    ages: RetirementAges,
}

/// Enqueue-to-removal wall time for queued objects. Includes GPU waiting and
/// deferred maintenance; it does not claim to measure GPU completion latency.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct RetirementAges {
    pub reclaimed: u64,
    pub total_ns: u64,
    pub maximum_ns: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CollectionProgress {
    pub scanned: usize,
    pub destroyed: usize,
}

impl<T> Default for Retirement<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Retirement<T> {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            scan_cursor: 0,
            ages: RetirementAges {
                reclaimed: 0,
                total_ns: 0,
                maximum_ns: 0,
            },
        }
    }

    /// 登记一个对象。`after` 为空（从未提交给 GPU）或已完成时立即还给调用方。
    #[must_use = "返回的对象未被 GPU 使用，应由调用方立即销毁"]
    pub fn retire(&mut self, after: SyncSet, item: T, completed: &CompletedValues) -> Option<T> {
        if after.is_reached(completed) {
            return Some(item);
        }
        self.enqueue(after, item);
        None
    }

    /// Queue an object even when its dependency has already completed. Device
    /// destruction uses this path so dropping a submitted owner does not bypass
    /// the maintenance budget.
    pub fn enqueue(&mut self, after: SyncSet, item: T) {
        self.entries.push((after, item, Instant::now()));
    }
    pub fn ages(&self) -> RetirementAges {
        self.ages
    }
    fn observe_age(&mut self, queued: Instant) {
        let ns = queued.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
        self.ages.reclaimed = self.ages.reclaimed.saturating_add(1);
        self.ages.total_ns = self.ages.total_ns.saturating_add(ns);
        self.ages.maximum_ns = self.ages.maximum_ns.max(ns);
    }

    /// Inspect and remove a bounded number of entries. The callback must only
    /// move the object into already available storage; destruction belongs
    /// outside the caller's retirement lock.
    pub fn collect_limited(
        &mut self,
        completed: &CompletedValues,
        max_scans: usize,
        max_destroys: usize,
        mut ready: impl FnMut(T),
    ) -> CollectionProgress {
        let scan_limit = max_scans.min(self.entries.len());
        let mut progress = CollectionProgress::default();
        while progress.scanned < scan_limit
            && progress.destroyed < max_destroys
            && !self.entries.is_empty()
        {
            self.scan_cursor %= self.entries.len();
            progress.scanned += 1;
            if self.entries[self.scan_cursor].0.is_reached(completed) {
                let (_, item, queued) = self.entries.swap_remove(self.scan_cursor);
                self.observe_age(queued);
                progress.destroyed += 1;
                ready(item);
                // Inspect the replacement before advancing; advancing here
                // would skip an entry every time swap_remove moves the tail.
            } else {
                self.scan_cursor += 1;
            }
        }
        if self.entries.is_empty() {
            self.scan_cursor = 0;
        } else {
            self.scan_cursor %= self.entries.len();
        }
        progress
    }

    /// 把所有已完成的对象移入 `ready`，返回移出的数量。
    pub fn collect(&mut self, completed: &CompletedValues, ready: &mut Vec<T>) -> usize {
        let before = ready.len();
        let mut index = 0;
        while index < self.entries.len() {
            if self.entries[index].0.is_reached(completed) {
                let (_, item, queued) = self.entries.swap_remove(index);
                self.observe_age(queued);
                ready.push(item);
            } else {
                index += 1;
            }
        }
        self.scan_cursor = 0;
        ready.len() - before
    }

    /// 无条件移出全部对象（`vkDeviceWaitIdle` 之后或设备丢失后）。
    pub fn drain_all(&mut self, ready: &mut Vec<T>) -> usize {
        let count = self.entries.len();
        while let Some((_, item, queued)) = self.entries.pop() {
            self.observe_age(queued);
            ready.push(item);
        }
        self.scan_cursor = 0;
        count
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
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
        let mut retirement = Retirement::new();
        let done = CompletedValues::default();
        assert_eq!(
            retirement.retire(SyncSet::EMPTY, "never submitted", &done),
            Some("never submitted")
        );
        let mut done = done;
        done.raise(QueueId::new(0).unwrap(), 3);
        assert_eq!(
            retirement.retire(point(0, 3), "finished", &done),
            Some("finished")
        );
        assert!(retirement.is_empty());
    }

    #[test]
    fn items_wait_for_every_queue_they_used() {
        let mut retirement = Retirement::new();
        let mut done = CompletedValues::default();
        let mut both = point(0, 2);
        both.merge(&point(1, 5));
        assert!(retirement.retire(both, "cross-queue", &done).is_none());
        assert!(retirement.retire(point(0, 1), "graphics", &done).is_none());

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
        let mut retirement = Retirement::new();
        let done = CompletedValues::default();
        for value in 1..=3 {
            assert!(retirement.retire(point(0, value), value, &done).is_none());
        }
        let mut ready = Vec::new();
        assert_eq!(retirement.drain_all(&mut ready), 3);
        ready.sort_unstable();
        assert_eq!(ready, [1, 2, 3]);
        assert!(retirement.is_empty());
    }

    #[test]
    fn bounded_scans_progress_past_pending_entries_and_inspect_swapped_tail() {
        let mut retirement = Retirement::new();
        for value in 0..7 {
            retirement.enqueue(point(0, if value < 4 { 9 } else { 1 }), value);
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
        retirement.enqueue(point(0, 1), 8);
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
                    let mut retirement = Retirement::new();
                    let mut expected = Vec::new();
                    for item in 0..23 {
                        let mut after = SyncSet::EMPTY;
                        for queue in 0..3 {
                            after.merge(&point(queue, (item % 3 + 1) as u64));
                        }
                        expected.push(after);
                        retirement.enqueue(after, item);
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
            let mut retirement = Retirement::new();
            let mut expected = Vec::new();
            for item in 0..97 {
                let mut after = SyncSet::EMPTY;
                for queue in 0..4 {
                    after.merge(&point(queue, random(&mut seed) % 8 + 1));
                }
                expected.push(after);
                retirement.enqueue(after, item);
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
}
