//! Routing of uploads to the graphics or transfer queue and texture support.
use ash::vk;
use crate::device::Device;
use crate::resource::buffer::Buffer;
use std::sync::Arc;
use super::record;
use super::record::QueueTarget;
use super::record::UploadRoute;
use super::staging::Staging;
use super::texture;
use super::texture::TextureUpload;
use super::texture::TransferPublish;

/// Upper bound of regions per native copy; queue admission enforces it.
pub(super) const MAX_COPY_REGIONS: usize = 256;

pub(super) const MAX_UPLOAD_TARGETS: usize = 64;

/// Disjoint borrows of the manager used by one upload submission.
pub(super) struct Uploader<'a> {
    pub(super) device: &'a Arc<Device>,
    pub(super) graphics: &'a QueueTarget,
    pub(super) transfer: Option<&'a QueueTarget>,
    pub(super) staging: &'a mut Staging,
    pub(super) stalls: &'a mut u64,
}

pub(super) fn route(
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
                        crate::tracking::OwnershipRange::Buffer {
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

/// Recreating an evicted staging slot under memory pressure defers the
/// request (like full staging) instead of failing it.
pub(super) fn defer_on_exhaustion(error: crate::Error) -> crate::Error {
    if crate::device::allocator::is_memory_exhaustion(&error) {
        crate::Error::Timeout
    } else {
        error
    }
}

/// Transfer-family capabilities relevant to image copies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TransferImageSupport {
    /// `minImageTransferGranularity` is (1,1,1): arbitrary regions are legal.
    pub fine_granularity: bool,
    /// The family supports graphics, so depth/stencil aspects may be copied.
    pub graphics: bool,
}

impl TransferImageSupport {
    pub(super) const NONE: Self = Self {
        fine_granularity: false,
        graphics: false,
    };
    pub(super) fn query(device: &Device, family: u32) -> Self {
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
pub(super) fn texture_route(
    transfer_family: u32,
    support: TransferImageSupport,
    uploads: &[TextureUpload<'_>],
) -> bool {
    support.fine_granularity
        && uploads.iter().enumerate().all(|(index, upload)| {
            let aspects =
                crate::format::vk_format_valid_aspect_flags(upload.texture.desc.format);
            (support.graphics
                || !aspects.intersects(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL))
                && texture::transfer_accessible(upload.texture, transfer_family)
                && !uploads[..index]
                    .iter()
                    .any(|previous| Arc::ptr_eq(previous.texture, upload.texture))
        })
}

/// How each texture of a transfer-queue batch is published to graphics.
pub(super) fn texture_publish(
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
