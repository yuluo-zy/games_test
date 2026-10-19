//! GPU 完成的唯一真相源。
//!
//! 每个逻辑队列一条时间线信号量（`timeline::QueueTimeline`）。一次成功的提交在该队列上
//! signal 一个严格递增的值，得到 [`SyncPoint`]；资源或命令池记录"在哪些队列上用到哪个值"
//! （[`SyncSet`]），当各队列已完成的值（[`CompletedValues`]）都不小于记录值时即可回收
//! （`retire::Retirement`）。
//!
//! 不变量：
//! - 值只在 `vkQueueSubmit` 成功后才算已提交；失败的提交不占用值，因此任何已知的
//!   [`SyncPoint`] 最终都会完成（或设备丢失）。
//! - 等待一个尚未提交的值是误用（会永远挂起），返回 [`crate::Error::InvalidUsage`]。
internal_mod!(retire, timeline);
pub use timeline::{QueueLockStats, TimelineApi, Timelines};

/// 一个设备上最多同时使用的逻辑队列数。
pub const MAX_QUEUES: usize = 8;

/// 逻辑队列编号，`0..MAX_QUEUES`。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueueId(u8);

impl QueueId {
    pub const fn new(index: usize) -> Option<Self> {
        if index < MAX_QUEUES {
            Some(Self(index as u8))
        } else {
            None
        }
    }
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// 某个队列时间线上的一个值。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SyncPoint {
    pub queue: QueueId,
    pub value: u64,
}

/// 每个队列至多一个需要等待的值（0 = 不依赖该队列）；合并取逐队列最大值。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SyncSet {
    values: [u64; MAX_QUEUES],
}

impl SyncSet {
    pub const EMPTY: Self = Self {
        values: [0; MAX_QUEUES],
    };

    pub fn from_point(point: SyncPoint) -> Self {
        let mut set = Self::EMPTY;
        set.insert(point);
        set
    }
    pub fn insert(&mut self, point: SyncPoint) {
        let slot = &mut self.values[point.queue.index()];
        *slot = (*slot).max(point.value);
    }
    pub fn merge(&mut self, other: &Self) {
        for (mine, theirs) in self.values.iter_mut().zip(other.values) {
            *mine = (*mine).max(theirs);
        }
    }
    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|&value| value == 0)
    }
    pub fn get(&self, queue: QueueId) -> u64 {
        self.values[queue.index()]
    }
    pub fn points(&self) -> impl Iterator<Item = SyncPoint> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter(|&(_, &value)| value != 0)
            .map(|(index, &value)| SyncPoint {
                queue: QueueId(index as u8),
                value,
            })
    }
    /// 所有依赖的值都已完成。
    pub fn is_reached(&self, completed: &CompletedValues) -> bool {
        self.values
            .iter()
            .zip(completed.values)
            .all(|(&need, done)| need <= done)
    }
}

impl From<SyncPoint> for SyncSet {
    fn from(point: SyncPoint) -> Self {
        Self::from_point(point)
    }
}

/// Cross-queue prerequisites with the destination stages that first need
/// each queue's value. A wait built from this set blocks only those stages;
/// prerequisites without a proven consumer stage carry `ALL_COMMANDS`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StagedDependencies {
    points: SyncSet,
    stages: [ash::vk::PipelineStageFlags; MAX_QUEUES],
}

