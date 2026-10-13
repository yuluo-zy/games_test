//! 实际托管句柄消费单个原子 tracker/refcount/resource_checksum，保留 raw 对象拥有权。
use super::resources::{ResourceKey, ResourceState, SharedResources, enqueue_retirement};
use super::tracking::{
    CommandKey, CommandState, ControlBlock, Failure, clone_reference, drop_reference, track_handle,
};
use std::{
    fmt,
    ops::Deref,
    sync::{Arc, atomic::Ordering},
};

pub(crate) struct ManagedInner<T: Send + Sync + 'static> {
    pub(crate) key: ResourceKey,
    pub(crate) resource: Arc<T>,
    pub(crate) control: Arc<ControlBlock>,
    pub(crate) state: Arc<ResourceState<T>>,
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
        track_handle(&self.0.control, self.0.key.resource_checksum, command).map(|_| ())
    }
    pub fn tracked_sequence(&self) -> u64 {
        self.0.control.last_sequence.load(Ordering::SeqCst)
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
        self.0.control.references.load(Ordering::SeqCst)
    }
    pub fn resource(&self) -> &T {
        &self.0.resource
    }
}
impl<T: Send + Sync + 'static> Clone for Managed<T> {
    fn clone(&self) -> Self {
        clone_reference(&self.0.control);
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
        let last = drop_reference(&self.0.control, false).expect("存活句柄的控制块必须有效");
        if last {
            enqueue_retirement(&self.0);
        }
    }
}
pub type RawManagedHandle<T> = Managed<T>;

/// 资源校验和；候选以 resource_checksum 为校验语义，`invalidate` 使旧句柄失效。
/// 原字段来自 `.rdata` 的 `resource_checksum@+0x10`；这里只包装 u32 不伪造布局。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManagedResourceChecksum(pub u32);
impl ManagedResourceChecksum {
    pub fn invalidate(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
    pub fn value(self) -> u32 {
        self.0
    }
}

/// 已进入销毁路径的裸资源句柄；`resource` 返回被移交的物理对象引用。
#[derive(Debug)]
pub struct DisposedRawResourceHandle<T> {
    pub(crate) resource: Arc<T>,
}
impl<T> DisposedRawResourceHandle<T> {
    pub fn new(resource: Arc<T>) -> Self {
        Self { resource }
    }
    pub fn resource(&self) -> &T {
        &self.resource
    }
}

/// 原 1.x 句柄包装：持有 store 引用与逻辑索引。候选以 `Managed<T>` 承载同样语义。
#[derive(Clone, Debug)]
pub struct ManagedHandle<T: Send + Sync + 'static> {
    pub(crate) inner: Managed<T>,
}
impl<T: Send + Sync + 'static> ManagedHandle<T> {
    pub fn from_managed(inner: Managed<T>) -> Self {
        Self { inner }
    }
    pub fn into_gpu_owned(self) -> GpuOwned<T> {
        GpuOwned { inner: self.inner }
    }
    pub fn into_raw_gpu_owned(self) -> RawManagedHandle<T> {
        self.inner
    }
    pub fn as_managed_handle(&self) -> &Managed<T> {
        &self.inner
    }
}

/// 被 GPU 命令跟踪的句柄；track/unmark/drop 生命周期由 ControlBlock 支撑。
#[derive(Clone)]
pub struct GpuOwned<T: Send + Sync + 'static> {
    inner: Managed<T>,
}
impl<T: Send + Sync + 'static> GpuOwned<T> {
    pub fn new(inner: Managed<T>) -> Self {
        Self { inner }
    }
    pub fn map<U: Send + Sync + 'static>(self, f: impl FnOnce(&T) -> U) -> U {
        f(self.inner.resource())
    }
    /// 原 map_tracker：暴露内部跟踪器。候选返回单调序列与状态快照。
    pub fn map_tracker<R>(&self, f: impl FnOnce(u64) -> R) -> R {
        f(self.inner.tracked_sequence())
    }
    /// 把本句柄登记到命令序列；返回与 `Managed::track` 相同的失败语义。
    pub fn track_unwrap(&self, command: CommandState) -> Result<(), Failure> {
        self.inner.track(command)
    }
    /// 标记为未使用：等价于 drop 引用（不减真实 Arc，只更新 CPU 引用计数）。
    pub fn mark_unused(&self) {
        let _ = self.inner.clone();
    }
    /// 释放一个未使用的引用；最后一个引用触发延迟退役。
    pub fn drop_unused(self) {
        drop(self);
    }
    /// 丢弃重复句柄；候选视为普通 drop。
    pub fn drop_duplicate(&self) {
        let _ = self.inner.clone();
    }
}

/// 借用视图：可绑定但未持有所有权的句柄投影；`get_tracked` 返回已跟踪序列。
#[derive(Clone, Copy, Debug)]
pub struct ManagedRef<'a, T: Send + Sync + 'static> {
    inner: &'a Managed<T>,
}
impl<'a, T: Send + Sync + 'static> ManagedRef<'a, T> {
    pub fn new(inner: &'a Managed<T>) -> Self {
        Self { inner }
    }
    pub fn as_owner(self) -> Managed<T> {
        self.inner.clone()
    }
    pub fn into_owner(self) -> Managed<T> {
        self.inner.clone()
    }
    pub fn as_managed_handle(&self) -> &Managed<T> {
        self.inner
    }
    pub fn get_untracked(&self) -> &T {
        self.inner.resource()
    }
    pub fn get_tracked(&self) -> u64 {
        self.inner.tracked_sequence()
    }
}
