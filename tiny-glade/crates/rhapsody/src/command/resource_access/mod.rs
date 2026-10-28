//! Explicit raw access recording, using the same physical range state and
//! successful-submit publication as render graphs.
use crate::command::barrier::{BarrierTarget, RangeBarrier, SynchronizationScope};
use crate::resource::buffer::Buffer;
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::resource::texture::Texture;
use crate::{Error, Result, sync::QueueId};
use crate::tracking::{OwnershipRange, ResourceSharing, SyncCell};
use crate::tracking::range_state::{NativeState, OwnershipDenial, PatchAction, RangeCell, StatePatch, released_range_error};
use crate::native::AccessScope;
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
        device.queues.primary_family
    } else {
        device
            .queues
            .additional
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
    let discarded = cell.state.contents_discarded(cell.range);
    match cell
        .state
        .owner
        .admits_discarding(sharing, family, discarded)
    {
        Ok(()) => {}
        Err(OwnershipDenial::Released) => return Err(released_range_error()),
        Err(_) => {
            return Err(Error::unsupported(
                "native access requires a matching ownership acquire",
            ));
        }
    }
    let mut source = cell.state.writer.for_queue(queue);
    let hazard = destination.write || cell.state.layout != layout;
    if hazard {
        let readers = cell.state.readers.for_queue(queue);
        source.stages |= readers.stages;
        source.access |= readers.access;
        source.write |= readers.write;
    }
    // Submission waits other queues' prerequisites at exactly the destination
    // stages; including them here chains the barrier (and any layout
    // transition) after that semaphore wait.
    let foreign = |frontier: &crate::tracking::range_state::Frontier| {
        frontier.points.points().any(|point| point.queue != queue)
    };
    if foreign(&cell.state.writer) || hazard && foreign(&cell.state.readers) {
        source.stages |= destination.stages;
    }
    Ok(AccessStep { cell, source })
}

fn steps(
    command: &VulkanCommandBuffer,
    cell: &Arc<SyncCell>,
    sharing: ResourceSharing,
    request: AccessRequest,
) -> Result<smallvec::SmallVec<[AccessStep; 4]>> {
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
    crate::tracking::range_state::validate_range(range)?;
    if command.has_opaque_overlap(cell, range) {
        return Err(Error::unsupported(
            "explicit native access follows an opaque use in the same command",
        ));
    }
    // Almost every access touches one to a few cells: keep them inline instead
    // of allocating a budgeted list per access. The old 65,536-step bound and
    // OOM reporting are preserved explicitly.
    const MAX_STEPS: usize = 65_536;
    let mut result: smallvec::SmallVec<[AccessStep; 4]> = smallvec::SmallVec::new();
    command.with_recorded_state(cell, NativeState::default(), queue, |table| {
        if !crate::tracking::range_state::contains(table.domain(), range) {
            return Err(Error::invalid_usage(
                "native access exceeds the physical resource",
            ));
        }
        for cell in table.prepared_cells() {
            if let Some(overlap) = crate::tracking::range_state::intersection(cell.range, range) {
                if result.len() >= MAX_STEPS {
                    return Err(Error::Timeout);
                }
                result.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
                result.push(step(
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
                )?);
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
            cell,
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
    emit_barriers(
        command,
        BarrierTarget::Image(texture.handle),
        steps.as_slice(),
        scope,
        layout,
    )
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
    texture: &Arc<Texture>,
    queue: QueueId,
    range: OwnershipRange,
    scope: AccessScope,
    layout: vk::ImageLayout,
) -> Result<()> {
    unsafe {
        command.retain_recorded(texture.clone())?;
    }
    image_access_borrowed(command, texture, queue, range, scope, layout, None)?;
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
    emit_barriers(
        command,
        BarrierTarget::Buffer(buffer.handle),
        steps.as_slice(),
        scope,
        vk::ImageLayout::UNDEFINED,
    )
}

/// Issue the barriers of `steps`: one batch per distinct source stage mask,
/// so every barrier keeps exactly its own execution scope.
fn emit_barriers(
    command: &mut VulkanCommandBuffer,
    target: BarrierTarget,
    steps: &[AccessStep],
    destination: AccessScope,
    layout: vk::ImageLayout,
) -> Result<()> {
    let result = (|| {
    let mut done = 0;
    let mut issued = smallvec::SmallVec::<[vk::PipelineStageFlags; 4]>::new();
    while done < steps.len() {
        let Some(stages) = steps
            .iter()
            .map(|step| step.source.stages)
            .find(|stages| !issued.contains(stages))
        else {
            break;
        };
        issued.push(stages);
        let mut batch = SynchronizationScope::new(stages, destination.stages);
        for step in steps.iter().filter(|step| step.source.stages == stages) {
            batch.push(
                target,
                &RangeBarrier::local(
                    step.cell.range,
                    step.source.access,
                    destination.access,
                    step.cell.state.layout,
                    layout,
                ),
            )?;
            done += 1;
        }
        batch.issue_into(command)?;
    }
    Ok(())
    })();
    command.remember_metadata_error(&result);
    result
}

/// # Safety
/// Subsequent native operations must access only these bytes with this scope
/// until the next recorded access. Call outside rendering; submit on `queue`.
pub unsafe fn buffer_access(
    command: &mut VulkanCommandBuffer,
    buffer: &Arc<Buffer>,
    queue: QueueId,
    offset: u64,
    size: u64,
    scope: AccessScope,
) -> Result<()> {
    unsafe {
        command.retain_recorded(buffer.clone())?;
    }
    buffer_access_borrowed(command, buffer, queue, offset, size, scope)?;
    Ok(())
}

#[cfg(test)]
mod tests;
