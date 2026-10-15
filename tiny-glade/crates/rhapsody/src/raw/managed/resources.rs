//! 七类 store 共用退役来源及销毁桶；CPU dispose 不直接代表 GPU 销毁。
//! 槽位表用 `append_only_vec::AppendOnlyVec`（原作 `ManagedResourceStore::allocate` 经 `AppendOnlyVec::push`
//! 追加 `ManagedResource<T>`，`get_maybe_tracked` 经其 `Index`/`len` 读取，PDB 注解可证）；
//! 读取（`get`/`generation_at`）因此完全无锁。空闲链表与槽位内的句柄弱引用是候选布局：
//! 原作 slot 内部布局未恢复（见 DEPENDENCY_AUDIT.md）。
use super::handle::{DisposedRawResourceHandleAbi, Managed, ManagedInner, ResourceTag};
use super::tracking::{CommandKey, CommandState, ControlBlock, Failure, dispose_control_kind};
use crate::log_ring;
use crate::raw::disposal::{
    CommandBufferDisposalBuckets, DestroyAction, LOG_KIND_BUCKET_HIT, LOG_KIND_BUCKET_MISS, LOG_KIND_RETIRED,
    SequentialDisposal,
};
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
    /// 空闲链表的 next（原版把 next 存进空闲槽的 `+0x10`，与 `resource` 复用同一内存；这里单独存 `index + 1`，0 = 链尾）。
    next_free: AtomicU32,
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
/// 参数 `strict`：true = 缺桶返回 `MissingBucket`（兼容检查路径）；false = 原版语义，缺桶立即销毁。
/// 第二参数是调用方（原版：`&GraphicsDevice+0xa50`）传入的销毁桶表。
type DisposeTask =
    Box<dyn FnOnce(&SharedResources, &CommandBufferDisposalBuckets, bool) -> Result<(), Failure> + Send>;
