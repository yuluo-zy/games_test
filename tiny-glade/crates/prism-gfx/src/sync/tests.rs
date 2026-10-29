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
fn host_resource_dependencies_wait_at_device_stages() {
    use ash::vk::PipelineStageFlags as Stages;
    let point = SyncPoint {
        queue: q(0),
        value: 7,
    };
    let mut dependencies = StagedDependencies::EMPTY;
    dependencies.insert(point, Stages::HOST);
    assert_eq!(dependencies.stages(point.queue), Stages::ALL_COMMANDS);
    let mut combined = StagedDependencies::EMPTY;
    combined.insert(point, Stages::HOST | Stages::TRANSFER);
    assert_eq!(combined.stages(point.queue), Stages::TRANSFER);
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
