//! 光栅 pass 的宿主接线：core 的 `PassHost`（`RasterPass`/`DrawBuilder` 链的外部调用面）+ `RgResources`
//! 落到 rhapsody 的渲染通道缓存、帧缓冲缓存、管线、描述符集与 `vkCmdDraw*`。
//!
//! 调用链（一个 `RasterPass::render()`）：
//! 1. `RasterPass::render` 构造 `RasterRenderPassDesc` → [`PassHost::render_pass_cache`] 未命中 →
//!    [`PassHost::create_render_pass`] = [`RenderPassHost::create_from_desc`]（`RenderPass::from_desc`，0x98B 键，按键去重），
//!    返回的 `ManagedHandle` 就是 `RasterPipelineKey.handle`；
//! 2. [`PassHost::submit_raster_pass`]：每个 `DrawPacket` 取 `RasterShaderProgram.variants[variant].get_or_generate`
//!    （键 = 通道句柄 + `packet.state` 4 字节位域）得到 [`RasterEntry`]；pass 的资源访问（颜色/深度/RG 缓冲与纹理）
//!    进 rhapsody 图声明；
//! 3. 图执行时的回调：`AttachmentCache::prepare`（视图 + 兼容通道 + 帧缓冲）→ `begin_attachments` →
//!    默认视口（负高度翻转 Y）+ stencil ref → 逐 draw：`vkCmdBindPipeline`（原版属于 `GraphicsDevice::bind_pipeline`，不是
//!    `RenderPassApi`）→ shared bindless heap + parameter slice（`DrawPacket.bindings` + RG 访问）→ `RenderPassApi::draw*`。
//!
//! 已核对（O）与未证实（U）的取舍都在对应函数注释里；未恢复的 draw 种类进入正式帧错误通道。
use crate::render_host::{
    BufferHost, RasterEntry, ShaderHost, TextureHost, compute_view_range, rh_view_desc,
    upsert_shader_resource,
};
use ash::vk;
use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    render::{
        pass_builder::{
            draw_stream::{
                AttachmentDesc, DescriptorSetBinding as CoreBinding, DrawKind, DrawPacket,
                DynamicConstantsBinding, PassHost, RasterPassColorAttachment, RasterPassDepth,
                RasterPassSubmission, RasterRenderPassDesc, RasterState,
                ShaderReflection as CoreReflection,
            },
            framebuffer_binding::{AttachmentTexture, DepthClear},
            pass_resource_access::RgTexture,
            rg_resources::{
                RgError, RgResources, TemporalBuffer, TemporalTexture, TemporalTexturePair,
            },
        },
        rg_helpers::rg_boundary::ResourceLease,
        shader::{
            RasterShaderProgram,
            boundary::{RasterPipelineKey, ShaderBackend},
        },
        ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc,
    },
    rhapsody_contract::{Handle, ManagedHandle, RenderPassBinding},
    rhapsody_types::TextureDesc as CoreTextureDesc,
};
use hashbrown::HashMap as HbMap;

/// `u64`-keyed table for ids that are already well-mixed hashes or counters
/// (xxh3 name hashes, handle keys): no SipHash on the per-draw lookups.
pub type IdMap<V> = std::collections::HashMap<
    u64,
    V,
    std::hash::BuildHasherDefault<rhapsody::collections::token_map::TokenHasher>,
>;
use parking_lot::Mutex;
use rhapsody::bindless::BindlessResource;
use rhapsody::{
    pipeline::RasterPipelineOpts, resource::DynamicConstants, resource::DynamicConstantsChunk,
    rg::AshCommandBackend, rg::Attachment, rg::AttachmentArray, rg::AttachmentCache,
    rg::BuildError, rg::DepthAccess, rg::ManagedPassResources, rg::RenderGraph as RhGraph,
    rg::RenderPassApi as RhRenderPassApi, rg::RenderPassAttachmentDesc as RhAttachmentDesc,
    rg::RenderPassDesc as RhRenderPassDesc, rg::ResourceHandle, rg::ResourceSpec,
    rg::VulkanGraphSession,
};
use shader_pipeline::reflection::BindingLocation;
use shader_pipeline::{
    compiled::CompiledShaderPipeline as RhCompiled, reflection::BindingNameHash,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

// ---------------------------------------------------------------- 图内资源声明

/// 在图里“创建”资源需要后端登记物理规格（`VulkanGraphBackend::declare`），而后端归会话所有。
/// 宿主不持有会话，由帧驱动方提供这个窄接口（对 `Rc<RefCell<VulkanGraphSession>>` 有现成实现）。
pub trait GraphDeclarer {
    fn import_buffer(
        &self,
        graph: &mut RhGraph,
        buffer: Arc<rhapsody::resource::Buffer>,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle>;
    fn import_texture(
        &self,
        graph: &mut RhGraph,
        texture: Arc<rhapsody::resource::Texture>,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle>;
    fn declare(
        &self,
        graph: &mut RhGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<ResourceHandle, BuildError>;
    fn named_texture(
        &self,
        graph: &mut RhGraph,
        name: &str,
        desc: rhapsody::resource::TextureDesc,
    ) -> rhapsody::Result<rhapsody::rg::NamedTexture>;
    fn temporal_texture(
        &self,
        graph: &mut RhGraph,
        name: &str,
        desc: rhapsody::resource::TextureDesc,
    ) -> rhapsody::Result<rhapsody::rg::TemporalTexturePair>;
    fn named_buffer(
        &self,
        graph: &mut RhGraph,
        name: &str,
        desc: rhapsody::resource::BufferDesc,
    ) -> rhapsody::Result<rhapsody::rg::NamedBuffer>;
}
impl GraphDeclarer for RefCell<VulkanGraphSession> {
    fn import_buffer(
        &self,
        graph: &mut RhGraph,
        buffer: Arc<rhapsody::resource::Buffer>,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        self.borrow_mut()
            .import_frame_buffer(graph, buffer, initial_access)
    }
    fn import_texture(
        &self,
        graph: &mut RhGraph,
        texture: Arc<rhapsody::resource::Texture>,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        self.borrow_mut()
            .import_frame_texture(graph, texture, initial_access)
    }
    fn declare(
        &self,
        graph: &mut RhGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<ResourceHandle, BuildError> {
        self.borrow_mut().backend.declare(graph, name, spec)
    }
    fn named_texture(
        &self,
        graph: &mut RhGraph,
        name: &str,
        desc: rhapsody::resource::TextureDesc,
    ) -> rhapsody::Result<rhapsody::rg::NamedTexture> {
        self.borrow_mut().named_texture(graph, name, desc)
    }
    fn temporal_texture(
        &self,
        graph: &mut RhGraph,
        name: &str,
        desc: rhapsody::resource::TextureDesc,
    ) -> rhapsody::Result<rhapsody::rg::TemporalTexturePair> {
        self.borrow_mut().temporal_texture(graph, name, desc)
    }
    fn named_buffer(
        &self,
        graph: &mut RhGraph,
        name: &str,
        desc: rhapsody::resource::BufferDesc,
    ) -> rhapsody::Result<rhapsody::rg::NamedBuffer> {
        self.borrow_mut().named_buffer(graph, name, desc)
    }
}

/// Core resource requests share the frame driver's real session-owned named slots.
pub struct GraphRgResources {
    graph: Rc<RefCell<RhGraph>>,
    declarer: Rc<dyn GraphDeclarer>,
    textures: Arc<TextureHost>,
}

impl RgResources for GraphRgResources {
    fn create_raw_texture(&mut self, name: &str, desc: &CoreTextureDesc) -> RgTexture {
        let rh = self
            .textures
            .convert_for_graph(desc)
            .unwrap_or_else(|e| panic!("create_raw_texture({name}): {e:?}"));
        let handle = self
            .declarer
            .declare(&mut self.graph.borrow_mut(), name, ResourceSpec::Image(rh))
            .unwrap_or_else(|e| panic!("RenderGraph::create_raw_resource({name}): {e:?}"));
        RgTexture {
            resource: handle.raw(),
            desc: *desc,
        }
    }

    fn create_raw_buffer(
        &mut self,
        name: &str,
        desc: &CoreBufferDesc,
    ) -> country_core::render::pass_builder::compute_pass_builder::RgBuffer {
        let handle = self
            .declarer
            .declare(
                &mut self.graph.borrow_mut(),
                name,
                ResourceSpec::Buffer(
                    BufferHost::convert(desc)
                        .unwrap_or_else(|e| panic!("create_raw_buffer({name}): {e}")),
                ),
            )
            .unwrap_or_else(|e| panic!("RenderGraph::create_raw_resource({name}): {e:?}"));
        country_core::render::pass_builder::compute_pass_builder::RgBuffer {
            resource: handle.raw(),
        }
    }

    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        desc: &CoreTextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        let native = self
            .textures
            .convert_for_graph(desc)
            .map_err(|error| RgError::msg(format!("invalid temporal texture: {error:?}")))?;
        let named = self
            .declarer
            .named_texture(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalTexture {
            texture: RgTexture {
                resource: named.texture.raw().raw(),
                desc: *desc,
            },
            // Same native history-valid polarity as the compute host.
            flag: u8::from(named.history_valid),
        })
    }
    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &CoreTextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        let native = self
            .textures
            .convert_for_graph(desc)
            .map_err(|error| RgError::msg(format!("invalid temporal texture: {error:?}")))?;
        let pair = self
            .declarer
            .temporal_texture(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalTexturePair {
            current: RgTexture {
                resource: pair.current().raw().raw(),
                desc: *desc,
            },
            previous: RgTexture {
                resource: pair.previous().raw().raw(),
                desc: *desc,
            },
            fresh: pair.history_valid(),
        })
    }
    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        desc: &CoreBufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        let named = self.declarer.named_buffer(
            &mut self.graph.borrow_mut(),
            name,
            BufferHost::convert(desc)?,
        )?;
        Ok(TemporalBuffer {
            buffer: country_core::render::pass_builder::compute_pass_builder::RgBuffer {
                resource: named.buffer.raw().raw(),
            },
            flag: u8::from(named.history_valid),
        })
    }
}

// ---------------------------------------------------------------- 共享状态

/// 资产库在 `PassHost` 上的几个只读查询（`AssetSsboLibrary` / `AssetTextureLibrary` / `PlacementDataSsboLibrary`）。
/// 这些库本身是 core 的世界资源，宿主在这里按需登记；没登记就按原版“库里没有该项”的路径失败。
#[derive(Default)]
pub struct AssetTables {
    /// Published query metadata; Collect holds one immutable owner per scope.
    /// Writers copy only while an earlier scope still retains its snapshot.
    pub collect: Arc<CollectAssets>,
    /// VAO refreshes do not copy the independent Collect query snapshot.
    vaos: Option<Arc<VaoSnapshotTable>>,
    wrapper_reuse: Option<Arc<VaoSnapshotTable>>,
}

#[derive(Clone, Default)]
pub struct CollectAssets {
    /// `SsboId`（`xxh3_64(NAME)`）→ 图里的缓冲资源下标。
    pub rg_buffers: IdMap<u32>,
    /// Metadata only: actual canonical entries and their original shared lazy
    /// import cache. A snapshot never imports the whole SSBO library.
    pub ssbo_leases:
        Option<IdMap<country_core::asset_libraries::ssbo_library::SsboGraphLease>>,
    /// Same-frame InstanceBuffer counts for original RT uniforms. These are
    /// CPU counts, never Vulkan or asset buffer identifiers.
    pub ssbo_instance_counts: IdMap<u32>,
    /// `GlobalId` → (缓冲句柄, flag)。
    pub placement: IdMap<(ManagedHandle, u32)>,
    /// `TextureFromDisk` 文件键 → (纹理句柄, 描述)。
    pub textures_by_name: IdMap<(ManagedHandle, AttachmentTexture)>,
    /// `TexturePreset` → RG 纹理。
    pub presets: HashMap<u8, AttachmentTexture>,
}
#[derive(Clone)]
struct VaoSnapshot {
    source: Arc<country_core::asset_libraries::vao_library::VaoEntry>,
    index: Arc<rhapsody::resource::Buffer>,
    index_count: u32,
    bindings: Vec<(Arc<str>, Arc<rhapsody::resource::Buffer>)>,
}

/// One immutable zero source per App, bounded by its largest requested extent.
/// Replacing it never writes storage that an earlier graph can still read.
#[derive(Default)]
struct ZeroVertexBuffer(Mutex<Option<Arc<rhapsody::resource::Buffer>>>);

impl ZeroVertexBuffer {
    fn get(
        &self,
        device: &Arc<rhapsody::device::Device>,
        bytes: u64,
    ) -> rhapsody::Result<Arc<rhapsody::resource::Buffer>> {
        if let Some(buffer) = self.0.lock().as_ref().filter(|buffer| buffer.desc.size >= bytes) {
            return Ok(buffer.clone());
        }
        let zeros = vec![0u8; usize::try_from(bytes)
            .map_err(|_| rhapsody::Error::invalid_usage("placeholder vertex size overflow"))?];
        // Allocate/upload outside the shared lock; publish only a complete owner.
        let created = device.create_buffer(
            &rhapsody::resource::BufferDesc::new(
                bytes,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                rhapsody::resource::MemoryLocation::GpuOnly,
            ),
            "Vertex_Color zero source",
            Some(zeros.as_slice()),
        )?.shared();
        let mut cached = self.0.lock();
        if let Some(buffer) = cached.as_ref().filter(|buffer| buffer.desc.size >= bytes) {
            return Ok(buffer.clone());
        }
        *cached = Some(created.clone());
        Ok(created)
    }
}

#[cfg(test)]
mod zero_vertex_tests {
    use super::*;

    #[test]
    fn zero_vertex_owner_reuses_capacity_and_preserves_the_old_extent() {
        let device = match rhapsody::device::Device::new_headless(
            &rhapsody::device::DeviceConfig::headless(),
        ) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict zero vertex GPU initialization: {error}");
                return;
            }
        };
        let cache = ZeroVertexBuffer::default();
        let first = cache.get(&device, 64).unwrap();
        for size in [16, 64, 32, 64] {
            assert!(Arc::ptr_eq(&first, &cache.get(&device, size).unwrap()));
        }
        let retained = Arc::downgrade(&first);
        let larger = cache.get(&device, 128).unwrap();
        assert!(!Arc::ptr_eq(&first, &larger));
        assert_eq!(first.desc.size, 64);
        assert_eq!(larger.desc.size, 128);
        assert!(Arc::ptr_eq(&larger, &cache.get(&device, 48).unwrap()));
        drop(first);
        assert!(retained.upgrade().is_none());
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct VaoScopeState {
    /// Address of the `DrawLists` the snapshot was taken for; only compared while
    /// the scope is open, so a recycled address cannot match across frames.
    lists: usize,
    library_revision: u64,
    table_revision: u64,
}

/// One physical owner for the publication's actual map capacity and Arc data.
/// Reuse guards clone this owner without constructing another map.
struct VaoSnapshotTable {
    entries: IdMap<Arc<VaoSnapshot>>,
    // Storage is freed before budget credit is returned.
    _memory: rhapsody::device::CpuStorageReservation,
}

impl VaoSnapshotTable {
    fn metadata_bytes() -> u64 {
        let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
            .extend(std::alloc::Layout::new::<Self>()).expect("VAO table Arc layout");
        layout.pad_to_align().size() as u64
    }

    fn map_bytes(capacity: usize) -> u64 {
        if capacity == 0 { return 0; }
        capacity.saturating_add(1).checked_next_power_of_two().unwrap_or(usize::MAX)
            .saturating_mul(std::mem::size_of::<(u64, Arc<VaoSnapshot>)>() + 1)
            .saturating_add(16) as u64
    }
}

/// A synchronous world declaration may temporarily publish a terrain subset.
/// This guard keeps its preceding full wrappers available as *candidates* only:
/// every candidate still needs current typed and physical-owner validation.
pub struct VaoWrapperReuseScope {
    shared: Arc<RasterShared>,
    active: bool,
    _owner: std::marker::PhantomData<Rc<()>>,
}
impl Drop for VaoWrapperReuseScope {
    fn drop(&mut self) {
        if self.active {
            let previous = self.shared.assets.lock().wrapper_reuse.take();
            drop(previous);
        }
    }
}

/// Closes the reuse window opened by [`RasterShared::begin_vao_scope`] on drop.
pub struct VaoScope(Arc<RasterShared>);
impl Drop for VaoScope {
    fn drop(&mut self) {
        *self.0.vao_scope.lock() = None;
    }
}

/// 全部光栅 pass 共享的宿主状态。
pub struct RasterShared {
    pub buffers: Arc<BufferHost>,
    frame_buffers: Mutex<Option<Arc<BufferHost>>>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
    device: Arc<rhapsody::device::Device>,
    attachments: Mutex<AttachmentCache>,
    programs: Arc<AssetShaderLibrary>,
    reflections: Mutex<crate::draw_template_cache::ReflectionCache>,
    constant_pool: Arc<crate::draw_constant_pool::ConstantPool>,
    commit_scratch: country_core::render::pass_builder::draw_commit_scratch::DrawCommitScratchPool,
    #[cfg(test)]
    fail_constant_write: std::sync::atomic::AtomicUsize,
    zero_vertex: ZeroVertexBuffer,
    collect_chunk_draws: usize,
    vao_refresh_timing: Mutex<VaoRefreshTiming>,
    /// Bumped every time the VAO snapshot table is replaced.
    vao_table_revision: std::sync::atomic::AtomicU64,
    vao_snapshot_allocations: std::sync::atomic::AtomicU64,
    #[cfg(feature = "cpu-probe")]
    management_counts: [std::sync::atomic::AtomicU64; 6],
    #[cfg(feature = "cpu-probe")]
    management_modes: [bool; 3],
    /// The global texture table's (resource, range) reads for the graph being
    /// declared. Every raster pass of one graph reads the same table, so it is
    /// imported and range-declared once per graph instead of once per pass.
    table_reads: Mutex<Option<(rhapsody::rg::GraphId, Arc<[(ResourceHandle, rhapsody::rg::AccessRange)]>)>>,
    /// The frame-constants block's import for the graph being declared; every
    /// draw of a graph reads the same block, so it is imported once per graph.
    frame_constants: Mutex<Option<(rhapsody::rg::GraphId, (ResourceHandle, rhapsody::rg::AccessRange))>>,
    /// Open while one synchronous world-graph declaration reuses a single full snapshot.
    vao_scope: Mutex<Option<VaoScopeState>>,
    dynamic: Arc<Mutex<DynamicConstants>>,
    pub assets: Mutex<AssetTables>,
}

/// Cumulative opt-in CPU wall time for full draw-list VAO refreshes on this App.
/// Counts include failed attempts; direct subset refreshes are not counted here.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct VaoRefreshTiming {
    pub calls: u64,
    pub list_entries: u64,
    pub queried_entities: u64,
    pub unique_vaos: u64,
    pub elapsed_ns: u64,
}

impl VaoRefreshTiming {
    /// Compare snapshots from the same RasterShared owner only.
    pub fn delta_since(self, before: Self) -> Self {
        Self {
            calls: self.calls.saturating_sub(before.calls),
            list_entries: self.list_entries.saturating_sub(before.list_entries),
            queried_entities: self.queried_entities.saturating_sub(before.queried_entities),
            unique_vaos: self.unique_vaos.saturating_sub(before.unique_vaos),
            elapsed_ns: self.elapsed_ns.saturating_sub(before.elapsed_ns),
        }
    }
}

struct VaoRefreshMeasurement<'a> {
    totals: &'a Mutex<VaoRefreshTiming>,
    started: std::time::Instant,
    counts: VaoRefreshTiming,
}

impl Drop for VaoRefreshMeasurement<'_> {
    fn drop(&mut self) {
        // Drop also observes an early Err without altering its error or cleanup.
        let elapsed_ns = self.started.elapsed().as_nanos().min(u64::MAX as u128) as u64;
        let mut totals = self.totals.lock();
        totals.calls = totals.calls.saturating_add(1);
        totals.list_entries = totals.list_entries.saturating_add(self.counts.list_entries);
        totals.queried_entities = totals.queried_entities.saturating_add(self.counts.queried_entities);
        totals.unique_vaos = totals.unique_vaos.saturating_add(self.counts.unique_vaos);
        totals.elapsed_ns = totals.elapsed_ns.saturating_add(elapsed_ns);
    }
}

