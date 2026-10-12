//! 描述符写入与管线绑定的可运行候选接口；原反射/managed 解析尚未全量核对。
use crate::raw::device::VulkanDevice;
use ash::vk;
use std::sync::Arc;

/// 延长资源宿主对象的生命期；实际提交链还需持有此对象直至 GPU 完成。
pub type ResourceOwner = Arc<dyn Send + Sync>;

#[derive(Clone)]
pub enum DescriptorSetBinding {
    Buffer {
        ty: vk::DescriptorType,
        info: vk::DescriptorBufferInfo,
        owner: ResourceOwner,
    },
    Image {
        ty: vk::DescriptorType,
        info: vk::DescriptorImageInfo,
        owners: Vec<ResourceOwner>,
    },
    TexelBuffer {
        ty: vk::DescriptorType,
        view: vk::BufferView,
        owner: ResourceOwner,
    },
}

impl DescriptorSetBinding {
    pub fn descriptor_type(&self) -> vk::DescriptorType {
        match self {
            Self::Buffer { ty, .. } | Self::Image { ty, .. } | Self::TexelBuffer { ty, .. } => *ty,
        }
    }
    pub fn owners(&self) -> Vec<ResourceOwner> {
        match self {
            Self::Buffer { owner, .. } | Self::TexelBuffer { owner, .. } => vec![owner.clone()],
            Self::Image { owners, .. } => owners.clone(),
        }
    }
    /// # Safety
    /// set 与本描述符中的对象必须属于 device，binding/array_element 和布局相符，
    /// 更新时该 set 不得被 GPU 或其他宿主线程访问（除布局允许的更新情况）。
    pub unsafe fn write(
        &self,
        device: &VulkanDevice,
        set: vk::DescriptorSet,
        binding: u32,
        array_element: u32,
    ) {
        let write = vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(binding)
            .dst_array_element(array_element)
            .descriptor_type(self.descriptor_type());
        unsafe {
            match self {
                Self::Buffer { info, .. } => device
                    .device
                    .update_descriptor_sets(&[write.buffer_info(std::slice::from_ref(info))], &[]),
                Self::Image { info, .. } => device
                    .device
                    .update_descriptor_sets(&[write.image_info(std::slice::from_ref(info))], &[]),
                Self::TexelBuffer { view, .. } => device.device.update_descriptor_sets(
                    &[write.texel_buffer_view(std::slice::from_ref(view))],
                    &[],
                ),
            }
        }
    }
}

/// # Safety
/// command_buffer、pipeline 必须有效且满足 Vulkan 录制状态要求。
pub unsafe fn bind_pipeline(
    device: &VulkanDevice,
    command_buffer: vk::CommandBuffer,
    point: vk::PipelineBindPoint,
    pipeline: vk::Pipeline,
) {
    unsafe {
        device
            .device
            .cmd_bind_pipeline(command_buffer, point, pipeline)
    }
}

/// # Safety
/// descriptor sets 必须与布局兼容；调用者持有其宿主及资源至提交完成。
pub unsafe fn bind_persistent_descriptor_set(
    device: &VulkanDevice,
    command_buffer: vk::CommandBuffer,
    point: vk::PipelineBindPoint,
    layout: vk::PipelineLayout,
    first_set: u32,
    sets: &[vk::DescriptorSet],
    dynamic_offsets: &[u32],
) {
    unsafe {
        device.device.cmd_bind_descriptor_sets(
            command_buffer,
            point,
            layout,
            first_set,
            sets,
            dynamic_offsets,
        )
    }
}
