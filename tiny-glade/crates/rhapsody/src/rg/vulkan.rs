//! 将候选渲染图主链接到真实 Vulkan；显式访问表避免猜测原访问码的阶段和布局。
//! 创建描述符为候选适配，原闭包/反射/缓存策略仍需整链对照。
use super::{
    executor::{
        ExecutionBackend, ExecutionChain, GraphBatchSync, GraphExecution, GraphTimelineSignal,
        GraphTimelineWait,
    },
    graph,
    resource::ResourceOrigin,
};
use crate::raw::{
    buffer,
    command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
    device::VulkanDevice,
    managed::{lifecycle, resources::ManagedResources},
    semaphore::TimelineSemaphore,
    submit, texture,
};
use ash::vk::{self, Handle};
use parking_lot::RwLock;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

/// 后端导入 token 与跨帧 temporal token 共用的物理资源标识空间。
///
/// 两边曾各自从 1 起分配，会撞号：`bind_import_named` 会因“token 已属于其他物理资源”拒绝导入。
/// 现在统一用一个全进程单调计数器；外部指定的 token（`bind_import_named`）通过 [`reserve_token`]
/// 把计数器推过它，之后分配的 token 一定更大。0 与 `u64::MAX` 保留为无效。
static NEXT_TOKEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
fn try_allocate_token() -> Option<u64> {
    use std::sync::atomic::Ordering;
    NEXT_TOKEN
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |t| {
            (t < u64::MAX - 1).then_some(t + 1)
        })
        .ok()
}
fn allocate_token() -> u64 {
    try_allocate_token().expect("token 空间耗尽")
}
fn reserve_token(token: u64) {
    NEXT_TOKEN.fetch_max(token.saturating_add(1), std::sync::atomic::Ordering::Relaxed);
}

