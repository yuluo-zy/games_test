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
    std::hash::BuildHasherDefault<rhapsody::rg::token_map::TokenHasher>,
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
use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc, sync::Arc};

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
    /// Shared, so a draw takes one Arc instead of cloning the binding vector.
    vaos: IdMap<Arc<VaoSnapshot>>,
}
#[derive(Clone)]
struct VaoSnapshot {
    source: Arc<country_core::asset_libraries::vao_library::VaoEntry>,
    index: Arc<rhapsody::resource::Buffer>,
    index_count: u32,
    bindings: Vec<(Arc<str>, Arc<rhapsody::resource::Buffer>)>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct VaoScopeState {
    /// Address of the `DrawLists` the snapshot was taken for; only compared while
    /// the scope is open, so a recycled address cannot match across frames.
    lists: usize,
    library_revision: u64,
    table_revision: u64,
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
    programs: RasterPrograms,
    reflections: Mutex<crate::draw_template_cache::ReflectionCache>,
    constant_pool: Arc<crate::draw_constant_pool::ConstantPool>,
    vao_refresh_timing: Mutex<VaoRefreshTiming>,
    /// Bumped every time the VAO snapshot table is replaced.
    vao_table_revision: std::sync::atomic::AtomicU64,
    vao_snapshot_allocations: std::sync::atomic::AtomicU64,
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

/// The two handle spaces are deliberately exclusive. A game handle must never
/// fall back to a standalone table entry with an accidentally equal integer.
enum RasterPrograms {
    Standalone(Mutex<Vec<Arc<RasterShaderProgram>>>),
    Core(Arc<AssetShaderLibrary>),
}

impl RasterPrograms {
    fn with_program<R>(&self, id: u64, f: impl FnOnce(&RasterShaderProgram) -> R) -> Option<R> {
        match self {
            Self::Core(library) => library.get_raster_by_id(id).map(f),
            Self::Standalone(table) => {
                let program = table.lock().get((id as usize).checked_sub(1)?).cloned()?;
                Some(f(&program))
            }
        }
    }

    fn register(
        &self,
        program: Arc<RasterShaderProgram>,
    ) -> rhapsody::Result<Handle<RasterShaderProgram>> {
        let Self::Standalone(table) = self else {
            return Err(rhapsody::Error::invalid_usage(
                "raster programs are owned by the core shader library",
            ));
        };
        let mut table = table.lock();
        table
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        table.push(program);
        Ok(Handle(ManagedHandle(table.len() as u64), PhantomData))
    }
}

impl RasterShared {
    pub(crate) fn set_frame_buffers(&self, buffers: Option<Arc<BufferHost>>) {
        *self.frame_buffers.lock() = buffers;
    }

    fn buffer_host(&self) -> Arc<BufferHost> {
        self.frame_buffers.lock().as_ref().unwrap_or(&self.buffers).clone()
    }
    fn reflection(&self, shader: u64, variant: u8) -> Option<Arc<CoreReflection>> {
        let key = (shader, variant);
        if let Some(hit) = self.reflections.lock().get(&key) { return Some(hit); }
        let reflection = self.programs.with_program(shader, |program| core_reflection(program, variant))??;
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
        use country_core::asset_libraries::{Handle as AssetHandle, vao_library::AssetVAOLibrary};
        use country_core::render::vao::Vao;
        let mut timing = crate::game_loop::frame_timing_enabled().then(|| VaoRefreshMeasurement {
            totals: &self.vao_refresh_timing,
            started: std::time::Instant::now(),
            counts: VaoRefreshTiming::default(),
        });
        let wanted = (0
            ..country_core::systems::render_loop::draw_passes::draw_lists::DRAW_LIST_COUNT)
            .flat_map(|index| lists.list(index))
            .map(|entry| {
                if let Some(timing) = timing.as_mut() {
                    timing.counts.list_entries += 1;
                }
                entry.0.vao
            })
            .collect::<std::collections::BTreeSet<_>>();
        if let Some(timing) = timing.as_mut() {
            timing.counts.unique_vaos = wanted.len() as u64;
        }
        let mut query = world.query::<&AssetHandle<Vao>>();
        let handles = query
            .iter(world)
            .inspect(|_| {
                if let Some(timing) = timing.as_mut() {
                    timing.counts.queried_entities += 1;
                }
            })
            .filter(|handle| wanted.contains(&handle.id().raw()))
            .map(|handle| (handle.id().raw(), *handle))
            .collect::<HashMap<_, _>>();
        let handles = wanted
            .into_iter()
            .map(|id| {
                handles.get(&id).copied().ok_or_else(|| {
                    rhapsody::Error::invalid_usage("draw-list VAO has no matching typed ECS handle")
                })
            })
            .collect::<rhapsody::Result<Vec<_>>>()?;
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
        let mut snapshots = IdMap::default();
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
            let cached = self.assets.lock().vaos.get(&id).filter(|snapshot|
                Arc::ptr_eq(&snapshot.source, entry) && Arc::ptr_eq(&snapshot.index, &index)).cloned();
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
        self.assets.lock().vaos = snapshots;
        self.vao_table_revision
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(())
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

    pub fn new(
        device: Arc<rhapsody::device::Device>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
        shaders: Arc<ShaderHost>,
    ) -> Result<Arc<Self>, rhapsody::Error> {
        Self::new_with_programs(
            device,
            buffers,
            textures,
            shaders,
            RasterPrograms::Standalone(Mutex::new(Vec::new())),
        )
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
        Self::new_with_programs(
            device,
            buffers,
            textures,
            shaders,
            RasterPrograms::Core(library),
        )
    }

    fn new_with_programs(
        device: Arc<rhapsody::device::Device>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
        shaders: Arc<ShaderHost>,
        programs: RasterPrograms,
    ) -> Result<Arc<Self>, rhapsody::Error> {
        Ok(Arc::new(Self {
            attachments: Mutex::new(AttachmentCache::new(device.clone())?),
            dynamic: Arc::new(Mutex::new(DynamicConstants::new(device.clone())?)),
            device,
            buffers,
            frame_buffers: Mutex::new(None),
            textures,
            shaders,
            programs,
            reflections: Mutex::new(Default::default()),
            constant_pool: Arc::new(Default::default()),
            vao_refresh_timing: Mutex::new(VaoRefreshTiming::default()),
            vao_table_revision: std::sync::atomic::AtomicU64::new(0),
            vao_snapshot_allocations: std::sync::atomic::AtomicU64::new(0),
            table_reads: Mutex::new(None),
            frame_constants: Mutex::new(None),
            vao_scope: Mutex::new(None),
            assets: Mutex::new(AssetTables::default()),
        }))
    }

    /// Standalone compatibility registration (ids start at 1).
    /// Game integration uses `new_with_shader_library` and the library's ids.
    pub fn register_program(
        &self,
        program: Arc<RasterShaderProgram>,
    ) -> Handle<RasterShaderProgram> {
        self.try_register_program(program)
            .expect("standalone raster program registration failed")
    }

    pub fn try_register_program(
        &self,
        program: Arc<RasterShaderProgram>,
    ) -> rhapsody::Result<Handle<RasterShaderProgram>> {
        self.programs.register(program)
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
    constants_error: Option<rhapsody::Error>,
    resource_graph_factory: Option<Rc<dyn Fn() -> country_core::rhapsody_types::RenderGraph>>,
}

impl RasterPassHost {
    /// Prepare the graph-free declaration capability, also used by equivalence
    /// diagnostics without changing the process-wide scheduling policy.
    pub fn prepare_draw_collection(&self) -> Arc<dyn Fn() -> Box<dyn PassHost> + Send + Sync> {
        let shared = self.shared.clone();
        Arc::new(move || Box::new(RasterCollectHost {
            shared: shared.clone(), deferred: RefCell::new(Vec::new()),
        }))
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
    vertices: Vec<(u32, ResourceHandle, u64)>,
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
    deferred: RefCell<Vec<country_core::render::pass_builder::draw_stream::DeferredBuffer>>,
}

use crate::draw_constant_pool::CollectedConstants;

impl PassHost for RasterCollectHost {
    fn raster_shader(
        &self,
        shader: &Handle<RasterShaderProgram>,
        variant: u8,
    ) -> Option<Arc<CoreReflection>> {
        self.shared.reflection(shader.0.0, variant)
    }

    fn ssbo_instance_count(&self, rg_buffer_hash: u64) -> Option<u32> {
        let assets = self.shared.assets.lock();
        if let Some(leases) = &assets.ssbo_leases {
            return leases
                .get(&rg_buffer_hash)?
                .instance_count()
                .map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&rg_buffer_hash).copied()
    }

    fn ssbo_instance_count_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        let assets = self.shared.assets.lock();
        if let Some(leases) = &assets.ssbo_leases {
            let lease = leases.get(&hash)?;
            assert_eq!(lease.canonical_type_name, expected_type);
            return lease.instance_count().map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&hash).copied()
    }

    fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)> {
        self.shared.assets.lock().placement.get(&id).cloned()
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
            .textures_by_name
            .get(&name_hash)
            .cloned()
            .unwrap_or_else(|| panic!("No such texture: name hash 0x{name_hash:016x}"))
    }

    fn rg_texture(&self, preset: u8) -> AttachmentTexture {
        self.shared
            .assets
            .lock()
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
    fn allocate_dynamic_constants(&mut self, reflection: &CoreReflection) -> Option<DynamicConstantsBinding> {
        Some(DynamicConstantsBinding {
            data: self.shared.constant_pool.take(reflection.uniform_block_size as usize),
            binding_index: reflection.uniform_block_binding,
            keepalive: ManagedHandle(0),
            lease: None,
        })
    }
    fn finish_dynamic_constants(&mut self, constants: DynamicConstantsBinding, packet: &mut DrawPacket) {
        // A material can overwrite the uniform binding. Preserve that operation.
        if let Some(binding) = packet.bindings.get_mut(&constants.binding_index) {
            if let CoreBinding::UniformBuffer { keepalive, lease, .. } = &mut binding.binding {
                if keepalive.0 == 0 && lease.is_none() {
                    *lease = Some(ResourceLease::new(Arc::new(self.shared.constant_pool.own(constants.data))));
                    return;
                }
            }
        }
        self.shared.constant_pool.recycle(constants.data);
    }
    fn discard_dynamic_constants(&mut self, constants: DynamicConstantsBinding) {
        self.shared.constant_pool.recycle(constants.data);
    }
    fn rg_buffer_resource(&self, hash: u64) -> Option<u32> {
        let assets = self.shared.assets.lock();
        let id = assets.rg_buffers.get(&hash).copied()?;
        assert!(id < country_core::render::pass_builder::draw_stream::DEFERRED_BUFFER_BASE);
        Some(id)
    }
    fn rg_buffer_resource_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        use country_core::render::pass_builder::draw_stream::{DeferredBuffer, DEFERRED_BUFFER_BASE};
        {
            let assets = self.shared.assets.lock();
            let Some(leases) = &assets.ssbo_leases else {
                let id = assets.rg_buffers.get(&hash).copied()?;
                assert!(id < DEFERRED_BUFFER_BASE);
                return Some(id);
            };
            assert_eq!(leases.get(&hash)?.canonical_type_name, expected_type);
        }
        let mut deferred = self.deferred.borrow_mut();
        if let Some(buffer) = deferred.iter().find(|buffer| buffer.hash == hash) {
            assert_eq!(buffer.type_name, expected_type);
            return Some(buffer.token);
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
    fn collect_factory(&self) -> Option<Arc<dyn Fn() -> Box<dyn PassHost> + Send + Sync>> {
        // Opt in until the hardware p95 acceptance gate has been measured.
        match std::env::var("TG_DRAW_COLLECT").as_deref() {
            Ok("serial" | "parallel") => {}
            _ => return None,
        }
        Some(self.prepare_draw_collection())
    }

    fn try_parallel_collect(&self) -> Option<Box<dyn std::any::Any>> {
        // Never block an ECS/task-pool worker waiting for work in its own pool.
        if std::thread::current().name() != Some("tg-render")
            || std::env::var("TG_DRAW_COLLECT").as_deref() != Ok("parallel") { return None; }
        // While the conservative upload boundary exists, Update workers can
        // wait on this frame. Never enqueue Collect behind those workers.
        crate::render_upload_boundary::ECS_COLLECT_GATE.try_lock()
            .map(|lease| Box::new(lease) as Box<dyn std::any::Any>)
    }

    fn collect_chunk_draws(&self) -> usize {
        std::env::var("TG_DRAW_COLLECT_CHUNK").ok()
            .and_then(|value| value.parse::<usize>().ok()).unwrap_or(64).clamp(1, 4096)
    }

    fn commit_collected_draw(&mut self, packet: &mut DrawPacket) {
        for binding in packet.bindings.values_mut() {
            let CoreBinding::UniformBuffer { keepalive, lease, .. } = &mut binding.binding else { continue };
            let Some(bytes) = lease.as_ref().and_then(ResourceLease::downcast_ref::<CollectedConstants>) else { continue };
            let result = self.shared.dynamic.lock().allocate(bytes.bytes.len() as u64)
                .and_then(|chunk| {
                    chunk.write_bytes(&bytes.bytes)?;
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
        self.shared.reflection(shader.0.0, variant)
    }

    /// 在常量页里为该管线的 uniform 块分配一块；字节在 [`PassHost::flush_dynamic_constants`] 写入。
    fn allocate_dynamic_constants(
        &mut self,
        reflection: &CoreReflection,
    ) -> Option<DynamicConstantsBinding> {
        let size = u64::from(reflection.uniform_block_size);
        let chunk = match self.shared.dynamic.lock().allocate(size) {
            Ok(chunk) => Arc::new(chunk),
            Err(error) => {
                self.constants_error.get_or_insert(error);
                return None;
            }
        };
        let mut data = Vec::new();
        if data.try_reserve_exact(size as usize).is_err() {
            self.constants_error
                .get_or_insert(rhapsody::Error::OutOfHostMemory);
            return None;
        }
        data.resize(size as usize, 0);
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
            .rg_buffers
            .get(&rg_buffer_hash)
            .copied()
    }

    fn rg_buffer_resource_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        let lease = {
            let assets = self.shared.assets.lock();
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
        let assets = self.shared.assets.lock();
        if let Some(leases) = &assets.ssbo_leases {
            return leases
                .get(&rg_buffer_hash)?
                .instance_count()
                .map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&rg_buffer_hash).copied()
    }
    fn ssbo_instance_count_typed(&self, hash: u64, expected_type: &'static str) -> Option<u32> {
        let assets = self.shared.assets.lock();
        if let Some(leases) = &assets.ssbo_leases {
            let lease = leases.get(&hash)?;
            assert_eq!(lease.canonical_type_name, expected_type);
            return lease.instance_count().map(|n| n as u32);
        }
        assets.ssbo_instance_counts.get(&hash).copied()
    }

    fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)> {
        self.shared.assets.lock().placement.get(&id).cloned()
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
            .textures_by_name
            .get(&name_hash)
            .cloned()
            .unwrap_or_else(|| panic!("No such texture: name hash 0x{name_hash:016x}"))
    }

    fn rg_texture(&self, preset: u8) -> AttachmentTexture {
        self.shared
            .assets
            .lock()
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
                    .with_program(packet.shader.0.0, |program| {
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
                Ok(PreparedDraw {
                    packet,
                    entry,
                    index: None,
                    index_count: None,
                    vertices: Vec::new(),
                    frame_constants: None,
                    external: rhapsody::resource::TempList::new(
                        country_core::render::pass_builder::draw_builder::MAX_PASS_BINDINGS,
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
                let mut declared = graph.add_pass(name);
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error));
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
                        .get(&draw.packet.head[1])
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
                                let zeros = vec![0u8; usize::try_from(bytes).map_err(|_| {
                                    rhapsody::Error::invalid_usage("placeholder vertex size overflow")
                                })?];
                                let placeholder = shared.device.create_buffer(
                                    &rhapsody::resource::BufferDesc::new(
                                        bytes,
                                        vk::BufferUsageFlags::VERTEX_BUFFER,
                                        rhapsody::resource::MemoryLocation::GpuOnly,
                                    ),
                                    "Vertex_Color placeholder",
                                    Some(zeros.as_slice()),
                                )?;
                                (placeholder.shared(), bytes)
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
            let mut declared = graph.add_pass(name);
            declared.side_effect();
            declared.render_with_result_context(move |_| Err(error));
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
                let mut declared = graph.add_pass(name);
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error));
                return;
            }
        };
        // A native shader binding carries its view independently from the
        // owner's graph identity. Project the very same view before graph build;
        // a descriptor selecting one layer must not read every layer/mip.
        let attachment_accesses = match raster_attachment_accesses(&colors, depth.as_ref()) {
            Ok(attachments) => attachments,
            Err(error) => {
                let mut declared = graph.add_pass(name);
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error));
                return;
            }
        };
        let mut access_ranges = match raster_binding_ranges(
            &accesses,
            &bindings,
            colors.len() + usize::from(depth.is_some()),
            |resource| {
                let handle = graph.resource_handle(resource)?;
                Ok(graph.declaration().resources[handle.raw() as usize].descriptor)
            },
        ) {
            Ok(ranges) => ranges,
            Err(error) => {
                let mut declared = graph.add_pass(name);
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error));
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
                    let mut declared = graph.add_pass(name);
                    declared.side_effect();
                    declared.render_with_result_context(move |_| Err(error));
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
                let resource = declarer.import_texture(&mut graph, view.texture.clone(), 0x1d)?;
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
                let mut declared = graph.add_pass(name);
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error));
                return;
            }
        };
        country_core::render::declare_probe::stage(3, stage_started);
        let mut declared = graph.add_pass(name);
        let table_started = country_core::render::declare_probe::start();
        for (resource, range) in table_reads.iter() {
            if let Err(error) = declared.add_resource_access_range(*resource, 0x1d, 0, *range) {
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error.into()));
                return;
            }
        }
        country_core::render::declare_probe::table(table_started, table_reads.len());
        let draw_access_started = country_core::render::declare_probe::start();
        for draw in &prepared {
            // Native DrawBuilder also records these as access 2/group 1.
            // Register them here as a final owner/access invariant, including
            // packets constructed through the public raw draw() boundary.
            let indirect = match &draw.packet.kind {
                DrawKind::MultiDrawIndirect { buffer, .. }
                | DrawKind::IndexedIndirect { buffer, .. } => vec![*buffer],
                DrawKind::IndexedIndirectCount {
                    buffer,
                    count_buffer,
                    ..
                } => vec![*buffer, *count_buffer],
                _ => Vec::new(),
            };
            for id in indirect {
                let resource = declared
                    .resource_handle(id)
                    .expect("validated indirect graph resource");
                declared
                    .add_resource_access(resource, 2, 1)
                    .expect("validated indirect access");
            }
            if let Some((resource, range)) = draw.frame_constants {
                if let Err(error) = declared.add_resource_access_range(resource, 0x1f, 0, range) {
                    declared.side_effect();
                    declared.render_with_result_context(move |_| Err(error.into()));
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
                    declared.side_effect();
                    declared.render_with_result_context(move |_| Err(error.into()));
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
            for (draw, pipeline) in prepared.iter().zip(&pipelines) {
                country_core::render::declare_probe::rec_draw();
                let probe = country_core::render::declare_probe::start();
                // SAFETY: 管线属于本设备，命令缓冲在录制中。
                unsafe {
                    device.raw_device().cmd_bind_pipeline(
                        command,
                        vk::PipelineBindPoint::GRAPHICS,
                        pipeline.raw_handle(),
                    )
                };
                country_core::render::declare_probe::rec_pipeline(probe);
                let parameters = match bindless_parameters(
                    &shared,
                    &ctx,
                    draw,
                    &bindings,
                    managed,
                    &mut page_cursor,
                    frozen_globals.clone(),
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
                if let Err(error) = record_draw(&mut api, draw, managed, count_form) {
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

fn record_draw<B: rhapsody::rg::CommandBackend>(
    api: &mut RhRenderPassApi<'_, B>,
    draw: &PreparedDraw,
    managed: &ManagedPassResources,
    count_form: bool,
) -> rhapsody::Result<()> {
    // O 0x1416988f8: reflected location selects the binding point. Pipeline
    // formats/strides were produced from that very same compiled reflection.
    for (binding, resource, _) in &draw.vertices {
        let buffer = managed
            .buffer_for(resource.raw())
            .map_err(rhapsody::Error::from)?;
        api.bind_vertex_buffer_by_binding_point(*binding, buffer.raw());
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
            let buffer = managed
                .buffer_for(index.raw())
                .map_err(rhapsody::Error::from)?;
            api.bind_index_buffer(buffer.raw(), vk::IndexType::UINT32);
            api.draw_indexed(*index_count, 1, 0, 0, 0);
        }
        DrawKind::InstancedGlDrawFlag { flag } => {
            let index = draw.index.ok_or_else(|| {
                rhapsody::Error::invalid_usage("mesh index access was not declared")
            })?;
            let buffer = managed
                .buffer_for(index.raw())
                .map_err(rhapsody::Error::from)?;
            api.bind_index_buffer(buffer.raw(), vk::IndexType::UINT32);
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
            let buffer = managed
                .buffer_for(index.raw())
                .map_err(rhapsody::Error::from)?;
            api.bind_index_buffer(buffer.raw(), vk::IndexType::UINT32);
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
            let index = managed.buffer_for(
                draw.index
                    .expect("prepared actual indirect index owner")
                    .raw(),
            )?;
            api.bind_index_buffer(index.raw(), vk::IndexType::UINT32);
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
            let index = managed.buffer_for(
                draw.index
                    .expect("prepared actual indirect index owner")
                    .raw(),
            )?;
            api.bind_index_buffer(index.raw(), vk::IndexType::UINT32);
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
    let resource = declarer.import_buffer(graph, buffer, 0x1f)?;
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
                let (access, layout) = match kind {
                    BindingKind::SampledImage => (0x1d, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL),
                    BindingKind::StorageImage => (0x37, vk::ImageLayout::GENERAL),
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
                    resource: declarer.import_texture(graph, view.texture.clone(), access)?,
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

fn bindless_parameters(
    shared: &RasterShared,
    ctx: &rhapsody::rg::PassContext<'_>,
    draw: &PreparedDraw,
    pass_bindings: &[RenderPassBinding],
    managed: &ManagedPassResources,
    cursor: &mut rhapsody::bindless::BindlessPageCursor,
    globals: Option<Arc<crate::frame_setup::FrameGlobals>>,
) -> rhapsody::Result<rhapsody::bindless::PreparedBindlessParameters> {
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
        upsert_shader_resource(&mut resources, binding.binding, value)?;
    }
    for access in &draw.packet.accesses {
        let binding = pass_bindings
            .get(usize::from(access.slot))
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
        &shared.dynamic,
    );
    country_core::render::declare_probe::rec_prepare(probe_prepare);
    prepared
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.component_journaled::<country_core::asset_libraries::Handle<country_core::render::vao::Vao>>();
    m.resource_incremental::<country_core::asset_libraries::vao_library::AssetVAOLibrary>(
        country_core::asset_libraries::vao_library::AssetVAOLibrary::snapshot_revision,
        country_core::asset_libraries::vao_library::AssetVAOLibrary::update_render_snapshot);
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
        let programs = RasterPrograms::Core(library.clone());
        let _padding = country_core::asset_libraries::Handle::<()>::new();
        let handle = library.add_raster(program("game"), false);
        assert!(handle.id().raw() > 1);
        assert!(
            programs
                .with_program(handle.id().raw(), |resolved| {
                    std::ptr::eq(resolved, library.get_raster(handle).unwrap())
                })
                .unwrap()
        );
        assert!(programs.with_program(0, |_| ()).is_none());
        assert!(programs.with_program(u64::MAX, |_| ()).is_none());
    }

    #[test]
    fn core_source_cannot_alias_or_fall_back_to_standalone_handles() {
        let standalone = RasterPrograms::Standalone(Mutex::new(Vec::new()));
        let standalone_handle = standalone
            .register(Arc::new(program("standalone")))
            .unwrap();
        assert_eq!(standalone_handle.0.0, 1);
        let library = Arc::new(AssetShaderLibrary::new());
        let core = RasterPrograms::Core(library.clone());
        assert!(core.with_program(standalone_handle.0.0, |_| ()).is_none());
        assert!(matches!(
            core.register(Arc::new(program("wrong-source"))),
            Err(rhapsody::Error::InvalidUsage { .. })
        ));
        assert!(library.raster_shaders.is_empty());
        assert_eq!(
            standalone.with_program(standalone_handle.0.0, |p| p.name),
            Some("standalone")
        );
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

/// Pair each native pass binding with the access appended by the same
/// DrawBuilder operation. Attachment and indirect accesses need no shader slot;
/// they retain their own declaration. Repeated views of one image are consumed
/// once in original order, instead of replacing them with an owner-wide access.
fn raster_binding_ranges(
    accesses: &[country_core::render::pass_builder::draw_stream::PassAccess],
    bindings: &[RenderPassBinding],
    attachment_accesses: usize,
    mut descriptor: impl FnMut(u32) -> rhapsody::Result<rhapsody::rg::Descriptor>,
) -> rhapsody::Result<Vec<(rhapsody::rg::AccessRange, Option<u8>)>> {
    use rhapsody::rg::{AccessRange, Descriptor};
    if attachment_accesses > accesses.len() {
        return Err(rhapsody::Error::invalid_usage(
            "raster attachment accesses are absent",
        ));
    }
    let mut used = vec![false; bindings.len()];
    let mut ranges = Vec::new();
    ranges
        .try_reserve_exact(accesses.len())
        .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    for (access_index, access) in accesses.iter().enumerate() {
        // RasterPass::new writes attachments before any DrawBuilder bindings.
        // A shader alias must never steal that independent attachment access.
        if access_index < attachment_accesses {
            ranges.push((AccessRange::Whole, None));
            continue;
        }
        let matching = bindings.iter().enumerate().find(|(slot, binding)| {
            !used[*slot]
                && binding.resource_index == access.resource
                && binding.access_type == access.access
        });
        let (range, slot) = if let Some((slot, binding)) = matching {
            let slot_id = u8::try_from(slot).map_err(|_| {
                rhapsody::Error::invalid_usage("raster pass binding index exceeds u8")
            })?;
            used[slot] = true;
            let desc = descriptor(binding.resource_index)?;
            let range = match desc {
                Descriptor::Image(_) => {
                    compute_view_range(desc, &binding.view.unwrap_or_default())?
                }
                Descriptor::Buffer(_) if binding.view.is_none() => AccessRange::Whole,
                Descriptor::Buffer(_) => {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster texture view refers to a buffer",
                    ));
                }
            };
            (range, Some(slot_id))
        } else {
            (AccessRange::Whole, None)
        };
        ranges.push((range, slot));
    }
    if used.iter().any(|used| !used) {
        return Err(rhapsody::Error::invalid_usage(
            "raster pass binding has no matching native resource access",
        ));
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
) -> rhapsody::Result<(Vec<u32>, std::collections::BTreeMap<u8, RasterReadContents>)> {
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

    fn packet(intent: RasterReadContents, binding: u32, slot: u8) -> DrawPacket {
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
    fn repeated_owner_views_keep_ranges_slots_and_attachment_order() {
        let bindings = [binding(2, 1, 0x1d), binding(3, 2, 0x1d)];
        // The first access has an identical owner/code but belongs to the
        // framebuffer, before the two native shader binding operations.
        let ranges = raster_binding_ranges(
            &[access(0x1d), access(0x1d), access(0x1d)],
            &bindings,
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
            raster_binding_ranges(&[access(0x1d)], &[binding(2, 1, 0x1d)], 0, |_| Ok(image()))
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
            raster_binding_ranges(&[access(0x1d)], &[binding(4, 0, 0x1d)], 0, |_| Ok(image()))
                .is_err()
        );
        assert!(
            raster_binding_ranges(&[access(0x1d)], &[binding(0, 3, 0x1d)], 0, |_| Ok(image()))
                .is_err()
        );
        assert!(raster_binding_ranges(&[], &[binding(0, 0, 0x1d)], 0, |_| Ok(image())).is_err());
        assert!(
            raster_binding_ranges(&[access(0x1d)], &[binding(0, 0, 0x1d)], 2, |_| Ok(image()))
                .is_err()
        );
        let buffer = ResourceSpec::Buffer(rhapsody::resource::BufferDesc::new(
            64,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            rhapsody::resource::MemoryLocation::GpuOnly,
        ))
        .descriptor();
        assert!(
            raster_binding_ranges(&[access(0x1d)], &[binding(0, 0, 0x1d)], 0, |_| Ok(buffer))
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
