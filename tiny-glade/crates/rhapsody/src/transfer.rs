//! Bounded asynchronous buffer transfers on the production graphics queue.
//!
//! Each slot owns a separate mapped allocation. Host writes cannot overlap a
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
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
    },
    sync::{SyncPoint, SyncSet},
};
mod queue;
use ash::vk;
use queue::UploadQueue;
pub use queue::{
    UploadCancellation, UploadFailure, UploadKey, UploadOptions, UploadPriority, UploadQueueConfig,
    UploadQueueStats, UploadStatus,
};
use std::sync::Arc;

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
    pool: Arc<VulkanCommandPool>,
    config: TransferConfig,
    uploads: Vec<UploadSlot>,
    readbacks: Vec<ReadbackSlot>,
    committed_bytes: u64,
    capacity_stalls: u64,
    queued: UploadQueue<Buffer>,
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
        let pool = VulkanCommandPool::new(device.clone())?;
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
            let upload = Arc::new(Buffer::new_dedicated(
                device.clone(),
                &BufferDesc::new_cpu_to_gpu_low_prio(
                    config.chunk_bytes,
                    vk::BufferUsageFlags::TRANSFER_SRC,
                ),
            )?);
            let readback = Arc::new(Buffer::new_dedicated(
                device.clone(),
                &BufferDesc::new_gpu_to_cpu(config.chunk_bytes, vk::BufferUsageFlags::TRANSFER_DST),
            )?);
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
                    queue: device.queue(crate::device::QueueClass::Graphics)?,
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
        Ok(Self {
            device,
            pool,
            config,
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
            match submit_upload(
                &self.device,
                &self.pool,
                &mut self.uploads,
                &mut self.capacity_stalls,
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
        let point = submit_upload(
            &self.device,
            &self.pool,
            &mut self.uploads,
            &mut self.capacity_stalls,
            &destination,
            std::iter::once((offset, bytes)),
        )?;
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
        // The slot's baseline owner keeps a budgeted envelope reusable. Only
        // CPU-exclusive, GPU-completed leases can reach this mutation.
        Arc::get_mut(&mut slot.lease)
            .expect("exclusive readback slot")
            .bytes = bytes;
        let mut command = self.pool.allocate()?;
        // The recorded copy and both barriers access these exact intervals.
        unsafe {
            command.retain_opaque_buffer_range(source.clone(), offset, bytes as u64)?;
            command.retain_opaque_buffer_range(slot.buffer.clone(), 0, bytes as u64)?;
        }
        command.begin()?;
        unsafe {
            buffer_barrier(
                &self.device,
                command.handle,
                source.handle,
                offset,
                bytes as u64,
                (
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    vk::AccessFlags::MEMORY_WRITE,
                ),
                (
                    vk::PipelineStageFlags::TRANSFER,
                    vk::AccessFlags::TRANSFER_READ,
                ),
            );
            self.device.device.cmd_copy_buffer(
                command.handle,
                source.handle,
                slot.buffer.handle,
                &[vk::BufferCopy {
                    src_offset: offset,
                    dst_offset: 0,
                    size: bytes as u64,
                }],
            );
            buffer_barrier(
                &self.device,
                command.handle,
                slot.buffer.handle,
                0,
                bytes as u64,
                (
                    vk::PipelineStageFlags::TRANSFER,
                    vk::AccessFlags::TRANSFER_WRITE,
                ),
                (vk::PipelineStageFlags::HOST, vk::AccessFlags::HOST_READ),
            );
        }
        command.end()?;
        let submitted = command.submit_to(
            crate::raw::device::DeviceQueue {
                handle: self.device.queue,
                family: self.device.queue_family,
                index: 0,
            },
            &[],
            &[],
        )?;
        Arc::get_mut(&mut slot.lease)
            .expect("lease is private until publication")
            .point = submitted.point;
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

/// The sole upload submit kernel. Dirty segments are packed into mapped staging
/// without copying gaps, then copied in one command/submit. Every failure before
/// submit leaves the staging completion unchanged; success records it before
/// any caller can observe/publish the point.
fn submit_upload<'a>(
    device: &Arc<Device>,
    pool: &Arc<VulkanCommandPool>,
    uploads: &mut [UploadSlot],
    stalls: &mut u64,
    destination: &Arc<Buffer>,
    segments: impl Iterator<Item = (u64, &'a [u8])>,
) -> crate::Result<SyncPoint> {
    device.try_collect_garbage()?;
    let completed = device.poll()?;
    let Some(slot) = uploads
        .iter_mut()
        .find(|slot| slot.last_use.is_reached(&completed))
    else {
        *stalls += 1;
        return Err(crate::Error::Timeout);
    };
    // Queue admission limits this table to 256 and the immediate path uses one.
    let mut copies = [vk::BufferCopy::default(); 256];
    let mut count = 0;
    let mut packed = 0u64;
    for (offset, bytes) in segments {
        if count == copies.len()
            || packed
                .checked_add(bytes.len() as u64)
                .is_none_or(|end| end > slot.buffer.size)
        {
            return Err(crate::Error::invalid_usage(
                "upload segments exceed admitted staging capacity",
            ));
        }
        slot.buffer.write_bytes_unsynchronized(packed, bytes)?;
        copies[count] = vk::BufferCopy {
            src_offset: packed,
            dst_offset: offset,
            size: bytes.len() as u64,
        };
        count += 1;
        packed += bytes.len() as u64;
    }
    let mut command = pool.allocate()?;
    // Admission precedes the queue call; no range metadata can fail once GPU
    // work has been submitted. Packed source and destination spans are distinct.
    unsafe {
        command.retain_opaque_buffer_range(slot.buffer.clone(), 0, packed)?;
        for copy in &copies[..count] {
            command.retain_opaque_buffer_range(destination.clone(), copy.dst_offset, copy.size)?;
        }
    }
    command.begin()?;
    unsafe {
        for copy in &copies[..count] {
            buffer_barrier(
                device,
                command.handle,
                destination.handle,
                copy.dst_offset,
                copy.size,
                (
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
                ),
                (
                    vk::PipelineStageFlags::TRANSFER,
                    vk::AccessFlags::TRANSFER_WRITE,
                ),
            );
        }
        device.device.cmd_copy_buffer(
            command.handle,
            slot.buffer.handle,
            destination.handle,
            &copies[..count],
        );
        for copy in &copies[..count] {
            buffer_barrier(
                device,
                command.handle,
                destination.handle,
                copy.dst_offset,
                copy.size,
                (
                    vk::PipelineStageFlags::TRANSFER,
                    vk::AccessFlags::TRANSFER_WRITE,
                ),
                (
                    vk::PipelineStageFlags::ALL_COMMANDS,
                    vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
                ),
            );
        }
    }
    command.end()?;
    let submitted = command.submit_to(
        crate::raw::device::DeviceQueue {
            handle: device.queue,
            family: device.queue_family,
            index: 0,
        },
        &[],
        &[],
    )?;
    slot.last_use = SyncSet::from_point(submitted.point);
    Ok(submitted.point)
}

unsafe fn buffer_barrier(
    device: &Device,
    command: vk::CommandBuffer,
    buffer: vk::Buffer,
    offset: u64,
    size: u64,
    source: (vk::PipelineStageFlags, vk::AccessFlags),
    destination: (vk::PipelineStageFlags, vk::AccessFlags),
) {
    let barrier = vk::BufferMemoryBarrier::default()
        .buffer(buffer)
        .offset(offset)
        .size(size)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .src_access_mask(source.1)
        .dst_access_mask(destination.1);
    unsafe {
        device.device.cmd_pipeline_barrier(
            command,
            source.0,
            destination.0,
            vk::DependencyFlags::empty(),
            &[],
            &[barrier],
            &[],
        );
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
}
