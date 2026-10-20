//! One game-side frame builder for core compute/raster consumers and rhapsody.
//!
//! This is an integration API, not the missing full-game `FramePasses` adapter.
//! Every phase uses the same graph/session, device and asset-library identities.
//! Failed or unwound builders cancel declarations without submitting GPU work.
use std::{
    cell::{Cell, RefCell},
    path::Path,
    rc::Rc,
    sync::Arc,
};

use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary, render::ssbo::rhapsody_boundary::Buffer,
    rhapsody_types::Texture,
};
use rhapsody::rg::{
    graph::RenderGraph,
    resource::{GraphId, ResourceHandle},
    vulkan::{
        AccessTable, ResourceSpec, SessionFrameReceipt, VulkanGraphBackend, VulkanGraphSession,
    },
};

use crate::{
    compute_frame::CoreComputeFrame,
    raster_host::{GraphDeclarer, RasterPassHost, RasterShared},
    render_host::{BufferHost, DeviceHost, GraphPasses, HostTables, ShaderHost, TextureHost},
};

/// Owns the production session and the host tables used by both core pass families.
/// Shader programs must be built through `shader_backend()` and stored in the
/// exact library passed here. Buffer/texture handles must originate from this bridge;
/// legacy slot values do not encode an owner namespace. Stale generations are rejected.
pub struct CoreRenderBridge {
    session: Rc<RefCell<VulkanGraphSession>>,
    tables: HostTables,
    raster: Arc<RasterShared>,
    shaders: Arc<AssetShaderLibrary>,
}

/// This App's exact physical handle namespaces and their retained owners.
/// The worker creates its Rc frame driver locally from the same tables.
#[derive(Clone)]
pub struct CoreRenderOwners {
    tables: HostTables,
    raster: Arc<RasterShared>,
    shaders: Arc<AssetShaderLibrary>,
}

/// Sent back with the actual gameplay SubApp after a fully completed original
/// world render. The current session's named GPU owners preserve its first
/// frame history; Mutex permits the single main-thread adoption through &World.
#[derive(bevy_ecs::resource::Resource)]
pub struct CompletedGameplayWarmup {
    owners: CoreRenderOwners,
    named: parking_lot::Mutex<Option<rhapsody::rg::vulkan::CompletedNamedResources>>,
}

impl CoreRenderBridge {
    pub fn new(
        device: Arc<rhapsody::device::Device>,
        compiled_shader_root: impl AsRef<Path>,
        shaders: Arc<AssetShaderLibrary>,
    ) -> rhapsody::Result<Self> {
        let tables = HostTables {
            buffers: Arc::new(BufferHost::new(device.clone())),
            textures: Arc::new(TextureHost::new(device.clone())),
            shaders: Arc::new(ShaderHost::new(
                device.clone(),
                compiled_shader_root.as_ref(),
            )?),
        };
        let raster = RasterShared::new_with_shader_library(
            device.clone(),
            tables.buffers.clone(),
            tables.textures.clone(),
            tables.shaders.clone(),
            shaders.clone(),
        )?;
        let session =
            VulkanGraphSession::new(VulkanGraphBackend::new(device, AccessTable::original())?);
        Ok(Self {
            session: Rc::new(RefCell::new(session)),
            tables,
            raster,
            shaders,
        })
    }

    /// Captured before moving the original gameplay SubApp to its worker.
    pub fn warmup_owners(&self) -> rhapsody::Result<CoreRenderOwners> {
        if self.session.borrow().named_resource_count() != 0 {
            return Err(rhapsody::Error::invalid_usage("gameplay warmup cannot fork an existing temporal owner"));
        }
        Ok(CoreRenderOwners { tables: self.tables.clone(), raster: self.raster.clone(),
            shaders: self.shaders.clone() })
    }

    pub fn from_owners(owners: CoreRenderOwners) -> rhapsody::Result<Self> {
        let device = owners.tables.buffers.graphics_device().clone();
        // Current VulkanGraphBackend implementation, one thread-local session;
        // no new BufferHost/TextureHost/ShaderHost, descriptor heap or library.
        let session = VulkanGraphSession::new(VulkanGraphBackend::new(device, AccessTable::original())?);
        Ok(Self { session: Rc::new(RefCell::new(session)), tables: owners.tables,
            raster: owners.raster, shaders: owners.shaders })
    }

