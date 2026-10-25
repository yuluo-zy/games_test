//! Canonical ECS active instance, original12B identity plus dense-index hint.
//! Original queries borrow this component immutably, while update_mesh writes
//! the hint at+4. Relaxed atomics are the safe Rust ownership adaptation; the
//! native source field's Rust type itself is not asserted from a MOV opcode.
use super::gpu_world::{GpuWorldSubsetKind,InstanceKey};
use bevy_ecs::component::Component;
use std::{ops::{Deref,DerefMut},sync::atomic::{AtomicU32,Ordering}};

#[repr(C)]
#[derive(Component,Debug)]
pub struct NaniActiveInstance {
    id:u32,
    index:AtomicU32,
    kind:GpuWorldSubsetKind,
}
impl NaniActiveInstance {
    pub fn from_key(key:InstanceKey)->Self {
        Self{id:key.id,index:AtomicU32::new(key.index),kind:key.kind}
    }
    pub fn snapshot(&self)->InstanceKey {
        InstanceKey{id:self.id,index:self.index.load(Ordering::Relaxed),kind:self.kind}
    }
    /// Existing GPU algorithms keep their safe mutable-key interface. Its
    /// resolved cache hint returns to this same owner even if a later operation
    /// panics, as original resolve writes before later bounds/type assertions.
    pub fn key_guard(&self)->NaniActiveInstanceKeyGuard<'_> {
        NaniActiveInstanceKeyGuard{owner:self,key:self.snapshot()}
    }
}
impl From<InstanceKey> for NaniActiveInstance {
    fn from(value:InstanceKey)->Self {Self::from_key(value)}
}
pub struct NaniActiveInstanceKeyGuard<'a> {owner:&'a NaniActiveInstance,key:InstanceKey}
impl Deref for NaniActiveInstanceKeyGuard<'_> {
    type Target=InstanceKey;
    fn deref(&self)->&Self::Target {&self.key}
}
impl DerefMut for NaniActiveInstanceKeyGuard<'_> {
    fn deref_mut(&mut self)->&mut Self::Target {&mut self.key}
}
impl Drop for NaniActiveInstanceKeyGuard<'_> {
    fn drop(&mut self) {self.owner.index.store(self.key.index,Ordering::Relaxed);}
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn active_owner_keeps_original_size_and_unwind_resolved_hint() {
        use core::mem::{size_of,offset_of};assert_eq!(size_of::<NaniActiveInstance>(),12);assert_eq!(offset_of!(NaniActiveInstance,index),4);assert_eq!(offset_of!(NaniActiveInstance,kind),8);
        let owner=NaniActiveInstance::from_key(InstanceKey{id:71,index:99,kind:GpuWorldSubsetKind::RoofFloor});
        let result=std::panic::catch_unwind(||{let mut key=owner.key_guard();key.index=3;panic!("later GPU bounds assertion");});
        assert!(result.is_err());assert_eq!(owner.snapshot(),InstanceKey{id:71,index:3,kind:GpuWorldSubsetKind::RoofFloor});
    }
}
