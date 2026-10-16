//! Vulkan 格式元数据（Khronos 注册表生成）。
use ash::vk;
#[path = "format_table.rs"]
mod registry;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormatInfo {
    pub block_extent: [u32; 3],
    pub block_bytes: u32,
    pub aspect: vk::ImageAspectFlags,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneInfo {
    pub width_divisor: u32,
    pub height_divisor: u32,
    pub compatible: vk::Format,
}
pub fn vk_format_info(format: vk::Format) -> Option<FormatInfo> {
    registry::registry_info(format)
}
pub fn vk_format_valid_aspect_flags(format: vk::Format) -> vk::ImageAspectFlags {
    vk_format_info(format).map_or(vk::ImageAspectFlags::empty(), |info| info.aspect)
}
pub fn vk_format_block_bytes(format: vk::Format) -> Option<u32> {
    vk_format_info(format).map(|i| i.block_bytes)
}
pub fn vk_format_planes(format: vk::Format) -> &'static [PlaneInfo] {
    registry::registry_planes(format)
}
/// 整图视图使用 COLOR；选择单平面时额外允许 PLANE_n。
pub fn vk_format_all_aspects(format: vk::Format) -> vk::ImageAspectFlags {
    let mut flags = vk_format_valid_aspect_flags(format);
    for i in 0..vk_format_planes(format).len() {
        flags |= vk::ImageAspectFlags::from_raw(0x10 << i);
    }
    flags
}

/// 原 ash 对应头文件版本的公开注册表，含原私有入口不支持的扩展/多平面格式。
pub mod khronos {
    pub use super::vk_format_all_aspects as all_aspects;
    pub use super::vk_format_block_bytes as block_bytes;
    pub use super::vk_format_info as info;
    pub use super::vk_format_planes as planes;
    pub use super::vk_format_valid_aspect_flags as valid_aspect_flags;
    pub use super::{FormatInfo, PlaneInfo};
}
