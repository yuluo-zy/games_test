//! Range cells, state patches and their configuration.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::native::access_table::AccessScope;
use crate::sync::QueueId;
use crate::sync::SyncPoint;
use crate::tracking::ResourceSharing;
use super::frontier::Frontier;
use super::native::NativeRange;
use super::native::NativeState;
use super::ops::contains;
use super::ops::released_range_error;
use super::ops::validate_transfer;
use super::owner::OwnershipDenial;
use super::owner::RangeOwner;
use super::owner::RangeTransfer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeCell {
    pub range: NativeRange,
    pub state: NativeState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchAction {
    Access {
        scope: AccessScope,
        layout: vk::ImageLayout,
        queue: QueueId,
        family: u32,
    },
    Release(RangeTransfer),
    Acquire {
        transfer: RangeTransfer,
        release_point: SyncPoint,
    },
    /// An opaque native use cannot assert that an image kept its old layout.
    /// Its lifetime and completion are still recorded on the physical resource.
    Unknown {
        queue: QueueId,
        family: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatePatch {
    pub range: NativeRange,
    pub expected_revision: u64,
    pub action: PatchAction,
}

#[derive(Clone, Copy, Debug)]
pub struct RangeStateConfig {
    pub max_cells: usize,
    pub max_cpu_bytes: usize,
}

impl Default for RangeStateConfig {
    fn default() -> Self {
        Self {
            max_cells: 4096,
            max_cpu_bytes: 16 << 20,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PreparedCell {
    pub(super) cell: RangeCell,
    pub(super) writer: bool,
    pub(super) readers: bool,
    pub(super) release: Option<RangeTransfer>,
}

impl From<RangeCell> for PreparedCell {
    fn from(cell: RangeCell) -> Self {
        Self {
            cell,
            writer: false,
            readers: false,
            release: None,
        }
    }
}

pub(super) fn apply_preview(
    prepared: &mut PreparedCell,
    action: PatchAction,
    sharing: ResourceSharing,
    queue: QueueId,
    family: u32,
) -> Result<()> {
    let state = &mut prepared.cell.state;
    match action {
        PatchAction::Access {
            scope,
            layout,
            queue: expected_queue,
            family: expected_family,
        } => {
            if queue != expected_queue || family != expected_family || prepared.release.is_some() {
                return Err(Error::invalid_usage(
                    "range access uses another queue or a pending release",
                ));
            }
            let discarded = state.contents_discarded(prepared.cell.range);
            match state.owner.admits_discarding(sharing, family, discarded) {
                Ok(()) => {}
                Err(OwnershipDenial::NotShared) => {
                    return Err(Error::invalid_usage("concurrent access family is absent"));
                }
                Err(OwnershipDenial::Released) => return Err(released_range_error()),
                Err(OwnershipDenial::OtherFamily) => {
                    return Err(Error::invalid_usage(
                        "range access needs a matching ownership acquire",
                    ));
                }
            }
            if !sharing.is_concurrent() && state.owner != RangeOwner::Owned(family) {
                // Unclaimed, or a discarded range last owned by another family.
                state.owner = RangeOwner::Owned(family);
            }
            // A layout conversion is a visibility checkpoint even for a read.
            // Later readers on another VkQueue must wait its actual point.
            if scope.write || !state.layout_known || state.layout != layout {
                state.writer = Frontier::EMPTY;
                state.writer.add(queue, scope);
                state.readers = Frontier::EMPTY;
                prepared.writer = true;
                prepared.readers = false;
            } else {
                state.readers.add(queue, scope);
                prepared.readers = true;
            }
            state.layout = layout;
            state.layout_known = true;
        }
        PatchAction::Release(transfer) => {
            validate_transfer(transfer)?;
            if sharing.is_concurrent()
                || family != transfer.source_family
                || prepared.release.is_some()
                || state.owner != RangeOwner::Owned(family)
                || !contains(transfer.range, prepared.cell.range)
                || !state.layout_known
                || state.layout != transfer.old_layout
            {
                return Err(Error::invalid_usage(
                    "release range, owner or old layout does not match",
                ));
            }
            prepared.release = Some(transfer);
            state.layout = transfer.new_layout;
        }
        PatchAction::Acquire {
            transfer,
            release_point,
        } => {
            validate_transfer(transfer)?;
            if sharing.is_concurrent()
                || family != transfer.destination_family
                || prepared.release.is_some()
                || state.owner
                    != (RangeOwner::Released {
                        transfer,
                        point: release_point,
                    })
                || !contains(transfer.range, prepared.cell.range)
            {
                return Err(Error::invalid_usage(
                    "acquire does not match the actual released range and point",
                ));
            }
            state.owner = RangeOwner::Owned(family);
            state.layout = transfer.new_layout;
            state.layout_known = true;
            // Acquire checkpoints visibility but does not claim logical contents
            // were initialized; temporal/version validity lives in the graph.
            state.writer = Frontier::EMPTY;
            state.readers = Frontier::EMPTY;
            state.writer.add(queue, transfer.destination);
            prepared.writer = true;
            prepared.readers = false;
        }
        PatchAction::Unknown {
            queue: expected_queue,
            family: expected_family,
        } => {
            if queue != expected_queue
                || family != expected_family
                || prepared.release.is_some()
                || !sharing.allows_family(family)
            {
                return Err(Error::invalid_usage(
                    "opaque access uses another queue or pending release",
                ));
            }
            match state.owner.admits(sharing, family) {
                Ok(()) => {}
                Err(OwnershipDenial::Released) => return Err(released_range_error()),
                Err(_) => {
                    return Err(Error::invalid_usage(
                        "opaque access requires ownership acquire",
                    ));
                }
            }
            if !sharing.is_concurrent() && state.owner == RangeOwner::Unclaimed {
                state.owner = RangeOwner::Owned(family);
            }
            state.writer = Frontier::EMPTY;
            state.writer.add(
                queue,
                AccessScope {
                    stages: vk::PipelineStageFlags::ALL_COMMANDS,
                    access: vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
                    write: true,
                },
            );
            state.readers = Frontier::EMPTY;
            state.layout_known = matches!(prepared.cell.range, NativeRange::Buffer { .. });
            prepared.writer = true;
            prepared.readers = false;
        }
    }
    Ok(())
}
