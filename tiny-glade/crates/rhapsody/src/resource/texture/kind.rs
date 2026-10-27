//! Texture kinds and sample counts.
use ash::vk;
use super::desc::TextureDesc;

/// Sample count; `to_vk` maps to Vulkan `SampleCountFlags`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureSampleCount {
    One,
    Two,
    Four,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

impl TextureSampleCount {
    pub fn to_vk(self) -> vk::SampleCountFlags {
        match self {
            Self::One => vk::SampleCountFlags::TYPE_1,
            Self::Two => vk::SampleCountFlags::TYPE_2,
            Self::Four => vk::SampleCountFlags::TYPE_4,
            Self::Eight => vk::SampleCountFlags::TYPE_8,
            Self::Sixteen => vk::SampleCountFlags::TYPE_16,
            Self::ThirtyTwo => vk::SampleCountFlags::TYPE_32,
            Self::SixtyFour => vk::SampleCountFlags::TYPE_64,
        }
    }
}

/// Image kind.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextureKind {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D3 = 4,
    Cube = 5,
    CubeArray = 6,
}

impl TextureKind {
    pub fn image_type(self) -> vk::ImageType {
        match self {
            Self::D1 | Self::D1Array => vk::ImageType::TYPE_1D,
            Self::D3 => vk::ImageType::TYPE_3D,
            _ => vk::ImageType::TYPE_2D,
        }
    }
    /// Vulkan image extent of this kind.
    pub fn image_extent(self, desc: &TextureDesc) -> vk::Extent3D {
        match self {
            Self::D1 | Self::D1Array => vk::Extent3D {
                width: desc.width,
                height: 1,
                depth: 1,
            },
            Self::D3 => desc.extent(),
            _ => vk::Extent3D {
                width: desc.width,
                height: desc.height,
                depth: 1,
            },
        }
    }
    pub fn is_cube(self) -> bool {
        matches!(self, Self::Cube | Self::CubeArray)
    }
    /// Only 1D arrays, 2D arrays and cube arrays may have several elements.
    pub fn is_array(self) -> bool {
        (0x4au32 >> self as u32) & 1 != 0
    }
    pub fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::D1,
            1 => Self::D1Array,
            2 => Self::D2,
            3 => Self::D2Array,
            4 => Self::D3,
            5 => Self::Cube,
            6 => Self::CubeArray,
            _ => return None,
        })
    }
}
