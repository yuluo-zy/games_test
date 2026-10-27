use super::*;
use crate::accounting::AllocatorStats;
use crate::tracking::{OwnershipRange, ResourceSharing};
use crate::tracking::range_state::{Frontier, NativeState, PatchAction, RangeStateConfig, RangeStateTable, StatePatch};
use crate::native::AccessScope;
use crate::sync::StagedDependencies;
use std::sync::Arc;

fn q(index: usize) -> QueueId {
    QueueId::new(index).unwrap()
}
fn semaphore(queue: QueueId) -> Result<vk::Semaphore> {
    Ok(ash::vk::Handle::from_raw(0x100 + queue.index() as u64))
}
/// A buffer last written on queue 1 at value 5, now used on queue 0.
fn written_on_other_queue() -> RangeStateTable {
    let mut writer = Frontier::EMPTY;
    writer.points.insert(SyncPoint {
        queue: q(1),
        value: 5,
    });
    writer.add(
        q(1),
        AccessScope {
            stages: vk::PipelineStageFlags::COMPUTE_SHADER,
            access: vk::AccessFlags::SHADER_WRITE,
            write: true,
        },
    );
    RangeStateTable::new(
        OwnershipRange::Buffer {
            offset: 0,
            size: 64,
        },
        ResourceSharing::Exclusive,
        NativeState {
            writer,
            ..NativeState::default()
        },
        RangeStateConfig::default(),
        Arc::new(AllocatorStats::default()),
    )
    .unwrap()
}
fn waits_for(action: PatchAction, planner: Option<vk::PipelineStageFlags>) -> PreparedSubmit {
    let mut table = written_on_other_queue();
    table
        .prepare(
            &[StatePatch {
                range: OwnershipRange::Buffer {
                    offset: 0,
                    size: 64,
                },
                expected_revision: if matches!(action, PatchAction::Unknown { .. }) {
                    u64::MAX
                } else {
                    0
                },
                action,
            }],
            q(0),
            0,
        )
        .unwrap();
    let mut dependencies = table.prepared_staged_dependencies();
    if let Some(stages) = planner {
        dependencies.insert(
            SyncPoint {
                queue: q(1),
                value: 5,
            },
            stages,
        );
    }
    let mut batch = PreparedSubmit::default();
    apply_staged_waits(&mut batch, q(0), &dependencies, semaphore).unwrap();
    batch
}

#[test]
fn planner_proven_cross_queue_dependency_waits_only_first_use_stages() {
    let batch = waits_for(
        PatchAction::Access {
            scope: AccessScope {
                stages: vk::PipelineStageFlags::FRAGMENT_SHADER,
                access: vk::AccessFlags::SHADER_READ,
                write: false,
            },
            layout: vk::ImageLayout::UNDEFINED,
            queue: q(0),
            family: 0,
        },
        Some(vk::PipelineStageFlags::FRAGMENT_SHADER),
    );
    assert_eq!(
        batch.wait_semaphores.as_slice(),
        &[semaphore(q(1)).unwrap()]
    );
    assert_eq!(batch.wait_values.as_slice(), &[5]);
    assert_eq!(
        batch.wait_stages.as_slice(),
        &[vk::PipelineStageFlags::FRAGMENT_SHADER]
    );
}

#[test]
fn opaque_cross_queue_dependency_waits_all_commands() {
    let batch = waits_for(
        PatchAction::Unknown {
            queue: q(0),
            family: 0,
        },
        None,
    );
    assert_eq!(batch.wait_values.as_slice(), &[5]);
    assert_eq!(
        batch.wait_stages.as_slice(),
        &[vk::PipelineStageFlags::ALL_COMMANDS]
    );
    // An opaque use widens a precise planner wait on the same semaphore.
    let batch = waits_for(
        PatchAction::Unknown {
            queue: q(0),
            family: 0,
        },
        Some(vk::PipelineStageFlags::FRAGMENT_SHADER),
    );
    assert_eq!(batch.wait_semaphores.len(), 1);
    assert!(batch.wait_stages[0].contains(vk::PipelineStageFlags::ALL_COMMANDS));
}

#[test]
fn same_queue_dependencies_add_no_wait_and_opaque_wrapper_is_conservative() {
    let mut dependencies = StagedDependencies::EMPTY;
    dependencies.insert(
        SyncPoint {
            queue: q(0),
            value: 3,
        },
        vk::PipelineStageFlags::TRANSFER,
    );
    let mut batch = PreparedSubmit::default();
    apply_staged_waits(&mut batch, q(0), &dependencies, semaphore).unwrap();
    assert!(batch.wait_semaphores.is_empty());
    let opaque = StagedDependencies::opaque(crate::sync::SyncSet::from_point(SyncPoint {
        queue: q(2),
        value: 1,
    }));
    assert_eq!(opaque.stages(q(2)), vk::PipelineStageFlags::ALL_COMMANDS);
    assert!(opaque.stages(q(1)).is_empty());
}
