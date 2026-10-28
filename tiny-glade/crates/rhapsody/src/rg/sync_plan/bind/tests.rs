use super::*;
use crate::tracking::range_state::{Frontier, RangeOwner};
fn buffer(offset: u64, size: u64) -> NativeRange {
    NativeRange::Buffer { offset, size }
}
fn queue(index: u32, family: u32) -> QueueDomain {
    QueueDomain {
        queue_index: index,
        family,
    }
}
fn usage(range: NativeRange, write: bool) -> RangeUse {
    RangeUse {
        resource: 0,
        range,
        scope: AccessScope {
            stages: vk::PipelineStageFlags::TRANSFER,
            access: if write {
                vk::AccessFlags::TRANSFER_WRITE
            } else {
                vk::AccessFlags::TRANSFER_READ
            },
            write,
        },
        layout: vk::ImageLayout::UNDEFINED,
        access_code: if write { 0x34 } else { 0x25 },
    }
}
fn program(domain: NativeRange, uses: Vec<RangeUse>, domains: &[QueueDomain]) -> RangeProgram {
    let count = uses.len();
    RangeProgram {
        resources: vec![ResourceDomain {
            range: domain,
            sharing: ResourceSharing::Exclusive,
        }],
        uses,
        passes: (0..count)
            .map(|pass| ProgramPass {
                pass,
                uses: pass..pass + 1,
            })
            .collect(),
        batches: domains
            .iter()
            .enumerate()
            .map(|(pass, &domain)| ProgramBatch {
                domain,
                passes: pass..pass + 1,
            })
            .collect(),
        ..Default::default()
    }
}
#[test]
fn disjoint_unclaimed_ranges_need_no_cross_queue_or_ownership_wait() {
    let stats = Arc::new(AllocatorStats::default());
    let program = program(
        buffer(0, 16),
        vec![usage(buffer(0, 8), true), usage(buffer(8, 8), true)],
        &[queue(0, 0), queue(1, 1)],
    );
    let mut planner = SynchronizationPlanner::new(stats, Default::default()).unwrap();
    planner
        .add_initial(
            0,
            &[RangeCell {
                range: buffer(0, 16),
                state: NativeState::default(),
            }],
        )
        .unwrap();
    planner
        .bind(&program, |family| Ok(queue(family, family)))
        .unwrap();
    assert_eq!(planner.bound().execution_order(), &[0, 1]);
    assert!(planner.bound().waits().is_empty());
    assert!(planner.bound().transitions().is_empty());
    assert_eq!(planner.bound().patches()[0].range, buffer(0, 8));
    assert_eq!(planner.bound().patches()[1].range, buffer(8, 8));
}
#[test]
fn exclusive_transfer_is_exact_and_acquire_refers_to_actual_release_batch() {
    let program = program(
        buffer(0, 16),
        vec![usage(buffer(8, 8), true)],
        &[queue(1, 1)],
    );
    let mut planner =
        SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
            .unwrap();
    planner
        .add_initial(
            0,
            &[RangeCell {
                range: buffer(0, 16),
                state: NativeState {
                    owner: RangeOwner::Owned(0),
                    ..NativeState::default()
                },
            }],
        )
        .unwrap();
    planner
        .bind(&program, |family| Ok(queue(family, family)))
        .unwrap();
    assert_eq!(planner.bound().execution_order(), &[1, 0]);
    let transitions = planner.bound().transitions();
    assert_eq!(transitions[0].range, buffer(8, 8));
    assert_eq!(transitions[1].range, buffer(8, 8));
    assert!(matches!(
        planner.bound().patches()[1].action,
        BoundPatchAction::Acquire {
            release: DependencyRef::Batch(1),
            ..
        }
    ));
    assert!(planner.bound().patches()[1].resolve(|_| None).is_err());
    let real = SyncPoint {
        queue: QueueId::new(0).unwrap(),
        value: 17,
    };
    assert!(
        matches!(planner.bound().patches()[1].resolve(|_|Some(real)).unwrap().action,PatchAction::Acquire {release_point,..} if release_point==real)
    );
}
#[test]
fn cross_queue_layout_transition_and_acquire_chain_with_their_precise_waits() {
    let image = NativeRange::Image {
        aspects: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip: 0,
        mip_count: 1,
        base_layer: 0,
        layer_count: 1,
    };
    let fragment = AccessScope {
        stages: vk::PipelineStageFlags::FRAGMENT_SHADER,
        access: vk::AccessFlags::SHADER_READ,
        write: false,
    };
    let writes = RangeUse {
        resource: 0,
        range: image,
        scope: AccessScope {
            stages: vk::PipelineStageFlags::COMPUTE_SHADER,
            access: vk::AccessFlags::SHADER_WRITE,
            write: true,
        },
        layout: vk::ImageLayout::GENERAL,
        access_code: 0x34,
    };
    let reads = RangeUse {
        scope: fragment,
        layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        access_code: 0x25,
        ..writes
    };
    for (sharing, acquire) in [
        (
            ResourceSharing::Concurrent(crate::tracking::QueueFamilies::new(&[0, 1]).unwrap()),
            false,
        ),
        (ResourceSharing::Exclusive, true),
    ] {
        let mut program = program(image, vec![writes, reads], &[queue(1, 1), queue(0, 0)]);
        program.resources[0].sharing = sharing;
        let mut planner = SynchronizationPlanner::new(
            Arc::new(AllocatorStats::default()),
            Default::default(),
        )
        .unwrap();
        planner
            .add_initial(
                0,
                &[RangeCell {
                    range: image,
                    state: NativeState::default(),
                }],
            )
            .unwrap();
        planner
            .bind(&program, |family| Ok(queue(family, family)))
            .unwrap();
        let bound = planner.bound();
        let wait = bound
            .waits()
            .iter()
            .find(|wait| wait.batch == 1)
            .expect("cross-queue wait");
        assert_eq!(wait.stages, vk::PipelineStageFlags::FRAGMENT_SHADER);
        let consumer = bound
            .transitions()
            .iter()
            .filter(|transition| transition.batch == 1)
            .collect::<Vec<_>>();
        assert!(!consumer.is_empty());
        assert_eq!(
            consumer
                .iter()
                .any(|transition| matches!(transition.kind, BarrierKind::Acquire { .. })),
            acquire
        );
        for transition in consumer {
            // Every barrier on the consumer queue chains with the wait.
            assert!(
                transition.source.stages.contains(wait.stages),
                "{transition:?} does not chain with {wait:?}"
            );
        }
    }
}
/// Two exclusive alias members of one backing used on family 0. Member 1
/// was released to family 1 by an earlier frame whose acquire never ran.
fn alias_after_failed_release(reset: bool) -> Result<SynchronizationPlanner> {
    use crate::tracking::SyncCell;
    use crate::tracking::range_state::{PatchAction, StatePatch};
    let stats = Arc::new(AllocatorStats::default());
    let range = buffer(0, 64);
    let q1 = QueueId::new(1).unwrap();
    let cell = Arc::new(
        SyncCell::for_device(crate::sync::DeviceGeneration::next())
            .with_ownership_range(range),
    );
    cell.with_range_state(NativeState::default(), &stats, |_| Ok(()))
        .unwrap();
    let transfer = RangeTransfer {
        pair: 7,
        source_family: 1,
        destination_family: 0,
        range,
        source: usage(range, true).scope,
        destination: usage(range, false).scope,
        old_layout: vk::ImageLayout::UNDEFINED,
        new_layout: vk::ImageLayout::UNDEFINED,
    };
    cell.prepare_range_state(
        &[
            StatePatch {
                range,
                expected_revision: 0,
                action: PatchAction::Access {
                    scope: usage(range, true).scope,
                    layout: vk::ImageLayout::UNDEFINED,
                    queue: q1,
                    family: 1,
                },
            },
            StatePatch {
                range,
                expected_revision: 1,
                action: PatchAction::Release(transfer),
            },
        ],
        q1,
        1,
        &stats,
    )
    .unwrap();
    let release = SyncPoint {
        queue: q1,
        value: 5,
    };
    cell.publish_range_state(release);
    cell.insert_overall(release);
    let mut completed = crate::sync::CompletedValues::default();
    // Still executing: the member must not be reset (or returned) yet.
    assert!(!cell.discard_idle_range_state(&completed));
    cell.with_range_state(NativeState::default(), &stats, |table| {
        assert!(matches!(
            table.cells()[0].state.owner,
            RangeOwner::Released { .. }
        ));
        Ok(())
    })
    .unwrap();
    if reset {
        completed.raise(q1, 5);
        assert!(cell.discard_idle_range_state(&completed));
    }
    let initial = cell
        .with_range_state(NativeState::default(), &stats, |table| {
            Ok(table.cells().to_vec())
        })
        .unwrap();
    let mut program = program(
        range,
        vec![usage(range, true), usage(range, true)],
        &[queue(0, 0), queue(0, 0)],
    );
    program.resources.push(program.resources[0]);
    program.uses[1].resource = 1;
    program.dependencies = vec![(0, 1)];
    let mut planner = SynchronizationPlanner::new(stats, Default::default()).unwrap();
    planner
        .add_initial(
            0,
            &[RangeCell {
                range,
                state: NativeState::default(),
            }],
        )
        .unwrap();
    planner.add_initial(1, &initial).unwrap();
    planner.add_alias(0, 1).unwrap();
    planner.add_alias(1, 0).unwrap();
    planner.bind(&program, |family| Ok(queue(family, family)))?;
    Ok(planner)
}
#[test]
fn alias_member_released_by_a_failed_frame_is_discarded_without_stale_acquire() {
    let planner = alias_after_failed_release(true).unwrap();
    let bound = planner.bound();
    assert!(
        bound
            .batches()
            .iter()
            .all(|batch| matches!(batch.kind, BoundBatchKind::Graph { .. })),
        "no ownership prelude batch"
    );
    assert!(
        bound.transitions().iter().all(|transition| matches!(
            transition.kind,
            BarrierKind::Memory
        ) && transition.source_family
            == vk::QUEUE_FAMILY_IGNORED),
        "no ownership transfer barrier"
    );
    assert!(
        bound
            .patches()
            .iter()
            .all(|patch| matches!(patch.action, BoundPatchAction::Access { .. }))
    );
    // The completed release is neither waited on nor carried forward.
    assert!(
        bound
            .waits()
            .iter()
            .all(|wait| !matches!(wait.dependency, DependencyRef::Point(_)))
    );
    // An unreset stale release is rejected instead of acquired.
    let Err(error) = alias_after_failed_release(false) else {
        panic!("stale release must not bind");
    };
    assert!(error.to_string().contains("alias member"), "{error}");
}
#[test]
fn three_alias_members_preserve_each_previous_writer_and_the_wrap_edge() {
    let range = buffer(0, 16);
    let mut program = program(range, vec![usage(range, true); 3], &[queue(0, 0); 3]);
    program.resources.extend([program.resources[0]; 2]);
    for (index, access) in program.uses.iter_mut().enumerate() { access.resource = index as u32; }
    program.uses[1].scope = AccessScope { stages: vk::PipelineStageFlags::COMPUTE_SHADER,
        access: vk::AccessFlags::SHADER_WRITE, write: true };
    program.dependencies = vec![(0, 1), (1, 2)];
    let mut planner = SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default()).unwrap();
    for resource in 0..3 {
        let mut state = NativeState::default();
        if resource == 2 {
            state.writer.points.insert(SyncPoint { queue: QueueId::new(0).unwrap(), value: 8 });
            state.writer.add(QueueId::new(0).unwrap(), usage(range, true).scope);
        }
        planner.add_initial(resource, &[RangeCell { range, state }]).unwrap();
    }
    for (previous, next) in [(0, 1), (1, 2), (2, 0)] { planner.add_alias(previous, next).unwrap(); }
    planner.bind(&program, |family| Ok(queue(0, family))).unwrap();
    for (resource, source) in [(0, vk::AccessFlags::TRANSFER_WRITE), (1, vk::AccessFlags::TRANSFER_WRITE), (2, vk::AccessFlags::SHADER_WRITE)] {
        assert!(planner.bound().transitions().iter().any(|transition|
            transition.resource == resource && transition.source.access.contains(source)));
    }
    crate::rg::plan_check::check_bound_synchronization(&program, planner.bound()).unwrap();
}

