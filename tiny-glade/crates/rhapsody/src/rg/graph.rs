pub use crate::rg::resource_info::{AccessDeclaration, ExportDeclaration, ResourceInfo};
pub use crate::rg::resource_info::{
    LifetimeEvent, NO_ACCESS, ResourceAccessRecord, ResourceLifetimeRecord,
};
pub use crate::rg::resource_registry::{
    Buffer, BufferDesc, RegistryResource, ResourceExports, ResourcePool, ResourceRegistry,
    ResourceReuseInfo, Texture, TextureDesc,
};
use std::ops::Range;

pub use crate::rg::compiled as batch_state;
pub type Registry = ResourceRegistry<u64, u64, ()>;
pub type Pool = ResourcePool<u64, u64>;
pub type Exports = ResourceExports<u64, u64, ()>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Descriptor {
    Image(TextureDesc),
    Buffer(BufferDesc),
}
impl Descriptor {
    pub fn kind_code(self) -> u8 {
        match self {
            Self::Image(_) => 0,
            Self::Buffer(_) => 1,
        }
    }
    pub fn usage(self) -> u32 {
        match self {
            Self::Image(desc) => desc.words_at_00_through_18[0],
            Self::Buffer(desc) => desc.word_at_08,
        }
    }
    pub fn with_usage(self, usage: u32) -> Self {
        match self {
            Self::Image(mut desc) => {
                desc.words_at_00_through_18[0] = usage;
                Self::Image(desc)
            }
            Self::Buffer(mut desc) => {
                desc.word_at_08 = usage;
                Self::Buffer(desc)
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginSource {
    Create,
    ImportOwned { token: u64, initial_access: u8 },
    ImportUnowned { token: u64, initial_access: u8 },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDeclaration {
    pub sharing: crate::resource::ResourceSharing,
    pub name: String,
    pub descriptor: Descriptor,
    pub source: OriginSource,
}

/// 可导出资源载体；`raw()` 返回图内逻辑句柄，`index()` 返回逻辑索引。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExportableResource {
    handle: super::resource::ResourceHandle,
}
impl ExportableResource {
    pub fn new(handle: super::resource::ResourceHandle) -> Self {
        Self { handle }
    }
    pub fn raw(&self) -> super::resource::ResourceHandle {
        self.handle
    }
    pub fn index(&self) -> u32 {
        self.handle.raw()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PassDeclaration {
    pub queue_class: crate::device::QueueClass,
    pub graph_id: Option<super::resource::GraphId>,
    pub name: std::sync::Arc<str>,
    pub accesses: Vec<AccessDeclaration>,
    /// Keeps externally observable work even when none of its outputs is exported.
    pub side_effect: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GraphDeclaration {
    pub subgraphs: Vec<super::subgraph::SubgraphDeclaration>,
    pub resources: Vec<ResourceDeclaration>,
    pub passes: Vec<PassDeclaration>,
    pub exports: Vec<ExportDeclaration>,
    pub batch_per_pass: bool,
}
impl GraphDeclaration {
    /// 在资源分配和索引打包之前检查调用方可直接构造的声明。
    pub fn validate(&self) -> Result<(), BuildError> {
        if self.resources.len() > usize::from(u16::MAX) {
            return Err(BuildError::ResourceLimit);
        }
        if self.passes.len() > MAX_PASSES {
            return Err(BuildError::PassLimit);
        }
        // Common frame graphs have only a few external resources. Keep their
        // identity check on the stack; larger declarations retain the ordered
        // set bound rather than turning validation into an unbounded quadratic scan.
        let mut inline_imports = [0u64; 32];
        let mut inline_count = 0usize;
        let mut imports = std::collections::BTreeSet::new();
        for resource in &self.resources {
            if let OriginSource::ImportOwned { token, .. }
            | OriginSource::ImportUnowned { token, .. } = resource.source
            {
                if inline_count < inline_imports.len() {
                    if inline_imports[..inline_count].contains(&token) {
                        return Err(BuildError::AliasedImport(token));
                    }
                    inline_imports[inline_count] = token;
                    inline_count += 1;
                } else {
                    if imports.is_empty() {
                        imports.extend(inline_imports);
                    }
                    if !imports.insert(token) {
                        return Err(BuildError::AliasedImport(token));
                    }
                }
            }
        }
        let mut count = 0usize;
        for pass in &self.passes {
            count = count
                .checked_add(pass.accesses.len())
                .ok_or(BuildError::AccessLimit)?;
            if count >= NO_ACCESS as usize {
                return Err(BuildError::AccessLimit);
            }
            for access in &pass.accesses {
                if access.resource_index as usize >= self.resources.len() {
                    return Err(BuildError::InvalidHandle(access.resource_index));
                }
            }
        }
        for export in &self.exports {
            let resource = self
                .resources
                .get(export.resource_index as usize)
                .ok_or(BuildError::InvalidHandle(export.resource_index))?;
            if export.tag != resource.descriptor.kind_code() {
                return Err(BuildError::ResourceKindMismatch(export.resource_index));
            }
            export
                .range
                .validate(resource.descriptor)
                .map_err(|error| {
                    super::resource_contracts::range_error(export.resource_index, error)
                })?;
        }
        Ok(())
    }
}

pub trait AccessDependency {
    /// The declared access of `code`; resource usage flags derive from it.
    fn access_mask(&mut self, code: u8) -> ash::vk::AccessFlags;
    fn is_write(&mut self, code: u8) -> bool;
    /// Custom CPU policies may omit native details. Their conservative scope
    /// cannot be reused for a policy with different Vulkan stages or layouts.
    fn native_semantics(
        &mut self,
        code: u8,
    ) -> (
        crate::resource::range_state::AccessScope,
        ash::vk::ImageLayout,
    ) {
        let write = self.is_write(code);
        (
            crate::resource::range_state::AccessScope {
                stages: ash::vk::PipelineStageFlags::ALL_COMMANDS,
                access: if write {
                    ash::vk::AccessFlags::MEMORY_READ | ash::vk::AccessFlags::MEMORY_WRITE
                } else {
                    ash::vk::AccessFlags::MEMORY_READ
                },
                write,
            },
            ash::vk::ImageLayout::GENERAL,
        )
    }
}

/// 渲染回调须为 `Send`：并行录制时不同块的回调在任务池线程上执行。
pub type RenderCallback = Box<dyn for<'a> FnMut(super::executor::PassContext<'a>) -> bool + Send>;
pub type RetireCallback = Box<dyn FnOnce()>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    WrongGraph {
        expected: super::resource::GraphId,
        actual: super::resource::GraphId,
    },
    Contract(super::resource::ResourceContractError),
    IncompatibleAccess(super::access_ledger::IncompatibleAccess),
    ResourceLimit,
    InvalidHandle(u32),
    ResourceKindMismatch(u32),
    AccessLimit,
    MissingTemporal(String),
    DuplicateResourceName(String),
    AliasedImport(u64),
    ResourceCreationFailed(String),
    BackendFailure(crate::error::SharedError),
    /// pass 数超过生命周期事件的打包域（pass 下标左移 17 位存入 u32，见 `MAX_PASSES`）。
    PassLimit,
}

impl From<crate::Error> for BuildError {
    fn from(error: crate::Error) -> Self {
        Self::BackendFailure(error.into())
    }
}

/// 单图最多 `1 << 15` 个 pass；构图与编译入口均返回 [`BuildError::PassLimit`]。
pub const MAX_PASSES: usize = 1 << 15;
impl std::fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "构图失败: {self:?}")
    }
}
impl std::error::Error for BuildError {}

/// 一帧的渲染图声明：资源、pass、回调与导出。
pub struct RenderGraph {
    pub(crate) graph_id: super::resource::GraphId,
    pub(crate) declaration: GraphDeclaration,
    pub(crate) callbacks: Vec<RenderCallback>,
    retire_callbacks: Vec<RetireCallback>,
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self {
            graph_id: super::resource::GraphId::next(),
            declaration: GraphDeclaration::default(),
            callbacks: Vec::new(),
            retire_callbacks: Vec::new(),
        }
    }
}

pub struct BuiltGraph {
    graph_id: super::resource::GraphId,
    pub declaration: GraphDeclaration,
    pub callbacks: Vec<RenderCallback>,
    retire_callbacks: Vec<RetireCallback>,
}

impl RenderGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_raw_resource(
        &mut self,
        name: impl Into<String>,
        origin: super::resource::ResourceOrigin,
    ) -> Result<super::resource::ResourceHandle, BuildError> {
        // 复用记录采用 u16 物理索引，最大值保留给无映射状态。
        if self.declaration.resources.len() >= usize::from(u16::MAX) {
            return Err(BuildError::ResourceLimit);
        }
        let handle = super::resource::ResourceHandle::scoped(
            self.graph_id,
            self.declaration.resources.len() as u32,
        );
        self.declaration.resources.push(ResourceDeclaration {
            sharing: crate::resource::ResourceSharing::Exclusive,
            name: name.into(),
            descriptor: origin.descriptor,
            source: origin.source,
        });
        Ok(handle)
    }

