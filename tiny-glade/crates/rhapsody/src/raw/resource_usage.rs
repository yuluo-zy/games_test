// 候选实现迁移来源：reconstruction/render/resource_usage.rs.
// 这是行为重建，不是原作者源码。
//! 来自原机器码的私有访问掩码到资源用途转换。
//!
//! 输入是原私有访问掩码，不能当成 Vulkan access mask。类型名称未知；Vulkan 输出复用原 ash 依赖。

use ash::vk::{BufferUsageFlags, ImageUsageFlags};

/// 原函数符号：`rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags`
/// 原机器码地址：`0x140bd4b00`.
///
/// 原代码接受任意输入位，未消费位不产生用途标记；不代表游戏会产生所有位组合。
pub const fn buffer_access_mask_to_usage_flags(access_mask: u32) -> BufferUsageFlags {
    let mut flags = ((access_mask & 0x6) << 5) | ((access_mask & 0x1) << 8);
    flags |= (access_mask & 0x8) << 1;
    if access_mask & 0x18060 != 0 {
        flags |= BufferUsageFlags::STORAGE_BUFFER.as_raw();
    }
    flags |= (access_mask >> 11) & 0x3;
    BufferUsageFlags::from_raw(flags)
}

/// 原函数符号：`rhapsody::rg::resource_info::image_access_mask_to_usage_flags`
/// 原机器码地址：`0x140bd49d0`.
///
/// Unsupported image access masks are invalid declarations, not device failures.
pub fn image_access_mask_to_usage_flags(access_mask: u32) -> crate::Result<ImageUsageFlags> {
    Ok(match access_mask {
        0 => ImageUsageFlags::empty(),
        0x20 => ImageUsageFlags::SAMPLED,
        0x40 | 0x18000 => ImageUsageFlags::STORAGE,
        0x80 | 0x100 | 0x180 => ImageUsageFlags::COLOR_ATTACHMENT,
        0x200 | 0x400 | 0x600 => ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        0x220 => ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | ImageUsageFlags::SAMPLED,
        0x800 => ImageUsageFlags::TRANSFER_SRC,
        0x1000 => ImageUsageFlags::TRANSFER_DST,
        _ => {
            return Err(crate::Error::invalid_usage(format!(
                "unsupported image access mask: {access_mask:#x}"
            )));
        }
    })
}
