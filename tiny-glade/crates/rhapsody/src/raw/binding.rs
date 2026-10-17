//! 原生描述符写入、管线绑定及受控命令资源租约。
use crate::raw::device::Device;
use ash::vk;
use std::sync::Arc;

/// 实际资源所有者，由命令登记访问并持有至真实 GPU 完成。
pub type ResourceOwner = Arc<dyn crate::resource::GpuUse>;

/// Private recording leases. Graph objects never enter an opaque access path.
pub struct RetainedResource(pub(crate) Retention);
pub(crate) enum Retention {
    Object(ResourceOwner),
    Recorded(ResourceOwner),
    UploadRead {
        buffer: Arc<super::buffer::Buffer>,
        offset: u64,
        size: u64,
        scope: crate::resource::range_state::AccessScope,
    },
}

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
    pub(crate) fn owner_refs(&self) -> &[ResourceOwner] {
        match self {
            Self::Buffer { owner, .. } | Self::TexelBuffer { owner, .. } => {
                std::slice::from_ref(owner)
            }
            Self::Image { owners, .. } => owners,
        }
    }
    /// # Safety
    /// set 与本描述符中的对象必须属于 device，binding/array_element 和布局相符，
    /// 更新时该 set 不得被 GPU 或其他宿主线程访问（除布局允许的更新情况）。
    pub unsafe fn write(
        &self,
        device: &Device,
        set: vk::DescriptorSet,
        binding: u32,
        array_element: u32,
    ) -> crate::Result<()> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let write = vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(binding)
            .dst_array_element(array_element)
            .descriptor_type(self.descriptor_type());
        // 原 `lock inc [device+0xab0]`：每次 vkUpdateDescriptorSets 之后 +1。
        let count = &device.descriptor_update_count;
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
        count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
}

/// # Safety
/// command_buffer、pipeline 必须有效且满足 Vulkan 录制状态要求。
pub unsafe fn bind_pipeline(
    device: &Device,
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
    device: &Device,
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

/// # Safety
/// 与 `bind_persistent_descriptor_set` 同一底层绑定；保留原名供非持久路径调用。
pub unsafe fn bind_descriptor_set(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    point: vk::PipelineBindPoint,
    layout: vk::PipelineLayout,
    first_set: u32,
    set: vk::DescriptorSet,
    dynamic_offsets: &[u32],
) {
    unsafe {
        bind_persistent_descriptor_set(
            device,
            command_buffer,
            point,
            layout,
            first_set,
            &[set],
            dynamic_offsets,
        )
    }
}

/// 原 `Device::bind_persistent_descriptor_set`（0x140bee150）逐字形态：
/// `get_tracked` 解析句柄 → 命令缓冲状态必须是 1（recording，否则 panic）→
/// `vkCmdBindDescriptorSets(cmd=[rec+0x48], bindPoint=[pipeline+0x30], layout=[pipeline+0x20],
/// firstSet=参数, setCount=1, &set, dynamicOffsetCount=0, null)`（机器码 0x140bee187..0x140bee1c1）。
/// 即原版持久集绑定恒为 **1 个集合、无动态偏移**；上面的通用版本只是它的超集。
///
/// # Safety
/// 与 `bind_persistent_descriptor_set` 相同。
pub unsafe fn bind_persistent_descriptor_set_single(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    point: vk::PipelineBindPoint,
    layout: vk::PipelineLayout,
    first_set: u32,
    set: vk::DescriptorSet,
) {
    unsafe {
        bind_persistent_descriptor_set(
            device,
            command_buffer,
            point,
            layout,
            first_set,
            &[set],
            &[],
        )
    }
}