    /// `create_raw_resource` 的别名。
    pub fn create_resource_handle(
        &mut self,
        name: impl Into<String>,
        origin: super::resource::ResourceOrigin,
    ) -> Result<super::resource::ResourceHandle, BuildError> {
        self.create_raw_resource(name, origin)
    }

    pub(crate) fn check_handle(
        &self,
        handle: super::resource::ResourceHandle,
    ) -> Result<(), BuildError> {
        if (handle.raw() as usize) >= self.declaration.resources.len() {
            return Err(BuildError::InvalidHandle(handle.raw()));
        }
        if handle.graph_id() != self.graph_id {
            return Err(BuildError::WrongGraph {
                expected: self.graph_id,
                actual: handle.graph_id(),
            });
        }
        Ok(())
    }

    /// Bind a stored logical index to this graph after checking its declaration.
    pub fn resource_handle(
        &self,
        index: u32,
    ) -> Result<super::resource::ResourceHandle, BuildError> {
        let handle = super::resource::ResourceHandle::scoped(self.graph_id, index);
        self.check_handle(handle)?;
        Ok(handle)
    }

    pub fn graph_id(&self) -> super::resource::GraphId {
        self.graph_id
    }
    pub fn declaration(&self) -> &GraphDeclaration {
        &self.declaration
    }