    /// Call only after the actual full compile_main builder was compiled and
    /// executed. This validates actual GPU completion, not a synthetic ready bit.
    pub fn complete_gameplay_warmup(&mut self, timeout_ns: u64)
        -> rhapsody::Result<CompletedGameplayWarmup> {
        if Rc::strong_count(&self.session) != 1 {
            return Err(rhapsody::Error::invalid_usage("warmup has live graph/declarer leases"));
        }
        let named = self.session.borrow_mut().take_completed_named_resources(timeout_ns)?;
        Ok(CompletedGameplayWarmup {
            owners: CoreRenderOwners { tables: self.tables.clone(), raster: self.raster.clone(),
                shaders: self.shaders.clone() }, named: parking_lot::Mutex::new(Some(named)),
        })
    }

    pub fn accept_gameplay_warmup(&mut self, completed: &CompletedGameplayWarmup)
        -> rhapsody::Result<()> {
        if !Arc::ptr_eq(&self.tables.buffers, &completed.owners.tables.buffers)
            || !Arc::ptr_eq(&self.tables.textures, &completed.owners.tables.textures)
            || !Arc::ptr_eq(&self.tables.shaders, &completed.owners.tables.shaders)
            || !Arc::ptr_eq(&self.raster, &completed.owners.raster)
            || !Arc::ptr_eq(&self.shaders, &completed.owners.shaders) {
            return Err(rhapsody::Error::invalid_usage("completed gameplay warmup has different App owners"));
        }
        let mut named = completed.named.lock();
        if let Some(named) = named.take() {
            self.session.borrow_mut().adopt_completed_named_resources(named)?;
        }
        Ok(())
    }

    /// 将同一设备的资产表安装到游戏世界；每帧维护由 GameFrameDriver 调度。
    pub fn install_asset_libraries(
        &self,
        world: &mut bevy_ecs::world::World,
    ) -> rhapsody::Result<()> {
        use country_core::asset_libraries::{
            mesh_library::AssetMeshLibrary, runtime::AssetLibraryRuntime,
            texture_library::AssetTextureLibrary, vao_library::AssetVAOLibrary,
        };
        if world.contains_resource::<AssetLibraryRuntime>()
            || world.contains_resource::<AssetTextureLibrary>()
            || world
                .get_resource::<AssetVAOLibrary>()
                .is_some_and(|library| !library.is_empty())
        {
            return Err(rhapsody::Error::invalid_usage(
                "资产设备已经安装，不能更换资源所有者",
            ));
        }
        let mut textures = AssetTextureLibrary::try_new(self.tables.textures.as_ref())?;
        textures.try_create_preset_textures(self.tables.textures.as_ref())?;
        world.init_resource::<AssetMeshLibrary>();
        world.init_resource::<AssetVAOLibrary>();
        world.init_resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();
        world.init_resource::<country_core::geometry::runtime::WallBrickUploadStatus>();
        world.insert_resource(textures);
        world.insert_resource(AssetLibraryRuntime::new(
            self.tables.buffers.clone(),
            self.tables.textures.clone(),
        ));
        world.insert_resource(country_core::render::device::GraphicsDeviceResource::new(
            country_core::rhapsody_contract::GraphicsDevice::new(self.tables.buffers.graphics_device().clone())
                .with_buffer_host(self.tables.buffers.clone()),
        ));
        Ok(())
    }
    pub fn hosts(&self) -> &HostTables {
        &self.tables
    }

    pub fn shader_library(&self) -> &Arc<AssetShaderLibrary> {
        &self.shaders
    }

    pub fn shader_backend(&self) -> &ShaderHost {
        &self.tables.shaders
    }

    pub fn compute_device(&self) -> Arc<DeviceHost> {
        Arc::new(DeviceHost {
            buffers: self.tables.buffers.clone(),
            textures: self.tables.textures.clone(),
            shaders: self.tables.shaders.clone(),
        })
    }

    pub fn raster_resources(&self) -> &Arc<RasterShared> {
        &self.raster
    }

