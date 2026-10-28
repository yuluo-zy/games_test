//! Command recording shared by buffer and texture uploads: queue targets,
//! state-derived barrier batches and queue-family ownership transfers.
use crate::{Error, Result, device::Device, sync::{QueueId, SyncPoint}};
use crate::tracking::OwnershipRange;
use crate::tracking::range_state::{NativeState, OwnershipDenial, PatchAction, RangeTransfer, StatePatch, contains, intersection, released_range_error};
use crate::command::barrier::{BarrierTarget, OwnershipHalf, RangeBarrier, SynchronizationScope};
use crate::command::command_buffer::{VulkanCommandBuffer, VulkanCommandPool};
use crate::device::DeviceQueue;
use crate::resource::buffer::Buffer;
use crate::command::{AcquireTarget, PendingAcquire};
use crate::native::AccessScope;
use ash::vk;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

/// Copy writes performed by uploads.
pub(super) const TRANSFER_WRITE: AccessScope = AccessScope {
    stages: vk::PipelineStageFlags::TRANSFER,
    access: vk::AccessFlags::TRANSFER_WRITE,
    write: true,
};
/// Every read a buffer upload can feed on a graphics queue. Used as the
/// destination of the visibility barrier/ownership acquire after a copy, so
/// consumers that do not track resource state (opaque raw users) are covered.
pub(crate) const BUFFER_CONSUMERS: AccessScope = AccessScope {
    stages: vk::PipelineStageFlags::from_raw(
        vk::PipelineStageFlags::DRAW_INDIRECT.as_raw()
            | vk::PipelineStageFlags::VERTEX_INPUT.as_raw()
            | vk::PipelineStageFlags::VERTEX_SHADER.as_raw()
            | vk::PipelineStageFlags::FRAGMENT_SHADER.as_raw()
            | vk::PipelineStageFlags::COMPUTE_SHADER.as_raw()
            | vk::PipelineStageFlags::TRANSFER.as_raw(),
    ),
    access: vk::AccessFlags::from_raw(
        vk::AccessFlags::INDIRECT_COMMAND_READ.as_raw()
            | vk::AccessFlags::INDEX_READ.as_raw()
            | vk::AccessFlags::VERTEX_ATTRIBUTE_READ.as_raw()
            | vk::AccessFlags::UNIFORM_READ.as_raw()
            | vk::AccessFlags::SHADER_READ.as_raw()
            | vk::AccessFlags::TRANSFER_READ.as_raw(),
    ),
    write: false,
};

/// One device queue with its own command pool.
pub(super) struct QueueTarget {
    pub queue: DeviceQueue,
    pub id: QueueId,
    pub pool: Arc<VulkanCommandPool>,
}
impl QueueTarget {
    pub fn new(device: &Arc<Device>, id: QueueId) -> Result<Self> {
        let queue = device_queue(device, id)?;
        Ok(Self {
            queue,
            id,
            pool: VulkanCommandPool::new_for_family(device.clone(), queue.family)?,
        })
    }
    pub fn family(&self) -> u32 {
        self.queue.family
    }
}

pub(super) fn device_queue(device: &Device, id: QueueId) -> Result<DeviceQueue> {
    if id.index() == 0 {
        return Ok(DeviceQueue {
            handle: device.queues.primary,
            family: device.queues.primary_family,
            index: 0,
        });
    }
    device
        .queues
        .additional
        .get(id.index() - 1)
        .copied()
        .ok_or_else(|| Error::invalid_usage("planned queue is not part of this device"))
}

/// Which queue a buffer upload records on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UploadRoute {
    /// The graphics queue; no ownership transfer.
    Graphics,
    /// A distinct transfer queue of the graphics family: ordering through the
    /// timeline semaphore only, no ownership transfer.
    SameFamily,
    /// A transfer queue of another family: release on the transfer queue,
    /// acquire on the graphics queue.
    OwnershipTransfer,
}

/// Pure route policy. `transfer_family` is `None` when the queue plan aliases
/// transfer to graphics. `transfer_accessible` reports whether the
/// destination's sharing and current range ownership allow the transfer family.
pub(crate) fn upload_route(
    graphics_family: u32,
    transfer_family: Option<u32>,
    transfer_accessible: bool,
    concurrent: bool,
) -> UploadRoute {
    match transfer_family {
        None => UploadRoute::Graphics,
        Some(_) if !transfer_accessible => UploadRoute::Graphics,
        Some(family) if family == graphics_family || concurrent => UploadRoute::SameFamily,
        Some(_) => UploadRoute::OwnershipTransfer,
    }
}

