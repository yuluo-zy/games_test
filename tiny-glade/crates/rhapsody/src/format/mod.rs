//! 公共 Vulkan 注册表与原发行格式函数分别提供接口。
//! 顶层旧接口保留候选兼容性；原构建的精确输出/失败规则使用 observed。
use ash::vk;
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
    for i in 0..vk_format_planes(format).len().min(3) {
        flags |= vk::ImageAspectFlags::from_raw(0x10 << i);
    }
    flags
}

/// 原 ash 对应头文件版本的公开注册表，含原私有入口不支持的扩展/多平面格式。
pub mod khronos {
    pub use crate::format::vk_format_all_aspects as all_aspects;
    pub use crate::format::vk_format_block_bytes as block_bytes;
    pub use crate::format::vk_format_info as info;
    pub use crate::format::vk_format_planes as planes;
    pub use crate::format::vk_format_valid_aspect_flags as valid_aspect_flags;
    pub use crate::format::{FormatInfo, PlaneInfo};
}

/// 原构建的完整入口行为。安全载体与 Result 适配不宣称原 Rust 类型/返回 ABI 全部恢复。
pub mod observed {
    use crate::format::registry;
    use ash::vk;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FormatError {
        UnsupportedAspect(i32),
        UnsupportedInfo(i32),
        UnsupportedBlockBytes(i32),
        OutputTooShort,
    }
    /// 原 vk_format_info 写入的精确 10 字节；输出位置 +10 以后不写入。
    /// 普通格式的两个 word 与通道数/存储字节吻合；未把这些推断当完整原字段名。
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct FormatInfo {
        pub bytes: [u8; 10],
    }
    impl FormatInfo {
        pub fn word_at_00(self) -> u32 {
            u32::from_le_bytes(self.bytes[0..4].try_into().unwrap())
        }
        pub fn word_at_04(self) -> u32 {
            u32::from_le_bytes(self.bytes[4..8].try_into().unwrap())
        }
        pub fn byte_at_08(self) -> u8 {
            self.bytes[8]
        }

        pub fn write_into(self, destination: &mut [u8]) -> Result<(), FormatError> {
            if destination.len() < 10 {
                return Err(FormatError::OutputTooShort);
            }
            destination[..10].copy_from_slice(&self.bytes);
            Ok(())
        }
    }
    pub fn try_vk_format_valid_aspect_flags(
        format: vk::Format,
    ) -> Result<vk::ImageAspectFlags, FormatError> {
        let index = (format.as_raw() as u32).wrapping_sub(1);
        if index >= 184 {
            return Err(FormatError::UnsupportedAspect(format.as_raw()));
        }
        Ok(vk::ImageAspectFlags::from_raw(
            registry::OBSERVED_ASPECTS[index as usize],
        ))
    }
    pub fn try_vk_format_info(format: vk::Format) -> Result<FormatInfo, FormatError> {
        let index = format.as_raw() as u32;
        if index >= 147 {
            return Err(FormatError::UnsupportedInfo(format.as_raw()));
        }
        let mut bytes = [0; 10];
        bytes[..4].copy_from_slice(&registry::OBSERVED_INFO_WORD00[index as usize].to_le_bytes());
        bytes[4..8].copy_from_slice(&registry::OBSERVED_INFO_WORD04[index as usize].to_le_bytes());
        bytes[8] = registry::OBSERVED_INFO_BYTE08[index as usize];
        bytes[9] = bytes[8];
        Ok(FormatInfo { bytes })
    }
    /// 名称来自原符号；原 1..130 返回 1，不能当标准每像素/压缩块字节数。
    /// 压缩分支只有 bitmask 指定子集，缺失分支保持原失败行为。
    pub fn try_vk_format_block_bytes(format: vk::Format) -> Result<u32, FormatError> {
        let raw = format.as_raw() as u32;
        if raw.wrapping_sub(1) < 130 {
            return Ok(1);
        }
        let index = raw.wrapping_sub(131);
        if index <= 15 {
            if (0xccc0 >> index) & 1 != 0 {
                return Ok(16);
            }
            if (3 >> index) & 1 != 0 || index == 8 {
                return Ok(8);
            }
        }
        Err(FormatError::UnsupportedBlockBytes(format.as_raw()))
    }
}