    /// 不检查 pass 上限，保持既有调用面；超限会在 `PassBuilder::add_resource_access` 返回
    /// [`BuildError::PassLimit`]，需要在拿到 builder 前就得到错误时用 [`Self::try_add_pass`]。
    pub fn add_pass(
        &mut self,
        name: impl Into<std::sync::Arc<str>>,
    ) -> super::pass_builder::PassBuilder<'_> {
        let graph_id = self.graph_id;
        super::pass_builder::PassBuilder {
            graph: self,
            declaration: PassDeclaration {
                queue_class: crate::device::QueueClass::Graphics,
                graph_id: Some(graph_id),
                name: name.into(),
                accesses: vec![],
                side_effect: false,
            },
        }
    }

    /// 带 pass 上限检查的 `add_pass`：已有 [`MAX_PASSES`] 个 pass 时返回 [`BuildError::PassLimit`]。
    pub fn try_add_pass(
        &mut self,
        name: impl Into<std::sync::Arc<str>>,
    ) -> Result<super::pass_builder::PassBuilder<'_>, BuildError> {
        self.check_pass_capacity()?;
        Ok(self.add_pass(name))
    }

    pub(crate) fn check_pass_capacity(&self) -> Result<(), BuildError> {
        if self.declaration.passes.len() >= MAX_PASSES {
            Err(BuildError::PassLimit)
        } else {
            Ok(())
        }
    }

    pub fn export(
        &mut self,
        handle: super::resource::ResourceHandle,
        access_code: u8,
    ) -> Result<(), BuildError> {
        self.export_range(handle, access_code, super::resource::AccessRange::Whole)
    }

    /// Publish the boundary state of exactly this initialized range.
    pub fn export_range(
        &mut self,
        handle: super::resource::ResourceHandle,
        access_code: u8,
        range: super::resource::AccessRange,
    ) -> Result<(), BuildError> {
        self.check_handle(handle)?;
        let resource = &self.declaration.resources[handle.raw() as usize];
        range
            .validate(resource.descriptor)
            .map_err(|error| super::resource_contracts::range_error(handle.raw(), error))?;
        self.declaration.exports.push(ExportDeclaration {
            tag: resource.descriptor.kind_code(),
            resource_index: handle.raw(),
            access_code,
            range,
        });
        Ok(())
    }

    pub fn batch_per_pass(&mut self, enabled: bool) {
        self.declaration.batch_per_pass = enabled;
    }
    pub fn on_retire(&mut self, callback: impl FnOnce() + 'static) {
        self.retire_callbacks.push(Box::new(callback));
    }
    pub(crate) fn reserve_retirement_callback(&mut self) -> crate::Result<()> {
        self.retire_callbacks
            .try_reserve(1)
            .map_err(|_| crate::Error::OutOfHostMemory)
    }
    pub fn build(self) -> BuiltGraph {
        BuiltGraph {
            graph_id: self.graph_id,
            declaration: self.declaration,
            callbacks: self.callbacks,
            retire_callbacks: self.retire_callbacks,
        }
    }
}

