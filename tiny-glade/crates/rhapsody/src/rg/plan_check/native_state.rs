//! Independent replay of successful range-state publication. Geometry uses the
//! checker's disjoint slabs; production preparation, splitting and merging are
//! never invoked to compute the expected state.
use super::{invalid, range_oracle::SpatialLedger};
use crate::{
    resource::{
        OwnershipRange, ResourceSharing,
        range_state::{
            AccessScope, Frontier, NativeState, PatchAction, RangeCell, RangeOwner, StatePatch,
        },
    },
    sync::{SyncPoint, SyncSet},
};

#[derive(Clone)]
pub(super) struct StateOracle {
    domain: OwnershipRange,
    sharing: ResourceSharing,
    cells: SpatialLedger<NativeState>,
}

fn wait_for(point: SyncPoint, submitted: SyncPoint, waits: &SyncSet) -> crate::Result<()> {
    if point.value == 0 {
        return Err(invalid("native dependency uses an unsubmitted zero point"));
    }
    if point.queue != submitted.queue && waits.get(point.queue) < point.value {
        return Err(invalid(
            "native range access omits a foreign queue dependency",
        ));
    }
    Ok(())
}

fn wait_for_frontier(
    frontier: &Frontier,
    submitted: SyncPoint,
    waits: &SyncSet,
) -> crate::Result<()> {
    for point in frontier.points.points() {
        wait_for(point, submitted, waits)?;
    }
    Ok(())
}

impl StateOracle {
    pub(super) fn new(
        domain: OwnershipRange,
        sharing: ResourceSharing,
        initial: &[RangeCell],
    ) -> crate::Result<Self> {
        Ok(Self {
            domain,
            sharing,
            cells: SpatialLedger::new(domain, initial.iter().map(|cell| (cell.range, cell.state)))?,
        })
    }

