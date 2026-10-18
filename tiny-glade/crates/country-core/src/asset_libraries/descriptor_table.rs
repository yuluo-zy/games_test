//! 纹理库的描述符所有权。分配、Vulkan 写入和 GPU 延迟回收全部复用 rhapsody。
use crate::rhapsody_types::Texture;
use ash::vk;
use parking_lot::Mutex;
use rhapsody::{
    bindless::{BindlessHeap, BindlessLease},
    raw::texture::TextureView,
};
use std::sync::Arc;

type ViewResolver = dyn Fn(&Texture) -> rhapsody::Result<Arc<TextureView>> + Send + Sync;

/// 保存有效租约；纹理替换后释放库内旧引用，在途 GPU 命令继续持有自己的租约。
#[derive(Clone, Default)]
pub struct DescriptorSet {
    native: Option<Arc<NativeTable>>,
}
struct NativeTable {
    heap: Arc<BindlessHeap>,
    sampled_capacity: u32,
    resolve_view: Arc<ViewResolver>,
    entries: Mutex<Vec<Option<Entry>>>,
}
struct Entry {
    view: Arc<TextureView>,
    lease: Arc<BindlessLease>,
}
impl std::fmt::Debug for DescriptorSet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DescriptorSet")
            .field("connected", &self.native.is_some())
            .finish()
    }
}
impl DescriptorSet {
    pub fn from_heap(
        heap: Arc<BindlessHeap>,
        sampled_capacity: u32,
        resolve_view: impl Fn(&Texture) -> rhapsody::Result<Arc<TextureView>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            native: Some(Arc::new(NativeTable {
                heap,
                sampled_capacity,
                resolve_view: Arc::new(resolve_view),
                entries: Mutex::new(Vec::new()),
            })),
        }
    }
    pub fn sampled_capacity(&self) -> u32 {
        self.native
            .as_ref()
            .map_or(u32::MAX, |table| table.sampled_capacity)
    }
    pub fn heap(&self) -> rhapsody::Result<&Arc<BindlessHeap>> {
        self.native
            .as_ref()
            .map(|table| &table.heap)
            .ok_or_else(|| rhapsody::Error::unsupported("描述符表未连接 GPU"))
    }
    /// 真实后端直接返回实际槽位，不能再用游戏计数器猜测 GPU 分配结果。
    /// None 表示记录型设备，由上层维护可重试的延迟提交队列。
    pub fn allocate_texture(&self, texture: &Texture) -> rhapsody::Result<Option<u32>> {
        let Some(native) = &self.native else {
            return Ok(None);
        };
        let view = (native.resolve_view)(texture)?;
        let lease = native
            .heap
            .sampled_image(view.clone(), vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)?;
        let index = lease.index().index;
        let mut entries = native.entries.lock();
        let required_length = entries.len().max(index as usize + 1);
        entries.resize_with(required_length, || None);
        entries[index as usize] = Some(Entry { view, lease });
        Ok(Some(index))
    }
    /// 渲染命令必须保留租约并按 rhapsody 协议登记 GPU 使用，不能只复制数值索引。
    pub fn lease(&self, index: u32) -> Option<Arc<BindlessLease>> {
        self.native
            .as_ref()?
            .entries
            .lock()
            .get(index as usize)?
            .as_ref()
            .map(|entry| entry.lease.clone())
    }

    /// 只快照实际已分配的视图与槽位，供帧参数系统保留资源。
    /// 帧宿主可以在自己的堆中分配租约，但不能为未请求的纹理臆造槽位。
    pub fn sampled_views(&self) -> rhapsody::Result<Vec<(u32, Arc<TextureView>)>> {
        let native = self
            .native
            .as_ref()
            .ok_or_else(|| rhapsody::Error::unsupported("描述符表未连接 GPU"))?;
        Ok(native
            .entries
            .lock()
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| {
                entry
                    .as_ref()
                    .map(|entry| (index as u32, entry.view.clone()))
            })
            .collect())
    }
    /// 只释放资产库持有的引用；实际槽位等待所有已登记 GPU 使用结束才回收。
    pub fn retire(&self, index: u32) {
        if let Some(native) = &self.native
            && let Some(entry) = native.entries.lock().get_mut(index as usize)
        {
            *entry = None;
        }
    }
    pub fn validate_sampled_image(
        &self,
        index: u32,
        view: &Arc<TextureView>,
    ) -> rhapsody::Result<()> {
        let native = self
            .native
            .as_ref()
            .ok_or_else(|| rhapsody::Error::unsupported("描述符表未连接 GPU"))?;
        let entries = native.entries.lock();
        match entries.get(index as usize).and_then(Option::as_ref) {
            Some(entry) if Arc::ptr_eq(&entry.view, view) => Ok(()),
            _ => Err(rhapsody::Error::invalid_usage("描述符槽位与视图不匹配")),
        }
    }
}
