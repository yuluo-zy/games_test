//! country-core — assembled reconstruction of the Tiny Glade `country_core` RENDER
//! domain (version B), integrating the five parallel work packages:
//!
//!   - wp1 `render/pass_builder`   pass-declaration layer (ComputePass / DrawBuilder /
//!                                 RasterPass / DrawPacket / PassResources / UniformBinder /
//!                                 ShaderUri / pass-access binding types)
//!   - wp2 `render/*`              GPU buffer primitives (TypedStorageBuffer / InstanceBuffer /
//!                                 ShadowedBuffer / SparseInstanceBuffer / indirect wrappers)
//!   - wp3 `render/nani`           nani GPU-driven world / instance data / mesh atlas
//!                                 (GpuWorld / subsets / MeshAtlas / TrimeshBuilder /
//!                                 cull-shaders closure structs)
//!   - wp4 `render` + `asset_libraries`   mesh / shader / config / view+frame constants /
//!                                 asset libraries (Mesh, FrameConstants, ViewConstants,
//!                                 DirectionalLight, SunSkyColors, RenderConfig, Asset*Library)
//!   - wp5 `systems/render_loop`   frame-pass composition (compute_passes/ + draw_passes/),
//!                                 wired to the real types through `crate::consume`.
//!
//! Rhapsody boundary decision (see `_integration/INTEGRATION_REPORT.md`): the crate's
//! single canonical render-graph boundary is `crate::rhapsody_contract` — wp4's typed
//! contract, extended to the game-facing surface the wp skeletons use. `rhapsody-rebuild`
//! (path dep) is declared as the reused boundary but its recovered surface (raw Vulkan /
//! queue machinery + graph core) does not overlap the game-facing API the packages call;
//! no fabricated bridging is introduced. Puffin / enum-iterator / nohash-hasher are
//! offline-cache gaps and are served by `puffin_contract` / manual `all()` / local
//! no-op hashers.

extern crate alloc;

// Batch-4 assembly (wp11-16): the app-shell / input / startup / ui / audio /
// wall / session-history-save / misc resource+system domains merged into the
// render-domain crate. See `_integration/INTEGRATION_BATCH4.md`.
pub mod app_loading_state; // wp15 app-shell
pub mod asset_libraries; // wp4 + wp14 audio_trigger_library + wp16 non-render registries
pub mod audio_events; // wp14 audio event rows (16×register_event, O)
pub mod frozen_map; // `frozen::FrozenMap` call-surface stand-in (external crate)
pub mod name_hash; // utils::name_hash::NameHash (xxh3_64)
pub mod rhapsody_types; // single placeholder set for rhapsody texture/shader types
pub mod boundary; // wp16 game-side render/core boundary (real types where surface matches)
pub mod components; // wp6 drawable + wp16 residual markers/geometry/utils
pub mod game_config; // wp15
pub mod game_key_bindings; // wp15
pub mod game_profile; // wp15
pub mod geometry; // wp15 procedural geometry
pub mod input; // wp15 input domain
pub mod meta_commander; // wp15
pub mod render;
pub mod reused; // wp16 reused-dep call surfaces (contract modules; deps not offline)
pub mod startup; // wp15 startup + wp14 audio startup
pub mod systems;
pub mod ui; // wp12 wall-domain UI typed-gap boundary (distinct from systems::ui)
// `crate::utils` / `crate::resources` are file modules now (merged wp5-wp16 content).

/// Typed-gap marker: a field whose layout/semantics were not recovered from the
/// original machine code. Never fabricated.
///
/// NOTE: from the WP20 integration, `Gap` also derives bevy `Resource` /
/// `Component` / `Event` so the many `Res<Gap>` / `Query<…Gap…>` /
/// `EventWriter<Gap>` placeholder system parameters in the wp20 camera/UI/time
/// systems compile. The integration owner swaps each placeholder for the real
/// country-core type where recovered; the placeholders are documented in
/// INTEGRATION_FINAL.md. The derives are inert otherwise.
#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
    bevy_ecs::resource::Resource,
    bevy_ecs::component::Component,
    bevy_ecs::event::Event,
)]
pub struct Gap;

