//! Private rhapsody resource-usage conversions recovered from the shipped binary.
//!
//! The input is the original renderer's access-mask bitset, not a Vulkan access
//! mask. Its named source type has not yet been recovered. Vulkan output types
//! come directly from the original ash dependency (0.38.0+1.3.281).
//! See evidence/render-leaves/findings.md and verification.json.

use ash::vk::{BufferUsageFlags, ImageUsageFlags};

/// `rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags`
/// at original address `0x140bd4b00`.
///
/// The machine code tolerates arbitrary input bits: unconsumed bits contribute
/// no usage flags. This does not establish that such inputs occur in the game.
pub const fn buffer_access_mask_to_usage_flags(access_mask: u32) -> BufferUsageFlags {
    let mut flags = ((access_mask & 0x6) << 5) | ((access_mask & 0x1) << 8);
    flags |= (access_mask & 0x8) << 1;
    if access_mask & 0x18060 != 0 {
        flags |= BufferUsageFlags::STORAGE_BUFFER.as_raw();
    }
    flags |= (access_mask >> 11) & 0x3;
    BufferUsageFlags::from_raw(flags)
}

/// `rhapsody::rg::resource_info::image_access_mask_to_usage_flags`
/// at original address `0x140bd49d0`.
///
/// Unlike the buffer conversion, this matches whole masks. The original
/// panics for every mask outside the listed combinations; it does not union
/// the independently requested usages. Panic formatting/stack ABI is outside
/// the recovered behavioral contract.
pub fn image_access_mask_to_usage_flags(access_mask: u32) -> ImageUsageFlags {
    match access_mask {
        0x20 => ImageUsageFlags::SAMPLED,
        0x40 | 0x18000 => ImageUsageFlags::STORAGE,
        0x80 | 0x100 | 0x180 => ImageUsageFlags::COLOR_ATTACHMENT,
        0x200 | 0x400 | 0x600 => ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        0x220 => ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | ImageUsageFlags::SAMPLED,
        0x800 => ImageUsageFlags::TRANSFER_SRC,
        0x1000 => ImageUsageFlags::TRANSFER_DST,
        _ => panic!("unsupported original image access mask: {access_mask:#x}"),
    }
}