/// A destination piece written in this command and its revision after it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct WrittenPiece {
    pub range: OwnershipRange,
    pub revision: u64,
}

/// Record TRANSFER write state for every destination range and issue at most
/// one barrier batch whose source scope is the union of the actual previous
/// uses on this queue. Ranges must be pairwise disjoint.
pub(super) fn record_buffer_writes(
    command: &mut VulkanCommandBuffer,
    destination: &Arc<Buffer>,
    target: &QueueTarget,
    ranges: &[(u64, u64)],
    pieces: &mut Vec<WrittenPiece>,
) -> Result<()> {
    let family = target.family();
    let sharing = destination.sharing();
    let cell = destination.last_use.clone();
    let mut source = AccessScope::EMPTY;
    pieces.clear();
    command.with_recorded_state(&cell, NativeState::default(), target.id, |table| {
        for &(offset, size) in ranges {
            let range = OwnershipRange::Buffer { offset, size };
            if !contains(table.domain(), range) {
                return Err(Error::invalid_usage("upload range exceeds the buffer"));
            }
            for current in table.prepared_cells() {
                let Some(overlap) = intersection(current.range, range) else {
                    continue;
                };
                match current.state.owner.admits(sharing, family) {
                    Ok(()) => {}
                    Err(OwnershipDenial::Released) => return Err(released_range_error()),
                    Err(_) => {
                        return Err(Error::unsupported(
                            "upload destination requires a matching ownership acquire",
                        ));
                    }
                }
                source = source.union(current.state.source_scope(target.id, true));
                pieces.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
                pieces.push(WrittenPiece {
                    range: overlap,
                    revision: current.state.revision,
                });
            }
        }
        Ok(())
    })?;
    for piece in pieces.iter_mut() {
        command.record_state_patch(
            &cell,
            StatePatch {
                range: piece.range,
                expected_revision: piece.revision,
                action: PatchAction::Access {
                    scope: TRANSFER_WRITE,
                    layout: vk::ImageLayout::UNDEFINED,
                    queue: target.id,
                    family,
                },
            },
        )?;
        piece.revision += 1;
    }
    // SAFETY: the copy writes exactly the patched ranges.
    unsafe {
        command.retain_recorded(destination.clone())?;
    }
    if source.stages.is_empty() && source.access.is_empty() {
        // No earlier use on this queue: submission order plus the semaphore
        // waits derived from the tracked state already order the copy.
        return Ok(());
    }
    let mut scope = SynchronizationScope::new(source.stages, TRANSFER_WRITE.stages);
    push_buffer_ranges(
        &mut scope,
        destination.handle,
        ranges,
        source.access,
        TRANSFER_WRITE.access,
    )?;
    scope.issue_into(command)
}

/// Make TRANSFER writes of `ranges` visible to [`BUFFER_CONSUMERS`] on the same
/// queue with one barrier batch.
pub(super) fn record_buffer_visibility(
    command: &VulkanCommandBuffer,
    buffer: vk::Buffer,
    ranges: &[(u64, u64)],
) -> Result<()> {
    let mut scope = SynchronizationScope::new(TRANSFER_WRITE.stages, BUFFER_CONSUMERS.stages);
    push_buffer_ranges(
        &mut scope,
        buffer,
        ranges,
        TRANSFER_WRITE.access,
        BUFFER_CONSUMERS.access,
    )?;
    scope.issue_into(command)
}

/// Append one queue-local barrier per `(offset, size)` range of `buffer`.
fn push_buffer_ranges(
    scope: &mut SynchronizationScope,
    buffer: vk::Buffer,
    ranges: &[(u64, u64)],
    source: vk::AccessFlags,
    destination: vk::AccessFlags,
) -> Result<()> {
    for &(offset, size) in ranges {
        scope.push(
            BarrierTarget::Buffer(buffer),
            &RangeBarrier::local(
                OwnershipRange::Buffer { offset, size },
                source,
                destination,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::UNDEFINED,
            ),
        )?;
    }
    Ok(())
}