/// Alias module so modules can write `crate::typed_gap::Gap`.
pub mod typed_gap {
    pub use crate::Gap;
    /// Typed-gap event placeholder (wp14 path/garden audio read `EventReader<…>`
    /// on a not-yet-recovered event type).
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct GapEventPlaceholder;
}

// ===========================================================================
// Contract for the REUSED `rhapsody` crate surface (crate's single boundary).
// ===========================================================================
pub mod rhapsody_contract {
    use std::marker::PhantomData;

    use super::render::shader::{ComputeShaderProgram, RasterShaderProgram};

    /// Vulkan device handle (reused `rhapsody::raw::device::GraphicsDevice`).
    #[derive(Clone, Default)]
    pub struct GraphicsDevice(pub u64);

    impl GraphicsDevice {
        pub fn create_persistent_descriptor_set(&self, _name: &str) -> DescriptorSet {
            DescriptorSet
        }
        pub fn update_buffer_bytes(
            &self,
            _handle: ResourceHandle,
            _bytes: &[u8],
        ) -> Result<(), TextureError> {
            Ok(())
        }
        pub fn update_persistent_descriptor_set(&self) {}

        // ---- wp1 `ComputePass::dispatch_3d_immediate` surface (added at assembly) ----
        /// `full_barrier_required` (O call site in `dispatch_3d_immediate`); typed gap.
        pub fn full_barrier_required(&self) -> bool {
            false
        }
        /// `issue_full_pipeline_barrier` (O: `rhapsody::raw::device::GraphicsDevice`).
        pub fn issue_full_pipeline_barrier(&self, _command: ash::vk::CommandBuffer) {}
        /// `dispatch` — vkCmdDispatch groups (O call site).
        pub fn dispatch(&self, _command: ash::vk::CommandBuffer, _x: u32, _y: u32, _z: u32) {}
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct DescriptorSet;

    /// 设备/着色器/纹理占位类型：全 crate 唯一一份，见 `crate::rhapsody_types`。
    pub use crate::render::pass_builder::shader_uri::ComputePipelineDesc;
    pub use crate::rhapsody_types::{
        CompiledShaderPipeline, ComputePipeline, RasterPipeline, RasterPipelineDesc, RasterPipelineKey,
        Texture, TextureDesc,
    };

    /// Managed GPU-object handle (reused `rhapsody::raw::managed::handle`).
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct ManagedHandle(pub u64);