#[derive(Clone, Copy, Debug)]
pub struct AccessSpec {
    pub original_mask: u32,
    pub write: bool,
    pub stages: vk::PipelineStageFlags,
    pub access: vk::AccessFlags,
    pub layout: vk::ImageLayout,
}
#[derive(Clone)]
pub struct AccessTable(pub BTreeMap<u8, AccessSpec>);
impl Default for AccessTable {
    fn default() -> Self {
        Self(BTreeMap::from([(
            0,
            AccessSpec {
                original_mask: 0,
                write: false,
                stages: vk::PipelineStageFlags::TOP_OF_PIPE,
                access: vk::AccessFlags::empty(),
                layout: vk::ImageLayout::UNDEFINED,
            },
        )]))
    }
}
impl AccessTable {
    /// 原版全部 64 个访问码：`(stage, access, layout)` 取自 EXE 的表（`raw::access_table`），
    /// 写分类为 `0x28..=0x37`，`original_mask` 即 access 掩码（资源用法推导的输入）。
    pub fn original() -> Self {
        use crate::raw::access_table::{ACCESS_TABLE, is_write};
        Self(
            ACCESS_TABLE
                .iter()
                .enumerate()
                .map(|(code, info)| {
                    (
                        code as u8,
                        AccessSpec {
                            original_mask: info.access,
                            write: is_write(code as u8),
                            stages: info.stages(),
                            access: info.access_flags(),
                            layout: info.image_layout(),
                        },
                    )
                })
                .collect(),
        )
    }
}
impl graph::AccessDependency for AccessTable {
    fn access_mask(&mut self, code: u8) -> u32 {
        self.0[&code].original_mask
    }
    fn is_write(&mut self, code: u8) -> bool {
        self.0[&code].write
    }
}
#[derive(Clone, Copy)]
pub enum ResourceSpec {
    Buffer(buffer::BufferDesc),
    Image(texture::TextureDesc),
}
impl ResourceSpec {
    /// 按原描述字节布局编码（缓冲：size@0 usage@8 alignment@0xc memory_location@0xe）。
    pub fn descriptor(self) -> graph::Descriptor {
        match self {
            Self::Buffer(d) => graph::Descriptor::Buffer(graph::BufferDesc {
                word_at_00: d.size,
                word_at_08: d.usage.as_raw(),
                // 原 `BufferDesc`：alignment u16 @0xc、memory_location u8 @0xe。
                bytes_at_0c_through_0e: [
                    d.alignment.to_le_bytes()[0],
                    d.alignment.to_le_bytes()[1],
                    d.memory_location.as_raw(),
                ],
            }),
            Self::Image(d) => graph::Descriptor::Image(graph::TextureDesc {
                // 原 `TextureDesc` 的字节布局：usage@0 flags@4 format@8 width@0xc height@0x10 depth@0x14
                // array_elements@0x18；mip_levels@0x1c sample_count@0x1d kind@0x1e。
                words_at_00_through_18: [
                    d.usage.as_raw(),
                    d.flags.as_raw(),
                    d.format.as_raw() as u32,
                    d.width,
                    d.height,
                    d.depth,
                    d.array_elements,
                ],
                bytes_at_1c_through_1e: [d.mip_levels, d.sample_count, d.kind as u8],
            }),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum RawResource {
    Buffer(vk::Buffer),
    Image(vk::Image),
}
/// 回调在资源创建后解析命名资源，实际拥有者由后端保留到 GPU 完成。
#[derive(Clone, Default)]
pub struct ResourceResolver(Arc<RwLock<BTreeMap<String, RawResource>>>);
impl ResourceResolver {
    pub fn get(&self, name: &str) -> Option<RawResource> {
        self.0.read().get(name).copied()
    }
    pub fn buffer(&self, name: &str) -> Option<vk::Buffer> {
        match self.get(name) {
            Some(RawResource::Buffer(buffer)) => Some(buffer),
            _ => None,
        }
    }
    pub fn image(&self, name: &str) -> Option<vk::Image> {
        match self.get(name) {
            Some(RawResource::Image(image)) => Some(image),
            _ => None,
        }
    }
}
#[derive(Clone)]
pub enum GpuResource {
    Buffer(Arc<buffer::Buffer>),
    Image(Arc<texture::Texture>),
}
pub struct VulkanGraphBackend {
    pub device: Arc<VulkanDevice>,
    pub timeline: Arc<TimelineSemaphore>,
    pub resources: BTreeMap<u64, GpuResource>,
    pub accesses: AccessTable,
    pub resolver: ResourceResolver,
    pub submissions: Vec<GpuSubmissionCapture>,
    specifications: BTreeMap<String, ResourceSpec>,
    pool: Arc<VulkanCommandPool>,
    commands: BTreeMap<u64, VulkanCommandBuffer>,
    managed: BTreeMap<u64, lifecycle::ResourceHandle>,
    queue: lifecycle::Queue,
    family: u32,
    queue_timelines: Vec<Arc<TimelineSemaphore>>,
    import_families: BTreeMap<u64, u32>,
    synchronization: BTreeMap<usize, OwnedBatchSync>,
    graph_waits: Vec<TimelineWait>,
    graph_signals: Vec<TimelineSignal>,
    frame_borrows: bool,
    frame_handles: Vec<lifecycle::ResourceHandle>,
    recycled_commands: Vec<u64>,
    /// 每块一个命令池（Vulkan 要求录制命令的命令池外部同步）。槽 0 用主池，槽 k≥1 用 `chunk_pools[k-1]`。
    chunk_pools: Vec<Arc<VulkanCommandPool>>,
    /// 命令句柄所属的池槽，回收后仍回到同一个池。
    command_slot: BTreeMap<u64, usize>,
    pub stores: ManagedResources,
    pass_resources: Option<Arc<super::pass_bindings::ManagedPassResources>>,
    managed_prefix: u64,
    barriers: BTreeMap<u64, crate::raw::barrier::SynchronizationScope>,
    top_level_pool: Option<vk::QueryPool>,
    top_level_started: bool,
    /// 失败路径经 `release_created_resource` 移出的托管句柄（基础引用），由会话在生命周期里释放。
    released_managed: Vec<lifecycle::ResourceHandle>,
}
/// `submissions` 捕获的上限：只保留最近的提交，避免不经 `begin_frame` 的长时间运行无界增长。
pub const SUBMISSION_CAPTURE_LIMIT: usize = 64;
pub struct GpuSubmissionCapture {
    pub queue: vk::Queue,
    pub batches: Vec<submit::PreparedSubmit>,
    pub result: i32,
}
pub type TimelineWait = (Arc<TimelineSemaphore>, u64, vk::PipelineStageFlags);
pub type TimelineSignal = (Arc<TimelineSemaphore>, u64);
#[derive(Default)]
struct OwnedBatchSync {
    waits: Vec<TimelineWait>,
    signals: Vec<TimelineSignal>,
}
/// 把一次访问转换加入屏障作用域；串行 `transition` 与并行块共用，保证两条路径发出的屏障一致。
fn add_transition(
    scope: &mut crate::raw::barrier::SynchronizationScope,
    accesses: &AccessTable,
    resources: &BTreeMap<u64, GpuResource>,
    token: u64,
    from: u8,
    to: u8,
) -> bool {
    let Some(source) = accesses.0.get(&from) else {
        return false;
    };
    let Some(target) = accesses.0.get(&to) else {
        return false;
    };
    scope.source_stages |= source.stages;
    scope.destination_stages |= target.stages;
    match resources.get(&token) {
        Some(GpuResource::Buffer(buffer)) => {
            let barrier = vk::BufferMemoryBarrier::default()
                .src_access_mask(source.access)
                .dst_access_mask(target.access)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(buffer.handle)
                .offset(0)
                .size(vk::WHOLE_SIZE);
            scope.buffers.push(barrier);
        }
        Some(GpuResource::Image(image)) => {
            let range = vk::ImageSubresourceRange::default()
                .aspect_mask(crate::raw::format::vk_format_valid_aspect_flags(
                    image.desc.format,
                ))
                .level_count(u32::from(image.desc.mip_levels))
                .layer_count(image.desc.array_layers());
            let barrier = vk::ImageMemoryBarrier::default()
                .src_access_mask(source.access)
                .dst_access_mask(target.access)
                .old_layout(source.layout)
                .new_layout(target.layout)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(image.handle)
                .subresource_range(range);
            scope.images.push(barrier);
        }
        None => return false,
    }
    true
}
/// 并行录制时各块共享的只读视图：只读访问表与资源表，直接把每个 pass 的屏障写入块自己的命令缓冲。
struct VulkanChunkView<'a> {
    device: &'a ash::Device,
    accesses: &'a AccessTable,
    resources: &'a BTreeMap<u64, GpuResource>,
}
impl super::executor::ChunkRecorder for VulkanChunkView<'_> {
    fn record_transitions(
        &self,
        command: u64,
        transitions: &[super::executor::ChunkTransition],
    ) -> bool {
        if transitions.is_empty() {
            return true;
        }
        let mut scope = crate::raw::barrier::SynchronizationScope::default();
        for t in transitions {
            if !add_transition(
                &mut scope,
                self.accesses,
                self.resources,
                t.token,
                t.from,
                t.to,
            ) {
                return false;
            }
        }
        // 与串行路径相同：每个 pass 一次屏障命令。
        unsafe {
            self.device.cmd_pipeline_barrier(
                vk::CommandBuffer::from_raw(command),
                scope.source_stages,
                scope.destination_stages,
                vk::DependencyFlags::empty(),
                &scope.memory,
                &scope.buffers,
                &scope.images,
            );
        }
        true
    }
}
impl VulkanGraphBackend {
    pub fn new(device: Arc<VulkanDevice>, accesses: AccessTable) -> Result<Self, vk::Result> {
        Self::new_for_queue(device, accesses, lifecycle::Queue::Primary)
    }
    pub fn new_for_queue(
        device: Arc<VulkanDevice>,
        accesses: AccessTable,
        queue: lifecycle::Queue,
    ) -> Result<Self, vk::Result> {
        let (slot, family) = match queue {
            lifecycle::Queue::Primary => (0, device.queue_family),
            lifecycle::Queue::Auxiliary(index) => {
                let selected = device
                    .additional_queues
                    .get(index as usize)
                    .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
                (index as usize + 1, selected.family)
            }
        };
        let queue_timelines = (0..=device.additional_queues.len())
            .map(|_| TimelineSemaphore::new(device.clone(), 0))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            timeline: queue_timelines[slot].clone(),
            pool: VulkanCommandPool::new_for_family(device.clone(), family)?,
            device,
            accesses,
            resolver: ResourceResolver::default(),
            submissions: vec![],
            resources: BTreeMap::new(),
            specifications: BTreeMap::new(),
            commands: BTreeMap::new(),
            managed: BTreeMap::new(),
            queue,
            family,
            queue_timelines,
            import_families: BTreeMap::new(),
            synchronization: BTreeMap::new(),
            graph_waits: vec![],
            graph_signals: vec![],
            frame_borrows: false,
            frame_handles: vec![],
            recycled_commands: vec![],
            chunk_pools: vec![],
            command_slot: BTreeMap::new(),
            stores: ManagedResources::new(),
            pass_resources: None,
            managed_prefix: 0,
            barriers: BTreeMap::new(),
            top_level_pool: None,
            top_level_started: false,
            released_managed: vec![],
        })
    }
    /// 取第 `slot` 个命令池的命令：优先复用该池已完成的命令，否则新分配。
    fn command_for_slot(&mut self, slot: usize) -> Result<u64, lifecycle::Failure> {
        let position = self
            .recycled_commands
            .iter()
            .rposition(|handle| self.command_slot.get(handle).copied().unwrap_or(0) == slot);
        if let Some(position) = position {
            let handle = self.recycled_commands.remove(position);
            // recycled_commands 只在对应时间线 wait 完成后入池（见 wait/wait_frame），无需整设备 idle。
            unsafe {
                self.commands
                    .get_mut(&handle)
                    .ok_or(lifecycle::Failure::InvalidPendingState)?
                    .reset_after_completion()
            }
            .map_err(|_| lifecycle::Failure::BackendReset)?;
            return Ok(handle);
        }
        let pool = if slot == 0 {
            self.pool.clone()
        } else {
            while self.chunk_pools.len() < slot {
                let pool = VulkanCommandPool::new_for_family(self.device.clone(), self.family)
                    .map_err(|_| lifecycle::Failure::InvalidPendingState)?;
                self.chunk_pools.push(pool);
            }
            self.chunk_pools[slot - 1].clone()
        };
        let command = pool
            .allocate()
            .map_err(|_| lifecycle::Failure::InvalidPendingState)?;
        let handle = command.handle.as_raw();
        self.commands.insert(handle, command);
        self.command_slot.insert(handle, slot);
        Ok(handle)
    }
    pub fn selected_queue(&self) -> lifecycle::Queue {
        self.queue
    }
    pub fn selected_family(&self) -> u32 {
        self.family
    }
    /// 惰性创建顶层时间戳查询池（两条查询：begin/end）。
    fn top_level_pool_handle(&self) -> Result<vk::QueryPool, vk::Result> {
        let device = &self.device;
        let properties = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        };
        if properties.limits.timestamp_period == 0.0 {
            return Err(vk::Result::ERROR_FEATURE_NOT_PRESENT);
        }
        let info = vk::QueryPoolCreateInfo::default()
            .query_type(vk::QueryType::TIMESTAMP)
            .query_count(2);
        unsafe { device.device.create_query_pool(&info, None) }
    }
    /// 原 GraphExecutor::begin_top_level_timestamp_query：把 TOP_OF_PIPE 时间戳写入
    /// 帧内第一条命令。返回 0 表示已写入；重复 begin 返回错误。
    pub fn begin_top_level_timestamp_query(
        &mut self,
        command: vk::CommandBuffer,
    ) -> Result<(), vk::Result> {
        if self.top_level_started {
            return Err(vk::Result::ERROR_OUT_OF_POOL_MEMORY);
        }
        if self.top_level_pool.is_none() {
            self.top_level_pool = Some(self.top_level_pool_handle()?);
        }
        let pool = self.top_level_pool.unwrap();
        unsafe {
            self.device.device.cmd_write_timestamp(
                command,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                pool,
                0,
            );
        }
        self.top_level_started = true;
        Ok(())
    }
    /// 原 GraphExecutor::end_top_level_timestamp_query：写入 BOTTOM_OF_PIPE 时间戳并
    /// 返回可选帧时长；读回失败/未 begin 时返回 Err。
    pub fn end_top_level_timestamp_query(
        &mut self,
        command: vk::CommandBuffer,
    ) -> Result<Option<u64>, vk::Result> {
        if !self.top_level_started {
            return Err(vk::Result::NOT_READY);
        }
        let pool = self
            .top_level_pool
            .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
        unsafe {
            self.device.device.cmd_write_timestamp(
                command,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                pool,
                1,
            );
        }
        self.top_level_started = false;
        let mut values = [0u64; 2];
        unsafe {
            self.device.device.get_query_pool_results(
                pool,
                0,
                &mut values,
                vk::QueryResultFlags::TYPE_64 | vk::QueryResultFlags::WAIT,
            )?;
        }
        let properties = unsafe {
            self.device
                .instance
                .instance
                .get_physical_device_properties(self.device.physical)
        };
        let period_ns = properties.limits.timestamp_period.round() as u64;
        Ok(Some(
            values[1].wrapping_sub(values[0]).wrapping_mul(period_ns),
        ))
    }
    fn check_synchronization(
        &self,
        waits: &[TimelineWait],
        signals: &[TimelineSignal],
    ) -> Result<(), String> {
        if waits
            .iter()
            .any(|(s, _, stage)| !Arc::ptr_eq(&s.device, &self.device) || stage.is_empty())
            || signals
                .iter()
                .any(|(s, _)| !Arc::ptr_eq(&s.device, &self.device))
        {
            return Err("外部时间线必须来自同一设备且等待阶段不能为空".into());
        }
        if waits
            .iter()
            .map(|(s, _, _)| s.handle)
            .chain(signals.iter().map(|(s, _)| s.handle))
            .any(|handle| self.queue_timelines.iter().any(|own| own.handle == handle))
        {
            return Err("队列内部时间线由提交生产者管理，不能重复作为外部同步输入".into());
        }
        Ok(())
    }
    /// 图依赖只等待首批次，图完成信号只写入最后批次，semaphore Arc 保留到回执持有者释放。
    pub fn set_graph_synchronization(
        &mut self,
        waits: Vec<TimelineWait>,
        signals: Vec<TimelineSignal>,
    ) -> Result<(), String> {
        self.check_synchronization(&waits, &signals)?;
        self.graph_waits = waits;
        self.graph_signals = signals;
        Ok(())
    }
    pub fn set_batch_synchronization(
        &mut self,
        index: usize,
        waits: Vec<TimelineWait>,
        signals: Vec<TimelineSignal>,
    ) -> Result<(), String> {
        self.check_synchronization(&waits, &signals)?;
        self.synchronization
            .insert(index, OwnedBatchSync { waits, signals });
        Ok(())
    }
    pub fn declare(
        &mut self,
        graph: &mut graph::RenderGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<super::resource::ResourceHandle, graph::BuildError> {
        let descriptor = spec.descriptor();
        if self.specifications.contains_key(name) {
            return Err(graph::BuildError::DuplicateResourceName(name.into()));
        }
        let handle = graph.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor,
                source: graph::OriginSource::Create,
            },
        )?;
        self.specifications.insert(name.into(), spec);
        Ok(handle)
    }

    pub fn resource_clone(&self, token: u64) -> Option<GpuResource> {
        self.resources.get(&token).cloned()
    }
    fn collect_completed_managed(&mut self, prefix: u64) -> Result<(), String> {
        self.stores
            .take_out_dropped_waste_checked()
            .map_err(|e| format!("退役记录处理失败: {e:?}"))?;
        while self.managed_prefix < prefix {
            let sequence = self.managed_prefix + 1;
            self.stores
                .drain_completed_sequence(sequence)
                .map_err(|e| format!("命令销毁桶排出失败: {e:?}"))?;
            self.managed_prefix = sequence;
        }
        Ok(())
    }

    /// 跨帧沿用 token 和真实 Arc，不以 token 猜测 Vulkan 句柄或 generation。
    pub fn bind_import_named(
        &mut self,
        name: &str,
        token: u64,
        resource: GpuResource,
    ) -> Result<(), graph::BuildError> {
        self.bind_import_named_owned_by(name, token, resource, self.device.queue_family)
    }
    /// 已有独占资源须声明真实归属；不为跨队列族资源伪造 release/acquire。
    pub fn bind_import_named_owned_by(
        &mut self,
        name: &str,
        token: u64,
        resource: GpuResource,
        owner_family: u32,
    ) -> Result<(), graph::BuildError> {
        if token == 0 || token == u64::MAX {
            return Err(graph::BuildError::ResourceCreationFailed(
                "跨帧资源token无效".into(),
            ));
        }
        if let Some(old) = self.resources.get(&token) {
            let same = match (old, &resource) {
                (GpuResource::Buffer(a), GpuResource::Buffer(b)) => Arc::ptr_eq(a, b),
                (GpuResource::Image(a), GpuResource::Image(b)) => Arc::ptr_eq(a, b),
                _ => false,
            };
            if !same {
                return Err(graph::BuildError::ResourceCreationFailed(
                    "token已经属于其他物理资源".into(),
                ));
            }
        }
        let raw = match &resource {
            GpuResource::Buffer(b) => RawResource::Buffer(b.handle),
            GpuResource::Image(i) => RawResource::Image(i.handle),
        };
        self.resources.insert(token, resource);
        reserve_token(token);
        self.import_families.insert(token, owner_family);
        self.resolver.0.write().insert(name.into(), raw);
        Ok(())
    }
    pub fn bind_import(&mut self, resource: GpuResource) -> u64 {
        self.bind_import_owned_by(resource, self.device.queue_family)
    }
    pub fn bind_import_owned_by(&mut self, resource: GpuResource, owner_family: u32) -> u64 {
        let token = allocate_token();
        self.resources.insert(token, resource);
        self.import_families.insert(token, owner_family);
        token
    }
    fn validate_graph(&self, declaration: &graph::GraphDeclaration) -> Result<(), String> {
        // compile_graph 对超出生命周期打包域的 pass 数保留断言；宿主在执行前以错误拒绝。
        if declaration.passes.len() > graph::MAX_PASSES {
            return Err(format!("{:?}", graph::BuildError::PassLimit));
        }
        let total = if declaration.batch_per_pass {
            declaration.passes.len()
        } else {
            usize::from(!declaration.passes.is_empty())
        };
        if self.synchronization.keys().any(|&index| index >= total)
            || (total == 0 && (!self.graph_waits.is_empty() || !self.graph_signals.is_empty()))
        {
            return Err("同步计划指向不存在的图批次".into());
        }
        let mut last_signals = BTreeMap::new();
        for index in 0..total {
            let plan = self.batch_synchronization(index, total);
            for signal in &plan.signals {
                let previous = if let Some(value) = last_signals.get(&signal.semaphore.as_raw()) {
                    *value
                } else {
                    unsafe {
                        self.device
                            .device
                            .get_semaphore_counter_value(signal.semaphore)
                    }
                    .map_err(|e| format!("查询外部时间线失败: {e:?}"))?
                };
                let required = plan
                    .waits
                    .iter()
                    .filter(|w| w.semaphore == signal.semaphore)
                    .map(|w| w.value)
                    .max()
                    .unwrap_or(0)
                    .max(previous);
                if signal.value <= required {
                    return Err("图时间线信号必须严格大于该批等待值和前次信号值".into());
                }
                last_signals.insert(signal.semaphore.as_raw(), signal.value);
            }
        }
        for resource in &declaration.resources {
            if let graph::OriginSource::ImportOwned { token, .. }
            | graph::OriginSource::ImportUnowned { token, .. } = resource.source
            {
                let owner = self
                    .import_families
                    .get(&token)
                    .copied()
                    .ok_or("导入资源缺少队列族归属声明")?;
                if owner != self.family {
                    return Err("跨队列族独占资源未提供 release/acquire 计划，拒绝提交".into());
                }
            }
        }
        for code in declaration
            .passes
            .iter()
            .flat_map(|p| &p.accesses)
            .map(|a| a.access_code)
            .chain(declaration.exports.iter().map(|e| e.access_code))
            .chain(declaration.resources.iter().filter_map(|r| match r.source {
                graph::OriginSource::Create => None,
                graph::OriginSource::ImportOwned { initial_access, .. }
                | graph::OriginSource::ImportUnowned { initial_access, .. } => Some(initial_access),
            }))
        {
            if !self.accesses.0.contains_key(&code) {
                return Err(format!("缺少访问码 {code} 的显式 Vulkan 映射"));
            }
        }
        Ok(())
    }
    fn new_execution_chain(&self) -> ExecutionChain {
        ExecutionChain::new(
            self.device.additional_queues.len() as u32,
            self.device
                .additional_queues
                .iter()
                .map(|q| q.family)
                .chain(std::iter::once(self.device.queue_family))
                .max()
                .unwrap_or(0)
                + 1,
            submit::DeviceState {
                primary: submit::QueueState {
                    handle: self.device.queue,
                    timeline_semaphore: self.queue_timelines[0].handle,
                    pending_timelines: VecDeque::new(),
                    pending_commands: VecDeque::new(),
                },
                auxiliary: self
                    .device
                    .additional_queues
                    .iter()
                    .enumerate()
                    .map(|(i, q)| submit::QueueState {
                        handle: q.handle,
                        timeline_semaphore: self.queue_timelines[i + 1].handle,
                        pending_timelines: VecDeque::new(),
                        pending_commands: VecDeque::new(),
                    })
                    .collect(),
                next_timeline: 1,
                oldest_pending: None,
            },
        )
    }
    pub fn execute(mut self, built: graph::BuiltGraph) -> Result<GpuGraphRun, String> {
        let (declaration, mut callbacks, retire) = built.into_parts();
        self.validate_graph(&declaration)?;
        let mut chain = self.new_execution_chain();
        let mut accesses = self.accesses.clone();
        let execution = chain
            .execute_graph_with_context(
                declaration,
                &mut graph::Pool::default(),
                &mut accesses,
                &mut self,
                &mut callbacks,
            )
            .map_err(|e| format!("{e:?}"))?;
        Ok(GpuGraphRun {
            execution,
            chain,
            backend: self,
            retire,
            completed: false,
        })
    }
}
impl graph::ResourceBackend for VulkanGraphBackend {
    fn create_without_initial_data(
        &mut self,
        descriptor: graph::Descriptor,
        name: &str,
    ) -> Result<u64, graph::BackendFailure> {
        let resource = match self.specifications.get(name).copied() {
            Some(ResourceSpec::Buffer(mut d)) => {
                d.usage |= vk::BufferUsageFlags::from_raw(descriptor.usage());
                buffer::Buffer::new(self.device.clone(), &d)
                    .map(|r| GpuResource::Buffer(Arc::new(r)))
            }
            Some(ResourceSpec::Image(mut d)) => {
                d.usage |= vk::ImageUsageFlags::from_raw(descriptor.usage());
                texture::Texture::new(self.device.clone(), &d)
                    .map(|r| GpuResource::Image(Arc::new(r)))
            }
            None => {
                return Err(graph::BackendFailure {
                    discriminant: 0,
                    message: format!("缺少资源规格 {name}"),
                });
            }
        }
        .map_err(|e| graph::BackendFailure {
            discriminant: 0,
            message: e.to_string(),
        })?;
        let raw = match &resource {
            GpuResource::Buffer(buffer) => RawResource::Buffer(buffer.handle),
            GpuResource::Image(image) => RawResource::Image(image.handle),
        };
        self.resolver.0.write().insert(name.into(), raw);
        Ok(self.bind_import_owned_by(resource, self.family))
    }
    fn set_debug_name(&mut self, _token: u64, _descriptor: graph::Descriptor, _name: &str) {}
    fn release_created_resource(&mut self, token: u64) {
        // 只在 GPU 不会再用到该资源时调用（见 graph::reclaim_failed_resources）；命令拥有者仍持有的 Arc 不受影响。
        self.resources.remove(&token);
        self.import_families.remove(&token);
        if let Some(handle) = self.managed.remove(&token) {
            self.released_managed.push(handle);
        }
    }
}
impl ExecutionBackend for VulkanGraphBackend {
    fn prepare_compiled_resources(&mut self, compiled: &graph::CompiledGraph) -> bool {
        // 每帧仅发布本次编译的资源，避免裁剪后留下上一帧已退役对象的裸句柄。
        let mut resolver = BTreeMap::new();
        for (index, declaration) in compiled.declarations.resources.iter().enumerate() {
            let Some(token) = compiled.snapshot.physical_mapping[index].token else {
                continue;
            };
            let raw = match self.resources.get(&token) {
                Some(GpuResource::Buffer(b)) => RawResource::Buffer(b.handle),
                Some(GpuResource::Image(i)) => RawResource::Image(i.handle),
                None => return false,
            };
            resolver.insert(declaration.name.clone(), raw);
        }
        *self.resolver.0.write() = resolver;
        self.pass_resources = match super::pass_bindings::ManagedPassResources::from_registry(
            self.device.clone(),
            &compiled.registry,
            &self.stores,
            |token| self.resources.get(&token).cloned(),
        ) {
            Ok(resources) => Some(Arc::new(resources)),
            Err(_) => return false,
        };
        true
    }
    fn managed_pass_resources(&self) -> Option<Arc<super::pass_bindings::ManagedPassResources>> {
        self.pass_resources.clone()
    }
    fn command_started(&mut self, command: &lifecycle::Command) -> Result<(), lifecycle::Failure> {
        self.stores
            .begin_command_bucket(command.sequence().0)
            .map_err(|_| lifecycle::Failure::DuplicateBucket)
    }
    fn retain_command_owners(
        &mut self,
        handle: u64,
        owners: Vec<crate::raw::binding::ResourceOwner>,
    ) -> bool {
        let Some(command) = self.commands.get_mut(&handle) else {
            return false;
        };
        for owner in owners {
            command.keep_resource_owner(owner);
        }
        true
    }
    fn collect_managed_waste(&mut self) -> bool {
        self.stores.take_out_dropped_waste_checked().is_ok()
    }
    fn graph_queue(&self) -> (lifecycle::Queue, u32) {
        (self.queue, self.family)
    }
    fn batch_synchronization(&self, index: usize, total: usize) -> GraphBatchSync {
        let mut synchronization = GraphBatchSync::default();
        let explicit = self.synchronization.get(&index);
        let waits = explicit
            .into_iter()
            .flat_map(|s| s.waits.iter())
            .chain(self.graph_waits.iter().filter(|_| index == 0));
        let signals = explicit
            .into_iter()
            .flat_map(|s| s.signals.iter())
            .chain(self.graph_signals.iter().filter(|_| index + 1 == total));
        synchronization.waits = waits
            .map(|(semaphore, value, stage)| GraphTimelineWait {
                semaphore: semaphore.handle,
                value: *value,
                stage: *stage,
            })
            .collect();
        synchronization.signals = signals
            .map(|(semaphore, value)| GraphTimelineSignal {
                semaphore: semaphore.handle,
                value: *value,
            })
            .collect();
        synchronization
    }
    fn managed_resource(
        &mut self,
        token: u64,
        runtime: &mut lifecycle::SequenceRuntime,
    ) -> Option<lifecycle::ResourceHandle> {
        if !self.resources.contains_key(&token) {
            return None;
        }
        let existing = self.managed.get(&token).copied();
        let handle = *self
            .managed
            .entry(token)
            .or_insert_with(|| runtime.create_resource(token, 1));
        if self.frame_borrows || existing.is_some() {
            runtime.clone_handle(handle).ok()?;
        }
        if self.frame_borrows {
            self.frame_handles.push(handle);
        }
        Some(handle)
    }
    fn command_handle(&mut self) -> Result<u64, lifecycle::Failure> {
        self.command_for_slot(0)
    }
    fn chunk_command_handle(&mut self, chunk: usize) -> Result<u64, lifecycle::Failure> {
        self.command_for_slot(chunk)
    }
    fn abandon_command(&mut self, handle: u64) {
        self.barriers.remove(&handle);
        let Some(command) = self.commands.get_mut(&handle) else {
            return;
        };
        // SAFETY: 被放弃的命令从未提交，GPU 不可能引用其拥有者。
        unsafe { command.release_completed_owners() };
        // 仍在录制的命令可能停在未结束的 render pass 里，不能安全 end/reset：留给命令池销毁，不复用。
        if !command.is_recording() && !self.recycled_commands.contains(&handle) {
            self.recycled_commands.push(handle);
        }
    }
    fn chunk_recorder(&self) -> Option<Box<dyn super::executor::ChunkRecorder + '_>> {
        Some(Box::new(VulkanChunkView {
            device: &self.device.device,
            accesses: &self.accesses,
            resources: &self.resources,
        }))
    }
    fn begin_command(&mut self, handle: u64) -> Result<(), lifecycle::Failure> {
        self.commands
            .get_mut(&handle)
            .unwrap()
            .begin()
            .map_err(|_| lifecycle::Failure::InvalidPendingState)
    }
    fn end_command(&mut self, handle: vk::CommandBuffer) -> bool {
        if !self.flush_barriers(handle.as_raw()) {
            return false;
        }
        self.commands
            .get_mut(&handle.as_raw())
            .is_some_and(|c| c.end().is_ok())
    }
    fn flush_barriers(&mut self, handle: u64) -> bool {
        let Some(scope) = self.barriers.remove(&handle) else {
            return true;
        };
        if !self.commands.contains_key(&handle) {
            return false;
        }
        if !scope.is_empty() {
            unsafe {
                self.device.device.cmd_pipeline_barrier(
                    vk::CommandBuffer::from_raw(handle),
                    scope.source_stages,
                    scope.destination_stages,
                    vk::DependencyFlags::empty(),
                    &scope.memory,
                    &scope.buffers,
                    &scope.images,
                );
            }
        }
        true
    }
    fn transition(&mut self, command: u64, token: u64, from: u8, to: u8) -> bool {
        if !self.commands.contains_key(&command) {
            return false;
        }
        let scope = self.barriers.entry(command).or_default();
        add_transition(scope, &self.accesses, &self.resources, token, from, to)
    }
    fn submit(&mut self, queue: vk::Queue, prepared: &[submit::PreparedSubmit]) -> (i32, bool) {
        let mut timelines = prepared
            .iter()
            .map(|p| {
                vk::TimelineSemaphoreSubmitInfo::default()
                    .wait_semaphore_values(&p.wait_values)
                    .signal_semaphore_values(&p.signal_values)
            })
            .collect::<Vec<_>>();
        let _guard = self.device.queue_lock.lock();
        let retire = self.device.note_submit_on(queue);
        // 末尾追加一个空提交，让退役时间线在本次所有批次之后 signal。
        let retire_handles: Vec<_> = retire.iter().map(|(s, _)| *s).collect();
        let retire_values: Vec<_> = retire.iter().map(|(_, v)| *v).collect();
        let mut retire_timeline =
            vk::TimelineSemaphoreSubmitInfo::default().signal_semaphore_values(&retire_values);
        let mut info = prepared
            .iter()
            .zip(timelines.iter_mut())
            .map(|(p, t)| {
                vk::SubmitInfo::default()
                    .wait_semaphores(&p.wait_semaphores)
                    .wait_dst_stage_mask(&p.wait_stages)
                    .command_buffers(&p.commands)
                    .signal_semaphores(&p.signal_semaphores)
                    .push_next(t)
            })
            .collect::<Vec<_>>();
        if retire.is_some() {
            info.push(
                vk::SubmitInfo::default()
                    .signal_semaphores(&retire_handles)
                    .push_next(&mut retire_timeline),
            );
        }
        let result = match unsafe {
            self.device
                .device
                .queue_submit(queue, &info, vk::Fence::null())
        } {
            Ok(()) => (0, false),
            Err(e) => {
                self.device.note_submit_failed(retire);
                (e.as_raw(), false)
            }
        };
        if self.submissions.len() >= SUBMISSION_CAPTURE_LIMIT {
            let excess = self.submissions.len() + 1 - SUBMISSION_CAPTURE_LIMIT;
            self.submissions.drain(..excess);
        }
        self.submissions.push(GpuSubmissionCapture {
            queue,
            batches: prepared.to_vec(),
            result: result.0,
        });
        result
    }
}
impl Drop for VulkanGraphBackend {
    fn drop(&mut self) {
        if self.device.wait_idle().is_ok() {
            self.commands.clear();
            self.pass_resources.take();
            // 后端正在永久关闭；实际 idle 已证实，允许处理未提交命令的退役记录。
            let _ = self
                .stores
                .buffers
                .dispose_after_wait(|| Ok::<(), vk::Result>(()));
        }
    }
}
pub struct GpuGraphRun {
    pub execution: GraphExecution,
    pub chain: ExecutionChain,
    pub backend: VulkanGraphBackend,
    retire: Vec<graph::RetireCallback>,
    completed: bool,
}
impl GpuGraphRun {
    pub fn wait(&mut self, timeout: u64) -> Result<(), String> {
        if self.completed {
            return Ok(());
        }
        for receipt in &self.execution.receipts {
            self.backend
                .timeline
                .wait(receipt.last_timeline.0, timeout)
                .map_err(|e| format!("{e:?}"))?;
        }
        for handle in &self.execution.resources {
            self.chain.lifecycle.cpu_retire(*handle);
            self.chain
                .lifecycle
                .drop_handle(*handle)
                .map_err(|e| format!("{e:?}"))?;
        }
        self.chain.lifecycle.collect_waste();
        for receipt in &self.execution.receipts {
            self.chain
                .complete(receipt.queue, receipt.last_timeline)
                .map_err(|e| format!("{e:?}"))?;
        }
        while let Some(command) = self.chain.lifecycle.recycle(self.backend.family) {
            if let Some(raw) = self.backend.commands.get_mut(&command.handle()) {
                // 上述 GPU wait 已覆盖对应 command 的真实 timeline。
                unsafe { raw.release_completed_owners() };
            }
            self.backend.recycled_commands.push(command.handle());
        }
        self.backend.pass_resources.take();
        for receipt in &self.execution.receipts {
            self.backend
                .collect_completed_managed(self.chain.lifecycle.prefix(receipt.queue))?;
        }
        for callback in self.retire.drain(..) {
            callback()
        }
        self.completed = true;
        Ok(())
    }
}
impl Drop for GpuGraphRun {
    fn drop(&mut self) {
        let _ = self.wait(u64::MAX);
    }
}