/// Identity of release/acquire pairs created by transfer uploads. Kept in a
/// namespace disjoint from per-graph pair numbering.
static NEXT_PAIR: AtomicU64 = AtomicU64::new(1 << 62);

pub(super) fn buffer_transfer(
    range: OwnershipRange,
    source_family: u32,
    destination_family: u32,
) -> RangeTransfer {
    RangeTransfer {
        pair: NEXT_PAIR.fetch_add(1, Ordering::Relaxed),
        source_family,
        destination_family,
        range,
        source: TRANSFER_WRITE,
        destination: BUFFER_CONSUMERS,
        old_layout: vk::ImageLayout::UNDEFINED,
        new_layout: vk::ImageLayout::UNDEFINED,
    }
}

/// Record release patches and one release barrier batch for written pieces.
pub(super) fn record_buffer_release(
    command: &mut VulkanCommandBuffer,
    destination: &Arc<Buffer>,
    pieces: &[WrittenPiece],
    destination_family: u32,
    released: &mut Vec<RangeTransfer>,
) -> Result<()> {
    let source_family = command.pool.family;
    let mut scope =
        SynchronizationScope::new(TRANSFER_WRITE.stages, vk::PipelineStageFlags::empty());
    released.clear();
    released
        .try_reserve(pieces.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    for piece in pieces {
        let transfer = buffer_transfer(piece.range, source_family, destination_family);
        command.record_state_patch(
            &destination.last_use,
            StatePatch {
                range: piece.range,
                expected_revision: piece.revision,
                action: PatchAction::Release(transfer),
            },
        )?;
        scope.push(
            BarrierTarget::Buffer(destination.handle),
            &RangeBarrier::ownership(&transfer, OwnershipHalf::Release),
        )?;
        released.push(transfer);
    }
    scope.issue_into(command)
}

/// Ownership transfer of a whole uploaded image piece. The release performs the
/// TRANSFER_DST_OPTIMAL -> SHADER_READ_ONLY_OPTIMAL transition; the acquire
/// repeats exactly the same layouts, families and subresource range.
pub(super) fn image_transfer(
    range: OwnershipRange,
    source_family: u32,
    destination_family: u32,
) -> RangeTransfer {
    RangeTransfer {
        pair: NEXT_PAIR.fetch_add(1, Ordering::Relaxed),
        source_family,
        destination_family,
        range,
        source: TRANSFER_WRITE,
        destination: crate::command::texture_copy::IMAGE_CONSUMERS,
        old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    }
}

/// Register the acquires matching `released` (all recorded by the command
/// submitted at `release_point`) on the device: they are recorded by the next
/// render-graph batch or graphics-queue transfer command on the destination
/// family, or by [`super::TransferManager::flush_acquires`]. Room must have
/// been reserved with `Device::reserve_pending_acquires` before the release
/// was submitted.
pub(super) fn queue_acquires(
    device: &Device,
    reservation: &mut crate::command::PendingAcquireReservation,
    target: &AcquireTarget,
    released: &[RangeTransfer],
    release_point: SyncPoint,
) -> Result<()> {
    for transfer in released {
        device.queue_pending_acquire(reservation, PendingAcquire {
            target: target.clone(),
            transfer: *transfer,
            release_point,
        })?;
    }
    Ok(())
}

/// Release patches and barriers for every written image piece. Pure: the
/// caller records the patches and issues one barrier batch.
pub(super) fn image_release_program(
    image: vk::Image,
    pieces: &[WrittenPiece],
    source_family: u32,
    destination_family: u32,
    patches: &mut Vec<StatePatch>,
    scope: &mut SynchronizationScope,
    released: &mut Vec<RangeTransfer>,
) -> Result<()> {
    patches.clear();
    released.clear();
    patches
        .try_reserve(pieces.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    released
        .try_reserve(pieces.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    scope
        .images
        .try_reserve(pieces.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    for piece in pieces {
        let transfer = image_transfer(piece.range, source_family, destination_family);
        patches.push(StatePatch {
            range: piece.range,
            expected_revision: piece.revision,
            action: PatchAction::Release(transfer),
        });
        scope.push(
            BarrierTarget::Image(image),
            &RangeBarrier::ownership(&transfer, OwnershipHalf::Release),
        )?;
        released.push(transfer);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
