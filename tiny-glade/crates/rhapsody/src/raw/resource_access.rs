//! Explicit raw access recording, using the same physical range state and
//! successful-submit publication as render graphs.
use super::{
    barrier::{BarrierDispatch, SynchronizationScope},
    buffer::Buffer,
    command_buffer::VulkanCommandBuffer,
    temp_list::TempList,
    texture::Texture,
};
use crate::{
    Error, Result,
    resource::{
        OwnershipRange, ResourceSharing, SyncCell,
        range_state::{AccessScope, NativeState, PatchAction, RangeCell, RangeOwner, StatePatch},
    },
    sync::QueueId,
};
use ash::vk;
use std::sync::Arc;

#[derive(Clone, Copy)]
struct AccessStep {
    cell: RangeCell,
    source: AccessScope,
}

#[derive(Clone, Copy)]
struct AccessRequest {
    queue: QueueId,
    range: OwnershipRange,
    scope: AccessScope,
    layout: vk::ImageLayout,
    expected_layout: Option<vk::ImageLayout>,
}

fn family(command: &VulkanCommandBuffer, queue: QueueId) -> Result<u32> {
    if !command.is_recording() {
        return Err(Error::invalid_usage(
            "native access requires a recording command",
        ));
    }
    let device = &command.pool.device;
    if device.is_device_lost() {
        return Err(Error::DeviceLost);
    }
    let family = if queue.index() == 0 {
        device.queue_family
    } else {
        device
            .additional_queues
            .get(queue.index() - 1)
            .ok_or_else(|| Error::invalid_usage("native access queue belongs to another device"))?
            .family
    };
    if family != command.pool.family {
        return Err(Error::invalid_usage(
            "native access uses another command-pool family",
        ));
    }
    Ok(family)
}

fn step(
    cell: RangeCell,
    sharing: ResourceSharing,
    queue: QueueId,
    family: u32,
    destination: AccessScope,
    layout: vk::ImageLayout,
    expected_layout: Option<vk::ImageLayout>,
) -> Result<AccessStep> {
    if !cell.state.layout_known {
        return Err(Error::unsupported(
            "native image layout is unknown after an opaque use",
        ));
    }
    if expected_layout.is_some_and(|expected| expected != cell.state.layout) {
        return Err(Error::invalid_usage(
            "native image old layout differs from recorded state",
        ));
    }
    let owns = match cell.state.owner {
        RangeOwner::Unclaimed => true,
        RangeOwner::Owned(owner) => owner == family,
        RangeOwner::Released { .. } => false,
    };
    if !sharing.allows_family(family) || !sharing.is_concurrent() && !owns {
        return Err(Error::unsupported(
            "native access requires a matching ownership acquire",
        ));
    }
    let mut source = cell.state.writer.for_queue(queue);
    if destination.write || cell.state.layout != layout {
        let readers = cell.state.readers.for_queue(queue);
        source.stages |= readers.stages;
        source.access |= readers.access;
        source.write |= readers.write;
    }
    Ok(AccessStep { cell, source })
}

fn steps(
    command: &VulkanCommandBuffer,
    cell: &Arc<SyncCell>,
    sharing: ResourceSharing,
    request: AccessRequest,
) -> Result<TempList<AccessStep>> {
    let AccessRequest {
        queue,
        range,
        scope,
        layout,
        expected_layout,
    } = request;
    let family = family(command, queue)?;
    if scope.stages.is_empty() {
        return Err(Error::invalid_usage("native access has no pipeline stages"));
    }
    cell.validate_device(command.pool.device.generation())?;
    crate::resource::range_state::validate_range(range)?;
    if command.has_opaque_overlap(cell, range) {
        return Err(Error::unsupported(
            "explicit native access follows an opaque use in the same command",
        ));
    }
    let mut result = TempList::new(65_536, command.pool.device.allocator_stats.clone())?;
    command.with_recorded_state(cell, NativeState::default(), queue, |table| {
        if !crate::resource::range_state::contains(table.domain(), range) {
            return Err(Error::invalid_usage(
                "native access exceeds the physical resource",
            ));
        }
        for cell in table.prepared_cells() {
            if let Some(overlap) = crate::resource::range_state::intersection(cell.range, range) {
                result.add(step(
                    RangeCell {
                        range: overlap,
                        ..cell
                    },
                    sharing,
                    queue,
                    family,
                    scope,
                    layout,
                    expected_layout,
                )?)?;
            }
        }
        Ok(())
    })?;
    Ok(result)
}

