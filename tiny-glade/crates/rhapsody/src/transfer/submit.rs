//! Recording and submission of uploader batches, including acquire bookkeeping.
use ash::vk;
use crate::device::Device;
use crate::resource::buffer::Buffer;
use crate::sync::SyncPoint;
use crate::sync::SyncSet;
use std::sync::Arc;
use super::record;
use super::record::QueueTarget;
use super::record::UploadRoute;
use super::record::WrittenPiece;
use super::route::MAX_COPY_REGIONS;
use super::route::MAX_UPLOAD_TARGETS;
use super::route::Uploader;
use super::route::defer_on_exhaustion;
use super::route::route;
use super::route::texture_publish;
use super::texture;
use super::texture::TextureUpload;

impl Uploader<'_> {
    /// One staging flush and submission for a bounded set of distinct targets.
    /// Each target retains its own range-state/barrier and ownership program.
    pub(super) fn submit<'b>(
        &mut self,
        mut segments: impl Iterator<Item = (&'b Arc<Buffer>, u64, &'b [u8])> + Clone,
        #[cfg(feature = "cpu-probe")] submitted_payload: &mut Option<u64>,
    ) -> crate::Result<SyncPoint> {
        self.device.try_collect_garbage()?;
        let completed = self.device.poll()?;
        let Some(index) = self.staging.free_upload(&completed) else {
            *self.stalls += 1;
            return Err(crate::Error::Timeout);
        };
        let buffer = self
            .staging
            .upload_buffer(index)
            .map_err(defer_on_exhaustion)?;
        let mut copies = [vk::BufferCopy::default(); MAX_COPY_REGIONS];
        let mut ranges = [(0u64, 0u64); MAX_COPY_REGIONS];
        let mut groups = smallvec::SmallVec::<
            [(
                &Arc<Buffer>,
                std::ops::Range<usize>,
                UploadRoute,
                Vec<crate::tracking::range_state::RangeTransfer>,
            ); MAX_UPLOAD_TARGETS],
        >::new();
        let mut count = 0;
        let mut packed = 0u64;
        for (destination, offset, bytes) in segments.clone() {
            if count == copies.len()
                || packed
                    .checked_add(bytes.len() as u64)
                    .is_none_or(|end| end > buffer.size)
            {
                return Err(crate::Error::invalid_usage(
                    "upload segments exceed admitted staging capacity",
                ));
            }
            copies[count] = vk::BufferCopy {
                src_offset: packed,
                dst_offset: offset,
                size: bytes.len() as u64,
            };
            ranges[count] = (offset, bytes.len() as u64);
            if let Some(group) = groups
                .last_mut()
                .filter(|group| Arc::ptr_eq(group.0, destination))
            {
                group.1.end = count + 1;
            } else {
                // Queue admission groups segments by distinct target; immediate
                // uploads use the same kernel with exactly one target.
                debug_assert!(groups.len() < MAX_UPLOAD_TARGETS);
                groups.push((
                    destination,
                    count..count + 1,
                    UploadRoute::Graphics,
                    Vec::new(),
                ));
            }
            count += 1;
            packed += bytes.len() as u64;
        }
        if count == 0 {
            return Err(crate::Error::invalid_usage("upload has no segments"));
        }
        if count == 1 {
            let (_, _, bytes) = segments.next().expect("one segment");
            buffer.write_bytes_unsynchronized(0, bytes)?;
        } else {
            buffer.write_with_unsynchronized(0, packed, |dst| {
                let mut at = 0;
                for (_, _, bytes) in segments {
                    dst[at..at + bytes.len()].copy_from_slice(bytes);
                    at += bytes.len();
                }
            })?;
        }
        for group in &mut groups {
            group.2 = route(
                self.graphics,
                self.transfer,
                group.0,
                &copies[group.1.clone()],
            );
        }
        let graphics = groups.iter().any(|group| group.2 == UploadRoute::Graphics);
        let target = if graphics {
            self.graphics
        } else {
            self.transfer.expect("transfer routes")
        };
        let mut command = target.pool.allocate()?;
        // SAFETY: the copy reads exactly the packed span, retained through submit.
        unsafe {
            command.retain_opaque_buffer_range(&buffer, 0, packed)?;
        }
        let recorded = RecordedAcquires::begin(self.device, &mut command)?;
        let mut pieces = Vec::<WrittenPiece>::new();
        for group in &mut groups {
            let destination = group.0;
            let spans = &ranges[group.1.clone()];
            record::record_buffer_writes(&mut command, destination, target, spans, &mut pieces)?;
            // SAFETY: recording; each target and its source span are retained.
            unsafe {
                self.device.core.device.cmd_copy_buffer(
                    command.handle,
                    buffer.handle,
                    destination.handle,
                    &copies[group.1.clone()],
                );
            }
            if graphics {
                record::record_buffer_visibility(&command, destination.handle, spans)?;
            } else if group.2 == UploadRoute::OwnershipTransfer {
                record::record_buffer_release(
                    &mut command,
                    destination,
                    &pieces,
                    self.graphics.family(),
                    &mut group.3,
                )?;
            }
        }
        command.end()?;
        let mut acquire_reservation = self.device
            .reserve_pending_acquires(groups.iter().map(|group| group.3.len()).sum())?;
        let submitted = command.submit_to(target.queue, &[], &[])?;
        #[cfg(feature = "cpu-probe")]
        { *submitted_payload = Some(packed); }
        recorded.submitted();
        self.staging.uploads[index].last_use = SyncSet::from_point(submitted.point);
        for group in groups {
            record::queue_acquires(
                self.device,
                &mut acquire_reservation,
                &crate::command::AcquireTarget::buffer(group.0),
                &group.3,
                submitted.point,
            )?;
        }
        Ok(submitted.point)
    }
}

