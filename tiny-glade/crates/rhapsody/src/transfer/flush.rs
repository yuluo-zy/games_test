//! Flushing queued uploads and reading back buffers.
use ash::vk;
use super::staging;
use crate::native::AccessScope;
use crate::resource::buffer::Buffer;
use crate::sync::SyncPoint;
use std::sync::Arc;
use super::TransferManager;
use super::route::MAX_COPY_REGIONS;
use super::route::Uploader;
use super::route::defer_on_exhaustion;
use super::route::texture_route;
use super::submit::RecordedAcquires;
use super::submit::record_graphics_textures;
use super::submit::record_transfer_textures;
use super::texture;
use super::texture::TextureUpload;
use super::tickets::ReadbackTicket;
use super::tickets::UploadFlush;

impl TransferManager {
    /// Pump bounded accepted writes without waiting for GPU work. Staging
    /// pressure defers jobs; OOM preserves their payload and FIFO position for
    /// an explicit later retry. Device loss and other errors terminate tickets.
    pub fn flush_uploads(&mut self, max_submits: usize) -> crate::Result<UploadFlush> {
        let mut result = UploadFlush::default();
        if self.device.memory.stats.budget_action() != crate::accounting::BudgetAction::None {
            self.queued.trim_idle();
        }
        if max_submits != 0 && self.queued.queued_jobs() != 0 {
            // Poll first: device-level pressure processing can evict idle native
            // allocator blocks before deciding to defer low-priority payloads.
            if let Err(error) = self.device.poll() {
                if error.kind() == crate::ErrorKind::OutOfMemory {
                    return Err(error);
                }
                return Err(self.queued.fail_all(error).to_error());
            }
        }
        for _ in 0..max_submits {
            let batch = self.queued.next_batch(
                self.device.memory.stats.budget_action(),
                self.config.chunk_bytes as usize,
                MAX_COPY_REGIONS,
            );
            if batch.is_empty() {
                result.capacity_deferred = self.queued.queued_jobs() != 0;
                break;
            }
            let bytes: usize = batch
                .iter()
                .map(|&index| self.queued.job(index).bytes())
                .sum();
            let mut staging = self.staging.lock();
            #[cfg(feature = "cpu-probe")]
            let mut submitted_payload = None;
            let mut uploader = Uploader {
                device: &self.device,
                graphics: &self.graphics,
                transfer: self.transfer.as_ref(),
                staging: &mut staging,
                stalls: &mut self.capacity_stalls,
            };
            let submission = uploader.submit(batch.iter().flat_map(|&index| {
                let job = self.queued.job(index);
                job.segments
                    .iter()
                    .map(move |segment| (&job.target, segment.offset, segment.bytes()))
            }),
                #[cfg(feature = "cpu-probe")]
                &mut submitted_payload,
            );
            #[cfg(feature = "cpu-probe")]
            self.queued.record_native_submission(batch.len(), submitted_payload);
            match submission {
                Ok(point) => {
                    for &index in batch.iter().rev() {
                        self.queued.publish(index, point);
                    }
                    result.submitted_jobs += batch.len();
                    result.submitted_bytes += bytes;
                }
                Err(crate::Error::Timeout) => {
                    self.queued.defer();
                    result.capacity_deferred = true;
                    break;
                }
                Err(error) if error.kind() == crate::ErrorKind::OutOfMemory => {
                    // Ownership never returns to the producer on an accepted
                    // write. Keep both the original bytes and physical target;
                    // later writes to this target cannot overtake this job.
                    return Err(error);
                }
                Err(error) => {
                    let failure = if error.is_device_lost() {
                        self.queued.fail_all(error)
                    } else {
                        self.queued.fail_batch(&batch, error)
                    };
                    return Err(failure.to_error());
                }
            }
        }
        result.remaining_jobs = self.queued.queued_jobs();
        Ok(result)
    }