    /// Build core consumers and execute only after every construction phase succeeds.
    /// The caller must declare exports/side effects; rhapsody controls culling and
    /// publication. The receipt is submission evidence, not GPU completion.
    pub fn render_frame<T>(
        &mut self,
        build: impl FnOnce(&mut CoreGraphFrame<'_>) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<(T, SessionFrameReceipt)> {
        let (value, graph) = self.build_frame(build)?;
        let receipt = self.execute_built(graph)?;
        Ok((value, receipt))
    }

    /// Separate graph declaration from execution for the recovered outer render
    /// loop (B 0x1404e8010: compile main -> execute -> acquire -> display).
    /// The returned graph must be executed or cancelled before declaring another.
    pub fn build_frame<T>(
        &mut self,
        build: impl FnOnce(&mut CoreGraphFrame<'_>) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<(T, rhapsody::rg::graph::BuiltGraph)> {
        let graph = self.session.borrow_mut().begin_frame()?;
        let declarer = Rc::new(FrameDeclarer {
            session: self.session.clone(),
            scope: FrameScope {
                active: Cell::new(true),
                graph: graph.graph_id(),
            },
        });
        let mut frame = CoreGraphFrame {
            bridge: self,
            declarer,
            graph: Some(Rc::new(RefCell::new(graph))),
            failed: false,
        };
        let value = build(&mut frame)?;
        if frame.failed {
            return Err(rhapsody::Error::invalid_usage(
                "a failed core frame phase was ignored by its caller",
            ));
        }
        let graph = frame.take_graph()?;
        Ok((value, graph.build()))
    }

    pub fn execute_built(
        &mut self,
        graph: rhapsody::rg::graph::BuiltGraph,
    ) -> rhapsody::Result<SessionFrameReceipt> {
        self.session.borrow_mut().execute_frame(graph)
    }

    pub fn compile_built(
        &mut self,
        graph: rhapsody::rg::graph::BuiltGraph,
    ) -> rhapsody::Result<CompiledCoreGraph> {
        let frame = self.session.borrow_mut().compile_frame(graph)?;
        Ok(CompiledCoreGraph {
            session: self.session.clone(),
            frame: Some(frame),
        })
    }

    pub fn execute_compiled(
        &mut self,
        mut compiled: CompiledCoreGraph,
    ) -> rhapsody::Result<SessionFrameReceipt> {
        if !Rc::ptr_eq(&self.session, &compiled.session) {
            return Err(rhapsody::Error::invalid_usage(
                "compiled frame belongs to another bridge",
            ));
        }
        self.session
            .borrow_mut()
            .execute_prepared(compiled.frame.take().expect("compiled graph consumed once"))
    }

    pub fn execute_presentation_compiled(
        &mut self,
        compiled: CompiledCoreGraph,
        lease: Arc<parking_lot::Mutex<rhapsody::raw::swapchain::GraphImageLease>>,
    ) -> rhapsody::Result<SessionFrameReceipt> {
        self.session
            .borrow_mut()
            .backend
            .attach_presentation_submission(lease)?;
        let result = self.execute_compiled(compiled);
        self.session
            .borrow_mut()
            .backend
            .detach_presentation_submission();
        result
    }

    pub fn cancel_built(&mut self, graph: rhapsody::rg::graph::BuiltGraph) -> rhapsody::Result<()> {
        self.session.borrow_mut().cancel_frame(graph)
    }

    /// Use the frame's actual allocation mapping; a logical graph index is never
    /// a Vulkan handle. The Arc remains owned through the display submission.
    pub fn exported_texture(
        &self,
        receipt: &SessionFrameReceipt,
        resource: u32,
    ) -> rhapsody::Result<Arc<rhapsody::raw::texture::Texture>> {
        let mapping = receipt
            .snapshot
            .physical_mapping
            .iter()
            .find(|mapping| mapping.logical == resource)
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("exported texture is not in this frame")
            })?;
        // SnapshotStatic.exports stores logical resource indices; physical
        // allocation indices can differ after pooling/aliasing. The latter
        // only locates the actual retained owner below.
        if !u16::try_from(resource)
            .is_ok_and(|logical| receipt.snapshot.exports.contains(&logical))
        {
            return Err(rhapsody::Error::invalid_usage(
                "texture was not exported by this frame",
            ));
        }
        let token = mapping.token.ok_or_else(|| {
            rhapsody::Error::invalid_usage("exported texture has no physical allocation")
        })?;
        match self.session.borrow().backend.resource_clone(token) {
            Some(rhapsody::rg::vulkan::GpuResource::Image(texture)) => Ok(texture),
            _ => Err(rhapsody::Error::invalid_usage(
                "exported resource is not a texture",
            )),
        }
    }