#[derive(Clone, Debug)]
pub struct SessionFrameReceipt {
    pub frame: u64,
    pub receipts: Vec<lifecycle::SubmissionReceipt>,
    pub snapshot: graph::Snapshot,
    pub command_sequences: Vec<u64>,
    /// 本帧编译/录制/提交耗时与并行分块统计。
    pub stats: super::executor::ExecutionStats,
}
struct PendingSessionFrame {
    frame: u64,
    execution: GraphExecution,
    retire: Vec<graph::RetireCallback>,
    started_at: std::time::Instant,
}

/// 顺序录制的持久帧队列：同一个 backend、队列时间线、counter、资源池跨帧存活。
/// 不扩张为原游戏并行调度策略；回调/提交失败进入停态，显式关闭后才释放拥有者。
/// 执行期间若发生 panic 展开，把会话标记为已停止。
struct PoisonOnUnwind<'a> {
    poisoned: &'a mut Option<String>,
    armed: bool,
}
impl Drop for PoisonOnUnwind<'_> {
    fn drop(&mut self) {
        if self.armed {
            log::error!("渲染回调 panic，图会话被标记为 poisoned");
            *self.poisoned = Some("渲染回调 panic，命令缓冲可能处于未完成录制状态".into());
        }
    }
}
pub struct VulkanGraphSession {
    pub backend: VulkanGraphBackend,
    pub chain: ExecutionChain,
    pool: graph::Pool,
    /// 原 `GraphExecutor` 首字段 `VecDeque<PendingSubmission>`（0x140beabd0/0x140bec400/0x140bea680）。
    pending: super::frame_throttle::FrameThrottle<PendingSessionFrame>,
    /// 可选：挂接后把提交队列状态镜像给设备的 `SubmissionEngine`（原 `+0x720`）。
    graphics_device: Option<Arc<crate::raw::device::GraphicsDevice>>,
    next_frame: u64,
    poisoned: Option<String>,
    failed_retire: Vec<graph::RetireCallback>,
    /// 上一帧从 begin_frame 到该帧完成的墙钟时长；仅统计用途。
    last_frame_duration_ns: u64,
    /// 顶层 GPU 时间戳查询最近一次给出的 begin→end 时长（原 `last_known_frame_duration_ns` 的 `Option<u64>` 形态）。
    last_gpu_frame_ns: Option<u64>,
    /// 原 recorder 里 `+0x108` 回执标签 / `+0x164` 结束标记的等价物（`frame::FrameLedger`）。
    ledger: super::frame::FrameLedger,
}
impl VulkanGraphSession {
    pub fn new(mut backend: VulkanGraphBackend) -> Self {
        backend.frame_borrows = true;
        let mut chain = backend.new_execution_chain();
        // 恢复原分块路径；轻帧门槛属于显式 opt-in 的候选优化。
        chain.parallel = super::parallel::ParallelRecording::ORIGINAL;
        #[cfg(feature = "candidate-optimizations")]
        {
            chain.parallel = super::parallel::ParallelRecording::ADAPTIVE;
        }
        Self {
            backend,
            chain,
            pool: graph::Pool::default(),
            pending: Default::default(),
            graphics_device: None,
            next_frame: 1,
            poisoned: None,
            failed_retire: vec![],
            last_frame_duration_ns: 0,
            last_gpu_frame_ns: None,
            ledger: Default::default(),
        }
    }
    pub fn is_poisoned(&self) -> bool {
        self.poisoned.is_some()
    }
    /// 原 GraphExecutor::last_known_frame_duration_ns：最近一次完成帧的墙钟时长。
    pub fn last_known_frame_duration_ns(&self) -> u64 {
        self.last_frame_duration_ns
    }
    /// 顶层 GPU 时间戳查询的 begin 委托；command 必须是本帧录制中的命令缓冲。
    pub fn begin_top_level_timestamp_query(
        &mut self,
        command: vk::CommandBuffer,
    ) -> Result<(), vk::Result> {
        self.backend.begin_top_level_timestamp_query(command)
    }
    /// 顶层 GPU 时间戳查询的 end 委托；返回本帧 begin→end 的时长（ns）。
    pub fn end_top_level_timestamp_query(
        &mut self,
        command: vk::CommandBuffer,
    ) -> Result<Option<u64>, vk::Result> {
        let ns = self.backend.end_top_level_timestamp_query(command)?;
        // 原 `last_known_frame_duration_ns`（0x140beaae0）：不小于 1e9 ns 视为无效 → None。
        self.last_gpu_frame_ns = ns.filter(|&v| (v as f64) < super::frame::FRAME_DURATION_VALID_BELOW_NS);
        Ok(ns)
    }
    /// 原 `GraphExecutor::last_known_frame_duration_ns` 的真实返回形态 `Option<u64>`（见 `rg::frame`）：
    /// 最近一次顶层 GPU 时间戳时长，且小于 1 秒；没有或无效为 `None`。
    /// 与 [`Self::last_known_frame_duration_ns`]（墙钟，候选统计）不同。
    pub fn last_known_gpu_frame_duration_ns(&self) -> Option<u64> {
        self.last_gpu_frame_ns
    }
    /// 本帧提交账本（`begin_frame` 清零，`execute_frame` 记提交，`end_frame` 校验）。
    pub fn frame_ledger(&self) -> &super::frame::FrameLedger {
        &self.ledger
    }
    /// 原 GraphExecutor::wait_submits_done：等待在途提交全部完成。
    /// wait_frame 对每个在途帧执行有界等待，超时以 Err 返回。
    pub fn wait_submits_done(&mut self, timeout_ns: u64) -> Result<(), String> {
        while let Some(pending) = self.pending.front() {
            let frame = pending.frame;
            self.wait_frame(frame, timeout_ns)?;
        }
        Ok(())
    }
    pub fn begin_frame(&mut self) -> Result<graph::RenderGraph, String> {
        // 原 begin_frame 才排出队头，保留最近一帧；trim 只等待，不改变队列。
        // 0x140beabd0：承接弹出项的是容量 2 的 ArrayVec，第三次弹出（在途 >= 4）panic。
        // `execute_frame` 要求在途 <= 1 才能提交，所以会话内在途最多 2、此处至多弹 1 次，正常不可达；
        // 保留断言使越界与原版同样是 panic 而非静默多排（见 `FrameThrottle::begin_frame_bounded`）。
        let mut drained = 0usize;
        while self.pending.in_flight() > 1 {
            assert!(
                drained < super::frame::BEGIN_FRAME_DRAIN_CAPACITY,
                "begin_frame drained more in-flight submissions than the original ArrayVec holds"
            );
            let frame = self.pending.front().unwrap().frame;
            self.wait_frame(frame, 10_000_000_000)?;
            drained += 1;
        }
        if let Some(error) = &self.poisoned {
            return Err(format!("会话已停止: {error}"));
        }
        self.ledger.begin();
        self.backend.specifications.clear();
        self.backend.resolver.0.write().clear();
        self.backend.synchronization.clear();
        self.backend.graph_waits.clear();
        self.backend.graph_signals.clear();
        self.backend.submissions.clear();
        Ok(graph::RenderGraph::new())
    }
    pub fn execute_frame(
        &mut self,
        built: graph::BuiltGraph,
    ) -> Result<SessionFrameReceipt, String> {
        self.execute_built(built, |_| Ok(()))
    }
    /// temporal 帧的开始：同 [`Self::begin_frame`]，并把上一帧请求后被放弃（未导出）的 temporal 记录恢复为可请求。
    pub fn begin_temporal_frame(
        &mut self,
        temporal: &mut super::temporal::TemporalRenderGraphState,
    ) -> Result<graph::RenderGraph, String> {
        let graph = self.begin_frame()?;
        temporal.begin_frame();
        Ok(graph)
    }
    /// temporal 帧的执行：把本帧请求的 temporal 物理资源导入后端，`export_temporal` 导出全部请求，
    /// 编译后先以 `validate_compiled_exports` 校验导出映射（失败时不开始任何命令），再录制提交。
    /// 有提交时确认导出访问；失败或空图则回退到上次确认的访问，GPU 未执行的导出不会让下一帧带着错误的初始布局。
    pub fn execute_temporal_frame(
        &mut self,
        mut graph: graph::RenderGraph,
        temporal: &mut super::temporal::TemporalRenderGraphState,
        resources: &TemporalGpuResources,
    ) -> Result<SessionFrameReceipt, String> {
        for (name, resource) in temporal.requested() {
            if let Err(error) = resources.import_into(&mut self.backend, &name, resource.token) {
                temporal.abandon_frame();
                return Err(format!("{error:?}"));
            }
        }
        if let Err(error) = temporal.export_temporal(&mut graph) {
            temporal.abandon_frame();
            return Err(format!("{error:?}"));
        }
        let validator = &*temporal;
        let result = self.execute_built(graph.build(), |compiled| {
            validator
                .validate_compiled_exports(compiled)
                .map_err(|error| format!("{error:?}"))
        });
        match &result {
            Ok(receipt) if !receipt.receipts.is_empty() => temporal.confirm_exports(),
            _ => temporal.abandon_frame(),
        }
        result
    }
    fn execute_built(
        &mut self,
        built: graph::BuiltGraph,
        validate: impl FnOnce(&graph::CompiledGraph) -> Result<(), String>,
    ) -> Result<SessionFrameReceipt, String> {
        // 提交捕获只描述最近一帧；不依赖调用方先 begin_frame。
        self.backend.submissions.clear();
        if self.pending.in_flight() > 1 {
            return Err("须先调用 begin_frame 执行帧节流".into());
        }
        // 上一帧已结束而调用方没有再 begin_frame（原版 begin_frame 重建 recorder）：开新账本，避免误报重复结束。
        if self.ledger.has_ended() {
            self.ledger.begin();
        }
        if let Some(error) = &self.poisoned {
            return Err(format!("会话已停止: {error}"));
        }
        let (declaration, mut callbacks, retire) = built.into_parts();
        self.backend.validate_graph(&declaration)?;
        let start = self.chain.lifecycle.events.len();
        let mut access = self.backend.accesses.clone();
        // 回调 panic 是展开而不是 Err，会绕过下面的 poisoned 处理并留下半录制的命令缓冲；
        // 守卫在展开时把会话标记为已停止，后续 begin/execute 会明确拒绝而不是静默继续。
        let mut guard = PoisonOnUnwind {
            poisoned: &mut self.poisoned,
            armed: true,
        };
        let executed = self.chain.execute_graph_validated(
            declaration,
            &mut self.pool,
            &mut access,
            &mut self.backend,
            &mut callbacks,
            validate,
        );
        guard.armed = false;
        drop(guard);
        match executed {
            Ok(execution) => {
                self.backend.frame_handles.clear();
                let frame = self.next_frame;
                self.next_frame += 1;
                let command_sequences = self.chain.lifecycle.events[start..]
                    .iter()
                    .filter_map(|event| {
                        let mut parts = event.split(':');
                        (parts.next() == Some("begin"))
                            .then(|| {
                                parts.next();
                                parts.next()?.parse().ok()
                            })
                            .flatten()
                    })
                    .collect();
                let receipt = SessionFrameReceipt {
                    frame,
                    receipts: execution.receipts.clone(),
                    snapshot: execution.compiled.snapshot.clone(),
                    command_sequences,
                    stats: execution.stats,
                };
                self.end_frame(PendingSessionFrame {
                    frame,
                    execution,
                    retire,
                    started_at: std::time::Instant::now(),
                });
                Ok(receipt)
            }
            Err(error) => {
                let error = format!("{error:?}");
                // 开始过命令就在生命周期里留下了未提交的序号：会话进入停态，等 idle 清理。
                let begun = self.chain.lifecycle.events[start..]
                    .iter()
                    .any(|event| event.starts_with("begin:"));
                if begun {
                    self.poisoned = Some(error.clone());
                    self.failed_retire.extend(retire);
                    self.release_managed_after_failure(false);
                } else {
                    // 命令开始前失败（编译、校验、资源映射）：执行器已归还资源，GPU 未见任何东西，
                    // 本帧借用的托管句柄可立即释放，退役回调立即执行，会话保持可用。
                    self.backend.pass_resources.take();
                    self.release_managed_after_failure(true);
                    for callback in retire {
                        callback();
                    }
                }
                Err(error)
            }
        }
    }
    /// 释放失败帧经 `release_created_resource` 移出的托管基础引用；`frame_borrows` 时连同本帧借用一起释放。
    fn release_managed_after_failure(&mut self, frame_borrows: bool) {
        let mut handles = std::mem::take(&mut self.backend.released_managed);
        if frame_borrows {
            handles.append(&mut self.backend.frame_handles);
        }
        for handle in handles {
            self.chain.lifecycle.cpu_retire(handle);
            if let Err(error) = self.chain.lifecycle.drop_handle(handle) {
                log::warn!("失败帧托管句柄释放失败: {error:?}");
            }
        }
        self.chain.lifecycle.collect_waste();
    }
    fn end_frame(&mut self, pending: PendingSessionFrame) {
        // 原 end_frame 在 ensure_active_cb + submit_pending_cbs 之后检查"本帧是否有提交"，没有就 panic；
        // 这里的 execute_frame 只在有回执时才到这一步，空图（无 pass）可能没有回执，所以只告警不 panic。
        if !pending.execution.receipts.is_empty() {
            self.ledger.note_submission();
        }
        if let Err(error) = self.ledger.end() {
            log::warn!("end_frame 账本异常: {error:?}（原版此处 panic）");
        }
        self.pending.end_frame(pending);
        self.mirror_submission_state();
    }
    /// 挂接设备：创建原 `+0x720` 提交引擎（镜像本会话 `chain.device` 的队列状态）。
    /// 之后每次 end_frame / 帧完成都同步引擎，`GraphicsDevice::wait_idle_and_recycle_resources` 才真正推进队列。
    pub fn attach_graphics_device(&mut self, device: Arc<crate::raw::device::GraphicsDevice>) {
        device.attach_submission_engine(submit::SubmissionEngine::new(self.chain.device.clone()));
        self.graphics_device = Some(device);
    }
    fn mirror_submission_state(&self) {
        if let Some(device) = &self.graphics_device {
            let state = self.chain.device.clone();
            device.with_submission_engine(|engine| engine.state = state);
        }
    }
    /// 原 `wait_idle_and_recycle_resources` 的第 4 步（推进图形/辅助队列时间线并回收）；
    /// 该方法在原类型上取 `&mut self`，Arc 持有者无法调用，故在此经 `with_submission_engine` 复现。
    fn recycle_submission_engine(&self) {
        if let Some(device) = &self.graphics_device {
            let raw = self.backend.device.clone();
            device.with_submission_engine(|engine| {
                engine.update_queue_pending_submissions_with(|queue| unsafe {
                    raw.device
                        .get_semaphore_counter_value(queue.timeline_semaphore)
                        .unwrap_or(0)
                });
                engine.reclaim_command_buffer_resources_all_queues(0);
            });
        }
    }
    pub fn pending_frame_count(&self) -> usize {
        self.pending.in_flight()
    }
    /// 有界等候队头；即使完成也不弹出，退役只在 begin_frame/wait_frame 发生。
    pub fn wait_gpu_queue_trim(&self, timeout: u64) -> Result<bool, String> {
        let Some(pending) = self.pending.front() else {
            return Ok(true);
        };
        for receipt in &pending.execution.receipts {
            match self.backend.timeline.wait(receipt.last_timeline.0, timeout) {
                Ok(()) => {}
                Err(vk::Result::TIMEOUT) => return Ok(false),
                Err(error) => return Err(format!("帧节流等待失败: {error:?}")),
            }
        }
        Ok(true)
    }
    pub fn wait_frame(&mut self, frame: u64, timeout: u64) -> Result<(), String> {
        let pending = self.pending.front().ok_or("没有待完成帧")?;
        if pending.frame != frame {
            return Err("帧回执不属于当前待完成帧".into());
        }
        // 等待失败（含超时）时帧仍在队头，调用方可重试。
        for receipt in &pending.execution.receipts {
            self.backend
                .timeline
                .wait(receipt.last_timeline.0, timeout)
                .map_err(|e| format!("{e:?}"))?;
        }
        let pending = self.pending.pop_front().unwrap();
        // 帧已出队：后续每一步出错都只记下首个错误并继续，退役回调、资源池归还与命令回收不能被跳过。
        let mut first_error = None;
        for handle in pending.execution.resources {
            self.chain.lifecycle.cpu_retire(handle);
            keep_first_error(
                &mut first_error,
                self.chain
                    .lifecycle
                    .drop_handle(handle)
                    .map_err(|e| format!("{e:?}")),
            );
        }
        self.chain.lifecycle.collect_waste();
        for receipt in &pending.execution.receipts {
            keep_first_error(
                &mut first_error,
                self.chain
                    .complete(receipt.queue, receipt.last_timeline)
                    .map_err(|e| format!("{e:?}")),
            );
        }
        // 真正完成后才把 Command 交回后端；下次 reset/重录时仍使用递增的 per-queue counter。
        while let Some(command) = self.chain.lifecycle.recycle(self.backend.family) {
            if let Some(raw) = self.backend.commands.get_mut(&command.handle()) {
                unsafe { raw.release_completed_owners() };
            }
            self.backend.recycled_commands.push(command.handle());
        }
        keep_first_error(
            &mut first_error,
            self.backend
                .collect_completed_managed(self.chain.lifecycle.prefix(self.backend.queue)),
        );
        let compiled = pending.execution.compiled;
        let created = compiled
            .declarations
            .resources
            .iter()
            .enumerate()
            .filter(|(_, r)| matches!(r.source, graph::OriginSource::Create))
            .filter_map(|(i, _)| compiled.snapshot.physical_mapping[i].token)
            .collect::<std::collections::BTreeSet<_>>();
        // 只有本图创建的非导出资源加入 Pool；外部导入不被误当作可任意覆盖的暂存资源。
        for resource in compiled.into_exports().remaining {
            match resource {
                graph::RegistryResource::OwnedBuffer(buffer)
                    if created.contains(&buffer.handle) =>
                {
                    self.pool.insert_buffer(buffer)
                }
                graph::RegistryResource::OwnedTexture(texture)
                    if created.contains(&texture.handle) =>
                {
                    self.pool.insert_texture(texture)
                }
                _ => {}
            }
        }
        for callback in pending.retire {
            callback();
        }
        self.last_frame_duration_ns = pending.started_at.elapsed().as_nanos() as u64;
        self.mirror_submission_state();
        self.recycle_submission_engine();
        first_error.map_or(Ok(()), Err)
    }
    /// 显式淘汰缓存，必须在当前帧完成后调用；保持 counter/timeline，避免 resize 残留无限增长。
    pub fn collect_after_completion(&mut self, retained_tokens: &[u64]) -> Result<usize, String> {
        if !self.pending.is_empty() {
            return Err("待完成帧仍拥有资源，不能清理缓存".into());
        }
        self.backend
            .device
            .wait_idle()
            .map_err(|e| format!("{e:?}"))?;
        // 缩放/收尾级等待（原版此处走 wait_idle_and_recycle_resources）：idle 后推进并回收提交引擎。
        self.mirror_submission_state();
        self.recycle_submission_engine();
        self.backend.pass_resources.take();
        for command in self.backend.commands.values_mut() {
            unsafe { command.release_completed_owners() };
        }
        for handle in self.backend.frame_handles.drain(..) {
            self.chain.lifecycle.cpu_retire(handle);
            self.chain
                .lifecycle
                .drop_handle(handle)
                .map_err(|e| format!("{e:?}"))?;
        }
        self.release_managed_after_failure(false);
        let removed = self
            .backend
            .resources
            .keys()
            .copied()
            .filter(|token| !retained_tokens.contains(token))
            .collect::<Vec<_>>();
        self.pool.clear();
        for token in &removed {
            if let Some(handle) = self.backend.managed.remove(token) {
                self.chain.lifecycle.cpu_retire(handle);
                self.chain
                    .lifecycle
                    .drop_handle(handle)
                    .map_err(|e| format!("{e:?}"))?;
            }
            self.backend.resources.remove(token);
            self.backend.import_families.remove(token);
        }
        self.chain.lifecycle.collect_waste();
        self.backend.resolver.0.write().clear();
        for callback in self.failed_retire.drain(..) {
            callback();
        }
        if self.poisoned.is_some() {
            // 停态无法再次执行；actual idle 是未提交 command 清理的证明，不生成假 timeline。
            self.backend
                .stores
                .buffers
                .dispose_after_wait(|| Ok::<(), vk::Result>(()))
                .map_err(|e| format!("{e:?}"))?;
        } else {
            self.backend
                .collect_completed_managed(self.chain.lifecycle.prefix(self.backend.queue))?;
        }
        Ok(removed.len())
    }
    /// 释放单个导入资源（`bind_import*` 登记的 token），不需要整设备空闲，也不清别的缓存。
    ///
    /// 只拒绝“仍被在飞帧引用”的 token：那种资源 GPU 可能还在用，必须等对应帧 `wait_frame` 之后。
    /// 返回是否真的移除了东西（未登记的 token 返回 `Ok(false)`）。
    /// 以前只能靠 `collect_after_completion`（要求在飞为空 + 设备级 `wait_idle`）才会放掉导入资源，
    /// 每帧新建并导入的资源因此会一直堆到排空点。
    pub fn release_import(&mut self, token: u64) -> Result<bool, String> {
        if self.pending.iter().any(|frame| {
            frame
                .execution
                .compiled
                .snapshot
                .physical_mapping
                .iter()
                .any(|mapping| mapping.token == Some(token))
        }) {
            return Err(format!("token {token} 仍被在飞帧引用，不能释放"));
        }
        // `pass_resources` 只在录制期用；录制结束后在飞命令缓冲靠自己的 keep_alive 持有资源，
        // 这份缓存继续留着只会拖住已释放的导入资源。
        self.backend.pass_resources.take();
        let mut removed = self.backend.resources.remove(&token).is_some();
        self.backend.import_families.remove(&token);
        if let Some(handle) = self.backend.managed.remove(&token) {
            self.chain.lifecycle.cpu_retire(handle);
            self.chain
                .lifecycle
                .drop_handle(handle)
                .map_err(|e| format!("{e:?}"))?;
            self.chain.lifecycle.collect_waste();
            // 与 `wait_frame` 相同：推进完成前缀，让已退役的托管资源真正离开 store。
            self.backend
                .collect_completed_managed(self.chain.lifecycle.prefix(self.backend.queue))?;
            removed = true;
        }
        Ok(removed)
    }
    pub fn clear_after_completion(&mut self) -> Result<usize, String> {
        self.collect_after_completion(&[])
    }
    pub fn resource_count(&self) -> usize {
        self.backend.resources.len()
    }
    pub fn command_count(&self) -> usize {
        self.backend.commands.len()
    }
}
fn keep_first_error(first: &mut Option<String>, result: Result<(), String>) {
    if let Err(error) = result {
        log::warn!("帧完成清理步骤失败: {error}");
        first.get_or_insert(error);
    }
}
impl Drop for VulkanGraphSession {
    fn drop(&mut self) {
        while let Some(frame) = self.pending.front().map(|p| p.frame) {
            if self.wait_frame(frame, u64::MAX).is_err() {
                break;
            }
        }
        let _ = self.clear_after_completion();
    }
}

