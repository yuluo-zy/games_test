//! 延迟回收：GPU 可能仍在使用的对象挂在它依赖的 [`SyncSet`] 上，完成后交还调用方销毁。
//!
//! 销毁本身不在这里发生：[`Retirement::collect`] 把到期对象移到调用方提供的缓冲里，
//! 调用方在释放锁之后再 drop，避免持锁执行 `vkDestroy*`。
use super::{CompletedValues, SyncSet};

pub struct Retirement<T> {
    entries: Vec<(SyncSet, T)>,
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
        }
    }

    /// 登记一个对象。`after` 为空（从未提交给 GPU）或已完成时立即还给调用方。
    #[must_use = "返回的对象未被 GPU 使用，应由调用方立即销毁"]
    pub fn retire(&mut self, after: SyncSet, item: T, completed: &CompletedValues) -> Option<T> {
        if after.is_reached(completed) {
            return Some(item);
        }
        self.entries.push((after, item));
        None
    }

    /// 把所有已完成的对象移入 `ready`，返回移出的数量。
    pub fn collect(&mut self, completed: &CompletedValues, ready: &mut Vec<T>) -> usize {
        let before = ready.len();
        let mut index = 0;
        while index < self.entries.len() {
            if self.entries[index].0.is_reached(completed) {
                ready.push(self.entries.swap_remove(index).1);
            } else {
                index += 1;
            }
        }
        ready.len() - before
    }

    /// 无条件移出全部对象（`vkDeviceWaitIdle` 之后或设备丢失后）。
    pub fn drain_all(&mut self, ready: &mut Vec<T>) -> usize {
        let count = self.entries.len();
        ready.extend(self.entries.drain(..).map(|(_, item)| item));
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
        assert_eq!(retirement.retire(SyncSet::EMPTY, "never submitted", &done), Some("never submitted"));
        let mut done = done;
        done.raise(QueueId::new(0).unwrap(), 3);
        assert_eq!(retirement.retire(point(0, 3), "finished", &done), Some("finished"));
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
}
