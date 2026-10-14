//! Texture/ImageView 完整资源路径候选；描述符字段不声称恢复原 ABI。
use super::{
    allocator::MemoryBlock,
    device::VulkanDevice,
    error::{Error, Result},
    format::vk_format_valid_aspect_flags,
};
use ash::vk;
use std::sync::Arc;

/// 1D 尺寸的全 mip 数量：32 - leading_zeros(size)。
pub fn mip_count_1d(size: u32) -> u32 {
    32 - size.leading_zeros()
}

/// 纹理采样数候选枚举；`to_vk` 映射到 Vulkan `SampleCountFlags`。
/// 枚举名来自 PDB，具体排序与语义未逐证明，保留显式映射。
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

/// 线性初始数据的逐层迭代器；每次 `next` 返回一层的数据切片。
/// 原 `LinearInitialDataIter` 从连续内存按 mip 偏移切分；候选按分配步进。
pub struct LinearInitialDataIter<'a> {
    data: &'a [u8],
    step: usize,
}
impl<'a> LinearInitialDataIter<'a> {
    pub fn new(data: &'a [u8], step: usize) -> Self {
        Self {
            data,
            step: step.max(1),
        }
    }
}
impl<'a> Iterator for LinearInitialDataIter<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        let take = self.step.min(self.data.len());
        let (head, rest) = self.data.split_at(take);
        self.data = rest;
        Some(head)
    }
}

/// 构建 `ImageCreateInfo`；`Texture::new` 内联的同构造被提取为可复用入口。
pub fn get_image_create_info(desc: &TextureDesc) -> vk::ImageCreateInfo<'_> {
    vk::ImageCreateInfo::default()
        .flags(desc.flags)
        .image_type(desc.image_type)
        .format(desc.format)
        .extent(desc.extent)
        .mip_levels(desc.mip_levels)
        .array_layers(desc.array_layers)
        .samples(desc.samples)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(desc.usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
}

