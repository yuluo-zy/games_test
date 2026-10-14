//! 持久描述符集托管。布局/池/绑定对象由 raw 描述符集保持，最后 CPU 引用释放后延迟回收。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::descriptor_set::{
    VulkanDescriptorPool, VulkanDescriptorSet, VulkanDescriptorSetLayout,
};
use ash::vk;
use std::sync::Arc;
pub type ManagedPersistentDescriptorSet = Managed<VulkanDescriptorSet>;
pub type ManagedDescriptorSet = Managed<VulkanDescriptorSet>;
impl Managed<VulkanDescriptorSet> {
    pub fn allocate_descriptor_set(
        store: &ResourceManager<VulkanDescriptorSet>,
        pool: &Arc<VulkanDescriptorPool>,
        layout: Arc<VulkanDescriptorSetLayout>,
    ) -> Result<Self, vk::Result> {
        Ok(store.allocate(pool.allocate(layout)?))
    }
    pub fn raw(&self) -> vk::DescriptorSet {
        self.handle
    }
}