    /// A submit failure never calls this method. All validation and replay are
    /// performed against a local copy, so any rejected later patch preserves
    /// every previously published batch and the entire current batch input.
    pub(super) fn apply_success(
        &mut self,
        patches: &[StatePatch],
        submitted: SyncPoint,
        family: u32,
        waits: SyncSet,
    ) -> crate::Result<()> {
        if submitted.value == 0 || family == u32::MAX {
            return Err(invalid(
                "native publication needs a real queue point and family",
            ));
        }
        for patch in patches {
            // Distinguish pre-existing real points from earlier accesses in
            // this batch, which will all receive this one actual receipt.
            self.cells.visit(patch.range, |state| {
                for point in state
                    .writer
                    .points
                    .points()
                    .chain(state.readers.points.points())
                {
                    if point.queue == submitted.queue && point.value >= submitted.value {
                        return Err(invalid("native publication precedes its input state"));
                    }
                }
                Ok(())
            })?;
        }
        let mut next = self.cells.clone();
        for patch in patches {
            next.update(patch.range, |old| {
                let opaque = matches!(patch.action, PatchAction::Unknown { .. })
                    && patch.expected_revision == u64::MAX;
                if !opaque && old.revision != patch.expected_revision {
                    return Err(invalid("native patch has a stale per-range revision"));
                }
                let mut state = *old;
                match patch.action {
                    PatchAction::Access {
                        scope,
                        layout,
                        queue,
                        family: expected_family,
                    } => {
                        if queue != submitted.queue || family != expected_family {
                            return Err(invalid("native patch belongs to another queue domain"));
                        }
                        match self.sharing {
                            ResourceSharing::Concurrent(families) => {
                                if !families.as_slice().contains(&family) {
                                    return Err(invalid("native patch uses an undeclared family"));
                                }
                            }
                            ResourceSharing::Exclusive => match state.owner {
                                RangeOwner::Unclaimed => state.owner = RangeOwner::Owned(family),
                                RangeOwner::Owned(owner) if owner == family => {}
                                _ => return Err(invalid("native access lacks ownership acquire")),
                            },
                        }
                        wait_for_frontier(&state.writer, submitted, &waits)?;
                        if scope.write || !state.layout_known || state.layout != layout {
                            wait_for_frontier(&state.readers, submitted, &waits)?;
                        }
                        if scope.write || !state.layout_known || state.layout != layout {
                            state.writer = Frontier::EMPTY;
                            state.writer.scopes[queue.index()] = scope;
                            state.writer.points.insert(submitted);
                            state.readers = Frontier::EMPTY;
                        } else {
                            let slot = &mut state.readers.scopes[queue.index()];
                            slot.stages |= scope.stages;
                            slot.access |= scope.access;
                            slot.write |= scope.write;
                            state.readers.points.insert(submitted);
                        }
                        state.layout = layout;
                        state.layout_known = true;
                    }
                    PatchAction::Release(transfer) => {
                        if transfer.pair == 0
                            || transfer.source_family == transfer.destination_family
                            || transfer.source_family == u32::MAX
                            || transfer.destination_family == u32::MAX
                            || transfer.range != patch.range
                            || self.sharing.is_concurrent()
                            || state.owner != RangeOwner::Owned(family)
                            || family != transfer.source_family
                            || !state.layout_known
                            || state.layout != transfer.old_layout
                        {
                            return Err(invalid(
                                "native release mismatches its range, owner or layout",
                            ));
                        }
                        wait_for_frontier(&state.writer, submitted, &waits)?;
                        wait_for_frontier(&state.readers, submitted, &waits)?;
                        state.owner = RangeOwner::Released {
                            transfer,
                            point: submitted,
                        };
                        state.layout = transfer.new_layout;
                    }
                    PatchAction::Acquire {
                        transfer,
                        release_point,
                    } => {
                        if transfer.pair == 0
                            || transfer.source_family == transfer.destination_family
                            || transfer.source_family == u32::MAX
                            || transfer.destination_family == u32::MAX
                            || release_point == submitted
                            || transfer.range != patch.range
                            || self.sharing.is_concurrent()
                            || family != transfer.destination_family
                            || state.owner
                                != (RangeOwner::Released {
                                    transfer,
                                    point: release_point,
                                })
                            || !state.layout_known
                            || state.layout != transfer.new_layout
                        {
                            return Err(invalid("native acquire mismatches the actual release"));
                        }
                        wait_for(release_point, submitted, &waits)?;
                        state.owner = RangeOwner::Owned(family);
                        state.layout = transfer.new_layout;
                        state.layout_known = true;
                        state.writer = Frontier::EMPTY;
                        state.readers = Frontier::EMPTY;
                        state.writer.scopes[submitted.queue.index()] = transfer.destination;
                        state.writer.points.insert(submitted);
                    }
                    PatchAction::Unknown {
                        queue,
                        family: expected_family,
                    } => {
                        if queue != submitted.queue
                            || family != expected_family
                            || !self.sharing.allows_family(family)
                        {
                            return Err(invalid("opaque native use has an invalid queue domain"));
                        }
                        if !self.sharing.is_concurrent() {
                            match state.owner {
                                RangeOwner::Unclaimed => state.owner = RangeOwner::Owned(family),
                                RangeOwner::Owned(owner) if owner == family => {}
                                _ => return Err(invalid("opaque native use lacks ownership")),
                            }
                        }
                        wait_for_frontier(&state.writer, submitted, &waits)?;
                        wait_for_frontier(&state.readers, submitted, &waits)?;
                        state.writer = Frontier::EMPTY;
                        state.writer.scopes[submitted.queue.index()] = AccessScope {
                            stages: ash::vk::PipelineStageFlags::ALL_COMMANDS,
                            access: ash::vk::AccessFlags::MEMORY_READ
                                | ash::vk::AccessFlags::MEMORY_WRITE,
                            write: true,
                        };
                        state.writer.points.insert(submitted);
                        state.readers = Frontier::EMPTY;
                        state.layout_known = matches!(patch.range, OwnershipRange::Buffer { .. });
                    }
                }
                state.revision = state
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| invalid("native range revision overflows"))?;
                Ok(state)
            })?;
        }
        self.cells = next;
        Ok(())
    }

    pub(super) fn check_snapshot(&self, actual: &[RangeCell]) -> crate::Result<()> {
        // Validate complete, non-overlapping physical coverage independently of
        // whether the producer chose to coalesce equal neighboring cells.
        SpatialLedger::new(
            self.domain,
            actual.iter().map(|cell| (cell.range, cell.state)),
        )?;
        for cell in actual {
            self.cells.visit(cell.range, |expected| {
                if expected != &cell.state {
                    return Err(invalid(
                        "native published state differs from its submitted patches",
                    ));
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}

/// Explicit validation entry point for a successful physical publication. This
/// cold-path verifier owns temporary state; actual submission admission remains
/// gate-held and allocation-free after the Vulkan call succeeds.
pub struct StatePublication<'a> {
    pub domain: OwnershipRange,
    pub sharing: ResourceSharing,
    pub before: &'a [RangeCell],
    pub patches: &'a [StatePatch],
    pub submitted: SyncPoint,
    pub family: u32,
    pub waits: SyncSet,
    pub after: &'a [RangeCell],
}

pub fn check_state_publication(publication: StatePublication<'_>) -> crate::Result<()> {
    let mut oracle = StateOracle::new(publication.domain, publication.sharing, publication.before)?;
    oracle.apply_success(
        publication.patches,
        publication.submitted,
        publication.family,
        publication.waits,
    )?;
    oracle.check_snapshot(publication.after)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        raw::allocator::AllocatorStats,
        resource::{
            QueueFamilies,
            range_state::{RangeStateConfig, RangeStateTable, RangeTransfer},
        },
        sync::QueueId,
    };
    use ash::vk;
    use std::sync::Arc;

    fn point(queue: usize, value: u64) -> SyncPoint {
        SyncPoint {
            queue: QueueId::new(queue).unwrap(),
            value,
        }
    }
    fn buffer(offset: u64, size: u64) -> OwnershipRange {
        OwnershipRange::Buffer { offset, size }
    }
    fn scope(stage: vk::PipelineStageFlags, write: bool) -> AccessScope {
        AccessScope {
            stages: stage,
            access: if write {
                vk::AccessFlags::SHADER_WRITE
            } else {
                vk::AccessFlags::SHADER_READ
            },
            write,
        }
    }
    fn access(range: OwnershipRange, revision: u64, queue: usize, write: bool) -> StatePatch {
        StatePatch {
            range,
            expected_revision: revision,
            action: PatchAction::Access {
                scope: scope(vk::PipelineStageFlags::COMPUTE_SHADER, write),
                layout: vk::ImageLayout::GENERAL,
                queue: QueueId::new(queue).unwrap(),
                family: queue as u32,
            },
        }
    }
    fn table(domain: OwnershipRange, sharing: ResourceSharing) -> RangeStateTable {
        RangeStateTable::new(
            domain,
            sharing,
            NativeState::default(),
            RangeStateConfig::default(),
            Arc::new(AllocatorStats::default()),
        )
        .unwrap()
    }
    fn apply(
        table: &mut RangeStateTable,
        oracle: &mut StateOracle,
        patches: &[StatePatch],
        submitted: SyncPoint,
        waits: SyncSet,
    ) {
        let family = submitted.queue.index() as u32;
        let before = table.cells().to_vec();
        table.prepare(patches, submitted.queue, family).unwrap();
        table.publish(submitted);
        oracle
            .apply_success(patches, submitted, family, waits)
            .unwrap();
        oracle.check_snapshot(table.cells()).unwrap();
        check_state_publication(StatePublication {
            domain: table.domain(),
            sharing: oracle.sharing,
            before: &before,
            patches,
            submitted,
            family,
            waits,
            after: table.cells(),
        })
        .unwrap();
    }

    #[test]
    fn readers_from_three_queues_are_preserved_and_every_reader_is_required_before_write() {
        let whole = buffer(0, 16);
        let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1, 2]).unwrap());
        let mut table = table(whole, sharing);
        let mut oracle = StateOracle::new(whole, sharing, table.cells()).unwrap();
        apply(
            &mut table,
            &mut oracle,
            &[access(whole, 0, 0, true)],
            point(0, 1),
            SyncSet::EMPTY,
        );
        for queue in [1, 2] {
            apply(
                &mut table,
                &mut oracle,
                &[access(buffer(0, 8), queue, queue as usize, false)],
                point(queue as usize, 1),
                point(0, 1).into(),
            );
        }
        let write = access(buffer(0, 8), 3, 0, true);
        for missing in [1, 2] {
            let mut waits = SyncSet::EMPTY;
            waits.insert(point(3 - missing, 1));
            assert!(
                oracle
                    .apply_success(&[write], point(0, 2), 0, waits)
                    .is_err()
            );
            oracle.check_snapshot(table.cells()).unwrap();
        }
        let mut waits = SyncSet::from(point(1, 1));
        waits.insert(point(2, 1));
        apply(&mut table, &mut oracle, &[write], point(0, 2), waits);

        // This patch reaches one matching region before a later stale region.
        // Neither the real table nor the independent oracle may partly commit.
        let stale = access(whole, 4, 0, true);
        assert!(table.prepare(&[stale], point(0, 3).queue, 0).is_err());
        assert!(
            oracle
                .apply_success(&[stale], point(0, 3), 0, waits)
                .is_err()
        );
        oracle.check_snapshot(table.cells()).unwrap();
        table.cancel_prepared();
        apply(
            &mut table,
            &mut oracle,
            &[access(buffer(8, 8), 1, 2, true)],
            point(2, 2),
            point(0, 1).into(),
        );
        // A later failed batch cannot roll back this already successful prefix.
        assert!(
            oracle
                .apply_success(&[access(whole, 0, 1, true)], point(1, 2), 1, waits)
                .is_err()
        );
        oracle.check_snapshot(table.cells()).unwrap();
    }

    #[test]
    fn image_publications_match_dense_revision_grid_across_queues() {
        let whole = OwnershipRange::Image {
            aspects: 0b110,
            base_mip: 0,
            mip_count: 3,
            base_layer: 0,
            layer_count: 4,
        };
        let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1, 2]).unwrap());
        let mut table = table(whole, sharing);
        let mut oracle = StateOracle::new(whole, sharing, table.cells()).unwrap();
        let mut dense = [[[0u64; 4]; 3]; 2];
        let mut counts = [0u64; 3];
        let mut seed = 0x938b_ac76_u64;
        for _ in 0..200 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let aspect = seed as usize % 2;
            let mip = (seed >> 8) as usize % 3;
            let layer = (seed >> 16) as usize % 4;
            let queue = (seed >> 24) as usize % 3;
            let mut waits = SyncSet::EMPTY;
            for (prior_queue, value) in counts.into_iter().enumerate() {
                if value != 0 {
                    waits.insert(point(prior_queue, value));
                }
            }
            counts[queue] += 1;
            let range = OwnershipRange::Image {
                aspects: 1 << (aspect + 1),
                base_mip: mip as u32,
                mip_count: 1,
                base_layer: layer as u32,
                layer_count: 1,
            };
            apply(
                &mut table,
                &mut oracle,
                &[access(
                    range,
                    dense[aspect][mip][layer],
                    queue,
                    seed & 1 != 0,
                )],
                point(queue, counts[queue]),
                waits,
            );
            dense[aspect][mip][layer] += 1;
            for (plane, rows) in dense.iter().enumerate() {
                for (mip, row) in rows.iter().enumerate() {
                    for (layer, revision) in row.iter().enumerate() {
                        oracle
                            .cells
                            .visit(
                                OwnershipRange::Image {
                                    aspects: 1 << (plane + 1),
                                    base_mip: mip as u32,
                                    mip_count: 1,
                                    base_layer: layer as u32,
                                    layer_count: 1,
                                },
                                |state| {
                                    assert_eq!(state.revision, *revision);
                                    Ok(())
                                },
                            )
                            .unwrap();
                    }
                }
            }
        }
    }

    #[test]
    fn acquire_requires_exact_release_identity_real_wait_and_matching_layout() {
        let whole = OwnershipRange::Image {
            aspects: 1,
            base_mip: 0,
            mip_count: 2,
            base_layer: 0,
            layer_count: 2,
        };
        let sharing = ResourceSharing::Exclusive;
        let mut table = table(whole, sharing);
        let mut oracle = StateOracle::new(whole, sharing, table.cells()).unwrap();
        apply(
            &mut table,
            &mut oracle,
            &[access(whole, 0, 0, true)],
            point(0, 1),
            SyncSet::EMPTY,
        );
        let transfer = RangeTransfer {
            pair: 53,
            source_family: 0,
            destination_family: 1,
            range: whole,
            source: scope(vk::PipelineStageFlags::COMPUTE_SHADER, true),
            destination: scope(vk::PipelineStageFlags::FRAGMENT_SHADER, false),
            old_layout: vk::ImageLayout::GENERAL,
            new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        };
        apply(
            &mut table,
            &mut oracle,
            &[StatePatch {
                range: whole,
                expected_revision: 1,
                action: PatchAction::Release(transfer),
            }],
            point(0, 2),
            SyncSet::EMPTY,
        );
        let acquire = StatePatch {
            range: whole,
            expected_revision: 2,
            action: PatchAction::Acquire {
                transfer,
                release_point: point(0, 2),
            },
        };
        assert!(
            oracle
                .apply_success(&[acquire], point(1, 1), 1, point(0, 1).into())
                .is_err()
        );
        oracle.check_snapshot(table.cells()).unwrap();
        let wrong = StatePatch {
            action: PatchAction::Acquire {
                transfer,
                release_point: point(0, 1),
            },
            ..acquire
        };
        assert!(table.prepare(&[wrong], point(1, 1).queue, 1).is_err());
        assert!(
            oracle
                .apply_success(&[wrong], point(1, 1), 1, point(0, 2).into())
                .is_err()
        );
        oracle.check_snapshot(table.cells()).unwrap();
        table.cancel_prepared();
        apply(
            &mut table,
            &mut oracle,
            &[acquire],
            point(1, 1),
            point(0, 2).into(),
        );
        let mut forged = table.cells().to_vec();
        forged[0].state.layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
        assert!(oracle.check_snapshot(&forged).is_err());
        forged[0] = table.cells()[0];
        forged[0].state.writer.scopes[1].stages = vk::PipelineStageFlags::empty();
        assert!(oracle.check_snapshot(&forged).is_err());
    }

    #[test]
    fn one_submit_replays_ordered_patches_before_publishing_its_single_receipt() {
        let whole = buffer(0, 16);
        let mut table = table(whole, ResourceSharing::Exclusive);
        let mut oracle =
            StateOracle::new(whole, ResourceSharing::Exclusive, table.cells()).unwrap();
        apply(
            &mut table,
            &mut oracle,
            &[
                access(whole, 0, 0, true),
                access(buffer(0, 8), 1, 0, false),
                access(buffer(0, 8), 2, 0, true),
                access(buffer(8, 8), 1, 0, false),
            ],
            point(0, 1),
            SyncSet::EMPTY,
        );
        oracle
            .cells
            .visit(buffer(0, 8), |state| {
                assert_eq!(state.revision, 3);
                assert!(state.readers.points.is_empty());
                Ok(())
            })
            .unwrap();
        oracle
            .cells
            .visit(buffer(8, 8), |state| {
                assert_eq!(state.revision, 2);
                assert_eq!(state.writer.points.get(point(0, 1).queue), 1);
                assert_eq!(state.readers.points.get(point(0, 1).queue), 1);
                Ok(())
            })
            .unwrap();
    }
}