impl BuiltGraph {
    pub(crate) fn into_graph(self) -> RenderGraph {
        RenderGraph {
            graph_id: self.graph_id,
            declaration: self.declaration,
            callbacks: self.callbacks,
            retire_callbacks: self.retire_callbacks,
        }
    }

    pub fn graph_id(&self) -> super::resource::GraphId {
        self.graph_id
    }
    /// 将退役回调交给完成回执的持有者；提交成功不代表 GPU 已完成。
    pub fn into_parts(self) -> (GraphDeclaration, Vec<RenderCallback>, Vec<RetireCallback>) {
        (self.declaration, self.callbacks, self.retire_callbacks)
    }
}
/// 资源创建错误保留设备丢失、显存不足等类别。
pub type BackendFailure = crate::Error;
pub trait ResourceBackend {
    fn frame_storage_pool(&self) -> Option<&super::frame_storage::FrameStoragePool> {
        None
    }

    fn history_validity(&self, _token: u64) -> Option<bool> {
        None
    }

    fn prepare_transient_resources(
        &mut self,
        _plan: &super::plan::CompiledPlan,
        _graph: &GraphDeclaration,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn has_prepared_resource(&self, _name: &str) -> bool {
        false
    }
    fn is_transient_alias(&self, _token: u64) -> bool {
        false
    }
    fn resource_matches_sharing(
        &self,
        _token: u64,
        _sharing: crate::resource::ResourceSharing,
    ) -> bool {
        true
    }
    fn device_generation(&self) -> Option<crate::device::DeviceGeneration> {
        None
    }
    fn create_without_initial_data(
        &mut self,
        descriptor: Descriptor,
        name: &str,
    ) -> Result<u64, BackendFailure>;
    fn set_debug_name(&mut self, token: u64, descriptor: Descriptor, name: &str);
    /// 失败路径归还：`create_without_initial_data` 创建、但本次图因失败不会再使用的 token。
    /// 资源填充失败时的回滚钩子；默认不做任何事。
    fn release_created_resource(&mut self, _token: u64) {}
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendEvent {
    CreateWithoutInitialData {
        descriptor: Descriptor,
        name: String,
        token: u64,
    },
    DebugName {
        token: u64,
        name: String,
    },
}
#[derive(Default)]
pub struct EventBackend {
    pub events: Vec<BackendEvent>,
    pub fail_at_creation: Option<(usize, u8)>,
    next_token: u64,
    creations: usize,
}
impl ResourceBackend for EventBackend {
    fn create_without_initial_data(
        &mut self,
        descriptor: Descriptor,
        name: &str,
    ) -> Result<u64, BackendFailure> {
        let attempt = self.creations;
        self.creations += 1;
        if let Some((failure_attempt, tag)) = self.fail_at_creation
            && attempt == failure_attempt
        {
            return Err(crate::Error::invalid_usage(format!(
                "injected resource creation {attempt}, class {tag}"
            )));
        }
        let token = self.next_token;
        self.next_token += 1;
        self.events.push(BackendEvent::CreateWithoutInitialData {
            descriptor,
            name: name.to_owned(),
            token,
        });
        Ok(token)
    }
    fn set_debug_name(&mut self, token: u64, _descriptor: Descriptor, name: &str) {
        self.events.push(BackendEvent::DebugName {
            token,
            name: name.to_owned(),
        });
    }
}
pub struct Population {
    pub registry: Registry,
    pub pool_hits: Vec<u32>,
}
pub struct PopulationFailure {
    pub error: BackendFailure,
    pub partial: Box<Population>,
}
impl std::fmt::Debug for PopulationFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PopulationFailure")
            .field("error", &self.error)
            .field(
                "backing_count",
                &self.partial.registry.backing_resources.len(),
            )
            .finish()
    }
}

