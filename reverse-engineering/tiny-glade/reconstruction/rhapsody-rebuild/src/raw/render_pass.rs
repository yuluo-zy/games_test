//! 创建原生 Vulkan render pass；附件和依赖由调用者明确声明。
use super::{device::VulkanDevice, error::Result};
use ash::vk;
use std::sync::Arc;

/// 精确创建键：load/store 和布局会改变执行语义，不能仅按格式查缓存。
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttachmentKey {
    pub format: i32,
    pub samples: u32,
    pub load: i32,
    pub store: i32,
    pub stencil_load: i32,
    pub stencil_store: i32,
    pub initial_layout: i32,
    pub final_layout: i32,
    pub subpass_layout: i32,
    pub depth_stencil: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RenderPassKey {
    /// 顺序同时是附件索引；颜色在前，可选深度模板在末尾。
    pub attachments: Vec<AttachmentKey>,
}

impl RenderPassKey {
    /// 管线兼容性忽略 load/store 和首尾布局，但保留子通道中的附件顺序与类型。
    /// 当前候选只生产单个图形子通道，没有 input/resolve 附件。
    pub fn compatibility_key(&self) -> Vec<(i32, u32, bool)> {
        self.attachments
            .iter()
            .map(|a| (a.format, a.samples, a.depth_stencil))
            .collect()
    }
}
pub struct RenderPass {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::RenderPass,
}
impl RenderPass {
    pub fn from_key(device: Arc<VulkanDevice>, key: &RenderPassKey) -> Result<Self> {
        let descriptions: Vec<_> = key
            .attachments
            .iter()
            .map(|a| {
                vk::AttachmentDescription::default()
                    .format(vk::Format::from_raw(a.format))
                    .samples(vk::SampleCountFlags::from_raw(a.samples))
                    .load_op(vk::AttachmentLoadOp::from_raw(a.load))
                    .store_op(vk::AttachmentStoreOp::from_raw(a.store))
                    .stencil_load_op(vk::AttachmentLoadOp::from_raw(a.stencil_load))
                    .stencil_store_op(vk::AttachmentStoreOp::from_raw(a.stencil_store))
                    .initial_layout(vk::ImageLayout::from_raw(a.initial_layout))
                    .final_layout(vk::ImageLayout::from_raw(a.final_layout))
            })
            .collect();
        let colors: Vec<_> = key
            .attachments
            .iter()
            .enumerate()
            .filter(|(_, a)| !a.depth_stencil)
            .map(|(i, a)| vk::AttachmentReference {
                attachment: i as u32,
                layout: vk::ImageLayout::from_raw(a.subpass_layout),
            })
            .collect();
        let depth = key
            .attachments
            .iter()
            .enumerate()
            .find(|(_, a)| a.depth_stencil)
            .map(|(i, a)| vk::AttachmentReference {
                attachment: i as u32,
                layout: vk::ImageLayout::from_raw(a.subpass_layout),
            });
        let mut subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&colors);
        if let Some(ref reference) = depth {
            subpass = subpass.depth_stencil_attachment(reference);
        }
        let subpasses = [subpass];
        // 外部资源图负责进入子通道前后的资源屏障，此处不猜测原作依赖掩码。
        let info = vk::RenderPassCreateInfo::default()
            .attachments(&descriptions)
            .subpasses(&subpasses);
        Self::new(device, &info)
    }
    pub fn new(device: Arc<VulkanDevice>, info: &vk::RenderPassCreateInfo<'_>) -> Result<Self> {
        let handle = unsafe { device.device.create_render_pass(info, None) }?;
        Ok(Self { device, handle })
    }
}
impl Drop for RenderPass {
    fn drop(&mut self) {
        let _guard = self
            .device
            .queue_lock
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            let _ = self.device.device.device_wait_idle();
            self.device.device.destroy_render_pass(self.handle, None);
        }
    }
}
