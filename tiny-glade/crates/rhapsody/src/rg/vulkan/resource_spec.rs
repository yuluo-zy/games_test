//! Physical resource descriptions and their import validation.
use ash::vk;
use crate::device::Device;
use crate::resource::buffer;
use crate::resource::texture;
use std::sync::Arc;
use crate::rg::graph;

#[derive(Clone, Copy)]
pub enum ResourceSpec {
    Buffer(buffer::BufferDesc),
    Image(texture::TextureDesc),
    ConcurrentBuffer(buffer::BufferDesc, crate::tracking::QueueFamilies),
    ConcurrentImage(texture::TextureDesc, crate::tracking::QueueFamilies),
}

impl ResourceSpec {
    pub fn sharing(self) -> crate::tracking::ResourceSharing {
        match self {
            Self::Buffer(_) | Self::Image(_) => crate::tracking::ResourceSharing::Exclusive,
            Self::ConcurrentBuffer(_, families) | Self::ConcurrentImage(_, families) => {
                crate::tracking::ResourceSharing::Concurrent(families)
            }
        }
    }
    pub fn with_sharing(self, sharing: crate::tracking::ResourceSharing) -> Self {
        match (self, sharing) {
            (
                Self::Buffer(desc) | Self::ConcurrentBuffer(desc, _),
                crate::tracking::ResourceSharing::Exclusive,
            ) => Self::Buffer(desc),
            (
                Self::Image(desc) | Self::ConcurrentImage(desc, _),
                crate::tracking::ResourceSharing::Exclusive,
            ) => Self::Image(desc),
            (
                Self::Buffer(desc) | Self::ConcurrentBuffer(desc, _),
                crate::tracking::ResourceSharing::Concurrent(families),
            ) => Self::ConcurrentBuffer(desc, families),
            (
                Self::Image(desc) | Self::ConcurrentImage(desc, _),
                crate::tracking::ResourceSharing::Concurrent(families),
            ) => Self::ConcurrentImage(desc, families),
        }
    }
    pub(crate) fn create_resource(
        self,
        device: Arc<Device>,
        usage: u32,
    ) -> crate::Result<GpuResource> {
        match self {
            Self::Buffer(mut desc) | Self::ConcurrentBuffer(mut desc, _) => {
                desc.usage |= vk::BufferUsageFlags::from_raw(usage);
                let object = match self.sharing() {
                    crate::tracking::ResourceSharing::Exclusive => {
                        buffer::Buffer::new(device, &desc)
                    }
                    crate::tracking::ResourceSharing::Concurrent(families) => {
                        buffer::Buffer::new_concurrent(device, &desc, families.as_slice())
                    }
                }?;
                Ok(GpuResource::Buffer(Arc::new(object)))
            }
            Self::Image(mut desc) | Self::ConcurrentImage(mut desc, _) => {
                desc.usage |= vk::ImageUsageFlags::from_raw(usage);
                let object = match self.sharing() {
                    crate::tracking::ResourceSharing::Exclusive => {
                        texture::Texture::new(device, &desc)
                    }
                    crate::tracking::ResourceSharing::Concurrent(families) => {
                        texture::Texture::new_concurrent(device, &desc, families.as_slice())
                    }
                }?;
                Ok(GpuResource::Image(Arc::new(object)))
            }
        }
    }

    /// 按描述符字节布局编码（缓冲：size@0 usage@8 alignment@0xc memory_location@0xe）。
    pub fn descriptor(self) -> graph::Descriptor {
        match self {
            Self::Buffer(d) | Self::ConcurrentBuffer(d, _) => {
                graph::Descriptor::Buffer(crate::rg::resource_registry::BufferDesc {
                    word_at_00: d.size,
                    word_at_08: d.usage.as_raw(),
                    // `BufferDesc`：alignment u16 @0xc、memory_location u8 @0xe。
                    bytes_at_0c_through_0e: [
                        d.alignment.to_le_bytes()[0],
                        d.alignment.to_le_bytes()[1],
                        d.memory_location.as_raw(),
                    ],
                })
            }
            Self::Image(d) | Self::ConcurrentImage(d, _) => {
                graph::Descriptor::Image(crate::rg::resource_registry::TextureDesc {
                    // `TextureDesc` 的字节布局：usage@0 flags@4 format@8 width@0xc height@0x10 depth@0x14
                    // array_elements@0x18；mip_levels@0x1c sample_count@0x1d kind@0x1e。
                    words_at_00_through_18: [
                        d.usage.as_raw(),
                        d.flags.as_raw(),
                        d.format.as_raw() as u32,
                        d.width,
                        d.height,
                        d.depth,
                        d.array_elements,
                    ],
                    bytes_at_1c_through_1e: [d.mip_levels, d.sample_count, d.kind as u8],
                })
            }
        }
    }
}

#[derive(Clone)]
pub enum GpuResource {
    Buffer(Arc<buffer::Buffer>),
    Image(Arc<texture::Texture>),
}

impl GpuResource {
    pub fn generation(&self) -> crate::sync::DeviceGeneration {
        match self {
            Self::Buffer(buffer) => buffer.device.generation(),
            Self::Image(image) => image.device.generation(),
        }
    }

    pub fn descriptor(&self) -> graph::Descriptor {
        match self {
            Self::Buffer(buffer) => ResourceSpec::Buffer(buffer.desc).descriptor(),
            Self::Image(image) => ResourceSpec::Image(image.desc).descriptor(),
        }
    }

    pub(super) fn check_device(&self, device: &Device) -> crate::Result<()> {
        crate::sync::Generationed::new((), self.generation()).into_value(device.generation())
    }
}

/// 导入覆盖完整资源：尺寸、格式、mip 等必须一致，实际 usage 可以包含额外用途。
pub(super) fn validate_import_descriptor(
    declared: graph::Descriptor,
    actual: graph::Descriptor,
) -> crate::Result<()> {
    if declared.usage() & !actual.usage() != 0 || declared.with_usage(actual.usage()) != actual {
        return Err(crate::Error::invalid_usage("导入声明与实际资源描述不匹配"));
    }
    Ok(())
}
