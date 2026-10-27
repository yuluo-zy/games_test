//! Independent replay of successful range-state publication. Geometry uses the
//! checker's disjoint slabs; production preparation, splitting and merging are
//! never invoked to compute the expected state.
use super::{invalid, range_oracle::SpatialLedger};
use ash::vk;
use crate::sync::{SyncPoint, SyncSet};
use crate::tracking::{OwnershipRange, ResourceSharing};
use crate::tracking::range_state::{Frontier, NativeRange, NativeState, PatchAction, RangeCell, RangeOwner, StatePatch};
use crate::native::AccessScope;

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
    if point.queue == submitted.queue && point.value > submitted.value {
        return Err(invalid("native same-queue dependency does not precede submission"));
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
                    if point.queue == submitted.queue && point.value > submitted.value {
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
                                // Undefined image contents have no owner to hand over.
                                RangeOwner::Owned(_)
                                    if matches!(patch.range, NativeRange::Image { .. })
                                        && state.layout_known
                                        && state.layout == vk::ImageLayout::UNDEFINED =>
                                {
                                    state.owner = RangeOwner::Owned(family)
                                }
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
mod tests;
