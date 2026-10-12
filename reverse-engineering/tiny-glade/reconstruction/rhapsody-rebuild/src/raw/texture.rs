//! Texture/ImageView 完整资源路径候选；描述符字段不声称恢复原 ABI。
use super::{
    allocator::MemoryBlock,
    device::VulkanDevice,
    error::{Error, Result},
    format::vk_format_valid_aspect_flags,
};
use ash::vk;
use std::sync::Arc;
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
        }
    }
}
pub struct Texture {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Image,
    pub desc: TextureDesc,
    allocation: MemoryBlock,
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
        let info = vk::ImageCreateInfo::default()
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
            .initial_layout(vk::ImageLayout::UNDEFINED);
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
                .bind_image_memory(handle, allocation.handle, 0)
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
            allocation,
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
        let _guard = self
            .device
            .queue_lock
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            let _ = self.device.device.device_wait_idle();
            self.device.device.destroy_image(self.handle, None);
        }
    }
}
pub struct TextureView {
    pub texture: Arc<Texture>,
    pub handle: vk::ImageView,
    pub desc: TextureViewDesc,
}
impl Drop for TextureView {
    fn drop(&mut self) {
        let _guard = self
            .texture
            .device
            .queue_lock
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            let _ = self.texture.device.device.device_wait_idle();
            self.texture
                .device
                .device
                .destroy_image_view(self.handle, None);
        }
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