/// Resolve actual typed handles once per unique VAO, using the entity already
/// carried by the list builder. Legacy explicit lists keep the original scan
/// fallback; native host/generation validation still happens after this step.
fn draw_vao_handles(
    world: &mut bevy_ecs::world::World,
    lists: &country_core::systems::render_loop::draw_passes::draw_lists::DrawLists,
    typed_lookup: bool,
    mut timing: Option<&mut VaoRefreshTiming>,
) -> rhapsody::Result<Vec<country_core::asset_libraries::Handle<country_core::render::vao::Vao>>> {
    use country_core::asset_libraries::Handle as AssetHandle;
    use country_core::render::vao::Vao;
    #[cfg(not(feature = "cpu-probe"))]
    let typed_lookup = { let _ = typed_lookup; true };
    let mut wanted = std::collections::BTreeSet::new();
    let mut handles = HashMap::new();
    for index in 0..country_core::systems::render_loop::draw_passes::draw_lists::DRAW_LIST_COUNT {
        for (item, _) in lists.list(index) {
            if let Some(counts) = timing.as_mut() { counts.list_entries += 1; }
            if wanted.insert(item.vao) && typed_lookup {
                if let Some(counts) = timing.as_mut() { counts.queried_entities += 1; }
                if let Some(handle) = world.get::<AssetHandle<Vao>>(item.params.entity)
                    && handle.id().raw() == item.vao {
                    handles.insert(item.vao, *handle);
                }
            }
        }
    }
    if let Some(counts) = timing.as_mut() { counts.unique_vaos = wanted.len() as u64; }
    if handles.len() != wanted.len() {
        let mut query = world.query::<&AssetHandle<Vao>>();
        for handle in query.iter(world) {
            if let Some(counts) = timing.as_mut() { counts.queried_entities += 1; }
            if wanted.contains(&handle.id().raw()) {
                handles.entry(handle.id().raw()).or_insert(*handle);
            }
        }
    }
    wanted.into_iter().map(|id| handles.get(&id).copied().ok_or_else(||
        rhapsody::Error::invalid_usage("draw-list VAO has no matching typed ECS handle")
    )).collect()
}

#[cfg(test)]
mod vao_handle_resolution_tests {
    use super::*;
    use bevy_ecs::world::World;
    use country_core::{
        asset_libraries::Handle as AssetHandle,
        render::{vao::Vao, pass_builder::{draw_builder::DrawBuilder, draw_list::EntityDrawParams}},
        systems::render_loop::draw_passes::draw_lists::{DrawItem, DrawLists, DRAW_LIST_COUNT},
    };

    fn unused(_: DrawBuilder<'_>, _: &EntityDrawParams, _: &World) {}

    #[test]
    fn production_entities_resolve_unique_vaos_without_scanning_unrelated_entities() {
        let mut world = World::new();
        for _ in 0..1000 { world.spawn(AssetHandle::<Vao>::new()); }
        let handle = AssetHandle::<Vao>::new();
        let entity = world.spawn(handle).id();
        let mut lists = DrawLists::new();
        for list in 0..DRAW_LIST_COUNT {
            for _ in 0..64 {
                let mut item = DrawItem::new(1, 0);
                item.vao = handle.id().raw();
                item.params.entity = entity;
                lists.list_mut(list).push((item, unused));
            }
        }
        let mut counts = VaoRefreshTiming::default();
        assert_eq!(draw_vao_handles(&mut world, &lists, true, Some(&mut counts)).unwrap(), [handle]);
        assert_eq!(counts.list_entries, (DRAW_LIST_COUNT * 64) as u64);
        assert_eq!(counts.unique_vaos, 1);
        assert_eq!(counts.queried_entities, 1, "one typed lookup rather than 1001-row ECS scan");
    }

    #[test]
    fn explicit_lists_keep_scan_fallback_and_missing_typed_handles_stay_errors() {
        let mut world = World::new();
        let handle = AssetHandle::<Vao>::new();
        world.spawn(handle);
        let mut lists = DrawLists::new();
        let mut item = DrawItem::new(1, 0);
        item.vao = handle.id().raw();
        lists.list_mut(0).push((item, unused));
        let mut counts = VaoRefreshTiming::default();
        assert_eq!(draw_vao_handles(&mut world, &lists, true, Some(&mut counts)).unwrap(), [handle]);
        assert_eq!(counts.queried_entities, 2, "failed entity lookup plus legacy scan");
        let invalid = AssetHandle::<Vao>::new();
        lists.list_mut(0)[0].0.vao = invalid.id().raw();
        assert!(draw_vao_handles(&mut world, &lists, true, None).is_err());
    }

    #[test]
    fn stale_entity_does_not_alias_a_different_live_vao_and_results_stay_sorted() {
        let mut world = World::new();
        let first = AssetHandle::<Vao>::new();
        let second = AssetHandle::<Vao>::new();
        world.spawn(first);
        let second_entity = world.spawn(second).id();
        let mut lists = DrawLists::new();
        for handle in [second, first, second] {
            let mut item = DrawItem::new(1, 0);
            item.vao = handle.id().raw();
            item.params.entity = second_entity;
            lists.list_mut(0).push((item, unused));
        }
        assert_eq!(draw_vao_handles(&mut world, &lists, true, None).unwrap(), [first, second]);
        world.despawn(second_entity);
        assert!(draw_vao_handles(&mut world, &lists, true, None).is_err());
    }
}

impl RasterShared {
    pub(crate) fn set_frame_buffers(&self, buffers: Option<Arc<BufferHost>>) {
        *self.frame_buffers.lock() = buffers;
    }

    fn buffer_host(&self) -> Arc<BufferHost> {
        self.frame_buffers.lock().as_ref().unwrap_or(&self.buffers).clone()
    }
    #[inline]
    fn management_mode(&self, index: usize) -> bool {
        #[cfg(feature = "cpu-probe")]
        { self.management_modes[index] }
        #[cfg(not(feature = "cpu-probe"))]
        { let _ = index; true }
    }
    /// Constructor-frozen diagnostic modes: reflection scope, typed VAO lookup,
    /// scoped VAO wrapper reuse. Does not change the original 13 counter fields.
    pub fn draw_management_modes(&self) -> [bool; 3] {
        [self.management_mode(0), self.management_mode(1), self.management_mode(2)]
    }

    #[cfg(feature = "cpu-probe")]
    pub fn draw_scratch_bill(&self) -> serde_json::Value {
        let bill = self.commit_scratch.stats();
        serde_json::json!({
            "checked_out": bill.checked_out, "cached": bill.cached,
            "live_bytes": bill.live_bytes, "cached_bytes": bill.cached_bytes,
            "metadata_bytes": bill.metadata_bytes, "high_water_bytes": bill.high_water_bytes,
            "checkouts_high_water": bill.checkouts_high_water, "creations": bill.creations,
            "capacity_growths": bill.capacity_growths, "pool_capacity_stalls": bill.pool_capacity_stalls,
            "checkouts": bill.checkouts,
            "pool_capacity_stalls_scope": "pool_slots_or_local_capacity_only",
            "global_host_budget_refusals_included": false,
        })
    }

    pub fn constant_batch_enabled(&self) -> bool { self.constant_pool.batch_enabled() }

    #[cfg(feature = "cpu-probe")]
    pub fn constant_pool_bill(&self) -> serde_json::Value {
        let bill = self.constant_pool.bill();
        serde_json::json!({
            "batch_constants": bill.batch_constants, "lock_acquisitions": bill.lock_acquisitions,
            "scanned_idle_entries": bill.scanned_idle_entries, "batch_checkouts": bill.batch_checkouts,
            "batch_returns": bill.batch_returns, "idle_blocks": bill.idle_blocks,
            "planned_tasks": bill.planned_tasks, "unplanned_tasks": bill.unplanned_tasks,
            "planned_cache_misses": bill.planned_cache_misses,
            "unused_stock_blocks_returned": bill.unused_stock_blocks_returned,
            "unused_stock_bytes_returned": bill.unused_stock_bytes_returned,
            "idle_bytes": bill.idle_bytes, "metadata_bytes": bill.metadata_bytes,
            "physical_allocations": bill.physical_allocations,
            "physical_allocated_bytes": bill.physical_allocated_bytes,
            "physical_allocation_scope": "successful_nonempty_cpu_bytes_through_constant_pool_not_malloc_calls",
            "duration_estimates": false,
        })
    }

    fn write_collected_constants(&self, chunk: &DynamicConstantsChunk, bytes: &[u8]) -> rhapsody::Result<()> {
        #[cfg(test)] {
            let prior = self.fail_constant_write.fetch_update(
                std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed,
                |value| value.checked_sub(1));
            if matches!(prior, Ok(1)) { return Err(rhapsody::Error::OutOfHostMemory); }
        }
        chunk.write_bytes(bytes)
    }

    fn count_management(&self, index: usize, amount: u64) {
        #[cfg(feature = "cpu-probe")]
        if crate::render_cpu_probe::management_enabled() {
            self.management_counts[index].fetch_add(amount, std::sync::atomic::Ordering::Relaxed);
        }
        #[cfg(not(feature = "cpu-probe"))]
        let _ = (index, amount);
    }
    /// App-owned cumulative counts. Diagnostic-only; serialization stays outside frames.
    #[cfg(feature = "cpu-probe")]
    pub fn draw_management_counts(&self) -> [u64; 13] {
        let mut result = [0; 13];
        for (index, counter) in self.management_counts.iter().enumerate() {
            result[index] = counter.load(std::sync::atomic::Ordering::Relaxed);
        }
        let vao = self.vao_refresh_timing();
        result[6] = vao.calls; result[7] = vao.list_entries;
        result[8] = vao.queried_entities; result[9] = vao.unique_vaos;
        result[10] = self.vao_table_revision(); result[11] = self.vao_snapshot_allocations();
        result[12] = self.reflections.lock().diagnostic_len() as u64;
        result
    }
    fn reflection(&self, shader: u64, variant: u8) -> Option<Arc<CoreReflection>> {
        self.count_management(0, 1);
        let key = (shader, variant);
        if let Some(hit) = self.reflections.lock().get(&key) { self.count_management(1, 1); return Some(hit); }
        let reflection = core_reflection(self.programs.get_raster_by_id(shader)?, variant)?;
        self.reflections.lock().insert(key, reflection.clone());
        Some(reflection)
    }

    /// Snapshot diagnostic totals; this never clears another frame's measurements.
    pub(crate) fn vao_refresh_timing(&self) -> VaoRefreshTiming {
        *self.vao_refresh_timing.lock()
    }

    /// Snapshot only handles actually present in the original draw lists. The
    /// typed ECS handle is resolved through the live AssetVAOLibrary, and every
    /// buffer is resolved through this App's exact physical host table.
    pub fn refresh_draw_vaos(
        &self,
        world: &mut bevy_ecs::world::World,
        lists: &country_core::systems::render_loop::draw_passes::draw_lists::DrawLists,
    ) -> rhapsody::Result<()> {
        let timing_enabled = crate::game_loop::frame_timing_enabled();
        #[cfg(feature = "cpu-probe")]
        let timing_enabled = timing_enabled || crate::render_cpu_probe::management_enabled();
        let mut timing = timing_enabled.then(|| VaoRefreshMeasurement {
            totals: &self.vao_refresh_timing,
            started: std::time::Instant::now(),
            counts: VaoRefreshTiming::default(),
        });
        let handles = draw_vao_handles(world, lists, self.management_mode(1),
            timing.as_mut().map(|measurement| &mut measurement.counts))?;
        self.refresh_vao_handles(world, &handles)
    }

    /// Snapshot actual typed VAO owners used by a direct-entity raster pass.
    /// This shares the same physical buffer validation as list-driven draws.
    pub fn refresh_vao_handles(
        &self,
        world: &bevy_ecs::world::World,
        handles: &[country_core::asset_libraries::Handle<country_core::render::vao::Vao>],
    ) -> rhapsody::Result<()> {
        use country_core::asset_libraries::vao_library::AssetVAOLibrary;
        let vaos = world.get_resource::<AssetVAOLibrary>().ok_or_else(|| {
            rhapsody::Error::invalid_usage("AssetVAOLibrary producer has not run")
        })?;
        // Reserve the conservative growth bound before allocating the map.
        // Duplicated handles can only reduce its actual unique entry count.
        let predicted = if handles.is_empty() { 0 } else { handles.len().saturating_mul(2).max(3) };
        let reserve = || self.device.allocator_stats().reserve_cpu_storage(
            VaoSnapshotTable::metadata_bytes().saturating_add(VaoSnapshotTable::map_bytes(predicted)));
        let mut table_memory = match reserve() {
            Ok(memory) => memory,
            Err(error) => {
                // Optional wrapper reuse must not require a third table's
                // permanent headroom. Release only our saved cache owner,
                // outside the lock, before one ordinary admission retry.
                // No local backup Arc has been taken at this point.
                let backup = self.assets.lock().wrapper_reuse.take();
                if backup.is_none() { return Err(error); }
                drop(backup);
                reserve()?
            }
        };
        let mut snapshots = IdMap::default();
        snapshots.try_reserve(handles.len()).map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        table_memory.resize(VaoSnapshotTable::metadata_bytes()
            .saturating_add(VaoSnapshotTable::map_bytes(snapshots.capacity())))?;
        let (current_table, reusable) = {
            let assets = self.assets.lock();
            (assets.vaos.clone(), assets.wrapper_reuse.clone())
        };
        for handle in handles {
            let id = handle.id().raw();
            let entry = vaos.shared_by_handle(*handle).ok_or_else(|| {
                rhapsody::Error::invalid_usage("draw-list VAO is not live in this App")
            })?;
            let index = self.buffer_host()
                .managed_buffer(&entry.vao.index_buffer)
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "VAO index buffer belongs to another host or is stale",
                    )
                })?
                .shared();
            let index_count = u32::try_from(entry.index_count).map_err(|_| {
                rhapsody::Error::invalid_usage("VAO index count exceeds Vulkan u32")
            })?;
            if u64::from(index_count) * 4 > index.desc.size {
                return Err(rhapsody::Error::invalid_usage(
                    "VAO index count exceeds its physical buffer",
                ));
            }
            // A published VAO entry is immutable. Reuse the complete native
            // view, while still checking live host generations: an externally
            // released buffer must not silently become valid through the cache.
            let matches = |snapshot: &&Arc<VaoSnapshot>|
                Arc::ptr_eq(&snapshot.source, entry) && Arc::ptr_eq(&snapshot.index, &index);
            let cached = current_table.as_ref()
                .and_then(|table| table.entries.get(&id)).filter(matches).cloned()
                .or_else(|| reusable.as_ref()
                    .and_then(|table| table.entries.get(&id)).filter(matches).cloned());
            if let Some(cached) = cached {
                let mut same_versions = cached.bindings.len() == entry.vao.bindings.len();
                for (binding, (_, owner)) in entry.vao.bindings.iter().zip(&cached.bindings) {
                    let current = self.buffer_host().managed_buffer(&binding.buffer)
                        .ok_or_else(|| rhapsody::Error::invalid_usage(
                            "VAO attribute buffer belongs to another host or is stale"))?;
                    same_versions &= Arc::ptr_eq(owner, &current.shared());
                }
                if same_versions {
                    snapshots.insert(id, cached);
                    continue;
                }
            }
            let bindings = entry
                .vao
                .bindings
                .iter()
                .map(|binding| {
                    self.buffer_host()
                        .managed_buffer(&binding.buffer)
                        .map(|buffer| (binding.name.clone(), buffer.shared()))
                        .ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "VAO attribute buffer belongs to another host or is stale",
                            )
                        })
                })
                .collect::<rhapsody::Result<Vec<_>>>()?;
            self.vao_snapshot_allocations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            snapshots.insert(
                id,
                Arc::new(VaoSnapshot {
                    source: entry.clone(),
                    index,
                    index_count,
                    bindings,
                }),
            );
        }
        // Move old owners out before releasing them: final leases may retire
        // native objects and must not extend the asset-table critical section.
        let table = Arc::new(VaoSnapshotTable { entries: snapshots, _memory: table_memory });
        let previous = std::mem::replace(&mut self.assets.lock().vaos, Some(table));
        drop(previous);
        self.vao_table_revision
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(())
    }

    /// Keep only this declaration's preceding full wrappers across a temporary
    /// terrain publication. Never bypasses actual physical identity checks.
    /// The guard must not outlive one synchronous world-graph declaration.
    pub fn preserve_vao_wrappers(self: &Arc<Self>) -> Option<VaoWrapperReuseScope> {
        #[cfg(feature = "cpu-probe")]
        if !self.management_mode(2) { return None; }
        let mut assets = self.assets.lock();
        // A nested guard never replaces or extends the outer owner. Dropping
        // guards out of order can only stop reuse early, never retain old data.
        let active = assets.wrapper_reuse.is_none() && assets.vaos.is_some();
        if active { assets.wrapper_reuse = assets.vaos.clone(); }
        Some(VaoWrapperReuseScope { shared: self.clone(), active, _owner: std::marker::PhantomData })
    }

    /// Number of times the VAO snapshot table was replaced (diagnostics/tests).
    #[doc(hidden)]
    pub fn vao_table_revision(&self) -> u64 {
        self.vao_table_revision
            .load(std::sync::atomic::Ordering::Acquire)
    }

    /// Actual immutable view allocations, independent of table publications.
    #[doc(hidden)]
    pub fn vao_snapshot_allocations(&self) -> u64 {
        self.vao_snapshot_allocations.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn vao_scope_state(
        &self,
        world: &bevy_ecs::world::World,
        lists: &country_core::systems::render_loop::draw_passes::draw_lists::DrawLists,
    ) -> Option<VaoScopeState> {
        let library = world.get_resource::<country_core::asset_libraries::vao_library::AssetVAOLibrary>()?;
        Some(VaoScopeState {
            lists: lists as *const _ as usize,
            library_revision: library.revision(),
            table_revision: self.vao_table_revision.load(std::sync::atomic::Ordering::Acquire),
        })
    }

    /// Takes one full draw-list VAO snapshot and keeps it valid for the guard's
    /// lifetime: [`Self::ensure_draw_vaos`] then skips the refresh while the same
    /// draw lists are used and neither the VAO library nor the snapshot table
    /// changed. Any direct `refresh_vao_handles` (for example the terrain readback)
    /// replaces the table and so ends the reuse until the next full refresh.
    /// Must not outlive the synchronous declaration of one frame.
    pub fn begin_vao_scope(
        self: &Arc<Self>,
        world: &mut bevy_ecs::world::World,
        lists: &country_core::systems::render_loop::draw_passes::draw_lists::DrawLists,
    ) -> rhapsody::Result<VaoScope> {
        self.refresh_draw_vaos(world, lists)?;
        *self.vao_scope.lock() = self.vao_scope_state(world, lists);
        Ok(VaoScope(self.clone()))
    }

    /// Same result as [`Self::refresh_draw_vaos`], skipping the work when the
    /// open [`VaoScope`] proves the snapshot is still exact. Never uses a stale
    /// table: any mismatch falls back to the full refresh.
    pub fn ensure_draw_vaos(
        &self,
        world: &mut bevy_ecs::world::World,
        lists: &country_core::systems::render_loop::draw_passes::draw_lists::DrawLists,
    ) -> rhapsody::Result<()> {
        let open = *self.vao_scope.lock();
        if let Some(open) = open
            && self.vao_scope_state(world, lists) == Some(open)
        {
            return Ok(());
        }
        self.refresh_draw_vaos(world, lists)?;
        if open.is_some() {
            // The snapshot is exact again: re-arm the open scope on it.
            *self.vao_scope.lock() = self.vao_scope_state(world, lists);
        }
        Ok(())
    }
    /// The same dynamic-constant allocator is read by PassResources::new_from
    /// and written by the raster callback. It must not be replaced per pass.
    pub fn dynamic_constants(&self) -> &Mutex<DynamicConstants> {
        &self.dynamic
    }

    /// 只克隆本 App 已有 owner；禁止把另一个设备的常量页接入当前 session。
    pub(crate) fn dynamic_constants_owner(
        &self,
        expected_device: &Arc<rhapsody::device::Device>,
    ) -> rhapsody::Result<Arc<Mutex<DynamicConstants>>> {
        if !Arc::ptr_eq(&self.device, expected_device) {
            return Err(rhapsody::Error::invalid_usage(
                "App constants belong to another device",
            ));
        }
        Ok(self.dynamic.clone())
    }

    /// Resolve the game world's real shader handles in its authoritative library.
    /// Programs and their pipeline caches remain in the library, without cloning
    /// or assigning a second renderer-local identity.
    pub fn new_with_shader_library(
        device: Arc<rhapsody::device::Device>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
        shaders: Arc<ShaderHost>,
        library: Arc<AssetShaderLibrary>,
    ) -> Result<Arc<Self>, rhapsody::Error> {
        let commit_scratch = country_core::render::pass_builder::draw_commit_scratch::DrawCommitScratchPool::new(
            device.allocator_stats().clone(), 8, 8 << 20, 1 << 20,
        )?;
        #[cfg(feature = "cpu-probe")]
        commit_scratch.set_billing(crate::render_cpu_probe::management_enabled());
        let constant_pool = crate::draw_constant_pool::ConstantPool::new(device.allocator_stats().clone())?;
        let evictable: Arc<dyn rhapsody::device::Evictable> = constant_pool.clone();
        device.register_evictable(Arc::downgrade(&evictable), rhapsody::device::EvictionPriority::Transient);
        Ok(Arc::new(Self {
            attachments: Mutex::new(AttachmentCache::new(device.clone())?),
            dynamic: Arc::new(Mutex::new(DynamicConstants::new(device.clone())?)),
            device,
            buffers,
            frame_buffers: Mutex::new(None),
            textures,
            shaders,
            programs: library,
            reflections: Mutex::new(Default::default()),
            constant_pool,
            commit_scratch,
            #[cfg(test)]
            fail_constant_write: Default::default(),
            zero_vertex: ZeroVertexBuffer::default(),
            collect_chunk_draws: std::env::var("TG_DRAW_COLLECT_CHUNK").ok()
                .and_then(|value| value.parse::<usize>().ok()).unwrap_or(64).clamp(1, 4096),
            vao_refresh_timing: Mutex::new(VaoRefreshTiming::default()),
            vao_table_revision: std::sync::atomic::AtomicU64::new(0),
            vao_snapshot_allocations: std::sync::atomic::AtomicU64::new(0),
            #[cfg(feature = "cpu-probe")]
            management_counts: [const { std::sync::atomic::AtomicU64::new(0) }; 6],
            #[cfg(feature = "cpu-probe")]
            management_modes: ["TG_REFLECTION_SCOPE", "TG_VAO_TYPED_LOOKUP", "TG_VAO_WRAPPER_REUSE"]
                .map(|name| std::env::var(name).ok().is_none_or(|value| value != "0")),
            table_reads: Mutex::new(None),
            frame_constants: Mutex::new(None),
            vao_scope: Mutex::new(None),
            assets: Mutex::new(AssetTables::default()),
        }))
    }

    /// 清掉帧缓冲/视图缓存（附件纹理被销毁/缩放后调用；对应会话的 `collect_after_completion` 之前）。
    pub fn clear_attachment_cache(&self) {
        self.attachments.lock().clear();
    }

    /// 一个 pass 一个宿主句柄（`RasterPass` 按 `Box<dyn PassHost>` 拥有它）。
    pub fn pass_host(
        self: &Arc<Self>,
        graph: Rc<RefCell<RhGraph>>,
        declarer: Option<Rc<dyn GraphDeclarer>>,
    ) -> RasterPassHost {
        RasterPassHost {
            shared: self.clone(),
            rg: declarer.map(|declarer| GraphRgResources {
                graph: graph.clone(),
                declarer,
                textures: self.textures.clone(),
            }),
            graph,
            cache: HbMap::new(),
            scope_reflections: RefCell::new(None),
            constants_error: None,
            resource_graph_factory: None,
        }
    }
}

