//! Format-derived aspect masks, mip counts and image create info.
use ash::vk;
use crate::Result;
use super::desc::TextureDesc;

/// Aspect mask of a format: depth/stencil aspects for D16 / X8_D24 / D32 / S8 /
/// D16S8 / D24S8 / D32S8 (`{2,2,2,4,6,6,6}`), COLOR otherwise.
pub fn image_aspect_mask_from_format(format: vk::Format) -> vk::ImageAspectFlags {
    const TABLE: [u32; 7] = [2, 2, 2, 4, 6, 6, 6];
    let index = (format.as_raw() as u32).wrapping_sub(0x7c);
    vk::ImageAspectFlags::from_raw(TABLE.get(index as usize).copied().unwrap_or(1))
}

/// 1D 尺寸的全 mip 数量：32 - leading_zeros(size)。
pub fn mip_count_1d(size: u32) -> u32 {
    32 - size.leading_zeros()
}

/// Stage names of texture creation failures, for diagnostics built with
/// [`create_failure_text`]; creation itself returns `Err(Error::Vulkan(..))`.
pub mod create_failure {
    /// `vkCreateImage` failed.
    pub const CREATE_IMAGE: &str = "create_image";
    /// `vkBindImageMemory` failed.
    pub const BIND_IMAGE_MEMORY: &str = "bind_image_memory";
    /// `vkSetDebugUtilsObjectNameEXT` failed.
    pub const SET_DEBUG_UTILS_OBJECT_NAME: &str = "set_debug_utils_object_name";
}

/// `"{stage}: {result:?}"` diagnostic text.
pub fn create_failure_text(stage: &str, result: vk::Result) -> String {
    format!("{stage}: {result:?}")
}

// Cube kinds add this native flag even when the caller leaves flags empty.
// Creation and view compatibility must use the same effective flags.
pub(super) fn image_create_flags(desc: &TextureDesc) -> vk::ImageCreateFlags {
    if desc.kind.is_cube() {
        desc.flags | vk::ImageCreateFlags::CUBE_COMPATIBLE
    } else {
        desc.flags
    }
}

/// `ImageCreateInfo` for `desc`.
pub fn get_image_create_info(desc: &TextureDesc) -> Result<vk::ImageCreateInfo<'_>> {
    get_image_create_info_with_data(desc, false)
}

/// Build an optimally tiled device-local image. Initial pixels are staged after
/// creation, so both initialized and empty images start in UNDEFINED. Vulkan
/// only permits PREINITIALIZED with LINEAR tiling; it is never used by this path.
/// Cube kinds add CUBE_COMPATIBLE so subsequent cube views remain valid.
pub fn get_image_create_info_with_data(
    desc: &TextureDesc,
    _has_data: bool,
) -> Result<vk::ImageCreateInfo<'_>> {
    let array_layers = desc.array_layers()?;
    let flags = image_create_flags(desc);
    Ok(vk::ImageCreateInfo::default()
        .flags(flags)
        .image_type(desc.kind.image_type())
        .format(desc.format)
        .extent(desc.kind.image_extent(desc))
        .mip_levels(u32::from(desc.mip_levels))
        .array_layers(array_layers)
        .samples(desc.samples())
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(desc.usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED))
}
