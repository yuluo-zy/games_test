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
use crate::device::Device;
#[cfg(all(test, feature = "cpu-probe"))]
use crate::sync::SyncSet;
use crate::resource::buffer::Buffer;
mod queue;
mod record;
internal_mod!(setup);
mod staging;
pub(crate) mod texture;
#[cfg(test)]
use ash::vk;
use queue::UploadQueue;
pub use queue::{
    OwnedUploadError, UploadCancellation, UploadFailure, UploadKey, UploadMode, UploadOptions,
    UploadPriority, UploadQueueConfig, UploadQueueStats, UploadStatus,
};
use record::QueueTarget;
use staging::StagingLock;
use std::sync::Arc;
pub use setup::{
    SETUP_WAIT_TIMEOUT_NS, SetupAccess, SetupContext, SetupStats, SetupSubmission,
    TextureInitialData, init_texture_data, update_texture, update_texture_regions,
    with_setup_command, with_setup_resources,
};
pub use texture::TextureUpload;
mod config;
mod tickets;
mod enqueue;
mod flush;
mod route;
use route::MAX_UPLOAD_TARGETS;
mod submit;
pub use config::{TransferConfig, TransferStats};
pub use tickets::{UploadTicket, QueuedUploadTicket, UploadFlush, ReadbackTicket};
#[cfg(any(test, feature = "cpu-probe"))]
pub use tickets::QueuedUploadObservation;
use route::TransferImageSupport;

pub struct TransferManager {
    device: Arc<Device>,
    graphics: QueueTarget,
    /// The planned transfer queue when it is not aliased to graphics.
    transfer: Option<QueueTarget>,
    /// Image copies the transfer family may record.
    transfer_images: TransferImageSupport,
    config: TransferConfig,
    /// Staging slots; registered as an evictable cache with the device.
    staging: Arc<StagingLock>,
    capacity_stalls: u64,
    queued: UploadQueue<Buffer>,
}

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "cpu-probe"))]
mod weak_upload_observation_tests;