// ---------------------------------------------------------------- 描述转换

/// core 的 `RasterRenderPassDesc`（`RasterPass::render` 构造）→ rhapsody 的 0x98B `RenderPassDesc` 键。
///
/// 颜色附件的 `+4`（原 `clear_input` 字，即 `loadOp`）在 core 里被覆盖为附件 tag：tag0 → LOAD、tag1 → CLEAR、
/// tag2 → DONT_CARE（O：`render` 里的覆盖写；`vkCreateRenderPass` 原样用该字作 loadOp，0x1418b8c50）。
/// 深度：`depth_state` 字节 3 = 无深度，其余按 `DepthAccess` 跳表（0 只读 / 1 读写 / 2 深度写+模板读）。
pub fn rh_render_pass_desc(desc: &RasterRenderPassDesc) -> rhapsody::Result<RhRenderPassDesc> {
    let attachment = |a: &AttachmentDesc, load: u32| RhAttachmentDesc {
        format: vk::Format::from_raw(a.format as i32),
        clear_input: load,
        store_op: 0,
        sample_count: a.sample_count,
    };
    let input_colors = desc
        .colors
        .get(..desc.color_count as usize)
        .ok_or_else(|| rhapsody::Error::invalid_usage("光栅通道颜色附件数量超过固定数组容量"))?;
    let colors =
        AttachmentArray::try_from_iter(input_colors.iter().map(|a| attachment(a, a.kind_tag)))?;
    let access = DepthAccess::from_tag(desc.depth_state)
        .ok_or_else(|| rhapsody::Error::invalid_usage("光栅通道深度访问标记无效"))?;
    let depth = access.map(|access| {
        (
            attachment(&desc.depth, u32::from(desc.depth.clear_input)),
            access,
        )
    });
    Ok(RhRenderPassDesc { colors, depth })
}

fn core_reflection(program: &RasterShaderProgram, variant: u8) -> Option<Arc<CoreReflection>> {
    let compiled = &program.variants.get(variant as usize)?.compiled;
    let decoded = RhCompiled::decode(&compiled.0).ok()?;
    let reflection = &decoded.reflection;
    Some(Arc::new(CoreReflection {
        program_name: program.name,
        bindings: reflection
            .binding_name_hashes
            .iter()
            .map(|(hash, location)| (*hash, (location.set, location.binding)))
            .collect(),
        has_uniform_block: reflection.uniform_block.is_some(),
        uniform_offsets: reflection
            .uniform_block
            .iter()
            .flat_map(|block| block.name_hash_offsets.iter().copied())
            .collect(),
        uniform_block_size: reflection.uniform_block.as_ref().map_or(0, |b| b.byte_size),
        uniform_block_binding: reflection
            .uniform_block
            .as_ref()
            .map_or(0, |b| b.binding.binding),
    }))
}

// ---------------------------------------------------------------- PassHost

/// 一个 `RasterPass` 的宿主句柄。
pub struct RasterPassHost {
    shared: Arc<RasterShared>,
    graph: Rc<RefCell<RhGraph>>,
    rg: Option<GraphRgResources>,
    /// 设备级 `HashMap<RenderPassDesc, RenderPass>` 缓存（`+0x188`）的每 pass 视图：去重由
    /// [`RenderPassHost::create_from_desc`] 的键表保证，所以不同 pass 拿到的是同一个通道句柄。
    cache: HbMap<RasterRenderPassDesc, ManagedHandle>,
    scope_reflections: RefCell<Option<Arc<crate::draw_template_cache::ReflectionScope>>>,
    constants_error: Option<rhapsody::Error>,
    resource_graph_factory: Option<Rc<dyn Fn() -> country_core::rhapsody_types::RenderGraph>>,
}

impl RasterPassHost {
    /// Prepare the graph-free declaration capability, also used by equivalence
    /// diagnostics without changing the process-wide scheduling policy.
    pub fn prepare_draw_collection(&self) -> Arc<dyn Fn() -> Box<dyn PassHost> + Send + Sync> {
        let shared = self.shared.clone();
        let assets = shared.assets.lock().collect.clone();
        let reflections = self.scope_reflections.borrow().clone();
        Arc::new(move || {
            let (constants, collect_error) = match shared.constant_pool.task() {
                Ok(task) => (Some(task), None),
                Err(error) => (None, Some(error)),
            };
            Box::new(RasterCollectHost {
                shared: shared.clone(), assets: assets.clone(), reflections: reflections.clone(),
                constants, collect_error, deferred: RefCell::new(Vec::new()),
            })
        })
    }
    pub fn with_resource_graph_factory(
        mut self,
        factory: Rc<dyn Fn() -> country_core::rhapsody_types::RenderGraph>,
    ) -> Self {
        self.resource_graph_factory = Some(factory);
        self
    }
    /// Another pass in the same scoped frame. Each RasterPass owns one host;
    /// its graph, tables and active declarer remain identical to the caller's.
    pub fn sibling(&self) -> Self {
        let mut sibling = self.shared.pass_host(
            self.graph.clone(),
            self.rg.as_ref().map(|rg| rg.declarer.clone()),
        );
        sibling.resource_graph_factory = self.resource_graph_factory.clone();
        sibling
    }
}

/// 一个 draw 在声明期解析好的东西：包 + 管线条目。
/// O `RasterPipelineOpts::default()` 0x140be6750.
pub(crate) const ORIGINAL_DEFAULT_RASTER_STATE: u32 = 0x1ffc_00d0;

struct PreparedDraw {
    packet: DrawPacket,
    entry: Arc<RasterEntry>,
    index: Option<ResourceHandle>,
    index_count: Option<u32>,
    vertices: smallvec::SmallVec<[(u32, ResourceHandle, u64); 4]>,
    external: rhapsody::resource::TempList<ExternalDrawBinding>,
    /// Graph owner and range of the FrameConstants block behind set 1/b1 (see `import_frame_constants`).
    frame_constants: Option<(ResourceHandle, rhapsody::rg::AccessRange)>,
}
struct ExternalDrawBinding {
    binding: u32,
    resource: ResourceHandle,
    access: u8,
    range: rhapsody::rg::AccessRange,
    view: Option<rhapsody::resource::TextureViewDesc>,
}


/// No graph pointer or GPU constant allocator is available during Collect.
/// The Rc-free shared queries are read-only; each task owns its logical IDs.
struct RasterCollectHost {
    shared: Arc<RasterShared>,
    assets: Arc<CollectAssets>,
    reflections: Option<Arc<crate::draw_template_cache::ReflectionScope>>,
    constants: Option<crate::draw_constant_pool::ConstantTask>,
    collect_error: Option<rhapsody::Error>,
    deferred: RefCell<Vec<country_core::render::pass_builder::draw_stream::DeferredBuffer>>,
}

use crate::draw_constant_pool::CollectedConstants;

impl PassHost for RasterCollectHost {
    fn raster_shader(
        &self,
        shader: &Handle<RasterShaderProgram>,
        variant: u8,
    ) -> Option<Arc<CoreReflection>> {
        self.reflections.as_ref().and_then(|scope| scope.get(&(shader.0.0, variant)))
            .or_else(|| self.shared.reflection(shader.0.0, variant))
    }

    fn ssbo_instance_count(&self, rg_buffer_hash: u64) -> Option<u32> {
        let assets = &self.assets;
        if let Some(leases) = &assets.ssbo_leases {
            return leases
                .get(&rg_buffer_hash)?
                .instance_count()
                .map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&rg_buffer_hash).copied()
    }

    fn ssbo_instance_count_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        let assets = &self.assets;
        if let Some(leases) = &assets.ssbo_leases {
            let lease = leases.get(&hash)?;
            assert_eq!(lease.canonical_type_name, expected_type);
            return lease.instance_count().map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&hash).copied()
    }

    fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)> {
        self.assets.placement.get(&id).cloned()
    }

    fn vk_buffer(&self, buffer: &ManagedHandle) -> Option<u64> {
        let managed = self.shared.buffer_host().get_by_id(buffer.0)?;
        Some(ash::vk::Handle::as_raw(managed.raw()))
    }

    fn vk_image_view(&self, texture: &ManagedHandle) -> Result<u64, String> {
        self.shared
            .textures
            .default_view(texture.0)
            .map_err(|error| error.to_string())
    }

    fn vk_image_view_with_desc(
        &self,
        texture: &ManagedHandle,
        view: Option<&country_core::rhapsody_types::TextureViewDesc>,
    ) -> Result<u64, String> {
        match view {
            Some(view) => self.shared.textures.explicit_view(texture.0, view),
            None => self.shared.textures.default_view(texture.0),
        }
        .map_err(|error| error.to_string())
    }

    fn texture_by_name(&self, name_hash: u64) -> (ManagedHandle, AttachmentTexture) {
        self.assets
            .textures_by_name
            .get(&name_hash)
            .cloned()
            .unwrap_or_else(|| panic!("No such texture: name hash 0x{name_hash:016x}"))
    }

    fn rg_texture(&self, preset: u8) -> AttachmentTexture {
        self.assets
            .presets
            .get(&preset)
            .copied()
            .expect("no entry found for key")
    }

    fn with_suffix_dims(&self, name_hash: u64) -> u64 {
        BindingNameHash(name_hash).with_suffix_dims().0
    }

    fn default_raster_state(&self) -> RasterState {
        // O `<RasterPipelineOpts as Default>::default` 0x140be6750 = `mov eax,0x1ffc00d0`:
        // depth test+write on with GREATER (reverse-Z), stencil test off but compare ALWAYS /
        // mask 0xff, blend off, triangle list, fill, no cull. Callers (e.g. deferred raster
        // `s & 0xbf`) start from these bits, so a zeroed state (depth LESS, stencil NEVER)
        // would reject every fragment of draws that enable the stencil test.
        RasterState(ORIGINAL_DEFAULT_RASTER_STATE)
    }
    fn prepare_collect_constants(&mut self, sizes: &[u32]) {
        if self.collect_error.is_some() { return; }
        if let Some(task) = self.constants.as_mut() {
            if let Err(error) = task.prepare_demand(sizes) {
                self.collect_error.get_or_insert(error);
            }
        }
    }

    fn allocate_dynamic_constants(&mut self, reflection: &CoreReflection) -> Option<DynamicConstantsBinding> {
        self.shared.count_management(2, 1);
        if self.collect_error.is_some() { return None; }
        let data = match self.constants.as_mut()?.take(reflection.uniform_block_size as usize) {
            Ok(data) => data,
            Err(error) => { self.collect_error = Some(error); return None; }
        };
        Some(DynamicConstantsBinding {
            data, binding_index: reflection.uniform_block_binding,
            keepalive: ManagedHandle(0), lease: None,
        })
    }
    fn finish_dynamic_constants(&mut self, constants: DynamicConstantsBinding, packet: &mut DrawPacket) {
        if let Some(binding) = packet.bindings.get_mut(&constants.binding_index) {
            if let CoreBinding::UniformBuffer { keepalive, lease, .. } = &mut binding.binding {
                if keepalive.0 == 0 && lease.is_none() {
                    let task = self.constants.as_mut().expect("admitted CPU constants task");
                    match task.own(constants.data) {
                        Ok(owner) => { *lease = Some(ResourceLease::new(Arc::new(owner))); }
                        Err((error, data)) => { self.collect_error.get_or_insert(error); task.recycle(data); }
                    }
                    return;
                }
            }
        }
        self.constants.as_mut().expect("admitted CPU constants task").recycle(constants.data);
    }
    fn discard_dynamic_constants(&mut self, constants: DynamicConstantsBinding) {
        self.constants.as_mut().expect("admitted CPU constants task").recycle(constants.data);
    }
    fn take_collect_error(&mut self) -> Option<rhapsody::Error> { self.collect_error.take() }
    fn rg_buffer_resource(&self, hash: u64) -> Option<u32> {
        let assets = &self.assets;
        let id = assets.rg_buffers.get(&hash).copied()?;
        assert!(id < country_core::render::pass_builder::draw_stream::DEFERRED_BUFFER_BASE);
        Some(id)
    }
    fn rg_buffer_resource_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        use country_core::render::pass_builder::draw_stream::{DeferredBuffer, DEFERRED_BUFFER_BASE};
        let mut deferred = self.deferred.borrow_mut();
        if let Some(buffer) = deferred.iter().find(|buffer| buffer.hash == hash) {
            assert_eq!(buffer.type_name, expected_type);
            return Some(buffer.token);
        }
        {
            let assets = &self.assets;
            let Some(leases) = &assets.ssbo_leases else {
                let id = assets.rg_buffers.get(&hash).copied()?;
                assert!(id < DEFERRED_BUFFER_BASE);
                return Some(id);
            };
            assert_eq!(leases.get(&hash)?.canonical_type_name, expected_type);
        }
        let token = DEFERRED_BUFFER_BASE.checked_add(u32::try_from(deferred.len()).unwrap()).unwrap();
        deferred.push(DeferredBuffer { token, hash, type_name: expected_type });
        Some(token)
    }
    fn take_deferred_buffers(&mut self) -> Vec<country_core::render::pass_builder::draw_stream::DeferredBuffer> {
        std::mem::take(self.deferred.get_mut())
    }
    fn create_render_pass(&mut self, _: &RasterRenderPassDesc) -> ManagedHandle {
        panic!("Collect cannot create a render pass")
    }
    fn render_pass_cache(&mut self) -> &mut HbMap<RasterRenderPassDesc, ManagedHandle> {
        panic!("Collect cannot access the render-pass cache")
    }
    fn submit_raster_pass(&mut self, _: RasterPassSubmission) {
        panic!("Collect cannot submit a render pass")
    }
}

impl PassHost for RasterPassHost {
    fn collect_constant_plan_enabled(&self) -> bool {
        self.shared.constant_pool.batch_enabled()
    }

    fn reject_failed_declaration(&mut self) -> bool {
        let Some(error) = self.constants_error.take() else { return false; };
        self.graph.borrow_mut().reject(error);
        true
    }

    fn begin_collect_scope(&self) {
        #[cfg(feature = "cpu-probe")]
        if !self.shared.management_mode(0) {
            *self.scope_reflections.borrow_mut() = None;
            return;
        }
        *self.scope_reflections.borrow_mut() = crate::draw_template_cache::ReflectionScope::try_new(
            self.shared.device.allocator_stats()).ok().map(Arc::new);
    }

    fn checkout_draw_scratch(&self) -> rhapsody::Result<country_core::render::pass_builder::draw_commit_scratch::DrawCommitScratchLease> {
        self.shared.commit_scratch.checkout()
    }

    fn record_collect_error(&mut self, error: rhapsody::Error) {
        self.constants_error.get_or_insert(error);
    }

    fn collect_factory(&self) -> Arc<dyn Fn() -> Box<dyn PassHost> + Send + Sync> {
        self.prepare_draw_collection()
    }

    fn collect_chunk_draws(&self) -> usize {
        self.shared.collect_chunk_draws
    }

