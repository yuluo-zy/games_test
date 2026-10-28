//! Shared CPU resource ownership backed by the renderer's typed generational store.
use crate::resource::managed::resources::{ManagedKey, ResourceState};
use crate::tracking::{GpuUse, UseVisitor};
use std::{fmt, ops::Deref, sync::Arc};

pub(crate) struct ManagedInner<T: Send + Sync + 'static> {
    pub(crate) key: ManagedKey<T>,
    pub(crate) resource: Arc<T>,
    pub(crate) state: Arc<ResourceState<T>>,
    pub(crate) views: crate::resource::managed::texture::ViewCache,
    pub(crate) _memory: crate::accounting::CpuMemoryReservation,
}
impl<T: Send + Sync + 'static> Drop for ManagedInner<T> {
    fn drop(&mut self) {
        // Removing the weak slot invalidates its generation. Release the store
        // lock before dropping fields: a raw resource may release its Device.
        let removed = self.state.remove(self.key);
        drop(removed);
    }
}

/// Owning CPU reference. Commands retain the actual resource and register its
/// SyncCell; this handle has no second completion or reference-count protocol.
pub struct Managed<T: Send + Sync + 'static>(pub(crate) Arc<ManagedInner<T>>);
impl<T: Send + Sync + 'static> Managed<T> {
    pub fn key(&self) -> ManagedKey<T> {
        self.0.key
    }
    pub fn resource(&self) -> &T {
        &self.0.resource
    }
    pub fn shared(&self) -> Arc<T> {
        self.0.resource.clone()
    }
}
impl<T: Send + Sync + 'static> Clone for Managed<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Send + Sync + 'static> Deref for Managed<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.resource()
    }
}
impl<T: Send + Sync + 'static> PartialEq for Managed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}
impl<T: Send + Sync + 'static> Eq for Managed<T> {}
impl<T: Send + Sync + 'static> fmt::Debug for Managed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Managed")
            .field("key", &self.key())
            .finish_non_exhaustive()
    }
}

impl<T: GpuUse + Send + Sync + 'static> GpuUse for Managed<T> {
    fn visit_uses(&self, visitor: &mut UseVisitor<'_>) -> crate::Result<()> {
        self.resource().visit_uses(visitor)
    }
}
