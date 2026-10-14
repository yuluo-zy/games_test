//! 托管采样器。构造参数显式传入，不推定原作过滤与寻址默认值。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{device::VulkanDevice, error::Result, sampler::Sampler};
use ash::vk;
use std::sync::Arc;
pub type ManagedSampler = Managed<Sampler>;
impl Managed<Sampler> {
    pub fn new_sampler(
        store: &ResourceManager<Sampler>,
        device: Arc<VulkanDevice>,
        desc: &vk::SamplerCreateInfo<'_>,
    ) -> Result<Self> {
        Ok(store.allocate(Sampler::new(device, desc)?))
    }
    pub fn raw(&self) -> vk::Sampler {
        self.handle
    }
}
