//! 托管图像：保存原图像所有权，图像视图需和图像一起跟踪实际命令用途。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{
    device::VulkanDevice,
    error::Result,
    texture::{Texture, TextureDesc, TextureView, TextureViewDesc},
};
use ash::vk;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
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
    /// 原 `ManagedTexture::view`（0x140bdc5d0）：`desc = None` 取默认视图（缓存在纹理里，命中直接返回）；
    /// 否则在纹理自带的视图缓存里按描述符查找，未命中才 `create_vk_image_view`（0x140bdcba0）并登记。
    pub fn view(&self, desc: Option<&TextureViewDesc>) -> Result<ManagedTextureView> {
        let cache = &self.0.views;
        let view = match desc {
            None => {
                if let Some(view) = cache.default_view.get() {
                    view.clone()
                } else {
                    let created = Arc::new(create_vk_image_view(
                        &self.0.resource,
                        &view_desc_impl(&self.0.resource.desc),
                    )?);
                    cache.default_view.get_or_init(|| created).clone()
                }
            }
            Some(desc) => {
                let key = ViewKey::new(desc);
                if let Some(view) = cache.views.read().get(&key) {
                    view.clone()
                } else {
                    let mut views = cache.views.write();
                    match views.get(&key) {
                        Some(view) => view.clone(),
                        None => {
                            let created = Arc::new(create_vk_image_view(&self.0.resource, desc)?);
                            views.insert(key, created.clone());
                            created
                        }
                    }
                }
            }
        };
        Ok(ManagedTextureView {
            view,
            texture: self.clone(),
        })
    }
    pub fn image_view(&self, desc: &TextureViewDesc) -> Result<ManagedTextureView> {
        self.view(Some(desc))
    }
    /// 视图缓存里已登记的视图数（默认视图不计）。
    pub fn cached_view_count(&self) -> usize {
        self.0.views.views.read().len()
    }
}

/// 纹理自带的视图缓存。第二批读码（0x140bdc5d0 / 0x140bdc2d0 / 0x1418bd030 / 0x1418bd600）结论：
/// 原版就是标准库的 `RwLock<HashMap<TextureViewDesc, 视图>>`，没有私有位图/校验和：
/// * `ManagedTexture` 内部（0xC0B）：`+0x30` 设备上下文、`+0x38` RwLock 状态字（读者 +0x10，写者位 `&8`）、
///   `+0x40` hashbrown 控制字节指针、`+0x48` `bucket_mask`、`+0x58` 元素数、`+0x60` `RandomState{k0,k1}`、
///   `+0x70` 默认视图缓存（非 0 直接返回）、`+0x78` 内存块（0x48B，`+0xbc` 即块内 `+0x44` 的 Option niche）；
/// * “校验和”= hashbrown 的 h2（哈希高 7 位 `>>0x39`，SSE2 16 字节组比较）；“位图”= 组匹配掩码
///   （`pmovmskb`）与 EMPTY(0xff) 探测；桶 0x48B 位于控制字节之下（`ctrl - 0x48*(i+1)`）：键 `TextureViewDesc`
///   （0x3c，填到 0x40）+ 值 u64（视图句柄）；
/// * 哈希（0x1418bd030）是 SipHash-1-3（`RandomState` 的两个 u64 密钥经 `pshufd/pxor` 展开成 4 个状态字，
///   逐字段 `Hasher::write`，`Option<u32>` 先写 8B 判别再在 `Some` 时写 4B 值，即 `#[derive(Hash)]`），
///   相等（0x1418bd600）是 `#[derive(PartialEq)]` 的逐字段比较；
/// * “纹理全局 id 计数器”不存在：0x140bdc2d0 里被递增的线程局部 `{inited, k0, k1}` 是 std
///   `RandomState::new()` 的每线程密钥（首次用 OS 随机 `0x140d64e00`，之后每建一个 HashMap `k0 += 1`）。
/// 这里 `HashMap<完整描述符, 视图>` 语义等价（相同描述符命中同一视图），只是哈希器不同（不影响可见行为）。
#[derive(Default)]
pub(crate) struct ViewCache {
    default_view: OnceLock<Arc<TextureView>>,
    views: RwLock<HashMap<ViewKey, Arc<TextureView>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ViewKey {
    view_type: i32,
    format: i32,
    aspect: u32,
    base_mip_level: u32,
    level_count: u32,
    base_layer: u32,
    layer_count: u32,
    usage: u32,
}
impl ViewKey {
    fn new(desc: &TextureViewDesc) -> Self {
        Self {
            view_type: desc.view_type.as_raw(),
            format: desc.format.as_raw(),
            aspect: desc.aspect.as_raw(),
            base_mip_level: desc.base_mip_level,
            level_count: desc.level_count,
            base_layer: desc.base_layer,
            layer_count: desc.layer_count,
            usage: desc.usage.as_raw(),
        }
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