fn record_patches(
    command: &mut VulkanCommandBuffer,
    cell: &Arc<SyncCell>,
    queue: QueueId,
    scope: AccessScope,
    layout: vk::ImageLayout,
    steps: &[AccessStep],
) -> Result<()> {
    for step in steps {
        command.record_state_patch(
            cell.clone(),
            StatePatch {
                range: step.cell.range,
                expected_revision: step.cell.state.revision,
                action: PatchAction::Access {
                    scope,
                    layout,
                    queue,
                    family: command.pool.family,
                },
            },
        )?;
    }
    Ok(())
}

pub(crate) fn image_access_borrowed(
    command: &mut VulkanCommandBuffer,
    texture: &Texture,
    queue: QueueId,
    mut range: OwnershipRange,
    scope: AccessScope,
    layout: vk::ImageLayout,
    expected_layout: Option<vk::ImageLayout>,
) -> Result<()> {
    if matches!(
        layout,
        vk::ImageLayout::UNDEFINED | vk::ImageLayout::PREINITIALIZED
    ) {
        return Err(Error::invalid_usage(
            "native image target layout cannot be an initial-only layout",
        ));
    }
    if !Arc::ptr_eq(&texture.device, &command.pool.device)
        || !matches!(range, OwnershipRange::Image { .. })
    {
        return Err(Error::invalid_usage(
            "native image access has another device or range type",
        ));
    }
    // Separate depth/stencil layouts are currently disabled by the device
    // contract. Both aspects share one layout and must transition together.
    if let (OwnershipRange::Image { aspects, .. }, OwnershipRange::Image { aspects: all, .. }) =
        (&mut range, Texture::ownership_range(&texture.desc)?)
    {
        let both = (vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL).as_raw();
        if all & both == both && *aspects & both != 0 {
            *aspects |= both;
        }
    }
    let steps = steps(
        command,
        &texture.last_use,
        texture.sharing(),
        AccessRequest {
            queue,
            range,
            scope,
            layout,
            expected_layout,
        },
    )?;
    record_patches(
        command,
        &texture.last_use,
        queue,
        scope,
        layout,
        steps.as_slice(),
    )?;
    // The public wrapper owns this resource, or setup waits for actual completion.
    unsafe {
        command.retain_borrowed_recorded(texture)?;
    }
    let dispatch = BarrierDispatch::new(&command.pool.device);
    for step in steps.as_slice() {
        let OwnershipRange::Image {
            aspects,
            base_mip,
            mip_count,
            base_layer,
            layer_count,
        } = step.cell.range
        else {
            unreachable!("validated image domain")
        };
        let barrier = vk::ImageMemoryBarrier::default()
            .src_access_mask(step.source.access)
            .dst_access_mask(scope.access)
            .old_layout(step.cell.state.layout)
            .new_layout(layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(texture.handle)
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::from_raw(aspects))
                    .base_mip_level(base_mip)
                    .level_count(mip_count)
                    .base_array_layer(base_layer)
                    .layer_count(layer_count),
            );
        let mut synchronization = SynchronizationScope::new(step.source.stages, scope.stages);
        synchronization.images.push(barrier);
        unsafe {
            dispatch.issue(&command.pool.device, command.handle, &synchronization)?;
        }
    }
    Ok(())
}

