//! Destructible CPU recording values use owned leases, never a Copy bump arena.
use crate::{
    Result,
    raw::{
        binding::{ResourceOwner, RetainedResource, Retention},
        temp_list_pool::{TempListLease, TempListPool},
    },
};

pub struct ResourceOwners {
    storage: Storage,
}
enum Storage {
    Pooled(TempListLease<RetainedResource>),
    // Device-less CPU adapters keep a small fixed stack capacity. Production
    // Vulkan recording uses the separately budgeted pooled variant.
    Local(smallvec::SmallVec<[RetainedResource; 4]>),
}
impl Default for ResourceOwners {
    fn default() -> Self {
        Self::empty()
    }
}
impl ResourceOwners {
    pub(crate) fn new(pool: Option<&TempListPool<RetainedResource>>) -> Result<Self> {
        Ok(Self {
            storage: match pool {
                Some(pool) => Storage::Pooled(pool.checkout()?),
                None => Storage::Local(smallvec::SmallVec::new()),
            },
        })
    }
    pub(crate) fn empty() -> Self {
        Self {
            storage: Storage::Local(smallvec::SmallVec::new()),
        }
    }
    pub fn add_object(&mut self, owner: ResourceOwner) -> Result<()> {
        self.add_retained(RetainedResource(Retention::Object(owner)))
    }
    pub(crate) fn add_recorded(&mut self, owner: ResourceOwner) -> Result<()> {
        self.add_retained(RetainedResource(Retention::Recorded(owner)))
    }
    pub(crate) fn add_upload_read(
        &mut self,
        buffer: std::sync::Arc<crate::raw::buffer::Buffer>,
        offset: u64,
        size: u64,
        scope: crate::resource::range_state::AccessScope,
    ) -> Result<()> {
        self.add_retained(RetainedResource(Retention::UploadRead {
            buffer,
            offset,
            size,
            scope,
        }))
    }
    fn add_retained(&mut self, owner: RetainedResource) -> Result<()> {
        match &mut self.storage {
            Storage::Pooled(list) => list.add(owner),
            Storage::Local(items) => {
                if items.len() == items.inline_size() {
                    return Err(crate::Error::Timeout);
                }
                items.push(owner);
                Ok(())
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        match &self.storage {
            Storage::Pooled(list) => list.is_empty(),
            Storage::Local(items) => items.is_empty(),
        }
    }
    pub(crate) fn drain(&mut self) -> OwnerDrain<'_> {
        match &mut self.storage {
            Storage::Pooled(list) => OwnerDrain::Pooled(list.drain()),
            Storage::Local(items) => OwnerDrain::Local(items.drain(..)),
        }
    }
}

pub(crate) enum OwnerDrain<'a> {
    Pooled(std::vec::Drain<'a, RetainedResource>),
    Local(smallvec::Drain<'a, [RetainedResource; 4]>),
}
impl Iterator for OwnerDrain<'_> {
    type Item = RetainedResource;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Pooled(items) => items.next(),
            Self::Local(items) => items.next(),
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Pooled(items) => items.size_hint(),
            Self::Local(items) => items.size_hint(),
        }
    }
}
impl ExactSizeIterator for OwnerDrain<'_> {}
