//! 原 built-in sampler 的策略未恢复；此接口接受明确 Vulkan 描述符。
use super::{device::VulkanDevice, error::Result};
use ash::vk;
use std::sync::Arc;
pub struct Sampler {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Sampler,
}
impl Sampler {
    pub fn new(device: Arc<VulkanDevice>, info: &vk::SamplerCreateInfo<'_>) -> Result<Self> {
        let handle = unsafe { device.device.create_sampler(info, None) }?;
        Ok(Self { device, handle })
    }
}
impl Drop for Sampler {
    fn drop(&mut self) {
        self.device
            .bury(super::graveyard::Doomed::Sampler(self.handle));
    }
}
