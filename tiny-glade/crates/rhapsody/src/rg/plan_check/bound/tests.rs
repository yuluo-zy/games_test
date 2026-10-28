use super::*;
use crate::native::AccessScope;
use crate::rg::sync_plan::{AliasEdge, BarrierKind, BarrierPhase, BoundBatch, BoundPatch, BoundPatchAction, BoundTransition, BoundWait, DependencyRef, InitialState, RangeUse};
use crate::sync::QueueId;
use crate::tracking::OwnershipRange;
use crate::tracking::range_state::{NativeState, RangeOwner, RangeTransfer};
use ash::vk;
use crate::{rg::{queue_schedule::QueueDomain, sync_plan::{ProgramBatch, ProgramPass, ResourceDomain, SynchronizationConfig, SynchronizationPlanner}}, sync::SyncPoint};
use crate::tracking::{QueueFamilies, ResourceSharing};
use crate::tracking::range_state::RangeCell;
use crate::accounting::AllocatorStats;
use std::sync::Arc;

#[derive(Clone)]
struct Capture {
    initial: Vec<InitialState>,
    aliases: Vec<AliasEdge>,
    batches: Vec<BoundBatch>,
    order: Vec<usize>,
    transitions: Vec<BoundTransition>,
    patches: Vec<BoundPatch>,
    waits: Vec<BoundWait>,
}
impl Capture {
    fn check(&self, program: &RangeProgram) -> crate::Result<()> {
        check(
            program,
            View {
                initial: &self.initial,
                aliases: &self.aliases,
                batches: &self.batches,
                order: &self.order,
                transitions: &self.transitions,
                patches: &self.patches,
                waits: &self.waits,
            },
        )
    }
}
fn capture(program: &RangeProgram, initial: &[RangeCell]) -> Capture {
    let mut planner = SynchronizationPlanner::new(
        Arc::new(AllocatorStats::default()),
        SynchronizationConfig::default(),
    )
    .unwrap();
    planner.add_initial(0, initial).unwrap();
    planner
        .bind(program, |family| {
            Ok(QueueDomain {
                queue_index: family,
                family,
            })
        })
        .unwrap();
    check_bound_synchronization(program, planner.bound()).unwrap();
    let bound = planner.bound();
    Capture {
        initial: bound.initial_inputs().to_vec(),
        aliases: bound.aliases().to_vec(),
        batches: bound.batches().to_vec(),
        order: bound.execution_order().to_vec(),
        transitions: bound.transitions().to_vec(),
        patches: bound.patches().to_vec(),
        waits: bound.waits().to_vec(),
    }
}
fn point(queue: usize, value: u64) -> SyncPoint {
    SyncPoint {
        queue: QueueId::new(queue).unwrap(),
        value,
    }
}
fn scope(stages: vk::PipelineStageFlags, write: bool) -> AccessScope {
    AccessScope {
        stages,
        access: if write {
            vk::AccessFlags::SHADER_WRITE
        } else {
            vk::AccessFlags::SHADER_READ
        },
        write,
    }
}
fn program(
    range: OwnershipRange,
    sharing: ResourceSharing,
    entries: &[(u32, AccessScope, vk::ImageLayout)],
) -> RangeProgram {
    RangeProgram {
        resources: vec![ResourceDomain { range, sharing }],
        uses: entries
            .iter()
            .map(|(_, scope, layout)| RangeUse {
                resource: 0,
                range,
                scope: *scope,
                layout: *layout,
                access_code: 0x22,
            })
            .collect(),
        passes: (0..entries.len())
            .map(|pass| ProgramPass {
                pass,
                uses: pass..pass + 1,
            })
            .collect(),
        batches: entries
            .iter()
            .enumerate()
            .map(|(index, (queue, _, _))| ProgramBatch {
                domain: QueueDomain {
                    queue_index: *queue,
                    family: *queue,
                },
                passes: index..index + 1,
            })
            .collect(),
        ..RangeProgram::default()
    }
}
fn initial(range: OwnershipRange, queue: usize, layout: vk::ImageLayout) -> RangeCell {
    let mut state = NativeState {
        layout,
        owner: RangeOwner::Owned(queue as u32),
        ..NativeState::default()
    };
    state.writer.scopes[queue] = scope(vk::PipelineStageFlags::COMPUTE_SHADER, true);
    state.writer.points.insert(point(queue, 9));
    RangeCell { range, state }
}
#[test]
fn local_reader_stage_union_is_required_and_previous_visibility_can_remove_redundant_barriers()
{
    let range = OwnershipRange::Buffer {
        offset: 0,
        size: 16,
    };
    let make = |first| {
        program(
            range,
            ResourceSharing::Exclusive,
            &[
                (0, scope(first, false), vk::ImageLayout::UNDEFINED),
                (
                    0,
                    scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
                    vk::ImageLayout::UNDEFINED,
                ),
                (
                    0,
                    scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
                    vk::ImageLayout::UNDEFINED,
                ),
            ],
        )
    };
    let graph = make(vk::PipelineStageFlags::VERTEX_SHADER);
    let baseline = capture(&graph, &[initial(range, 0, vk::ImageLayout::UNDEFINED)]);
    let mut broken = baseline.clone();
    let last = broken
        .transitions
        .iter_mut()
        .find(|barrier| barrier.phase == BarrierPhase::BeforePass(2))
        .unwrap();
    last.source.stages &= !vk::PipelineStageFlags::FRAGMENT_SHADER;
    assert!(broken.check(&graph).is_err());

    let graph = make(vk::PipelineStageFlags::ALL_COMMANDS);
    let mut optimized = capture(&graph, &[initial(range, 0, vk::ImageLayout::UNDEFINED)]);
    optimized
        .transitions
        .retain(|barrier| barrier.phase != BarrierPhase::BeforePass(1));
    optimized.check(&graph).unwrap();
}
#[test]
fn every_foreign_reader_wait_and_its_destination_stages_are_checked() {
    let range = OwnershipRange::Buffer {
        offset: 0,
        size: 16,
    };
    let graph = program(
        range,
        ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1, 2]).unwrap()),
        &[
            (
                1,
                scope(vk::PipelineStageFlags::VERTEX_SHADER, false),
                vk::ImageLayout::UNDEFINED,
            ),
            (
                2,
                scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
                vk::ImageLayout::UNDEFINED,
            ),
            (
                0,
                scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
                vk::ImageLayout::UNDEFINED,
            ),
        ],
    );
    let source = capture(&graph, &[initial(range, 0, vk::ImageLayout::UNDEFINED)]);
    for reader in [0, 1] {
        let mut broken = source.clone();
        broken.waits.retain(|wait| {
            !(wait.batch == 2 && wait.dependency == DependencyRef::Batch(reader))
        });
        assert!(broken.check(&graph).is_err());
    }
    let mut broken = source.clone();
    broken
        .waits
        .iter_mut()
        .filter(|wait| wait.batch == 2)
        .for_each(|wait| wait.stages = vk::PipelineStageFlags::FRAGMENT_SHADER);
    assert!(broken.check(&graph).is_err());
    let mut broken = source;
    broken.order.swap(0, 2);
    assert!(broken.check(&graph).is_err());
}
#[test]
fn mixed_mip_layouts_missing_patch_ranges_and_stale_revisions_are_rejected() {
    let whole = OwnershipRange::Image {
        aspects: 1,
        base_mip: 0,
        mip_count: 2,
        base_layer: 0,
        layer_count: 2,
    };
    let graph = program(
        whole,
        ResourceSharing::Exclusive,
        &[(
            0,
            scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )],
    );
    let mip = |base_mip| OwnershipRange::Image {
        base_mip,
        mip_count: 1,
        aspects: 1,
        base_layer: 0,
        layer_count: 2,
    };
    let capture = capture(
        &graph,
        &[
            initial(mip(0), 0, vk::ImageLayout::GENERAL),
            initial(mip(1), 0, vk::ImageLayout::TRANSFER_SRC_OPTIMAL),
        ],
    );
    assert_eq!(capture.transitions.len(), 2);
    let mut broken = capture.clone();
    broken.transitions[0].old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
    assert!(broken.check(&graph).is_err());
    let mut broken = capture.clone();
    broken.patches.pop();
    assert!(broken.check(&graph).is_err());
    let mut broken = capture;
    broken.patches[1].expected_revision += 1;
    assert!(broken.check(&graph).is_err());
}
#[test]
fn failed_acquire_recovery_requires_original_pair_before_releasing_to_a_new_family() {
    let range = OwnershipRange::Buffer {
        offset: 0,
        size: 16,
    };
    let graph = program(
        range,
        ResourceSharing::Exclusive,
        &[(
            2,
            scope(vk::PipelineStageFlags::COMPUTE_SHADER, false),
            vk::ImageLayout::UNDEFINED,
        )],
    );
    let transfer = RangeTransfer {
        pair: 17,
        source_family: 0,
        destination_family: 1,
        range,
        source: scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
        destination: scope(vk::PipelineStageFlags::COMPUTE_SHADER, false),
        old_layout: vk::ImageLayout::UNDEFINED,
        new_layout: vk::ImageLayout::UNDEFINED,
    };
    let mut released = initial(range, 0, vk::ImageLayout::UNDEFINED);
    released.state.owner = RangeOwner::Released {
        transfer,
        point: point(0, 10),
    };
    released.state.revision = 7;
    let source = capture(&graph, &[released]);
    assert!(source.batches.len() >= 3);
    assert_ne!(source.order[0], 0);
    let mut broken = source.clone();
    broken
        .transitions
        .retain(|barrier| barrier.kind != BarrierKind::Acquire { pair: 17 });
    assert!(broken.check(&graph).is_err());
    let mut broken = source.clone();
    let acquire = broken
        .patches
        .iter_mut()
        .find(|patch| {
            patch.batch == 0 && matches!(patch.action, BoundPatchAction::Acquire { .. })
        })
        .unwrap();
    let BoundPatchAction::Acquire { transfer, .. } = acquire.action else {
        unreachable!()
    };
    acquire.action = BoundPatchAction::Acquire {
        transfer,
        release: DependencyRef::Point(point(0, 10)),
    };
    assert!(broken.check(&graph).is_err());
    let mut broken = source;
    broken
        .waits
        .retain(|wait| wait.dependency != DependencyRef::Point(point(0, 10)));
    assert!(broken.check(&graph).is_err());
}

