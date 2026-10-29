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
#[doc(hidden)]
pub mod generation;
#[doc(hidden)]
pub mod timeline;

use crate::native_types::StageMask;

pub use generation::{DeviceGeneration, Generationed};
pub use timeline::{QueueLockStats, SubmitSignal, TimelineApi, Timelines};

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
    stages: [StageMask; MAX_QUEUES],
}

impl StagedDependencies {
    pub const EMPTY: Self = Self {
        points: SyncSet::EMPTY,
        stages: [StageMask::empty(); MAX_QUEUES],
    };
    /// Conservative wrapper for prerequisites whose consumer stage is unknown.
    pub fn opaque(points: SyncSet) -> Self {
        let mut result = Self::EMPTY;
        result.merge_set(&points, StageMask::ALL_COMMANDS);
        result
    }
    /// Add one prerequisite needed from `stages` onward. Empty stages mean no
    /// proven consumer stage and are widened to `ALL_COMMANDS`.
    pub fn insert(&mut self, point: SyncPoint, stages: StageMask) {
        if point.value == 0 {
            return;
        }
        self.points.insert(point);
        // Host access is synchronized by waiting for the submission receipt;
        // HOST is never a legal semaphore wait destination in a command queue.
        let stages = stages & !StageMask::HOST;
        self.stages[point.queue.index()] |= if stages.is_empty() {
            StageMask::ALL_COMMANDS
        } else {
            stages
        };
    }
    pub fn merge_set(&mut self, points: &SyncSet, stages: StageMask) {
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
    pub fn stages(&self, queue: QueueId) -> StageMask {
        if self.points.get(queue) == 0 {
            StageMask::empty()
        } else {
            self.stages[queue.index()]
        }
    }
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
    /// `(point, stages)` per source queue.
    pub fn waits(&self) -> impl Iterator<Item = (SyncPoint, StageMask)> + '_ {
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

#[cfg(all(test, feature = "native-types"))]
mod tests;