/// Record the exact barriers and state admission for a subsequent image use.
/// Layouts come from the committed physical state plus earlier accesses in this
/// command. A concurrent host/raw change rejects submission through revisions.
///
/// # Safety
/// Subsequent native operations must match `range`, `scope`, and `layout` until
/// the next recorded access. Call outside rendering and submit on `queue`.
pub unsafe fn image_access(
    command: &mut VulkanCommandBuffer,
    texture: Arc<Texture>,
    queue: QueueId,
    range: OwnershipRange,
    scope: AccessScope,
    layout: vk::ImageLayout,
) -> Result<()> {
    unsafe {
        command.retain_recorded(texture.clone())?;
    }
    image_access_borrowed(command, &texture, queue, range, scope, layout, None)?;
    Ok(())
}

pub(crate) fn buffer_access_borrowed(
    command: &mut VulkanCommandBuffer,
    buffer: &Buffer,
    queue: QueueId,
    offset: u64,
    size: u64,
    scope: AccessScope,
) -> Result<()> {
    if !Arc::ptr_eq(&buffer.device, &command.pool.device) {
        return Err(Error::invalid_usage(
            "native buffer access belongs to another device",
        ));
    }
    let steps = steps(
        command,
        &buffer.last_use,
        buffer.sharing(),
        AccessRequest {
            queue,
            range: OwnershipRange::Buffer { offset, size },
            scope,
            layout: vk::ImageLayout::UNDEFINED,
            expected_layout: None,
        },
    )?;
    record_patches(
        command,
        &buffer.last_use,
        queue,
        scope,
        vk::ImageLayout::UNDEFINED,
        steps.as_slice(),
    )?;
    // The public wrapper owns this resource, or setup waits for actual completion.
    unsafe {
        command.retain_borrowed_recorded(buffer)?;
    }
    let dispatch = BarrierDispatch::new(&command.pool.device);
    for step in steps.as_slice() {
        let OwnershipRange::Buffer { offset, size } = step.cell.range else {
            unreachable!("validated buffer domain")
        };
        let barrier = vk::BufferMemoryBarrier::default()
            .src_access_mask(step.source.access)
            .dst_access_mask(scope.access)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .buffer(buffer.handle)
            .offset(offset)
            .size(size);
        let mut synchronization = SynchronizationScope::new(step.source.stages, scope.stages);
        synchronization.buffers.push(barrier);
        unsafe {
            dispatch.issue(&command.pool.device, command.handle, &synchronization)?;
        }
    }
    Ok(())
}