#[derive(Clone, Copy, Debug)]
pub struct TextureDesc {
    pub extent: vk::Extent3D,
    pub format: vk::Format,
    pub usage: vk::ImageUsageFlags,
    pub image_type: vk::ImageType,
    pub flags: vk::ImageCreateFlags,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: vk::SampleCountFlags,
}
impl TextureDesc {
    pub fn new_2d(width: u32, height: u32, format: vk::Format) -> Self {
        Self {
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            format,
            usage: vk::ImageUsageFlags::SAMPLED,
            image_type: vk::ImageType::TYPE_2D,
            flags: vk::ImageCreateFlags::empty(),
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
        }
    }
    /// 全字段构造；默认 2D 单 mip 单层采样纹理，再由调用方链式设置用途。
    pub fn new(extent: vk::Extent3D, format: vk::Format) -> Self {
        Self {
            extent,
            format,
            usage: vk::ImageUsageFlags::SAMPLED,
            image_type: if extent.depth > 1 {
                vk::ImageType::TYPE_3D
            } else {
                vk::ImageType::TYPE_2D
            },
            flags: vk::ImageCreateFlags::empty(),
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
        }
    }
    /// 原 `array_elements`：数组层数（候选字段名 `array_layers`）。
    pub fn array_elements(&self) -> u32 {
        self.array_layers
    }
    pub fn sample_count(&self) -> vk::SampleCountFlags {
        self.samples
    }
    /// 整除 extent 的三分量；除数 0 保留原值（防御）。
    pub fn div_up_extent(&self, divisor: u32) -> vk::Extent3D {
        vk::Extent3D {
            width: if divisor == 0 {
                self.extent.width
            } else {
                self.extent.width.div_ceil(divisor)
            },
            height: if divisor == 0 {
                self.extent.height
            } else {
                self.extent.height.div_ceil(divisor)
            },
            depth: if divisor == 0 {
                self.extent.depth
            } else {
                self.extent.depth.div_ceil(divisor)
            },
        }
    }
    pub fn div_extent(&self, divisor: u32) -> vk::Extent3D {
        vk::Extent3D {
            width: self
                .extent
                .width
                .checked_div(divisor)
                .unwrap_or(self.extent.width),
            height: self
                .extent
                .height
                .checked_div(divisor)
                .unwrap_or(self.extent.height),
            depth: self
                .extent
                .depth
                .checked_div(divisor)
                .unwrap_or(self.extent.depth),
        }
    }
    /// 全 mip 链长度；`all_mip_levels` 的只读投影。
    pub fn full_mip_chain_length(&self) -> u32 {
        32 - self
            .extent
            .width
            .max(self.extent.height)
            .max(self.extent.depth)
            .leading_zeros()
    }
    /// 是否需要 `MUTABLE_FORMAT` 才能以不同格式采样：仅纯色格式（aspect 为 COLOR）
    /// 且已有用途含 `SAMPLED` 时才允许，且必须显式声明可变格式标志。
    pub fn sampled_with_different_format(&self) -> bool {
        self.usage.contains(vk::ImageUsageFlags::SAMPLED)
            && vk_format_valid_aspect_flags(self.format) == vk::ImageAspectFlags::COLOR
    }
    pub fn new_2d_array(width: u32, height: u32, layers: u32, format: vk::Format) -> Self {
        Self {
            array_layers: layers,
            ..Self::new_2d(width, height, format)
        }
    }
    pub fn new_3d(width: u32, height: u32, depth: u32, format: vk::Format) -> Self {
        Self {
            extent: vk::Extent3D {
                width,
                height,
                depth,
            },
            image_type: vk::ImageType::TYPE_3D,
            ..Self::new_2d(width, height, format)
        }
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.usage = usage;
        self
    }
    pub fn sampled(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::SAMPLED;
        self
    }
    pub fn storage(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::STORAGE;
        self
    }
    pub fn transfer_dst(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        self
    }
    pub fn color_attachment(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::COLOR_ATTACHMENT;
        self
    }
    pub fn depth_stencil_attachment(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;
        self
    }
    pub fn all_mip_levels(mut self) -> Self {
        self.mip_levels = 32
            - self
                .extent
                .width
                .max(self.extent.height)
                .max(self.extent.depth)
                .leading_zeros();
        self
    }
    pub fn extent_2d(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.extent.width,
            height: self.extent.height,
        }
    }
    pub fn half_res(mut self) -> Self {
        self.extent.width = self.extent.width.div_ceil(2);
        self.extent.height = self.extent.height.div_ceil(2);
        self
    }
}
#[derive(Clone, Copy, Debug)]
pub struct TextureViewDesc {
    pub view_type: vk::ImageViewType,
    pub format: vk::Format,
    pub aspect: vk::ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_layer: u32,
    pub layer_count: u32,
    pub usage: vk::ImageUsageFlags,
}
impl TextureViewDesc {
    pub fn new(texture: &TextureDesc) -> Self {
        Self {
            view_type: if texture.image_type == vk::ImageType::TYPE_3D {
                vk::ImageViewType::TYPE_3D
            } else if texture.array_layers > 1 {
                vk::ImageViewType::TYPE_2D_ARRAY
            } else {
                vk::ImageViewType::TYPE_2D
            },
            format: texture.format,
            aspect: vk_format_valid_aspect_flags(texture.format),
            base_mip_level: 0,
            level_count: texture.mip_levels,
            base_layer: 0,
            layer_count: texture.array_layers,
            usage: texture.usage,
        }
    }
    /// 是否逐 mip 单独绑定：视图只覆盖单个 mip 层。
    pub fn bind_individual_mips(&self) -> bool {
        self.level_count == 1
    }
    pub fn usage(&self) -> vk::ImageUsageFlags {
        self.usage
    }
}
pub struct Texture {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Image,
    pub desc: TextureDesc,
    allocation: std::mem::ManuallyDrop<MemoryBlock>,
}
impl Texture {
    pub fn new(device: Arc<VulkanDevice>, desc: &TextureDesc) -> Result<Self> {
        if desc.extent.width == 0
            || desc.extent.height == 0
            || desc.extent.depth == 0
            || desc.array_layers == 0
            || desc.mip_levels == 0
        {
            return Err(Error::Invalid("纹理范围和层数必须非零"));
        }
        let info = get_image_create_info(desc);
        let handle = unsafe { device.device.create_image(&info, None) }?;
        let requirements = unsafe { device.device.get_image_memory_requirements(handle) };
        let allocation = match MemoryBlock::allocate(
            Arc::clone(&device),
            requirements,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ) {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.device.destroy_image(handle, None);
                }
                return Err(error);
            }
        };
        if let Err(error) = unsafe {
            device
                .device
                .bind_image_memory(handle, allocation.handle, allocation.offset)
        } {
            unsafe {
                device.device.destroy_image(handle, None);
            }
            return Err(error.into());
        }
        Ok(Self {
            device,
            handle,
            desc: *desc,
            allocation: std::mem::ManuallyDrop::new(allocation),
        })
    }
    pub fn image_view(self: &Arc<Self>, desc: &TextureViewDesc) -> Result<TextureView> {
        if desc
            .base_mip_level
            .checked_add(desc.level_count)
            .is_none_or(|n| n > self.desc.mip_levels)
            || desc
                .base_layer
                .checked_add(desc.layer_count)
                .is_none_or(|n| n > self.desc.array_layers)
            || desc.level_count == 0
            || desc.layer_count == 0
        {
            return Err(Error::Invalid("图片视图子资源范围无效"));
        }
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(desc.aspect)
            .base_mip_level(desc.base_mip_level)
            .level_count(desc.level_count)
            .base_array_layer(desc.base_layer)
            .layer_count(desc.layer_count);
        let info = vk::ImageViewCreateInfo::default()
            .image(self.handle)
            .view_type(desc.view_type)
            .format(desc.format)
            .subresource_range(range);
        let handle = unsafe { self.device.device.create_image_view(&info, None) }?;
        Ok(TextureView {
            texture: Arc::clone(self),
            handle,
            desc: *desc,
        })
    }
    pub fn allocation_bytes(&self) -> u64 {
        self.allocation.size
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        // SAFETY: allocation 只在此处取出一次，之后不会再被访问。
        let memory = unsafe { std::mem::ManuallyDrop::take(&mut self.allocation) }.into_deferred();
        self.device
            .bury(super::graveyard::Doomed::Image(self.handle, memory));
    }
}
pub struct TextureView {
    pub texture: Arc<Texture>,
    pub handle: vk::ImageView,
    pub desc: TextureViewDesc,
}
impl Drop for TextureView {
    fn drop(&mut self) {
        self.texture
            .device
            .bury(super::graveyard::Doomed::ImageView(self.handle));
    }
}
pub struct MipChainIter {
    extent: vk::Extent3D,
    remaining: u32,
}
impl MipChainIter {
    pub fn new(extent: vk::Extent3D, levels: u32) -> Self {
        Self {
            extent,
            remaining: levels,
        }
    }
}
impl Iterator for MipChainIter {
    type Item = vk::Extent3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let current = self.extent;
        self.remaining -= 1;
        self.extent = vk::Extent3D {
            width: (current.width / 2).max(1),
            height: (current.height / 2).max(1),
            depth: (current.depth / 2).max(1),
        };
        Some(current)
    }
}