pub fn populate_resources(
    info: &ResourceInfo,
    declarations: &[ResourceDeclaration],
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
) -> Result<Population, PopulationFailure> {
    let mut population = Population {
        registry: Registry::default(),
        pool_hits: Vec::new(),
    };
    if let Err(error) = populate_resources_into(
        info,
        declarations,
        pool,
        backend,
        &mut population.registry,
        &mut population.pool_hits,
    ) {
        return Err(PopulationFailure {
            error,
            partial: Box::new(population),
        });
    }
    Ok(population)
}

fn populate_resources_into(
    info: &ResourceInfo,
    declarations: &[ResourceDeclaration],
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    registry: &mut Registry,
    pool_hits: &mut Vec<u32>,
) -> crate::Result<()> {
    // Declarations remain frame-owned. Borrowing the origin avoids cloning
    // every name and descriptor when the immutable plan is already cached.
    fn reserve<T>(values: &mut Vec<T>, len: usize) -> crate::Result<()> {
        if values.capacity() < len {
            values
                .try_reserve_exact(len.saturating_sub(values.len()))
                .map_err(|_| crate::Error::OutOfHostMemory)?;
        }
        Ok(())
    }
    reserve(&mut registry.resources, declarations.len())?;
    reserve(&mut registry.resource_initial_access, declarations.len())?;
    reserve(&mut registry.backing_resources, declarations.len())?;
    reserve(
        &mut registry.exported_logical_handles,
        info.exported_indices.len(),
    )?;
    reserve(pool_hits, declarations.len())?;
    registry.resources.clear();
    registry
        .resources
        .resize(declarations.len(), ResourceReuseInfo::default());
    registry.resource_initial_access.clear();
    registry
        .resource_initial_access
        .resize(declarations.len(), 0);
    registry.backing_resources.clear();
    registry.exported_logical_handles.clear();
    registry
        .exported_logical_handles
        .extend(info.exported_indices.iter().copied());
    pool_hits.clear();
    for event in &info.lifetime_events {
        if event.release {
            continue;
        }
        let logical = usize::from(event.resource);
        let declaration = &declarations[logical];
        assert_eq!(
            registry.resources[logical].backing_index,
            u16::MAX,
            "origin already consumed"
        );
        let initial_access = match declaration.source {
            OriginSource::Create => 0,
            OriginSource::ImportOwned { initial_access, .. }
            | OriginSource::ImportUnowned { initial_access, .. } => initial_access,
        };
        let resource = match declaration.source {
            OriginSource::Create => {
                let usage = match declaration.descriptor {
                    Descriptor::Image(_) => info.image_usage_flags[logical],
                    Descriptor::Buffer(_) => info.buffer_usage_flags[logical],
                };
                let descriptor = declaration.descriptor.with_usage(usage);
                if usage == 0 {
                    RegistryResource::Empty
                } else {
                    let pooled = if backend.has_prepared_resource(&declaration.name) {
                        None
                    } else {
                        match descriptor {
                            Descriptor::Image(desc) => pool
                                .take_texture_where(&desc, |token| {
                                    backend.resource_matches_sharing(*token, declaration.sharing)
                                })
                                .map(RegistryResource::OwnedTexture),
                            Descriptor::Buffer(desc) => pool
                                .take_buffer_where(&desc, |token| {
                                    backend.resource_matches_sharing(*token, declaration.sharing)
                                })
                                .map(RegistryResource::OwnedBuffer),
                        }
                    };
                    let resource = if let Some(resource) = pooled {
                        pool_hits.push(logical as u32);
                        resource
                    } else {
                        let token = match backend
                            .create_without_initial_data(descriptor, &declaration.name)
                        {
                            Ok(token) => token,
                            Err(error) => {
                                return Err(error);
                            }
                        };
                        match descriptor {
                            Descriptor::Image(desc) => RegistryResource::OwnedTexture(Texture {
                                handle: token,
                                desc,
                            }),
                            Descriptor::Buffer(desc) => RegistryResource::OwnedBuffer(Buffer {
                                handle: token,
                                desc,
                            }),
                        }
                    };
                    let token = match &resource {
                        RegistryResource::OwnedTexture(texture) => texture.handle,
                        RegistryResource::OwnedBuffer(buffer) => buffer.handle,
                        _ => unreachable!(),
                    };
                    backend.set_debug_name(token, descriptor, &declaration.name);
                    resource
                }
            }
            OriginSource::ImportOwned { token, .. } => match declaration.descriptor {
                Descriptor::Image(desc) => RegistryResource::OwnedTexture(Texture {
                    handle: token,
                    desc,
                }),
                Descriptor::Buffer(desc) => RegistryResource::OwnedBuffer(Buffer {
                    handle: token,
                    desc,
                }),
            },
            OriginSource::ImportUnowned { token, .. } => match declaration.descriptor {
                Descriptor::Image(desc) => RegistryResource::UnownedTexture(Texture {
                    handle: token,
                    desc,
                }),
                Descriptor::Buffer(desc) => RegistryResource::UnownedBuffer(Buffer {
                    handle: token,
                    desc,
                }),
            },
        };
        assert!(
            registry.backing_resources.len() < 65536,
            "physical backing index exceeds u16"
        );
        registry.resources[logical] = ResourceReuseInfo {
            backing_index: registry.backing_resources.len() as u16,
            reused: false,
        };
        registry.resource_initial_access[logical] = initial_access;
        registry.backing_resources.push(resource);
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalMapping {
    pub logical: u32,
    pub physical: u16,
    pub reused: bool,
    pub initial_access: u8,
    pub payload_tag: u8,
    pub token: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotStatic {
    pub pass_order: Vec<String>,
    pub pass_ranges: Vec<Range<usize>>,
    pub accesses: Vec<ResourceAccessRecord>,
    pub lifetimes: Vec<ResourceLifetimeRecord>,
    pub image_usage: Vec<u32>,
    pub buffer_usage: Vec<u32>,
    pub lifetime_events: Vec<LifetimeEvent>,
    pub batches: Vec<batch_state::Batch>,
    pub imports: Vec<u32>,
    pub exports: Vec<u16>,
}
impl SnapshotStatic {
    pub(crate) fn from_plan(
        definition: &super::plan::GraphDefinition,
        retained: &[usize],
        info: &ResourceInfo,
        batches: &[batch_state::Batch],
    ) -> Self {
        let mut exports = info.exported_indices.iter().copied().collect::<Vec<_>>();
        exports.sort_unstable();
        Self {
            pass_order: retained
                .iter()
                .map(|&index| definition.passes[index].name.to_string())
                .collect(),
            pass_ranges: info
                .pass_access_ranges
                .iter()
                .map(|(start, end)| start.0 as usize..end.0 as usize)
                .collect(),
            accesses: info.accesses.clone(),
            lifetimes: info.lifetimes.clone(),
            image_usage: info.image_usage_flags.clone(),
            buffer_usage: info.buffer_usage_flags.clone(),
            lifetime_events: info.lifetime_events.clone(),
            batches: batches.to_vec(),
            imports: definition
                .resources
                .iter()
                .enumerate()
                .filter_map(|(index, resource)| resource.is_external().then_some(index as u32))
                .collect(),
            exports,
        }
    }
    pub(crate) fn estimated_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self
                .pass_order
                .iter()
                .map(|name| std::mem::size_of::<String>() + name.capacity())
                .sum::<usize>()
            + self.pass_ranges.capacity() * std::mem::size_of::<Range<usize>>()
            + self.accesses.capacity() * std::mem::size_of::<ResourceAccessRecord>()
            + self.lifetimes.capacity() * std::mem::size_of::<ResourceLifetimeRecord>()
            + (self.image_usage.capacity() + self.buffer_usage.capacity() + self.imports.capacity())
                * std::mem::size_of::<u32>()
            + self.lifetime_events.capacity() * std::mem::size_of::<LifetimeEvent>()
            + self.batches.capacity() * std::mem::size_of::<batch_state::Batch>()
            + self.exports.capacity() * std::mem::size_of::<u16>()
    }
}
/// Immutable plan data is shared by frame receipts. Physical resource mappings
/// remain owned by the individual frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub shared: std::sync::Arc<SnapshotStatic>,
    pub physical_mapping: Vec<PhysicalMapping>,
    pub pool_hits: Vec<u32>,
}
impl std::ops::Deref for Snapshot {
    type Target = SnapshotStatic;
    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}
impl std::ops::DerefMut for Snapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        std::sync::Arc::make_mut(&mut self.shared)
    }
}
pub struct FrameInstance {
    pub plan: std::sync::Arc<super::plan::CompiledPlan>,
    pub declarations: GraphDeclaration,
    pub info: std::sync::Arc<ResourceInfo>,
    pub(crate) tables: FrameInstanceTables,
    storage: Option<super::frame_storage::FrameStorageLease>,
}
/// Read-only dynamic frame metadata. Capacity growth is admitted before the
/// frame is published; public callers cannot mutate or move pooled vectors.
pub struct FrameInstanceTables {
    pub registry: Registry,
    pub snapshot: Snapshot,
    pub(crate) history_validity: Option<Vec<Option<bool>>>,
}
impl std::ops::Deref for FrameInstance {
    type Target = FrameInstanceTables;
    fn deref(&self) -> &Self::Target {
        &self.tables
    }
}