    /// Upload several textures with one submit. Their data is packed with one
    /// host write into a free staging slot when it fits a chunk, otherwise
    /// into a temporary staging buffer retired with the command. Each texture
    /// transitions from `old_layout` to TRANSFER_DST_OPTIMAL and ends in
    /// SHADER_READ_ONLY_OPTIMAL for shader reads, with barriers derived from
    /// its tracked state.
    ///
    /// The batch runs on the planned transfer queue when it is a distinct
    /// queue that can write every texture now (see `texture_route`): a queue
    /// of the graphics family or a concurrent image orders consumers through
    /// the timeline semaphore only; an exclusive image of another family is
    /// released there (including the final layout transition) and its acquire
    /// is queued on the device (see `UploadTicket`). Otherwise it runs on
    /// the graphics queue. Never waits; the returned point is the batch's
    /// submit point (the release point when ownership moved).
    pub fn upload_textures(&mut self, uploads: &[TextureUpload<'_>]) -> crate::Result<SyncPoint> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if uploads.is_empty()
            || uploads
                .iter()
                .any(|upload| !Arc::ptr_eq(&upload.texture.device, &self.device))
        {
            return Err(crate::Error::invalid_usage(
                "texture uploads must be nonempty and belong to this device",
            ));
        }
        let mut plans = Vec::new();
        plans
            .try_reserve_exact(uploads.len())
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        for upload in uploads {
            plans.push(texture::plan_texture_upload(
                upload.texture,
                upload.regions,
                upload.old_layout,
            )?);
        }
        let (bases, total) = texture::layout(plans.iter())?;
        let pairs: Vec<_> = plans
            .iter()
            .zip(uploads)
            .map(|(plan, upload)| (plan, upload.regions))
            .collect();
        self.device.try_collect_garbage()?;
        let completed = self.device.poll()?;
        let staging_slots = self.staging.clone();
        let mut slots = staging_slots.lock();
        let mut slot = if total <= self.config.chunk_bytes {
            slots.free_upload(&completed)
        } else {
            None
        };
        // An evicted slot that cannot be recreated now falls back to a
        // temporary staging buffer.
        let resident = match slot {
            Some(index) => match slots.upload_buffer(index) {
                Ok(buffer) => Some(buffer),
                Err(error) if crate::device::allocator::is_memory_exhaustion(&error) => {
                    slot = None;
                    None
                }
                Err(error) => return Err(error),
            },
            None => None,
        };
        if let Some(index) = slot {
            slots.uploads[index].leased = true;
        }
        let mut lease = staging::UploadLease {
            staging: staging_slots.clone(),
            index: slot,
            submitted: None,
        };
        drop(slots);
        let staging = match resident {
            Some(buffer) => {
                // Pack each region directly into the mapped staging buffer.
                buffer.write_with_unsynchronized(0, total, |dst| {
                    for ((plan, regions), base) in pairs.iter().zip(&bases) {
                        texture::pack_regions(dst, *base, plan, regions);
                    }
                })?;
                buffer
            }
            None => {
                let mut bytes = Vec::new();
                texture::pack_all(&pairs, &bases, total, &mut bytes)?;
                texture::new_staging(&self.device, &bytes)?
            }
        };
        let transfer = self
            .transfer
            .as_ref()
            .filter(|transfer| texture_route(transfer.family(), self.transfer_images, uploads));
        let result = match transfer {
            None => record_graphics_textures(
                &self.device,
                &self.graphics,
                uploads,
                &plans,
                &bases,
                &staging,
                total,
            ),
            Some(transfer) => record_transfer_textures(
                &self.device,
                &self.graphics,
                transfer,
                uploads,
                &plans,
                &bases,
                &staging,
                total,
                &mut lease.submitted,
            ),
        };
        if let Ok(point) = &result {
            lease.submitted = Some(*point);
        }
        // This lease is fresh (submitted: None) for this invocation. The
        // transfer helper sets its proof immediately after the copy submit,
        // before fallible acquire registration; graphics sets it on Ok above.
        #[cfg(feature = "cpu-probe")]
        if lease.submitted.is_some() {
            let payload = uploads.iter().try_fold(0u64, |sum, upload| {
                upload.regions.iter().try_fold(sum, |sum, region| {
                    u64::try_from(region.data.len()).ok().and_then(|bytes| sum.checked_add(bytes))
                })
            });
            self.device.record_texture_upload_submission(crate::device::TextureUploadProbeKind::TransferTextures, payload);
        }
        Ok(result?)
    }

    /// Record every pending graphics-family acquire into one command and
    /// submit it (at most one submit; none when nothing is pending). For
    /// callers whose consumers are not a render graph, e.g. raw command
    /// buffers. On failure the acquires stay pending.
    pub fn flush_acquires(&mut self) -> crate::Result<Option<SyncPoint>> {
        if self.pending_acquire_count() == 0 {
            return Ok(None);
        }
        let mut command = self.graphics.pool.allocate()?;
        let recorded = RecordedAcquires::begin(&self.device, &mut command)?;
        if recorded.is_empty() {
            command.end()?;
            return Ok(None);
        }
        command.end()?;
        let point = command.submit_to(self.graphics.queue, &[], &[])?.point;
        recorded.submitted();
        Ok(Some(point))
    }

    pub fn read_buffer(
        &mut self,
        source: &Arc<Buffer>,
        offset: u64,
        bytes: usize,
    ) -> crate::Result<ReadbackTicket> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        self.validate_buffer(
            source,
            offset,
            bytes as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
        )?;
        self.device.try_collect_garbage()?;
        let completed = self.device.poll()?;
        let staging = self.staging.clone();
        let mut slots = staging.lock();
        let Some(index) = slots.free_readback(&completed) else {
            self.capacity_stalls += 1;
            return Err(crate::Error::Timeout);
        };
        slots.ensure_readback(index).map_err(defer_on_exhaustion)?;
        let readback = slots.readback_buffer(index);
        let size = bytes as u64;
        let target = &self.graphics;
        let mut command = target.pool.allocate()?;
        // SAFETY: the copy writes exactly these slot bytes; the source access is
        // recorded below from its tracked state.
        unsafe {
            command.retain_opaque_buffer_range(&readback, 0, size)?;
            command.retain_recorded(source.clone())?;
        }
        let recorded = RecordedAcquires::begin(&self.device, &mut command)?;
        crate::command::resource_access::buffer_access_borrowed(
            &mut command,
            source,
            target.id,
            offset,
            size,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ,
                write: false,
            },
        )?;
        // SAFETY: recording; both buffers are retained by the command.
        unsafe {
            self.device.core.device.cmd_copy_buffer(
                command.handle,
                source.handle,
                readback.handle,
                &[vk::BufferCopy {
                    src_offset: offset,
                    dst_offset: 0,
                    size,
                }],
            );
        }
        let mut host = crate::command::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
        );
        host.push(
            crate::command::barrier::BarrierTarget::Buffer(readback.handle),
            &crate::command::barrier::RangeBarrier::local(
                crate::tracking::OwnershipRange::Buffer { offset: 0, size },
                vk::AccessFlags::TRANSFER_WRITE,
                vk::AccessFlags::HOST_READ,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::UNDEFINED,
            ),
        )?;
        host.issue_into(&command)?;
        command.end()?;
        let submitted = command.submit_to(target.queue, &[], &[])?;
        recorded.submitted();
        // Publication happens only after a successful submit; a failed submit
        // leaves the private lease untouched.
        Ok(ReadbackTicket {
            lease: slots.publish_readback(index, submitted.point, bytes),
        })
    }
}
