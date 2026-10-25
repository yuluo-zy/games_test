//! 七类 store 共用退役来源及销毁桶；CPU dispose 不直接代表 GPU 销毁。
//! 槽位表用 `append_only_vec::AppendOnlyVec`（原作 `ManagedResourceStore::allocate` 经 `AppendOnlyVec::push`
//! 追加 `ManagedResource<T>`，`get_maybe_tracked` 经其 `Index`/`len` 读取，PDB 注解可证）；
//! 读取（`get`/`generation_at`）因此完全无锁。空闲链表与槽位内的句柄弱引用是候选布局：
//! 原作 slot 内部布局未恢复（见 DEPENDENCY_AUDIT.md）。
use super::handle::{Managed, ManagedInner};
use super::tracking::{CommandKey, CommandState, ControlBlock, Failure, dispose_control_kind};
use crate::raw::disposal::{CommandBufferDisposalBuckets, DestroyAction};
use append_only_vec::AppendOnlyVec;
use arc_swap::ArcSwapWeak;
use parking_lot::Mutex;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Weak};
static NEXT_STORE: AtomicU64 = AtomicU64::new(1);
static NEXT_GROUP: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceKey {
    /// 安全句柄代理，不冒充原 store/inner 指针 ABI。
    pub store: u64,
    pub index: u32,
    pub resource_checksum: u32,
    pub device_id: u16,
    pub resource_tag: u8,
}
struct Slot<T: Send + Sync + 'static> {
    control: Arc<ControlBlock>,
    /// 当前占用该槽位的句柄；`ArcSwapWeak` 让读者无锁 `load`，复用槽位时 `store` 换入新句柄。
    handle: ArcSwapWeak<ManagedInner<T>>,
}

/// 资源元数据投影；`checksum` 以 resource_checksum 为校验语义。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManagedResourceMeta {
    pub key: ResourceKey,
    pub checksum: u32,
}
impl ManagedResourceMeta {
    pub fn new(key: ResourceKey, checksum: u32) -> Self {
        Self { key, checksum }
    }
    pub fn checksum(&self) -> u32 {
        self.checksum
    }
}

