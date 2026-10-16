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

/// 时间线相关的设备调用；真实实现为 `ash::Device`，测试可替换。
pub trait TimelineApi: Send + Sync {
    fn counter_value(&self, semaphore: vk::Semaphore) -> Result<u64>;
    /// 等待全部信号量到达对应值。到达返回 `Ok(true)`，超时返回 `Ok(false)`。
    fn wait_all(&self, semaphores: &[vk::Semaphore], values: &[u64], timeout_ns: u64) -> Result<bool>;
}

impl TimelineApi for ash::Device {
    fn counter_value(&self, semaphore: vk::Semaphore) -> Result<u64> {
        Ok(unsafe { self.get_semaphore_counter_value(semaphore) }?)
    }
    fn wait_all(&self, semaphores: &[vk::Semaphore], values: &[u64], timeout_ns: u64) -> Result<bool> {
        let info = vk::SemaphoreWaitInfo::default()
            .semaphores(semaphores)
            .values(values);
        match unsafe { self.wait_semaphores(&info, timeout_ns) } {
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
}

pub struct Timelines<A> {
    api: A,
    queues: Vec<QueueTimeline>,
    device_lost: AtomicBool,
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
                })
                .collect(),
            device_lost: AtomicBool::new(false),
        })
    }

    pub fn api(&self) -> &A {
        &self.api
    }
    pub fn queue_count(&self) -> usize {
        self.queues.len()
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
        let mut last = timeline.submit.lock();
        let value = *last + 1;
        let output = self.observe(submit(SubmitSignal {
            semaphore: timeline.semaphore,
            value,
        }))?;
        *last = value;
        timeline.submitted.store(value, Ordering::Release);
        Ok((SyncPoint { queue, value }, output))
    }

    /// 在该队列的提交锁内执行不 signal 时间线的队列操作（例如 `vkQueuePresentKHR`）。
    pub fn with_queue<R>(&self, queue: QueueId, operation: impl FnOnce() -> Result<R>) -> Result<R> {
        self.check_alive()?;
        let _guard = self.queue(queue)?.submit.lock();
        self.observe(operation())
    }

    /// 该队列最后一次成功提交的值（0 = 尚无提交）。
    pub fn submitted(&self, queue: QueueId) -> Result<u64> {
        Ok(self.queue(queue)?.submitted.load(Ordering::Acquire))
    }

    /// 非阻塞查询各队列完成值。只对仍有未完成提交的队列发起设备查询。
    pub fn poll(&self) -> Result<CompletedValues> {
        self.check_alive()?;
        let mut completed = CompletedValues::default();
        for (index, timeline) in self.queues.iter().enumerate() {
            let queue = QueueId::new(index).expect("队列数已在构造时限制");
            let cached = timeline.completed.load(Ordering::Acquire);
            let value = if cached >= timeline.submitted.load(Ordering::Acquire) {
                cached
            } else {
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
        let (semaphores, values): (Vec<_>, Vec<_>) = set
            .points()
            .filter(|point| point.value > completed.get(point.queue))
            .map(|point| (self.queues[point.queue.index()].semaphore, point.value))
            .unzip();
        if !self.observe(self.api.wait_all(&semaphores, &values, timeout_ns))? {
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
mod tests {
    use super::*;
    use crate::error::ErrorKind;
    use ash::vk::Handle;
    use std::collections::HashMap;

    /// 用 CPU 计数器模拟时间线：`complete` 推进完成值，`wait_all` 只看当前值。
    #[derive(Default)]
    struct FakeApi {
        counters: Mutex<HashMap<u64, u64>>,
        lost: AtomicBool,
        queries: AtomicU64,
    }
    impl FakeApi {
        fn complete(&self, semaphore: vk::Semaphore, value: u64) {
            self.counters.lock().insert(semaphore.as_raw(), value);
        }
    }
    impl TimelineApi for FakeApi {
        fn counter_value(&self, semaphore: vk::Semaphore) -> Result<u64> {
            self.queries.fetch_add(1, Ordering::Relaxed);
            if self.lost.load(Ordering::Relaxed) {
                return Err(vk::Result::ERROR_DEVICE_LOST.into());
            }
            Ok(*self.counters.lock().get(&semaphore.as_raw()).unwrap_or(&0))
        }
        fn wait_all(&self, semaphores: &[vk::Semaphore], values: &[u64], _: u64) -> Result<bool> {
            let counters = self.counters.lock();
            Ok(semaphores
                .iter()
                .zip(values)
                .all(|(s, v)| counters.get(&s.as_raw()).unwrap_or(&0) >= v))
        }
    }

    fn sem(raw: u64) -> vk::Semaphore {
        vk::Semaphore::from_raw(raw)
    }
    fn q(index: usize) -> QueueId {
        QueueId::new(index).unwrap()
    }
    fn timelines(count: u64) -> Timelines<FakeApi> {
        let semaphores: Vec<_> = (1..=count).map(sem).collect();
        Timelines::new(FakeApi::default(), &semaphores).unwrap()
    }

    #[test]
    fn values_are_committed_only_after_a_successful_submit() {
        let t = timelines(1);
        let (point, ()) = t.submit(q(0), |signal| {
            assert_eq!(signal, SubmitSignal { semaphore: sem(1), value: 1 });
            Ok(())
        })
        .unwrap();
        assert_eq!(point.value, 1);

        let failed = t.submit(q(0), |signal| {
            assert_eq!(signal.value, 2);
            Err::<(), _>(vk::Result::ERROR_OUT_OF_HOST_MEMORY.into())
        });
        assert_eq!(failed.unwrap_err().kind(), ErrorKind::OutOfMemory);
        assert_eq!(t.submitted(q(0)).unwrap(), 1, "失败的提交不占用值");

        let (point, ()) = t.submit(q(0), |signal| {
            assert_eq!(signal.value, 2, "失败后复用同一个值");
            Ok(())
        })
        .unwrap();
        assert_eq!(point.value, 2);
    }

    #[test]
    fn waiting_on_an_unsubmitted_value_is_rejected_instead_of_hanging() {
        let t = timelines(1);
        let future = SyncSet::from_point(SyncPoint { queue: q(0), value: 1 });
        assert_eq!(t.wait(&future, u64::MAX).unwrap_err().kind(), ErrorKind::InvalidUsage);
        assert_eq!(t.is_reached(&future).unwrap_err().kind(), ErrorKind::InvalidUsage);
    }

    #[test]
    fn poll_and_wait_track_each_queue_independently() {
        let t = timelines(2);
        let (graphics, ()) = t.submit(q(0), |_| Ok(())).unwrap();
        let (compute, ()) = t.submit(q(1), |_| Ok(())).unwrap();
        let mut both = SyncSet::from_point(graphics);
        both.insert(compute);

        assert!(!t.is_reached(&both).unwrap());
        t.api().complete(sem(1), 1);
        assert!(!t.is_reached(&both).unwrap(), "计算队列未完成");
        assert_eq!(t.wait(&both, 0).unwrap_err().kind(), ErrorKind::Timeout);
        t.api().complete(sem(2), 1);
        t.wait(&both, 0).unwrap();
        assert!(t.is_reached(&both).unwrap());
    }

    #[test]
    fn poll_skips_queues_without_outstanding_work() {
        let t = timelines(3);
        t.poll().unwrap();
        assert_eq!(t.api().queries.load(Ordering::Relaxed), 0, "无提交时不查询设备");
        t.submit(q(1), |_| Ok(())).unwrap();
        t.poll().unwrap();
        assert_eq!(t.api().queries.load(Ordering::Relaxed), 1);
        t.api().complete(sem(2), 1);
        t.poll().unwrap();
        t.poll().unwrap();
        assert_eq!(t.api().queries.load(Ordering::Relaxed), 2, "完成值缓存后不再查询");
    }

    #[test]
    fn device_loss_is_sticky() {
        let t = timelines(1);
        let lost = t.submit(q(0), |_| Err::<(), _>(vk::Result::ERROR_DEVICE_LOST.into()));
        assert_eq!(lost.unwrap_err().kind(), ErrorKind::DeviceLost);
        assert!(t.is_device_lost());
        assert_eq!(t.submit(q(0), |_| Ok(())).unwrap_err().kind(), ErrorKind::DeviceLost);
        assert_eq!(t.poll().unwrap_err().kind(), ErrorKind::DeviceLost);
        assert_eq!(t.with_queue(q(0), || Ok(())).unwrap_err().kind(), ErrorKind::DeviceLost);
    }

    #[test]
    fn device_loss_observed_by_a_query_is_sticky_too() {
        let t = timelines(1);
        t.submit(q(0), |_| Ok(())).unwrap();
        t.api().lost.store(true, Ordering::Relaxed);
        assert_eq!(t.poll().unwrap_err().kind(), ErrorKind::DeviceLost);
        assert!(t.is_device_lost());
    }

    #[test]
    fn queues_and_counts_are_validated() {
        assert!(Timelines::new(FakeApi::default(), &[]).is_err());
        let too_many: Vec<_> = (1..=MAX_QUEUES as u64 + 1).map(sem).collect();
        assert!(Timelines::new(FakeApi::default(), &too_many).is_err());
        let t = timelines(1);
        assert_eq!(t.submit(q(1), |_| Ok(())).unwrap_err().kind(), ErrorKind::InvalidUsage);
    }

    #[test]
    fn submits_on_different_queues_do_not_block_each_other() {
        let t = std::sync::Arc::new(timelines(2));
        let (entered, release) = (std::sync::Barrier::new(2), std::sync::Barrier::new(2));
        std::thread::scope(|scope| {
            scope.spawn(|| {
                t.submit(q(0), |_| {
                    entered.wait();
                    release.wait();
                    Ok(())
                })
                .unwrap();
            });
            entered.wait();
            // 队列 0 的提交锁仍被持有；队列 1 必须能立即提交。
            t.submit(q(1), |_| Ok(())).unwrap();
            release.wait();
        });
        assert_eq!((t.submitted(q(0)).unwrap(), t.submitted(q(1)).unwrap()), (1, 1));
    }
}
