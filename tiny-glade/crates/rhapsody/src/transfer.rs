//! Bounded asynchronous buffer and texture transfers.
//!
//! Uploads run on the device's planned transfer queue when it is a distinct
//! queue and the destination's ownership allows it; otherwise on the graphics
//! queue. A transfer queue of another family releases the written ranges and
//! registers their acquires on the device (`Device::queue_pending_acquire`):
//! the next render-graph batch, graphics-queue transfer command or
//! [`TransferManager::flush_acquires`] on the graphics family records them all
//! in one barrier batch, so N uploads cost no extra submit when a graph frame
//! follows. Until then the tracked range state stays released, and any use
//! that has not recorded the acquire fails validation instead of racing.
//! Barriers come from the tracked resource state as one batch per command.
//!
//! Each slot owns a separate persistently mapped allocation. Host writes cannot overlap a
//! previous GPU use, and a readback slot cannot be recycled while its ticket
//! still exposes the result. The CPU queue coalesces dirty intervals by physical
//! buffer/key/epoch, preserves write order, and defers work when staging is full.
//! Queue payloads, task envelopes and epoch history have explicit local bounds;
//! their known host bytes participate in the device budget. No worker thread is
//! required: the production frame/service owner pumps `flush_uploads`.
//! Immediate requests return `Timeout` without waiting; the sparse synchronous
//! convenience waits only for its own successful GPU submission points.
use crate::{
    device::Device,
    raw::buffer::Buffer,
    resource::range_state::AccessScope,
    sync::{SyncPoint, SyncSet},
};
mod queue;
mod record;
mod staging;
pub(crate) mod texture;
use ash::vk;
use queue::UploadQueue;
pub use queue::{
    UploadCancellation, UploadFailure, UploadKey, UploadOptions, UploadPriority, UploadQueueConfig,
    UploadQueueStats, UploadStatus,
};
use record::{QueueTarget, UploadRoute, WrittenPiece};
use staging::{ReadbackLease, Staging};
use std::sync::Arc;
pub use texture::TextureUpload;
use texture::TransferPublish;

