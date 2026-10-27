use super::*;
use crate::Error;
use ash::vk;
#[test]
fn pending_descriptor_pool_owners_keep_the_global_quota_charged() {
    let stats = std::sync::Arc::new(AllocatorStats::default());
    let first = stats.reserve_update_after_bind_descriptors(6, 10).unwrap();
    assert!(stats.reserve_update_after_bind_descriptors(5, 10).is_err());
    let second = stats.reserve_update_after_bind_descriptors(4, 10).unwrap();
    drop(first);
    let third = stats.reserve_update_after_bind_descriptors(6, 10).unwrap();
    assert!(stats.reserve_update_after_bind_descriptors(1, 10).is_err());
    drop((second, third));
    assert!(stats.reserve_update_after_bind_descriptors(10, 10).is_ok());
}
use crate::sync::{CompletedValues, QueueId, SyncPoint};
use crate::accounting::BudgetCategory;
#[test]
fn allocation_budget_tracks_real_lifetime_and_rejects_overcommit_atomically() {
    let stats = AllocatorStats::default();
    stats.set_total_budget(100).unwrap();
    let id = stats
        .register(
            60,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "resource".into(),
        )
        .unwrap();
    assert_eq!(stats.budget_usage().live, 60);
    let point = SyncPoint {
        queue: QueueId::new(0).unwrap(),
        value: 3,
    };
    stats.record_gpu_use(id, point);
    assert_eq!(stats.budget_usage().in_flight, 60);
    assert!(matches!(
        stats.register(
            50,
            0,
            0,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            "over budget".into()
        ),
        Err(Error::OutOfDeviceMemory)
    ));
    assert_eq!(stats.total_registered(), 1);
    assert_eq!(stats.report().allocations.len(), 1);
    let mut completed = CompletedValues::default();
    completed.raise(point.queue, 2);
    stats.observe_completed(&completed);
    stats.mark_cached(id, &completed);
    stats.mark_live(id, &completed);
    assert_eq!(stats.budget_usage().in_flight, 60);
    completed.raise(point.queue, 3);
    stats.observe_completed(&completed);
    assert_eq!(stats.budget_usage().live, 60);
    stats.mark_cached(id, &completed);
    assert_eq!(stats.budget_usage().cache, 60);
    stats.mark_live(id, &completed);
    assert_eq!(stats.budget_usage().live, 60);
    assert_eq!(
        stats.state.lock().phases[&id].last_use.points().next(),
        Some(point)
    );
    stats.mark_cached(id, &completed);
    stats.mark_retiring(id);
    stats.observe_completed(&completed);
    stats.mark_cached(id, &completed);
    stats.mark_live(id, &completed);
    assert_eq!(
        stats.state.lock().phases[&id].category,
        BudgetCategory::Retiring
    );
    assert_eq!(stats.budget_usage().retiring, 60);
    stats.release(id).unwrap();
    assert_eq!(stats.budget_usage().total(), 0);
}
#[test]
fn completion_moves_only_reached_in_flight_uses_across_queues() {
    let stats = AllocatorStats::default();
    let register = || {
        stats
            .register(
                10,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                String::new(),
            )
            .unwrap()
    };
    let (a, b, c) = (register(), register(), register());
    let (q0, q1) = (QueueId::new(0).unwrap(), QueueId::new(1).unwrap());
    // Out-of-order publication on one queue and a two-queue dependency.
    stats.record_gpu_use(
        a,
        SyncPoint {
            queue: q0,
            value: 5,
        },
    );
    stats.record_gpu_use(
        b,
        SyncPoint {
            queue: q0,
            value: 2,
        },
    );
    stats.record_gpu_use(
        c,
        SyncPoint {
            queue: q0,
            value: 1,
        },
    );
    stats.record_gpu_use(
        c,
        SyncPoint {
            queue: q1,
            value: 7,
        },
    );
    assert_eq!(stats.budget_usage().in_flight, 30);
    let mut completed = CompletedValues::default();
    completed.raise(q0, 2);
    stats.observe_completed(&completed);
    assert_eq!(
        stats.budget_usage().in_flight,
        20,
        "b reached; c waits for q1"
    );
    assert_eq!(stats.state.lock().in_flight[0].len(), 1);
    stats.observe_completed(&completed);
    completed.raise(q1, 7);
    stats.observe_completed(&completed);
    assert_eq!(stats.budget_usage().in_flight, 10);
    // A use already covered by the observed snapshot stays live.
    stats.record_gpu_use(
        b,
        SyncPoint {
            queue: q0,
            value: 2,
        },
    );
    assert_eq!(stats.budget_usage().in_flight, 10);
    completed.raise(q0, 5);
    stats.observe_completed(&completed);
    assert_eq!(stats.budget_usage().live, 30);
    for id in [a, b, c] {
        stats.release(id).unwrap();
    }
}