/// Two buffers share memory: pass 0 writes A, pass 1 reads A, pass 2 writes
/// B, pass 3 reads B. `queues` assigns each pass its queue.
fn alias_capture(queues: [u32; 4], aliases: &[AliasEdge]) -> (RangeProgram, Capture) {
    let range = OwnershipRange::Buffer {
        offset: 0,
        size: 256,
    };
    let transfer = |write| AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: if write {
            vk::AccessFlags::TRANSFER_WRITE
        } else {
            vk::AccessFlags::TRANSFER_READ
        },
        write,
    };
    let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1]).unwrap());
    let mut program = RangeProgram {
        resources: vec![ResourceDomain { range, sharing }; 2],
        ..RangeProgram::default()
    };
    for (pass, (resource, write)) in [(0, true), (0, false), (1, true), (1, false)]
        .into_iter()
        .enumerate()
    {
        program.uses.push(RangeUse {
            resource,
            range,
            scope: transfer(write),
            layout: vk::ImageLayout::UNDEFINED,
            access_code: 0x34,
        });
        program.passes.push(ProgramPass {
            pass,
            uses: pass..pass + 1,
        });
        program.batches.push(ProgramBatch {
            domain: QueueDomain {
                queue_index: queues[pass],
                family: queues[pass],
            },
            passes: pass..pass + 1,
        });
    }
    program.dependencies = vec![(0, 1), (1, 2), (2, 3)];
    let mut planner = SynchronizationPlanner::new(
        Arc::new(AllocatorStats::default()),
        SynchronizationConfig::default(),
    )
    .unwrap();
    for resource in 0..2 {
        planner
            .add_initial(
                resource,
                &[RangeCell {
                    range,
                    state: NativeState::default(),
                }],
            )
            .unwrap();
    }
    for edge in aliases {
        planner.add_alias(edge.previous, edge.next).unwrap();
    }
    planner
        .bind(&program, |family| {
            Ok(QueueDomain {
                queue_index: family,
                family,
            })
        })
        .unwrap();
    check_bound_synchronization(&program, planner.bound()).unwrap();
    let bound = planner.bound();
    let capture = Capture {
        initial: bound.initial_inputs().to_vec(),
        aliases: bound.aliases().to_vec(),
        batches: bound.batches().to_vec(),
        order: bound.execution_order().to_vec(),
        transitions: bound.transitions().to_vec(),
        patches: bound.patches().to_vec(),
        waits: bound.waits().to_vec(),
    };
    (program, capture)
}
const ALIAS: [AliasEdge; 2] = [
    AliasEdge {
        previous: 0,
        next: 1,
    },
    AliasEdge {
        previous: 1,
        next: 0,
    },
];

