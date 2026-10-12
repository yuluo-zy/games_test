//! 托管图像：保存原图像所有权，图像视图需和图像一起跟踪实际命令用途。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{
    device::VulkanDevice,
    error::Result,
    texture::{Texture, TextureDesc, TextureView, TextureViewDesc},
};
use ash::vk;
use std::sync::Arc;
pub type ManagedTexture = Managed<Texture>;
impl Managed<Texture> {
    pub fn new_texture(
        store: &ResourceManager<Texture>,
        device: Arc<VulkanDevice>,
        desc: &TextureDesc,
    ) -> Result<Self> {
        Ok(store.allocate(Texture::new(device, desc)?))
    }
    pub fn raw(&self) -> vk::Image {
        self.handle
    }
    pub fn image_view(&self, desc: &TextureViewDesc) -> Result<ManagedTextureView> {
        let view = Arc::new(self.0.resource.image_view(desc)?);
        Ok(ManagedTextureView {
            view,
            texture: self.clone(),
        })
    }
}

/// 图像视图保留托管图像句柄，避免只保留裸图像编号而丢失退役关系。
#[derive(Clone)]
pub struct ManagedTextureView {
    pub view: Arc<TextureView>,
    pub texture: ManagedTexture,
}
impl ManagedTextureView {
    pub fn raw(&self) -> vk::ImageView {
        self.view.handle
    }
    pub fn track_on(
        &self,
        domain: u32,
        command: super::tracking::CommandState,
    ) -> std::result::Result<(), super::tracking::Failure> {
        self.texture.track_on(domain, command)
    }
}