impl Drop for FrameInstance {
    fn drop(&mut self) {
        if let Some(mut storage) = self.storage.take() {
            storage.return_tables(super::frame_storage::FrameTables {
                registry: std::mem::take(&mut self.tables.registry),
                physical_mapping: std::mem::take(&mut self.tables.snapshot.physical_mapping),
                pool_hits: std::mem::take(&mut self.tables.snapshot.pool_hits),
                history_validity: self.tables.history_validity.take().unwrap_or_default(),
            });
        }
    }
}

pub fn compile_graph(
    graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    dependency: &mut impl AccessDependency,
) -> crate::Result<FrameInstance> {
    let definition = super::plan::GraphDefinition::from_declaration(
        &graph,
        dependency,
        super::plan::PlanContext::default(),
    )?;
    let plan = std::sync::Arc::new(super::plan::compile_definition(definition)?);
    instantiate_validated_plan(plan, graph, pool, backend, dependency)
}

/// Bind physical resources and dynamic import states to an immutable plan.
/// The complete normalized declaration must match, even if its hash collides.
pub fn instantiate_plan(
    plan: std::sync::Arc<super::plan::CompiledPlan>,
    graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    dependency: &mut impl AccessDependency,
) -> crate::Result<FrameInstance> {
    if plan.definition.context.device_generation != 0
        && backend.device_generation().map(|g| g.value())
            != Some(plan.definition.context.device_generation)
    {
        return Err(crate::Error::invalid_usage(
            "compiled plan belongs to a different device generation",
        ));
    }
    let definition = super::plan::GraphDefinition::from_declaration(
        &graph,
        dependency,
        plan.definition.context,
    )?;
    if definition != plan.definition {
        return Err(crate::Error::invalid_usage(
            "frame declaration differs from compiled plan",
        ));
    }
    if super::validation_enabled() {
        super::plan_check::check_plan(&definition, &plan)?;
    }
    instantiate_validated_plan(plan, graph, pool, backend, dependency)
}

