//! Command recording shared by buffer and texture uploads: queue targets,
//! state-derived barrier batches and queue-family ownership transfers.
use crate::{
    Error, Result,
    device::Device,
    raw::{
        barrier::{BarrierDispatch, SynchronizationScope},
        buffer::Buffer,
        command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
        device::DeviceQueue,
    },
    resource::{
        OwnershipRange,
        range_state::{
            AccessScope, NativeState, PatchAction, RangeOwner, RangeTransfer, StatePatch, contains,
            intersection,
        },
    },
    sync::{QueueId, SyncPoint},
};
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
            handle: device.queue,
            family: device.queue_family,
            index: 0,
        });
    }
    device
        .additional_queues
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
                let owned = match current.state.owner {
                    RangeOwner::Unclaimed => true,
                    RangeOwner::Owned(owner) => owner == family,
                    RangeOwner::Released { .. } => false,
                };
                if !sharing.allows_family(family) || !sharing.is_concurrent() && !owned {
                    return Err(Error::unsupported(
                        "upload destination requires a matching ownership acquire",
                    ));
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
            cell.clone(),
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
    for &(offset, size) in ranges {
        scope.buffers.push(
            vk::BufferMemoryBarrier::default()
                .src_access_mask(source.access)
                .dst_access_mask(TRANSFER_WRITE.access)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(destination.handle)
                .offset(offset)
                .size(size),
        );
    }
    issue(command, &scope)
}

/// Make TRANSFER writes of `ranges` visible to [`BUFFER_CONSUMERS`] on the same
/// queue with one barrier batch.
pub(super) fn record_buffer_visibility(
    command: &VulkanCommandBuffer,
    buffer: vk::Buffer,
    ranges: &[(u64, u64)],
) -> Result<()> {
    let mut scope = SynchronizationScope::new(TRANSFER_WRITE.stages, BUFFER_CONSUMERS.stages);
    for &(offset, size) in ranges {
        scope.buffers.push(
            vk::BufferMemoryBarrier::default()
                .src_access_mask(TRANSFER_WRITE.access)
                .dst_access_mask(BUFFER_CONSUMERS.access)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(buffer)
                .offset(offset)
                .size(size),
        );
    }
    issue(command, &scope)
}

/// Identity of release/acquire pairs created by transfer uploads. Kept in a
/// namespace disjoint from per-graph pair numbering.
static NEXT_PAIR: AtomicU64 = AtomicU64::new(1 << 62);

/// A release recorded on the transfer queue, to be acquired on graphics.
#[derive(Clone, Copy, Debug)]
pub(super) struct PendingAcquire {
    pub transfer: RangeTransfer,
    pub revision: u64,
}

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

fn ownership_barrier(
    buffer: vk::Buffer,
    transfer: &RangeTransfer,
    source_access: vk::AccessFlags,
    destination_access: vk::AccessFlags,
) -> Result<vk::BufferMemoryBarrier<'static>> {
    let OwnershipRange::Buffer { offset, size } = transfer.range else {
        return Err(Error::invalid_usage(
            "buffer ownership transfer of an image range",
        ));
    };
    Ok(vk::BufferMemoryBarrier::default()
        .src_access_mask(source_access)
        .dst_access_mask(destination_access)
        .src_queue_family_index(transfer.source_family)
        .dst_queue_family_index(transfer.destination_family)
        .buffer(buffer)
        .offset(offset)
        .size(size))
}

/// Record release patches and one release barrier batch for written pieces.
pub(super) fn record_buffer_release(
    command: &mut VulkanCommandBuffer,
    destination: &Arc<Buffer>,
    pieces: &[WrittenPiece],
    destination_family: u32,
    released: &mut Vec<PendingAcquire>,
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
            destination.last_use.clone(),
            StatePatch {
                range: piece.range,
                expected_revision: piece.revision,
                action: PatchAction::Release(transfer),
            },
        )?;
        scope.buffers.push(ownership_barrier(
            destination.handle,
            &transfer,
            TRANSFER_WRITE.access,
            vk::AccessFlags::empty(),
        )?);
        released.push(PendingAcquire {
            transfer,
            revision: piece.revision + 1,
        });
    }
    issue(command, &scope)
}