    /// Managed handle wrapper for a typed GPU object
    /// (reused `rhapsody::raw::managed::handle::Handle<T>`; added at assembly for
    /// the wp1 `DrawPacket` / `draw_stream` surface).
    /// Clone/Debug are implemented manually (no `T: Clone/Debug` bound) so the
    /// handle stays usable as a map key / packet field for any object type.
    pub struct Handle<T>(pub ManagedHandle, pub PhantomData<fn() -> T>);
    impl<T> Clone for Handle<T> {
        fn clone(&self) -> Self {
            Handle(self.0.clone(), PhantomData)
        }
    }
    impl<T> core::fmt::Debug for Handle<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_tuple("Handle").field(&self.0).finish()
        }
    }
    impl<T> Handle<T> {
        /// Typed-gap constructor (original `shader::impl$7::default()` path).
        pub fn gap_new() -> Self {
            Handle(ManagedHandle(0), PhantomData)
        }
        /// Managed-handle word (O: `handle()` call sites).
        pub fn handle(&self) -> u32 {
            self.0.0 as u32
        }
        /// Deref to the managed object (O call site); wired to rhapsody in integration.
        pub fn get(&self) -> &T {
            todo!("managed Handle<T>::get — wired to rhapsody managed handle")
        }
    }

    /// Temporal read-only handle (reused `rhapsody::rg::temporal::ReadOnlyHandle<T>`).
    pub struct ReadOnlyHandle<T>(pub Handle<T>);
    impl<T> Clone for ReadOnlyHandle<T> {
        fn clone(&self) -> Self {
            ReadOnlyHandle(self.0.clone())
        }
    }
    impl<T> core::fmt::Debug for ReadOnlyHandle<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_tuple("ReadOnlyHandle").field(&self.0).finish()
        }
    }
    impl<T> ReadOnlyHandle<T> {
        pub fn handle(&self) -> u32 {
            self.0.handle()
        }
        pub fn get(&self) -> &T {
            todo!("ReadOnlyHandle<T>::get — wired to rhapsody temporal pair")
        }
    }

    /// RG resource handle.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ResourceHandle(pub u64);
    impl ResourceHandle {
        pub const NONE: ResourceHandle = ResourceHandle(0);
    }

    /// Texture view descriptor (reused `rhapsody::raw::texture::TextureViewDesc`)；唯一定义见 `rhapsody_types`。
    pub use crate::rhapsody_types::TextureViewDesc;

    /// 2D extent (reused `rhapsody::raw::texture::Extent2D` — `vk::Extent2D`).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Extent2D {
        pub width: u32,
        pub height: u32,
    }

    /// Render pass descriptor used as a dedupe key
    /// (reused `rhapsody::raw::render_pass::RenderPassDesc`).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct RenderPassDesc {
        pub selector: u32,
    }

    /// Runtime render pass (reused `rhapsody::raw::render_pass::RenderPass`).
    #[derive(Clone, Debug, Default)]
    pub struct RenderPass;

    /// Render-pass attachment descriptor
    /// (reused `rhapsody::raw::shader::RenderPassAttachmentDesc`).
    #[derive(Clone, Copy, Debug, Default)]
    pub struct RenderPassAttachmentDesc;
    impl RenderPassAttachmentDesc {
        pub fn new() -> Self {
            Self
        }
    }

    /// Pass resource slot (reused `rhapsody::rg::binding::PassResourceSlot`).
    #[derive(Clone, Copy, Debug)]
    pub enum PassResourceSlot {
        StorageBuffer { resource: u32 },
        UniformBuffer { resource: u32 },
        SampledImage { resource: u32 },
        StorageImage { resource: u32 },
    }

    /// A recorded pass binding (reused `rhapsody::rg::pass_api::RenderPassBinding`).
    #[derive(Clone, Copy, Debug, Default)]
    pub struct RenderPassBinding {
        pub resource_index: u32,
        pub access_type: u8,
    }

    /// `rhapsody::rg::pass_api::bind` — record one resource access binding.
    pub fn bind(resource_index: u32, access_type: u8) -> RenderPassBinding {
        RenderPassBinding {
            resource_index,
            access_type,
        }
    }

    /// `rhapsody::rg::pass_builder::PassBuilder<UD>` — pass-declaration builder.
    /// The original is generic over the pass user-data type `UD`; only the surface
    /// the country-core skeletons call is defined here.
    #[derive(Debug)]
    pub struct PassBuilder<'a, UD> {
        _marker: PhantomData<&'a mut UD>,
    }
    impl<'a, UD> PassBuilder<'a, UD> {
        /// `PassBuilder::add_resource_access` (O: resource_index + access_code + group).
        pub fn add_resource_access(
            &mut self,
            _resource_index: u32,
            _access_type: u8,
            _sync_group: u32,
        ) {
        }
        /// `PassBuilder::resolve_resource` — hash → (set_index, resource_index) (typed gap).
        pub fn resolve_resource(_hash: u64) -> (u32, u32) {
            (0, 0)
        }
        /// `PassBuilder::render<PassResources, closure>` (O path; wired in integration).
        pub fn render(self, _callback: impl FnOnce(u64) -> bool + 'static) {}
    }

    // B-phase append (a4_compute): a builder not attached to a recorder — used by
    // `ComputePass`'s `BindGameResources::as_pass_builder` view; the live accesses are
    // recorded in `pass_resource_access::RecordingPassBuilder`.
    impl<'a, UD> PassBuilder<'a, UD> {
        pub fn detached() -> Self {
            PassBuilder { _marker: PhantomData }
        }
    }

    /// Render-graph handle (reused `rhapsody::rg::RenderGraph`; added at assembly
    /// for the wp3 `rhapsody_rg::render_graph::RenderGraph` import sites).
    #[derive(Default)]
    pub struct RenderGraph;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PipelineError {
        Artifacts,
        Create,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TextureError {
        Io,
        Create,
    }

    /// Storage-buffer descriptor with the observed vtable (+0x18 get-ptr,
    /// +0x20 get-mut, +0x30 get-value) + type name.
    pub trait StorageBufferDescriptor: Send + Sync {
        fn name(&self) -> &'static str;
    }
    pub struct StorageBufferDescriptorImpl<T> {
        // `fn() -> T` (not `T`) keeps the descriptor Send+Sync for any T.
        _marker: PhantomData<fn() -> T>,
    }
    impl<T: 'static> StorageBufferDescriptorImpl<T> {
        pub fn new() -> Self {
            StorageBufferDescriptorImpl {
                _marker: PhantomData,
            }
        }
        pub fn data_ptr(_data: &T) -> *mut u8 {
            core::ptr::null_mut()
        }
    }
    impl<T: 'static> StorageBufferDescriptor for StorageBufferDescriptorImpl<T> {
        fn name(&self) -> &'static str {
            std::any::type_name::<T>()
        }
    }

    // Concrete descriptors for the WP4-owned shader program types (used when
    // registering them through the SSBO-style descriptor table).
    impl StorageBufferDescriptor for ComputeShaderProgram {
        fn name(&self) -> &'static str {
            "country_core::render::shader::ComputeShaderProgram"
        }
    }
    impl StorageBufferDescriptor for RasterShaderProgram {
        fn name(&self) -> &'static str {
            "country_core::render::shader::RasterShaderProgram"
        }
    }
}