    fn commit_collected_draws(&mut self, packets: &mut [DrawPacket]) {
        #[cfg(feature = "cpu-probe")]
        if !self.shared.constant_pool.batch_enabled() {
            for packet in packets { self.commit_collected_draw(packet); }
            return;
        }
        const LIMIT: usize = 64;
        // All temporary indices/chunks/return owners stay inline and bounded. The
        // page's host budget is not silently charged by an unbounded scratch spill.
        for page in packets.chunks_mut(LIMIT) {
            let mut keys = smallvec::SmallVec::<[(usize, u32); LIMIT]>::new();
            let mut overflow = false;
            'scan: for (draw_index, packet) in page.iter().enumerate() {
                for (&binding_index, binding) in &packet.bindings {
                    let CoreBinding::UniformBuffer { lease, .. } = &binding.binding else { continue };
                    if lease.as_ref().and_then(ResourceLease::downcast_ref::<CollectedConstants>).is_none() { continue; }
                    if keys.len() == LIMIT { overflow = true; break 'scan; }
                    keys.push((draw_index, binding_index));
                }
            }
            if overflow {
                for packet in page { self.commit_collected_draw(packet); }
                continue;
            }
            if keys.is_empty() { continue; }
            let mut chunks = smallvec::SmallVec::<[(usize, u32, rhapsody::Result<DynamicConstantsChunk>); LIMIT]>::new();
            {
                let mut allocator = self.shared.dynamic.lock();
                for (draw_index, binding_index) in keys {
                    let CoreBinding::UniformBuffer { lease, .. } = &page[draw_index].bindings[&binding_index].binding
                        else { unreachable!("collected constants binding changed without a host call") };
                    let bytes = lease.as_ref().and_then(ResourceLease::downcast_ref::<CollectedConstants>)
                        .expect("collected CPU constant owner changed");
                    chunks.push((draw_index, binding_index, allocator.allocate(bytes.bytes.len() as u64)));
                }
            }
            let mut returned = smallvec::SmallVec::<[rhapsody::resource::CpuBytes; LIMIT]>::new();
            for (draw_index, binding_index, allocation) in chunks {
                let chunk = match allocation {
                    Ok(chunk) => chunk,
                    Err(error) => { self.constants_error.get_or_insert(error); continue; }
                };
                let CoreBinding::UniformBuffer { keepalive, lease, .. } = &mut page[draw_index]
                    .bindings.get_mut(&binding_index).expect("collected binding disappeared").binding
                    else { unreachable!("collected uniform binding changed") };
                let bytes = lease.as_ref().and_then(ResourceLease::downcast_ref::<CollectedConstants>)
                    .expect("collected CPU constant owner disappeared");
                if let Err(error) = self.shared.write_collected_constants(&chunk, &bytes.bytes) {
                    self.constants_error.get_or_insert(error);
                    continue; // Keep the original CPU lease; retry/error semantics unchanged.
                }
                self.shared.count_management(3, bytes.bytes.len() as u64);
                *keepalive = ManagedHandle(ash::vk::Handle::as_raw(chunk.buffer.raw_handle()));
                let old = lease.replace(ResourceLease::new(Arc::new(chunk)))
                    .expect("CPU constant lease exists until its successful replacement");
                // Shared CPU owners can escape Commit. They keep their exact old
                // lifetime; only an exclusive owner returns storage immediately.
                if let Ok(mut constants) = old.try_unwrap::<CollectedConstants>() {
                    returned.push(std::mem::take(&mut constants.bytes));
                }
            }
            self.shared.constant_pool.recycle_batch(returned);
        }
    }

    fn commit_collected_draw(&mut self, packet: &mut DrawPacket) {
        for binding in packet.bindings.values_mut() {
            let CoreBinding::UniformBuffer { keepalive, lease, .. } = &mut binding.binding else { continue };
            let Some(bytes) = lease.as_ref().and_then(ResourceLease::downcast_ref::<CollectedConstants>) else { continue };
            let allocation = { self.shared.dynamic.lock().allocate(bytes.bytes.len() as u64) };
            let result = allocation.and_then(|chunk| {
                    self.shared.write_collected_constants(&chunk, &bytes.bytes)?;
                    self.shared.count_management(3, bytes.bytes.len() as u64);
                    Ok(Arc::new(chunk))
                });
            match result {
                Ok(chunk) => {
                    *keepalive = ManagedHandle(ash::vk::Handle::as_raw(chunk.buffer.raw_handle()));
                    *lease = Some(ResourceLease::new(chunk));
                }
                Err(error) => { self.constants_error.get_or_insert(error); }
            }
        }
    }

    fn raster_shader(
        &self,
        shader: &Handle<RasterShaderProgram>,
        variant: u8,
    ) -> Option<Arc<CoreReflection>> {
        let key = (shader.0.0, variant);
        {
            let scopes = self.scope_reflections.borrow();
            let Some(scope) = scopes.as_ref() else { return self.shared.reflection(key.0, key.1); };
            if let Some(hit) = scope.get(&key) { return Some(hit); }
        }
        let reflection = self.shared.reflection(key.0, key.1)?;
        if let Some(scope) = self.scope_reflections.borrow_mut().as_mut() {
            if let Some(scope) = Arc::get_mut(scope) { scope.insert(key, reflection.clone()); }
        }
        Some(reflection)
    }

    /// 在常量页里为该管线的 uniform 块分配一块；字节在 [`PassHost::flush_dynamic_constants`] 写入。
    fn allocate_dynamic_constants(
        &mut self,
        reflection: &CoreReflection,
    ) -> Option<DynamicConstantsBinding> {
        let size = u64::from(reflection.uniform_block_size);
        let allocation = { self.shared.dynamic.lock().allocate(size) };
        let chunk = match allocation {
            Ok(chunk) => Arc::new(chunk),
            Err(error) => {
                self.constants_error.get_or_insert(error);
                return None;
            }
        };
        let data = match rhapsody::resource::CpuBytes::new(self.shared.device.allocator_stats(), size as usize) {
            Ok(data) => data,
            Err(error) => { self.constants_error.get_or_insert(error); return None; }
        };
        Some(DynamicConstantsBinding {
            data,
            binding_index: reflection.uniform_block_binding,
            keepalive: ManagedHandle(ash::vk::Handle::as_raw(chunk.buffer.raw_handle())),
            lease: Some(ResourceLease::new(chunk)),
        })
    }