pub(crate) fn instantiate_validated_plan(
    plan: std::sync::Arc<super::plan::CompiledPlan>,
    graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    dependency: &mut impl AccessDependency,
) -> crate::Result<FrameInstance> {
    let storage = backend
        .frame_storage_pool()
        .map(|pool| pool.checkout(&plan))
        .transpose()?;
    instantiate_with_storage(plan, graph, pool, backend, dependency, storage)
}

pub(crate) fn instantiate_with_storage(
    plan: std::sync::Arc<super::plan::CompiledPlan>,
    mut graph: GraphDeclaration,
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
    _dependency: &mut impl AccessDependency,
    mut storage: Option<super::frame_storage::FrameStorageLease>,
) -> crate::Result<FrameInstance> {
    let mut original = 0usize;
    let mut retained = plan.retained_passes.iter().copied().peekable();
    graph.passes.retain_mut(|pass| {
        let index = original;
        original += 1;
        if retained.peek() != Some(&index) {
            return false;
        }
        retained.next();
        pass.accesses
            .copy_from_slice(&plan.definition.passes[index].accesses);
        true
    });
    backend.prepare_transient_resources(&plan, &graph)?;
    let mut tables = storage.as_mut().map_or_else(
        super::frame_storage::FrameTables::default,
        super::frame_storage::FrameStorageLease::take_tables,
    );
    // Public CPU backends without a recycler use the same allocation/failure
    // path. Vulkan has already admitted and reserved these exact table sizes.
    if storage.is_none() {
        tables.reserve_plan(&plan)?;
    }
    let info = plan.info.clone();
    let mut frame = FrameInstance {
        tables: FrameInstanceTables {
            snapshot: Snapshot {
                shared: plan.snapshot.clone(),
                physical_mapping: tables.physical_mapping,
                pool_hits: tables.pool_hits,
            },
            registry: tables.registry,
            history_validity: Some(tables.history_validity),
        },
        storage,
        plan,
        declarations: graph,
        info,
    };
    if let Err(error) = populate_resources_into(
        &frame.info,
        &frame.declarations.resources,
        pool,
        backend,
        &mut frame.tables.registry,
        &mut frame.tables.snapshot.pool_hits,
    ) {
        reclaim_failed_resources(
            &mut frame.tables.registry,
            &frame.declarations.resources,
            &frame.tables.snapshot.pool_hits,
            pool,
            backend,
        );
        return Err(error);
    }
    let registry = &mut frame.tables.registry;
    let validity = frame
        .tables
        .history_validity
        .as_mut()
        .expect("frame validity table");
    validity.resize(frame.declarations.resources.len(), None);
    for (logical, mapping) in registry.resources.iter().enumerate() {
        if let Some(payload) = registry
            .backing_resources
            .get(mapping.backing_index as usize)
            && let (_, Some(token)) = payload_projection(payload)
        {
            validity[logical] = backend.history_validity(token);
        }
    }
    for (logical, mapping) in registry.resources.iter().enumerate() {
        let (payload_tag, token) = registry
            .backing_resources
            .get(mapping.backing_index as usize)
            .map_or((255, None), payload_projection);
        frame
            .tables
            .snapshot
            .physical_mapping
            .push(PhysicalMapping {
                logical: logical as u32,
                physical: mapping.backing_index,
                reused: mapping.reused,
                initial_access: registry.resource_initial_access[logical],
                payload_tag,
                token,
            });
    }
    Ok(frame)
}