/// 按访问声明解析托管资源的接口；`get_maybe_tracked` 只返回已被命令跟踪的句柄。
pub trait AccessManagedResource {
    type Resource: Send + Sync + 'static;
    fn get_maybe_tracked(&self, key: ResourceKey) -> Option<Managed<Self::Resource>>;
}
type DisposeTask = Box<dyn FnOnce(&SharedResources) -> Result<(), Failure> + Send>;
struct Dropped {
    key: ResourceKey,
    control: Arc<ControlBlock>,
    task: Option<DisposeTask>,
}
impl Drop for Dropped {
    fn drop(&mut self) {
        // 未处理记录意外关闭时保留拥有权，不能把 CPU 析构当作 GPU 完成。
        if let Some(task) = self.task.take() {
            std::mem::forget(task);
        }
    }
}
#[derive(Default)]
struct LegacyReceipts {
    queue: Option<u32>,
    submitted: u64,
    completed: u64,
    commands: BTreeMap<u64, u64>,
    trackers: Vec<Weak<ControlBlock>>,
}
impl LegacyReceipts {
    fn prune(&mut self) {
        let mut required = BTreeSet::new();
        self.trackers.retain(|weak| {
            let Some(control) = weak.upgrade() else {
                return false;
            };
            required.insert(control.last_sequence.load(Ordering::SeqCst));
            true
        });
        self.commands.retain(|sequence, threshold| {
            required.contains(sequence) || *threshold > self.completed
        });
    }
}
pub(crate) struct SharedResources {
    device_id: u16,
    dropped: Mutex<Vec<Dropped>>,
    buckets: Mutex<CommandBufferDisposalBuckets>,
    /// 只接收现有 lifecycle 的连续已排出序号，不能从 timeline 猜造。
    drained_prefix: AtomicU64,
    /// 限定单域的旧候选兼容路径，新主图不得用它替代真实 queue consumer。
    legacy: Mutex<LegacyReceipts>,
}
impl SharedResources {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            device_id: u16::try_from(NEXT_GROUP.fetch_add(1, Ordering::Relaxed))
                .expect("store id 超过 u16"),
            dropped: Mutex::new(Vec::new()),
            buckets: Mutex::new(CommandBufferDisposalBuckets::default()),
            drained_prefix: AtomicU64::new(0),
            legacy: Mutex::new(LegacyReceipts::default()),
        })
    }
    fn already_drained(&self, sequence: u64) -> bool {
        if sequence == 0 || sequence <= self.drained_prefix.load(Ordering::SeqCst) {
            return true;
        }
        let legacy = self.legacy.lock();
        legacy
            .commands
            .get(&sequence)
            .is_some_and(|&threshold| threshold <= legacy.completed)
    }
    fn take_out_dropped_waste(&self) -> Result<usize, Failure> {
        let mut dropped = self.dropped.lock();
        // 缺失 begin 桶是接线缺口；先检查，保留完整退役记录，不能静默提前析构。
        for record in dropped.iter() {
            let sequence = record.control.last_sequence.load(Ordering::SeqCst);
            if !self.already_drained(sequence)
                && !self
                    .buckets
                    .lock()
                    .contains(sequence)
            {
                return Err(Failure::MissingBucket);
            }
        }
        let records = std::mem::take(&mut *dropped);
        drop(dropped);
        let count = records.len();
        for mut record in records {
            record.task.take().unwrap()(self)?;
        }
        Ok(count)
    }
    fn begin_command_bucket(&self, sequence: u64) -> Result<(), Failure> {
        self.buckets
            .lock()
            .begin_command_bucket(sequence)
    }
    fn drain_completed_sequence(&self, sequence: u64) -> Result<usize, Failure> {
        if sequence != self.drained_prefix.load(Ordering::SeqCst).wrapping_add(1) {
            return Err(Failure::DuplicateCompletion);
        }
        let actions = self
            .buckets
            .lock()
            .remove(sequence)
            .ok_or(Failure::MissingBucket)?;
        self.drained_prefix.store(sequence, Ordering::SeqCst);
        let count = actions.len();
        for action in actions {
            action();
        }
        Ok(count)
    }
    fn record_submission(
        &self,
        queue: u32,
        timeline: u64,
        commands: &[CommandKey],
    ) -> Result<(), Failure> {
        let mut state = self.legacy.lock();
        if commands.iter().any(|key| key.domain != 0) || state.queue.is_some_and(|old| old != queue)
        {
            return Err(Failure::CrossDomainMappingUnknown);
        }
        if timeline <= state.submitted {
            return Err(Failure::TimelineRegression);
        }
        if commands
            .iter()
            .map(|k| k.sequence)
            .collect::<BTreeSet<_>>()
            .len()
            != commands.len()
            || commands
                .iter()
                .any(|key| state.commands.contains_key(&key.sequence))
        {
            return Err(Failure::DuplicateBucket);
        }
        state.prune();
        for key in commands {
            if !self
                .buckets
                .lock()
                .contains(key.sequence)
            {
                self.begin_command_bucket(key.sequence)?;
            }
            state.commands.insert(key.sequence, timeline);
        }
        state.queue = Some(queue);
        state.submitted = timeline;
        Ok(())
    }
    fn complete(&self, queue: u32, timeline: u64) -> Result<(), Failure> {
        let mut state = self.legacy.lock();
        if state.queue.is_some_and(|old| old != queue) {
            return Err(Failure::CrossDomainMappingUnknown);
        }
        if timeline < state.completed {
            return Err(Failure::TimelineRegression);
        }
        state.completed = timeline;
        let keys = state
            .commands
            .iter()
            .filter_map(|(&seq, &threshold)| (threshold <= timeline).then_some(seq))
            .collect::<Vec<_>>();
        state.prune();
        drop(state);
        // 旧兼容路径按成功凭据关联排出；timeline 从未被当作命令序号。
        for sequence in keys {
            let actions = self
                .buckets
                .lock()
                .remove(sequence)
                .unwrap_or_default();
            for action in actions {
                action();
            }
        }
        Ok(())
    }
}

