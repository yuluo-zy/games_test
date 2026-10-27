//! 每队列时间线与提交锁。
//!
//! [`Timelines`] 为每个逻辑队列持有一条时间线信号量和一把提交锁。提交锁同时满足 Vulkan
//! 对单个 `VkQueue` 的外部同步要求（`vkQueueSubmit`、`vkQueuePresentKHR` 等），因此不需要
//! 设备级全局锁：不同队列的提交互不阻塞。
//!
//! 锁顺序：交换链锁 → 队列提交锁 → 回收锁。提交锁内只做队列操作，不做阻塞等待。
use super::{CompletedValues, MAX_QUEUES, QueueId, SyncPoint, SyncSet};
use crate::error::{Error, Result};
use ash::vk;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

/// 时间线相关的设备调用；真实实现为 `ash::Device`，测试可替换。
pub trait TimelineApi: Send + Sync {
    fn counter_value(&self, semaphore: vk::Semaphore) -> Result<u64>;
    /// 等待全部信号量到达对应值。到达返回 `Ok(true)`，超时返回 `Ok(false)`。
    fn wait_all(
        &self,
        semaphores: &[vk::Semaphore],
        values: &[u64],
        timeout_ns: u64,
    ) -> Result<bool>;
}

impl TimelineApi for ash::Device {
    fn counter_value(&self, semaphore: vk::Semaphore) -> Result<u64> {
        Ok(unsafe { self.get_semaphore_counter_value(semaphore) }?)
    }
    fn wait_all(
        &self,
        semaphores: &[vk::Semaphore],
        values: &[u64],
        timeout_ns: u64,
    ) -> Result<bool> {
        let info = vk::SemaphoreWaitInfo::default()
            .semaphores(semaphores)
            .values(values);
        let mut trace = crate::trace::SubmitTrace::new("vkWaitSemaphores");
        trace.count("semaphores", semaphores.len());
        let result = unsafe { self.wait_semaphores(&info, timeout_ns) };
        trace.mark("driver");
        match result {
            Ok(()) => Ok(true),
            Err(vk::Result::TIMEOUT) => Ok(false),
            Err(error) => Err(error.into()),
        }
    }
}

/// 一次提交需要 signal 的时间线值。调用方把它放进 `VkTimelineSemaphoreSubmitInfo`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubmitSignal {
    pub semaphore: vk::Semaphore,
    pub value: u64,
}

struct QueueTimeline {
    semaphore: vk::Semaphore,
    /// 最后一次成功提交的值；锁本身是该队列的外部同步锁。
    submit: Mutex<u64>,
    /// `submit` 的无锁镜像，供等待前校验。
    submitted: AtomicU64,
    /// 已观测到的完成值缓存。
    completed: AtomicU64,
    lock_acquisitions: AtomicU64,
    lock_wait_ns: AtomicU64,
    lock_wait_max_ns: AtomicU64,
}
/// Wall time to acquire a queue's external synchronization lock. Includes the
/// uncontended mutex/clock overhead; does not include the Vulkan operation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct QueueLockStats {
    pub queue: usize,
    pub acquisitions: u64,
    pub total_wait_ns: u64,
    pub maximum_wait_ns: u64,
}
impl QueueTimeline {
    fn lock(&self, statistics: bool) -> parking_lot::MutexGuard<'_, u64> {
        if !statistics {
            return self.submit.lock();
        }
        let started = Instant::now();
        let guard = self.submit.lock();
        let elapsed = started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
        let _ =
            self.lock_acquisitions
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                    Some(value.saturating_add(1))
                });
        let _ = self
            .lock_wait_ns
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                Some(value.saturating_add(elapsed))
            });
        self.lock_wait_max_ns.fetch_max(elapsed, Ordering::Relaxed);
        guard
    }
}

pub struct Timelines<A> {
    api: A,
    queues: Vec<QueueTimeline>,
    device_lost: AtomicBool,
    /// Lock wait statistics cost two clock reads per queue lock; opt-in only.
    lock_statistics: AtomicBool,
    /// `poll` calls and the driver queries they issued (diagnostics; Relaxed).
    poll_calls: AtomicU64,
    driver_queries: AtomicU64,
}

