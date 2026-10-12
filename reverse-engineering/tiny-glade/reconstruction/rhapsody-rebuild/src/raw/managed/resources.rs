//! 七类 store 共用退役来源及销毁桶；CPU dispose 不直接代表 GPU 销毁。
//! Arc/Vec 是安全候选布局，原 slab/Treiber 栈布局仍需另行还原。
use super::handle::{Managed, ManagedInner};
use super::tracking::{CommandKey, CommandState, ControlBlock, Failure, dispose_control_kind};
use crate::raw::disposal::{CommandBufferDisposalBuckets, DestroyAction};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
static NEXT_STORE: AtomicU64 = AtomicU64::new(1);
static NEXT_GROUP: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceKey {
    /// 安全句柄代理，不冒充原 store/inner 指针 ABI。
    pub store: u64,
    pub index: u32,
    pub generation: u32,
    pub store_id: u16,
    pub resource_tag: u8,
}
struct Slot<T: Send + Sync + 'static> {
    control: Arc<Mutex<ControlBlock>>,
    handle: Weak<ManagedInner<T>>,
}
type DisposeTask = Box<dyn FnOnce(&SharedResources) -> Result<(), Failure> + Send>;
struct Dropped {
    key: ResourceKey,
    control: Arc<Mutex<ControlBlock>>,
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
    trackers: Vec<Weak<Mutex<ControlBlock>>>,
}
impl LegacyReceipts {
    fn prune(&mut self) {
        let mut required = BTreeSet::new();
        self.trackers.retain(|weak| {
            let Some(control) = weak.upgrade() else {
                return false;
            };
            required.insert(
                control
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .last_sequence
                    .load(Ordering::SeqCst),
            );
            true
        });
        self.commands.retain(|sequence, threshold| {
            required.contains(sequence) || *threshold > self.completed
        });
    }
}
pub(crate) struct SharedResources {
    store_id: u16,
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
            store_id: u16::try_from(NEXT_GROUP.fetch_add(1, Ordering::Relaxed))
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
        let legacy = self.legacy.lock().unwrap_or_else(|e| e.into_inner());
        legacy
            .commands
            .get(&sequence)
            .is_some_and(|&threshold| threshold <= legacy.completed)
    }
    fn take_out_dropped_waste(&self) -> Result<usize, Failure> {
        let mut dropped = self.dropped.lock().unwrap_or_else(|e| e.into_inner());
        // 缺失 begin 桶是接线缺口；先检查，保留完整退役记录，不能静默提前析构。
        for record in dropped.iter() {
            let sequence = record
                .control
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .last_sequence
                .load(Ordering::SeqCst);
            if !self.already_drained(sequence)
                && !self
                    .buckets
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
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
            .unwrap_or_else(|e| e.into_inner())
            .begin_command_bucket(sequence)
    }
    fn drain_completed_sequence(&self, sequence: u64) -> Result<usize, Failure> {
        if sequence != self.drained_prefix.load(Ordering::SeqCst).wrapping_add(1) {
            return Err(Failure::DuplicateCompletion);
        }
        let actions = self
            .buckets
            .lock()
            .unwrap_or_else(|e| e.into_inner())
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
        let mut state = self.legacy.lock().unwrap_or_else(|e| e.into_inner());
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
                .unwrap_or_else(|e| e.into_inner())
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
        let mut state = self.legacy.lock().unwrap_or_else(|e| e.into_inner());
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
                .unwrap_or_else(|e| e.into_inner())
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
    slots: Vec<Slot<T>>,
    available: Vec<u32>,
    shared: Arc<SharedResources>,
}
pub(crate) fn enqueue_retirement<T: Send + Sync + 'static>(inner: &Arc<ManagedInner<T>>) {
    let key = inner.key;
    let control = inner.control.clone();
    let resource = inner.resource.clone();
    let owner = inner.state.clone();
    let task: DisposeTask = Box::new(move |shared| {
        let mut state = owner.lock().unwrap_or_else(|e| e.into_inner());
        if key.store != state.store || key.store_id != shared.store_id {
            return Err(Failure::DeviceMismatch);
        }
        if key.resource_tag != state.tag {
            return Err(Failure::ResourceKindMismatch);
        }
        let sequence = control
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_sequence
            .load(Ordering::SeqCst);
        let already_drained = shared.already_drained(sequence);
        let mut control = control.lock().unwrap_or_else(|e| e.into_inner());
        let mut action: Option<DestroyAction> = Some(Box::new(move || drop(resource)));
        let mut immediate = None;
        dispose_control_kind(
            &mut control,
            key.generation,
            key.store_id,
            shared.store_id,
            key.resource_tag,
            state.tag,
            || {
                let action = action.take().expect("一次退役只转移一次 raw 拥有权");
                if already_drained {
                    immediate = Some(action);
                } else if let Err(action) = shared
                    .buckets
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .append_existing(sequence, action)
                {
                    std::mem::forget(action);
                    return Err(Failure::MissingBucket);
                }
                Ok(())
            },
        )?;
        // 原 dispose<T> 在 inner 内递增 generation，然后把该 inner 回 free-list。
        state.available.push(key.index);
        drop(control);
        drop(state);
        if let Some(action) = immediate {
            action();
        }
        Ok(())
    });
    inner
        .shared
        .dropped
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(Dropped {
            key,
            control: inner.control.clone(),
            task: Some(task),
        });
}