#[test]
fn released_recovery_first_acquires_the_original_destination_then_moves_again() {
    let range = buffer(0, 16);
    let transfer = RangeTransfer {
        pair: 31,
        source_family: 0,
        destination_family: 1,
        range,
        source: usage(range, true).scope,
        destination: usage(range, false).scope,
        old_layout: vk::ImageLayout::UNDEFINED,
        new_layout: vk::ImageLayout::UNDEFINED,
    };
    let original = SyncPoint {
        queue: QueueId::new(0).unwrap(),
        value: 9,
    };
    let program = program(range, vec![usage(buffer(8, 8), true)], &[queue(2, 2)]);
    let mut planner =
        SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
            .unwrap();
    planner
        .add_initial(
            0,
            &[RangeCell {
                range,
                state: NativeState {
                    owner: RangeOwner::Released {
                        transfer,
                        point: original,
                    },
                    revision: 4,
                    ..NativeState::default()
                },
            }],
        )
        .unwrap();
    planner
        .bind(&program, |family| Ok(queue(family, family)))
        .unwrap();
    assert_eq!(planner.bound().execution_order(), &[1, 2, 0]);
    let first = planner.bound().patches()[0];
    assert_eq!(first.range, range);
    assert_eq!(first.expected_revision, 4);
    assert!(
        matches!(first.action,BoundPatchAction::Acquire {release:DependencyRef::Point(point),..} if point==original)
    );
    assert!(
        matches!(planner.bound().patches()[1].action,BoundPatchAction::Release(pair) if pair.range==buffer(8,8)&&pair.source_family==1&&pair.destination_family==2)
    );
}
#[test]
fn shared_planner_envelope_keeps_credit_until_its_last_cpu_owner() {
    let stats = Arc::new(AllocatorStats::default());
    let planner =
        SynchronizationPlanner::new_shared(stats.clone(), Default::default()).unwrap();
    let expected = crate::accounting::shared_cpu_bytes::<SynchronizationPlanner>().unwrap();
    assert_eq!(stats.cpu_memory_usage().total(), expected);
    let observer = planner.clone();
    drop(planner);
    assert_eq!(stats.cpu_memory_usage().total(), expected);
    drop(observer);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    assert!(matches!(
        SynchronizationPlanner::new_shared(
            stats.clone(),
            SynchronizationConfig {
                max_cpu_bytes: 1,
                ..Default::default()
            }
        ),
        Err(Error::Timeout)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
#[test]
fn warmed_binding_reuses_all_bounded_capacity() {
    let stats = Arc::new(AllocatorStats::default());
    let program = program(
        buffer(0, 16),
        vec![usage(buffer(0, 16), true), usage(buffer(0, 16), false)],
        &[queue(0, 0), queue(0, 0)],
    );
    let mut planner = SynchronizationPlanner::new(stats.clone(), Default::default()).unwrap();
    let initial = [RangeCell {
        range: buffer(0, 16),
        state: NativeState::default(),
    }];
    let mut bytes = 0;
    for frame in 0..1000 {
        planner.begin();
        planner.add_initial(0, &initial).unwrap();
        planner
            .bind(&program, |family| Ok(queue(0, family)))
            .unwrap();
        if frame == 0 {
            bytes = planner.known_cpu_bytes();
        } else {
            assert_eq!(planner.known_cpu_bytes(), bytes);
        }
    }
    drop(planner);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}

/// Deterministic synthetic program: `resources` buffers, `passes` passes
/// of `per_pass` sub-range uses each, spread over `domains` batches.
fn synthetic(
    resources: u32,
    passes: usize,
    per_pass: usize,
    domains: &[QueueDomain],
) -> (RangeProgram, Vec<RangeCell>) {
    const SIZE: u64 = 4096;
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut uses = Vec::new();
    let mut program_passes = Vec::new();
    for pass in 0..passes {
        let first = uses.len();
        for _ in 0..per_pass {
            let resource = (next() % resources as u64) as u32;
            let offset = (next() % 16) * 256;
            let size = (1 + next() % 4) * 256;
            let mut item = usage(buffer(offset, size.min(SIZE - offset)), next() % 3 == 0);
            item.resource = resource;
            uses.push(item);
        }
        program_passes.push(ProgramPass {
            pass,
            uses: first..uses.len(),
        });
    }
    let per_batch = passes.div_ceil(domains.len().max(1)).max(1);
    let batches = (0..passes.div_ceil(per_batch))
        .map(|batch| ProgramBatch {
            domain: domains[batch % domains.len()],
            passes: batch * per_batch..((batch + 1) * per_batch).min(passes),
        })
        .collect();
    let program = RangeProgram {
        resources: (0..resources)
            .map(|_| ResourceDomain {
                range: buffer(0, SIZE),
                sharing: ResourceSharing::Exclusive,
            })
            .collect(),
        uses,
        passes: program_passes,
        batches,
        ..Default::default()
    };
    let initial = vec![
        RangeCell {
            range: buffer(0, SIZE),
            state: NativeState::default(),
        };
        1
    ];
    (program, initial)
}
fn bind_synthetic(program: &RangeProgram, initial: &[RangeCell]) -> SynchronizationPlanner {
    let mut planner =
        SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
            .unwrap();
    for resource in 0..program.resources.len() as u32 {
        planner.add_initial(resource, initial).unwrap();
    }
    planner
        .bind(program, |family| Ok(queue(family, family)))
        .unwrap();
    planner
}
fn fingerprint(bound: &BoundSynchronization) -> u64 {
    let text = format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}",
        bound.execution_order(),
        bound.transitions(),
        bound.patches(),
        bound.waits(),
        bound.batches()
    );
    text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3)
    })
}
/// The planner output for a fixed synthetic program must not change when
/// the lookup structure changes. Values recorded from the scan-everything
/// implementation.
#[test]
fn synthetic_binding_output_is_stable() {
    let one = [queue(0, 0)];
    let two = [queue(0, 0), queue(1, 1)];
    let cases: [(&[QueueDomain], u32, usize, usize); 3] =
        [(&one, 40, 60, 6), (&two, 24, 48, 5), (&two, 120, 160, 12)];
    let mut prints = Vec::new();
    for (domains, resources, passes, per_pass) in cases {
        let (program, initial) = synthetic(resources, passes, per_pass, domains);
        let planner = bind_synthetic(&program, &initial);
        prints.push(fingerprint(planner.bound()));
    }
    eprintln!("FINGERPRINTS {prints:?}");
    assert_eq!(prints, EXPECTED_FINGERPRINTS);
}
/// The bucketed accessors must equal the linear filters they replace, in
/// content and order, for every batch and phase.
#[test]
fn phase_index_matches_linear_filter() {
    let one = [queue(0, 0)];
    let two = [queue(0, 0), queue(1, 1)];
    for (domains, resources, passes, per_pass) in
        [(&one[..], 40, 60, 6), (&two[..], 24, 48, 5), (&two[..], 120, 160, 12)]
    {
        let (program, initial) = synthetic(resources, passes, per_pass, domains);
        let planner = bind_synthetic(&program, &initial);
        let bound = planner.bound();
        assert!(bound.index.valid, "synthetic program must index");
        let mut seen = 0;
        for (batch, entry) in bound.batches().iter().enumerate() {
            let mut phases = vec![BarrierPhase::Initial, BarrierPhase::Export];
            if let BoundBatchKind::Graph { passes, .. } = &entry.kind {
                phases.extend(passes.clone().map(BarrierPhase::BeforePass));
            }
            for phase in phases {
                let fast: Vec<_> = bound.transitions_for(batch, phase).copied().collect();
                let slow: Vec<_> = bound
                    .transitions()
                    .iter()
                    .copied()
                    .filter(|t| t.batch == batch && t.phase == phase)
                    .collect();
                seen += fast.len();
                assert_eq!(fast, slow, "transitions batch {batch} {phase:?}");
            }
            let fast: Vec<_> = bound.patches_for(batch).copied().collect();
            let slow: Vec<_> = bound.patches().iter().copied().filter(|p| p.batch == batch).collect();
            assert_eq!(fast, slow, "patches batch {batch}");
            let fast: Vec<_> = bound.waits_for(batch).copied().collect();
            let slow: Vec<_> = bound.waits().iter().copied().filter(|w| w.batch == batch).collect();
            assert_eq!(fast, slow, "waits batch {batch}");
        }
        assert_eq!(seen, bound.transitions().len(), "every transition has a slot");
    }
}
fn imported(revision: u64, point: u64, layout_known: bool) -> Vec<RangeCell> {
    let mut state = NativeState {
        revision,
        layout_known,
        ..NativeState::default()
    };
    let mut writer = Frontier::EMPTY;
    writer.points.insert(crate::sync::SyncPoint {
        queue: crate::sync::QueueId::new(0).unwrap(),
        value: point,
    });
    state.writer = writer;
    vec![RangeCell {
        range: buffer(0, 4096),
        state,
    }]
}
fn bind_with(
    planner: &mut SynchronizationPlanner,
    program: &RangeProgram,
    initial: &[RangeCell],
    cached: bool,
) {
    planner.begin();
    for resource in 0..program.resources.len() as u32 {
        planner.add_initial(resource, initial).unwrap();
    }
    let family = |family| Ok(queue(family, family));
    if cached {
        let pin: Arc<dyn std::any::Any + Send + Sync> = Arc::new(());
        planner
            .bind_cached(program, program as *const _ as usize, pin, family)
            .unwrap();
    } else {
        planner.bind(program, family).unwrap();
    }
}
/// A frame that differs only in revisions and timeline points must reuse
/// the template and produce exactly what a full bind produces.
#[test]
fn rebased_template_equals_full_binding_across_frames() {
    let one = [queue(0, 0)];
    let (program, _) = synthetic(40, 60, 6, &one);
    let mut cached = SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default()).unwrap();
    let mut fresh = SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default()).unwrap();
    for frame in 0..5u64 {
        let initial = imported(10 * frame + 3, frame + 1, true);
        bind_with(&mut cached, &program, &initial, true);
        bind_with(&mut fresh, &program, &initial, false);
        assert_eq!(fingerprint(cached.bound()), fingerprint(fresh.bound()), "frame {frame}");
        assert_eq!(cached.template_hits(), frame, "frame {frame}");
    }
    // A changed layout is a different key: no hit, still exact.
    let before = cached.template_hits();
    let initial = imported(99, 9, false);
    bind_with(&mut cached, &program, &initial, true);
    bind_with(&mut fresh, &program, &initial, false);
    assert_eq!(cached.template_hits(), before);
    assert_eq!(fingerprint(cached.bound()), fingerprint(fresh.bound()));
}
/// A pending ownership release can never be served from a template.
#[test]
fn released_owner_is_never_templated() {
    let two = [queue(0, 0), queue(1, 1)];
    let (program, initial) = synthetic(24, 48, 5, &two);
    let mut planner = SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default()).unwrap();
    for _ in 0..3 {
        bind_with(&mut planner, &program, &initial, true);
    }
    // Two families with exclusive resources produce ownership transfers.
    assert_eq!(planner.template_hits(), 0);
}
/// Cost of the submission-time replay for the synthetic program: every
/// resource's table prepares and publishes that resource's patches.
#[test]
#[ignore]
fn replay_cost_baseline() {
    use crate::tracking::range_state::{RangeStateTable, StatePatch};
    let one = [queue(0, 0)];
    let (program, initial) = synthetic(200, 160, 19, &one);
    let planner = bind_synthetic(&program, &initial);
    let bound = planner.bound();
    let q0 = crate::sync::QueueId::new(0).unwrap();
    let mut per_resource: Vec<Vec<StatePatch>> = vec![Vec::new(); program.resources.len()];
    for patch in bound.patches() {
        per_resource[patch.resource as usize].push(patch.resolve(|_| None).unwrap());
    }
    let stats = Arc::new(AllocatorStats::default());
    let (mut prepare_ns, mut publish_ns) = (0u128, 0u128);
    let mut samples = Vec::new();
    let rounds: usize = std::env::var("REPLAY_ROUNDS").ok().and_then(|v| v.parse().ok()).unwrap_or(30);
    for _ in 0..rounds {
        let mut tables: Vec<_> = (0..program.resources.len())
            .map(|_| {
                RangeStateTable::new(
                    buffer(0, 4096),
                    ResourceSharing::Exclusive,
                    NativeState::default(),
                    Default::default(),
                    stats.clone(),
                )
                .unwrap()
            })
            .collect();
        // Production tables are long-lived with warm capacity.
        for (table, patches) in tables.iter_mut().zip(&per_resource) {
            if !patches.is_empty() {
                table.prepare(patches, q0, 0).unwrap();
                table.cancel_prepared();
            }
        }
        let start = std::time::Instant::now();
        prepare_ns = 0;
        publish_ns = 0;
        for (table, patches) in tables.iter_mut().zip(&per_resource) {
            if patches.is_empty() {
                continue;
            }
            let t0 = std::time::Instant::now();
            table.prepare(patches, q0, 0).unwrap();
            let t1 = std::time::Instant::now();
            table.publish(crate::sync::SyncPoint { queue: q0, value: 1 });
            prepare_ns += (t1 - t0).as_nanos();
            publish_ns += t1.elapsed().as_nanos();
        }
        samples.push(start.elapsed());
    }
    eprintln!("last round: prepare={prepare_ns}ns publish={publish_ns}ns");
    samples.sort();
    eprintln!(
        "replay patches={} median={:?} p95={:?}",
        bound.patches().len(),
        samples[samples.len() / 2],
        samples[samples.len() * 95 / 100]
    );
}
/// A resource number beyond any possible graph is refused up front rather
/// than sizing the per-resource index for it.
#[test]
fn absurd_resource_numbers_are_rejected_without_allocating_an_index() {
    let mut planner =
        SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
            .unwrap();
    let cell = RangeCell {
        range: buffer(0, 16),
        state: NativeState::default(),
    };
    assert!(planner.add_initial(u32::MAX, &[cell]).is_err());
    assert!(planner.add_alias(0, u32::MAX).is_err());
    assert!(planner.by_resource.values.is_empty());
}
/// A template must not be applied when the patch's own revision increment
/// would exhaust the counter; a full bind rejects that frame.
#[test]
fn rebase_that_would_exhaust_a_revision_falls_back_to_the_full_bind_error() {
    let one = [queue(0, 0)];
    let (program, _) = synthetic(4, 6, 3, &one);
    let mut planner =
        SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
            .unwrap();
    bind_with(&mut planner, &program, &imported(1, 1, true), true);
    bind_with(&mut planner, &program, &imported(2, 2, true), true);
    assert_eq!(planner.template_hits(), 1);
    let hits = planner.template_hits();
    planner.begin();
    for resource in 0..program.resources.len() as u32 {
        planner
            .add_initial(resource, &imported(u64::MAX - 2, 3, true))
            .unwrap();
    }
    let pin: Arc<dyn std::any::Any + Send + Sync> = Arc::new(());
    let result = planner.bind_cached(
        &program,
        &program as *const _ as usize,
        pin,
        |family| Ok(queue(family, family)),
    );
    assert!(result.is_err(), "exhausted revision must be an error");
    assert_eq!(planner.template_hits(), hits, "no template served it");
}
const EXPECTED_FINGERPRINTS: [u64; 3] = [
    15114495150305685386,
    7488788626014099824,
    8713088647009251552,
];
/// `cargo test -p rhapsody --release --lib bind_scaling -- --ignored --nocapture`
#[test]
#[ignore]
fn bind_scaling_baseline() {
    let one = [queue(0, 0)];
    let (program, initial) = synthetic(200, 160, 19, &one);
    let mut planner =
        SynchronizationPlanner::new(Arc::new(AllocatorStats::default()), Default::default())
            .unwrap();
    let mut samples = Vec::new();
    for _ in 0..30 {
        let start = std::time::Instant::now();
        planner.begin();
        for resource in 0..program.resources.len() as u32 {
            planner.add_initial(resource, &initial).unwrap();
        }
        planner
            .bind(&program, |family| Ok(queue(family, family)))
            .unwrap();
        samples.push(start.elapsed());
    }
    samples.sort();
    let mut cached = Vec::new();
    for frame in 0..30u64 {
        let start = std::time::Instant::now();
        bind_with(&mut planner, &program, &imported(frame * 7, frame + 1, true), true);
        cached.push(start.elapsed());
    }
    cached.sort();
    eprintln!(
        "bind_cached hits={} median={:?}",
        planner.template_hits(),
        cached[cached.len() / 2]
    );
    eprintln!(
        "bind U={} S={} transitions={} median={:?} p95={:?}",
        program.uses.len(),
        planner.shadow.values.len(),
        planner.bound().transitions().len(),
        samples[samples.len() / 2],
        samples[samples.len() * 95 / 100]
    );
}
