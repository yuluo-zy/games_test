//! Managed images retain actual view owners in a bounded, budgeted cache.
use super::{handle::Managed, resources::ResourceManager};
use crate::{
    Error, Result,
    raw::{
        allocator::CpuMemoryReservation,
        device::Device,
        texture::{Texture, TextureDesc, TextureView, TextureViewDesc},
    },
    rg::cache::{BoundedCache, Inserted},
};
use ash::vk;
use parking_lot::Mutex;
use std::sync::Arc;

pub const MAX_CACHED_TEXTURE_VIEWS: usize = 256;
pub type ManagedTexture = Managed<Texture>;
impl Managed<Texture> {
    pub fn new_texture(
        store: &ResourceManager<Texture>,
        device: Arc<Device>,
        desc: &TextureDesc,
    ) -> Result<Self> {
        store.allocate(Texture::new(device, desc)?)
    }
    pub fn raw(&self) -> vk::Image {
        self.handle
    }
    pub fn view(&self, desc: Option<&TextureViewDesc>) -> Result<ManagedTextureView> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let desc = match desc {
            Some(desc) => *desc,
            None => TextureViewDesc::new(&self.desc)?,
        };
        desc.validate(&self.desc)?;
        let key = ViewKey::new(&desc);
        let mut entries = self.0.views.entries.lock();
        let view = match entries.cache.get(&key) {
            Some(view) => view.clone(),
            None => {
                entries.reserve_slot(&self.device)?;
                let view = Arc::new(self.0.resource.image_view(&desc)?);
                match entries.cache.insert(key, view.clone(), 1) {
                    Inserted::Stored(None) => view,
                    _ => {
                        return Err(Error::invalid_usage(
                            "view cache publication exceeded its reserved slot",
                        ));
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
    /// Includes the canonical default view; explicit and default requests with
    /// the same descriptor share the same native object.
    pub fn cached_view_count(&self) -> usize {
        self.0.views.entries.lock().cache.len()
    }
}
#[derive(Default)]
pub(crate) struct ViewCache {
    entries: Mutex<ViewEntries>,
}
struct ViewEntries {
    // Free actual backing before crediting its host reservation.
    cache: BoundedCache<ViewKey, Arc<TextureView>>,
    reserved_entries: usize,
    _memory: Option<CpuMemoryReservation>,
}
impl Default for ViewEntries {
    fn default() -> Self {
        Self {
            cache: BoundedCache::new(MAX_CACHED_TEXTURE_VIEWS, MAX_CACHED_TEXTURE_VIEWS),
            reserved_entries: 0,
            _memory: None,
        }
    }
}
impl ViewEntries {
    fn reserve_slot(&mut self, device: &Device) -> Result<()> {
        if self.cache.len() < self.reserved_entries {
            return Ok(());
        }
        if self.reserved_entries < MAX_CACHED_TEXTURE_VIEWS {
            let next = (self.reserved_entries * 2).clamp(8, MAX_CACHED_TEXTURE_VIEWS);
            // Conservative backing allowance includes hash buckets/control
            // storage, Entry metadata and the LRU key deque. Both old and new
            // storage remain charged during growth; rejected growth publishes
            // nothing and leaves the original cache unchanged.
            let bytes = (std::mem::size_of::<ViewKey>() * 2
                + std::mem::size_of::<Arc<TextureView>>()
                + std::mem::size_of::<usize>() * 2
                + 1)
            .saturating_mul(next)
            .saturating_mul(4);
            let memory = device
                .memory
                .stats
                .reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Cache)?;
            let mut cache = BoundedCache::new(MAX_CACHED_TEXTURE_VIEWS, MAX_CACHED_TEXTURE_VIEWS);
            cache
                .try_reserve(next)
                .map_err(|_| Error::OutOfHostMemory)?;
            for (key, view) in self.cache.iter_lru() {
                if !matches!(cache.insert(*key, view.clone(), 1), Inserted::Stored(None)) {
                    return Err(Error::invalid_usage(
                        "view cache migration exceeded its fixed limit",
                    ));
                }
            }
            let old_cache = std::mem::replace(&mut self.cache, cache);
            let old_memory = self._memory.replace(memory);
            self.reserved_entries = next;
            drop(old_cache);
            drop(old_memory);
            return Ok(());
        }
        let completed = device.poll()?;
        let key = self
            .cache
            .lru_key_where(|view| {
                Arc::strong_count(view) == 1
                    && Arc::weak_count(view) == 0
                    && view.last_use.is_reached(&completed)
            })
            .ok_or(Error::Timeout)?;
        drop(self.cache.remove(&key));
        Ok(())
    }
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
#[derive(Clone)]
pub struct ManagedTextureView {
    pub view: Arc<TextureView>,
    pub texture: ManagedTexture,
}
impl ManagedTextureView {
    pub fn raw(&self) -> vk::ImageView {
        self.view.handle
    }
}
impl crate::resource::GpuUse for ManagedTextureView {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        self.view.visit_uses(visitor)?;

        Ok(())
    }
}
