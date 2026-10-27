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
    let (point, ()) = t
        .submit(q(0), |signal| {
            assert_eq!(
                signal,
                SubmitSignal {
                    semaphore: sem(1),
                    value: 1
                }
            );
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

    let (point, ()) = t
        .submit(q(0), |signal| {
            assert_eq!(signal.value, 2, "失败后复用同一个值");
            Ok(())
        })
        .unwrap();
    assert_eq!(point.value, 2);
}

#[test]
fn waiting_on_an_unsubmitted_value_is_rejected_instead_of_hanging() {
    let t = timelines(1);
    let future = SyncSet::from_point(SyncPoint {
        queue: q(0),
        value: 1,
    });
    assert_eq!(
        t.wait(&future, u64::MAX).unwrap_err().kind(),
        ErrorKind::InvalidUsage
    );
    assert_eq!(
        t.is_reached(&future).unwrap_err().kind(),
        ErrorKind::InvalidUsage
    );
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
    assert_eq!(
        t.api().queries.load(Ordering::Relaxed),
        0,
        "无提交时不查询设备"
    );
    t.submit(q(1), |_| Ok(())).unwrap();
    t.poll().unwrap();
    assert_eq!(t.api().queries.load(Ordering::Relaxed), 1);
    t.api().complete(sem(2), 1);
    t.poll().unwrap();
    t.poll().unwrap();
    assert_eq!(
        t.api().queries.load(Ordering::Relaxed),
        2,
        "完成值缓存后不再查询"
    );
}

#[test]
fn cached_completion_never_queries_and_refresh_updates_it() {
    let t = timelines(2);
    t.submit(q(0), |_| Ok(())).unwrap();
    t.api().complete(sem(1), 1);
    assert_eq!(
        t.completed().get(q(0)),
        0,
        "cached snapshot is conservative"
    );
    assert_eq!(t.api().queries.load(Ordering::Relaxed), 0);
    assert_eq!(t.refresh().unwrap().get(q(0)), 1);
    let queries = t.api().queries.load(Ordering::Relaxed);
    assert_eq!(t.completed().get(q(0)), 1);
    assert_eq!(t.api().queries.load(Ordering::Relaxed), queries);
}

#[test]
fn queue_lock_statistics_are_opt_in() {
    let t = timelines(1);
    t.submit(q(0), |_| Ok(())).unwrap();
    assert_eq!(t.queue_lock_stats()[0].unwrap().acquisitions, 0);
    t.set_lock_statistics(true);
    t.submit(q(0), |_| Ok(())).unwrap();
    assert_eq!(t.queue_lock_stats()[0].unwrap().acquisitions, 1);
}

#[test]
fn device_loss_is_sticky() {
    let t = timelines(1);
    let lost = t.submit(q(0), |_| Err::<(), _>(vk::Result::ERROR_DEVICE_LOST.into()));
    assert_eq!(lost.unwrap_err().kind(), ErrorKind::DeviceLost);
    assert!(t.is_device_lost());
    assert_eq!(
        t.submit(q(0), |_| Ok(())).unwrap_err().kind(),
        ErrorKind::DeviceLost
    );
    assert_eq!(t.poll().unwrap_err().kind(), ErrorKind::DeviceLost);
    assert_eq!(
        t.with_queue(q(0), || Ok(())).unwrap_err().kind(),
        ErrorKind::DeviceLost
    );
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
    assert_eq!(
        t.submit(q(1), |_| Ok(())).unwrap_err().kind(),
        ErrorKind::InvalidUsage
    );
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
    assert_eq!(
        (t.submitted(q(0)).unwrap(), t.submitted(q(1)).unwrap()),
        (1, 1)
    );
}