/// Contract for the REUSED `shader-pipeline` crate (`ShaderCompilerArtifacts`).
pub mod shader_pipeline_contract {
    use std::sync::Arc;

    use super::rhapsody_contract::{
        CompiledShaderPipeline, ComputePipelineDesc, RasterPipelineDesc,
    };

    pub struct ShaderCompilerArtifacts;

    /// `shader_pipeline::reflection::BindingIndex` — uniform binding index used as a
    /// key in `UniformBinder`'s binding map (wp1); reused-dep type (typed gap).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    pub struct BindingIndex(pub u32);

    impl ShaderCompilerArtifacts {
        pub fn load_compiled_compute(
            _desc: &ComputePipelineDesc,
        ) -> Result<Arc<CompiledShaderPipeline>, ()> {
            Ok(Arc::new(CompiledShaderPipeline(Vec::new())))
        }
        pub fn load_compiled_raster(
            _artifact: &CompiledShaderPipeline,
        ) -> Result<Arc<CompiledShaderPipeline>, ()> {
            Ok(Arc::new(CompiledShaderPipeline(Vec::new())))
        }
        pub fn _unused(_desc: &RasterPipelineDesc) {}
    }
}

/// Contract for the REUSED `shader-variants` crate.
pub mod shader_variants_contract {
    use super::rhapsody_contract::{CompiledShaderPipeline, RasterPipelineDesc};

    /// `get_raster_shader_variants` — enumerate the two 16-hex variant compile
    /// products (see `evidence/render-passes/shaders/catalog.md`).
    pub fn get_raster_shader_variants(_desc: &RasterPipelineDesc) -> Vec<CompiledShaderPipeline> {
        vec![
            CompiledShaderPipeline(Vec::new()),
            CompiledShaderPipeline(Vec::new()),
        ]
    }
}

/// Contract for game-owned `utils` helpers referenced by WP4 types.
pub mod utils_contract {
    use std::path::Path;

    use glam::Vec3;

