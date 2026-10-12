//! 实际托管句柄消费单个原子 tracker/refcount/generation，保留 raw 对象拥有权。
use super::resources::{ResourceKey, ResourceState, SharedResources, enqueue_retirement};
use super::tracking::{
    CommandKey, CommandState, ControlBlock, Failure, clone_reference, drop_reference, track_handle,
};
use std::{
    fmt,
    ops::Deref,
    sync::{Arc, Mutex, atomic::Ordering},
};

pub(crate) struct ManagedInner<T: Send + Sync + 'static> {
    pub(crate) key: ResourceKey,
    pub(crate) resource: Arc<T>,
    pub(crate) control: Arc<Mutex<ControlBlock>>,
    pub(crate) state: Arc<Mutex<ResourceState<T>>>,
    pub(crate) shared: Arc<SharedResources>,
}
/// Arc 保障 Rust 内存安全；CPU 句柄数量由控制块的 references 独立维护。
pub struct Managed<T: Send + Sync + 'static>(pub(crate) Arc<ManagedInner<T>>);
impl<T: Send + Sync + 'static> Managed<T> {
    pub fn key(&self) -> ResourceKey {
        self.0.key
    }
    /// 原 inner 无 domain。跨域映射未恢复，不能补造多队列 tracker。
    pub fn track_on(&self, domain: u32, command: CommandState) -> Result<(), Failure> {
        if domain != 0 {
            return Err(Failure::CrossDomainMappingUnknown);
        }
        self.track(command)
    }
    pub fn track(&self, command: CommandState) -> Result<(), Failure> {
        let control = self.0.control.lock().unwrap_or_else(|e| e.into_inner());
        track_handle(&control, self.0.key.generation, command).map(|_| ())
    }
    pub fn tracked_sequence(&self) -> u64 {
        self.0
            .control
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_sequence
            .load(Ordering::SeqCst)
    }
    /// 仅供旧候选适配。domain=0 不是恢复出的原 ABI 字段。
    pub fn tracked_commands(&self) -> Vec<CommandKey> {
        let sequence = self.tracked_sequence();
        if sequence == 0 {
            Vec::new()
        } else {
            vec![CommandKey {
                domain: 0,
                sequence,
            }]
        }
    }
    pub fn reference_count(&self) -> u32 {
        self.0
            .control
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .references
            .load(Ordering::SeqCst)
    }
    pub fn resource(&self) -> &T {
        &self.0.resource
    }
}
impl<T: Send + Sync + 'static> Clone for Managed<T> {
    fn clone(&self) -> Self {
        clone_reference(&self.0.control.lock().unwrap_or_else(|e| e.into_inner()));
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
            .field("tracked_sequence", &self.tracked_sequence())
            .finish()
    }
}
impl<T: Send + Sync + 'static> Drop for Managed<T> {
    fn drop(&mut self) {
        let last = drop_reference(
            &self.0.control.lock().unwrap_or_else(|e| e.into_inner()),
            false,
        )
        .expect("存活句柄的控制块必须有效");
        if last {
            enqueue_retirement(&self.0);
        }
    }
}
pub type RawManagedHandle<T> = Managed<T>;