/// # Safety
/// Subsequent native operations must access only these bytes with this scope
/// until the next recorded access. Call outside rendering; submit on `queue`.
pub unsafe fn buffer_access(
    command: &mut VulkanCommandBuffer,
    buffer: Arc<Buffer>,
    queue: QueueId,
    offset: u64,
    size: u64,
    scope: AccessScope,
) -> Result<()> {
    unsafe {
        command.retain_recorded(buffer.clone())?;
    }
    buffer_access_borrowed(command, &buffer, queue, offset, size, scope)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::range_state::{Frontier, RangeStateConfig, RangeStateTable};

    fn image(state: NativeState) -> RangeCell {
        RangeCell {
            range: OwnershipRange::Image {
                aspects: 1,
                base_mip: 0,
                mip_count: 1,
                base_layer: 0,
                layer_count: 1,
            },
            state,
        }
    }
    fn scope(stage: vk::PipelineStageFlags, access: vk::AccessFlags, write: bool) -> AccessScope {
        AccessScope {
            stages: stage,
            access,
            write,
        }
    }
    #[test]
    fn write_barrier_uses_local_readers_writer_and_host_without_foreign_queue_stages() {
        let queue = QueueId::new(0).unwrap();
        let mut state = NativeState {
            layout: vk::ImageLayout::GENERAL,
            owner: RangeOwner::Owned(0),
            ..NativeState::default()
        };
        state.writer.scopes[0] = scope(
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::AccessFlags::SHADER_WRITE,
            true,
        );
        state.writer.host = scope(
            vk::PipelineStageFlags::HOST,
            vk::AccessFlags::HOST_WRITE,
            true,
        );
        state.readers.scopes[0] = scope(
            vk::PipelineStageFlags::VERTEX_SHADER | vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::AccessFlags::SHADER_READ,
            false,
        );
        state.readers.scopes[1] = scope(
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_READ,
            false,
        );
        let planned = step(
            image(state),
            ResourceSharing::Exclusive,
            queue,
            0,
            scope(
                vk::PipelineStageFlags::TRANSFER,
                vk::AccessFlags::TRANSFER_WRITE,
                true,
            ),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            None,
        )
        .unwrap();
        assert_eq!(
            planned.source.stages,
            vk::PipelineStageFlags::HOST
                | vk::PipelineStageFlags::COMPUTE_SHADER
                | vk::PipelineStageFlags::VERTEX_SHADER
                | vk::PipelineStageFlags::FRAGMENT_SHADER
        );
        assert_eq!(
            planned.source.access,
            vk::AccessFlags::HOST_WRITE
                | vk::AccessFlags::SHADER_WRITE
                | vk::AccessFlags::SHADER_READ
        );
        assert!(
            !planned
                .source
                .stages
                .contains(vk::PipelineStageFlags::TRANSFER)
        );
    }
    #[test]
    fn unknown_layout_old_scalar_mismatch_and_unacquired_owner_are_rejected() {
        let queue = QueueId::new(0).unwrap();
        let destination = scope(
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
            true,
        );
        let mut state = NativeState {
            layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            ..NativeState::default()
        };
        assert!(matches!(
            step(
                image(state),
                ResourceSharing::Exclusive,
                queue,
                0,
                destination,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                Some(vk::ImageLayout::UNDEFINED)
            ),
            Err(Error::InvalidUsage { .. })
        ));
        state.layout_known = false;
        assert!(matches!(
            step(
                image(state),
                ResourceSharing::Exclusive,
                queue,
                0,
                destination,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                None
            ),
            Err(Error::Unsupported(_))
        ));
        state.layout_known = true;
        state.owner = RangeOwner::Owned(1);
        assert!(matches!(
            step(
                image(state),
                ResourceSharing::Exclusive,
                queue,
                0,
                destination,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                None
            ),
            Err(Error::Unsupported(_))
        ));
    }
    #[test]
    fn staging_initial_and_second_barriers_use_real_prepared_layout_and_revision() {
        let queue = QueueId::new(0).unwrap();
        let destination = scope(
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
            true,
        );
        let initial = image(NativeState::default());
        let first = step(
            initial,
            ResourceSharing::Exclusive,
            queue,
            0,
            destination,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            Some(vk::ImageLayout::UNDEFINED),
        )
        .unwrap();
        assert!(first.source.stages.is_empty());
        assert!(first.source.access.is_empty());
        let budget = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut table = RangeStateTable::new(
            initial.range,
            ResourceSharing::Exclusive,
            initial.state,
            RangeStateConfig::default(),
            budget,
        )
        .unwrap();
        table
            .prepare(
                &[StatePatch {
                    range: first.cell.range,
                    expected_revision: 0,
                    action: PatchAction::Access {
                        scope: destination,
                        layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        queue,
                        family: 0,
                    },
                }],
                queue,
                0,
            )
            .unwrap();
        let prepared = table.prepared_cells().next().unwrap();
        assert_eq!(prepared.state.revision, 1);
        let last = step(
            prepared,
            ResourceSharing::Exclusive,
            queue,
            0,
            scope(
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::AccessFlags::SHADER_READ,
                false,
            ),
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            Some(vk::ImageLayout::TRANSFER_DST_OPTIMAL),
        )
        .unwrap();
        assert_eq!(last.source.access, vk::AccessFlags::TRANSFER_WRITE);
        assert_eq!(last.source.stages, vk::PipelineStageFlags::TRANSFER);
        assert_eq!(
            last.cell.state.layout,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL
        );
        // Preview and cancellation cannot turn the not-yet-submitted image into
        // an externally visible shader-read image.
        table.cancel_prepared();
        assert_eq!(table.cells()[0].state.layout, vk::ImageLayout::UNDEFINED);
        assert_eq!(table.cells()[0].state.writer, Frontier::EMPTY);
    }
}
