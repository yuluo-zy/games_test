//! 图声明中的类型化 shader 资源槽；正式参数绑定由 pass_bindings 验证。
use crate::rg::resource::ResourceHandle;
use ash::vk;
#[derive(Clone, Copy, Debug)]
pub enum PassResourceSlot {
    UniformBuffer {
        resource: ResourceHandle,
        offset: u64,
        size: u64,
    },
    StorageBuffer {
        resource: ResourceHandle,
        offset: u64,
        size: u64,
    },
    SampledImage {
        resource: ResourceHandle,
        layout: vk::ImageLayout,
    },
    StorageImage {
        resource: ResourceHandle,
        layout: vk::ImageLayout,
    },
}
impl PassResourceSlot {
    pub fn storage_buffer(resource: ResourceHandle, offset: u64, size: u64) -> Self {
        Self::StorageBuffer {
            resource,
            offset,
            size,
        }
    }
    pub fn sampled_image(resource: ResourceHandle, layout: vk::ImageLayout) -> Self {
        Self::SampledImage { resource, layout }
    }
    pub fn storage_image(resource: ResourceHandle, layout: vk::ImageLayout) -> Self {
        Self::StorageImage { resource, layout }
    }
}
