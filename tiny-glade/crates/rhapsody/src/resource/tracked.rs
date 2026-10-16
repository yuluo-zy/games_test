//! 资源“最后一次 GPU 使用”的登记与查询（[`SyncCell`]）。
//!
//! 每次提交成功得到 [`SyncPoint`]，执行器把它登记到本次提交引用的每个资源的
//! `SyncCell` 上；资源最后一个句柄 drop 时，携带 [`SyncCell::snapshot`] 进入
//! [`Retirement`](crate::sync::retire::Retirement)，待对应队列完成后再销毁。
//!
//! 与“先 `Mutex<SyncSet>`、基准显示争用再换原子数组”的保守路线不同，这里直接采用
//! 逐队列 `AtomicU64` 的 `fetch_max`：登记是每帧热路径（每次提交 × 每个引用资源），而
//! 读只发生在帧回收边界。每个队列一条独立原子，登记只写一条 cache line，不跨队列串行，
//! 也不分配堆内存。

use crate::sync::{CompletedValues, MAX_QUEUES, QueueId, SyncPoint, SyncSet};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct SyncCell {
    /// `values[queue.index()]` 记录该队列上最后一次登记的时间线值（0 = 从未使用）。
    values: [AtomicU64; MAX_QUEUES],
}

impl SyncCell {
    pub fn new() -> Self {
        Self {
            values: std::array::from_fn(|_| AtomicU64::new(0)),
        }
    }

    /// 登记一次成功提交：把该队列“用到的最晚值”推进到 `point.value`。
    ///
    /// 用 `AcqRel`：写侧以 release 发布本次登记，使回收线程 acquire 后能看到“该资源确实
    /// 被用到这一提交”；读侧 acquire 建立与其他写者的先后序。值只增不减（`fetch_max`）。
    pub fn insert(&self, point: SyncPoint) {
        self.values[point.queue.index()].fetch_max(point.value, Ordering::AcqRel);
    }

    /// 批量登记一组队列依赖。
    pub fn merge(&self, set: &SyncSet) {
        for point in set.points() {
            self.insert(point);
        }
    }

    /// 取当前登记快照；结果随资源的 `Arc` 一起交给 `Retirement`。
    pub fn snapshot(&self) -> SyncSet {
        let mut out = SyncSet::EMPTY;
        for (index, slot) in self.values.iter().enumerate() {
            let value = slot.load(Ordering::Acquire);
            if value != 0 {
                out.insert(SyncPoint {
                    queue: QueueId::new(index).expect("下标恒小于 MAX_QUEUES"),
                    value,
                });
            }
        }
        out
    }

    /// 所有已登记队列的值是否都已完成。
    pub fn is_reached(&self, completed: &CompletedValues) -> bool {
        self.values.iter().enumerate().all(|(index, slot)| {
            let need = slot.load(Ordering::Acquire);
            need == 0 || need <= completed.get(QueueId::new(index).expect("下标恒小于 MAX_QUEUES"))
        })
    }

    /// 清空登记。仅用于显式重置使用语义的场景；正常退役依赖 [`SyncCell::snapshot`]，
    /// 不需要清空。
    pub fn clear(&self) {
        for slot in &self.values {
            slot.store(0, Ordering::Release);
        }
    }
}

impl Default for SyncCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QueueId {
        QueueId::new(index).unwrap()
    }

    #[test]
    fn insert_keeps_the_latest_value_per_queue() {
        let cell = SyncCell::new();
        cell.insert(SyncPoint {
            queue: q(0),
            value: 5,
        });
        cell.insert(SyncPoint {
            queue: q(0),
            value: 3,
        });
        cell.insert(SyncPoint {
            queue: q(2),
            value: 7,
        });

        let snapshot = cell.snapshot();
        assert_eq!(snapshot.get(q(0)), 5, "只增不减");
        assert_eq!(snapshot.get(q(1)), 0);
        assert_eq!(snapshot.get(q(2)), 7);
    }

    #[test]
    fn merge_applies_every_queue_dependency() {
        let cell = SyncCell::new();
        let mut set = SyncSet::from_point(SyncPoint {
            queue: q(0),
            value: 4,
        });
        set.insert(SyncPoint {
            queue: q(1),
            value: 2,
        });
        cell.merge(&set);
        assert_eq!(cell.snapshot(), set);
    }

    #[test]
    fn reached_requires_every_registered_queue() {
        let cell = SyncCell::new();
        cell.insert(SyncPoint {
            queue: q(0),
            value: 4,
        });
        cell.insert(SyncPoint {
            queue: q(1),
            value: 2,
        });

        let mut done = CompletedValues::default();
        done.raise(q(0), 10);
        assert!(!cell.is_reached(&done), "队列 1 尚未完成");
        done.raise(q(1), 2);
        assert!(cell.is_reached(&done));
        assert!(SyncCell::new().is_reached(&CompletedValues::default()));
    }

    #[test]
    fn clear_resets_all_queues() {
        let cell = SyncCell::new();
        cell.insert(SyncPoint {
            queue: q(3),
            value: 9,
        });
        cell.clear();
        assert!(cell.snapshot().is_empty());
    }

    #[test]
    fn concurrent_inserts_are_serialized_and_lossless() {
        use std::sync::Arc;

        let cell = Arc::new(SyncCell::new());
        std::thread::scope(|scope| {
            for worker in 0..8u64 {
                let cell = Arc::clone(&cell);
                scope.spawn(move || {
                    for value in 1..=1000 {
                        cell.insert(SyncPoint {
                            queue: q((worker % 4) as usize),
                            value,
                        });
                    }
                });
            }
        });

        let snapshot = cell.snapshot();
        for queue in 0..4 {
            assert_eq!(snapshot.get(q(queue)), 1000, "每个队列都有线程写满到 1000");
        }
    }
}
