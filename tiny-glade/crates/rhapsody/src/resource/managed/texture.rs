//! Managed images retain actual view owners in a bounded, budgeted cache.
use crate::resource::managed::handle::Managed;
use crate::resource::managed::resources::ResourceManager;
use crate::{Error, Result, collections::cache::{BoundedCache, Inserted}};
use crate::device::Device;
use crate::resource::texture::{Texture, TextureDesc, TextureView, TextureViewDesc};
use crate::accounting::CpuMemoryReservation;
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
        let key = ViewKey::new(&desc);
        if let Some(view) = self.0.views.entries.lock().cache.get(&key).cloned() {
            return Ok(ManagedTextureView { view, texture: self.clone() });
        }
        // Native creation, budget admission and completion queries never hold
        // the shared cache lock. A racing creator publishes only one view.
        let created = Arc::new(self.0.resource.image_view(&desc)?);
        let view = loop {
            let (capacity, full) = {
                let entries = self.0.views.entries.lock();
                (entries.reserved_entries, entries.cache.len() >= entries.reserved_entries)
            };
            let growth = if full && capacity < MAX_CACHED_TEXTURE_VIEWS {
                Some(ViewEntries::storage(&self.device, (capacity * 2).clamp(8, MAX_CACHED_TEXTURE_VIEWS))?)
            } else { None };
            let completed = if full && capacity == MAX_CACHED_TEXTURE_VIEWS { Some(self.device.poll()?) } else { None };
            let mut entries = self.0.views.entries.lock();
            if let Some(view) = entries.cache.get(&key).cloned() { break view; }
            let mut old = None;
            if entries.cache.len() >= entries.reserved_entries {
                if let Some((mut cache, memory, next)) = growth {
                    if next <= entries.reserved_entries { continue; }
                    for (key, view) in entries.cache.iter_lru() {
                        cache.insert(*key, view.clone(), 1);
                    }
                    let previous = std::mem::replace(&mut entries.cache, cache);
                    let previous_memory = entries._memory.replace(memory);
                    entries.reserved_entries = next;
                    old = Some((previous, previous_memory));
                } else {
                    let Some(completed) = completed.as_ref() else { continue; };
                    let key = entries.cache.lru_key_where(|view| {
                        Arc::strong_count(view) == 1 && Arc::weak_count(view) == 0 && view.last_use.is_reached(completed)
                    }).ok_or_else(|| Error::exhausted("texture view cache has no retired entry"))?;
                    let retired = entries.cache.remove(&key);
                    // Drop the native owner after releasing the shared lock.
                    drop(entries);
                    drop(retired);
                    continue;
                }
            }
            let stored = entries.cache.insert(key, created.clone(), 1);
            drop(entries);
            drop(old);
            match stored {
                Inserted::Stored(None) => break created,
                _ => return Err(Error::invalid_usage("view cache publication exceeded its reserved slot")),
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
    fn storage(device: &Device, next: usize) -> Result<(BoundedCache<ViewKey, Arc<TextureView>>, CpuMemoryReservation, usize)> {
        let bytes = (std::mem::size_of::<ViewKey>() * 2
            + std::mem::size_of::<Arc<TextureView>>() + std::mem::size_of::<usize>() * 5 + 1)
            .saturating_mul(next).saturating_mul(4);
        let memory = device.memory.stats.reserve_cpu_bytes(bytes as u64, crate::accounting::BudgetCategory::Cache)?;
        let mut cache = BoundedCache::new(MAX_CACHED_TEXTURE_VIEWS, MAX_CACHED_TEXTURE_VIEWS);
        cache.try_reserve(next).map_err(|_| Error::OutOfHostMemory)?;
        Ok((cache, memory, next))
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
impl crate::tracking::GpuUse for ManagedTextureView {
    fn visit_uses(&self, visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        self.view.visit_uses(visitor)?;

        Ok(())
    }
}