pub(super) fn record_graphics_textures(
    device: &Device,
    target: &QueueTarget,
    uploads: &[TextureUpload<'_>],
    plans: &[texture::TexturePlan],
    bases: &[u64],
    staging: &Arc<Buffer>,
    total: u64,
) -> crate::Result<SyncPoint> {
    let mut command = target.pool.allocate()?;
    // SAFETY: the copies read only this staging span; each texture's
    // accesses are recorded from its tracked state below.
    unsafe {
        command.retain_opaque_buffer_range(staging, 0, total)?;
        for upload in uploads {
            command.retain_recorded(upload.texture.clone())?;
        }
    }
    let recorded = RecordedAcquires::begin(device, &mut command)?;
    for ((upload, plan), base) in uploads.iter().zip(plans).zip(bases) {
        texture::record_texture_copies(
            &mut command,
            upload.texture,
            target.id,
            staging.handle,
            *base,
            plan,
            upload.old_layout,
        )?;
    }
    command.end()?;
    let point = command.submit_to(target.queue, &[], &[])?.point;
    recorded.submitted();
    Ok(point)
}

/// Record and submit a texture batch on the transfer queue and queue the
/// acquires of every released texture on the device. Returns the transfer
/// submit point. A failed submit publishes no release.
#[allow(clippy::too_many_arguments)]
pub(super) fn record_transfer_textures(
    device: &Device,
    graphics: &QueueTarget,
    transfer: &QueueTarget,
    uploads: &[TextureUpload<'_>],
    plans: &[texture::TexturePlan],
    bases: &[u64],
    staging: &Arc<Buffer>,
    total: u64,
    submitted: &mut Option<SyncPoint>,
) -> crate::Result<SyncPoint> {
    let mut command = transfer.pool.allocate()?;
    let mut released: Vec<Vec<crate::tracking::range_state::RangeTransfer>> = Vec::new();
    released
        .try_reserve_exact(uploads.len())
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    // SAFETY: the copies read only this staging span.
    unsafe {
        command.retain_opaque_buffer_range(staging, 0, total)?;
    }
    let recorded = RecordedAcquires::begin(device, &mut command)?;
    for ((upload, plan), base) in uploads.iter().zip(plans).zip(bases) {
        let publish = texture_publish(
            graphics.family(),
            transfer.family(),
            upload.texture.sharing().is_concurrent(),
        );
        let mut pieces = Vec::new();
        texture::record_texture_copies_on_transfer(
            &mut command,
            upload.texture,
            transfer.id,
            staging.handle,
            *base,
            plan,
            upload.old_layout,
            publish,
            &mut pieces,
        )?;
        released.push(pieces);
    }
    command.end()?;
    let mut acquire_reservation = device.reserve_pending_acquires(released.iter().map(Vec::len).sum())?;
    let point = command.submit_to(transfer.queue, &[], &[])?.point;
    *submitted = Some(point);
    recorded.submitted();
    for (upload, pieces) in uploads.iter().zip(&released) {
        record::queue_acquires(
            device,
            &mut acquire_reservation,
            &crate::command::AcquireTarget::image(upload.texture),
            pieces,
            point,
        )?;
    }
    Ok(point)
}

/// Pending acquires recorded at the start of one of this manager's commands.
/// They go back to the device queue unless that command is submitted.
pub(super) struct RecordedAcquires<'a> {
    device: &'a Device,
    drained: crate::command::DrainedAcquires,
}

impl<'a> RecordedAcquires<'a> {
    /// Begin `command` and record every acquire pending for its family.
    pub(super) fn begin(
        device: &'a Device,
        command: &mut crate::command::command_buffer::VulkanCommandBuffer,
    ) -> crate::Result<Self> {
        command.begin()?;
        Ok(Self {
            device,
            drained: device.record_pending_acquires(command)?,
        })
    }
    pub(super) fn is_empty(&self) -> bool {
        self.drained.is_empty()
    }
    /// The command was submitted: its acquires are no longer pending.
    pub(super) fn submitted(mut self) {
        self.drained.submitted();
    }
}

impl Drop for RecordedAcquires<'_> {
    fn drop(&mut self) {
        if !self.drained.is_empty() {
            self.device
                .requeue_pending_acquires(std::mem::take(&mut self.drained));
        }
    }
}
