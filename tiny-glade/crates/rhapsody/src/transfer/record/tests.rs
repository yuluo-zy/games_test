use super::*;
use crate::tracking::range_state::RangeOwner;

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
    crate::tracking::range_state::validate_transfer(first).unwrap();
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
    stats: &Arc<crate::accounting::AllocatorStats>,
) -> Arc<crate::tracking::SyncCell> {
    let cell = Arc::new(
        crate::tracking::SyncCell::for_device(crate::sync::DeviceGeneration::next())
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
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
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
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
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
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let range = image_range(vk::ImageAspectFlags::COLOR, 1, 1);
    let cell = Arc::new(
        crate::tracking::SyncCell::for_device(crate::sync::DeviceGeneration::next())
            .with_concurrent_families(crate::tracking::QueueFamilies::new(&[0, 2]).unwrap())
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