pub struct ResourceManager<T: Send + Sync + 'static> {
    state: Arc<Mutex<ResourceState<T>>>,
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
            state: Arc::new(Mutex::new(ResourceState {
                store: NEXT_STORE.fetch_add(1, Ordering::Relaxed),
                tag,
                slots: Vec::new(),
                available: Vec::new(),
                shared,
            })),
        }
    }
    fn shared(&self) -> Arc<SharedResources> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .shared
            .clone()
    }
    pub fn allocate(&self, resource: T) -> Managed<T> {
        self.allocate_shared(Arc::new(resource))
    }
    pub fn allocate_shared(&self, resource: Arc<T>) -> Managed<T> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let index = state.available.pop().unwrap_or_else(|| {
            let index = u32::try_from(state.slots.len()).expect("托管资源槽位超过 u32");
            state.slots.push(Slot {
                control: Arc::new(Mutex::new(ControlBlock {
                    last_sequence: AtomicU64::new(0),
                    references: AtomicU32::new(1),
                    generation: 0,
                })),
                handle: Weak::new(),
            });
            index
        });
        let control = state.slots[index as usize].control.clone();
        let generation = {
            let control = control.lock().unwrap_or_else(|e| e.into_inner());
            control.last_sequence.store(0, Ordering::SeqCst);
            control.references.store(1, Ordering::SeqCst);
            control.generation
        };
        {
            let mut legacy = state
                .shared
                .legacy
                .lock()
                .unwrap_or_else(|e| e.into_inner());
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
            generation,
            store_id: state.shared.store_id,
            resource_tag: state.tag,
        };
        let inner = Arc::new(ManagedInner {
            key,
            resource,
            control,
            state: self.state.clone(),
            shared: state.shared.clone(),
        });
        state.slots[index as usize].handle = Arc::downgrade(&inner);
        Managed(inner)
    }
    pub fn get(&self, key: ResourceKey) -> Option<Managed<T>> {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if key.store != state.store
            || key.store_id != state.shared.store_id
            || key.resource_tag != state.tag
        {
            return None;
        }
        let slot = state.slots.get(key.index as usize)?;
        let control = slot.control.lock().unwrap_or_else(|e| e.into_inner());
        if control.generation != key.generation || control.references.load(Ordering::SeqCst) == 0 {
            return None;
        }
        let inner = slot.handle.upgrade()?;
        super::tracking::clone_reference(&control);
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
            .unwrap_or_else(|e| e.into_inner())
            .commands
            .len()
    }
    pub fn pending_disposals(&self) -> usize {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state
            .shared
            .dropped
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|record| record.key.store == state.store)
            .count()
    }
    pub fn pending_destroy(&self) -> usize {
        self.shared()
            .buckets
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pending()
    }
    pub fn generation_at(&self, index: u32) -> Option<u32> {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        Some(
            state
                .slots
                .get(index as usize)?
                .control
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .generation,
        )
    }
    pub fn take_out_dropped_waste_checked(&self) -> Result<usize, Failure> {
        self.shared().take_out_dropped_waste()
    }
    /// 返回 CPU dispose 数，generation/槽位已回收；不代表 vkDestroy 数。
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
            .unwrap_or_else(|e| e.into_inner())
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
        }
    }
}
impl ManagedResources {
    pub fn new() -> Self {
        Self::default()
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
            .unwrap_or_else(|e| e.into_inner())
            .len()
    }
    pub fn pending_destroy(&self) -> usize {
        self.shared
            .buckets
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pending()
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
        assert_eq!(texture.key().store_id, buffer.key().store_id);
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
