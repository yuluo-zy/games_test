//! Bounded asynchronous buffer and texture transfers.
//!
//! Uploads run on the device's planned transfer queue when it is a distinct
//! queue and the destination's ownership allows it; otherwise on the graphics
//! queue. A transfer queue of another family releases the written ranges and a
//! graphics-queue acquire publishes them, so every consumer sees the data as
//! graphics-owned and orders after the returned point. Barriers come from the
//! tracked resource state as one batch per command.
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
    raw::buffer::{Buffer, BufferDesc},
    resource::range_state::AccessScope,
    sync::{SyncPoint, SyncSet},
};
mod queue;
mod record;
pub(crate) mod texture;
use ash::vk;
use queue::UploadQueue;
pub use queue::{
    UploadCancellation, UploadFailure, UploadKey, UploadOptions, UploadPriority, UploadQueueConfig,
    UploadQueueStats, UploadStatus,
};
use record::{PendingAcquire, QueueTarget, UploadRoute, WrittenPiece};
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

struct UploadSlot {
    buffer: Arc<Buffer>,
    last_use: SyncSet,
}
struct ReadbackSlot {
    buffer: Arc<Buffer>,
    last_use: SyncSet,
    lease: Arc<ReadbackLease>,
}
struct ReadbackLease {
    buffer: Arc<Buffer>,
    point: SyncPoint,
    bytes: usize,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransferStats {
    pub committed_bytes: u64,
    pub pending_uploads: u32,
    pub pending_readbacks: u32,
    pub leased_readbacks: u32,
    pub capacity_stalls: u64,
}

pub struct TransferManager {
    device: Arc<Device>,
    graphics: QueueTarget,
    /// The planned transfer queue when it is not aliased to graphics.
    transfer: Option<QueueTarget>,
    /// Image copies the transfer family may record.
    transfer_images: TransferImageSupport,
    /// Ownership acquires whose release already executed on the transfer
    /// queue but whose graphics submit failed; retried before new work.
    pending_acquires: Vec<AcquireBatch>,
    config: TransferConfig,
    uploads: Vec<UploadSlot>,
    readbacks: Vec<ReadbackSlot>,
    committed_bytes: u64,
    capacity_stalls: u64,
    queued: UploadQueue<Buffer>,
    scratch: Scratch,
    // Vectors drop before their actual capacity credit.
    _slots_memory: crate::raw::allocator::CpuMemoryReservation,
}

/// A successful GPU upload; its completion point is never published on failure.
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
    /// Bind through the normal graph import validation. The queue is shared with
    /// the upload, so its normal submission order and barriers carry the result.
    pub fn import_buffer(
        &self,
        graph: &mut crate::rg::graph::RenderGraph,
        backend: &mut crate::rg::vulkan::VulkanGraphBackend,
        name: &str,
    ) -> crate::Result<crate::rg::resource::BufferId> {
        if backend.device.generation() != self.device.generation()
            || backend.selected_family() != self.device.queue_family
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
            device.allocator_stats.clone(),
            queue,
            usize::try_from(config.chunk_bytes).map_err(|_| crate::Error::OutOfHostMemory)?,
        )?;
        let graphics =
            QueueTarget::new(&device, device.queue(crate::device::QueueClass::Graphics)?)?;
        let transfer = if device
            .queue_plan
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
        let staging = |desc: &BufferDesc| -> crate::Result<Arc<Buffer>> {
            let mut buffer = match staging_families {
                Some(families) => Buffer::new_concurrent(device.clone(), desc, &families)?,
                None => Buffer::new_dedicated(device.clone(), desc)?,
            };
            // Map once for the slot lifetime; every later host copy reuses it.
            // SAFETY: the buffer is new, so no GPU or host access can overlap.
            unsafe {
                buffer.mapped_slice_mut(0, desc.size)?;
            }
            Ok(Arc::new(buffer))
        };
        let scratch = Scratch::new(&device, config.chunk_bytes)?;
        let mut slots_memory = device.allocator_stats.reserve_cpu_bytes(
            u64::from(config.slots) * (size_of::<UploadSlot>() + size_of::<ReadbackSlot>()) as u64,
            crate::device::BudgetCategory::Live,
        )?;
        let mut uploads = Vec::new();
        let mut readbacks = Vec::new();
        uploads
            .try_reserve_exact(config.slots as usize)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        readbacks
            .try_reserve_exact(config.slots as usize)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        slots_memory.resize(
            (uploads.capacity() * size_of::<UploadSlot>()
                + readbacks.capacity() * size_of::<ReadbackSlot>()) as u64,
        )?;
        let mut committed_bytes = 0u64;
        for _ in 0..config.slots {
            let upload = staging(&BufferDesc::new_cpu_to_gpu(
                config.chunk_bytes,
                vk::BufferUsageFlags::TRANSFER_SRC,
            ))?;
            let readback = staging(&BufferDesc::new_gpu_to_cpu(
                config.chunk_bytes,
                vk::BufferUsageFlags::TRANSFER_DST,
            ))?;
            committed_bytes = committed_bytes
                .checked_add(upload.allocation_bytes())
                .and_then(|n| n.checked_add(readback.allocation_bytes()))
                .ok_or(crate::Error::OutOfDeviceMemory)?;
            if committed_bytes > config.max_bytes {
                return Err(crate::Error::OutOfDeviceMemory);
            }
            uploads.push(UploadSlot {
                buffer: upload,
                last_use: SyncSet::EMPTY,
            });
            let memory = device.allocator_stats.reserve_cpu_bytes(
                crate::raw::allocator::shared_cpu_bytes::<ReadbackLease>()?,
                crate::device::BudgetCategory::Live,
            )?;
            let lease = Arc::new(ReadbackLease {
                buffer: readback.clone(),
                point: SyncPoint {
                    queue: graphics.id,
                    value: 0,
                },
                bytes: 0,
                _memory: memory,
            });
            readbacks.push(ReadbackSlot {
                buffer: readback,
                last_use: SyncSet::EMPTY,
                lease,
            });
        }
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
            pending_acquires: Vec::new(),
            config,
            scratch,
            uploads,
            readbacks,
            committed_bytes,
            capacity_stalls: 0,
            queued,
            _slots_memory: slots_memory,
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
        buffer.last_use.validate_queue_range(
            self.device.queue_family,
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
        self.retry_pending_acquires()?;
        let mut result = UploadFlush::default();
        if self.device.allocator_stats.budget_action() != crate::device::BudgetAction::None {
            self.queued.trim_idle();
        }
        if max_submits != 0 && self.queued.stats().queued_jobs != 0 {
            // Poll first: device-level pressure processing can evict idle native
            // allocator blocks before deciding to defer low-priority payloads.
            if let Err(error) = self.device.poll() {
                return Err(self.queued.fail_all(error).to_error());
            }
        }
        for _ in 0..max_submits {
            let Some(index) = self
                .queued
                .next(self.device.allocator_stats.budget_action())
            else {
                break;
            };
            let job = self.queued.job(index);
            let bytes = job.bytes();
            let mut uploader = Uploader {
                device: &self.device,
                graphics: &self.graphics,
                transfer: self.transfer.as_ref(),
                uploads: &mut self.uploads,
                stalls: &mut self.capacity_stalls,
                scratch: &mut self.scratch,
                pending_acquires: &mut self.pending_acquires,
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
        result.remaining_jobs = self.queued.stats().queued_jobs;
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
        self.retry_pending_acquires()?;
        let point = self
            .uploader()
            .submit(&destination, std::iter::once((offset, bytes)))?;
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
            destination.last_use.validate_queue_range(
                self.device.queue_family,
                crate::resource::OwnershipRange::Buffer {
                    offset: destination_offset + range.start as u64,
                    size: range.len() as u64,
                },
            )?;
        }
        if self.queued.stats().queued_jobs != 0 {
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
    /// released there (including the final layout transition) and acquired on
    /// the graphics queue. Otherwise it runs on the graphics queue. Never
    /// waits; the returned point (the acquire point when ownership moved) is
    /// at or after every texture's recorded last use.
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
        self.retry_pending_acquires()?;
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
        let slot = if total <= self.config.chunk_bytes {
            self.uploads
                .iter()
                .position(|slot| slot.last_use.is_reached(&completed))
        } else {
            None
        };
        let staging = match slot {
            Some(index) => {
                let bytes = self.scratch.buffer(total as usize)?;
                texture::pack_all(&pairs, &bases, total, bytes)?;
                self.uploads[index]
                    .buffer
                    .write_bytes_unsynchronized(0, bytes)?;
                self.uploads[index].buffer.clone()
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
            None => {
                record_graphics_textures(&self.graphics, uploads, &plans, &bases, &staging, total)?
            }
            Some(transfer) => {
                let (release, acquire) = record_transfer_textures(
                    &self.graphics,
                    transfer,
                    uploads,
                    &plans,
                    &bases,
                    &staging,
                    total,
                )?;
                if let Some(index) = slot {
                    self.uploads[index].last_use = SyncSet::from_point(release);
                }
                return match acquire {
                    None => Ok(release),
                    Some((command, batches)) => acquire_or_defer(
                        &self.graphics,
                        command,
                        batches,
                        &mut self.pending_acquires,
                    ),
                };
            }
        };
        if let Some(index) = slot {
            self.uploads[index].last_use = SyncSet::from_point(point);
        }
        Ok(point)
    }

    /// Ownership acquires waiting for a graphics submit after an earlier
    /// failure. Their resources cannot be used until these complete.
    pub fn pending_acquire_count(&self) -> usize {
        self.pending_acquires.len()
    }

    /// Resubmit acquires left pending by a failed graphics submit. On failure
    /// they stay pending and the error is returned.
    pub fn retry_pending_acquires(&mut self) -> crate::Result<Option<SyncPoint>> {
        if self.pending_acquires.is_empty() {
            return Ok(None);
        }
        let command = self.graphics.pool.allocate()?;
        let batches = std::mem::take(&mut self.pending_acquires);
        acquire_or_defer(&self.graphics, command, batches, &mut self.pending_acquires).map(Some)
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
        let Some(slot) = self.readbacks.iter_mut().find(|slot| {
            Arc::strong_count(&slot.lease) == 1
                && Arc::weak_count(&slot.lease) == 0
                && slot.last_use.is_reached(&completed)
        }) else {
            self.capacity_stalls += 1;
            return Err(crate::Error::Timeout);
        };
        let size = bytes as u64;
        let target = &self.graphics;
        let mut command = target.pool.allocate()?;
        // SAFETY: the copy writes exactly these slot bytes; the source access is
        // recorded below from its tracked state.
        unsafe {
            command.retain_opaque_buffer_range(slot.buffer.clone(), 0, size)?;
            command.retain_recorded(source.clone())?;
        }
        command.begin()?;
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
            self.device.device.cmd_copy_buffer(
                command.handle,
                source.handle,
                slot.buffer.handle,
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
        host.buffers.push(
            vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(slot.buffer.handle)
                .offset(0)
                .size(size),
        );
        record::issue(&command, &host)?;
        command.end()?;
        let submitted = command.submit_to(target.queue, &[], &[])?;
        // Publication happens only after a successful submit; a failed submit
        // leaves the private lease untouched.
        let lease = Arc::get_mut(&mut slot.lease).expect("lease is private until publication");
        lease.point = submitted.point;
        lease.bytes = bytes;
        slot.last_use = SyncSet::from_point(submitted.point);
        Ok(ReadbackTicket {
            lease: slot.lease.clone(),
        })
    }

    pub fn stats(&self) -> crate::Result<TransferStats> {
        let completed = self.device.poll()?;
        Ok(TransferStats {
            committed_bytes: self.committed_bytes,
            pending_uploads: self
                .uploads
                .iter()
                .filter(|slot| !slot.last_use.is_reached(&completed))
                .count() as u32,
            pending_readbacks: self
                .readbacks
                .iter()
                .filter(|slot| !slot.last_use.is_reached(&completed))
                .count() as u32,
            leased_readbacks: self
                .readbacks
                .iter()
                .filter(|slot| {
                    Arc::strong_count(&slot.lease) != 1 || Arc::weak_count(&slot.lease) != 0
                })
                .count() as u32,
            capacity_stalls: self.capacity_stalls,
        })
    }
}

/// Reusable host packing buffer for multi-segment jobs, charged to the budget.
struct Scratch {
    bytes: Vec<u8>,
    limit: usize,
    memory: crate::raw::allocator::CpuMemoryReservation,
}
impl Scratch {
    fn new(device: &Device, limit: u64) -> crate::Result<Self> {
        Ok(Self {
            bytes: Vec::new(),
            limit: usize::try_from(limit).map_err(|_| crate::Error::OutOfHostMemory)?,
            memory: device
                .allocator_stats
                .reserve_cpu_bytes(0, crate::device::BudgetCategory::Live)?,
        })
    }
    /// An empty vector with capacity for `total` bytes (at most one chunk).
    fn buffer(&mut self, total: usize) -> crate::Result<&mut Vec<u8>> {
        if total > self.limit {
            return Err(crate::Error::invalid_usage(
                "upload exceeds admitted staging capacity",
            ));
        }
        self.bytes.clear();
        if self.bytes.capacity() < total {
            let target = total.next_power_of_two().min(self.limit);
            self.memory.resize(target as u64)?;
            self.bytes
                .try_reserve_exact(target)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
        }
        Ok(&mut self.bytes)
    }
    /// Pack `segments` contiguously and return the packed bytes.
    fn pack<'a>(
        &mut self,
        segments: impl Iterator<Item = &'a [u8]>,
        total: usize,
    ) -> crate::Result<&[u8]> {
        let bytes = self.buffer(total)?;
        for segment in segments {
            bytes.extend_from_slice(segment);
        }
        Ok(bytes)
    }
}

/// Disjoint borrows of the manager used by one upload submission.
struct Uploader<'a> {
    device: &'a Arc<Device>,
    graphics: &'a QueueTarget,
    transfer: Option<&'a QueueTarget>,
    uploads: &'a mut [UploadSlot],
    stalls: &'a mut u64,
    scratch: &'a mut Scratch,
    pending_acquires: &'a mut Vec<AcquireBatch>,
}
impl TransferManager {
    fn uploader(&mut self) -> Uploader<'_> {
        Uploader {
            device: &self.device,
            graphics: &self.graphics,
            transfer: self.transfer.as_ref(),
            uploads: &mut self.uploads,
            stalls: &mut self.capacity_stalls,
            scratch: &mut self.scratch,
            pending_acquires: &mut self.pending_acquires,
        }
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
        let Some(slot) = self
            .uploads
            .iter_mut()
            .find(|slot| slot.last_use.is_reached(&completed))
        else {
            *self.stalls += 1;
            return Err(crate::Error::Timeout);
        };
        let mut copies = [vk::BufferCopy::default(); MAX_COPY_REGIONS];
        let mut ranges = [(0u64, 0u64); MAX_COPY_REGIONS];
        let mut count = 0;
        let mut packed = 0u64;
        for (offset, bytes) in segments.clone() {
            if count == copies.len()
                || packed
                    .checked_add(bytes.len() as u64)
                    .is_none_or(|end| end > slot.buffer.size)
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
            slot.buffer.write_bytes_unsynchronized(0, bytes)?;
        } else {
            let bytes = self
                .scratch
                .pack(segments.map(|(_, bytes)| bytes), packed as usize)?;
            slot.buffer.write_bytes_unsynchronized(0, bytes)?;
        }
        let route = route(self.graphics, self.transfer, destination, copies);
        let target = match route {
            UploadRoute::Graphics => self.graphics,
            _ => self.transfer.expect("route has a transfer queue"),
        };
        let mut command = target.pool.allocate()?;
        // Allocate the acquire before the release is submitted, so that host
        // allocation failure cannot strand a released range.
        let acquire = if route == UploadRoute::OwnershipTransfer {
            Some(self.graphics.pool.allocate()?)
        } else {
            None
        };
        // SAFETY: the copy reads exactly the packed staging span.
        unsafe {
            command.retain_opaque_buffer_range(slot.buffer.clone(), 0, packed)?;
        }
        command.begin()?;
        let mut pieces: Vec<WrittenPiece> = Vec::new();
        record::record_buffer_writes(&mut command, destination, target, ranges, &mut pieces)?;
        // SAFETY: recording; staging and destination are retained.
        unsafe {
            self.device.device.cmd_copy_buffer(
                command.handle,
                slot.buffer.handle,
                destination.handle,
                copies,
            );
        }
        let mut released: Vec<PendingAcquire> = Vec::new();
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
        // A failed submit publishes no state: nothing was released and no
        // acquire becomes pending.
        let submitted = command.submit_to(target.queue, &[], &[])?;
        slot.last_use = SyncSet::from_point(submitted.point);
        let Some(acquire) = acquire else {
            return Ok(submitted.point);
        };
        let mut batches = Vec::new();
        batches
            .try_reserve_exact(1)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        batches.push(AcquireBatch {
            target: AcquireTarget::Buffer(destination.clone()),
            acquires: released,
            release_point: submitted.point,
        });
        acquire_or_defer(self.graphics, acquire, batches, self.pending_acquires)
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
                .instance
                .instance
                .get_physical_device_queue_family_properties(device.physical)
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
    command.begin()?;
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
    Ok(command.submit_to(target.queue, &[], &[])?.point)
}

type PreparedAcquire = (
    crate::raw::command_buffer::VulkanCommandBuffer,
    Vec<AcquireBatch>,
);

/// Record and submit a texture batch on the transfer queue. Returns the
/// transfer submit point and, when ownership moved, the preallocated acquire
/// command with its batches. A failed submit publishes no release.
fn record_transfer_textures(
    graphics: &QueueTarget,
    transfer: &QueueTarget,
    uploads: &[TextureUpload<'_>],
    plans: &[texture::TexturePlan],
    bases: &[u64],
    staging: &Arc<Buffer>,
    total: u64,
) -> crate::Result<(SyncPoint, Option<PreparedAcquire>)> {
    let publish: Vec<TransferPublish> = uploads
        .iter()
        .map(|upload| {
            texture_publish(
                graphics.family(),
                transfer.family(),
                upload.texture.sharing().is_concurrent(),
            )
        })
        .collect();
    let mut command = transfer.pool.allocate()?;
    let acquire = if publish.iter().all(|p| *p == TransferPublish::Local) {
        None
    } else {
        Some(graphics.pool.allocate()?)
    };
    let mut released: Vec<Vec<PendingAcquire>> = Vec::new();
    released
        .try_reserve_exact(uploads.len())
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    // SAFETY: the copies read only this staging span.
    unsafe {
        command.retain_opaque_buffer_range(staging.clone(), 0, total)?;
    }
    command.begin()?;
    for (((upload, plan), base), publish) in uploads.iter().zip(plans).zip(bases).zip(&publish) {
        let mut pieces = Vec::new();
        texture::record_texture_copies_on_transfer(
            &mut command,
            upload.texture,
            transfer.id,
            staging.handle,
            *base,
            plan,
            upload.old_layout,
            *publish,
            &mut pieces,
        )?;
        released.push(pieces);
    }
    command.end()?;
    let point = command.submit_to(transfer.queue, &[], &[])?.point;
    let Some(acquire) = acquire else {
        return Ok((point, None));
    };
    let mut batches = Vec::new();
    batches
        .try_reserve_exact(uploads.len())
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    for (upload, acquires) in uploads.iter().zip(released) {
        if !acquires.is_empty() {
            batches.push(AcquireBatch {
                target: AcquireTarget::Texture(upload.texture.clone()),
                acquires,
                release_point: point,
            });
        }
    }
    Ok((point, Some((acquire, batches))))
}

/// A resource released on the transfer queue that the graphics queue must
/// acquire before any other use.
enum AcquireTarget {
    Buffer(Arc<Buffer>),
    Texture(Arc<crate::raw::texture::Texture>),
}
struct AcquireBatch {
    target: AcquireTarget,
    acquires: Vec<PendingAcquire>,
    release_point: SyncPoint,
}

/// Submit the acquires; on failure keep every batch pending for a retry.
fn acquire_or_defer(
    graphics: &QueueTarget,
    command: crate::raw::command_buffer::VulkanCommandBuffer,
    batches: Vec<AcquireBatch>,
    pending: &mut Vec<AcquireBatch>,
) -> crate::Result<SyncPoint> {
    match submit_acquires(graphics, command, &batches) {
        Ok(point) => Ok(point),
        Err(error) => {
            if !error.is_device_lost() {
                // Keeping the batches is required for correctness: the
                // released ranges are unusable until acquired.
                pending
                    .try_reserve(batches.len())
                    .map_err(|_| crate::Error::OutOfHostMemory)?;
                pending.extend(batches);
            }
            Err(error)
        }
    }
}

/// The sole submission point of cross-family ownership acquires created by
/// transfer uploads: one graphics-queue command with one barrier batch per
/// resource, waiting on the release points through the tracked state.
/// Replace this body to coalesce the acquires into the next graphics submit.
fn submit_acquires(
    graphics: &QueueTarget,
    mut command: crate::raw::command_buffer::VulkanCommandBuffer,
    batches: &[AcquireBatch],
) -> crate::Result<SyncPoint> {
    command.begin()?;
    for batch in batches {
        match &batch.target {
            AcquireTarget::Buffer(buffer) => record::record_buffer_acquire(
                &mut command,
                buffer,
                &batch.acquires,
                batch.release_point,
            )?,
            AcquireTarget::Texture(texture) => texture::record_texture_acquire(
                &mut command,
                texture,
                &batch.acquires,
                batch.release_point,
            )?,
        }
    }
    command.end()?;
    Ok(command.submit_to(graphics.queue, &[], &[])?.point)
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
}