    fn flush_dynamic_constants(&mut self, constants: &DynamicConstantsBinding) {
        let result = constants
            .lease
            .as_ref()
            .and_then(ResourceLease::downcast_ref::<DynamicConstantsChunk>)
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("flush_dynamic_constants: missing resource lease")
            })
            .and_then(|chunk| chunk.write_bytes(&constants.data));
        if let Err(error) = result {
            self.constants_error.get_or_insert(error);
        }
    }

    fn rg_buffer_resource(&self, rg_buffer_hash: u64) -> Option<u32> {
        self.shared
            .assets
            .lock()
            .collect
            .rg_buffers
            .get(&rg_buffer_hash)
            .copied()
    }

    fn rg_buffer_resource_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        let lease = {
            let tables = self.shared.assets.lock();
            let assets = &tables.collect;
            let Some(leases) = &assets.ssbo_leases else {
                // Standalone diagnostics use explicit graph resources. Game
                // FrameSetup always publishes a real canonical metadata map.
                return assets.rg_buffers.get(&hash).copied();
            };
            leases.get(&hash)?.clone()
        };
        // A lease already imported into this frame's graph needs no graph at
        // all; building a RenderGraph (backend box, Rc clones, two locks) per
        // lookup was pure overhead on the cache-hit path.
        let imported = match lease.rg_handle() {
            Some(imported) => {
                assert_eq!(lease.canonical_type_name, expected_type);
                imported
            }
            None => {
                let mut graph = self
                    .resource_graph_factory
                    .as_ref()
                    .expect("real raster SSBO lookup requires the enclosing frame resource graph")(
                );
                lease.get_rg_buffer(&mut graph, expected_type)
            }
        };
        assert_eq!(
            imported.generation,
            self.graph.borrow().graph_id().raw(),
            "raster SSBO cache belongs to another frame"
        );
        Some(u32::try_from(imported.resource).expect("graph resource exceeds u32"))
    }

    fn ssbo_instance_count(&self, rg_buffer_hash: u64) -> Option<u32> {
        let tables = self.shared.assets.lock();
        let assets = &tables.collect;
        if let Some(leases) = &assets.ssbo_leases {
            return leases
                .get(&rg_buffer_hash)?
                .instance_count()
                .map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&rg_buffer_hash).copied()
    }
    fn ssbo_instance_count_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        let tables = self.shared.assets.lock();
        let assets = &tables.collect;
        if let Some(leases) = &assets.ssbo_leases {
            let lease = leases.get(&hash)?;
            assert_eq!(lease.canonical_type_name, expected_type);
            return lease.instance_count().map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&hash).copied()
    }

    fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)> {
        self.shared.assets.lock().collect.placement.get(&id).cloned()
    }

    fn vk_buffer(&self, buffer: &ManagedHandle) -> Option<u64> {
        let managed = self.shared.buffer_host().get_by_id(buffer.0)?;
        Some(ash::vk::Handle::as_raw(managed.raw()))
    }

    fn vk_image_view(&self, texture: &ManagedHandle) -> Result<u64, String> {
        self.shared
            .textures
            .default_view(texture.0)
            .map_err(|error| error.to_string())
    }

    fn vk_image_view_with_desc(
        &self,
        texture: &ManagedHandle,
        view: Option<&country_core::rhapsody_types::TextureViewDesc>,
    ) -> Result<u64, String> {
        match view {
            Some(view) => self.shared.textures.explicit_view(texture.0, view),
            None => self.shared.textures.default_view(texture.0),
        }
        .map_err(|error| error.to_string())
    }

    fn texture_by_name(&self, name_hash: u64) -> (ManagedHandle, AttachmentTexture) {
        self.shared
            .assets
            .lock()
            .collect
            .textures_by_name
            .get(&name_hash)
            .cloned()
            .unwrap_or_else(|| panic!("No such texture: name hash 0x{name_hash:016x}"))
    }

    fn rg_texture(&self, preset: u8) -> AttachmentTexture {
        self.shared
            .assets
            .lock()
            .collect
            .presets
            .get(&preset)
            .copied()
            .expect("no entry found for key")
    }

    /// `BindingNameHash::with_suffix_dims`（O：`wrapping_add(1)`，RHAPSODY-DEEP-SUMMARY §3 #9）。
    fn with_suffix_dims(&self, name_hash: u64) -> u64 {
        BindingNameHash(name_hash).with_suffix_dims().0
    }

    /// `rhapsody::pipeline::impl$7::default()`：`RasterPipelineOpts` 的 4 字节位域缺省值。
    fn default_raster_state(&self) -> RasterState {
        // O `<RasterPipelineOpts as Default>::default` 0x140be6750 = `mov eax,0x1ffc00d0`:
        // depth test+write on with GREATER (reverse-Z), stencil test off but compare ALWAYS /
        // mask 0xff, blend off, triangle list, fill, no cull. Callers (e.g. deferred raster
        // `s & 0xbf`) start from these bits, so a zeroed state (depth LESS, stencil NEVER)
        // would reject every fragment of draws that enable the stencil test.
        RasterState(ORIGINAL_DEFAULT_RASTER_STATE)
    }

    fn create_render_pass(&mut self, desc: &RasterRenderPassDesc) -> ManagedHandle {
        let id = rh_render_pass_desc(desc)
            .and_then(|desc| {
                self.shared
                    .shaders
                    .passes()
                    .create_from_desc(&self.shared.device, &desc)
            })
            .unwrap_or_else(|e| panic!("GraphicsDevice::create_render_pass: {e:?}"));
        ManagedHandle(id)
    }

    fn render_pass_cache(&mut self) -> &mut HbMap<RasterRenderPassDesc, ManagedHandle> {
        &mut self.cache
    }

    fn submit_raster_pass(&mut self, submission: RasterPassSubmission) {
        let constants_error = self.constants_error.take();
        let shared = self.shared.clone();
        let RasterPassSubmission {
            name,
            draws,
            bindings,
            binding_accesses,
            render_pass,
            accesses,
            colors,
            depth,
            ..
        } = submission;

        let stage_started = country_core::render::declare_probe::start();
        // 声明期解析每个 draw 的管线（失败 = 原版 `Result::unwrap` panic，见 `get_or_generate`）。
        let prepared: rhapsody::Result<Vec<PreparedDraw>> = draws
            .into_iter()
            .map(|packet| {
                let entry = shared
                    .programs
                    .get_raster_by_id(packet.shader.0.0).map(|program| {
                        let variant = program
                            .variants
                            .get(usize::from(packet.variant))
                            .ok_or_else(|| {
                                rhapsody::Error::invalid_usage(
                                    "draw packet variant is out of range",
                                )
                            })?;
                        let key = RasterPipelineKey {
                            handle: render_pass.0,
                            selector: packet.state.0,
                        };
                        let pipeline = variant
                            .get_or_generate(&*shared.shaders as &dyn ShaderBackend, key)
                            .map_err(|error| match error.downcast::<rhapsody::Error>() {
                                Ok(error) => error,
                                Err(error) => rhapsody::Error::InvalidUsage {
                                    what: "raster pipeline preparation".into(),
                                    source: Some(error.into()),
                                },
                            })?;
                        shared.shaders.raster_entry(pipeline).ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "raster pipeline was not created through this host",
                            )
                        })
                    })
                    .ok_or_else(|| {
                        rhapsody::Error::invalid_usage(
                            "draw packet shader program was not registered",
                        )
                    })??;
                let external_limit = packet.bindings.len().max(1);
                Ok(PreparedDraw {
                    packet,
                    entry,
                    index: None,
                    index_count: None,
                    vertices: smallvec::SmallVec::new(),
                    frame_constants: None,
                    external: rhapsody::resource::TempList::new(
                        external_limit,
                        shared.device.allocator_stats().clone(),
                    )?,
                })
            })
            .collect();

        country_core::render::declare_probe::stage(0, stage_started);
        let mut graph = self.graph.borrow_mut();
        let mut prepared = match (constants_error, prepared) {
            (None, Ok(prepared)) => prepared,
            (Some(error), _) | (_, Err(error)) => {
                log::error!("raster declaration failed: pass={name}, error={error}");
                graph.reject(error);
                return;
            }
        };
        let stage_started = country_core::render::declare_probe::start();
        let index_imports = (|| -> rhapsody::Result<()> {
            for draw in &mut prepared {
                if draw.packet.bindings.values().any(|owned| {
                    owned.state
                        == country_core::render::pass_builder::draw_stream::GpuOwned::STATE_OWNED
                        && !matches!(owned.binding, CoreBinding::UniformBuffer { .. })
                }) {
                    let declarer = &self
                        .rg
                        .as_ref()
                        .ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "external draw resources require the frame resource declarer",
                            )
                        })?
                        .declarer;
                    import_draw_bindings(&shared, draw, declarer.as_ref(), &mut graph)?;
                }
                {
                    let declarer = &self
                        .rg
                        .as_ref()
                        .ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "frame constants import requires the frame resource declarer",
                            )
                        })?
                        .declarer;
                    import_frame_constants(&shared, draw, declarer.as_ref(), &mut graph)?;
                }
                if matches!(
                    draw.packet.kind,
                    DrawKind::IndexedIndirect { .. } | DrawKind::IndexedIndirectCount { .. }
                ) && draw.packet.head[0] != 1
                {
                    return Err(rhapsody::Error::unsupported(
                        "indexed indirect draw has no recovered actual index-buffer owner",
                    ));
                }
                validate_indirect_ranges(&graph, &draw.packet.kind)?;
                // Native DrawIndirectCount stays native on enabled devices.
                // A physically supported but unenabled feature is a configuration
                // error; only unsupported devices use the GPU platform lowering.
                if let DrawKind::IndexedIndirectCount {
                    buffer,
                    count_buffer,
                    args,
                } = draw.packet.kind.clone()
                {
                    use rhapsody::device::Feature;
                    if !shared.device.caps().is_enabled(Feature::DrawIndirectCount) {
                        if shared
                            .device
                            .caps()
                            .is_supported(Feature::DrawIndirectCount)
                        {
                            shared
                                .device
                                .caps()
                                .require_enabled(Feature::DrawIndirectCount)?;
                        }
                        // Moving a counter snapshot before this render pass
                        // is equivalent only while its commands/counter remain
                        // read-only throughout the pass (true for original Nani).
                        if accesses.iter().any(|a| {
                            (a.resource == buffer || a.resource == count_buffer)
                                && country_core::render::pass_builder::draw_stream::access_is_write(
                                    a.access,
                                )
                        }) {
                            return Err(rhapsody::Error::unsupported(
                                "indirect count source/counter is modified inside the raster pass",
                            ));
                        }
                        let maximum = args[3] as u32;
                        let (target, offset, stride) = if maximum == 0 {
                            (buffer, args[0], args[1])
                        } else {
                            let request = rhapsody::rg::IndexedCountRequest {
                                command_offset: args[0],
                                command_stride: args[1] as u32,
                                count_offset: args[2],
                                maximum_draws: maximum,
                            };
                            // Validate native command limits before allocating.
                            if maximum > shared.device.max_draw_indirect_count() {
                                return Err(rhapsody::Error::invalid_usage(
                                    "indexed count maximum exceeds device draw limit",
                                ));
                            }
                            if maximum > 1 {
                                shared
                                    .device
                                    .caps()
                                    .require_enabled(Feature::MultiDrawIndirect)?;
                            }
                            let declarer = &self
                                .rg
                                .as_ref()
                                .ok_or_else(|| {
                                    rhapsody::Error::invalid_usage(
                                        "indirect count lowering requires the frame declarer",
                                    )
                                })?
                                .declarer;
                            let commands = graph.resource_handle(buffer)?;
                            let counter = graph.resource_handle(count_buffer)?;
                            // Each packet owns independent scratch in this same graph.
                            // The resource-index scope advances with every declaration;
                            // existing names are checked before allocation as well.
                            let mut scope = graph.declaration().resources.len();
                            let scratch_name = loop {
                                let candidate = format!(
                                    "platform indexed indirect count commands [{name}:{scope}]"
                                );
                                if !graph
                                    .declaration()
                                    .resources
                                    .iter()
                                    .any(|r| r.name == candidate)
                                {
                                    break candidate;
                                }
                                scope = scope.checked_add(1).ok_or_else(|| {
                                    rhapsody::Error::invalid_usage(
                                        "indirect scratch scope overflow",
                                    )
                                })?;
                            };
                            let scratch = declarer.declare(
                                &mut graph,
                                &scratch_name,
                                ResourceSpec::Buffer(request.scratch_desc()?),
                            )?;
                            shared.shaders.indexed_count_adapter()?.add_pass(
                                &mut graph,
                                commands,
                                counter,
                                scratch,
                                request,
                                shared.dynamic.clone(),
                            )?;
                            (scratch.raw(), 0, 20)
                        };
                        draw.packet.kind = DrawKind::IndexedIndirect {
                            buffer: target,
                            arg_48: offset,
                            arg_50: u64::from(maximum),
                            arg_58: stride,
                            first_index_count: None,
                        };
                    }
                }
                admit_draw_kind(&draw.packet.kind, &shared.device)?;
                if matches!(
                    draw.packet.kind,
                    DrawKind::InstancedGlDrawFlag { .. }
                        | DrawKind::MultiInstanced { .. }
                        | DrawKind::IndexedIndirect { .. }
                        | DrawKind::IndexedIndirectCount { .. }
                ) || (matches!(draw.packet.kind, DrawKind::MultiDrawIndirect { .. })
                    && draw.packet.head[0] == 1)
                {
                    // O B 0x141697ff9: head tag1 is a live Handle<Vao>,
                    // not a VkBuffer/u64 address. The native library lookup is
                    // keyed by that same typed asset identity.
                    if draw.packet.head[0] != 1 {
                        return Err(rhapsody::Error::invalid_usage(
                            "indexed mesh draw requires an original VAO head",
                        ));
                    }
                    let vao = shared
                        .assets
                        .lock()
                        .vaos
                        .as_ref()
                        .and_then(|table| table.entries.get(&draw.packet.head[1]))
                        .cloned()
                        .ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "mesh VAO was not resolved from this App's actual AssetVAOLibrary",
                            )
                        })?;
                    let declarer = &self
                        .rg
                        .as_ref()
                        .ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "mesh draw requires the frame resource declarer",
                            )
                        })?
                        .declarer;
                    draw.index = Some(declarer.import_buffer(&mut graph, vao.index.clone(), 0)?);
                    draw.index_count = Some(vao.index_count);
                    for (name, location, stride) in &draw.entry.vertex_inputs {
                        let (buffer, size) = match vao
                            .bindings
                            .iter()
                            .find(|(attribute, _)| attribute.as_ref() == name)
                        {
                            Some((_, buffer)) => (buffer.clone(), buffer.desc.size),
                            None => {
                                // O 0x141697ab0 `RasterPass::render` closure iterates the VAO's
                                // bindings and looks each name up in the program; a reflected
                                // input that the VAO lacks is never bound (no error, no
                                // placeholder), and 0x1416a3720 still creates the pipeline
                                // attribute. The only such case is nani_cloth `Vertex_Color`
                                // (vs_main copies it to a varying `C2` the PS never reads;
                                // the cloth meshes carry no color attribute). Vulkan forbids
                                // drawing with an unbound vertex binding, so this host binds
                                // a zero-filled buffer of the right extent instead (DEVIATION,
                                // value unobservable). Any other missing input stays an error.
                                if name != "Vertex_Color" {
                                    return Err(rhapsody::Error::invalid_usage(format!(
                                        "VAO lacks the shader's reflected vertex attribute {name}"
                                    )));
                                }
                                let vertices = draw
                                    .entry
                                    .vertex_inputs
                                    .iter()
                                    .filter_map(|(other, _, other_stride)| {
                                        vao.bindings
                                            .iter()
                                            .find(|(attribute, _)| attribute.as_ref() == other)
                                            .map(|(_, other_buffer)| {
                                                other_buffer.desc.size / u64::from(*other_stride)
                                            })
                                    })
                                    .min()
                                    .ok_or_else(|| {
                                        rhapsody::Error::invalid_usage(
                                            "VAO has no bound attribute to size the placeholder",
                                        )
                                    })?;
                                let bytes = vertices * u64::from(*stride);
                                (shared.zero_vertex.get(&shared.device, bytes)?, bytes)
                            }
                        };
                        let resource = declarer.import_buffer(&mut graph, buffer, 0)?;
                        draw.vertices.push((*location, resource, size));
                    }
                    continue;
                }
                let DrawKind::NoVaoIndexed {
                    index_buffer,
                    index_count,
                } = &draw.packet.kind
                else {
                    continue;
                };
                if *index_count == 0 {
                    continue;
                }
                let buffer = shared.buffer_host()
                    .get_by_id(index_buffer.0)
                    .ok_or_else(|| rhapsody::Error::invalid_usage("index buffer is not live"))?;
                if u64::from(*index_count) * 4 > buffer.resource().size {
                    return Err(rhapsody::Error::invalid_usage(
                        "index count exceeds buffer extent",
                    ));
                }
                let declarer = &self
                    .rg
                    .as_ref()
                    .ok_or_else(|| {
                        rhapsody::Error::invalid_usage(
                            "indexed draw requires the frame resource declarer",
                        )
                    })?
                    .declarer;
                draw.index = Some(declarer.import_buffer(&mut graph, buffer.shared(), 0x26)?);
                draw.index_count = Some(*index_count);
            }
            Ok(())
        })();
        country_core::render::declare_probe::stage(1, stage_started);
        let stage_started = country_core::render::declare_probe::start();
        if let Err(error) = index_imports {
            log::error!("raster declaration failed: pass={name}, error={error}");
            graph.reject(error);
            return;
        }
        let (preserving_buffers, read_contents) = match raster_content_intents(
            prepared.iter().map(|draw| &draw.packet),
            &bindings,
            |resource| {
                graph
                    .declaration()
                    .resources
                    .get(resource as usize)
                    .map(|resource| {
                        matches!(resource.descriptor, rhapsody::rg::Descriptor::Image(_))
                    })
                    .ok_or_else(|| {
                        rhapsody::Error::invalid_usage(
                            "raster metadata resource is outside the graph",
                        )
                    })
            },
        ) {
            Ok(intents) => intents,
            Err(error) => {
                log::error!("raster declaration failed: pass={name}, error={error}");
                graph.reject(error);
                return;
            }
        };
        // A native shader binding carries its view independently from the
        // owner's graph identity. Project the very same view before graph build;
        // a descriptor selecting one layer must not read every layer/mip.
        let attachment_accesses = match raster_attachment_accesses(&colors, depth.as_ref()) {
            Ok(attachments) => attachments,
            Err(error) => {
                log::error!("raster declaration failed: pass={name}, error={error}");
                graph.reject(error);
                return;
            }
        };
        let mut access_ranges = match raster_binding_ranges(
            &accesses,
            &bindings,
            &binding_accesses,
            colors.len() + usize::from(depth.is_some()),
            |resource| {
                let handle = graph.resource_handle(resource)?;
                Ok(graph.declaration().resources[handle.raw() as usize].descriptor)
            },
        ) {
            Ok(ranges) => ranges,
            Err(error) => {
                log::error!("raster declaration failed: pass={name}, error={error}");
                graph.reject(error);
                return;
            }
        };
        for (index, attachment) in attachment_accesses.iter().enumerate() {
            let actual = &accesses[index];
            let projection = (|| -> rhapsody::Result<_> {
                if actual.resource != attachment.resource
                    || actual.access != attachment.access
                    || actual.group != 1
                {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster attachment differs from its native resource access",
                    ));
                }
                let handle = graph.resource_handle(attachment.resource)?;
                let descriptor = graph.declaration().resources[handle.raw() as usize].descriptor;
                compute_view_range(descriptor, &attachment.view)
            })();
            match projection {
                Ok(range) => access_ranges[index] = (range, None),
                Err(error) => {
                    log::error!("raster declaration failed: pass={name}, error={error}");
                    graph.reject(error);
                    return;
                }
            }
        }
        // Native LOAD keeps the producer of this exact view. An independent
        // layer or mip never turns a first write into a preserving operation.
        // Undefined/partially initialized inputs are not invented here: an
        // overlapping producer still requires the complete LOAD range defined.
        let preserving_attachments = attachment_accesses
            .iter()
            .enumerate()
            .map(|(index, attachment)| {
                attachment.load
                    && rhapsody::command::is_write(attachment.access)
                    && raster_has_previous_contents(
                        &graph,
                        attachment.resource,
                        access_ranges[index].0,
                    )
            })
            .collect::<Vec<_>>();
        country_core::render::declare_probe::stage(2, stage_started);
        // The global texture array (set 2/b0) is indexed dynamically by the shader, so every
        // table texture is a potential read of every pass that binds it; the original keeps them
        // all in SHADER_READ_ONLY. Declare each exact view range once per pass.
        let stage_started = country_core::render::declare_probe::start();
        let table_reads = (|| -> rhapsody::Result<Arc<[(ResourceHandle, rhapsody::rg::AccessRange)]>> {
            let mut reads = Vec::new();
            // Only a pass that binds the table reads it.
            let Some(first) = prepared.iter().find(|draw| {
                draw.entry
                    .bindings
                    .abi
                    .slot(BindingLocation { set: 2, binding: 0 })
                    .is_some()
            }) else {
                return Ok(Arc::from(reads));
            };
            let graph_id = graph.graph_id();
            if let Some((cached, reads)) = shared.table_reads.lock().as_ref()
                && *cached == graph_id
            {
                // Frame globals are set once per frame, before the graph is
                // built, so every pass of a graph sees the same table.
                debug_assert_eq!(
                    reads.len(),
                    first.entry.bindings.frame_sampled_table().len(),
                    "texture table changed within one graph"
                );
                return Ok(reads.clone());
            }
            let declarer = &self
                .rg
                .as_ref()
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "texture table declaration requires the frame resource declarer",
                    )
                })?
                .declarer;
            for view in first.entry.bindings.frame_sampled_table() {
                let resource = declarer.import_texture(&mut graph, view.texture.clone(), 0x22)?;
                reads.push((
                    resource,
                    rhapsody::rg::AccessRange::Texture {
                        aspect_mask: view.desc.aspect.as_raw(),
                        base_mip_level: view.desc.base_mip_level,
                        level_count: view.desc.level_count,
                        base_array_layer: view.desc.base_layer,
                        layer_count: view.desc.layer_count,
                    },
                ));
            }
            let reads: Arc<[_]> = Arc::from(reads);
            *shared.table_reads.lock() = Some((graph_id, reads.clone()));
            Ok(reads)
        })();
        let table_reads = match table_reads {
            Ok(reads) => reads,
            Err(error) => {
                log::error!("raster declaration failed: pass={name}, error={error}");
                graph.reject(error);
                return;
            }
        };
        country_core::render::declare_probe::stage(3, stage_started);
        let mut declared = graph.add_pass(name);
        let table_started = country_core::render::declare_probe::start();
        for (resource, range) in table_reads.iter() {
            if let Err(error) = declared.add_resource_access_range(*resource, 0x1d, 0, *range) {
                declared.rg_mut().reject(error.into());
                return;
            }
        }
        country_core::render::declare_probe::table(table_started, table_reads.len());
        let draw_access_started = country_core::render::declare_probe::start();
        let mut frame_constants_declared = false;
        for draw in &prepared {
            // Native DrawBuilder also records these as access 2/group 1.
            // Register them here as a final owner/access invariant, including
            // packets constructed through the public raw draw() boundary.
            let indirect = match &draw.packet.kind {
                DrawKind::MultiDrawIndirect { buffer, .. }
                | DrawKind::IndexedIndirect { buffer, .. } => [Some(*buffer), None],
                DrawKind::IndexedIndirectCount {
                    buffer,
                    count_buffer,
                    ..
                } => [Some(*buffer), Some(*count_buffer)],
                _ => [None, None],
            };
            for id in indirect.into_iter().flatten() {
                let resource = declared
                    .resource_handle(id)
                    .expect("validated indirect graph resource");
                declared
                    .add_resource_access(resource, 2, 1)
                    .expect("validated indirect access");
            }
            // import_frame_constants resolves one immutable owner/range per
            // GraphId. Keep its first declaration position and strict read;
            // later identical reads cannot strengthen its content policy.
            if !frame_constants_declared && let Some((resource, range)) = draw.frame_constants {
                frame_constants_declared = true;
                if let Err(error) = declared.add_resource_access_range(resource, 0x1f, 0, range) {
                    declared.rg_mut().reject(error.into());
                    return;
                }
            }
            for binding in draw.external.as_slice() {
                if let Err(error) = declared.add_resource_access_range(
                    binding.resource,
                    binding.access,
                    0,
                    binding.range,
                ) {
                    declared.rg_mut().reject(error.into());
                    return;
                }
            }
            if let Some(index) = draw.index {
                let index_count = draw.index_count.expect("prepared index count");
                declared
                    .add_resource_access_range(
                        index,
                        0x03,
                        0,
                        rhapsody::rg::AccessRange::Buffer {
                            offset: 0,
                            size: u64::from(index_count) * 4,
                        },
                    )
                    .expect("validated index buffer range");
            }
            for (_, resource, size) in &draw.vertices {
                declared
                    .add_resource_access_range(
                        *resource,
                        0x04,
                        0,
                        rhapsody::rg::AccessRange::Buffer {
                            offset: 0,
                            size: *size,
                        },
                    )
                    .expect("validated vertex buffer range");
            }
        }
        for (index, (a, (range, binding_slot))) in accesses.iter().zip(&access_ranges).enumerate() {
            let resource = declared
                .resource_handle(a.resource)
                .expect("add_resource_access: resource does not belong to this graph");
            let writes = country_core::render::pass_builder::draw_stream::access_is_write(a.access);
            let result = if (preserving_attachments.get(index) == Some(&true)
                || preserving_buffers.contains(&a.resource))
                && writes
            {
                declared.add_preserving_resource_access_range(resource, a.access, a.group, *range)
            } else if !writes
                && (*binding_slot).and_then(|slot| read_contents.get(&slot))
                    == Some(&RasterReadContents::Inactive)
                && index >= attachment_accesses.len()
            {
                declared.add_inactive_resource_read_range(resource, a.access, a.group, *range)
            } else if !writes
                && (*binding_slot).and_then(|slot| read_contents.get(&slot))
                    == Some(&RasterReadContents::UndefinedAllowed)
                && index >= attachment_accesses.len()
            {
                declared.add_undefined_content_read_range(resource, a.access, a.group, *range)
            } else {
                declared.add_resource_access_range(resource, a.access, a.group, *range)
            };
            result.expect("add_resource_access: resource does not belong to this graph");
        }
        country_core::render::declare_probe::draw_access(draw_access_started);
        declared.render_with_context(move |ctx| {
            let managed = ctx
                .managed
                .expect("pass callback without managed resources");
            let device = shared.device.clone();
            let command = <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());
            // Explicit frame backpressure while a variant compiles: the normal
            // transaction cancels unsubmitted commands and preserves the last image.
            let mut pipelines = Vec::with_capacity(prepared.len());
            for draw in &prepared {
                match draw.entry.pipeline.try_get() {
                    Ok(pipeline) => pipelines.push(pipeline),
                    Err(error) => {
                        ctx.report_error(error);
                        return false;
                    }
                }
            }

            // 附件：tag 决定 loadOp（0 LOAD / 1 CLEAR / 2 DONT_CARE，CLEAR 值来自原四字 payload）。
            let mut attachments = Vec::new();
            let mut extent = None;
            for (index, c) in colors.iter().enumerate() {
                let texture = managed
                    .texture_for(c.id)
                    .expect("color attachment: unresolved image");
                extent.get_or_insert(vk::Extent2D {
                    width: texture.desc.width,
                    height: texture.desc.height,
                });
                // B color tag1's +4..+0x14 words become VkClearValue.
                // The legacy core field is named `view`; preserve its bits.
                let clear = (c.tag == 1).then(|| c.clear_color_bits().map(f32::from_bits));
                let mut a = match Attachment::color(texture.shared(), 0, clear) {
                    Ok(attachment) => attachment,
                    Err(error) => {
                        ctx.report_error(error);
                        return false;
                    }
                };
                a.view = match rh_view_desc(&texture, &attachment_accesses[index].view) {
                    Ok(view) => view,
                    Err(error) => {
                        ctx.report_error(error);
                        return false;
                    }
                };
                a.desc.load = vk::AttachmentLoadOp::from_raw(c.tag as i32);
                attachments.push(a);
            }
            if let Some(d) = &depth {
                let texture = managed
                    .texture_for(d.texture_id)
                    .expect("depth attachment: unresolved image");
                extent.get_or_insert(vk::Extent2D {
                    width: texture.desc.width,
                    height: texture.desc.height,
                });
                let clear = match d.clear {
                    DepthClear::Clear(v) => Some(v),
                    DepthClear::Load => None,
                };
                let a = match Attachment::depth(texture.shared(), 0, clear) {
                    Ok(attachment) => attachment,
                    Err(error) => {
                        ctx.report_error(error);
                        return false;
                    }
                };
                let mut a = a.depth_access(if d.writable {
                    rhapsody::rg::DepthAccess::ReadWrite
                } else {
                    rhapsody::rg::DepthAccess::ReadOnly
                });
                a.view = match rh_view_desc(&texture, &attachment_accesses[colors.len()].view)
                    .and_then(|mut view| {
                        // A depth/stencil attachment keeps every aspect of its format: the
                        // DEPTH-only default of `rh_view_desc` is the sampled-view default
                        // (original 0x140bdcba0) and would drop the stencil writes that the
                        // deferred material pass tests.
                        if attachment_accesses[colors.len()].view.aspect_mask.is_none() {
                            view.aspect =
                                rhapsody::resource::TextureViewDesc::new(&texture.desc)?.aspect;
                        }
                        Ok(view)
                    }) {
                    Ok(view) => view,
                    Err(error) => {
                        ctx.report_error(error);
                        return false;
                    }
                };
                attachments.push(a);
            }
            let extent = extent.expect("raster pass without attachments");
            let prepared_pass = match shared.attachments.lock().prepare_for_rendering(
                &attachments,
                extent,
                1,
                rhapsody::rg::RenderingPolicy::Auto,
            ) {
                Ok(prepared) => prepared,
                Err(error) => {
                    ctx.report_error(error);
                    return false;
                }
            };
            if let Err(error) = prepared_pass.retain_for_pass(&ctx) {
                ctx.report_error(error);
                return false;
            }

            // SAFETY: command recording and all referenced owners belong to this device.
            let mut backend = unsafe { AshCommandBackend::new_for_device(&device) };
            let mut api = RhRenderPassApi::new(&mut backend, command, 1, false);
            if let Err(error) = api.begin_prepared(&prepared_pass) {
                ctx.report_error(error);
                return false;
            }
            api.set_default_view_and_scissor(extent);
            api.set_stencil_ref(0);

            let mut page_cursor = shared.shaders.page_cursor();
            let frozen_globals = shared.shaders.frame_globals_snapshot();
            let count_form = plain_indirect_needs_count_form(&device);
            let mut bound_pipeline = None;
            let mut bound_buffers = DrawBufferBindings::default();
            let parameter_chunks = match reserve_parameter_chunks(&shared, &prepared) {
                Ok(chunks) => chunks,
                Err(error) => { ctx.report_error(error); return false; }
            };
            for ((draw, pipeline), parameter_chunk) in prepared.iter().zip(&pipelines).zip(parameter_chunks.as_slice()) {
                country_core::render::declare_probe::rec_draw();
                let probe = country_core::render::declare_probe::start();
                // SAFETY: 管线属于本设备，命令缓冲在录制中。
                if bound_pipeline != Some(pipeline.raw_handle()) {
                    unsafe {
                        device.raw_device().cmd_bind_pipeline(
                            command,
                            vk::PipelineBindPoint::GRAPHICS,
                            pipeline.raw_handle(),
                        )
                    };
                    bound_pipeline = Some(pipeline.raw_handle());
                }
                country_core::render::declare_probe::rec_pipeline(probe);
                let parameters = match bindless_parameters(
                    &shared,
                    &ctx,
                    draw,
                    &bindings,
                    managed,
                    &mut page_cursor,
                    frozen_globals.clone(),
                    parameter_chunk,
                ) {
                    Ok(parameters) => parameters,
                    Err(error) => {
                        ctx.report_error(error);
                        return false;
                    }
                };
                let probe_bind = country_core::render::declare_probe::start();
                // SAFETY: the original pass bindings and graph accesses describe these resources.
                if let Err(error) = unsafe { parameters.bind_pass(&ctx, pipeline.clone()) } {
                    ctx.report_error(error);
                    return false;
                }
                country_core::render::declare_probe::rec_bind(probe_bind);
                // O 0x141698690..0x141698742 (per draw, after push_constants): when the state's
                // stencil test bit is set (byte1 bit0) the packet's +0xb1 byte is the stencil
                // reference (deferred material id); line topologies / LINE polygon mode take
                // their width from the packet's blend float (+0xac).
                let state = draw.packet.state.0;
                if state & 0x100 != 0 {
                    api.set_stencil_ref(draw.packet.unknown_b1);
                }
                match state & 3 {
                    0 if state >> 31 != 0 => api.set_line_width(draw.packet.blend),
                    1 | 2 => api.set_line_width(draw.packet.blend),
                    _ => {}
                }
                let probe_draw = country_core::render::declare_probe::start();
                if let Err(error) = record_draw(&mut api, draw, managed, count_form, &mut bound_buffers) {
                    ctx.report_error(error);
                    return false;
                }
                country_core::render::declare_probe::rec_draw_call(probe_draw);
            }
            api.end_render_pass();
            true
        });
    }

    fn rg_resources(&mut self) -> Option<&mut dyn RgResources> {
        self.rg.as_mut().map(|r| r as &mut dyn RgResources)
    }
}

/// B render callback @141698d40/8ece/903a: the u64 packet payloads
/// are offset, low-u32 drawCount and low-u32 stride. Counted draws require
/// the feature actually enabled at device creation, not merely driver support.
fn admit_draw_kind(kind: &DrawKind, device: &rhapsody::device::Device) -> rhapsody::Result<()> {
    use rhapsody::device::Feature;
    let count = match kind {
        DrawKind::MultiDrawIndirect { arg_50, .. } | DrawKind::IndexedIndirect { arg_50, .. } => {
            *arg_50 as u32
        }
        DrawKind::IndexedIndirectCount { args, .. } => {
            device.caps().require_enabled(Feature::DrawIndirectCount)?;
            args[3] as u32
        }
        _ => return Ok(()),
    };
    if count > 1 {
        device.caps().require_enabled(Feature::MultiDrawIndirect)?;
    }
    if count > device.max_draw_indirect_count() {
        return Err(rhapsody::Error::invalid_usage(
            "indirect draw count exceeds the actual device limit",
        ));
    }
    Ok(())
}