pub(crate) struct ResourceState<T: Send + Sync + 'static> {
    store: u64,
    tag: u8,
    slots: AppendOnlyVec<Slot<T>>,
    /// 空闲链表：只在分配/退役时短暂加锁，读路径不碰它。
    available: Mutex<Vec<u32>>,
    shared: Arc<SharedResources>,
}
impl<T: Send + Sync + 'static> ResourceState<T> {
    fn slot(&self, index: u32) -> Option<&Slot<T>> {
        // `AppendOnlyVec` 的 `Index` 越界会 panic；`len` 只计入已完整写入的元素。
        ((index as usize) < self.slots.len()).then(|| &self.slots[index as usize])
    }
}
pub(crate) fn enqueue_retirement<T: Send + Sync + 'static>(inner: &Arc<ManagedInner<T>>) {
    let key = inner.key;
    let control = inner.control.clone();
    let resource = inner.resource.clone();
    let owner = inner.state.clone();
    let task: DisposeTask = Box::new(move |shared| {
        let state = &*owner;
        if key.store != state.store || key.device_id != shared.device_id {
            return Err(Failure::DeviceMismatch);
        }
        if key.resource_tag != state.tag {
            return Err(Failure::ResourceKindMismatch);
        }
        let sequence = control.last_sequence.load(Ordering::SeqCst);
        let already_drained = shared.already_drained(sequence);
        let mut action: Option<DestroyAction> = Some(Box::new(move || drop(resource)));
        let mut immediate = None;
        dispose_control_kind(
            &control,
            key.resource_checksum,
            key.device_id,
            shared.device_id,
            key.resource_tag,
            state.tag,
            || {
                let action = action.take().expect("一次退役只转移一次 raw 拥有权");
                if already_drained {
                    immediate = Some(action);
                } else if let Err(action) = shared
                    .buckets
                    .lock()
                    .append_existing(sequence, action)
                {
                    std::mem::forget(action);
                    return Err(Failure::MissingBucket);
                }
                Ok(())
            },
        )?;
        // 原 dispose<T> 在 inner 内递增 resource_checksum，然后把该 inner 回 free-list。
        state.available.lock().push(key.index);
        if let Some(action) = immediate {
            action();
        }
        Ok(())
    });
    inner
        .shared
        .dropped
        .lock()
        .push(Dropped {
            key,
            control: inner.control.clone(),
            task: Some(task),
        });
}

