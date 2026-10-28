//! Upload, queued-upload and readback tickets handed back to callers.
use crate::device::Device;
use crate::resource::buffer::Buffer;
use crate::sync::SyncPoint;
use crate::sync::SyncSet;
use std::sync::Arc;
use super::queue;
use super::queue::UploadOptions;
use super::queue::UploadStatus;
use super::staging::ReadbackLease;

/// A successful GPU upload; its completion point is never published on failure.
///
/// `point` is the submit point of the copy on its upload queue. When the copy
/// ran on a transfer queue of another family, it is the release point: the
/// graphics-family acquire is still pending on the device and is recorded by
/// the next render-graph batch (or graph import), the next graphics-queue
/// command of the transfer manager, or `TransferManager::flush_acquires`.
/// Waiting `point` therefore guarantees the bytes were written, while GPU
/// consumers are ordered by the tracked range state.
#[derive(Clone)]
pub struct UploadTicket {
    pub(crate) device: Arc<Device>,
    pub(crate) buffer: Arc<Buffer>,
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
}

/// Diagnostic view of an existing upload task. It retains only its independent
/// CPU status/envelope and Weak native owners. The current success/cancellation
/// paths retain no native owner; an arbitrary future failure Error source must
/// be audited separately. Holding this view
/// delays task-slot reuse/CPU-credit return; release it after diagnostic export.
#[cfg(any(test, feature = "cpu-probe"))]
#[derive(Clone)]
pub struct QueuedUploadObservation {
    task: Arc<queue::Task>,
    buffer: std::sync::Weak<Buffer>,
    device: std::sync::Weak<Device>,
    pub device_generation: crate::sync::DeviceGeneration,
    pub offset: u64,
    pub bytes: u64,
    pub options: UploadOptions,
}

#[cfg(any(test, feature = "cpu-probe"))]
impl QueuedUploadObservation {
    /// Exact task status remains observable after Job/manager removal. A
    /// Submitted point is published submission evidence, not completed shader output.
    /// Pending/Failed cannot prove no native submit: native success followed by
    /// acquire-publication failure has no published task point and is invalid
    /// for this correctness proof. Preserve its status and aggregate prefix.
    pub fn status(&self) -> UploadStatus { self.task.status.lock().clone() }
    pub fn owner_alive(&self) -> bool { self.buffer.strong_count() != 0 }
    pub fn device_alive(&self) -> bool { self.device.strong_count() != 0 }
    pub fn same_owner(&self, owner: &Arc<Buffer>) -> bool {
        self.buffer.ptr_eq(&Arc::downgrade(owner))
    }
    pub fn weak_owner(&self) -> std::sync::Weak<Buffer> { self.buffer.clone() }
}

/// An accepted CPU upload. No completion point exists until a successful GPU
/// submit. Call `flush_uploads` from the owning frame/service loop, then poll;
/// dropping a ticket does not cancel already accepted work.
#[derive(Clone)]
pub struct QueuedUploadTicket {
    pub(super) device: Arc<Device>,
    pub(super) buffer: Arc<Buffer>,
    pub(super) task: Arc<queue::Task>,
    pub offset: u64,
    pub bytes: u64,
    pub options: UploadOptions,
}

impl QueuedUploadTicket {
    pub fn device_generation(&self) -> crate::sync::DeviceGeneration {
        self.device.generation()
    }
    #[cfg(any(test, feature = "cpu-probe"))]
    pub fn observe(&self) -> QueuedUploadObservation {
        QueuedUploadObservation { task: self.task.clone(), buffer: Arc::downgrade(&self.buffer),
            device: Arc::downgrade(&self.device), device_generation: self.device.generation(),
            offset: self.offset, bytes: self.bytes, options: self.options }
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
    pub(super) lease: Arc<ReadbackLease>,
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