fn validate_indirect_ranges(graph: &RhGraph, kind: &DrawKind) -> rhapsody::Result<()> {
    let (buffer, offset, count, stride, command_size, counter) = match kind {
        DrawKind::MultiDrawIndirect {
            buffer,
            arg_50,
            arg_58,
        } => (*buffer, 0, *arg_50 as u32, *arg_58 as u32, 16, None),
        DrawKind::IndexedIndirect {
            buffer,
            arg_48,
            arg_50,
            arg_58,
            ..
        } => (*buffer, *arg_48, *arg_50 as u32, *arg_58 as u32, 20, None),
        DrawKind::IndexedIndirectCount {
            buffer,
            count_buffer,
            args,
        } => (
            *buffer,
            args[0],
            args[3] as u32,
            args[1] as u32,
            20,
            Some((*count_buffer, args[2])),
        ),
        _ => return Ok(()),
    };
    let size = |id: u32| -> rhapsody::Result<u64> {
        graph.resource_handle(id)?;
        match graph.declaration().resources[id as usize].descriptor {
            rhapsody::rg::Descriptor::Buffer(desc) => Ok(desc.word_at_00),
            _ => Err(rhapsody::Error::invalid_usage(
                "indirect command resource is not a buffer",
            )),
        }
    };
    if offset % 4 != 0 || (count > 1 && (stride < command_size || stride % 4 != 0)) {
        return Err(rhapsody::Error::invalid_usage(
            "indirect command offset/stride is not Vulkan-compatible",
        ));
    }
    let span = if count == 0 {
        0
    } else {
        u64::from(count - 1) * u64::from(stride) + u64::from(command_size)
    };
    let buffer_size = size(buffer)?;
    if offset.checked_add(span).is_none_or(|end| end > buffer_size) {
        return Err(rhapsody::Error::invalid_usage(
            "indirect command range exceeds its real buffer",
        ));
    }
    if let Some((id, offset)) = counter {
        let size = size(id)?;
        if offset % 4 != 0 || offset.checked_add(4).is_none_or(|end| end > size) {
            return Err(rhapsody::Error::invalid_usage(
                "indirect count range exceeds its real buffer",
            ));
        }
    }
    Ok(())
}

/// Plain (non-count) indirect draws are recorded as one count-form call per command on the
/// Mesa llvmpipe/lavapipe driver.
///
/// Observed on lavapipe (Mesa 25.2.8): inside one frame, a plain `vkCmdDrawIndirect` /
/// `vkCmdDrawIndexedIndirect` that follows a multi-draw (`drawCount > 1`) or a count-form draw
/// whose GPU-written commands are all empty (second occlusion phase of Nani: nothing new to draw)
/// is silently dropped, even in a different render pass. The grass LOD0/LOD1 draws are the last
/// draws of "G-buffer OC/2" and vanished from the second frame on, while the first frame (every
/// Nani draw non-empty) was correct. The command stream is valid (validation layers report no
/// draw error) and the indirect data was verified on the GPU, so the draws are re-expressed
/// without changing their meaning: command `i` is drawn with `maxDrawCount = 1` and its own
/// `instanceCount` field as the count, which is exactly "draw it unless it has no instances".
/// No raster shader reads `DrawIndex`, so splitting a multi-draw is exact. Needs
/// `DrawIndirectCount`; other drivers keep the plain calls.
fn plain_indirect_needs_count_form(device: &rhapsody::device::Device) -> bool {
    use rhapsody::device::Feature;
    static LLVMPIPE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !device.caps().is_enabled(Feature::DrawIndirectCount) {
        return false;
    }
    *LLVMPIPE.get_or_init(|| {
        let mut driver = vk::PhysicalDeviceDriverProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut driver);
        // SAFETY: the physical device belongs to the instance it is queried through.
        unsafe {
            device
                .instance()
                .raw_instance()
                .get_physical_device_properties2(device.physical_device(), &mut properties);
        }
        driver.driver_id == vk::DriverId::MESA_LLVMPIPE
    })
}

/// State actually emitted by this raster callback, never guessed from packets.
/// Empty/zero-index draws leave the previous native buffer bindings intact.
#[derive(Default)]
struct DrawBufferBindings<'a> {
    vertices: Option<&'a [(u32, ResourceHandle, u64)]>,
    index: Option<(ResourceHandle, vk::IndexType)>,
}

fn bind_draw_index<B: rhapsody::rg::CommandBackend>(
    api: &mut RhRenderPassApi<'_, B>,
    index: ResourceHandle,
    managed: &ManagedPassResources,
    bindings: &mut DrawBufferBindings<'_>,
) -> rhapsody::Result<()> {
    let key = (index, vk::IndexType::UINT32);
    if bindings.index != Some(key) {
        let buffer = managed.buffer_for(index.raw())?;
        api.bind_index_buffer(buffer.raw(), key.1);
        bindings.index = Some(key);
    }
    Ok(())
}

fn record_draw<'a, B: rhapsody::rg::CommandBackend>(
    api: &mut RhRenderPassApi<'_, B>,
    draw: &'a PreparedDraw,
    managed: &ManagedPassResources,
    count_form: bool,
    bindings: &mut DrawBufferBindings<'a>,
) -> rhapsody::Result<()> {
    // O 0x1416988f8: reflected location selects the binding point. Pipeline
    // formats/strides were produced from that very same compiled reflection.
    if !draw.vertices.is_empty() && bindings.vertices != Some(draw.vertices.as_slice()) {
        for (binding, resource, _) in &draw.vertices {
            let buffer = managed.buffer_for(resource.raw())?;
            api.bind_vertex_buffer_by_binding_point(*binding, buffer.raw());
        }
        bindings.vertices = Some(&draw.vertices);
    }
    match &draw.packet.kind {
        // 全屏三角形/SSBO 拉取顶点：无顶点缓冲，`vkCmdDraw(count, 1, 0, 0)`（O：`draw_no_vao(3)`）。
        DrawKind::NoVao { vertex_count } | DrawKind::NoIndexBuffer { vertex_count } => {
            api.draw(*vertex_count, 1, 0, 0);
        }
        // 索引缓冲 `vao.rs`：`indices.len() * 4` 字节 → UINT32（O）。
        DrawKind::NoVaoIndexed { index_count, .. } => {
            if *index_count == 0 {
                return Ok(());
            }
            let index = draw
                .index
                .ok_or_else(|| rhapsody::Error::invalid_usage("index access was not declared"))?;
            bind_draw_index(api, index, managed, bindings)?;
            api.draw_indexed(*index_count, 1, 0, 0, 0);
        }
        DrawKind::InstancedGlDrawFlag { flag } => {
            let index = draw.index.ok_or_else(|| {
                rhapsody::Error::invalid_usage("mesh index access was not declared")
            })?;
            bind_draw_index(api, index, managed, bindings)?;
            // O 0x141698cae..d30: VAO entry index_count, payload as instance
            // count; firstIndex, vertexOffset, firstInstance are all zero.
            api.draw_indexed(
                draw.index_count.expect("prepared VAO index count"),
                *flag,
                0,
                0,
                0,
            );
        }
        DrawKind::MultiInstanced { ranges } => {
            let index = draw.index.ok_or_else(|| {
                rhapsody::Error::invalid_usage("mesh index access was not declared")
            })?;
            bind_draw_index(api, index, managed, bindings)?;
            // O 0x141698bd0..c32: pair[0] -> firstInstance,
            // pair[1] -> instanceCount, same actual VAO index count.
            for [first, count] in ranges {
                api.draw_indexed(
                    draw.index_count.expect("prepared VAO index count"),
                    *count,
                    0,
                    0,
                    *first,
                );
            }
        }
        DrawKind::MultiDrawIndirect {
            buffer,
            arg_50,
            arg_58,
        } => {
            let buffer = managed.buffer_for(*buffer)?;
            // B @141698f89..8f96: stride on stack, count in r9d, offset r8.
            if count_form {
                // Same commands, one count-form call each (see `plain_indirect_needs_count_form`).
                let stride = if *arg_50 > 1 { *arg_58 } else { 0 };
                for i in 0..*arg_50 {
                    let offset = i * stride;
                    api.draw_indirect_count(buffer.raw(), offset, buffer.raw(), offset + 4, 1, 16);
                }
            } else {
                api.draw_indirect(buffer.raw(), 0, *arg_50 as u32, *arg_58 as u32);
            }
        }
        DrawKind::IndexedIndirect {
            buffer,
            arg_48,
            arg_50,
            arg_58,
            ..
        } => {
            bind_draw_index(api, draw.index.expect("prepared actual indirect index owner"), managed, bindings)?;
            let buffer = managed.buffer_for(*buffer)?;
            // B @141698dfb..8e08, optional indexCount metadata is not read.
            if count_form {
                let stride = if *arg_50 > 1 { *arg_58 } else { 0 };
                for i in 0..*arg_50 {
                    let offset = *arg_48 + i * stride;
                    api.draw_indexed_indirect_count(buffer.raw(), offset, buffer.raw(), offset + 4, 1, 20);
                }
            } else {
                api.draw_indexed_indirect(buffer.raw(), *arg_48, *arg_50 as u32, *arg_58 as u32);
            }
        }
        DrawKind::IndexedIndirectCount {
            buffer,
            count_buffer,
            args,
        } => {
            bind_draw_index(api, draw.index.expect("prepared actual indirect index owner"), managed, bindings)?;
            let buffer = managed.buffer_for(*buffer)?;
            let count = managed.buffer_for(*count_buffer)?;
            // B @1416992af..92d3: offset, countOffset, maxCount, stride.
            api.draw_indexed_indirect_count(
                buffer.raw(),
                args[0],
                count.raw(),
                args[2],
                args[3] as u32,
                args[1] as u32,
            );
        }
    }
    Ok(())
}

/// Resolve external owners before pass construction. The callback subsequently
/// uses only the graph's managed projection, so host handle release cannot turn
/// an already declared draw into a stale descriptor.
/// Original global set 1/b1 is a storage view of the per-frame FrameConstants block. A storage
/// lease must be covered by a declared graph access of its owner, so a raster draw whose shader
/// declares set 1/b1 imports the exact frame constants range as a graphics-stage shader read
/// (same contract as the compute host's `frame_storage_view` import).
fn import_frame_constants(
    shared: &RasterShared,
    draw: &mut PreparedDraw,
    declarer: &dyn GraphDeclarer,
    graph: &mut RhGraph,
) -> rhapsody::Result<()> {
    let abi = &draw.entry.bindings.abi;
    if abi.slot(BindingLocation { set: 1, binding: 0 }).is_none()
        && abi.slot(BindingLocation { set: 1, binding: 1 }).is_none()
    {
        return Ok(());
    }
    let graph_id = graph.graph_id();
    if let Some((cached, constants)) = *shared.frame_constants.lock()
        && cached == graph_id
    {
        draw.frame_constants = Some(constants);
        return Ok(());
    }
    let (buffer, offset, size) = draw.entry.bindings.frame_storage_view()?;
    let resource = declarer.import_buffer(graph, buffer, 0x35)?;
    let constants = (resource, rhapsody::rg::AccessRange::Buffer { offset, size });
    *shared.frame_constants.lock() = Some((graph_id, constants));
    draw.frame_constants = Some(constants);
    Ok(())
}

fn import_draw_bindings(
    shared: &RasterShared,
    draw: &mut PreparedDraw,
    declarer: &dyn GraphDeclarer,
    graph: &mut RhGraph,
) -> rhapsody::Result<()> {
    use rhapsody::rg::AccessRange;
    use shader_pipeline::reflection::BindingKind;
    for (binding, owned) in &draw.packet.bindings {
        if owned.state != country_core::render::pass_builder::draw_stream::GpuOwned::STATE_OWNED {
            continue;
        }
        let external = match &owned.binding {
            CoreBinding::StorageBuffer {
                buffer,
                offset,
                range,
                keepalive,
            } => {
                if !matches!(
                    draw.entry
                        .bindings
                        .abi
                        .slot(BindingLocation {
                            set: 0,
                            binding: *binding
                        })
                        .map(|slot| slot.kind),
                    Some(BindingKind::StorageBuffer | BindingKind::UniformBuffer)
                ) {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster buffer binding differs from shader ABI",
                    ));
                }
                let owner = shared.buffer_host().get_by_id(keepalive.0).ok_or_else(|| {
                    rhapsody::Error::invalid_usage("raster storage buffer is not live")
                })?;
                if ash::vk::Handle::as_raw(owner.raw_handle()) != *buffer {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster storage buffer differs from retained owner",
                    ));
                }
                let size = if *range == vk::WHOLE_SIZE {
                    owner.size.checked_sub(*offset).ok_or_else(|| {
                        rhapsody::Error::invalid_usage("raster buffer offset exceeds its size")
                    })?
                } else {
                    *range
                };
                if size == 0 || offset.checked_add(size).is_none_or(|end| end > owner.size) {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster buffer binding exceeds its owner",
                    ));
                }
                // Original owned SSBO bindings do not preserve readonly metadata.
                // Declare the actual bytes with conservative shader-compatible RW.
                ExternalDrawBinding {
                    binding: *binding,
                    resource: declarer.import_buffer(graph, owner.shared(), 0x37)?,
                    access: 0x37,
                    range: AccessRange::Buffer {
                        offset: *offset,
                        size,
                    },
                    view: None,
                }
            }
            CoreBinding::SampledImage {
                image_view,
                layout_word,
                keepalive,
            } => {
                let view = shared.textures.owned_view(keepalive.0, *image_view)?;
                let kind = draw
                    .entry
                    .bindings
                    .abi
                    .slot(BindingLocation {
                        set: 0,
                        binding: *binding,
                    })
                    .ok_or_else(|| {
                        rhapsody::Error::invalid_usage(
                            "raster image binding is absent from the shader ABI",
                        )
                    })?
                    .kind;
                let (access, layout, initial_access) = match kind {
                    BindingKind::SampledImage => (0x1d, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL, 0x22),
                    BindingKind::StorageImage => (0x37, vk::ImageLayout::GENERAL, 0x37),
                    _ => {
                        return Err(rhapsody::Error::invalid_usage(
                            "raster image binding differs from shader ABI",
                        ));
                    }
                };
                if *layout_word != u64::try_from(layout.as_raw()).expect("positive image layout") {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster image layout differs from its shader ABI",
                    ));
                }
                ExternalDrawBinding {
                    binding: *binding,
                    resource: declarer.import_texture(graph, view.texture.clone(), initial_access)?,
                    access,
                    range: AccessRange::Texture {
                        aspect_mask: view.desc.aspect.as_raw(),
                        base_mip_level: view.desc.base_mip_level,
                        level_count: view.desc.level_count,
                        base_array_layer: view.desc.base_layer,
                        layer_count: view.desc.layer_count,
                    },
                    view: Some(view.desc),
                }
            }
            CoreBinding::UniformBuffer { .. } => continue,
        };
        draw.external.add(external)?;
    }
    Ok(())
}

/// Allocate each original chunk through the existing bounded page allocator,
/// with one lock acquisition for this callback. Alignment, per-page write
/// limits and actual GPU completion still belong to that allocator.
fn reserve_parameter_chunks(
    shared: &RasterShared,
    draws: &[PreparedDraw],
) -> rhapsody::Result<rhapsody::resource::TempList<DynamicConstantsChunk>> {
    let mut chunks = rhapsody::resource::TempList::new(
        draws.len().max(1), shared.device.allocator_stats().clone(),
    )?;
    chunks.reserve(draws.len())?;
    let mut constants = shared.dynamic.lock();
    for draw in draws {
        chunks.add(constants.allocate(u64::from(draw.entry.bindings.abi.parameter_bytes))?)?;
    }
    Ok(chunks)
}

fn bindless_parameters(
    shared: &RasterShared,
    ctx: &rhapsody::rg::PassContext<'_>,
    draw: &PreparedDraw,
    pass_bindings: &[RenderPassBinding],
    managed: &ManagedPassResources,
    cursor: &mut rhapsody::bindless::BindlessPageCursor,
    globals: Option<Arc<crate::frame_setup::FrameGlobals>>,
    parameter_chunk: &rhapsody::resource::DynamicConstantsChunk,
) -> rhapsody::Result<rhapsody::bindless::PreparedBindlessParameters> {
    shared.count_management(5, 1);
    let probe_resources = country_core::render::declare_probe::start();
    let mut resources = Vec::new();
    for (binding, owned) in &draw.packet.bindings {
        if owned.state != country_core::render::pass_builder::draw_stream::GpuOwned::STATE_OWNED {
            continue;
        }
        let value = match &owned.binding {
            CoreBinding::StorageBuffer { .. } | CoreBinding::SampledImage { .. } => continue,
            CoreBinding::UniformBuffer {
                size,
                keepalive,
                lease,
            } => {
                let chunk = lease
                    .as_ref()
                    .and_then(ResourceLease::downcast_ref::<DynamicConstantsChunk>)
                    .ok_or_else(|| {
                        rhapsody::Error::invalid_usage(
                            "uniform binding: missing dynamic constants lease",
                        )
                    })?;
                if u64::from(*size) > chunk.size
                    || keepalive.0 != ash::vk::Handle::as_raw(chunk.buffer.raw_handle())
                {
                    return Err(rhapsody::Error::invalid_usage(
                        "uniform binding metadata exceeds owned allocation",
                    ));
                }
                BindlessResource::Buffer {
                    buffer: chunk.buffer.clone(),
                    offset: chunk.offset,
                    size: u64::from(*size),
                }
            }
        };
        shared.count_management(4, 1);
        upsert_shader_resource(&mut resources, *binding, value)?;
    }
    for binding in draw.external.as_slice() {
        let value = if let Some(desc) = &binding.view {
            let texture = managed
                .texture_for(binding.resource.raw())
                .map_err(rhapsody::Error::from)?;
            let view = texture.image_view(desc)?;
            draw.entry
                .bindings
                .image(binding.binding, view.view.clone())?
        } else {
            let buffer = managed
                .buffer_for(binding.resource.raw())
                .map_err(rhapsody::Error::from)?;
            let rhapsody::rg::AccessRange::Buffer { offset, size } = binding.range else {
                unreachable!("buffer binding has a checked buffer range")
            };
            BindlessResource::Buffer {
                buffer: buffer.shared(),
                offset,
                size: crate::render_host::clamp_storage_binding_range(&shared.device, size),
            }
        };
        shared.count_management(4, 1);
        upsert_shader_resource(&mut resources, binding.binding, value)?;
    }
    for access in &draw.packet.accesses {
        let binding = pass_bindings
            .get(access.slot as usize)
            .ok_or_else(|| rhapsody::Error::invalid_usage("raster pass binding slot is absent"))?;
        let resource = if let Ok(texture) = managed.texture_for(binding.resource_index) {
            let desc = binding.view.as_ref().map_or_else(
                || crate::render_host::rh_view_desc(&texture, &country_core::rhapsody_types::TextureViewDesc::new()),
                |view| rh_view_desc(&texture, view),
            )?;
            let view = texture.image_view(&desc)?;
            // The descriptor layout is the layout this pass's declared access planned for the
            // view (a depth image read as attachment and sampled is DEPTH_STENCIL_READ_ONLY).
            draw.entry.bindings.image_for_pass(
                ctx,
                binding.resource_index,
                access.binding_index,
                view.view.clone(),
            )?
        } else {
            let buffer = managed
                .buffer_for(binding.resource_index)
                .map_err(|error| {
                    rhapsody::Error::invalid_usage_with("raster managed buffer binding", error)
                })?;
            BindlessResource::Buffer {
                buffer: buffer.shared(),
                offset: 0,
                size: crate::render_host::clamp_storage_binding_range(
                    &shared.device,
                    buffer.size,
                ),
            }
        };
        shared.count_management(4, 1);
        upsert_shader_resource(&mut resources, access.binding_index, resource)?;
    }
    // B EguiBackend::draw 0x1400def7e..0x1400defd0 stores the 8-byte
    // screen-size push payload in DrawPacket's byte Vec. Other shaders may use
    // an empty push block; ABI validation retains that distinction.
    country_core::render::declare_probe::rec_resources(probe_resources);
    let probe_prepare = country_core::render::declare_probe::start();
    let snapshot = draw.entry.bindings.snapshot_with_globals(globals);
    let prepared = snapshot.prepare(
        cursor,
        &draw.packet.unknown_bytes,
        resources.iter().map(|(binding, resource)| {
            (
                BindingLocation {
                    set: 0,
                    binding: *binding,
                },
                0,
                resource,
                false,
            )
        }),
        None,
        |_| Ok(rhapsody::resource::DynamicConstantsChunk {
            buffer: parameter_chunk.buffer.clone(), offset: parameter_chunk.offset, size: parameter_chunk.size,
        }),
    );
    country_core::render::declare_probe::rec_prepare(probe_prepare);
    prepared
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.component_journaled::<country_core::asset_libraries::Handle<country_core::render::vao::Vao>>();
    m.asset_library::<country_core::asset_libraries::vao_library::AssetVAOLibrary>();
}