    /// Transfer the exported owner to its next graph/consumer. In-flight native
    /// commands retain their Arc independently; keeping the old export token
    /// forever would leak one main target per frame.
    pub fn take_exported_texture(
        &mut self,
        receipt: &SessionFrameReceipt,
        resource: u32,
    ) -> rhapsody::Result<Arc<rhapsody::raw::texture::Texture>> {
        let texture = self.exported_texture(receipt, resource)?;
        let token = receipt
            .snapshot
            .physical_mapping
            .iter()
            .find(|m| m.logical == resource)
            .and_then(|m| m.token)
            .expect("validated export token");
        rhapsody::rg::graph::ResourceBackend::release_created_resource(
            &mut self.session.borrow_mut().backend,
            token,
        );
        Ok(texture)
    }

    pub fn with_session<T>(&mut self, body: impl FnOnce(&mut VulkanGraphSession) -> T) -> T {
        body(&mut self.session.borrow_mut())
    }

    pub fn collect_completed_frames(&mut self) -> rhapsody::Result<usize> {
        self.session.borrow_mut().collect_completed_frames()
    }

    pub fn wait_submits_done(&mut self, timeout_ns: u64) -> rhapsody::Result<()> {
        self.session.borrow_mut().wait_submits_done(timeout_ns)
    }
}

/// A compiled, allocated frame. Dropping it before execution follows the
/// original failed-compilation/abandoned-frame resource retirement path.
pub struct CompiledCoreGraph {
    session: Rc<RefCell<VulkanGraphSession>>,
    frame: Option<rhapsody::rg::vulkan::PreparedSessionFrame>,
}
impl CompiledCoreGraph {
    /// Inspect the actual prepared plan before executing its GPU callbacks.
    pub fn snapshot(&self) -> &rhapsody::rg::graph::Snapshot {
        self.frame.as_ref().expect("compiled graph consumed once").snapshot()
    }
}
impl Drop for CompiledCoreGraph {
    fn drop(&mut self) {
        if let Some(frame) = self.frame.take() {
            if let Err(error) = self.session.borrow_mut().cancel_prepared(frame) {
                log::warn!("cancelling unsubmitted compiled core frame: {error}");
            }
        }
    }
}

/// A scoped declaration phase. No GPU submission is possible through this object.
/// Raster hosts must be consumed during the callback; escaping hosts are rejected
/// before submission, because they could otherwise mutate an already built graph.
pub struct CoreGraphFrame<'a> {
    bridge: &'a CoreRenderBridge,
    declarer: Rc<FrameDeclarer>,
    graph: Option<Rc<RefCell<RenderGraph>>>,
    failed: bool,
}

