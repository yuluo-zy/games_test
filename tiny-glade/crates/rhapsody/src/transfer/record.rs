//! Command recording shared by buffer and texture uploads: queue targets,
//! state-derived barrier batches and queue-family ownership transfers.
use crate::{
    Error, Result,
    device::{AcquireTarget, Device, PendingAcquire},
    raw::{
        barrier::{BarrierTarget, OwnershipHalf, RangeBarrier, SynchronizationScope},
        buffer::Buffer,
        command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
        device::DeviceQueue,
    },
    resource::{
        OwnershipRange,
        range_state::{
            AccessScope, NativeState, OwnershipDenial, PatchAction, RangeTransfer, StatePatch,
            contains, intersection, released_range_error,
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
            destination.last_use.clone(),
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
        destination: super::texture::IMAGE_CONSUMERS,
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
    target: &AcquireTarget,
    released: &[RangeTransfer],
    release_point: SyncPoint,
) -> Result<()> {
    for transfer in released {
        device.queue_pending_acquire(PendingAcquire {
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
mod tests {
    use super::*;
    use crate::resource::range_state::RangeOwner;

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
        let release = RangeBarrier::ownership(&first, OwnershipHalf::Release)
            .buffer(vk::Buffer::null())
            .unwrap();
        let acquire = RangeBarrier::ownership(&first, OwnershipHalf::Acquire)
            .buffer(vk::Buffer::null())
            .unwrap();
        for barrier in [release, acquire] {
            assert_eq!(barrier.src_queue_family_index, 2);
            assert_eq!(barrier.dst_queue_family_index, 0);
            assert_eq!((barrier.offset, barrier.size), (64, 128));
        }
        assert_eq!(release.src_access_mask, TRANSFER_WRITE.access);
        assert_eq!(release.dst_access_mask, vk::AccessFlags::empty());
        assert_eq!(acquire.src_access_mask, vk::AccessFlags::empty());
        assert_eq!(acquire.dst_access_mask, first.destination.access);
        assert!(
            RangeBarrier::ownership(
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
                OwnershipHalf::Release
            )
            .buffer(vk::Buffer::null())
            .is_err()
        );
    }

    fn image_range(aspects: vk::ImageAspectFlags, mips: u32, layers: u32) -> OwnershipRange {
        OwnershipRange::Image {
            aspects: aspects.as_raw(),
            base_mip: 0,
            mip_count: mips,
            base_layer: 0,
            layer_count: layers,
        }
    }

    #[test]
    fn image_release_and_acquire_barriers_pair_layouts_families_and_subresources() {
        let image = vk::Image::null();
        for range in [
            image_range(vk::ImageAspectFlags::COLOR, 1, 1),
            // Cube with a full mip chain: six faces are six layers.
            image_range(vk::ImageAspectFlags::COLOR, 9, 6),
            // Cube array and plain 2D arrays.
            image_range(vk::ImageAspectFlags::COLOR, 4, 24),
            image_range(
                vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL,
                3,
                2,
            ),
        ] {
            let piece = WrittenPiece { range, revision: 4 };
            let mut patches = Vec::new();
            let mut released = Vec::new();
            let mut release = SynchronizationScope::new(
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::empty(),
            );
            image_release_program(
                image,
                &[piece],
                3,
                0,
                &mut patches,
                &mut release,
                &mut released,
            )
            .unwrap();
            assert_eq!(patches.len(), 1);
            assert_eq!(patches[0].expected_revision, 4);
            assert!(matches!(patches[0].action, PatchAction::Release(t) if t == released[0]));
            let a = RangeBarrier::ownership(&released[0], OwnershipHalf::Acquire)
                .image(image)
                .unwrap();
            let r = release.images[0];
            assert_eq!((r.old_layout, r.new_layout), (a.old_layout, a.new_layout));
            assert_eq!(r.old_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
            assert_eq!(r.new_layout, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            assert_eq!((r.src_queue_family_index, r.dst_queue_family_index), (3, 0));
            assert_eq!(
                (a.src_queue_family_index, a.dst_queue_family_index),
                (r.src_queue_family_index, r.dst_queue_family_index)
            );
            assert_eq!(
                format!("{:?}", r.subresource_range),
                format!("{:?}", a.subresource_range)
            );
            let OwnershipRange::Image {
                aspects,
                mip_count,
                layer_count,
                ..
            } = range
            else {
                unreachable!()
            };
            assert_eq!(r.subresource_range.aspect_mask.as_raw(), aspects);
            assert_eq!(r.subresource_range.level_count, mip_count);
            assert_eq!(r.subresource_range.layer_count, layer_count);
            assert_eq!(r.src_access_mask, vk::AccessFlags::TRANSFER_WRITE);
            assert_eq!(r.dst_access_mask, vk::AccessFlags::empty());
            assert_eq!(a.src_access_mask, vk::AccessFlags::empty());
            assert_eq!(a.dst_access_mask, vk::AccessFlags::SHADER_READ);
        }
        assert!(
            RangeBarrier::ownership(
                &buffer_transfer(OwnershipRange::Buffer { offset: 0, size: 4 }, 1, 0),
                OwnershipHalf::Release
            )
            .image(image)
            .is_err()
        );
    }

    fn image_cell(
        range: OwnershipRange,
        stats: &Arc<crate::raw::allocator::AllocatorStats>,
    ) -> Arc<crate::resource::SyncCell> {
        let cell = Arc::new(
            crate::resource::SyncCell::for_device(crate::device::DeviceGeneration::next())
                .with_ownership_range(range),
        );
        cell.with_range_state(NativeState::default(), stats, |_| Ok(()))
            .unwrap();
        cell
    }

    /// The acquire patches the device records for `released` pieces whose
    /// revision before the release was `revision`.
    fn acquire_patches(
        released: &[RangeTransfer],
        revision: u64,
        release_point: SyncPoint,
    ) -> Vec<StatePatch> {
        released
            .iter()
            .map(|transfer| StatePatch {
                range: transfer.range,
                expected_revision: revision + 1,
                action: PatchAction::Acquire {
                    transfer: *transfer,
                    release_point,
                },
            })
            .collect()
    }

    fn upload_access(range: OwnershipRange, queue: QueueId, family: u32) -> StatePatch {
        StatePatch {
            range,
            expected_revision: 0,
            action: PatchAction::Access {
                scope: TRANSFER_WRITE,
                layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                queue,
                family,
            },
        }
    }

    #[test]
    fn released_image_is_owned_by_graphics_in_its_final_layout_after_the_acquire() {
        let stats = Arc::new(crate::raw::allocator::AllocatorStats::default());
        let range = image_range(vk::ImageAspectFlags::COLOR, 5, 6);
        let cell = image_cell(range, &stats);
        let (transfer_queue, graphics_queue) = (QueueId::new(1).unwrap(), QueueId::new(0).unwrap());
        let (transfer_family, graphics_family) = (2, 0);
        let mut patches = vec![upload_access(range, transfer_queue, transfer_family)];
        let mut release_patches = Vec::new();
        let mut released = Vec::new();
        let mut scope = SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::empty(),
        );
        image_release_program(
            vk::Image::null(),
            &[WrittenPiece { range, revision: 1 }],
            transfer_family,
            graphics_family,
            &mut release_patches,
            &mut scope,
            &mut released,
        )
        .unwrap();
        patches.extend(release_patches);
        let release_point = SyncPoint {
            queue: transfer_queue,
            value: 3,
        };
        {
            let _gate = cell.lock_submission();
            cell.prepare_range_state(&patches, transfer_queue, transfer_family, &stats)
                .unwrap();
            cell.publish_range_state(release_point);
        }
        // Graphics cannot touch the released range before the acquire.
        assert!(cell.validate_queue_range(graphics_family, range).is_err());
        let acquire_patches = acquire_patches(&released, 1, release_point);
        let acquire_point = SyncPoint {
            queue: graphics_queue,
            value: 8,
        };
        {
            let _gate = cell.lock_submission();
            cell.prepare_range_state(&acquire_patches, graphics_queue, graphics_family, &stats)
                .unwrap();
            assert!(
                cell.prepared_range_staged_dependencies()
                    .points()
                    .is_reached(&{
                        let mut completed = crate::sync::CompletedValues::default();
                        completed.raise(transfer_queue, 3);
                        completed
                    }),
                "the acquire waits exactly for the release point"
            );
            cell.publish_range_state(acquire_point);
        }
        cell.with_range_state(NativeState::default(), &stats, |table| {
            for current in table.cells() {
                assert_eq!(current.state.owner, RangeOwner::Owned(graphics_family));
                assert_eq!(
                    current.state.layout,
                    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                );
                assert_eq!(current.state.writer.points.get(graphics_queue), 8);
            }
            Ok(())
        })
        .unwrap();
        assert!(cell.validate_queue_range(graphics_family, range).is_ok());
    }

    #[test]
    fn failed_release_submit_leaves_no_acquire_pending() {
        let stats = Arc::new(crate::raw::allocator::AllocatorStats::default());
        let range = image_range(vk::ImageAspectFlags::COLOR, 1, 4);
        let cell = image_cell(range, &stats);
        let transfer_queue = QueueId::new(1).unwrap();
        let mut release_patches = Vec::new();
        let mut released = Vec::new();
        let mut scope = SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::empty(),
        );
        image_release_program(
            vk::Image::null(),
            &[WrittenPiece { range, revision: 1 }],
            2,
            0,
            &mut release_patches,
            &mut scope,
            &mut released,
        )
        .unwrap();
        let mut patches = vec![upload_access(range, transfer_queue, 2)];
        patches.extend(release_patches);
        {
            // A queue-submit failure drops the prepared state without publishing.
            let _gate = cell.lock_submission();
            cell.prepare_range_state(&patches, transfer_queue, 2, &stats)
                .unwrap();
            cell.cancel_range_state();
        }
        cell.with_range_state(NativeState::default(), &stats, |table| {
            assert_eq!(table.cells()[0].state.owner, RangeOwner::Unclaimed);
            Ok(())
        })
        .unwrap();
        // Graphics may use the image directly; a stale acquire is rejected.
        assert!(cell.validate_queue_range(0, range).is_ok());
        let acquire_patches = acquire_patches(
            &released,
            1,
            SyncPoint {
                queue: transfer_queue,
                value: 1,
            },
        );
        let _gate = cell.lock_submission();
        assert!(
            cell.prepare_range_state(&acquire_patches, QueueId::new(0).unwrap(), 0, &stats)
                .is_err()
        );
        cell.cancel_range_state();
    }

    #[test]
    fn concurrent_images_reject_ownership_patches() {
        let stats = Arc::new(crate::raw::allocator::AllocatorStats::default());
        let range = image_range(vk::ImageAspectFlags::COLOR, 1, 1);
        let cell = Arc::new(
            crate::resource::SyncCell::for_device(crate::device::DeviceGeneration::next())
                .with_concurrent_families(crate::resource::QueueFamilies::new(&[0, 2]).unwrap())
                .with_ownership_range(range),
        );
        cell.with_range_state(NativeState::default(), &stats, |_| Ok(()))
            .unwrap();
        let mut patches = Vec::new();
        let mut released = Vec::new();
        let mut scope = SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::empty(),
        );
        image_release_program(
            vk::Image::null(),
            &[WrittenPiece { range, revision: 1 }],
            2,
            0,
            &mut patches,
            &mut scope,
            &mut released,
        )
        .unwrap();
        let queue = QueueId::new(1).unwrap();
        let mut all = vec![upload_access(range, queue, 2)];
        all.extend(patches);
        let _gate = cell.lock_submission();
        assert!(cell.prepare_range_state(&all, queue, 2, &stats).is_err());
        cell.cancel_range_state();
    }
}