    /// `utils::geometry::aabb::Aabb3` (game-owned; min + max).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Aabb3 {
        pub min: Vec3,
        pub max: Vec3,
    }
    impl Aabb3 {
        pub fn empty() -> Self {
            Aabb3 {
                min: Vec3::splat(f32::MAX),
                max: Vec3::splat(f32::MIN),
            }
        }
        pub fn expand_to_contain_point(&mut self, p: Vec3) {
            self.min = self.min.min(p);
            self.max = self.max.max(p);
        }
        /// O（0x141240460）：`max - min`。
        pub fn extent(&self) -> Vec3 {
            self.max - self.min
        }
        /// O（0x141240420）：`(min + max) * 0.5`。
        pub fn center(&self) -> Vec3 {
            (self.min + self.max) * 0.5
        }
    }

    /// `utils::geometry::rectangle::Rectangle2d`（O 0x140c98900，24 B）：方向 (1,0) @0、中心 @8、
    /// 宽 @0x10、长 @0x14；`new_axis_aligned` 断言 width / length 有限。
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rectangle2d {
        pub direction: glam::Vec2,
        pub center: glam::Vec2,
        pub width: f32,
        pub length: f32,
    }
    impl Rectangle2d {
        pub fn new_axis_aligned(width: f32, length: f32, center: glam::Vec2) -> Self {
            assert!(width.is_finite(), "assertion failed: width.is_finite()");
            assert!(length.is_finite(), "assertion failed: length.is_finite()");
            Rectangle2d { direction: glam::Vec2::X, center, width, length }
        }
    }

    /// `utils::geometry::intersect::Triangle`.
    #[derive(Clone, Copy, Debug)]
    pub struct Triangle(pub [Vec3; 3]);
    impl Triangle {
        pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
            Triangle([a, b, c])
        }
    }

    /// `utils::load_json::load_json_as_mesh` (O: mesh reload path).
    pub fn load_json_as_mesh(_path: &Path) -> Result<crate::render::mesh::Mesh, ()> {
        Err(())
    }
}

/// Contract for `puffin` / `ustr` (reused; puffin is an offline-cache env gap).
pub mod puffin_contract {
    pub struct Scope;
    impl Drop for Scope {
        fn drop(&mut self) {}
    }
    pub fn scope(_name: &str) -> Scope {
        Scope
    }
    // wp12 wall systems bracket their bodies in scopes.
    pub struct ProfilerScope;
    impl ProfilerScope {
        #[inline]
        pub fn new(_name: &'static str) -> Self {
            ProfilerScope
        }
    }
    #[inline]
    pub fn are_scopes_on() -> bool {
        false
    }
}
pub mod ustr_contract {
    pub fn intern(_path: &std::path::Path) -> u64 {
        0
    }
}

/// `crate::utils` — game-owned helper facade (file module; wp6 index_condenser/
/// components/float_grid + wp12 curve/geometry/swizzlers + wp15 util helpers +
/// wp14 audio_utils + wp16 residuals). See `src/utils/mod.rs`.
pub mod utils;

/// `crate::resources` — game CPU-side state (file module; wp5-16 merged content,
/// including the frame-pass typed-gap SSBO carriers `render_ssbo`). See
/// `src/resources/mod.rs`.
pub mod resources;

/// WP5 `consume::` — re-export layer wiring the render_loop systems to the real
/// assembled types (wp1-4). Replaces wp5's placeholder modules.
pub mod consume {
    /// wp1 `render::pass_builder` surface.
    pub mod pass_builder {
        pub use crate::render::pass_builder::bind_resources::BindGameResources;
        pub use crate::render::pass_builder::compute_pass_builder::{
            ComputeDispatchApi, ComputeDispatchIndirectApi, ComputePass, DispatchArgs,
        };
        pub use crate::render::pass_builder::draw_builder::{DrawBuilder, DrawCommand};
        pub use crate::render::pass_builder::draw_stream::{
            DrawPacket, DrawPassRenderCheck, RasterPass, TextureBindingKey,
        };
        pub use crate::render::pass_builder::pass_resources::PassResources;
        pub use crate::render::pass_builder::shader_uri::{
            AsComputeShaderSource, ComputeShaderSource, ShaderDefValue, ShaderUri,
        };
        pub use crate::render::pass_builder::uniform_binder::UniformBinder;
    }

