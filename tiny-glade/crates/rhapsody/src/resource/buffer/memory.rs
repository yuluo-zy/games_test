//! Memory locations and buffer alignment rules.
use ash::vk;

/// Buffer memory location (raw values 1..=4; 0 is invalid).
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryLocation {
    GpuOnly = 1,
    CpuToGpu = 2,
    /// HOST_VISIBLE 但非低延迟映射，不要求 HOST_COHERENT。
    CpuToGpuLowPrio = 3,
    GpuToCpu = 4,
}

impl MemoryLocation {
    pub const fn as_raw(self) -> u8 {
        self as u8
    }
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(Self::GpuOnly),
            2 => Some(Self::CpuToGpu),
            3 => Some(Self::CpuToGpuLowPrio),
            4 => Some(Self::GpuToCpu),
            _ => None,
        }
    }
    /// 返回 (必需, 偏好) 内存属性；映射集中在此处，创建路径不再自行拼装。
    pub fn memory_properties(self) -> (vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) {
        use vk::MemoryPropertyFlags as F;
        match self {
            Self::GpuOnly => (F::DEVICE_LOCAL, F::DEVICE_LOCAL),
            Self::CpuToGpu => (F::HOST_VISIBLE, F::HOST_COHERENT),
            Self::CpuToGpuLowPrio => (F::HOST_VISIBLE, F::empty()),
            Self::GpuToCpu => (F::HOST_VISIBLE, F::HOST_CACHED | F::HOST_COHERENT),
        }
    }
}

/// Usage bit `0x400` raises the minimum buffer alignment to 64 bytes.
pub const USAGE_ALIGN_64_BIT_RAW: u32 = 0x400;

/// Buffer memory alignment: `max(memreq.alignment, desc.alignment,
/// (usage & 0x400) ? 0x40 : 0x10)`。
pub fn buffer_alignment(requirement_alignment: u64, desc_alignment: u16, usage: u32) -> u64 {
    let floor = if usage & USAGE_ALIGN_64_BIT_RAW != 0 {
        0x40
    } else {
        0x10
    };
    requirement_alignment
        .max(u64::from(desc_alignment))
        .max(floor)
}

/// Names of the Vulkan calls that can fail during buffer creation, used in
/// error text (see [`crate::resource::texture::create_failure_text`]).
pub mod create_failure {
    pub const CREATE_BUFFER: &str = "create_buffer";
    pub const BIND_BUFFER_MEMORY: &str = "bind_buffer_memory";
    pub const SET_DEBUG_UTILS_OBJECT_NAME: &str = "set_debug_utils_object_name";
}