impl<A: TimelineApi> Timelines<A> {
    /// `semaphores[i]` 是 `QueueId(i)` 的时间线信号量，初值须为 0。
    pub fn new(api: A, semaphores: &[vk::Semaphore]) -> Result<Self> {
        if semaphores.is_empty() || semaphores.len() > MAX_QUEUES {
            return Err(Error::invalid_usage(format!(
                "逻辑队列数必须在 1..={MAX_QUEUES}，实际 {}",
                semaphores.len()
            )));
        }
        Ok(Self {
            api,
            queues: semaphores
                .iter()
                .map(|&semaphore| QueueTimeline {
                    semaphore,
                    submit: Mutex::new(0),
                    submitted: AtomicU64::new(0),
                    completed: AtomicU64::new(0),
                    lock_acquisitions: AtomicU64::new(0),
                    lock_wait_ns: AtomicU64::new(0),
                    lock_wait_max_ns: AtomicU64::new(0),
                })
                .collect(),
            device_lost: AtomicBool::new(false),
            lock_statistics: AtomicBool::new(false),
            poll_calls: AtomicU64::new(0),
            driver_queries: AtomicU64::new(0),
        })
    }

    pub fn api(&self) -> &A {
        &self.api
    }
    pub fn queue_count(&self) -> usize {
        self.queues.len()
    }
    /// Enable or disable queue lock wait statistics (disabled by default).
    pub fn set_lock_statistics(&self, enabled: bool) {
        self.lock_statistics.store(enabled, Ordering::Relaxed);
    }
    fn lock_queue<'a>(&self, timeline: &'a QueueTimeline) -> parking_lot::MutexGuard<'a, u64> {
        timeline.lock(self.lock_statistics.load(Ordering::Relaxed))
    }
    /// Counters stay zero unless [`Self::set_lock_statistics`] enabled them.
    pub fn queue_lock_stats(&self) -> [Option<QueueLockStats>; MAX_QUEUES] {
        std::array::from_fn(|index| {
            self.queues.get(index).map(|queue| QueueLockStats {
                queue: index,
                acquisitions: queue.lock_acquisitions.load(Ordering::Relaxed),
                total_wait_ns: queue.lock_wait_ns.load(Ordering::Relaxed),
                maximum_wait_ns: queue.lock_wait_max_ns.load(Ordering::Relaxed),
            })
        })
    }
    pub fn semaphore(&self, queue: QueueId) -> Result<vk::Semaphore> {
        Ok(self.queue(queue)?.semaphore)
    }
    fn queue(&self, queue: QueueId) -> Result<&QueueTimeline> {
        self.queues
            .get(queue.index())
            .ok_or_else(|| Error::invalid_usage(format!("队列 {} 不存在", queue.index())))
    }

    pub fn is_device_lost(&self) -> bool {
        self.device_lost.load(Ordering::Acquire)
    }
    pub fn mark_device_lost(&self) {
        if !self.device_lost.swap(true, Ordering::AcqRel) {
            log::error!("GPU 设备丢失：后续提交与等待将直接失败");
        }
    }
    fn check_alive(&self) -> Result<()> {
        if self.is_device_lost() {
            Err(Error::DeviceLost)
        } else {
            Ok(())
        }
    }
    fn observe<T>(&self, result: Result<T>) -> Result<T> {
        if let Err(error) = &result
            && error.is_device_lost()
        {
            self.mark_device_lost();
        }
        result
    }

    /// 在该队列的提交锁内执行一次提交。`submit` 收到本次要 signal 的值；只有它返回 `Ok`
    /// 之后该值才算已提交，失败不占用值。
    pub fn submit<R>(
        &self,
        queue: QueueId,
        submit: impl FnOnce(SubmitSignal) -> Result<R>,
    ) -> Result<(SyncPoint, R)> {
        self.check_alive()?;
        let timeline = self.queue(queue)?;
        let mut last = self.lock_queue(timeline);
        self.check_alive()?;
        let value = last
            .checked_add(1)
            .ok_or_else(|| Error::invalid_usage("队列时间线已耗尽"))?;
        let output = self.observe(submit(SubmitSignal {
            semaphore: timeline.semaphore,
            value,
        }))?;
        *last = value;
        timeline.submitted.store(value, Ordering::Release);
        Ok((SyncPoint { queue, value }, output))
    }

    /// 在该队列的提交锁内执行不 signal 时间线的队列操作（例如 `vkQueuePresentKHR`）。
    pub fn with_queue<R>(
        &self,
        queue: QueueId,
        operation: impl FnOnce() -> Result<R>,
    ) -> Result<R> {
        self.check_alive()?;
        let _guard = self.lock_queue(self.queue(queue)?);
        self.check_alive()?;
        self.observe(operation())
    }

    /// 整设备 idle 需要同步所有 VkQueue；按稳定的 QueueId 顺序取锁。
    pub fn with_all_queues<R>(&self, operation: impl FnOnce() -> Result<R>) -> Result<R> {
        let _guards: Vec<_> = self
            .queues
            .iter()
            .map(|queue| self.lock_queue(queue))
            .collect();
        self.check_alive()?;
        self.observe(operation())
    }

    /// 只包含成功提交的工作；不为失败或尚未提交的命令生成等待目标。
    pub fn submitted_set(&self) -> SyncSet {
        let mut set = SyncSet::default();
        for (index, queue) in self.queues.iter().enumerate() {
            set.insert(SyncPoint {
                queue: QueueId::new(index).expect("构造时已限制队列数"),
                value: queue.submitted.load(Ordering::Acquire),
            });
        }
        set
    }

    /// 该队列最后一次成功提交的值（0 = 尚无提交）。
    pub fn submitted(&self, queue: QueueId) -> Result<u64> {
        Ok(self.queue(queue)?.submitted.load(Ordering::Acquire))
    }

    /// Last observed completion values without any device query. Values only
    /// grow, so a stale snapshot is conservative: it may report work as still
    /// pending, never as completed early. Use [`Self::refresh`] once per frame
    /// or maintenance tick, and [`Self::poll`] when freshness is required.
    pub fn completed(&self) -> CompletedValues {
        let mut completed = CompletedValues::default();
        for (index, timeline) in self.queues.iter().enumerate() {
            if let Some(queue) = QueueId::new(index) {
                completed.raise(queue, timeline.completed.load(Ordering::Acquire));
            }
        }
        completed
    }
    /// `(poll calls, driver queries)` since creation.
    pub fn poll_stats(&self) -> (u64, u64) {
        (
            self.poll_calls.load(Ordering::Relaxed),
            self.driver_queries.load(Ordering::Relaxed),
        )
    }
    /// Query the device once and update the cached snapshot; same as [`Self::poll`].
    pub fn refresh(&self) -> Result<CompletedValues> {
        self.poll()
    }

    /// 非阻塞查询各队列完成值。只对仍有未完成提交的队列发起设备查询。
    pub fn poll(&self) -> Result<CompletedValues> {
        self.check_alive()?;
        self.poll_calls.fetch_add(1, Ordering::Relaxed);
        let mut completed = CompletedValues::default();
        for (index, timeline) in self.queues.iter().enumerate() {
            let queue = QueueId::new(index).expect("队列数已在构造时限制");
            let cached = timeline.completed.load(Ordering::Acquire);
            let value = if cached >= timeline.submitted.load(Ordering::Acquire) {
                cached
            } else {
                self.driver_queries.fetch_add(1, Ordering::Relaxed);
                let value = self.observe(self.api.counter_value(timeline.semaphore))?;
                timeline.completed.fetch_max(value, Ordering::AcqRel);
                value.max(cached)
            };
            completed.raise(queue, value);
        }
        Ok(completed)
    }

    /// 非阻塞判断 `set` 是否已全部完成。
    pub fn is_reached(&self, set: &SyncSet) -> Result<bool> {
        self.validate_submitted(set)?;
        Ok(set.is_reached(&self.poll()?))
    }

    /// 阻塞等待 `set` 全部完成；超时返回 [`Error::Timeout`]。
    pub fn wait(&self, set: &SyncSet, timeout_ns: u64) -> Result<()> {
        self.validate_submitted(set)?;
        let completed = self.poll()?;
        if set.is_reached(&completed) {
            return Ok(());
        }
        let mut semaphores = [vk::Semaphore::default(); MAX_QUEUES];
        let mut values = [0; MAX_QUEUES];
        let mut count = 0;
        for point in set
            .points()
            .filter(|point| point.value > completed.get(point.queue))
        {
            semaphores[count] = self.queues[point.queue.index()].semaphore;
            values[count] = point.value;
            count += 1;
        }
        if !self.observe(
            self.api
                .wait_all(&semaphores[..count], &values[..count], timeout_ns),
        )? {
            return Err(Error::Timeout);
        }
        for point in set.points() {
            self.queues[point.queue.index()]
                .completed
                .fetch_max(point.value, Ordering::AcqRel);
        }
        Ok(())
    }

    fn validate_submitted(&self, set: &SyncSet) -> Result<()> {
        for point in set.points() {
            let submitted = self.submitted(point.queue)?;
            if point.value > submitted {
                return Err(Error::invalid_usage(format!(
                    "等待队列 {} 的值 {}，但只提交到 {submitted}（会永远挂起）",
                    point.queue.index(),
                    point.value
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