#[derive(Clone, Copy, Debug)]
pub struct TransferConfig {
    pub slots: u32,
    pub chunk_bytes: u64,
    /// Includes both upload and readback backing allocations, using their real
    /// memory requirement sizes rather than the requested payload size.
    pub max_bytes: u64,
}
impl Default for TransferConfig {
    fn default() -> Self {
        Self {
            slots: 4,
            chunk_bytes: 4 * 1024 * 1024,
            max_bytes: 32 * 1024 * 1024,
        }
    }
}
impl TransferConfig {
    fn validate(self) -> crate::Result<()> {
        if self.slots == 0
            || self.slots > 64
            || self.chunk_bytes == 0
            || self.max_bytes == 0
            || self
                .chunk_bytes
                .checked_mul(u64::from(self.slots))
                .and_then(|n| n.checked_mul(2))
                .is_none_or(|n| n > self.max_bytes)
        {
            return Err(crate::Error::invalid_usage(
                "invalid transfer slot count, capacity or byte budget",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransferStats {
    pub committed_bytes: u64,
    pub pending_uploads: u32,
    pub pending_readbacks: u32,
    pub leased_readbacks: u32,
    pub capacity_stalls: u64,
    /// Idle staging slots released under device memory pressure.
    pub evicted_slots: u64,
}

pub struct TransferManager {
    device: Arc<Device>,
    graphics: QueueTarget,
    /// The planned transfer queue when it is not aliased to graphics.
    transfer: Option<QueueTarget>,
    /// Image copies the transfer family may record.
    transfer_images: TransferImageSupport,
    config: TransferConfig,
    /// Staging slots; registered as an evictable cache with the device.
    staging: Arc<parking_lot::Mutex<Staging>>,
    capacity_stalls: u64,
    queued: UploadQueue<Buffer>,
}

/// A successful GPU upload; its completion point is never published on failure.
///
/// `point` is the submit point of the copy on its upload queue. When the copy
/// ran on a transfer queue of another family, it is the release point: the
/// graphics-family acquire is still pending on the device and is recorded by
/// the next render-graph batch (or graph import), the next graphics-queue
/// command of the transfer manager, or [`TransferManager::flush_acquires`].
/// Waiting `point` therefore guarantees the bytes were written, while GPU
/// consumers are ordered by the tracked range state.
#[derive(Clone)]
pub struct UploadTicket {
    device: Arc<Device>,
    buffer: Arc<Buffer>,
    pub point: SyncPoint,
    pub offset: u64,
    pub bytes: u64,
}
impl UploadTicket {
    pub fn wait(&self, timeout_ns: u64) -> crate::Result<()> {
        self.device
            .wait(&SyncSet::from_point(self.point), timeout_ns)
    }
    pub fn is_complete(&self) -> crate::Result<bool> {
        Ok(SyncSet::from_point(self.point).is_reached(&self.device.poll()?))
    }
    /// Bind through the normal graph import validation. The graph plans its
    /// accesses from the buffer's tracked range state, acquiring a range
    /// still released by a cross-family upload itself.
    pub fn import_buffer(
        &self,
        graph: &mut crate::rg::graph::RenderGraph,
        backend: &mut crate::rg::vulkan::VulkanGraphBackend,
        name: &str,
    ) -> crate::Result<crate::rg::resource::BufferId> {
        if backend.device.generation() != self.device.generation()
            || backend.selected_family() != self.device.queues.primary_family
        {
            return Err(crate::Error::invalid_usage(
                "upload ticket belongs to another device or queue-family domain",
            ));
        }
        let token =
            backend.bind_import(crate::rg::vulkan::GpuResource::Buffer(self.buffer.clone()))?;
        let handle = graph.import_unowned(
            name,
            crate::rg::vulkan::ResourceSpec::Buffer(self.buffer.desc).descriptor(),
            token,
            0x37,
        )?;
        graph.buffer_id(handle).map_err(Into::into)
    }
}

/// An accepted CPU upload. No completion point exists until a successful GPU
/// submit. Call `flush_uploads` from the owning frame/service loop, then poll;
/// dropping a ticket does not cancel already accepted work.
#[derive(Clone)]
pub struct QueuedUploadTicket {
    device: Arc<Device>,
    buffer: Arc<Buffer>,
    task: Arc<queue::Task>,
    pub offset: u64,
    pub bytes: u64,
    pub options: UploadOptions,
}
impl QueuedUploadTicket {
    pub fn device_generation(&self) -> crate::device::DeviceGeneration {
        self.device.generation()
    }
    pub fn status(&self) -> UploadStatus {
        self.task.status.lock().clone()
    }
    /// Submitted tickets describe this caller's interval, including when its
    /// dirty data was coalesced with other requests in the same native submit.
    pub fn try_submitted(&self) -> crate::Result<Option<UploadTicket>> {
        match self.status() {
            UploadStatus::Pending => Ok(None),
            UploadStatus::Submitted(point) => Ok(Some(UploadTicket {
                device: self.device.clone(),
                buffer: self.buffer.clone(),
                point,
                offset: self.offset,
                bytes: self.bytes,
            })),
            UploadStatus::Cancelled(_) => {
                Err(crate::Error::invalid_usage("queued upload was cancelled"))
            }
            UploadStatus::Failed(error) => Err(error.to_error()),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct UploadFlush {
    pub submitted_jobs: usize,
    pub submitted_bytes: usize,
    pub remaining_jobs: usize,
    pub capacity_deferred: bool,
}

#[derive(Clone)]
pub struct ReadbackTicket {
    lease: Arc<ReadbackLease>,
}
impl ReadbackTicket {
    pub fn completion(&self) -> SyncPoint {
        self.lease.point
    }
    pub fn len(&self) -> usize {
        self.lease.bytes
    }
    pub fn is_empty(&self) -> bool {
        self.lease.bytes == 0
    }
    pub fn wait(&self, timeout_ns: u64) -> crate::Result<()> {
        self.lease
            .buffer
            .device
            .wait(&SyncSet::from_point(self.lease.point), timeout_ns)
    }
    /// Returns `None` while the GPU is pending. Copying remains protected from
    /// concurrent host accesses and slot reuse, including non-coherent memory.
    pub fn try_read(&self, output: &mut [u8]) -> crate::Result<Option<usize>> {
        if output.len() < self.len() {
            return Err(crate::Error::invalid_usage(
                "readback output slice is too small",
            ));
        }
        if !SyncSet::from_point(self.lease.point).is_reached(&self.lease.buffer.device.poll()?) {
            return Ok(None);
        }
        // SAFETY: this ticket keeps the slot leased, completion was observed,
        // and the transfer commands include the GPU-to-host memory dependency.
        unsafe {
            self.lease
                .buffer
                .read_bytes_after_completion(0, &mut output[..self.len()])?;
        }
        Ok(Some(self.len()))
    }
}

impl TransferManager {
    pub fn new(device: Arc<Device>, config: TransferConfig) -> crate::Result<Self> {
        Self::new_with_queue(device, config, UploadQueueConfig::default())
    }
    pub fn new_with_queue(
        device: Arc<Device>,
        config: TransferConfig,
        queue: UploadQueueConfig,
    ) -> crate::Result<Self> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        config.validate()?;
        let queued = UploadQueue::new(
            device.memory.stats.clone(),
            queue,
            usize::try_from(config.chunk_bytes).map_err(|_| crate::Error::OutOfHostMemory)?,
        )?;
        let graphics =
            QueueTarget::new(&device, device.queue(crate::device::QueueClass::Graphics)?)?;
        let transfer = if device
            .queues
            .plan
            .is_aliased(crate::device::QueueClass::Transfer)
        {
            None
        } else {
            Some(QueueTarget::new(
                &device,
                device.queue(crate::device::QueueClass::Transfer)?,
            )?)
        };
        // Staging is shared by both families instead of transferring ownership.
        let staging_families: Option<[u32; 2]> = transfer
            .as_ref()
            .filter(|target| target.family() != graphics.family())
            .map(|target| [graphics.family(), target.family()]);
        let staging = Arc::new(parking_lot::Mutex::new(Staging::new(
            device.clone(),
            staging_families,
            config.slots,
            config.chunk_bytes,
            config.max_bytes,
            graphics.id,
        )?));
        let evictable: Arc<dyn crate::device::Evictable> = staging.clone();
        device.register_evictable(
            Arc::downgrade(&evictable),
            crate::device::EvictionPriority::Transient,
        );
        let transfer_images = transfer
            .as_ref()
            .map_or(TransferImageSupport::NONE, |target| {
                TransferImageSupport::query(&device, target.family())
            });
        Ok(Self {
            device,
            graphics,
            transfer,
            transfer_images,
            config,
            staging,
            capacity_stalls: 0,
            queued,
        })
    }

    fn validate_buffer(
        &self,
        buffer: &Buffer,
        offset: u64,
        bytes: u64,
        usage: vk::BufferUsageFlags,
    ) -> crate::Result<()> {
        if buffer.device.generation() != self.device.generation()
            || bytes == 0
            || bytes > self.config.chunk_bytes
            || offset
                .checked_add(bytes)
                .is_none_or(|end| end > buffer.size)
            || !buffer.usage.contains(usage)
        {
            return Err(crate::Error::invalid_usage(
                "transfer buffer device, usage or range is invalid",
            ));
        }
        // A range released to the graphics family is acquired by the
        // graphics command that next touches it.
        buffer.last_use.validate_incoming_queue_range(
            self.device.queues.primary_family,
            crate::resource::OwnershipRange::Buffer {
                offset,
                size: bytes,
            },
        )?;
        Ok(())
    }

    /// Accept one dirty interval without issuing GPU work. Higher epochs
    /// cancel older pending jobs of the same physical buffer/logical key;
    /// previously submitted work keeps its actual completion point.
    pub fn enqueue_upload(
        &mut self,
        destination: Arc<Buffer>,
        offset: u64,
        bytes: &[u8],
        options: UploadOptions,
    ) -> crate::Result<QueuedUploadTicket> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        self.validate_buffer(
            &destination,
            offset,
            bytes.len() as u64,
            vk::BufferUsageFlags::TRANSFER_DST,
        )?;
        let task = self
            .queued
            .enqueue(destination.clone(), offset, bytes, options)?;
        Ok(QueuedUploadTicket {
            device: self.device.clone(),
            buffer: destination,
            task,
            offset,
            bytes: bytes.len() as u64,
            options,
        })
    }
    /// Cancelling one coalesced ticket cancels that whole pending native job.
    /// Submitted work cannot be cancelled and retains its completion proof.
    pub fn cancel_upload(&mut self, ticket: &QueuedUploadTicket) -> crate::Result<bool> {
        if ticket.device.generation() != self.device.generation() {
            return Err(crate::Error::invalid_usage(
                "upload ticket belongs to another device",
            ));
        }
        Ok(self.queued.cancel_task(&ticket.task))
    }
    pub fn trim_upload_queue(&mut self) -> usize {
        self.queued.trim_idle()
    }
    pub fn upload_queue_stats(&self) -> UploadQueueStats {
        self.queued.stats()
    }
    pub fn cancel_queued_before(&mut self, epoch: u64) -> usize {
        self.queued.cancel_before(epoch)
    }
    pub fn cancel_key_before(
        &mut self,
        destination: &Arc<Buffer>,
        key: UploadKey,
        epoch: u64,
    ) -> crate::Result<usize> {
        if destination.device.generation() != self.device.generation() {
            return Err(crate::Error::invalid_usage(
                "upload cancellation belongs to another device",
            ));
        }
        Ok(self.queued.cancel_key_before(destination, key, epoch))
    }
    /// Pump bounded work. Full staging defers accepted jobs; allocation/native
    /// errors are terminal for that job and remain visible through its ticket.
    /// Under device-budget pressure only High jobs can be submitted. This never
    /// waits for a device/queue or for an unrelated request.
    pub fn flush_uploads(&mut self, max_submits: usize) -> crate::Result<UploadFlush> {
        let mut result = UploadFlush::default();
        if self.device.memory.stats.budget_action() != crate::device::BudgetAction::None {
            self.queued.trim_idle();
        }
        if max_submits != 0 && self.queued.queued_jobs() != 0 {
            // Poll first: device-level pressure processing can evict idle native
            // allocator blocks before deciding to defer low-priority payloads.
            if let Err(error) = self.device.poll() {
                return Err(self.queued.fail_all(error).to_error());
            }
        }
        for _ in 0..max_submits {
            let Some(index) = self.queued.next(self.device.memory.stats.budget_action()) else {
                break;
            };
            let job = self.queued.job(index);
            let bytes = job.bytes();
            let mut staging = self.staging.lock();
            let mut uploader = Uploader {
                device: &self.device,
                graphics: &self.graphics,
                transfer: self.transfer.as_ref(),
                staging: &mut staging,
                stalls: &mut self.capacity_stalls,
            };
            match uploader.submit(
                &job.target,
                job.segments
                    .iter()
                    .map(|segment| (segment.offset, segment.bytes())),
            ) {
                Ok(point) => {
                    self.queued.publish(index, point);
                    result.submitted_jobs += 1;
                    result.submitted_bytes += bytes;
                }
                Err(crate::Error::Timeout) => {
                    self.queued.defer();
                    result.capacity_deferred = true;
                    break;
                }
                Err(error) => {
                    let failure = if error.is_device_lost() {
                        self.queued.fail_all(error)
                    } else {
                        self.queued.fail(index, error)
                    };
                    return Err(failure.to_error());
                }
            }
        }
        result.remaining_jobs = self.queued.queued_jobs();
        Ok(result)
    }
    /// Immediate compatibility entry. Accepted queued writes to the same target
    /// must be pumped first, so this path cannot overtake them.
    pub fn upload_buffer(
        &mut self,
        destination: Arc<Buffer>,
        offset: u64,
        bytes: &[u8],
    ) -> crate::Result<UploadTicket> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        self.validate_buffer(
            &destination,
            offset,
            bytes.len() as u64,
            vk::BufferUsageFlags::TRANSFER_DST,
        )?;
        if self.queued.has_target(&destination) {
            return Err(crate::Error::Timeout);
        }
        let point = self.with_uploader(|uploader| {
            uploader.submit(&destination, std::iter::once((offset, bytes)))
        })?;
        Ok(UploadTicket {
            device: self.device.clone(),
            buffer: destination,
            point,
            offset,
            bytes: bytes.len() as u64,
        })
    }

    /// Synchronous sparse-update convenience over the same queued service.
    /// Validates all source/destination ranges first, then packs bounded batches.
    /// Each batch waits only for its actual upload point, never device/queue idle.
    /// `timeout_ns` bounds each batch wait. On partial failure already submitted
    /// batches remain valid; unsubmitted data from this batch is cancelled.
    /// Use a manager without other queued jobs; async callers pump explicitly.
    pub fn upload_buffer_ranges_wait(
        &mut self,
        destination: Arc<Buffer>,
        data: &[u8],
        ranges: &[std::ops::Range<usize>],
        destination_offset: u64,
        options: UploadOptions,
        timeout_ns: u64,
    ) -> crate::Result<()> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        crate::raw::buffer::plan_buffer_update(
            &destination.desc,
            data.len(),
            ranges,
            destination_offset,
        )?;
        // Validate every requested ownership interval before submitting the
        // first sparse batch; a later invalid range must not cause partial work.
        if destination.device.generation() != self.device.generation()
            || !destination
                .usage
                .contains(vk::BufferUsageFlags::TRANSFER_DST)
        {
            return Err(crate::Error::invalid_usage(
                "transfer buffer device or usage is invalid",
            ));
        }
        for range in ranges.iter().filter(|range| !range.is_empty()) {
            destination.last_use.validate_incoming_queue_range(
                self.device.queues.primary_family,
                crate::resource::OwnershipRange::Buffer {
                    offset: destination_offset + range.start as u64,
                    size: range.len() as u64,
                },
            )?;
        }
        if self.queued.queued_jobs() != 0 {
            return Err(crate::Error::Timeout);
        }
        let chunk =
            usize::try_from(self.config.chunk_bytes).map_err(|_| crate::Error::OutOfHostMemory)?;
        let mut ticket: Option<QueuedUploadTicket> = None;
        let mut batch_bytes = 0;
        let mut batch_regions = 0;
        let outcome = (|| {
            for range in ranges {
                let mut cursor = range.start;
                while cursor < range.end {
                    if batch_bytes == chunk || batch_regions == self.queued.region_limit() {
                        self.finish_sparse_batch(
                            ticket.take().expect("nonempty admitted batch"),
                            timeout_ns,
                        )?;
                        batch_bytes = 0;
                        batch_regions = 0;
                    }
                    let bytes = (range.end - cursor).min(chunk - batch_bytes);
                    ticket = Some(self.enqueue_upload(
                        destination.clone(),
                        destination_offset + cursor as u64,
                        &data[cursor..cursor + bytes],
                        options,
                    )?);
                    cursor += bytes;
                    batch_bytes += bytes;
                    batch_regions += 1;
                }
            }
            if let Some(last) = ticket.take() {
                self.finish_sparse_batch(last, timeout_ns)?;
            }
            Ok(())
        })();
        if outcome.is_err()
            && let Some(last) = ticket
        {
            let _ = self.cancel_upload(&last);
        }
        outcome
    }
    fn finish_sparse_batch(
        &mut self,
        ticket: QueuedUploadTicket,
        timeout_ns: u64,
    ) -> crate::Result<()> {
        let outcome = (|| {
            self.flush_uploads(1)?;
            ticket
                .try_submitted()?
                .ok_or(crate::Error::Timeout)?
                .wait(timeout_ns)
        })();
        if outcome.is_err() {
            let _ = self.cancel_upload(&ticket);
        }
        outcome
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
    /// is queued on the device (see [`UploadTicket`]). Otherwise it runs on
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
                Err(error) if crate::raw::allocator::is_memory_exhaustion(&error) => {
                    slot = None;
                    None
                }
                Err(error) => return Err(error),
            },
            None => None,
        };
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
        let point = match transfer {
            None => record_graphics_textures(
                &self.device,
                &self.graphics,
                uploads,
                &plans,
                &bases,
                &staging,
                total,
            )?,
            Some(transfer) => record_transfer_textures(
                &self.device,
                &self.graphics,
                transfer,
                uploads,
                &plans,
                &bases,
                &staging,
                total,
            )?,
        };
        if let Some(index) = slot {
            slots.uploads[index].last_use = SyncSet::from_point(point);
        }
        Ok(point)
    }

    /// Ownership acquires queued on the device for the graphics family (by
    /// this manager or any other producer).
    pub fn pending_acquire_count(&self) -> usize {
        self.device.pending_acquire_count(self.graphics.family())
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
            return Ok(None);
        }
        command.end()?;
        let point = command.submit_to(self.graphics.queue, &[], &[])?.point;
        recorded.submitted();
        Ok(Some(point))
    }

    pub fn read_buffer(
        &mut self,
        source: Arc<Buffer>,
        offset: u64,
        bytes: usize,
    ) -> crate::Result<ReadbackTicket> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        self.validate_buffer(
            &source,
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
            command.retain_opaque_buffer_range(readback.clone(), 0, size)?;
            command.retain_recorded(source.clone())?;
        }
        let recorded = RecordedAcquires::begin(&self.device, &mut command)?;
        crate::raw::resource_access::buffer_access_borrowed(
            &mut command,
            &source,
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
        let mut host = crate::raw::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
        );
        host.push(
            crate::raw::barrier::BarrierTarget::Buffer(readback.handle),
            &crate::raw::barrier::RangeBarrier::local(
                crate::resource::OwnershipRange::Buffer { offset: 0, size },
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

    pub fn stats(&self) -> crate::Result<TransferStats> {
        let completed = self.device.poll()?;
        let staging = self.staging.lock();
        Ok(TransferStats {
            committed_bytes: staging.committed_bytes,
            pending_uploads: staging
                .uploads
                .iter()
                .filter(|slot| !slot.last_use.is_reached(&completed))
                .count() as u32,
            pending_readbacks: staging
                .readbacks
                .iter()
                .filter(|slot| !slot.last_use.is_reached(&completed))
                .count() as u32,
            leased_readbacks: staging
                .readbacks
                .iter()
                .filter(|slot| slot.leased())
                .count() as u32,
            capacity_stalls: self.capacity_stalls,
            evicted_slots: staging.evicted_slots,
        })
    }
}

/// Recreating an evicted staging slot under memory pressure defers the
/// request (like full staging) instead of failing it.
fn defer_on_exhaustion(error: crate::Error) -> crate::Error {
    if crate::raw::allocator::is_memory_exhaustion(&error) {
        crate::Error::Timeout
    } else {
        error
    }
}

/// Disjoint borrows of the manager used by one upload submission.
struct Uploader<'a> {
    device: &'a Arc<Device>,
    graphics: &'a QueueTarget,
    transfer: Option<&'a QueueTarget>,
    staging: &'a mut Staging,
    stalls: &'a mut u64,
}
impl TransferManager {
    fn with_uploader<R>(&mut self, run: impl FnOnce(&mut Uploader<'_>) -> R) -> R {
        let mut staging = self.staging.lock();
        run(&mut Uploader {
            device: &self.device,
            graphics: &self.graphics,
            transfer: self.transfer.as_ref(),
            staging: &mut staging,
            stalls: &mut self.capacity_stalls,
        })
    }
}
/// Upper bound of regions per native copy; queue admission enforces it.
const MAX_COPY_REGIONS: usize = 256;

fn route(
    graphics: &QueueTarget,
    transfer: Option<&QueueTarget>,
    destination: &Buffer,
    copies: &[vk::BufferCopy],
) -> UploadRoute {
    {
        let Some(transfer) = transfer else {
            return UploadRoute::Graphics;
        };
        let family = transfer.family();
        let accessible = destination.sharing().allows_family(family)
            && copies.iter().all(|copy| {
                destination
                    .last_use
                    .validate_queue_range(
                        family,
                        crate::resource::OwnershipRange::Buffer {
                            offset: copy.dst_offset,
                            size: copy.size,
                        },
                    )
                    .is_ok()
            });
        record::upload_route(
            graphics.family(),
            Some(family),
            accessible,
            destination.sharing().is_concurrent(),
        )
    }
}

impl Uploader<'_> {
    /// The sole buffer upload kernel. Dirty segments are packed into mapped
    /// staging without copying gaps (one host flush), then copied in one
    /// command with one barrier batch before and after the copies. Every
    /// failure before submit leaves the staging completion unchanged.
    fn submit<'b>(
        &mut self,
        destination: &Arc<Buffer>,
        segments: impl Iterator<Item = (u64, &'b [u8])> + Clone,
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
        let mut count = 0;
        let mut packed = 0u64;
        for (offset, bytes) in segments.clone() {
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
            count += 1;
            packed += bytes.len() as u64;
        }
        if count == 0 {
            return Err(crate::Error::invalid_usage("upload has no segments"));
        }
        let copies = &copies[..count];
        let ranges = &ranges[..count];
        // One host write (one poll and at most one non-coherent flush).
        if count == 1 {
            let (_, bytes) = segments.clone().next().expect("one segment");
            buffer.write_bytes_unsynchronized(0, bytes)?;
        } else {
            // Straight into mapped staging memory: one copy per segment, no
            // intermediate packed Vec.
            buffer.write_with_unsynchronized(0, packed, |dst| {
                let mut at = 0;
                for (_, bytes) in segments {
                    dst[at..at + bytes.len()].copy_from_slice(bytes);
                    at += bytes.len();
                }
            })?;
        }
        let route = route(self.graphics, self.transfer, destination, copies);
        let target = match route {
            UploadRoute::Graphics => self.graphics,
            _ => self.transfer.expect("route has a transfer queue"),
        };
        let mut command = target.pool.allocate()?;
        // SAFETY: the copy reads exactly the packed staging span.
        unsafe {
            command.retain_opaque_buffer_range(buffer.clone(), 0, packed)?;
        }
        // Acquires still pending for this family (e.g. of an earlier upload
        // to the same destination) ride in this command.
        let recorded = RecordedAcquires::begin(self.device, &mut command)?;
        let mut pieces: Vec<WrittenPiece> = Vec::new();
        record::record_buffer_writes(&mut command, destination, target, ranges, &mut pieces)?;
        // SAFETY: recording; staging and destination are retained.
        unsafe {
            self.device.core.device.cmd_copy_buffer(
                command.handle,
                buffer.handle,
                destination.handle,
                copies,
            );
        }
        let mut released = Vec::new();
        match route {
            UploadRoute::Graphics => {
                record::record_buffer_visibility(&command, destination.handle, ranges)?
            }
            // The semaphore wait derived from the tracked writer point makes
            // the copy visible to every later consumer on graphics.
            UploadRoute::SameFamily => {}
            UploadRoute::OwnershipTransfer => record::record_buffer_release(
                &mut command,
                destination,
                &pieces,
                self.graphics.family(),
                &mut released,
            )?,
        }
        command.end()?;
        // Reserve the registrations before the release is submitted, so that
        // host allocation failure cannot strand a released range.
        self.device.reserve_pending_acquires(released.len())?;
        // A failed submit publishes no state: nothing was released and no
        // acquire becomes pending.
        let submitted = command.submit_to(target.queue, &[], &[])?;
        recorded.submitted();
        self.staging.uploads[index].last_use = SyncSet::from_point(submitted.point);
        record::queue_acquires(
            self.device,
            &crate::device::AcquireTarget::buffer(destination),
            &released,
            submitted.point,
        )?;
        Ok(submitted.point)
    }
}

/// Transfer-family capabilities relevant to image copies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TransferImageSupport {
    /// `minImageTransferGranularity` is (1,1,1): arbitrary regions are legal.
    pub fine_granularity: bool,
    /// The family supports graphics, so depth/stencil aspects may be copied.
    pub graphics: bool,
}
impl TransferImageSupport {
    const NONE: Self = Self {
        fine_granularity: false,
        graphics: false,
    };
    fn query(device: &Device, family: u32) -> Self {
        // SAFETY: valid physical device of a live instance.
        let families = unsafe {
            device
                .core
                .instance
                .instance
                .get_physical_device_queue_family_properties(device.core.physical)
        };
        families
            .get(family as usize)
            .map_or(Self::NONE, |properties| {
                let granularity = properties.min_image_transfer_granularity;
                Self {
                    fine_granularity: (granularity.width, granularity.height, granularity.depth)
                        == (1, 1, 1),
                    graphics: properties.queue_flags.contains(vk::QueueFlags::GRAPHICS),
                }
            })
    }
}

/// Whether a texture batch may run on the dedicated transfer queue: the
/// family copies arbitrary regions, may copy every aspect involved, each
/// texture is writable there without an acquire, and no texture repeats
/// (an unsubmitted release cannot be followed by another use in the batch).
fn texture_route(
    transfer_family: u32,
    support: TransferImageSupport,
    uploads: &[TextureUpload<'_>],
) -> bool {
    support.fine_granularity
        && uploads.iter().enumerate().all(|(index, upload)| {
            let aspects =
                crate::raw::format::vk_format_valid_aspect_flags(upload.texture.desc.format);
            (support.graphics
                || !aspects.intersects(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL))
                && texture::transfer_accessible(upload.texture, transfer_family)
                && !uploads[..index]
                    .iter()
                    .any(|previous| Arc::ptr_eq(previous.texture, upload.texture))
        })
}

/// How each texture of a transfer-queue batch is published to graphics.
fn texture_publish(
    graphics_family: u32,
    transfer_family: u32,
    concurrent: bool,
) -> TransferPublish {
    if transfer_family == graphics_family || concurrent {
        TransferPublish::Local
    } else {
        TransferPublish::Release {
            destination_family: graphics_family,
        }
    }
}

fn record_graphics_textures(
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
        command.retain_opaque_buffer_range(staging.clone(), 0, total)?;
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
fn record_transfer_textures(
    device: &Device,
    graphics: &QueueTarget,
    transfer: &QueueTarget,
    uploads: &[TextureUpload<'_>],
    plans: &[texture::TexturePlan],
    bases: &[u64],
    staging: &Arc<Buffer>,
    total: u64,
) -> crate::Result<SyncPoint> {
    let mut command = transfer.pool.allocate()?;
    let mut released: Vec<Vec<crate::resource::range_state::RangeTransfer>> = Vec::new();
    released
        .try_reserve_exact(uploads.len())
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    // SAFETY: the copies read only this staging span.
    unsafe {
        command.retain_opaque_buffer_range(staging.clone(), 0, total)?;
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
    device.reserve_pending_acquires(released.iter().map(Vec::len).sum())?;
    let point = command.submit_to(transfer.queue, &[], &[])?.point;
    recorded.submitted();
    for (upload, pieces) in uploads.iter().zip(&released) {
        record::queue_acquires(
            device,
            &crate::device::AcquireTarget::image(upload.texture),
            pieces,
            point,
        )?;
    }
    Ok(point)
}

/// Pending acquires recorded at the start of one of this manager's commands.
/// They go back to the device queue unless that command is submitted.
struct RecordedAcquires<'a> {
    device: &'a Device,
    drained: crate::device::DrainedAcquires,
}
impl<'a> RecordedAcquires<'a> {
    /// Begin `command` and record every acquire pending for its family.
    fn begin(
        device: &'a Device,
        command: &mut crate::raw::command_buffer::VulkanCommandBuffer,
    ) -> crate::Result<Self> {
        command.begin()?;
        Ok(Self {
            device,
            drained: device.record_pending_acquires(command)?,
        })
    }
    fn is_empty(&self) -> bool {
        self.drained.is_empty()
    }
    /// The command was submitted: its acquires are no longer pending.
    fn submitted(mut self) {
        self.drained.clear();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transfer_configuration_rejects_unbounded_or_overflowing_capacity() {
        for config in [
            TransferConfig {
                slots: 0,
                ..Default::default()
            },
            TransferConfig {
                slots: 65,
                ..Default::default()
            },
            TransferConfig {
                chunk_bytes: u64::MAX,
                ..Default::default()
            },
            TransferConfig {
                max_bytes: 1,
                ..Default::default()
            },
        ] {
            assert!(config.validate().is_err());
        }
    }

    #[test]
    fn texture_publish_moves_ownership_only_for_exclusive_images_of_another_family() {
        assert_eq!(texture_publish(0, 0, false), TransferPublish::Local);
        assert_eq!(texture_publish(0, 2, true), TransferPublish::Local);
        assert_eq!(
            texture_publish(0, 2, false),
            TransferPublish::Release {
                destination_family: 0
            }
        );
    }

    mod cross_family {
        //! Cross-family uploads are simulated: the release state is recorded
        //! on the resource as if family 1 had submitted it, then registered
        //! through the same path as a transfer-queue upload.
        use super::*;
        use crate::device::pending_acquire::tests::{owner, released_buffer_with};
        use crate::resource::range_state::{RangeOwner, RangeTransfer};
        use crate::sync::QueueId;

        const GRAPHICS: QueueId = match QueueId::new(0) {
            Some(queue) => queue,
            None => unreachable!(),
        };

        fn device() -> Option<Arc<Device>> {
            crate::device::Device::new_headless(&crate::device::DeviceConfig::headless()).ok()
        }
        fn submitted(device: &Device) -> u64 {
            device.queues.timelines.submitted(GRAPHICS).unwrap()
        }
        /// `count` buffers released to the graphics family whose acquires are
        /// queued by the transfer registration path.
        fn released_uploads(
            device: &Arc<Device>,
            count: usize,
            usage: vk::BufferUsageFlags,
        ) -> Vec<(Arc<Buffer>, RangeTransfer)> {
            // SAFETY: an empty batch only signals the device timeline.
            let release = unsafe { device.submit(GRAPHICS, &[]) }.unwrap();
            let uploads: Vec<_> = (0..count)
                .map(|_| released_buffer_with(device, release, usage))
                .collect();
            device.reserve_pending_acquires(count).unwrap();
            for (buffer, transfer) in &uploads {
                record::queue_acquires(
                    device,
                    &crate::device::AcquireTarget::buffer(buffer),
                    std::slice::from_ref(transfer),
                    release,
                )
                .unwrap();
            }
            uploads
        }

        #[test]
        fn uploads_followed_by_a_graph_frame_cost_no_extra_submit() {
            use crate::raw::buffer::BufferDesc;
            use crate::rg::vulkan::{
                AccessTable, ResourceSpec, VulkanGraphBackend, VulkanGraphSession,
            };
            let Some(device) = device() else {
                eprintln!("SKIP: no Vulkan device");
                return;
            };
            let uploads = released_uploads(&device, 8, vk::BufferUsageFlags::TRANSFER_DST);
            let manager = TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
            assert_eq!(manager.pending_acquire_count(), 8);
            let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
            let mut session = VulkanGraphSession::new(backend);
            let before = submitted(&device);
            let mut graph = session.begin_frame().unwrap();
            let scratch = session
                .backend
                .declare(
                    &mut graph,
                    "scratch",
                    ResourceSpec::Buffer(BufferDesc::new_gpu_only(
                        64,
                        vk::BufferUsageFlags::TRANSFER_DST,
                    )),
                )
                .unwrap();
            let mut pass = graph.add_pass("fill").with_side_effect();
            pass.add_resource_access(scratch, 0x34, 0).unwrap();
            pass.render(|_| true);
            let receipt = session.execute_frame(graph.build()).unwrap();
            assert_eq!(receipt.stats.coalesced_acquires, 8);
            assert_eq!(
                submitted(&device) - before,
                u64::from(receipt.stats.batches),
                "the acquires ride in the graph's own submits"
            );
            assert_eq!(manager.pending_acquire_count(), 0);
            session.wait_submits_done(10_000_000_000).unwrap();
            for (buffer, _) in &uploads {
                assert_eq!(owner(buffer), RangeOwner::Owned(0));
            }
        }

        #[test]
        fn flush_acquires_records_every_pending_acquire_in_one_submit() {
            let Some(device) = device() else {
                eprintln!("SKIP: no Vulkan device");
                return;
            };
            let mut manager =
                TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
            assert_eq!(manager.flush_acquires().unwrap(), None);
            let uploads = released_uploads(&device, 6, vk::BufferUsageFlags::TRANSFER_DST);
            // A raw command touching a released range fails validation
            // instead of racing the release.
            let pool =
                crate::raw::command_buffer::VulkanCommandPool::new_for_family(device.clone(), 0)
                    .unwrap();
            let mut command = pool.allocate().unwrap();
            command.begin().unwrap();
            let error = crate::raw::resource_access::buffer_access_borrowed(
                &mut command,
                &uploads[0].0,
                GRAPHICS,
                0,
                256,
                AccessScope {
                    stages: vk::PipelineStageFlags::TRANSFER,
                    access: vk::AccessFlags::TRANSFER_READ,
                    write: false,
                },
            )
            .unwrap_err();
            assert!(error.to_string().contains("pending acquire"), "{error}");
            drop(command);
            let before = submitted(&device);
            let point = manager
                .flush_acquires()
                .unwrap()
                .expect("one acquire submit");
            assert_eq!(submitted(&device), before + 1);
            assert_eq!(manager.pending_acquire_count(), 0);
            manager
                .device
                .wait(&SyncSet::from_point(point), 10_000_000_000)
                .unwrap();
            for (buffer, _) in &uploads {
                assert_eq!(owner(buffer), RangeOwner::Owned(0));
            }
            assert_eq!(manager.flush_acquires().unwrap(), None);
            assert_eq!(submitted(&device), before + 1);
        }

        #[test]
        fn graphics_transfer_commands_record_pending_acquires_first() {
            let Some(device) = device() else {
                eprintln!("SKIP: no Vulkan device");
                return;
            };
            let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC;
            let uploads = released_uploads(&device, 2, usage);
            let mut manager =
                TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
            // An upload to a still-released destination acquires it (and
            // every other pending acquire) in its own command.
            let ticket = manager
                .upload_buffer(uploads[0].0.clone(), 16, &[7; 32])
                .unwrap();
            assert_eq!(manager.pending_acquire_count(), 0);
            ticket.wait(10_000_000_000).unwrap();
            for (buffer, _) in &uploads {
                assert_eq!(owner(buffer), RangeOwner::Owned(0));
            }
            let readback = manager.read_buffer(uploads[0].0.clone(), 16, 32).unwrap();
            readback.wait(10_000_000_000).unwrap();
            let mut bytes = [0; 32];
            assert_eq!(readback.try_read(&mut bytes).unwrap(), Some(32));
            assert_eq!(bytes, [7; 32]);
            // A readback of a released source acquires it the same way.
            let released = released_uploads(&device, 1, usage);
            let readback = manager.read_buffer(released[0].0.clone(), 0, 16).unwrap();
            assert_eq!(manager.pending_acquire_count(), 0);
            readback.wait(10_000_000_000).unwrap();
            assert_eq!(owner(&released[0].0), RangeOwner::Owned(0));
        }
    }
}
