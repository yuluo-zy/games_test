//! 自动生成的公开 Vulkan 格式元数据。来源：Khronos Vulkan-Headers v1.3.281 vk.xml。
//! 生成器：scripts/generate_formats.py；不代表原作私有格式分支已对照。
use super::{FormatInfo, PlaneInfo};
use ash::vk;
pub fn registry_info(format: vk::Format) -> Option<FormatInfo> {
    Some(match format.as_raw() {
        1 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R4G4_UNORM_PACK8
        2 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R4G4B4A4_UNORM_PACK16
        3 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B4G4R4A4_UNORM_PACK16
        4 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R5G6B5_UNORM_PACK16
        5 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B5G6R5_UNORM_PACK16
        6 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R5G5B5A1_UNORM_PACK16
        7 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B5G5R5A1_UNORM_PACK16
        8 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A1R5G5B5_UNORM_PACK16
        1000470000 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A1B5G5R5_UNORM_PACK16_KHR
        1000470001 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8_UNORM_KHR
        9 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8_UNORM
        10 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8_SNORM
        11 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8_USCALED
        12 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8_SSCALED
        13 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8_UINT
        14 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8_SINT
        15 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8_SRGB
        16 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8_UNORM
        17 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8_SNORM
        18 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8_USCALED
        19 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8_SSCALED
        20 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8_UINT
        21 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8_SINT
        22 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8_SRGB
        23 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8_UNORM
        24 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8_SNORM
        25 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8_USCALED
        26 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8_SSCALED
        27 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8_UINT
        28 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8_SINT
        29 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8_SRGB
        30 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8_UNORM
        31 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8_SNORM
        32 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8_USCALED
        33 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8_SSCALED
        34 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8_UINT
        35 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8_SINT
        36 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8_SRGB
        37 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8A8_UNORM
        38 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8A8_SNORM
        39 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8A8_USCALED
        40 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8A8_SSCALED
        41 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8A8_UINT
        42 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8A8_SINT
        43 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R8G8B8A8_SRGB
        44 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8A8_UNORM
        45 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8A8_SNORM
        46 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8A8_USCALED
        47 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8A8_SSCALED
        48 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8A8_UINT
        49 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8A8_SINT
        50 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8A8_SRGB
        51 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8B8G8R8_UNORM_PACK32
        52 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8B8G8R8_SNORM_PACK32
        53 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8B8G8R8_USCALED_PACK32
        54 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8B8G8R8_SSCALED_PACK32
        55 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8B8G8R8_UINT_PACK32
        56 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8B8G8R8_SINT_PACK32
        57 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A8B8G8R8_SRGB_PACK32
        58 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2R10G10B10_UNORM_PACK32
        59 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2R10G10B10_SNORM_PACK32
        60 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2R10G10B10_USCALED_PACK32
        61 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2R10G10B10_SSCALED_PACK32
        62 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2R10G10B10_UINT_PACK32
        63 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2R10G10B10_SINT_PACK32
        64 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2B10G10R10_UNORM_PACK32
        65 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2B10G10R10_SNORM_PACK32
        66 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2B10G10R10_USCALED_PACK32
        67 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2B10G10R10_SSCALED_PACK32
        68 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2B10G10R10_UINT_PACK32
        69 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A2B10G10R10_SINT_PACK32
        70 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16_UNORM
        71 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16_SNORM
        72 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16_USCALED
        73 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16_SSCALED
        74 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16_UINT
        75 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16_SINT
        76 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16_SFLOAT
        77 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_UNORM
        78 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_SNORM
        79 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_USCALED
        80 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_SSCALED
        81 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_UINT
        82 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_SINT
        83 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_SFLOAT
        84 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16_UNORM
        85 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16_SNORM
        86 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16_USCALED
        87 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16_SSCALED
        88 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16_UINT
        89 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16_SINT
        90 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16_SFLOAT
        91 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16A16_UNORM
        92 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16A16_SNORM
        93 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16A16_USCALED
        94 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16A16_SSCALED
        95 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16A16_UINT
        96 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16A16_SINT
        97 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16B16A16_SFLOAT
        98 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32_UINT
        99 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32_SINT
        100 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32_SFLOAT
        101 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32_UINT
        102 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32_SINT
        103 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32_SFLOAT
        104 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 12,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32B32_UINT
        105 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 12,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32B32_SINT
        106 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 12,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32B32_SFLOAT
        107 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32B32A32_UINT
        108 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32B32A32_SINT
        109 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R32G32B32A32_SFLOAT
        110 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64_UINT
        111 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64_SINT
        112 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64_SFLOAT
        113 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64_UINT
        114 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64_SINT
        115 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64_SFLOAT
        116 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 24,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64B64_UINT
        117 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 24,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64B64_SINT
        118 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 24,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64B64_SFLOAT
        119 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 32,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64B64A64_UINT
        120 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 32,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64B64A64_SINT
        121 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 32,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R64G64B64A64_SFLOAT
        122 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B10G11R11_UFLOAT_PACK32
        123 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_E5B9G9R9_UFLOAT_PACK32
        124 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(2),
        }, // VK_FORMAT_D16_UNORM
        125 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(2),
        }, // VK_FORMAT_X8_D24_UNORM_PACK32
        126 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(2),
        }, // VK_FORMAT_D32_SFLOAT
        127 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 1,
            aspect: vk::ImageAspectFlags::from_raw(4),
        }, // VK_FORMAT_S8_UINT
        128 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(6),
        }, // VK_FORMAT_D16_UNORM_S8_UINT
        129 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(6),
        }, // VK_FORMAT_D24_UNORM_S8_UINT
        130 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 5,
            aspect: vk::ImageAspectFlags::from_raw(6),
        }, // VK_FORMAT_D32_SFLOAT_S8_UINT
        131 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC1_RGB_UNORM_BLOCK
        132 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC1_RGB_SRGB_BLOCK
        133 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC1_RGBA_UNORM_BLOCK
        134 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC1_RGBA_SRGB_BLOCK
        135 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC2_UNORM_BLOCK
        136 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC2_SRGB_BLOCK
        137 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC3_UNORM_BLOCK
        138 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC3_SRGB_BLOCK
        139 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC4_UNORM_BLOCK
        140 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC4_SNORM_BLOCK
        141 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC5_UNORM_BLOCK
        142 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC5_SNORM_BLOCK
        143 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC6H_UFLOAT_BLOCK
        144 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC6H_SFLOAT_BLOCK
        145 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC7_UNORM_BLOCK
        146 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_BC7_SRGB_BLOCK
        147 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK
        148 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK
        149 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK
        150 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK
        151 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK
        152 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK
        153 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_EAC_R11_UNORM_BLOCK
        154 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_EAC_R11_SNORM_BLOCK
        155 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_EAC_R11G11_UNORM_BLOCK
        156 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_EAC_R11G11_SNORM_BLOCK
        157 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_4x4_UNORM_BLOCK
        158 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_4x4_SRGB_BLOCK
        159 => FormatInfo {
            block_extent: [5, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_5x4_UNORM_BLOCK
        160 => FormatInfo {
            block_extent: [5, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_5x4_SRGB_BLOCK
        161 => FormatInfo {
            block_extent: [5, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_5x5_UNORM_BLOCK
        162 => FormatInfo {
            block_extent: [5, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_5x5_SRGB_BLOCK
        163 => FormatInfo {
            block_extent: [6, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_6x5_UNORM_BLOCK
        164 => FormatInfo {
            block_extent: [6, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_6x5_SRGB_BLOCK
        165 => FormatInfo {
            block_extent: [6, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_6x6_UNORM_BLOCK
        166 => FormatInfo {
            block_extent: [6, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_6x6_SRGB_BLOCK
        167 => FormatInfo {
            block_extent: [8, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x5_UNORM_BLOCK
        168 => FormatInfo {
            block_extent: [8, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x5_SRGB_BLOCK
        169 => FormatInfo {
            block_extent: [8, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x6_UNORM_BLOCK
        170 => FormatInfo {
            block_extent: [8, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x6_SRGB_BLOCK
        171 => FormatInfo {
            block_extent: [8, 8, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x8_UNORM_BLOCK
        172 => FormatInfo {
            block_extent: [8, 8, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x8_SRGB_BLOCK
        173 => FormatInfo {
            block_extent: [10, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x5_UNORM_BLOCK
        174 => FormatInfo {
            block_extent: [10, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x5_SRGB_BLOCK
        175 => FormatInfo {
            block_extent: [10, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x6_UNORM_BLOCK
        176 => FormatInfo {
            block_extent: [10, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x6_SRGB_BLOCK
        177 => FormatInfo {
            block_extent: [10, 8, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x8_UNORM_BLOCK
        178 => FormatInfo {
            block_extent: [10, 8, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x8_SRGB_BLOCK
        179 => FormatInfo {
            block_extent: [10, 10, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x10_UNORM_BLOCK
        180 => FormatInfo {
            block_extent: [10, 10, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x10_SRGB_BLOCK
        181 => FormatInfo {
            block_extent: [12, 10, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_12x10_UNORM_BLOCK
        182 => FormatInfo {
            block_extent: [12, 10, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_12x10_SRGB_BLOCK
        183 => FormatInfo {
            block_extent: [12, 12, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_12x12_UNORM_BLOCK
        184 => FormatInfo {
            block_extent: [12, 12, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_12x12_SRGB_BLOCK
        1000156000 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G8B8G8R8_422_UNORM
        1000156001 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B8G8R8G8_422_UNORM
        1000156002 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM
        1000156003 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G8_B8R8_2PLANE_420_UNORM
        1000156004 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM
        1000156005 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G8_B8R8_2PLANE_422_UNORM
        1000156006 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM
        1000156007 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R10X6_UNORM_PACK16
        1000156008 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R10X6G10X6_UNORM_2PACK16
        1000156009 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16
        1000156010 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
        1000156011 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
        1000156012 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
        1000156013 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
        1000156014 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
        1000156015 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
        1000156016 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
        1000156017 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R12X4_UNORM_PACK16
        1000156018 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R12X4G12X4_UNORM_2PACK16
        1000156019 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16
        1000156020 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
        1000156021 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
        1000156022 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
        1000156023 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
        1000156024 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
        1000156025 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
        1000156026 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
        1000156027 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G16B16G16R16_422_UNORM
        1000156028 => FormatInfo {
            block_extent: [2, 1, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_B16G16R16G16_422_UNORM
        1000156029 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM
        1000156030 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G16_B16R16_2PLANE_420_UNORM
        1000156031 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM
        1000156032 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G16_B16R16_2PLANE_422_UNORM
        1000156033 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM
        1000054000 => FormatInfo {
            block_extent: [8, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG
        1000054001 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG
        1000054002 => FormatInfo {
            block_extent: [8, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG
        1000054003 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG
        1000054004 => FormatInfo {
            block_extent: [8, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG
        1000054005 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG
        1000054006 => FormatInfo {
            block_extent: [8, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG
        1000054007 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 8,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG
        1000066000 => FormatInfo {
            block_extent: [4, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK
        1000066001 => FormatInfo {
            block_extent: [5, 4, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK
        1000066002 => FormatInfo {
            block_extent: [5, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK
        1000066003 => FormatInfo {
            block_extent: [6, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK
        1000066004 => FormatInfo {
            block_extent: [6, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK
        1000066005 => FormatInfo {
            block_extent: [8, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK
        1000066006 => FormatInfo {
            block_extent: [8, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK
        1000066007 => FormatInfo {
            block_extent: [8, 8, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK
        1000066008 => FormatInfo {
            block_extent: [10, 5, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK
        1000066009 => FormatInfo {
            block_extent: [10, 6, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK
        1000066010 => FormatInfo {
            block_extent: [10, 8, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK
        1000066011 => FormatInfo {
            block_extent: [10, 10, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK
        1000066012 => FormatInfo {
            block_extent: [12, 10, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK
        1000066013 => FormatInfo {
            block_extent: [12, 12, 1],
            block_bytes: 16,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK
        1000330000 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 3,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G8_B8R8_2PLANE_444_UNORM
        1000330001 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16
        1000330002 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16
        1000330003 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 6,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_G16_B16R16_2PLANE_444_UNORM
        1000340000 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A4R4G4B4_UNORM_PACK16
        1000340001 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 2,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_A4B4G4R4_UNORM_PACK16
        1000464000 => FormatInfo {
            block_extent: [1, 1, 1],
            block_bytes: 4,
            aspect: vk::ImageAspectFlags::from_raw(1),
        }, // VK_FORMAT_R16G16_S10_5_NV
        _ => return None,
    })
}
pub fn registry_planes(format: vk::Format) -> &'static [PlaneInfo] {
    match format.as_raw() {
        1000156002 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R8_UNORM,
            },
        ],
        1000156003 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R8G8_UNORM,
            },
        ],
        1000156004 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
        ],
        1000156005 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R8G8_UNORM,
            },
        ],
        1000156006 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
        ],
        1000156012 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
        ],
        1000156013 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R10X6G10X6_UNORM_2PACK16,
            },
        ],
        1000156014 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
        ],
        1000156015 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R10X6G10X6_UNORM_2PACK16,
            },
        ],
        1000156016 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
        ],
        1000156022 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
        ],
        1000156023 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R12X4G12X4_UNORM_2PACK16,
            },
        ],
        1000156024 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
        ],
        1000156025 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R12X4G12X4_UNORM_2PACK16,
            },
        ],
        1000156026 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
        ],
        1000156029 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R16_UNORM,
            },
        ],
        1000156030 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 2,
                compatible: vk::Format::R16G16_UNORM,
            },
        ],
        1000156031 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
        ],
        1000156032 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 2,
                height_divisor: 1,
                compatible: vk::Format::R16G16_UNORM,
            },
        ],
        1000156033 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
        ],
        1000330000 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8_UNORM,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R8G8_UNORM,
            },
        ],
        1000330001 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R10X6G10X6_UNORM_2PACK16,
            },
        ],
        1000330002 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4_UNORM_PACK16,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R12X4G12X4_UNORM_2PACK16,
            },
        ],
        1000330003 => &[
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16_UNORM,
            },
            PlaneInfo {
                width_divisor: 1,
                height_divisor: 1,
                compatible: vk::Format::R16G16_UNORM,
            },
        ],
        _ => &[],
    }
}
// 原构建观测数据表；源码类型名称未知，不把公共注册表冒充原表。
// 原PE SHA256：f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03
// 原表地址：0x142ccd890。
pub const OBSERVED_ASPECTS: [u32; 184] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 4, 6,
    6, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];
// 原表地址：0x142ccdb70。
pub const OBSERVED_INFO_WORD00: [u32; 147] = [
    0, 2, 4, 4, 3, 3, 4, 4, 4, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4,
    4, 4, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 3, 3, 1, 1, 1, 1,
    2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];
// 原表地址：0x142ccddbc。
pub const OBSERVED_INFO_WORD04: [u32; 147] = [
    0, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 8, 8,
    8, 8, 4, 4, 4, 8, 8, 8, 12, 12, 12, 16, 16, 16, 8, 8, 8, 16, 16, 16, 24, 24, 24, 32, 32, 32, 4,
    4, 2, 4, 4, 1, 3, 4, 8, 8, 8, 8, 8, 16, 16, 16, 16, 8, 8, 16, 16, 16, 16, 16, 16,
];
// 原表地址：0x142cce008。
pub const OBSERVED_INFO_BYTE08: [u8; 147] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];