pub struct ResourceManager<T: Send + Sync + 'static> {
    state: Arc<ResourceState<T>>,
}
impl<T: Send + Sync + 'static> Clone for ResourceManager<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}
impl<T: Send + Sync + 'static> Default for ResourceManager<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Send + Sync + 'static> ResourceManager<T> {
    pub fn new() -> Self {
        // 独立 typed store 也保留已知原 tag；测试用非 raw 类型使用候选 tag255。
        let tags = [
            (std::any::TypeId::of::<crate::raw::texture::Texture>(), 0),
            (std::any::TypeId::of::<crate::raw::buffer::Buffer>(), 1),
            (
                std::any::TypeId::of::<crate::raw::swapchain::Swapchain>(),
                2,
            ),
            (
                std::any::TypeId::of::<crate::raw::render_pass::RenderPass>(),
                4,
            ),
            (std::any::TypeId::of::<crate::raw::sampler::Sampler>(), 5),
            (
                std::any::TypeId::of::<crate::raw::descriptor_set::VulkanDescriptorSet>(),
                6,
            ),
            (
                std::any::TypeId::of::<crate::raw::framebuffer::Framebuffer>(),
                7,
            ),
        ];
        let tag = tags
            .into_iter()
            .find_map(|(kind, tag)| (kind == std::any::TypeId::of::<T>()).then_some(tag))
            .unwrap_or(u8::MAX);
        Self::with_shared(SharedResources::new(), tag)
    }
    fn with_shared(shared: Arc<SharedResources>, tag: u8) -> Self {
        Self {
            state: Arc::new(ResourceState {
                store: NEXT_STORE.fetch_add(1, Ordering::Relaxed),
                tag,
                slots: AppendOnlyVec::new(),
                available: Mutex::new(Vec::new()),
                shared,
            }),
        }
    }
    fn shared(&self) -> Arc<SharedResources> {
        self.state.shared.clone()
    }
    pub fn allocate(&self, resource: T) -> Managed<T> {
        self.allocate_shared(Arc::new(resource))
    }
    pub fn allocate_shared(&self, resource: Arc<T>) -> Managed<T> {
        let state = &*self.state;
        let reused = state.available.lock().pop();
        let index = reused.unwrap_or_else(|| {
            let index = state.slots.push(Slot {
                control: Arc::new(ControlBlock::new(0)),
                handle: ArcSwapWeak::new(Weak::new()),
            });
            u32::try_from(index).expect("托管资源槽位超过 u32")
        });
        let control = state.slot(index).expect("刚分配的槽位必须可见").control.clone();
        // 槽位在 free-list 上时没有任何存活句柄，复用时重置序号与引用数不会与其他线程竞争。
        control.last_sequence.store(0, Ordering::SeqCst);
        control.references.store(1, Ordering::SeqCst);
        let resource_checksum = control.checksum();
        {
            let mut legacy = state.shared.legacy.lock();
            if !legacy
                .trackers
                .iter()
                .any(|weak| weak.as_ptr() == Arc::as_ptr(&control))
            {
                legacy.trackers.push(Arc::downgrade(&control));
            }
        }
        let key = ResourceKey {
            store: state.store,
            index,
            resource_checksum,
            device_id: state.shared.device_id,
            resource_tag: state.tag,
        };
        let inner = Arc::new(ManagedInner {
            key,
            resource,
            control,
            state: self.state.clone(),
            shared: state.shared.clone(),
        });
        state.slots[index as usize]
            .handle
            .store(Arc::downgrade(&inner));
        Managed(inner)
    }
    pub fn get(&self, key: ResourceKey) -> Option<Managed<T>> {
        let state = &*self.state;
        if key.store != state.store
            || key.device_id != state.shared.device_id
            || key.resource_tag != state.tag
        {
            return None;
        }
        let slot = state.slot(key.index)?;
        // 先 upgrade 再 CAS 克隆：引用数为 0（已进入退役）或校验和已变都会失败，不复活句柄。
        let inner = slot.handle.load().upgrade()?;
        if !super::tracking::clone_if_live(&slot.control, key.resource_checksum) {
            return None;
        }
        Some(Managed(inner))
    }
    pub fn get_tracked(
        &self,
        key: ResourceKey,
        domain: u32,
        command: CommandState,
    ) -> Result<Managed<T>, Failure> {
        let resource = self.get(key).ok_or(Failure::UnknownHandle)?;
        resource.track_on(domain, command)?;
        Ok(resource)
    }
    /// 只返回已被命令跟踪（last_sequence>0）的句柄。
    pub fn get_maybe_tracked(&self, key: ResourceKey) -> Option<Managed<T>> {
        let resource = self.get(key)?;
        if resource.tracked_sequence() == 0 {
            None
        } else {
            Some(resource)
        }
    }
    pub fn begin_command_bucket(&self, sequence: u64) -> Result<(), Failure> {
        self.shared().begin_command_bucket(sequence)
    }
    pub fn drain_completed_sequence(&self, sequence: u64) -> Result<usize, Failure> {
        self.shared().drain_completed_sequence(sequence)
    }
    pub fn record_submission(
        &self,
        queue: u32,
        timeline: u64,
        commands: &[CommandKey],
    ) -> Result<(), Failure> {
        self.shared().record_submission(queue, timeline, commands)
    }
    pub fn complete(&self, queue: u32, timeline: u64) -> Result<(), Failure> {
        self.shared().complete(queue, timeline)
    }
    pub fn retained_submission_count(&self) -> usize {
        self.shared()
            .legacy
            .lock()
            .commands
            .len()
    }
    pub fn pending_disposals(&self) -> usize {
        let state = &*self.state;
        state
            .shared
            .dropped
            .lock()
            .iter()
            .filter(|record| record.key.store == state.store)
            .count()
    }
    pub fn pending_destroy(&self) -> usize {
        self.shared()
            .buckets
            .lock()
            .pending()
    }
    pub fn generation_at(&self, index: u32) -> Option<u32> {
        Some(self.state.slot(index)?.control.checksum())
    }
    pub fn take_out_dropped_waste_checked(&self) -> Result<usize, Failure> {
        self.shared().take_out_dropped_waste()
    }
    /// 返回 CPU dispose 数，resource_checksum/槽位已回收；不代表 vkDestroy 数。
    pub fn take_out_dropped_waste(&self) -> usize {
        self.take_out_dropped_waste_checked()
            .expect("退役记录的命令桶生产者必须接通")
    }
    pub fn dispose_after_wait<E>(
        &self,
        wait_idle: impl FnOnce() -> Result<(), E>,
    ) -> Result<usize, E> {
        wait_idle()?;
        let shared = self.shared();
        shared.drained_prefix.store(u64::MAX, Ordering::SeqCst);
        let count = shared
            .take_out_dropped_waste()
            .expect("空闲关闭需要有效退役记录");
        let actions = shared
            .buckets
            .lock()
            .drain_after_idle();
        let pending = actions.len();
        for action in actions {
            action();
        }
        Ok(count + pending)
    }
}
pub type ManagedResourceStore<T> = ResourceManager<T>;
pub struct ManagedResources {
    pub buffers: ResourceManager<crate::raw::buffer::Buffer>,
    pub textures: ResourceManager<crate::raw::texture::Texture>,
    pub samplers: ResourceManager<crate::raw::sampler::Sampler>,
    pub descriptor_sets: ResourceManager<crate::raw::descriptor_set::VulkanDescriptorSet>,
    pub framebuffers: ResourceManager<crate::raw::framebuffer::Framebuffer>,
    pub render_passes: ResourceManager<crate::raw::render_pass::RenderPass>,
    pub swapchains: ResourceManager<crate::raw::swapchain::Swapchain>,
    shared: Arc<SharedResources>,
    device: Option<Arc<crate::raw::device::VulkanDevice>>,
}
impl Default for ManagedResources {
    fn default() -> Self {
        let shared = SharedResources::new();
        Self {
            textures: ResourceManager::with_shared(shared.clone(), 0),
            buffers: ResourceManager::with_shared(shared.clone(), 1),
            swapchains: ResourceManager::with_shared(shared.clone(), 2),
            render_passes: ResourceManager::with_shared(shared.clone(), 4),
            samplers: ResourceManager::with_shared(shared.clone(), 5),
            descriptor_sets: ResourceManager::with_shared(shared.clone(), 6),
            framebuffers: ResourceManager::with_shared(shared.clone(), 7),
            shared,
            device: None,
        }
    }
}
impl ManagedResources {
    pub fn new() -> Self {
        Self::default()
    }
    /// 关联宿主设备；不强制要求在任一资源创建前设置。
    pub fn set_device(&mut self, device: Arc<crate::raw::device::VulkanDevice>) {
        self.device = Some(device);
    }
    /// 原 owning_device：返回宿主设备。
    pub fn owning_device(&self) -> Option<Arc<crate::raw::device::VulkanDevice>> {
        self.device.clone()
    }
    /// 按资源键退役并移交给延迟销毁桶；`get` 后 drop 触发 shared 退役登记。
    pub fn throw_out_managed_resource(&self, key: ResourceKey) -> bool {
        let dropped = match key.resource_tag {
            0 => self.textures.get(key).map(drop),
            1 => self.buffers.get(key).map(drop),
            2 => self.swapchains.get(key).map(drop),
            4 => self.render_passes.get(key).map(drop),
            5 => self.samplers.get(key).map(drop),
            6 => self.descriptor_sets.get(key).map(drop),
            7 => self.framebuffers.get(key).map(drop),
            _ => None,
        };
        dropped.is_some()
    }
    pub fn begin_command_bucket(&self, sequence: u64) -> Result<(), Failure> {
        self.shared.begin_command_bucket(sequence)
    }
    pub fn drain_completed_sequence(&self, sequence: u64) -> Result<usize, Failure> {
        self.shared.drain_completed_sequence(sequence)
    }
    pub fn record_submission(
        &self,
        queue: u32,
        timeline: u64,
        commands: &[CommandKey],
    ) -> Result<(), Failure> {
        self.shared.record_submission(queue, timeline, commands)
    }
    pub fn complete(&self, queue: u32, timeline: u64) -> Result<(), Failure> {
        self.shared.complete(queue, timeline)
    }
    pub fn take_out_dropped_waste_checked(&self) -> Result<usize, Failure> {
        self.shared.take_out_dropped_waste()
    }
    pub fn take_out_dropped_waste(&self) -> usize {
        self.take_out_dropped_waste_checked()
            .expect("共享退役记录必须匹配命令桶")
    }
    pub fn pending_disposals(&self) -> usize {
        self.shared
            .dropped
            .lock()
            .len()
    }
    pub fn pending_destroy(&self) -> usize {
        self.shared
            .buckets
            .lock()
            .pending()
    }
}

