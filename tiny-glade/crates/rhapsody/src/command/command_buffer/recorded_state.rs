//! Recorded resource-state patches and opaque-use ranges.
use ash::vk;
use std::sync::Arc;
use super::VulkanCommandBuffer;

pub(super) struct RecordedOpaqueUse {
    pub(super) cell: Arc<crate::tracking::SyncCell>,
    pub(super) range: OpaqueUseRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OpaqueUseRange {
    Whole,
    Bytes { offset: u64, size: u64 },
}

impl OpaqueUseRange {
    pub(super) fn checked(offset: u64, size: u64, limit: u64) -> crate::Result<Self> {
        if size == 0 || offset.checked_add(size).is_none_or(|end| end > limit) {
            return Err(crate::Error::invalid_usage(
                "GPU buffer use range is out of bounds",
            ));
        }
        Ok(Self::Bytes { offset, size })
    }
}

pub(super) struct RecordedStatePatch {
    pub(super) cell: Arc<crate::tracking::SyncCell>,
    pub(super) patch: crate::tracking::range_state::StatePatch,
}

pub(super) type OwnershipValidationState = Arc<crate::tracking::SyncCell>;

/// Recover the recording order inside each physical cell without stable
/// sort's temporary allocation. The full key is unique for recorded entries.
pub(super) fn order_recorded_patches(recorded: &mut [(usize, u8, usize, usize)]) {
    recorded.sort_unstable_by_key(|&(slot, kind, command, entry)| (slot, command, kind, entry));
}

impl VulkanCommandBuffer {
    /// Virtual state for consecutive setup/copy helpers in this same command.
    /// Committed physical state is unchanged and no future point is fabricated.
    pub(crate) fn with_recorded_state<R>(
        &self,
        cell: &Arc<crate::tracking::SyncCell>,
        initial: crate::tracking::range_state::NativeState,
        queue: crate::sync::QueueId,
        run: impl FnOnce(&crate::tracking::range_state::RangeStateTable) -> crate::Result<R>,
    ) -> crate::Result<R> {
        use crate::tracking::range_state::PatchAction;
        let entries = self
            .state_patches
            .as_slice()
            .iter()
            .filter(|entry| Arc::ptr_eq(&entry.cell, cell));
        // An acquire recorded earlier in this command is a valid predecessor
        // (the preview applies it like submission does); a release is not.
        if entries
            .clone()
            .any(|entry| matches!(entry.patch.action, PatchAction::Release(_)))
        {
            return Err(crate::Error::unsupported(
                "copy-helper preview cannot expose an unsubmitted ownership transfer",
            ));
        }
        let patches = crate::native::submit::SubmitScratch::<[crate::tracking::range_state::StatePatch; 8]>::try_collect(
            entries.clone().count(), &self.pool.device.memory.stats, entries.map(|entry| Ok(entry.patch)),
        )?;
        cell.preview_range_state(
            initial,
            &self.pool.device.memory.stats,
            &patches,
            queue,
            self.pool.family,
            run,
        )
    }

    pub(crate) fn recorded_state_queue(
        &self,
        cell: &Arc<crate::tracking::SyncCell>,
    ) -> Option<crate::sync::QueueId> {
        self.state_patches
            .as_slice()
            .iter()
            .rev()
            .find_map(|entry| {
                if !Arc::ptr_eq(&entry.cell, cell) {
                    return None;
                }
                match entry.patch.action {
                    crate::tracking::range_state::PatchAction::Access { queue, .. }
                    | crate::tracking::range_state::PatchAction::Unknown { queue, .. } => {
                        Some(queue)
                    }
                    _ => None,
                }
            })
    }

    pub(crate) fn recorded_access(
        &self,
        cell: &Arc<crate::tracking::SyncCell>,
        range: crate::tracking::OwnershipRange,
    ) -> Option<(crate::native::AccessScope, vk::ImageLayout)> {
        let entry = self.state_patches.as_slice().iter().rev().find(|entry| {
            Arc::ptr_eq(&entry.cell, cell)
                && crate::tracking::range_state::contains(entry.patch.range, range)
        })?;
        match entry.patch.action {
            crate::tracking::range_state::PatchAction::Access { scope, layout, .. } => {
                Some((scope, layout))
            }
            _ => None,
        }
    }

    pub(crate) fn has_opaque_overlap(
        &self,
        cell: &Arc<crate::tracking::SyncCell>,
        range: crate::tracking::OwnershipRange,
    ) -> bool {
        self.range_uses.as_slice().iter().any(|entry| {
            Arc::ptr_eq(&entry.cell, cell)
                && match entry.range {
                    OpaqueUseRange::Whole => true,
                    OpaqueUseRange::Bytes { offset, size } => {
                        crate::tracking::range_state::intersection(
                            crate::tracking::OwnershipRange::Buffer { offset, size },
                            range,
                        )
                        .is_some()
                    }
                }
        })
    }

    pub(crate) fn recorded_access_covers(
        &self,
        cell: &Arc<crate::tracking::SyncCell>,
        range: crate::tracking::OwnershipRange,
        mut predicate: impl FnMut(crate::native::AccessScope, vk::ImageLayout) -> bool,
    ) -> crate::Result<bool> {
        if self.has_opaque_overlap(cell, range) {
            return Ok(false);
        }
        let Some(queue) = self.recorded_state_queue(cell) else {
            return Ok(false);
        };
        self.with_recorded_state(
            cell,
            crate::tracking::range_state::NativeState::default(),
            queue,
            |table| {
                if !crate::tracking::range_state::contains(table.domain(), range) {
                    return Ok(false);
                }
                let mut touched = false;
                for current in table.prepared_cells() {
                    if let Some(overlap) =
                        crate::tracking::range_state::intersection(current.range, range)
                    {
                        touched = true;
                        let Some((scope, layout)) = self.recorded_access(cell, overlap) else {
                            return Ok(false);
                        };
                        if !predicate(scope, layout) {
                            return Ok(false);
                        }
                    }
                }
                Ok(touched)
            },
        )
    }

    pub(crate) fn record_state_patch(
        &mut self,
        cell: &Arc<crate::tracking::SyncCell>,
        patch: crate::tracking::range_state::StatePatch,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            cell.validate_device(self.pool.device.generation())?;
            crate::tracking::range_state::validate_range(patch.range)?;
            if !cell
                .ownership_range()
                .is_some_and(|domain| crate::tracking::range_state::contains(domain, patch.range))
            {
                return Err(crate::Error::invalid_usage(
                    "state patch exceeds native resource domain",
                ));
            }
            self.state_patches.add(RecordedStatePatch {
                cell: cell.clone(),
                patch,
            })?;
            self.add_use_cell(cell)
        })();
        self.remember_metadata_error(&result);
        result
    }

    pub(super) fn note_opaque_use(
        &mut self,
        cell: Arc<crate::tracking::SyncCell>,
        range: OpaqueUseRange,
    ) -> crate::Result<()> {
        if cell.ownership_range().is_none()
            || self.range_uses.as_slice().iter().any(|previous| {
                Arc::ptr_eq(&previous.cell, &cell)
                    && (previous.range == OpaqueUseRange::Whole || previous.range == range)
            })
        {
            return Ok(());
        }
        self.range_uses.add(RecordedOpaqueUse { cell, range })?;
        Ok(())
    }
}