impl CoreGraphFrame<'_> {
    /// The current App's library, not another App sharing the physical device.
    pub fn shader_library(&self) -> Arc<AssetShaderLibrary> {
        self.bridge.shaders.clone()
    }

    pub fn compute_device(&self) -> Arc<DeviceHost> {
        self.bridge.compute_device()
    }

    pub fn shader_backend(&self) -> Arc<ShaderHost> {
        self.bridge.tables.shaders.clone()
    }

    pub fn raster_resources(&self) -> Arc<RasterShared> {
        self.bridge.raster.clone()
    }

    pub fn hosts(&self) -> HostTables {
        self.bridge.tables.clone()
    }

    fn graph(&self) -> Rc<RefCell<RenderGraph>> {
        self.graph.as_ref().expect("frame already consumed").clone()
    }

    pub fn with_compute<T>(
        &mut self,
        build: impl FnOnce(&mut CoreComputeFrame<'_, '_>) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<T> {
        let result = (|| {
            let device = self.bridge.session.borrow().backend.device.clone();
            let passes = GraphPasses::new(self.graph(), device)?
                .with_tables(self.bridge.tables.clone())
                .with_shared_session(self.bridge.session.clone())?;
            let mut frame = CoreComputeFrame::new(passes, &self.bridge.shaders)
                .with_resource_graph(self.resource_graph());
            let result = build(&mut frame);
            // A void PassFrame::record failure predates a later closure error.
            frame.finish()?;
            result
        })();
        self.failed |= result.is_err();
        result
    }

    pub fn resource_graph_factory(
        &self,
    ) -> Rc<dyn Fn() -> country_core::rhapsody_types::RenderGraph> {
        let graph = self.graph();
        let declarer = self.declarer.clone();
        let tables = self.bridge.tables.clone();
        Rc::new(move || {
            declarer.scope.validate(&graph.borrow())
                .expect("resource graph factory belongs to a closed frame");
            country_core::rhapsody_types::RenderGraph::with_backend(FrameBufferGraph {
                graph: graph.clone(),
                declarer: declarer.clone(),
                tables: tables.clone(),
            })
        })
    }

    pub fn resource_graph(&self) -> country_core::rhapsody_types::RenderGraph {
        self.resource_graph_factory()()
    }

    /// Main-graph-only setup at B 0x140161fbc. Loading and presentation do
    /// not import Nani instance subsets. The caller must create this actual
    /// GpuWorld with the current App's BufferHost and preserve the matching
    /// export below, including when a main builder returns an error.
    pub fn import_gpu_world(&mut self, world: &mut bevy_ecs::world::World) -> rhapsody::Result<()> {
        let result = (|| {
            let mut gpu = world
                .get_resource_mut::<country_core::render::nani::gpu_world::GpuWorld>()
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "original compile_main 0x140161fbc: missing GpuWorld producer",
                    )
                })?;
            gpu.import_to_rg(&mut self.resource_graph());
            Ok(())
        })();
        self.failed |= result.is_err();
        result
    }

    /// The native main closure's symmetric export at B 0x140163788. Only
    /// buffers actually imported by Owned Nani subsets are exported; External
    /// subsets and all other library buffers remain governed by their owners.
    pub fn export_gpu_world(&mut self, world: &mut bevy_ecs::world::World) -> rhapsody::Result<()> {
        let result = (|| {
            let mut gpu = world
                .get_resource_mut::<country_core::render::nani::gpu_world::GpuWorld>()
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "original compile_main 0x140163788: missing GpuWorld producer",
                    )
                })?;
            gpu.export_from_rg(&mut self.resource_graph());
            Ok(())
        })();
        self.failed |= result.is_err();
        result
    }

    /// Callable original main-graph prelude, not a replacement scene renderer.
    /// B: GpuWorld import @161fbc -> FrameConstants @16210a -> LUT @1622e7
    /// -> reset indirect @1622fd. The caller must provide the real downstream
    /// scene consumer; production's missing scene entry is not made successful
    /// by running only this prefix. Loading/presentation never call this helper.
    pub fn with_main_prelude<T>(
        &mut self,
        world: &mut bevy_ecs::world::World,
        build: impl FnOnce(&mut CoreGraphFrame<'_>, &mut bevy_ecs::world::World) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<T> {
        use country_core::{
            asset_libraries::{placement_data_library::AssetPlacementDataLibrary,
                ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
                vao_library::AssetVAOLibrary},
            resources::gpu_lut::GpuLutTracker,
            systems::render_loop::{compute_passes::{pass_kit::PassEnv,
                reset_indirect_args::reset_indirect_args}, indirect_params::{AtomicCounterBuffer,
                DispatchIndirectBuffer, DrawIndexedIndirectBuffer, DrawIndirectBuffer}},
        };
        use crate::frame_world::{FrameLibraries, FrameWorldAccess};

        self.import_gpu_world(world)?;
        let shaders = self.bridge.shaders.clone();
        let result = crate::frame_setup::with_frame_resources(self, world, |frame, world| {
            if !world.contains_resource::<GpuLutTracker>() {
                return Err(rhapsody::Error::invalid_usage(
                    "original compile_main 0x1401622e7: missing GpuLutPlugin producer"));
            }
            world.resource_scope(|world, mut tracker: bevy_ecs::change_detection::Mut<GpuLutTracker>| {
                let missing = |name| rhapsody::Error::invalid_usage(format!(
                    "original main prelude: missing {name}"));
                let vaos = world.get_resource::<AssetVAOLibrary>().ok_or_else(|| missing("AssetVAOLibrary"))?;
                let placement = world.get_resource::<AssetPlacementDataLibrary>().ok_or_else(|| missing("AssetPlacementDataLibrary"))?;
                let textures = world.get_resource::<AssetTextureLibrary>().ok_or_else(|| missing("AssetTextureLibrary"))?;
                let ssbos = world.get_resource::<AssetSsboLibrary>().ok_or_else(|| missing("AssetSsboLibrary"))?;
                let backend = frame.shader_backend();
                let raster = frame.raster_resources();
                let access = FrameWorldAccess::new(FrameLibraries { shaders: &shaders, textures,
                    ssbos, vaos, placement, constants: raster.dynamic_constants() }, frame.compute_device())
                    .with_resource_graph_factory(frame.resource_graph_factory());
                frame.with_compute(|compute| {
                    let mut uniforms = crate::compute_frame::ComputeUniformAllocator;
                    let mut env = PassEnv::new(&access, &*backend, &mut uniforms, compute);
                    tracker.render_luts(&mut env)?;
                    // Native LUT declaration (including its rendered flag)
                    // precedes the reset consumer's mandatory buffer lookups.
                    if !ssbos.contains::<DispatchIndirectBuffer>() || !ssbos.contains::<DrawIndexedIndirectBuffer>()
                        || !ssbos.contains::<DrawIndirectBuffer>() || !ssbos.contains::<AtomicCounterBuffer>() {
                        return Err(missing("original indirect SSBO startup producers (0x1401622fd)"));
                    }
                    reset_indirect_args(compute, ssbos)
                })
            })?;
            build(frame, world)
        });
        // The host clears frame-local IDs on a failed declaration as well.
        // It does not undo the native LUT rendered flag set during declaration.
        let export = self.export_gpu_world(world);
        result.and_then(|value| export.map(|()| value))
    }

    pub fn with_raster<T>(
        &mut self,
        build: impl FnOnce(RasterPassHost) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<T> {
        let declarer: Rc<dyn GraphDeclarer> = self.declarer.clone();
        let host = self.bridge.raster.pass_host(self.graph(), Some(declarer))
            .with_resource_graph_factory(self.resource_graph_factory());
        let result = build(host);
        self.failed |= result.is_err();
        result
    }

    /// For typed export declarations and pass-specific operations not yet covered
    /// by core's facade. The graph remains owned by the frame driver.
    pub fn with_graph<T>(
        &mut self,
        build: impl FnOnce(&mut RenderGraph) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<T> {
        let graph = self.graph();
        let result = build(&mut graph.borrow_mut());
        self.failed |= result.is_err();
        result
    }

    pub fn import_buffer(
        &mut self,
        buffer: &Buffer,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        let result = (|| {
            let buffer = self
                .bridge
                .tables
                .buffers
                .managed_buffer(buffer)
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage("buffer handle is stale or unavailable")
                })?;
            self.bridge.session.borrow_mut().import_frame_buffer(
                &mut self.graph().borrow_mut(),
                buffer.shared(),
                initial_access,
            )
        })();
        self.failed |= result.is_err();
        result
    }

    /// External engine resources still enter the same session's generation and
    /// lifetime checks. This is also used by bounded GPU readback validation.
    pub fn import_native_buffer(
        &mut self,
        buffer: Arc<rhapsody::raw::buffer::Buffer>,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        let result = self.bridge.session.borrow_mut().import_frame_buffer(
            &mut self.graph().borrow_mut(),
            buffer,
            initial_access,
        );
        self.failed |= result.is_err();
        result
    }

    pub fn import_native_texture(
        &mut self,
        texture: Arc<rhapsody::raw::texture::Texture>,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        let result = self.bridge.session.borrow_mut().import_frame_texture(
            &mut self.graph().borrow_mut(),
            texture,
            initial_access,
        );
        self.failed |= result.is_err();
        result
    }

    pub fn import_native_texture_owned(
        &mut self,
        texture: Arc<rhapsody::raw::texture::Texture>,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        let result = self.bridge.session.borrow_mut().import_frame_texture_owned(
            &mut self.graph().borrow_mut(),
            texture,
            initial_access,
        );
        self.failed |= result.is_err();
        result
    }

    /// Declare a raw graph image with a full rhapsody description. The core
    /// placeholder `TextureDesc` carries no `sample_count`, so the original
    /// `"msaa color out"` target (`new_2d(extent, 0x2b).sample_count(4)`,
    /// 0x14090d000) cannot be expressed through `RgResources::create_raw_texture`.
    /// This keeps the caller's native description instead of silently creating a
    /// single-sample image. Access-derived usage bits are added by rhapsody's
    /// resource plan exactly as for any other declared image.
    pub fn declare_native_image(
        &mut self,
        name: &str,
        desc: rhapsody::raw::texture::TextureDesc,
    ) -> rhapsody::Result<u32> {
        let result = self
            .declarer
            .declare(&mut self.graph().borrow_mut(), name, ResourceSpec::Image(desc))
            .map(|handle| handle.raw())
            .map_err(Into::into);
        self.failed |= result.is_err();
        result
    }

    pub fn import_texture(
        &mut self,
        texture: &Texture,
        initial_access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        let result = (|| {
            let texture = self.bridge.tables.textures.get(texture).ok_or_else(|| {
                rhapsody::Error::invalid_usage("texture handle is stale or unavailable")
            })?;
            self.bridge.session.borrow_mut().import_frame_texture(
                &mut self.graph().borrow_mut(),
                texture.shared(),
                initial_access,
            )
        })();
        self.failed |= result.is_err();
        result
    }

    fn take_graph(&mut self) -> rhapsody::Result<RenderGraph> {
        let graph = self.graph.as_ref().expect("frame already consumed");
        if Rc::strong_count(graph) != 1 {
            return Err(rhapsody::Error::invalid_usage(
                "a core raster host escaped its frame builder",
            ));
        }
        self.declarer.scope.active.set(false);
        Ok(Rc::try_unwrap(self.graph.take().unwrap())
            .ok()
            .expect("exclusive frame graph")
            .into_inner())
    }
}