impl StagedDependencies {
    pub const EMPTY: Self = Self {
        points: SyncSet::EMPTY,
        stages: [ash::vk::PipelineStageFlags::empty(); MAX_QUEUES],
    };
    /// Conservative wrapper for prerequisites whose consumer stage is unknown.
    pub fn opaque(points: SyncSet) -> Self {
        let mut result = Self::EMPTY;
        result.merge_set(&points, ash::vk::PipelineStageFlags::ALL_COMMANDS);
        result
    }
    /// Add one prerequisite needed from `stages` onward. Empty stages mean no
    /// proven consumer stage and are widened to `ALL_COMMANDS`.
    pub fn insert(&mut self, point: SyncPoint, stages: ash::vk::PipelineStageFlags) {
        if point.value == 0 {
            return;
        }
        self.points.insert(point);
        self.stages[point.queue.index()] |= if stages.is_empty() {
            ash::vk::PipelineStageFlags::ALL_COMMANDS
        } else {
            stages
        };
    }
    pub fn merge_set(&mut self, points: &SyncSet, stages: ash::vk::PipelineStageFlags) {
        for point in points.points() {
            self.insert(point, stages);
        }
    }
    pub fn merge(&mut self, other: &Self) {
        self.points.merge(&other.points);
        for (mine, theirs) in self.stages.iter_mut().zip(other.stages) {
            *mine |= theirs;
        }
    }
    pub fn points(&self) -> SyncSet {
        self.points
    }
    /// Destination stages that must wait for `queue`'s value (empty when the
    /// set has no prerequisite on that queue).
    pub fn stages(&self, queue: QueueId) -> ash::vk::PipelineStageFlags {
        if self.points.get(queue) == 0 {
            ash::vk::PipelineStageFlags::empty()
        } else {
            self.stages[queue.index()]
        }
    }
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
    /// `(point, stages)` per source queue.
    pub fn waits(&self) -> impl Iterator<Item = (SyncPoint, ash::vk::PipelineStageFlags)> + '_ {
        self.points
            .points()
            .map(|point| (point, self.stages[point.queue.index()]))
    }
}

/// 某一时刻各队列已完成的时间线值。只会单调增大。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompletedValues {
    values: [u64; MAX_QUEUES],
}

impl CompletedValues {
    pub fn get(&self, queue: QueueId) -> u64 {
        self.values[queue.index()]
    }
    /// 推进某队列的完成值；比现有值小的观测被忽略（计数器只增不减）。
    pub fn raise(&mut self, queue: QueueId, value: u64) {
        let slot = &mut self.values[queue.index()];
        *slot = (*slot).max(value);
    }
    /// 视为所有已提交工作都已完成（`vkDeviceWaitIdle` 之后或设备丢失后）。
    pub fn all() -> Self {
        Self {
            values: [u64::MAX; MAX_QUEUES],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QueueId {
        QueueId::new(index).unwrap()
    }

    #[test]
    fn queue_id_is_bounded() {
        assert!(QueueId::new(MAX_QUEUES - 1).is_some());
        assert!(QueueId::new(MAX_QUEUES).is_none());
    }

    #[test]
    fn sync_set_keeps_the_latest_value_per_queue() {
        let mut set = SyncSet::EMPTY;
        assert!(set.is_empty());
        set.insert(SyncPoint {
            queue: q(0),
            value: 5,
        });
        set.insert(SyncPoint {
            queue: q(0),
            value: 3,
        });
        set.insert(SyncPoint {
            queue: q(2),
            value: 7,
        });
        assert_eq!(set.get(q(0)), 5);
        assert_eq!(
            set.points().collect::<Vec<_>>(),
            [
                SyncPoint {
                    queue: q(0),
                    value: 5
                },
                SyncPoint {
                    queue: q(2),
                    value: 7
                }
            ]
        );
        let mut other = SyncSet::from_point(SyncPoint {
            queue: q(0),
            value: 9,
        });
        other.merge(&set);
        assert_eq!((other.get(q(0)), other.get(q(2))), (9, 7));
    }

    #[test]
    fn reached_requires_every_queue() {
        let mut set = SyncSet::from_point(SyncPoint {
            queue: q(0),
            value: 4,
        });
        set.insert(SyncPoint {
            queue: q(1),
            value: 2,
        });
        let mut done = CompletedValues::default();
        done.raise(q(0), 10);
        assert!(!set.is_reached(&done), "队列 1 尚未完成");
        done.raise(q(1), 2);
        assert!(set.is_reached(&done));
        done.raise(q(1), 1);
        assert_eq!(done.get(q(1)), 2, "完成值不回退");
        assert!(SyncSet::EMPTY.is_reached(&CompletedValues::default()));
        assert!(set.is_reached(&CompletedValues::all()));
    }
}
