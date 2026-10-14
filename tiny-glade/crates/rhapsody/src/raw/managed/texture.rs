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

/// 图像类型 → 视图类型（2D 数组/3D 的候选映射）。
pub fn convert_image_type_to_view_type(
    image_type: vk::ImageType,
    array_layers: u32,
) -> vk::ImageViewType {
    match image_type {
        vk::ImageType::TYPE_3D => vk::ImageViewType::TYPE_3D,
        vk::ImageType::TYPE_1D if array_layers > 1 => vk::ImageViewType::TYPE_1D_ARRAY,
        vk::ImageType::TYPE_1D => vk::ImageViewType::TYPE_1D,
        vk::ImageType::TYPE_2D if array_layers > 1 => vk::ImageViewType::TYPE_2D_ARRAY,
        _ => vk::ImageViewType::TYPE_2D,
    }
}

/// 构造默认视图描述符（基 mip=0、全层、全 mip）。
pub fn view_desc_impl(texture: &TextureDesc) -> TextureViewDesc {
    TextureViewDesc::new(texture)
}

/// 原 managed::create_vk_image_view：创建真实图像视图并保留纹理句柄。
pub fn create_vk_image_view(texture: &Arc<Texture>, desc: &TextureViewDesc) -> Result<TextureView> {
    texture.image_view(desc)
}
