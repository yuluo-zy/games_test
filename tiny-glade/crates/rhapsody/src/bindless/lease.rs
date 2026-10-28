//! Leases that keep descriptor slots alive while the GPU may read them.
use ash::vk;
use crate::Result;
use crate::resource::buffer::Buffer;
use crate::resource::sampler::Sampler;
use crate::resource::texture::TextureView;
use crate::tracking::GpuUse;
use std::sync::Arc;
use super::BindlessHeap;
use super::kind::BindlessIndex;

pub(crate) enum Owner {
    Buffer {
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
        parameters: bool,
    },
    View {
        view: Arc<TextureView>,
        layout: vk::ImageLayout,
    },
    Sampler(Arc<Sampler>),
}

impl GpuUse for Owner {
    fn visit_uses(&self, visit: &mut crate::tracking::UseVisitor<'_>) -> Result<()> {
        match self {
            Self::Buffer { buffer, .. } => buffer.visit_uses(visit),
            Self::View { view, .. } => view.visit_uses(visit),
            Self::Sampler(owner) => owner.visit_uses(visit),
        }
    }
}

/// Keep this Arc in every command that can dynamically access its index.
/// Copying the numeric shader index does not extend the lease lifetime.
pub struct BindlessLease {
    // Drop the resource before returning its descriptor slot to the page.
    pub(crate) owner: Owner,
    pub(super) slot: LeaseSlot,
}

pub(super) struct LeaseSlot {
    pub(super) heap: Arc<BindlessHeap>,
    pub(super) index: BindlessIndex,
}

impl BindlessLease {
    pub fn index(&self) -> BindlessIndex {
        self.slot.index
    }
}

impl GpuUse for BindlessLease {
    fn visit_uses(&self, visit: &mut crate::tracking::UseVisitor<'_>) -> Result<()> {
        self.owner.visit_uses(visit)?;
        self.slot.heap.visit_uses(visit)
    }
    fn held_until_completion(&self) -> bool {
        true
    }
}

/// Every command that uses a lease retains its Arc until the command completes
/// (`Doomed::Owners`), so the last drop proves the GPU no longer reads the slot.
impl Drop for LeaseSlot {
    fn drop(&mut self) {
        self.heap.slots.lock().release(self.index);
    }
}