/// 失败路径归还：本图 Create 资源中取自资源池的放回资源池，
/// 本次新建的经 [`ResourceBackend::release_created_resource`] 交还后端；导入资源不动。
/// 已归还的物理槽置为 `Empty`，避免同一资源被再次取出。只能在 GPU 不会再使用这些资源时调用。
pub fn reclaim_failed_resources(
    registry: &mut Registry,
    declarations: &[ResourceDeclaration],
    pool_hits: &[u32],
    pool: &mut Pool,
    backend: &mut impl ResourceBackend,
) {
    for (logical, declaration) in declarations.iter().enumerate() {
        if !matches!(declaration.source, OriginSource::Create) {
            continue;
        }
        let Some(reuse) = registry.resources.get(logical) else {
            continue;
        };
        let Some(slot) = registry
            .backing_resources
            .get_mut(usize::from(reuse.backing_index))
        else {
            continue;
        };
        let pooled = pool_hits.contains(&(logical as u32));
        match std::mem::replace(slot, RegistryResource::Empty) {
            RegistryResource::OwnedTexture(texture) if pooled => pool.insert_texture(texture),
            RegistryResource::OwnedBuffer(buffer) if pooled => pool.insert_buffer(buffer),
            RegistryResource::OwnedTexture(Texture { handle, .. })
            | RegistryResource::OwnedBuffer(Buffer { handle, .. }) => {
                backend.release_created_resource(handle)
            }
            other => *slot = other,
        }
    }
}
pub fn payload_projection(resource: &RegistryResource<u64, u64, ()>) -> (u8, Option<u64>) {
    match resource {
        RegistryResource::OwnedTexture(texture) => (0, Some(texture.handle)),
        RegistryResource::UnownedTexture(texture) => (1, Some(texture.handle)),
        RegistryResource::OwnedBuffer(buffer) => (2, Some(buffer.handle)),
        RegistryResource::UnownedBuffer(buffer) => (3, Some(buffer.handle)),
        RegistryResource::Other(_) => (4, None),
        RegistryResource::Empty => (5, None),
    }
}
