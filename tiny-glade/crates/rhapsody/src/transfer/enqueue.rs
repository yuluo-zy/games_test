//! Enqueueing, validating and cancelling uploads.
use ash::vk;
use crate::device::Device;
use crate::resource::buffer::Buffer;
use std::sync::Arc;
use super::TransferManager;
use super::config::TransferConfig;
use super::config::TransferStats;
use super::queue::OwnedUploadError;
use super::queue::UploadKey;
use super::queue::UploadMode;
use super::queue::UploadOptions;
use super::queue::UploadQueue;
use super::queue::UploadQueueConfig;
use super::queue::UploadQueueStats;
use super::record::QueueTarget;
use super::route::TransferImageSupport;
use super::staging::Staging;
use super::tickets::QueuedUploadTicket;

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
        let evictable: Arc<dyn crate::accounting::Evictable> = staging.clone();
        device.register_evictable(
            Arc::downgrade(&evictable),
            crate::accounting::EvictionPriority::Transient,
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

    pub(super) fn validate_buffer(
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
            crate::tracking::OwnershipRange::Buffer {
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

    /// Accept an owned interval without a CPU payload copy. Ordered deltas
    /// never supersede one another and have independent completion tickets.
    /// Replacement mode is only valid when no accepted frame still requires
    /// an older pending version. Admission failure returns the allocation.
    pub fn enqueue_upload_owned(
        &mut self,
        destination: Arc<Buffer>,
        offset: u64,
        bytes: Vec<u8>,
        options: UploadOptions,
        mode: UploadMode,
    ) -> core::result::Result<QueuedUploadTicket, OwnedUploadError> {
        let validation = if self.device.is_device_lost() {
            Err(crate::Error::DeviceLost)
        } else {
            self.validate_buffer(
                &destination,
                offset,
                bytes.len() as u64,
                vk::BufferUsageFlags::TRANSFER_DST,
            )
        };
        if let Err(error) = validation {
            return Err(OwnedUploadError {
                error,
                payload: bytes,
            });
        }
        let len = bytes.len() as u64;
        let task = self
            .queued
            .enqueue_owned(destination.clone(), offset, bytes, options, mode)?;
        Ok(QueuedUploadTicket {
            device: self.device.clone(),
            buffer: destination,
            task,
            offset,
            bytes: len,
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

    pub fn cancel_queued_before(&mut self, epoch: u64) -> usize {
        self.queued.cancel_before(epoch)
    }

    #[doc(hidden)]
    pub fn fixture_buffer_weak_history_count(&self, target: &Arc<Buffer>) -> usize {
        self.queued.fixture_weak_history_count(target)
    }

    pub fn upload_queue_stats(&self) -> UploadQueueStats {
        self.queued.stats()
    }

    /// Ownership acquires queued on the device for the graphics family (by
    /// this manager or any other producer).
    pub fn pending_acquire_count(&self) -> usize {
        self.device.pending_acquire_count(self.graphics.family())
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