impl<T: Send + Sync + 'static> AccessManagedResource for ResourceManager<T> {
    type Resource = T;
    fn get_maybe_tracked(&self, key: ResourceKey) -> Option<Managed<Self::Resource>> {
        self.get_maybe_tracked(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn different_typed_stores_share_dropped_source_and_keep_original_tags() {
        let shared = SharedResources::new();
        let textures = ResourceManager::with_shared(shared.clone(), 0);
        let buffers = ResourceManager::with_shared(shared, 1);
        let texture = textures.allocate(7_u64);
        let buffer = buffers.allocate(8_u32);
        assert_eq!(texture.key().device_id, buffer.key().device_id);
        assert_eq!(
            (texture.key().resource_tag, buffer.key().resource_tag),
            (0, 1)
        );
        let texture_key = texture.key();
        let buffer_key = buffer.key();
        drop(texture);
        drop(buffer);
        // 一个 typed store 的 consumer 取走同一共享 Vec 中两种记录。
        assert_eq!(buffers.take_out_dropped_waste_checked(), Ok(2));
        assert_eq!(textures.generation_at(texture_key.index), Some(1));
        assert_eq!(buffers.generation_at(buffer_key.index), Some(1));
        assert_eq!(buffers.pending_disposals(), 0);
    }
}