/// 帧缓存持有实际资源；每帧后端导入 Arc 后，resize 可安全释放旧缓存引用。
pub struct TemporalGpuResources {
    pub device: Arc<VulkanDevice>,
    specifications: BTreeMap<String, ResourceSpec>,
    resources: BTreeMap<u64, GpuResource>,
}
impl TemporalGpuResources {
    pub fn new(device: Arc<VulkanDevice>) -> Self {
        Self {
            device,
            specifications: BTreeMap::new(),
            resources: BTreeMap::new(),
        }
    }
    pub fn register(&mut self, name: &str, spec: ResourceSpec) -> graph::Descriptor {
        self.specifications.insert(name.into(), spec);
        spec.descriptor()
    }
    pub fn resource(&self, token: u64) -> Option<GpuResource> {
        self.resources.get(&token).cloned()
    }
    pub fn import_into(
        &self,
        backend: &mut VulkanGraphBackend,
        name: &str,
        token: u64,
    ) -> Result<(), graph::BuildError> {
        let resource = self
            .resource(token)
            .ok_or_else(|| graph::BuildError::ResourceCreationFailed("跨帧物理资源缺失".into()))?;
        backend.bind_import_named(name, token, resource)
    }
    pub fn len(&self) -> usize {
        self.resources.len()
    }
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
}
impl super::temporal::TemporalResourceOwner for TemporalGpuResources {
    fn create_temporal(
        &mut self,
        name: &str,
        descriptor: graph::Descriptor,
    ) -> Result<u64, graph::BuildError> {
        let spec = self.specifications.get(name).copied().ok_or_else(|| {
            graph::BuildError::ResourceCreationFailed(format!("没有{name}的物理资源规格"))
        })?;
        if spec.descriptor() != descriptor {
            return Err(graph::BuildError::ResourceCreationFailed(
                "跨帧声明与物理规格不一致".into(),
            ));
        }
        let resource = match spec {
            ResourceSpec::Buffer(desc) => buffer::Buffer::new(self.device.clone(), &desc)
                .map(|r| GpuResource::Buffer(Arc::new(r))),
            ResourceSpec::Image(desc) => texture::Texture::new(self.device.clone(), &desc)
                .map(|r| GpuResource::Image(Arc::new(r))),
        }
        .map_err(|e| graph::BuildError::ResourceCreationFailed(e.to_string()))?;
        let token = try_allocate_token()
            .ok_or_else(|| graph::BuildError::ResourceCreationFailed("跨帧token耗尽".into()))?;
        self.resources.insert(token, resource);
        Ok(token)
    }
    fn release_temporal(&mut self, token: u64) {
        self.resources.remove(&token);
    }
}