#[cfg(test)]
mod program_identity_tests {
    use super::*;
    use country_core::render::shader::boundary::{
        CompiledShaderPipeline, ComputePipeline, ComputePipelineDesc, Error, RasterPipeline,
        RasterPipelineDesc,
    };

    struct Artifacts;
    impl ShaderBackend for Artifacts {
        fn load_compiled_compute(
            &self,
            _: &ComputePipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            unreachable!()
        }
        fn load_compiled_raster(
            &self,
            desc: &RasterPipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            Ok(CompiledShaderPipeline(desc.path.as_bytes().to_vec()))
        }
        fn raster_shader_variants(&self, desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
            [desc.clone(), desc.clone()]
        }
        fn create_compute_pipeline(
            &self,
            _: &str,
            _: Arc<CompiledShaderPipeline>,
        ) -> rhapsody::Result<ComputePipeline> {
            unreachable!()
        }
        fn create_raster_pipeline(
            &self,
            _: Arc<CompiledShaderPipeline>,
            _: &RasterPipelineKey,
        ) -> Result<RasterPipeline, Error> {
            unreachable!()
        }
    }

    fn program(name: &'static str) -> RasterShaderProgram {
        RasterShaderProgram::new_draw(
            name,
            RasterPipelineDesc {
                path: name.into(),
                ..Default::default()
            },
            &Artifacts,
        )
    }

    #[test]
    fn core_source_uses_authoritative_handles_and_sees_later_insertions() {
        let library = Arc::new(AssetShaderLibrary::new());
        let _padding = country_core::asset_libraries::Handle::<()>::new();
        let handle = library.add_raster(program("game"), false);
        assert!(handle.id().raw() > 1);
        assert!(
            library
                .get_raster_by_id(handle.id().raw()).map(|resolved| {
                    std::ptr::eq(resolved, library.get_raster(handle).unwrap())
                })
                .unwrap()
        );
        assert!(library.get_raster_by_id(0).is_none());
        assert!(library.get_raster_by_id(u64::MAX).is_none());
    }

    #[test]
    fn shader_handles_do_not_alias_between_authoritative_libraries() {
        let first = AssetShaderLibrary::new();
        let second = AssetShaderLibrary::new();
        let handle = first.add_raster(program("first"), false);
        second.add_raster(program("second"), false);
        assert!(second.get_raster_by_id(handle.id().raw()).is_none());
        assert_eq!(first.get_raster(handle).unwrap().name, "first");
    }

}

/// Original new_impl gives every attachment mip zero / level_count one.
/// Only depth adds layer_count one and the caller's base_layer. Color's four
/// payload words are a clear value and never change the subresource view.
struct RasterAttachmentAccess {
    resource: u32,
    access: u8,
    load: bool,
    view: country_core::rhapsody_types::TextureViewDesc,
}

fn raster_attachment_accesses(
    colors: &[RasterPassColorAttachment],
    depth: Option<&RasterPassDepth>,
) -> rhapsody::Result<Vec<RasterAttachmentAccess>> {
    use country_core::render::pass_builder::framebuffer_binding::{
        COLOR_ACCESS_TEXTURE, COLOR_ACCESS_VIEW, DEPTH_ACCESS_READ, DEPTH_ACCESS_WRITE,
    };
    let mut attachments = Vec::new();
    attachments
        .try_reserve_exact(colors.len() + usize::from(depth.is_some()))
        .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    for color in colors {
        if color.tag > 2 {
            return Err(rhapsody::Error::invalid_usage(
                "invalid native color attachment tag",
            ));
        }
        attachments.push(RasterAttachmentAccess {
            resource: color.id,
            access: if color.tag == 0 {
                COLOR_ACCESS_TEXTURE
            } else {
                COLOR_ACCESS_VIEW
            },
            load: color.tag == 0,
            view: country_core::rhapsody_types::TextureViewDesc::new().level_count(Some(1)),
        });
    }
    if let Some(depth) = depth {
        attachments.push(RasterAttachmentAccess {
            resource: depth.texture_id,
            access: if depth.writable {
                DEPTH_ACCESS_WRITE
            } else {
                DEPTH_ACCESS_READ
            },
            load: matches!(depth.clear, DepthClear::Load),
            view: country_core::rhapsody_types::TextureViewDesc::new()
                .level_count(Some(1))
                .base_layer(depth.layer)
                .layer_count(Some(1)),
        });
    }
    Ok(attachments)
}

/// A preserving attachment must depend only on earlier writes to its selected
/// subresources. Source initialization remains checked by the production graph.
fn raster_has_previous_contents(
    graph: &RhGraph,
    resource: u32,
    range: rhapsody::rg::AccessRange,
) -> bool {
    let imported = graph
        .declaration()
        .resources
        .get(resource as usize)
        .is_some_and(|entry| match entry.source {
            rhapsody::rg::OriginSource::ImportOwned { initial_access, .. }
            | rhapsody::rg::OriginSource::ImportUnowned { initial_access, .. } => {
                initial_access != 0
            }
            rhapsody::rg::OriginSource::Create => false,
        });
    imported
        || graph.declaration().passes.iter().any(|pass| {
            pass.accesses.iter().any(|access| {
                access.resource_index == resource
                    && rhapsody::command::is_write(access.access_code)
                    && access.range.overlaps(range)
            })
        })
}

/// Project the exact producer/Commit mapping once. Unbound and attachment
/// accesses remain independent; repeated resource views never require a search.
fn raster_binding_ranges(
    accesses: &[country_core::render::pass_builder::draw_stream::PassAccess],
    bindings: &[RenderPassBinding],
    binding_accesses: &[usize],
    attachment_accesses: usize,
    mut descriptor: impl FnMut(u32) -> rhapsody::Result<rhapsody::rg::Descriptor>,
) -> rhapsody::Result<Vec<(rhapsody::rg::AccessRange, Option<u32>)>> {
    use rhapsody::rg::{AccessRange, Descriptor};
    if attachment_accesses > accesses.len() || bindings.len() != binding_accesses.len() {
        return Err(rhapsody::Error::invalid_usage("raster binding access mapping is incomplete"));
    }
    let mut ranges = Vec::new();
    ranges.try_reserve_exact(accesses.len()).map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    ranges.resize(accesses.len(), (AccessRange::Whole, None));
    for (slot, (binding, &access_index)) in bindings.iter().zip(binding_accesses).enumerate() {
        let access = accesses.get(access_index).ok_or_else(||
            rhapsody::Error::invalid_usage("raster binding access mapping is stale"))?;
        if access_index < attachment_accesses || ranges[access_index].1.is_some()
            || binding.resource_index != access.resource || binding.access_type != access.access {
            return Err(rhapsody::Error::invalid_usage("raster binding differs from its native resource access"));
        }
        let desc = descriptor(binding.resource_index)?;
        let range = match desc {
            Descriptor::Image(_) => compute_view_range(desc, &binding.view.unwrap_or_default())?,
            Descriptor::Buffer(_) if binding.view.is_none() => AccessRange::Whole,
            Descriptor::Buffer(_) => return Err(rhapsody::Error::invalid_usage("raster texture view refers to a buffer")),
        };
        let slot = u32::try_from(slot).map_err(|_| rhapsody::Error::invalid_usage("raster pass binding index exceeds u32"))?;
        ranges[access_index] = (range, Some(slot));
    }
    Ok(ranges)
}

/// 合并按严格程度排序；默认读取不能继承其它 draw 的 undefined 许可。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RasterReadContents {
    Inactive,
    UndefinedAllowed,
    DefinedRequired,
}

/// Resolve packet-local evidence onto the real graph resource. Any active
/// alias of that same pass slot tightens it back into a content read. Different
/// views of one image remain distinct until the graph evaluates their ranges.
fn raster_content_intents<'a>(
    packets: impl IntoIterator<Item = &'a DrawPacket>,
    bindings: &[RenderPassBinding],
    mut is_texture: impl FnMut(u32) -> rhapsody::Result<bool>,
) -> rhapsody::Result<(Vec<u32>, std::collections::BTreeMap<u32, RasterReadContents>)> {
    use country_core::render::pass_builder::draw_stream::access_is_write;
    let mut preserving = Vec::new();
    let mut reads = std::collections::BTreeMap::new();
    for packet in packets {
        for binding in packet
            .preserving_buffer_bindings
            .iter()
            .chain(&packet.inactive_texture_bindings)
            .chain(&packet.undefined_texture_bindings)
        {
            if !packet
                .accesses
                .iter()
                .any(|access| access.binding_index == *binding)
            {
                return Err(rhapsody::Error::invalid_usage(
                    "raster contents metadata lacks its actual binding",
                ));
            }
        }
        for binding in &packet.undefined_texture_bindings {
            if packet
                .accesses
                .iter()
                .filter(|access| access.binding_index == *binding)
                .count()
                != 1
                || packet.bindings.contains_key(binding)
            {
                return Err(rhapsody::Error::invalid_usage(
                    "undefined raster metadata requires one current graph binding",
                ));
            }
        }
        for access in &packet.accesses {
            let actual = bindings.get(access.slot as usize).ok_or_else(|| {
                rhapsody::Error::invalid_usage("raster contents metadata has a stale pass slot")
            })?;
            let texture = is_texture(actual.resource_index)?;
            let preserves = packet
                .preserving_buffer_bindings
                .contains(&access.binding_index);
            let unused = packet
                .inactive_texture_bindings
                .contains(&access.binding_index);
            let undefined = packet
                .undefined_texture_bindings
                .contains(&access.binding_index);
            if undefined && (!texture || access_is_write(actual.access_type) || unused || preserves)
            {
                return Err(rhapsody::Error::invalid_usage(
                    "undefined raster metadata requires a real non-writing texture read",
                ));
            }
            if preserves {
                if texture || !access_is_write(actual.access_type) || unused {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster preserving metadata requires an actual buffer write",
                    ));
                }
                if !preserving.contains(&actual.resource_index) {
                    preserving.push(actual.resource_index);
                }
            }
            // Inactive intent covers non-writing texture reads and non-writing
            // storage-buffer reads (the buffer variant mirrors compute's shared
            // per-binding inactive list).
            if unused && access_is_write(actual.access_type) {
                return Err(rhapsody::Error::invalid_usage(
                    "raster inactive metadata requires an actual non-writing read",
                ));
            }
            if !access_is_write(actual.access_type) {
                let contents = if unused {
                    RasterReadContents::Inactive
                } else if undefined {
                    RasterReadContents::UndefinedAllowed
                } else {
                    RasterReadContents::DefinedRequired
                };
                reads
                    .entry(access.slot)
                    .and_modify(|current: &mut RasterReadContents| {
                        *current = (*current).max(contents)
                    })
                    .or_insert(contents);
            }
        }
    }
    Ok((preserving, reads))
}

#[cfg(test)]
mod original_surface_raster_intent_tests {
    use super::*;
    use country_core::render::pass_builder::draw_stream::ResourceAccess;
    fn packet() -> DrawPacket {
        DrawPacket::new(
            country_core::rhapsody_contract::Handle(
                country_core::rhapsody_contract::ManagedHandle(1),
                std::marker::PhantomData,
            ),
            [0, 0],
            RasterState::default(),
        )
    }
    #[test]
    fn active_aliases_tighten_reads_and_atomic_preserving_is_explicit() {
        let bindings = vec![
            RenderPassBinding {
                resource_index: 42,
                access_type: 0x1d,
                view: Some(Default::default()),
            },
            RenderPassBinding {
                resource_index: 43,
                access_type: 0x37,
                view: None,
            },
        ];
        let mut first = packet();
        first.accesses = vec![
            ResourceAccess {
                binding_index: 0,
                slot: 0,
            },
            ResourceAccess {
                binding_index: 1,
                slot: 1,
            },
        ];
        first.inactive_texture_bindings.push(0);
        first.preserving_buffer_bindings.push(1);
        let (preserving, inactive) =
            raster_content_intents([&first], &bindings, |id| Ok(id == 42)).unwrap();
        assert_eq!(preserving, [43]);
        assert_eq!(inactive[&0], RasterReadContents::Inactive);
        let mut active = packet();
        active.accesses.push(ResourceAccess {
            binding_index: 0,
            slot: 0,
        });
        let (_, inactive) =
            raster_content_intents([&first, &active], &bindings, |id| Ok(id == 42)).unwrap();
        assert_eq!(inactive[&0], RasterReadContents::DefinedRequired);
        first.inactive_texture_bindings.push(1);
        assert!(raster_content_intents([&first], &bindings, |id| Ok(id == 42)).is_err());
    }
}

#[cfg(test)]
mod undefined_raster_content_tests {
    use super::*;
    use country_core::render::pass_builder::draw_stream::ResourceAccess;
    use rhapsody::rg::{AccessRange, AccessTable, GraphDefinition, PlanContext, ResourceSpec};

    fn packet(intent: RasterReadContents, binding: u32, slot: u32) -> DrawPacket {
        let mut packet = DrawPacket::new(
            country_core::rhapsody_contract::Handle(ManagedHandle(1), std::marker::PhantomData),
            [0, 0],
            RasterState::default(),
        );
        packet.accesses.push(ResourceAccess {
            binding_index: binding,
            slot,
        });
        match intent {
            RasterReadContents::Inactive => packet.inactive_texture_bindings.push(binding),
            RasterReadContents::UndefinedAllowed => packet.undefined_texture_bindings.push(binding),
            RasterReadContents::DefinedRequired => {}
        }
        packet
    }
    fn binding() -> RenderPassBinding {
        RenderPassBinding {
            resource_index: 0,
            access_type: 0x1d,
            view: Some(Default::default()),
        }
    }
    fn intents() -> [RasterReadContents; 3] {
        [
            RasterReadContents::Inactive,
            RasterReadContents::UndefinedAllowed,
            RasterReadContents::DefinedRequired,
        ]
    }

    #[test]
    fn same_slot_merges_all_packet_orders_with_strict_reads_winning() {
        for first in intents() {
            for second in intents() {
                let a = packet(first, 3, 0);
                let b = packet(second, 4, 0);
                for packets in [[&a, &b], [&b, &a]] {
                    let (_, reads) =
                        raster_content_intents(packets, &[binding()], |_| Ok(true)).unwrap();
                    assert_eq!(reads[&0], first.max(second));
                }
            }
        }
    }

    #[test]
    fn stale_rebound_or_conflicting_metadata_is_rejected_instead_of_granting_permission() {
        for case in 0..6 {
            let mut packet = packet(RasterReadContents::UndefinedAllowed, 3, 0);
            let mut binding = binding();
            match case {
                0 => packet.inactive_texture_bindings.push(3),
                1 => packet.accesses.push(ResourceAccess {
                    binding_index: 3,
                    slot: 0,
                }),
                2 => packet.undefined_texture_bindings.push(99),
                3 => binding.access_type = 0x37,
                4 => packet.accesses[0].slot = 1,
                _ => {}
            }
            assert!(
                raster_content_intents([&packet], &[binding], |_| Ok(case != 5)).is_err(),
                "forbidden metadata case {case}"
            );
        }
    }

