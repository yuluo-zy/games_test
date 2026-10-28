//! Textures, texture descriptions and image views.
use crate::device::Device;
use ash::vk;
use std::sync::Arc;
mod info;
mod kind;
mod initial;
mod desc;
mod view;
mod create;
pub use info::{image_aspect_mask_from_format, mip_count_1d, create_failure, create_failure_text, get_image_create_info, get_image_create_info_with_data};
pub use kind::{TextureSampleCount, TextureKind};
pub use initial::LinearInitialDataIter;
pub use desc::{TextureDesc, TextureViewDesc};
pub use view::{TextureView, MipChainIter};
use create::TextureStorage;

const _: () = assert!(std::mem::size_of::<TextureDesc>() == 32);
pub struct Texture {
    retirement: Option<crate::device::RetireTicket>,
    pub(crate) sharing: crate::tracking::ResourceSharing,
    allocation_use: Option<Arc<crate::tracking::SyncCell>>,
    pub(crate) last_use: Arc<crate::tracking::SyncCell>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::Image,
    pub desc: TextureDesc,
    storage: TextureStorage,
    _memory: crate::accounting::CpuMemoryReservation,
}

impl Texture {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::Image {
        self.handle
    }
}