struct Dropped {
    /// 原 16 B `DisposedRawResourceHandle` 项（`{control@0, checksum@8, device@0xc, tag@0xe}`）。
    abi: DisposedRawResourceHandleAbi,
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
    /// 销毁桶表。原版归 `GraphicsDevice+0xa50`（`take_out_dropped_waste(&ManagedResources,&GraphicsDevice)`
    /// 经设备参数取得；`ManagedResources+0xa50` 只是 u16 owning_device）。Rust 里设备持有 `Arc`，
    /// 设备内的 `ManagedResources` 共享同一表；脱离设备的独立 store（测试/rg 后端）持有私有表作为“替身设备”。
    buckets: Arc<CommandBufferDisposalBuckets>,
    /// 原 `SequentialDisposal`（`last_retired` + 乱序完成窗口）；`drained_prefix` 是其 `last_retired` 的无锁镜像。
    sequential: Mutex<SequentialDisposal>,
    drained_prefix: AtomicU64,
    /// 限定单域的旧候选兼容路径，新主图不得用它替代真实 queue consumer。
    legacy: Mutex<LegacyReceipts>,
}
impl SharedResources {
    fn new() -> Arc<Self> {
        Self::with_buckets(Arc::new(CommandBufferDisposalBuckets::default()))
    }
    fn with_buckets(buckets: Arc<CommandBufferDisposalBuckets>) -> Arc<Self> {
        Arc::new(Self {
            device_id: u16::try_from(NEXT_GROUP.fetch_add(1, Ordering::Relaxed))
                .expect("store id 超过 u16"),
            dropped: Mutex::new(Vec::new()),
            buckets,
            sequential: Mutex::new(SequentialDisposal::new()),
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
    /// `strict`：缺桶先整体拒绝（`MissingBucket`，保留全部记录）；非 strict 为原版语义：
    /// 缺桶的资源进回退桶并立即销毁（原 `dispose_cb_resource` 日志 kind 4 + `immediate_empty_bucket`）。
    fn take_out_dropped_waste(&self, strict: bool) -> Result<usize, Failure> {
        self.take_out_dropped_waste_with(&self.buckets, strict)
    }
    /// 真实签名 `(&ManagedResources, &GraphicsDevice)` 的实现体：桶表由调用方（设备）给出。
    fn take_out_dropped_waste_with(
        &self,
        buckets: &CommandBufferDisposalBuckets,
        strict: bool,
    ) -> Result<usize, Failure> {
        let mut dropped = self.dropped.lock();
        if strict {
            // 缺失 begin 桶是接线缺口；先检查，保留完整退役记录，不能静默提前析构。
            for record in dropped.iter() {
                let sequence = record.control.last_sequence.load(Ordering::SeqCst);
                if !self.already_drained(sequence) && !buckets.contains(sequence) {
                    return Err(Failure::MissingBucket);
                }
            }
        }
        let records = std::mem::take(&mut *dropped);
        drop(dropped);
        let count = records.len();
        for mut record in records {
            // 原跳表 0x142d060f0：tag 3（TextureView）是 `todo!()`；tag ≥ 8 不可达（8 为 None 哨兵）。
            if record.abi.resource_type == ResourceTag::UNIMPLEMENTED {
                todo!("ResourceType::TextureView 没有 dispose_cb_resource 单态");
            }
            record.task.take().unwrap()(self, buckets, strict)?;
        }
        Ok(count)
    }
    fn begin_command_bucket(&self, sequence: u64) -> Result<(), Failure> {
        self.buckets.begin_command_bucket(sequence)
    }
    /// 严格兼容入口：只接受 `last_retired + 1`，重复/乱序返回 `DuplicateCompletion`，缺桶返回 `MissingBucket`。
    fn drain_completed_sequence(&self, sequence: u64) -> Result<usize, Failure> {
        let sequential = self.sequential.lock();
        if sequence != sequential.last_retired().wrapping_add(1) {
            return Err(Failure::DuplicateCompletion);
        }
        if !self.buckets.contains(sequence) {
            return Err(Failure::MissingBucket);
        }
        drop(sequential);
        Ok(self.retire_in_order(sequence))
    }
    /// 原 `SequentialDisposal::dispose_in_order` 语义（0x1418c7ae0）：`seq <= last_retired` panic；
    /// 允许乱序完成，连续后依次 `remove`（缺桶 panic）并执行桶。返回本次执行的动作总数。
    fn retire_in_order(&self, sequence: u64) -> usize {
        let mut sequential = self.sequential.lock();
        let mut count = 0;
        sequential.dispose_in_order(sequence, |retired| {
            log_ring::record_event(LOG_KIND_RETIRED, retired);
            count += self.buckets.remove(retired).dispose();
        });
        self.drained_prefix
            .store(sequential.last_retired(), Ordering::SeqCst);
        count
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
            if !self.buckets.contains(key.sequence) {
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
            if let Ok(bucket) = self.buckets.try_remove(sequence) {
                bucket.dispose();
            }
        }
        Ok(())
    }
}

pub(crate) struct ResourceState<T: Send + Sync + 'static> {
    store: u64,
    tag: u8,
    slots: AppendOnlyVec<Slot<T>>,
    /// 空闲链表头：原版是 `store+0x170` 的 `AtomicPtr`，`allocate`（0x1418ae7e0 族）`lock cmpxchg` 弹出，
    /// `dispose<T>`（0x1418af090 族）`checksum++` 后 `lock cmpxchg` 压入（next 写入槽 +0x10）。
    /// 这里同样无锁 CAS；高 32 位是版本号（Rust 里不能依赖地址唯一性来避开 ABA），低 32 位是 `index + 1`（0 = 空）。
    free_head: AtomicU64,
    shared: Arc<SharedResources>,
}
impl<T: Send + Sync + 'static> ResourceState<T> {
    fn pop_free(&self) -> Option<u32> {
        let mut head = self.free_head.load(Ordering::Acquire);
        loop {
            let top = head as u32;
            if top == 0 {
                return None;
            }
            let next = u64::from(self.slots[(top - 1) as usize].next_free.load(Ordering::Relaxed));
            let new = ((head >> 32).wrapping_add(1) << 32) | next;
            match self
                .free_head
                .compare_exchange_weak(head, new, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return Some(top - 1),
                Err(current) => head = current,
            }
        }
    }
    fn push_free(&self, index: u32) {
        let slot = &self.slots[index as usize];
        let mut head = self.free_head.load(Ordering::Acquire);
        loop {
            slot.next_free.store(head as u32, Ordering::Relaxed);
            let new = ((head >> 32).wrapping_add(1) << 32) | u64::from(index + 1);
            match self
                .free_head
                .compare_exchange_weak(head, new, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return,
                Err(current) => head = current,
            }
        }
    }
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
    let task: DisposeTask = Box::new(move |shared, buckets, strict| {
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
                } else if let Err(action) = buckets.append_existing(sequence, action) {
                    if strict {
                        std::mem::forget(action);
                        return Err(Failure::MissingBucket);
                    }
                    // 原版：缺桶 → 日志 kind 4 + 回退桶 → immediate_empty_bucket。
                    log_ring::record_named("", None, LOG_KIND_BUCKET_MISS);
                    immediate = Some(action);
                } else {
                    log_ring::record_named("", Some(sequence), LOG_KIND_BUCKET_HIT);
                }
                Ok(())
            },
        )?;
        // 原 dispose<T> 在 inner 内递增 resource_checksum，然后把该 inner 回 free-list。
        state.push_free(key.index);
        if let Some(action) = immediate {
            action();
        }
        Ok(())
    });
    inner.shared.dropped.lock().push(Dropped {
        abi: DisposedRawResourceHandleAbi {
            control: Arc::as_ptr(&inner.control) as usize,
            resource_checksum: key.resource_checksum,
            device_id: key.device_id,
            resource_type: key.resource_tag,
            _pad: 0,
        },
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
        use crate::raw::{
            buffer::Buffer, descriptor_set::VulkanDescriptorSet, framebuffer::Framebuffer,
            render_pass::RenderPass, sampler::Sampler, swapchain::Swapchain, texture::Texture,
        };
        let id = std::any::TypeId::of::<T>();
        let tags = [
            (std::any::TypeId::of::<Texture>(), ResourceTag::Texture),
            (std::any::TypeId::of::<Buffer>(), ResourceTag::Buffer),
            (std::any::TypeId::of::<Swapchain>(), ResourceTag::Swapchain),
            (std::any::TypeId::of::<RenderPass>(), ResourceTag::RenderPass),
            (std::any::TypeId::of::<Sampler>(), ResourceTag::Sampler),
            (std::any::TypeId::of::<VulkanDescriptorSet>(), ResourceTag::DescriptorSet),
            (std::any::TypeId::of::<Framebuffer>(), ResourceTag::Framebuffer),
        ];
        let tag = tags
            .into_iter()
            .find_map(|(kind, tag)| (kind == id).then_some(tag as u8))
            .unwrap_or(u8::MAX);
        Self::with_shared(SharedResources::new(), tag)
    }
    fn with_shared(shared: Arc<SharedResources>, tag: u8) -> Self {
        Self {
            state: Arc::new(ResourceState {
                store: NEXT_STORE.fetch_add(1, Ordering::Relaxed),
                tag,
                slots: AppendOnlyVec::new(),
                free_head: AtomicU64::new(0),
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
        let reused = state.pop_free();
        let index = reused.unwrap_or_else(|| {
            let index = state.slots.push(Slot {
                control: Arc::new(ControlBlock::new(0)),
                next_free: AtomicU32::new(0),
                handle: ArcSwapWeak::new(Weak::new()),
            });
            u32::try_from(index).expect("托管资源槽位超过 u32")
        });
        let control = state
            .slot(index)
            .expect("刚分配的槽位必须可见")
            .control
            .clone();
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
            views: Default::default(),
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
    /// 原 `SequentialDisposal::dispose_in_order`：panic 语义，允许乱序完成。
    pub fn retire_in_order(&self, sequence: u64) -> usize {
        self.shared().retire_in_order(sequence)
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
        self.shared().legacy.lock().commands.len()
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
        self.shared().buckets.pending()
    }
    pub fn generation_at(&self, index: u32) -> Option<u32> {
        Some(self.state.slot(index)?.control.checksum())
    }
    pub fn take_out_dropped_waste_checked(&self) -> Result<usize, Failure> {
        self.shared().take_out_dropped_waste(true)
    }
    /// 返回 CPU dispose 数，resource_checksum/槽位已回收；不代表 vkDestroy 数。
    pub fn take_out_dropped_waste(&self) -> usize {
        self.shared()
            .take_out_dropped_waste(false)
            .expect("退役记录必须通过设备/类型校验")
    }
    pub fn dispose_after_wait<E>(
        &self,
        wait_idle: impl FnOnce() -> Result<(), E>,
    ) -> Result<usize, E> {
        wait_idle()?;
        let shared = self.shared();
        shared.drained_prefix.store(u64::MAX, Ordering::SeqCst);
        let count = shared
            .take_out_dropped_waste(true)
            .expect("空闲关闭需要有效退役记录");
        let actions = shared.buckets.drain_after_idle();
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
        Self::from_shared(SharedResources::new())
    }
}
impl ManagedResources {
    fn from_shared(shared: Arc<SharedResources>) -> Self {
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
    /// 设备路径：销毁桶表由 `GraphicsDevice`（原 `+0xa50`）持有，这里只共享引用。
    pub fn with_device_buckets(buckets: Arc<CommandBufferDisposalBuckets>) -> Self {
        Self::from_shared(SharedResources::with_buckets(buckets))
    }
    /// 原 `ManagedResources+0xa50`：u16 owning_device（7 个 `dispose<T>` 比较它）。
    pub fn owning_device_id(&self) -> u16 {
        self.shared.device_id
    }
    /// 原 `take_out_dropped_waste(&ManagedResources, &GraphicsDevice)`（0x1418c0a70）：桶表取自设备。
    /// 原版缺桶 → 回退桶 + `immediate_empty_bucket`；此处同样不 strict。
    pub fn take_out_dropped_waste_on(&self, device: &crate::raw::device::GraphicsDevice) -> usize {
        self.shared
            .take_out_dropped_waste_with(device.disposal(), false)
            .expect("退役记录必须通过设备/类型校验")
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
    /// 原 `SequentialDisposal::dispose_in_order`：panic 语义，允许乱序完成。
    pub fn retire_in_order(&self, sequence: u64) -> usize {
        self.shared.retire_in_order(sequence)
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
        self.shared.take_out_dropped_waste(true)
    }
    pub fn take_out_dropped_waste(&self) -> usize {
        self.shared
            .take_out_dropped_waste(false)
            .expect("共享退役记录必须通过设备/类型校验")
    }
    pub fn pending_disposals(&self) -> usize {
        self.shared.dropped.lock().len()
    }
    /// 待排水的 16 B 退役项（按入队顺序；`take_out_dropped_waste` 取走的就是这些）。
    pub fn pending_disposed_records(&self) -> Vec<DisposedRawResourceHandleAbi> {
        self.shared.dropped.lock().iter().map(|r| r.abi).collect()
    }
    pub fn pending_destroy(&self) -> usize {
        self.shared.buckets.pending()
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

    #[test]
    fn pending_records_are_original_16_byte_items_in_enqueue_order() {
        let shared = SharedResources::new();
        let device = shared.device_id;
        let textures = ResourceManager::with_shared(shared.clone(), 0);
        let samplers = ResourceManager::with_shared(shared.clone(), 5);
        let t = textures.allocate(1_u8);
        let s = samplers.allocate(2_u8);
        let (t_abi, s_abi) = (t.abi(), s.abi());
        drop(t);
        drop(s);
        let records: Vec<_> = shared.dropped.lock().iter().map(|r| r.abi).collect();
        assert_eq!(records.len(), 2);
        assert_eq!(std::mem::size_of_val(&records[0]), 0x10);
        assert_eq!((records[0].control, records[0].resource_type), (t_abi.control, 0));
        assert_eq!((records[1].control, records[1].resource_type), (s_abi.control, 5));
        assert!(records.iter().all(|r| r.device_id == device && r.resource_checksum == 0 && r._pad == 0));
        textures.take_out_dropped_waste();
    }

    /// 原跳表 0x142d060f0 的 tag 3（TextureView）= `todo!()`：没有对应的 dispose_cb_resource 单态。
    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn tag_3_retirement_hits_todo_like_the_original_jump_table() {
        let shared = SharedResources::new();
        let views = ResourceManager::with_shared(shared, 3);
        drop(views.allocate(0_u8));
        views.take_out_dropped_waste();
    }
}