    #[test]
    fn different_slot_aliases_reach_the_real_ledger_and_never_initialize_the_source() {
        let descriptor = ResourceSpec::Image(rhapsody::resource::TextureDesc::new_2d(
            8,
            8,
            vk::Format::R32_SFLOAT,
        ))
        .descriptor();
        for first in intents() {
            for second in intents() {
                for reverse in [false, true] {
                    let a = packet(first, 3, 0);
                    let b = packet(second, 4, 1);
                    let (_, reads) =
                        raster_content_intents([&a, &b], &[binding(), binding()], |_| Ok(true))
                            .unwrap();
                    let mut graph = RhGraph::new();
                    let texture = graph
                        .import_unowned("original uninitialized color", descriptor, 1, 0)
                        .unwrap();
                    let mut pass = graph.add_pass("real shader reads").with_side_effect();
                    for slot in if reverse { [1, 0] } else { [0, 1] } {
                        match reads[&slot] {
                            RasterReadContents::Inactive => {
                                pass.add_inactive_resource_read_range(
                                    texture,
                                    0x1d,
                                    1,
                                    AccessRange::Whole,
                                )
                                .unwrap();
                            }
                            RasterReadContents::UndefinedAllowed => {
                                pass.add_undefined_content_read_range(
                                    texture,
                                    0x1d,
                                    1,
                                    AccessRange::Whole,
                                )
                                .unwrap();
                            }
                            RasterReadContents::DefinedRequired => {
                                pass.add_resource_access_range(
                                    texture,
                                    0x1d,
                                    1,
                                    AccessRange::Whole,
                                )
                                .unwrap();
                            }
                        }
                    }
                    pass.render(|_| true);
                    let raw = graph.build().declaration;
                    let strict = first.max(second) == RasterReadContents::DefinedRequired;
                    let key = GraphDefinition::from_declaration(
                        &raw,
                        &mut AccessTable::original(),
                        PlanContext::default(),
                    );
                    assert_eq!(key.is_err(), strict);
                    if let Ok(key) = key {
                        let merged = key.passes[0].accesses[0];
                        assert_eq!(merged.access_code, 0x1d);
                        assert_eq!(merged.synchronization_group, 1);
                        assert_eq!(merged.range, AccessRange::Whole);
                        if first.max(second) == RasterReadContents::UndefinedAllowed {
                            assert!(merged.allow_undefined_contents);
                            assert!(!merged.contents_unused);
                            assert!(merged.reads_contents(false));
                        }
                        let plan = rhapsody::rg::compile_definition(key).unwrap();
                        assert_eq!(plan.retained_passes, [0]);
                        assert!(!plan.info.lifetimes.is_empty());
                        assert_ne!(
                            plan.info.image_usage_flags[0] & vk::ImageUsageFlags::SAMPLED.as_raw(),
                            0
                        );
                        let mut later = raw.clone();
                        let mut access = later.passes[0].accesses[0];
                        access.allow_undefined_contents = false;
                        access.contents_unused = false;
                        let mut strict_pass = later.passes[0].clone();
                        strict_pass.accesses = vec![access];
                        later.passes.push(strict_pass);
                        assert!(
                            GraphDefinition::from_declaration(
                                &later,
                                &mut AccessTable::original(),
                                PlanContext::default()
                            )
                            .is_err(),
                            "permission cannot initialize a later ordinary read"
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod raster_binding_range_tests {
    use super::*;
    use country_core::render::pass_builder::draw_stream::{PassAccess, ResourceAccess};
    use country_core::rhapsody_types::TextureViewDesc;
    use rhapsody::rg::{AccessRange, Descriptor, ResourceSpec};

    fn image() -> Descriptor {
        ResourceSpec::Image(
            rhapsody::resource::TextureDesc::new_2d_array(64, 64, 3, vk::Format::R32_SFLOAT)
                .mip_levels(4)
                .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::STORAGE),
        )
        .descriptor()
    }
    fn binding(mip: u32, layer: u32, access: u8) -> RenderPassBinding {
        RenderPassBinding {
            resource_index: 0,
            access_type: access,
            view: Some(
                TextureViewDesc::new()
                    .base_mip_level(mip)
                    .level_count(Some(1))
                    .base_layer(layer)
                    .layer_count(Some(1)),
            ),
        }
    }
    fn access(code: u8) -> PassAccess {
        PassAccess {
            resource: 0,
            access: code,
            group: 1,
        }
    }
    fn packet() -> DrawPacket {
        DrawPacket::new(
            country_core::rhapsody_contract::Handle(ManagedHandle(1), std::marker::PhantomData),
            [0, 0],
            RasterState::default(),
        )
    }

    #[test]
    fn high_alias_slots_preserve_views_and_independent_content_intents() {
        let bindings: Vec<_> = (0..1200).map(|slot| binding(slot % 4, slot % 3, 0x1d)).collect();
        let accesses = vec![access(0x1d); bindings.len() + 1];
        let ranges = raster_binding_ranges(&accesses, &bindings, &(1..=bindings.len()).collect::<Vec<_>>(), 1, |_| Ok(image())).unwrap();
        assert_eq!(ranges[0], (AccessRange::Whole, None));
        for (slot, range) in ranges[1..].iter().enumerate() {
            assert_eq!(range.1, Some(slot as u32));
            assert_eq!(range.0, compute_view_range(image(), &bindings[slot].view.unwrap()).unwrap());
        }
        let mut draw = packet();
        draw.accesses = vec![
            ResourceAccess { binding_index: 5, slot: 0 },
            ResourceAccess { binding_index: 6, slot: 256 },
            ResourceAccess { binding_index: 7, slot: 1199 },
        ];
        draw.inactive_texture_bindings.push(6);
        draw.undefined_texture_bindings.push(7);
        let (_, reads) = raster_content_intents([&draw], &bindings, |_| Ok(true)).unwrap();
        assert_eq!(reads[&0], RasterReadContents::DefinedRequired);
        assert_eq!(reads[&256], RasterReadContents::Inactive);
        assert_eq!(reads[&1199], RasterReadContents::UndefinedAllowed);
    }

    #[test]
    fn producer_mapping_keeps_nonsequential_views_in_their_real_access_positions() {
        let bindings = [binding(2, 1, 0x1d), binding(3, 2, 0x1d)];
        let accesses = [access(0x1d); 3];
        let ranges = raster_binding_ranges(&accesses, &bindings, &[2, 1], 1, |_| Ok(image())).unwrap();
        assert_eq!(ranges[0], (AccessRange::Whole, None));
        assert_eq!(ranges[1], (compute_view_range(image(), &bindings[1].view.unwrap()).unwrap(), Some(1)));
        assert_eq!(ranges[2], (compute_view_range(image(), &bindings[0].view.unwrap()).unwrap(), Some(0)));
        for mapping in [&[1, 1][..], &[0, 1], &[1], &[1, 3]] {
            assert!(raster_binding_ranges(&accesses, &bindings, mapping, 1, |_| Ok(image())).is_err());
        }
    }

    #[test]
    fn repeated_owner_views_keep_ranges_slots_and_attachment_order() {
        let bindings = [binding(2, 1, 0x1d), binding(3, 2, 0x1d)];
        // The first access has an identical owner/code but belongs to the
        // framebuffer, before the two native shader binding operations.
        let ranges = raster_binding_ranges(
            &[access(0x1d), access(0x1d), access(0x1d)],
            &bindings,
            &[1, 2],
            1,
            |_| Ok(image()),
        )
        .unwrap();
        assert_eq!(ranges[0], (AccessRange::Whole, None));
        assert_eq!(ranges[1].1, Some(0));
        assert_eq!(ranges[2].1, Some(1));
        assert_eq!(
            ranges[1].0,
            AccessRange::Texture {
                aspect_mask: 1,
                base_mip_level: 2,
                level_count: 1,
                base_array_layer: 1,
                layer_count: 1,
            }
        );
        assert!(!ranges[1].0.overlaps(ranges[2].0));
        let mut first = packet();
        first.accesses = vec![
            ResourceAccess {
                binding_index: 4,
                slot: 0,
            },
            ResourceAccess {
                binding_index: 5,
                slot: 1,
            },
        ];
        first.inactive_texture_bindings.push(5);
        let (_, inactive) = raster_content_intents([&first], &bindings, |_| Ok(true)).unwrap();
        assert_eq!(inactive[&0], RasterReadContents::DefinedRequired);
        assert_eq!(inactive[&1], RasterReadContents::Inactive);
        // A second packet's active alias of slot 1 must still tighten that slot.
        let mut second = packet();
        second.accesses.push(ResourceAccess {
            binding_index: 8,
            slot: 1,
        });
        let (_, inactive) =
            raster_content_intents([&first, &second], &bindings, |_| Ok(true)).unwrap();
        assert_eq!(inactive[&1], RasterReadContents::DefinedRequired);
    }

    #[test]
    fn selected_shader_view_reads_only_its_real_producer_and_keeps_other_layers_undefined() {
        use rhapsody::rg::{AccessTable, GraphDefinition, PlanContext};
        let selected =
            raster_binding_ranges(&[access(0x1d)], &[binding(2, 1, 0x1d)], &[0], 0, |_| Ok(image()))
                .unwrap()[0]
                .0;
        let definition = |read_range: AccessRange| {
            let mut graph = RhGraph::new();
            let texture = graph
                .import_unowned("partial array", image(), 1, 0)
                .unwrap();
            let mut writer = graph.add_pass("real selected producer");
            writer
                .add_resource_access_range(texture, 0x34, 0, selected)
                .unwrap();
            writer.render(|_| true);
            let mut reader = graph.add_pass("raster selected view");
            reader
                .add_resource_access_range(texture, 0x1d, 1, read_range)
                .unwrap();
            reader.side_effect();
            reader.render(|_| true);
            GraphDefinition::from_declaration(
                &graph.build().declaration,
                &mut AccessTable::original(),
                PlanContext::default(),
            )
        };
        assert!(definition(selected).is_ok());
        assert!(
            definition(AccessRange::Whole).is_err(),
            "owner-wide projection must retain its real uninitialized-content failure"
        );
        let wrong_layer = AccessRange::Texture {
            aspect_mask: 1,
            base_mip_level: 2,
            level_count: 1,
            base_array_layer: 2,
            layer_count: 1,
        };
        assert!(definition(wrong_layer).is_err());
    }

    #[test]
    fn bad_view_missing_access_and_invalid_slot_fail_without_widening() {
        assert!(
            raster_binding_ranges(&[access(0x1d)], &[binding(4, 0, 0x1d)], &[0], 0, |_| Ok(image()))
                .is_err()
        );
        assert!(
            raster_binding_ranges(&[access(0x1d)], &[binding(0, 3, 0x1d)], &[0], 0, |_| Ok(image()))
                .is_err()
        );
        assert!(raster_binding_ranges(&[], &[binding(0, 0, 0x1d)], &[0], 0, |_| Ok(image())).is_err());
        assert!(
            raster_binding_ranges(&[access(0x1d)], &[binding(0, 0, 0x1d)], &[0], 2, |_| Ok(image()))
                .is_err()
        );
        let buffer = ResourceSpec::Buffer(rhapsody::resource::BufferDesc::new(
            64,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            rhapsody::resource::MemoryLocation::GpuOnly,
        ))
        .descriptor();
        assert!(
            raster_binding_ranges(&[access(0x1d)], &[binding(0, 0, 0x1d)], &[0], 0, |_| Ok(buffer))
                .is_err()
        );
    }
}

#[cfg(test)]
mod raster_attachment_range_tests {
    use super::*;
    use rhapsody::rg::{AccessRange, AccessTable, GraphDefinition, PlanContext, ResourceSpec};

    fn color(tag: u32, bits: [u32; 4]) -> RasterPassColorAttachment {
        RasterPassColorAttachment {
            id: 0,
            format: 100,
            tag,
            sample_count: 1,
            view: bits,
        }
    }
    fn depth(layer: u32, clear: DepthClear) -> RasterPassDepth {
        RasterPassDepth {
            texture_id: 0,
            format: 126,
            sample_count: 1,
            clear,
            writable: true,
            layer,
        }
    }
    fn depth_desc() -> rhapsody::rg::Descriptor {
        ResourceSpec::Image(
            rhapsody::resource::TextureDesc::new_2d_array(64, 64, 3, vk::Format::D32_SFLOAT)
                .mip_levels(4)
                .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
        )
        .descriptor()
    }
    #[test]
    fn native_color_payload_is_clear_bits_and_only_depth_restricts_array_layer_count() {
        let bits = [0x3f80_0000, 0x8000_0000, 0x7fc0_0123, 0x3e80_0000];
        let c = color(1, bits);
        assert_eq!(c.clear_color_bits(), bits);
        let decoded = c.clear_color_bits().map(f32::from_bits);
        assert_eq!(decoded.map(f32::to_bits), bits);
        let plans = raster_attachment_accesses(&[c], Some(&depth(2, DepthClear::Load))).unwrap();
        assert_eq!(plans[0].access, 0x2e);
        assert!(!plans[0].load);
        assert!(plans[1].load);
        assert_eq!(plans[0].view.level_count, Some(1));
        assert_eq!(plans[0].view.layer_count, None);
        assert_eq!(plans[1].view.layer_count, Some(1));
        assert_eq!(plans[1].view.base_layer, 2);
        let desc = ResourceSpec::Image(
            rhapsody::resource::TextureDesc::new_2d_array(64, 64, 3, vk::Format::R32_SFLOAT)
                .mip_levels(4),
        )
        .descriptor();
        assert_eq!(
            compute_view_range(desc, &plans[0].view).unwrap(),
            AccessRange::Texture {
                aspect_mask: 1,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 3,
            }
        );
        assert_eq!(
            compute_view_range(depth_desc(), &plans[1].view).unwrap(),
            AccessRange::Texture {
                aspect_mask: 2,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 2,
                layer_count: 1,
            }
        );
        assert!(raster_attachment_accesses(&[color(3, bits)], None).is_err());
    }

    #[test]
    fn native_depth_layer_load_keeps_its_producer_and_does_not_initialize_other_mips_or_layers() {
        let plans = raster_attachment_accesses(&[], Some(&depth(1, DepthClear::Load))).unwrap();
        let selected = compute_view_range(depth_desc(), &plans[0].view).unwrap();
        let other = compute_view_range(depth_desc(), &plans[0].view.base_layer(2)).unwrap();
        let mut graph = RhGraph::new();
        let texture = graph
            .import_unowned("shadow cascades", depth_desc(), 1, 0)
            .unwrap();
        assert!(!raster_has_previous_contents(
            &graph,
            texture.raw(),
            selected
        ));
        let mut writer = graph.add_pass("cascade 1 depth clear and draw");
        writer
            .add_resource_access_range(texture, 0x2f, 1, selected)
            .unwrap();
        writer.render(|_| true);
        assert!(raster_has_previous_contents(
            &graph,
            texture.raw(),
            selected
        ));
        assert!(!raster_has_previous_contents(&graph, texture.raw(), other));
        let mut load = graph.add_pass("cascade 1 load");
        load.add_preserving_resource_access_range(texture, 0x2f, 1, selected)
            .unwrap();
        load.side_effect();
        load.render(|_| true);
        let definition = GraphDefinition::from_declaration(
            &graph.build().declaration,
            &mut AccessTable::original(),
            PlanContext::default(),
        )
        .unwrap();
        let plan = rhapsody::rg::compile_definition(definition).unwrap();
        assert_eq!(plan.retained_passes, [0, 1]);
        rhapsody::rg::check_plan(&plan.definition, &plan).unwrap();
        let mut wrong = RhGraph::new();
        let texture = wrong
            .import_unowned("undefined cascade 2", depth_desc(), 2, 0)
            .unwrap();
        let mut writer = wrong.add_pass("cascade 1 only");
        writer
            .add_resource_access_range(texture, 0x2f, 1, selected)
            .unwrap();
        writer.render(|_| true);
        let mut reader = wrong.add_pass("cascade 2 actual read");
        reader
            .add_resource_access_range(texture, 0x17, 1, other)
            .unwrap();
        reader.side_effect();
        reader.render(|_| true);
        assert!(
            GraphDefinition::from_declaration(
                &wrong.build().declaration,
                &mut AccessTable::original(),
                PlanContext::default()
            )
            .is_err()
        );
        assert!(compute_view_range(depth_desc(), &plans[0].view.base_layer(3)).is_err());
    }
}

// Append as a cfg(test) child module of raster_host.rs after strict host
// integration. The host owner agent applies it; this file is not compiled here.
#[cfg(test)]
mod constant_batch_gpu_tests {
    use super::*;
    use country_core::render::pass_builder::draw_stream::GpuOwned;

    fn shared() -> Option<Arc<RasterShared>> {
        let device = match rhapsody::device::Device::new_headless(
            &rhapsody::device::DeviceConfig::headless(),
        ) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "strict constants batch GPU initialization failed: {error}"
                );
                eprintln!("SKIP constants batch GPU initialization: {error}");
                return None;
            }
        };
        let buffers = Arc::new(BufferHost::new(device.clone()));
        let textures = Arc::new(TextureHost::new(device.clone()));
        let shaders = Arc::new(ShaderHost::new(device.clone(), std::env::temp_dir()).unwrap());
        Some(
            RasterShared::new_with_shader_library(
                device,
                buffers,
                textures,
                shaders,
                Arc::new(AssetShaderLibrary::new()),
            )
            .unwrap(),
        )
    }
    fn packet(shared: &Arc<RasterShared>, size: usize, marker: u8) -> DrawPacket {
        let mut task = shared.constant_pool.task().unwrap();
        let mut bytes = task.take(size).unwrap();
        bytes.fill(marker);
        let owner = task.own(bytes).ok().unwrap();
        let mut packet = DrawPacket::new(Handle::gap_new(), [0, 0], RasterState(0));
        packet.bindings.insert(
            0,
            GpuOwned::new(CoreBinding::UniformBuffer {
                size: size as u32,
                keepalive: ManagedHandle(0),
                lease: Some(ResourceLease::new(Arc::new(owner))),
            }),
        );
        packet
    }
    fn cpu(packet: &DrawPacket) -> Option<&CollectedConstants> {
        let CoreBinding::UniformBuffer { lease, .. } = &packet.bindings[&0].binding else {
            return None;
        };
        lease.as_ref()?.downcast_ref()
    }
    fn gpu(packet: &DrawPacket) -> Option<&DynamicConstantsChunk> {
        let CoreBinding::UniformBuffer { lease, .. } = &packet.bindings[&0].binding else {
            return None;
        };
        lease.as_ref()?.downcast_ref()
    }
    #[test]
    fn mid_batch_chunk_admission_error_preserves_original_cpu_owner_and_next_owner_recovers() {
        let Some(shared) = shared() else {
            return;
        };
        let invalid = rhapsody::resource::DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES as usize + 1;
        let mut packets = [packet(&shared, 16, 0xab), packet(&shared, invalid, 0xcd)];
        let pointer = cpu(&packets[1]).unwrap().bytes.as_ptr();
        let capacity = cpu(&packets[1]).unwrap().bytes.capacity();
        let mut host = shared.pass_host(Rc::new(RefCell::new(RhGraph::new())), None);
        host.commit_collected_draws(&mut packets);
        assert_eq!(
            gpu(&packets[0]).expect("first allocation published").size,
            16
        );
        let retained = cpu(&packets[1]).expect("rejected allocation retains CPU owner");
        assert_eq!(pointer, retained.bytes.as_ptr());
        assert_eq!(retained.bytes.accounted_bytes(), capacity as u64);
        assert!(retained.bytes.iter().all(|v| *v == 0xcd));
        assert!(host.constants_error.is_some());
        drop(packets);
        drop(host);
        let mut recovered = shared.pass_host(Rc::new(RefCell::new(RhGraph::new())), None);
        let mut packets = [packet(&shared, 16, 0xef)];
        recovered.commit_collected_draws(&mut packets);
        assert!(recovered.constants_error.is_none());
        assert_eq!(gpu(&packets[0]).expect("same allocator recovers").size, 16);
        drop(packets);
        drop(recovered);
        shared.device.wait_idle_and_collect().unwrap();
    }

    #[test]
    fn escaped_cpu_constant_owner_is_not_stolen_by_successful_commit() {
        let Some(shared) = shared() else {
            return;
        };
        let mut packets = [packet(&shared, 16, 0xa5)];
        let CoreBinding::UniformBuffer {
            lease: Some(escaped),
            ..
        } = &packets[0].bindings[&0].binding
        else {
            unreachable!()
        };
        let escaped = escaped.clone();
        let pointer = escaped
            .downcast_ref::<CollectedConstants>()
            .unwrap()
            .bytes
            .as_ptr();
        let mut host = shared.pass_host(Rc::new(RefCell::new(RhGraph::new())), None);
        host.commit_collected_draws(&mut packets);
        assert!(host.constants_error.is_none());
        assert_eq!(gpu(&packets[0]).unwrap().size, 16);
        let original = escaped.downcast_ref::<CollectedConstants>().unwrap();
        assert_eq!(pointer, original.bytes.as_ptr());
        assert!(original.bytes.iter().all(|v| *v == 0xa5));
        assert_eq!(
            original.bytes.accounted_bytes(),
            original.bytes.capacity() as u64
        );
        drop(escaped);
        drop(packets);
        drop(host);
        shared.device.wait_idle_and_collect().unwrap();
    }

    #[test]
    fn mid_batch_write_refusal_preserves_cpu_credit_and_next_owner_recovers() {
        let Some(shared) = shared() else {
            return;
        };
        let mut packets = [packet(&shared, 16, 0x51), packet(&shared, 16, 0x62)];
        let pointer = cpu(&packets[1]).unwrap().bytes.as_ptr();
        let credit = cpu(&packets[1]).unwrap().bytes.accounted_bytes();
        shared
            .fail_constant_write
            .store(2, std::sync::atomic::Ordering::Relaxed);
        let mut host = shared.pass_host(Rc::new(RefCell::new(RhGraph::new())), None);
        host.commit_collected_draws(&mut packets);
        assert_eq!(gpu(&packets[0]).expect("first write completed").size, 16);
        let retained = cpu(&packets[1]).expect("write refusal keeps original CPU bytes");
        assert_eq!(pointer, retained.bytes.as_ptr());
        assert_eq!(credit, retained.bytes.accounted_bytes());
        assert!(retained.bytes.iter().all(|v| *v == 0x62));
        assert!(matches!(
            host.constants_error,
            Some(rhapsody::Error::OutOfHostMemory)
        ));
        drop(packets);
        drop(host);
        let mut recovered = shared.pass_host(Rc::new(RefCell::new(RhGraph::new())), None);
        let mut packets = [packet(&shared, 16, 0x73)];
        recovered.commit_collected_draws(&mut packets);
        assert!(recovered.constants_error.is_none());
        assert_eq!(
            gpu(&packets[0])
                .expect("same allocator recovers after write refusal")
                .size,
            16
        );
        drop(packets);
        drop(recovered);
        shared.device.wait_idle_and_collect().unwrap();
    }

    #[test]
    fn cold_constant_plan_budget_error_rejects_before_native_pass_cache_and_recovers() {
        use country_core::render::pass_builder::{
            draw_commit_scratch::prepare_constant_plan, draw_stream::RasterPass,
        };
        let Some(shared) = shared() else { return; };
        let graph = Rc::new(RefCell::new(RhGraph::new()));
        let mut owner = shared.pass_host(graph.clone(), None);
        assert!(owner.cache.is_empty());
        assert_eq!(shared.shaders.passes().len(), 0, "native pass cache must start cold");
        let submitted = shared.device.retirement_stats().submitted;
        let mut scratch = owner.checkout_draw_scratch().unwrap();
        let stats = shared.device.allocator_stats();
        let usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(usage).unwrap();
        let error = prepare_constant_plan(&mut scratch, 4096).unwrap_err();
        assert!(matches!(error, rhapsody::Error::OutOfHostMemory), "{error:?}");
        assert_eq!(stats.cpu_memory_usage().total(), usage, "failed plan retains no extra CPU storage credit");
        owner.record_collect_error(error);
        // A cold cache would otherwise enter native render-pass creation before
        // submit_raster_pass can forward the already-known declaration error.
        RasterPass::collector(Box::new(owner), [16, 16]).render();
        assert_eq!(shared.shaders.passes().len(), 0, "rejected declaration creates no native pass");
        assert_eq!(shared.device.retirement_stats().submitted, submitted);
        let (declaration, callbacks, retire, declaration_error) = graph.replace(RhGraph::new()).build().into_parts();
        assert!(declaration.passes.is_empty());
        assert!(callbacks.is_empty());
        assert!(retire.is_empty());
        assert!(matches!(declaration_error, Some(rhapsody::Error::OutOfHostMemory)), "{declaration_error:?}");
        stats.set_host_budget(0).unwrap();
        drop(scratch);
        let mut recovered = shared.pass_host(graph, None);
        let mut packets = [packet(&shared, 16, 0x84)];
        recovered.commit_collected_draws(&mut packets);
        assert!(recovered.constants_error.is_none());
        assert_eq!(gpu(&packets[0]).expect("same shared allocator recovers").size, 16);
        drop(packets);
        drop(recovered);
        shared.device.wait_idle_and_collect().unwrap();
    }
}