impl Drop for CoreGraphFrame<'_> {
    fn drop(&mut self) {
        self.declarer.scope.active.set(false);
        let Some(graph) = self.graph.take() else {
            return;
        };
        let graph = match Rc::try_unwrap(graph) {
            Ok(graph) => graph.into_inner(),
            // Disconnect escaped hosts from the cancelled declaration. They
            // cannot mutate the graph that was just cancelled or a future frame.
            Err(graph) => graph.replace(RenderGraph::new()),
        };
        if let Err(error) = self.bridge.session.borrow_mut().cancel_frame(graph.build()) {
            log::warn!("cancelling unsubmitted core frame: {error}");
        }
    }
}

// A raster host owns its declarer, so its lease must be revoked independently
// of Rc counts. An escaped host can never mutate the session's next frame.
struct FrameScope {
    active: Cell<bool>,
    graph: GraphId,
}
impl FrameScope {
    fn validate(&self, graph: &RenderGraph) -> rhapsody::Result<()> {
        if !self.active.get() || self.graph != graph.graph_id() {
            return Err(rhapsody::Error::invalid_usage(
                "raster host belongs to a closed or different frame",
            ));
        }
        Ok(())
    }
}
struct FrameDeclarer {
    session: Rc<RefCell<VulkanGraphSession>>,
    scope: FrameScope,
}
/// Native get_rg_buffer remains a lazy library operation. This facade binds
/// its historical u64 handle projection to the same physical host tables,
/// graph scope and actual session as compute/raster declaration.
struct FrameBufferGraph {
    graph: Rc<RefCell<RenderGraph>>,
    declarer: Rc<FrameDeclarer>,
    tables: HostTables,
}
impl country_core::rhapsody_types::BufferGraphBackend for FrameBufferGraph {
    fn import_unowned(
        &mut self,
        id: u64,
        access: u8,
    ) -> country_core::rhapsody_types::ImportedBuffer {
        let mut graph = self
            .graph
            .try_borrow_mut()
            .expect("lazy resource graph is already borrowed");
        self.declarer
            .scope
            .validate(&graph)
            .expect("lazy import belongs to a closed frame");
        let handle = if id & crate::render_host::TEXTURE_ID_TAG != 0 {
            let texture = self
                .tables
                .textures
                .get_by_id(id)
                .expect("lazy import texture is not live in this App");
            self.declarer
                .import_texture(&mut graph, texture.shared(), access)
        } else {
            let buffer = self
                .tables
                .buffers
                .get_by_id(id)
                .expect("lazy import buffer is not live in this App");
            self.declarer
                .import_buffer(&mut graph, buffer.shared(), access)
        }
        .expect("lazy import failed in its owning frame session");
        country_core::rhapsody_types::ImportedBuffer {
            resource: u64::from(handle.raw()),
            // This is source-host scope metadata, not an asserted native ABI.
            generation: graph.graph_id().raw(),
            access: u32::from(access),
        }
    }
    fn export(&mut self, imported: country_core::rhapsody_types::ImportedBuffer, access: u8) {
        let mut graph = self
            .graph
            .try_borrow_mut()
            .expect("lazy resource graph is already borrowed");
        self.declarer
            .scope
            .validate(&graph)
            .expect("lazy export belongs to a closed frame");
        assert_eq!(
            imported.generation,
            graph.graph_id().raw(),
            "lazy export belongs to another graph"
        );
        let handle = graph
            .resource_handle(
                u32::try_from(imported.resource).expect("lazy import index exceeds u32"),
            )
            .expect("lazy imported buffer does not belong to this graph");
        graph
            .export(handle, access)
            .expect("lazy resource export failed");
    }
}
impl GraphDeclarer for FrameDeclarer {
    fn import_buffer(
        &self,
        graph: &mut RenderGraph,
        buffer: Arc<rhapsody::raw::buffer::Buffer>,
        access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        self.scope.validate(graph)?;
        self.session
            .borrow_mut()
            .import_frame_buffer(graph, buffer, access)
    }
    fn import_texture(
        &self,
        graph: &mut RenderGraph,
        texture: Arc<rhapsody::raw::texture::Texture>,
        access: u8,
    ) -> rhapsody::Result<ResourceHandle> {
        self.scope.validate(graph)?;
        self.session
            .borrow_mut()
            .import_frame_texture(graph, texture, access)
    }
    fn declare(
        &self,
        graph: &mut RenderGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<ResourceHandle, rhapsody::rg::graph::BuildError> {
        self.scope.validate(graph)?;
        self.session.borrow_mut().backend.declare(graph, name, spec)
    }
    fn named_texture(
        &self,
        graph: &mut RenderGraph,
        name: &str,
        desc: rhapsody::raw::texture::TextureDesc,
    ) -> rhapsody::Result<rhapsody::rg::temporal::NamedTexture> {
        self.scope.validate(graph)?;
        self.session.borrow_mut().named_texture(graph, name, desc)
    }
    fn temporal_texture(
        &self,
        graph: &mut RenderGraph,
        name: &str,
        desc: rhapsody::raw::texture::TextureDesc,
    ) -> rhapsody::Result<rhapsody::rg::temporal::TemporalTexturePair> {
        self.scope.validate(graph)?;
        self.session
            .borrow_mut()
            .temporal_texture(graph, name, desc)
    }
    fn named_buffer(
        &self,
        graph: &mut RenderGraph,
        name: &str,
        desc: rhapsody::raw::buffer::BufferDesc,
    ) -> rhapsody::Result<rhapsody::rg::temporal::NamedBuffer> {
        self.scope.validate(graph)?;
        self.session.borrow_mut().named_buffer(graph, name, desc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_declaration_lease_rejects_other_graphs_and_revoked_hosts() {
        let graph = RenderGraph::new();
        let scope = FrameScope {
            active: Cell::new(true),
            graph: graph.graph_id(),
        };
        assert!(scope.validate(&graph).is_ok());
        assert!(scope.validate(&RenderGraph::new()).is_err());
        scope.active.set(false);
        assert!(scope.validate(&graph).is_err());
    }
}


/// Actual full-world builder seam. The caller supplies the recovered world
/// stage plus the compile_main tail subset, ending in the real final22 export
/// of `"msaa color out"` (0x2b, sample_count 4, display extent) — not the TAA
/// texture. Equivalence with the complete native compile_main is **not**
/// claimed: `world_postprocess::WorldTailControls` lists the tail steps whose
/// hosts/producers remain U, and the delivery records them. Actual ECS
/// draw-list construction and every render phase use the same App owners.
/// Sending the SubApp is allowed only after this function returns Ok.
pub fn warm_up_gameplay_with_world_builder(
    owners: CoreRenderOwners,
    world: &mut bevy_ecs::world::World,
    build_world: impl FnOnce(
        &mut CoreGraphFrame<'_>, &mut bevy_ecs::world::World,
        &country_core::systems::render_loop::draw_passes::draw_lists::DrawLists,
    ) -> rhapsody::Result<country_core::render::pass_builder::pass_resource_access::RgTexture>,
) -> rhapsody::Result<()> {
    use country_core::systems::render_loop::draw_passes::draw_lists::{build_draw_lists, DrawLists};
    let mut bridge = CoreRenderBridge::from_owners(owners)?;
    let mut lists = DrawLists::new();
    build_draw_lists(world, bridge.shader_library().as_ref(), &mut lists);
    lists.sort();
    bridge.raster_resources().refresh_draw_vaos(world, &lists)?;
    let (output, built) = bridge.build_frame(|frame| build_world(frame, world, &lists))?;
    if let Err(error) = bridge.shader_backend().prewarm(std::time::Duration::from_secs(10)) {
        if let Err(cleanup) = bridge.cancel_built(built) {
            log::warn!("failed gameplay warmup prewarm cancellation: {cleanup}");
        }
        return Err(error);
    }
    let compiled = bridge.compile_built(built)?;
    let receipt = bridge.execute_compiled(compiled)?;
    // Real final export, physical allocation and retained native owner are
    // required; neither a logical texture number nor a ready flag suffices.
    let actual_output = bridge.exported_texture(&receipt, output.resource)?;
    let completed = bridge.complete_gameplay_warmup(10_000_000_000)?;
    drop(bridge.take_exported_texture(&receipt, output.resource)?);
    drop(actual_output);
    world.insert_resource(completed);
    Ok(())
}