    /// wp2 `render` GPU buffer primitives.
    pub mod buffers {
        pub use crate::render::indirect::{
            DispatchIndirectCommand, DrawIndexedIndirectCommand, DrawIndirectCommand,
        };
        pub use crate::render::shadowed_buffer::ShadowedBuffer;
        pub use crate::render::sparse_instance_buffer::SparseInstanceBuffer;
        pub use crate::render::ssbo::{InstanceBuffer, MappedStorageBuffer, TypedStorageBuffer};
        pub use crate::resources::render_ssbo::{
            AtomicCounterBuffer, DispatchIndirectBuffer, DrawIndexedIndirectBuffer,
            DrawIndirectBuffer,
        };
    }

    /// wp3 `render::nani` surface.
    pub mod nani {
        pub use crate::render::nani::buffers::{
            DrawListParams, NaniDrawListLengthSsbo, NaniDrawListSsbo, NaniInstanceIndirectionSsbo,
        };
        pub use crate::render::nani::cull_shaders::{
            NaniCullAndBucketBush, NaniCullAndBucketCurvedHalftimber, NaniCullAndBucketFlowers,
            NaniCullAndBucketMain, NaniCullAndBucketPlaster, NaniCullAndBucketRoofTile,
            NaniCullAndBucketSmoke, NaniCullAndBucketStairsFloor, NaniCullAndBucketWoodenPlank,
        };
        pub use crate::render::nani::gpu_world::{
            ExternalGpuWorldSubset, GpuWorld, GpuWorldSubsetDesc, GpuWorldSubsetKind,
            InstanceHandle, InstanceKey, NaniDrawListInfo, NaniInstanceData, OwnedGpuWorldSubset,
        };
        pub use crate::render::nani::instance_data::*;
        pub use crate::render::nani::mesh_atlas::{MeshAtlas, MeshAtlasPart, NaniMeshAtlasPartInfo};
    }

    /// wp4 `render` / `asset_libraries` core + render-loop typed-gap SSBOs.
    pub mod core {
        pub use crate::asset_libraries::color_library::ColorLibrarySsbo;
        pub use crate::asset_libraries::mesh_library::AssetMeshLibrary;
        pub use crate::asset_libraries::shader_library::AssetShaderLibrary;
        pub use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
        pub use crate::asset_libraries::texture_library::AssetTextureLibrary;
        pub use crate::render::atmosphere::SunSkyColors;
        pub use crate::render::directional_light::DirectionalLight;
        pub use crate::render::frame_constants::{FrameConstants, RenderViewIndex};
        pub use crate::render::mesh::Mesh;
        pub use crate::render::view_constants::ViewConstants;
        // Ray-tracing + frame-pass SSBO carriers (typed gaps / wp1 placeholders).
        pub use crate::render::pass_builder::bind_resources::{
            RtBvhSsbo, RtTriangleMetaSsbo, RtTrianglesSsbo,
        };
        pub use crate::resources::render_ssbo::{
            DofAutofocusSsbo, DofFocusReadbackSsbo, LanternSsbo, MouseDepthValueSsbo,
            ShadowMaskHistogramSsbo, ShadowMaskMeanSsbo, SpatialResolveOffsetsSsbo,
            WallColorIdSsbo,
        };
        // WP19-consumed shared types (real recoveries; kept in `consume::core`
        // so the wp19 boot/stylized modules compile against the assembled
        // surface — see INTEGRATION_FINAL.md).
        pub use crate::resources::audio_resources::AudioCore;
        pub use crate::resources::grading_lut::{GradingLuts, LutGalleryState};
        pub use crate::resources::logo_animation::LogoAnimation;
        pub use crate::resources::session::SavedSession;
        pub use crate::rhapsody_contract::Texture;
        pub use crate::utils::timer::FilteredWallTime;
    }
}