#[test]
fn aliased_buffer_first_write_waits_for_the_previous_member() {
    let (_, plain) = alias_capture([0; 4], &[]);
    assert!(
        !plain
            .transitions
            .iter()
            .any(|t| t.resource == 1 && t.phase == BarrierPhase::BeforePass(2)),
        "a fresh unaliased buffer needs no barrier"
    );
    let (program, source) = alias_capture([0; 4], &ALIAS);
    let alias = source
        .transitions
        .iter()
        .find(|t| t.resource == 1 && t.phase == BarrierPhase::BeforePass(2))
        .expect("alias barrier");
    assert!(
        alias
            .source
            .stages
            .contains(vk::PipelineStageFlags::TRANSFER)
    );
    assert!(alias.source.access.contains(vk::AccessFlags::TRANSFER_READ));
    assert!(
        alias
            .source
            .access
            .contains(vk::AccessFlags::TRANSFER_WRITE)
    );
    // The checker independently requires the alias barrier.
    let mut broken = source.clone();
    broken
        .transitions
        .retain(|t| !(t.resource == 1 && t.phase == BarrierPhase::BeforePass(2)));
    assert!(broken.check(&program).is_err());
    let mut broken = source;
    for transition in &mut broken.transitions {
        if transition.resource == 1 && transition.phase == BarrierPhase::BeforePass(2) {
            transition.source = AccessScope::EMPTY;
        }
    }
    assert!(broken.check(&program).is_err());
}

#[test]
fn cross_queue_alias_waits_at_the_first_use_stage() {
    let (program, source) = alias_capture([0, 0, 1, 1], &ALIAS);
    let wait = source
        .waits
        .iter()
        .find(|wait| wait.batch == 2)
        .expect("alias wait on the other queue");
    assert_eq!(wait.stages, vk::PipelineStageFlags::TRANSFER);
    assert_eq!(wait.dependency, DependencyRef::Batch(1));
    let mut broken = source;
    broken.waits.retain(|wait| wait.batch != 2);
    assert!(broken.check(&program).is_err());
}
