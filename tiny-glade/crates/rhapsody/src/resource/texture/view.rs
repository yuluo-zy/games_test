//! Texture views and mip-chain iteration.
use ash::vk;
use std::sync::Arc;
use super::Texture;
use super::desc::TextureViewDesc;

pub struct TextureView {
    pub(super) retirement: Option<crate::device::RetireTicket>,
    pub(crate) last_use: Arc<crate::tracking::SyncCell>,
    pub texture: Arc<Texture>,
    pub(crate) handle: vk::ImageView,
    pub desc: TextureViewDesc,
    pub(super) _memory: crate::accounting::CpuMemoryReservation,
}

impl TextureView {
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::ImageView {
        self.handle
    }
}

impl Drop for TextureView {
    fn drop(&mut self) {
        self.texture.device.retire_reserved(
            self.last_use.snapshot(),
            crate::device::graveyard::Doomed::ImageView(self.handle),
            self.retirement.take().expect("image view retirement node"),
        );
    }
}

impl crate::tracking::GpuUse for TextureView {
    fn visit_uses(&self, visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        self.texture.visit_uses(visitor)?;

        Ok(())
    }
}

pub struct MipChainIter {
    extent: vk::Extent3D,
    remaining: u32,
}

impl MipChainIter {
    pub fn new(extent: vk::Extent3D, levels: u32) -> Self {
        Self {
            extent,
            remaining: levels,
        }
    }
}

impl Iterator for MipChainIter {
    type Item = vk::Extent3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let current = self.extent;
        self.remaining -= 1;
        self.extent = vk::Extent3D {
            width: (current.width / 2).max(1),
            height: (current.height / 2).max(1),
            depth: (current.depth / 2).max(1),
        };
        Some(current)
    }
}
