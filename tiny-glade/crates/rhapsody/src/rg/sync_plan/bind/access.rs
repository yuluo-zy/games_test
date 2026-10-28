//! Per-use binding: access resolution, acquisition and cross-queue waits.
use crate::Result;
use ash::vk;
use crate::rg::sync_plan::{BoundBatch, BoundPatch, BoundTransition, BoundWait};
use crate::native::access_table::AccessScope;
use crate::Error;
use crate::rg::queue_schedule::QueueDomain;
use crate::rg::sync_plan::BarrierKind;
use crate::rg::sync_plan::BarrierPhase;
use crate::rg::sync_plan::BoundBatchKind;
use crate::rg::sync_plan::BoundPatchAction;
use crate::rg::sync_plan::DependencyRef;
use crate::rg::sync_plan::RangeProgram;
use crate::rg::sync_plan::RangeUse;
use crate::tracking::range_state::NativeRange;
use crate::tracking::range_state::RangeTransfer;
use crate::tracking::range_state;
use super::SynchronizationPlanner;
use super::frontier::PendingFrontier;
use super::frontier::PendingOwner;
use super::queue_id;
use super::shadow::ShadowCell;

impl SynchronizationPlanner {
    pub(super) fn bind_use(
        &mut self,
        access: RangeUse,
        batch: usize,
        phase: BarrierPhase,
        program: &RangeProgram,
        family_domain: &mut impl FnMut(u32) -> Result<QueueDomain>,
    ) -> Result<()> {
        let domain = self.bound.batches.values[batch].domain;
        let queue = queue_id(domain)?;
        let sharing = program.resources[access.resource as usize].sharing;
        if !sharing.allows_family(domain.family) {
            return Err(Error::invalid_usage(
                "native use family absent from sharing list",
            ));
        }
        let alias_flags = self.resource_shadow(access.resource)?.alias_flags;
        // Fragments split off below are appended past `visit`; like the old
        // flat scan, this use does not revisit them.
        let visit = self.by_resource.values[access.resource as usize].cells.len();
        let mut covered = false;
        for slot in 0..visit {
            let index = self.by_resource.values[access.resource as usize].cells[slot] as usize;
            let mut current = self.shadow.values[index];
            let Some(overlap) = range_state::intersection(current.range, access.range) else {
                continue;
            };
            covered = true;
            if !current.layout_known && matches!(current.range, NativeRange::Image { .. }) {
                return Err(Error::unsupported(
                    "opaque image use requires an explicit external native-state declaration",
                ));
            }
            // An alias member's contents are discarded at its first use in a
            // frame; its set returns it only when idle with the state reset
            // (AliasCache::give_back). Never acquire a stale release for it.
            if current.fresh
                && alias_flags != 0
                && (matches!(current.owner, PendingOwner::Released { .. })
                    || !sharing.is_concurrent()
                        && matches!(current.owner, PendingOwner::Owned(owner) if owner != domain.family))
            {
                return Err(Error::invalid_usage(
                    "alias member keeps queue-family ownership from an earlier frame",
                ));
            }
            // A prior failed acquire is completed for exactly its original
            // native pair before any splitting of that released range.
            if let PendingOwner::Released { transfer, release } = current.owner {
                if current.range != transfer.range {
                    return Err(Error::invalid_usage(
                        "released range lost its exact native pair coverage",
                    ));
                }
                if domain.family == transfer.destination_family {
                    current = self.acquire(current, transfer, release, batch, phase)?;
                } else {
                    let target = family_domain(transfer.destination_family)?;
                    let auxiliary = self.push_batch(BoundBatch {
                        domain: target,
                        kind: BoundBatchKind::OwnershipPrelude,
                    })?;
                    current =
                        self.acquire(current, transfer, release, auxiliary, BarrierPhase::Initial)?;
                    self.push_order(auxiliary)?;
                }
            }
            let outside = range_state::difference(current.range, overlap);
            for range in outside {
                self.push_shadow(ShadowCell { range, ..current })?;
            }
            current.range = overlap;
            if !sharing.is_concurrent() {
                match current.owner {
                    PendingOwner::Unclaimed => current.owner = PendingOwner::Owned(domain.family),
                    PendingOwner::Owned(owner) if owner == domain.family => {}
                    // Discarded contents (UNDEFINED image) have nothing to hand
                    // over: the using family just claims the range, and the
                    // UNDEFINED -> layout transition below runs on its queue.
                    PendingOwner::Owned(_) if Self::contents_discarded(&current) => {
                        current.owner = PendingOwner::Owned(domain.family);
                    }
                    PendingOwner::Owned(owner) => {
                        let source = family_domain(owner)?;
                        let auxiliary = self.push_batch(BoundBatch {
                            domain: source,
                            kind: BoundBatchKind::OwnershipPrelude,
                        })?;
                        let transfer = RangeTransfer {
                            pair: self.pair()?,
                            source_family: owner,
                            destination_family: domain.family,
                            range: overlap,
                            source: current
                                .writer
                                .source(queue_id(source)?)
                                .union(current.readers.source(queue_id(source)?)),
                            destination: access.scope,
                            old_layout: current.layout,
                            new_layout: access.layout,
                        };
                        self.wait_frontier(
                            current.writer,
                            auxiliary,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            source,
                        )?;
                        self.wait_frontier(
                            current.readers,
                            auxiliary,
                            vk::PipelineStageFlags::ALL_COMMANDS,
                            source,
                        )?;
                        self.push_transition(BoundTransition {
                            batch: auxiliary,
                            phase: BarrierPhase::Initial,
                            resource: access.resource,
                            range: overlap,
                            source: transfer.source,
                            destination: AccessScope::EMPTY,
                            old_layout: transfer.old_layout,
                            new_layout: transfer.new_layout,
                            source_family: owner,
                            destination_family: domain.family,
                            kind: BarrierKind::Release {
                                pair: transfer.pair,
                            },
                        })?;
                        self.push_patch_slot(current.initial_slot, BoundPatch {
                            batch: auxiliary,
                            phase: BarrierPhase::Initial,
                            resource: access.resource,
                            range: overlap,
                            expected_revision: current.revision,
                            action: BoundPatchAction::Release(transfer),
                        })?;
                        current.increment()?;
                        current.owner = PendingOwner::Released {
                            transfer,
                            release: DependencyRef::Batch(auxiliary),
                        };
                        current.layout = transfer.new_layout;
                        self.push_order(auxiliary)?;
                        current = self.acquire(
                            current,
                            transfer,
                            DependencyRef::Batch(auxiliary),
                            batch,
                            phase,
                        )?;
                    }
                    PendingOwner::Released { .. } => {
                        unreachable!("released ranges recovered before split")
                    }
                }
            }
            // Ownership recovery above keeps the stale physical layout; the
            // first access of an alias member then discards the contents.
            if current.fresh {
                current.fresh = false;
                if alias_flags & 1 != 0 {
                    current = self.discard_alias_contents(current);
                }
            }
            let changes_layout = current.layout != access.layout;
            let changes = access.scope.write || changes_layout;
            self.wait_frontier(current.writer, batch, access.scope.stages, domain)?;
            if changes {
                self.wait_frontier(current.readers, batch, access.scope.stages, domain)?;
            }
            let mut source = current.writer.source(queue).union(if changes {
                current.readers.source(queue)
            } else {
                AccessScope::EMPTY
            });
            // A layout transition after a cross-queue semaphore wait must chain
            // with that wait, which blocks exactly `access.scope.stages`.
            if changes_layout
                && (current.writer.waits_other_queue(queue)
                    || changes && current.readers.waits_other_queue(queue))
            {
                source.stages |= access.scope.stages;
            }
            if !source.is_empty() || changes_layout {
                self.push_transition(BoundTransition {
                    batch,
                    phase,
                    resource: access.resource,
                    range: overlap,
                    source,
                    destination: access.scope,
                    old_layout: current.layout,
                    new_layout: access.layout,
                    source_family: vk::QUEUE_FAMILY_IGNORED,
                    destination_family: vk::QUEUE_FAMILY_IGNORED,
                    kind: BarrierKind::Memory,
                })?;
            }
            self.push_patch_slot(current.initial_slot, BoundPatch {
                batch,
                phase,
                resource: access.resource,
                range: overlap,
                expected_revision: current.revision,
                action: BoundPatchAction::Access {
                    scope: access.scope,
                    layout: access.layout,
                    queue,
                    family: domain.family,
                },
            })?;
            current.increment()?;
            if changes {
                current.writer = PendingFrontier::EMPTY;
                current.writer.add(queue, access.scope, batch);
                current.readers = PendingFrontier::EMPTY;
            } else {
                current.readers.add(queue, access.scope, batch);
            }
            current.layout = access.layout;
            current.layout_known = true;
            self.shadow.values[index] = current;
        }
        if !covered {
            return Err(Error::invalid_usage(
                "native resource has no physical initial state",
            ));
        }
        Ok(())
    }