/// Record the matching acquires (one barrier batch) on the graphics queue.
pub(super) fn record_buffer_acquire(
    command: &mut VulkanCommandBuffer,
    destination: &Arc<Buffer>,
    acquires: &[PendingAcquire],
    release_point: SyncPoint,
) -> Result<()> {
    let mut scope =
        SynchronizationScope::new(vk::PipelineStageFlags::empty(), BUFFER_CONSUMERS.stages);
    for acquire in acquires {
        command.record_state_patch(
            destination.last_use.clone(),
            StatePatch {
                range: acquire.transfer.range,
                expected_revision: acquire.revision,
                action: PatchAction::Acquire {
                    transfer: acquire.transfer,
                    release_point,
                },
            },
        )?;
        scope.buffers.push(ownership_barrier(
            destination.handle,
            &acquire.transfer,
            vk::AccessFlags::empty(),
            acquire.transfer.destination.access,
        )?);
    }
    // SAFETY: the acquire touches exactly the patched ranges.
    unsafe {
        command.retain_recorded(destination.clone())?;
    }
    issue(command, &scope)
}

pub(super) fn issue(command: &VulkanCommandBuffer, scope: &SynchronizationScope) -> Result<()> {
    if scope.is_empty() {
        return Ok(());
    }
    let device = &command.pool.device;
    // SAFETY: the command is recording on this device and every barrier
    // resource is retained by it through completion.
    unsafe { BarrierDispatch::new(device).issue(device, command.handle, scope) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_uses_graphics_when_aliased_or_ownership_is_unavailable() {
        assert_eq!(upload_route(0, None, true, false), UploadRoute::Graphics);
        assert_eq!(
            upload_route(0, Some(1), false, false),
            UploadRoute::Graphics
        );
        assert_eq!(
            upload_route(0, Some(0), true, false),
            UploadRoute::SameFamily
        );
        assert_eq!(
            upload_route(0, Some(1), true, true),
            UploadRoute::SameFamily
        );
        assert_eq!(
            upload_route(0, Some(1), true, false),
            UploadRoute::OwnershipTransfer
        );
    }

    #[test]
    fn release_and_acquire_barriers_match_exactly() {
        let range = OwnershipRange::Buffer {
            offset: 64,
            size: 128,
        };
        let first = buffer_transfer(range, 2, 0);
        let second = buffer_transfer(range, 2, 0);
        assert_ne!(first.pair, second.pair);
        assert!(first.pair >= 1 << 62);
        crate::resource::range_state::validate_transfer(first).unwrap();
        let release = ownership_barrier(
            vk::Buffer::null(),
            &first,
            TRANSFER_WRITE.access,
            vk::AccessFlags::empty(),
        )
        .unwrap();
        let acquire = ownership_barrier(
            vk::Buffer::null(),
            &first,
            vk::AccessFlags::empty(),
            first.destination.access,
        )
        .unwrap();
        for barrier in [release, acquire] {
            assert_eq!(barrier.src_queue_family_index, 2);
            assert_eq!(barrier.dst_queue_family_index, 0);
            assert_eq!((barrier.offset, barrier.size), (64, 128));
        }
        assert_eq!(release.dst_access_mask, vk::AccessFlags::empty());
        assert_eq!(acquire.src_access_mask, vk::AccessFlags::empty());
        assert!(
            ownership_barrier(
                vk::Buffer::null(),
                &RangeTransfer {
                    range: OwnershipRange::Image {
                        aspects: 1,
                        base_mip: 0,
                        mip_count: 1,
                        base_layer: 0,
                        layer_count: 1
                    },
                    ..first
                },
                vk::AccessFlags::empty(),
                vk::AccessFlags::empty()
            )
            .is_err()
        );
    }
}