    fn acquire(
        &mut self,
        mut current: ShadowCell,
        transfer: RangeTransfer,
        release: DependencyRef,
        batch: usize,
        phase: BarrierPhase,
    ) -> Result<ShadowCell> {
        let domain = self.bound.batches.values[batch].domain;
        if domain.family != transfer.destination_family {
            return Err(Error::invalid_usage(
                "ownership acquire queue family mismatch",
            ));
        }
        self.wait(batch, release, transfer.destination.stages, domain)?;
        self.push_transition(BoundTransition {
            batch,
            phase,
            resource: current.resource,
            range: transfer.range,
            // Execution-only source: the acquire (and its layout transition)
            // chains with the release wait at the same stages.
            source: AccessScope {
                stages: transfer.destination.stages,
                ..AccessScope::EMPTY
            },
            destination: transfer.destination,
            old_layout: transfer.old_layout,
            new_layout: transfer.new_layout,
            source_family: transfer.source_family,
            destination_family: transfer.destination_family,
            kind: BarrierKind::Acquire {
                pair: transfer.pair,
            },
        })?;
        self.push_patch_slot(current.initial_slot, BoundPatch {
            batch,
            phase,
            resource: current.resource,
            range: transfer.range,
            expected_revision: current.revision,
            action: BoundPatchAction::Acquire { transfer, release },
        })?;
        current.increment()?;
        current.owner = PendingOwner::Owned(domain.family);
        current.layout = transfer.new_layout;
        current.layout_known = true;
        current.writer = PendingFrontier::EMPTY;
        current.readers = PendingFrontier::EMPTY;
        current
            .writer
            .add(queue_id(domain)?, transfer.destination, batch);
        Ok(current)
    }

    fn pair(&mut self) -> Result<u64> {
        let pair = self.next_pair;
        self.next_pair = self
            .next_pair
            .checked_add(1)
            .ok_or_else(|| Error::unsupported("ownership pair identity exhausted"))?;
        Ok(pair)
    }

    fn wait_frontier(
        &mut self,
        frontier: PendingFrontier,
        batch: usize,
        stages: vk::PipelineStageFlags,
        domain: QueueDomain,
    ) -> Result<()> {
        for dependency in frontier.points.into_iter().flatten() {
            self.wait(batch, dependency, stages, domain)?;
        }
        Ok(())
    }

    pub(super) fn wait(
        &mut self,
        batch: usize,
        dependency: DependencyRef,
        stages: vk::PipelineStageFlags,
        domain: QueueDomain,
    ) -> Result<()> {
        let source = match dependency {
            DependencyRef::Point(point) => point.queue.index(),
            DependencyRef::Batch(source) => {
                if source == batch {
                    return Ok(());
                }
                self.bound
                    .batches
                    .values
                    .get(source)
                    .ok_or_else(|| Error::invalid_usage("native wait refers to an absent batch"))?
                    .domain
                    .queue_index as usize
            }
        };
        if source == domain.queue_index as usize {
            return Ok(());
        }
        // HOST is a terminal resource access, not a command-buffer stage.
        // Host readers wait for the real completion receipt; ownership submits
        // must wait at an executable device stage instead.
        let device_stages = stages & !vk::PipelineStageFlags::HOST;
        let stages = if device_stages.is_empty() { vk::PipelineStageFlags::ALL_COMMANDS } else { device_stages };
        if let Some(wait) = self
            .bound
            .waits
            .values
            .iter_mut()
            .find(|wait| wait.batch == batch && wait.dependency == dependency)
        {
            wait.stages |= stages;
            return Ok(());
        }
        self.push_wait(BoundWait {
            batch,
            dependency,
            stages: if stages.is_empty() {
                vk::PipelineStageFlags::ALL_COMMANDS
            } else {
                stages
            },
        })
    }
}
