//! WP5 — `country_core::systems::render_loop` frame-pass reconstruction (B).
//!
//! Version B (patched `tiny-glade.exe` cf048e27…, PDB `tiny_glade.pdb` 2d9686b8…).
//! This is the CPU-side composition of every render_loop frame pass: which
//! `ComputePass` / `DrawBuilder` / `RasterPass` each pass constructs, which
//! `ShaderUri` it uses (cross-checked against `shader-uri-list.json` and the
//! `AsComputeShaderSource` table), which typed buffers/textures it declares
//! read/write, and its dispatch/draw mode + frame order (W1–W54, sequence.md).
//!
//! The sibling packages own the shared types; this package CONSUMES them and
//! does NOT redefine them:
//!   - wp1 `pass_builder`: ComputePass / DrawBuilder / RasterPass / DrawPacket /
//!     PassResources / ShaderUri / UniformBinder / bind_resources
//!   - wp2 `render`: TypedStorageBuffer / InstanceBuffer / ShadowedBuffer /
//!     SparseInstanceBuffer / indirect command wrappers
//!   - wp3 `render::nani`: GpuWorld / OwnedGpuWorldSubset / GpuWorldSubsetKind /
//!     MeshAtlas / NaniInstanceData / cull_shaders closure surface
//!   - wp4 `render`/`asset_libraries`: Mesh / FrameConstants / ViewConstants /
//!     DirectionalLight / Asset{Shader,Ssbo,Mesh,Texture,Color}Library /
//!     bindless_textures_dset
//!
//! `resources::` game data (water/terrain/roof/grass state) is OUT OF SCOPE —
//! referenced as typed gaps only.
//!
//! Reused dependencies (NOT reimplemented): `bevy_ecs`, `glam`, `ash`, `rhapsody`
//! (render graph), `shader-pipeline`/`shader-variants`, `enum-iterator` 1.2,
//! `itertools`, `parking_lot`, `puffin`, `bytemuck`.

// Evidence-aligned skeleton: private helpers are business-logic markers, not
// necessarily wired into the public surface yet, so dead code is expected.
#![allow(dead_code)]

pub mod typed_gap;

/// Contract modules for the sibling-owned shared types this package composes.
pub mod consume {
    /// wp1 `country_core::render::pass_builder` surface (ComputePass/DrawBuilder/
    /// RasterPass/DrawPacket/ShaderUri/UniformBinder/PassResources).
    pub mod pass_builder {
        use crate::typed_gap::Gap;

        /// 400-byte compute-pass builder (wp1).
        #[derive(Clone, Default)]
        pub struct ComputePass(pub Gap);

        /// 0xf8-byte raster draw builder (wp1).
        #[derive(Clone, Default)]
        pub struct DrawBuilder(pub Gap);

        /// Raster pass wrapper over ≤5 color attachments + optional depth (wp1).
        #[derive(Clone, Default)]
        pub struct RasterPass(pub Gap);

        /// Raster draw packet (default shader + two maps + blend 1.0) (wp1).
        #[derive(Clone, Default)]
        pub struct DrawPacket(pub Gap);

        /// 48-byte ShaderUri (whole/name/entry) (wp1).
        #[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
        pub struct ShaderUri(pub Gap);

        /// Uniform writer over a bindings table (wp1).
        #[derive(Clone, Default)]
        pub struct UniformBinder(pub Gap);

        /// 7-resource handle bundle resolved from the ECS world by hash (wp1).
        #[derive(Clone, Default)]
        pub struct PassResources(pub Gap);

        /// `AsComputeShaderSource`: compute-pass structs implement it (wp1).
        pub trait AsComputeShaderSource {
            fn source_hash(&self) -> u64;
            fn as_compute_pipeline(&self) -> ShaderUri;
        }

        /// `BindReadOnlyResources`: read_storage_buffer / optional_uniform / read_texture.
        pub trait BindReadOnlyResources {
            fn read_storage_buffer<T>(&mut self);
            fn optional_uniform<T>(&mut self, _name: &str, _value: &T);
        }

        /// `ComputeDispatchApi` / `ComputeDispatchIndirectApi` (wp1): dispatch entry modes.
        pub trait ComputeDispatchApi {
            fn push_constants(&mut self, _range: u32);
            fn dispatch_1d(&mut self, _x: u32);
            fn dispatch_2d(&mut self, _x: u32, _y: u32);
            fn dispatch_3d(&mut self, _x: u32, _y: u32, _z: u32);
            fn dispatch_groups(&mut self, _groups: u32);
        }
        pub trait ComputeDispatchIndirectApi {
            fn push_constants(&mut self, _range: u32);
            fn dispatch(&mut self, _indirect: &Gap);
        }

        /// ComputePass dispatch entry modes (O: all dispatch_* funnel to
        /// `PassBuilder::render<PassResources, dispatch_impl::closure_env_0>`;
        /// modes 2/3/4; see pass-declaration.md §1.1/§5).
        impl ComputeDispatchApi for ComputePass {
            fn push_constants(&mut self, _range: u32) { unimplemented!() }
            fn dispatch_1d(&mut self, _x: u32) { unimplemented!() }
            fn dispatch_2d(&mut self, _x: u32, _y: u32) { unimplemented!() }
            fn dispatch_3d(&mut self, _x: u32, _y: u32, _z: u32) { unimplemented!() }
            fn dispatch_groups(&mut self, _groups: u32) { unimplemented!() }
        }
    }

    /// wp2 `country_core::render` GPU buffer primitives.
    pub mod buffers {
        use crate::typed_gap::Gap;

        /// 40-byte typed storage buffer (byte_size at +0x18 load-bearing) (wp2).
        pub struct TypedStorageBuffer<T>(pub Gap, pub std::marker::PhantomData<T>);
        /// GPU-instance buffer over typed instances (wp2).
        pub struct InstanceBuffer<T>(pub Gap, pub std::marker::PhantomData<T>);
        /// Host-mapped storage buffer (wp2).
        pub struct MappedStorageBuffer<T>(pub Gap, pub std::marker::PhantomData<T>);
        /// 9-qword double-buffer with inclusive dirty range (empty=(MAX,0)) (wp2).
        pub struct ShadowedBuffer<T>(pub Gap, pub std::marker::PhantomData<T>);

        impl<T> InstanceBuffer<T> {
            pub fn instance_count(&self) -> u32 { unimplemented!() }
            pub fn gpu_write(&self, _indices: &[u32], _data: &[T]) { unimplemented!() }
        }
        impl<T> ShadowedBuffer<T> {
            pub fn buffer(&self) -> &Gap { unimplemented!() }
        }

        /// Indirect command wrappers (Vulkan ABI) (wp2).
        pub struct DrawIndexedIndirectCommand(pub u32, pub u32, pub u32, pub u32, pub u32);
        pub struct DrawIndirectCommand(pub u32, pub u32, pub u32, pub u32);
        pub struct DispatchIndirectCommand(pub u32, pub u32, pub u32);

        pub struct DrawIndexedIndirectBuffer(pub Gap);
        pub struct DrawIndirectBuffer(pub Gap);
        pub struct DispatchIndirectBuffer(pub Gap);
        pub struct AtomicCounterBuffer(pub Gap);
    }

    /// wp3 `country_core::render::nani` GPU-driven renderer.
    pub mod nani {
        use crate::typed_gap::Gap;
        use super::pass_builder::AsComputeShaderSource;

        /// 27-slot per-kind world subset store (wp3).
        pub struct GpuWorld(pub Gap);
        pub struct OwnedGpuWorldSubset(pub Gap);
        pub struct ExternalGpuWorldSubset(pub Gap);
        /// GpuWorldSubsetKind — the 27 subset kinds, iterated via enum_iterator.
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum GpuWorldSubsetKind { Kind0, /* …27 kinds (names U) */ }

        impl GpuWorldSubsetKind {
            /// Iterate all kinds (enum-iterator 1.2; O in gpu_generate_draw_lists).
            pub fn all() -> impl Iterator<Item = GpuWorldSubsetKind> {
                [GpuWorldSubsetKind::Kind0].into_iter()
            }
            pub fn draw_list_index(self) -> u32 { unimplemented!() }
            pub fn use_fine_grained_sorting(self) -> bool { unimplemented!() }
            pub fn gpu_world_subset_desc(self) -> Gap { unimplemented!() }
        }

        impl GpuWorld {
            pub fn mesh_atlas_info_buf(&self) -> &Gap { unimplemented!() }
            pub fn draw_list_info_buf(&self) -> &Gap { unimplemented!() }
            pub fn try_get_subset_mut(&self, _kind: GpuWorldSubsetKind) -> Option<&mut OwnedGpuWorldSubset> {
                unimplemented!()
            }
        }

        impl OwnedGpuWorldSubset {
            pub fn instance_count(&self) -> u32 { unimplemented!() }
        }

        /// cull_shaders closure surface (wp3): per-kind AsComputeShaderSource impls.
        pub struct NaniCullAndBucketMain;
        pub struct NaniCullAndBucketBush;
        pub struct NaniCullAndBucketCurvedHalftimber;
        pub struct NaniCullAndBucketFlowers;
        pub struct NaniCullAndBucketPlaster;
        pub struct NaniCullAndBucketRoofTile;
        pub struct NaniCullAndBucketSmoke;
        pub struct NaniCullAndBucketStairsFloor;
        pub struct NaniCullAndBucketWoodenPlank;
        impl AsComputeShaderSource for NaniCullAndBucketMain {
            fn source_hash(&self) -> u64 { unimplemented!() }
            fn as_compute_pipeline(&self) -> super::pass_builder::ShaderUri { unimplemented!() }
        }
        // … other kinds follow the same pattern (wp3 cull_shaders contract).

        /// Nani draw-list SSBOs (wp3): consumed by draw_ext::nani_draw + cull_and_bucket.
        pub struct NaniDrawListSsbo(pub Gap);
        pub struct NaniDrawListLengthSsbo(pub Gap);
        pub struct NaniInstanceIndirectionSsbo(pub Gap);
        pub struct NaniNextVisibilityBitmaskSsbo(pub Gap);
        pub struct NaniMeshAtlasPartInfo(pub Gap);
        pub struct NaniDrawListInfo(pub Gap);
        pub struct NaniDrawListParams(pub Gap);

        impl NaniDrawListParams {
            pub fn list_draw_merge_window_range(&self) -> (u32, u32) { unimplemented!() }
            pub fn list_draw_merge_window_subtotal_range(&self) -> (u32, u32) { unimplemented!() }
        }
    }

    /// wp4 `country_core::render` / `asset_libraries` core.
    pub mod core {
        use crate::typed_gap::Gap;

        pub struct Mesh(pub Gap);
        pub struct ViewConstants(pub Gap);
        pub struct FrameConstants(pub Gap);
        pub struct DirectionalLight(pub Gap);
        pub struct SunSkyColors(pub Gap);

        pub struct ColorLibrarySsbo(pub Gap);
        pub struct LanternSsbo(pub Gap);
        pub struct SpatialResolveOffsetsSsbo(pub Gap);
        pub struct MouseDepthValueSsbo(pub Gap);
        pub struct DofAutofocusSsbo(pub Gap);
        pub struct DofFocusReadbackSsbo(pub Gap);
        pub struct ShadowMaskHistogramSsbo(pub Gap);
        pub struct ShadowMaskMeanSsbo(pub Gap);

        /// Software-BVH ray-tracing SSBOs (wp4 bind_ray_tracing surface).
        pub struct RtTrianglesSsbo(pub Gap);
        pub struct RtTriangleMetaSsbo(pub Gap);
        pub struct RtBvhSsbo(pub Gap);
        pub struct WallColorIdSsbo(pub Gap);

        pub struct AssetShaderLibrary(pub Gap);
        pub struct AssetSsboLibrary(pub Gap);
        pub struct AssetMeshLibrary(pub Gap);
        pub struct AssetTextureLibrary(pub Gap);

        pub trait BindGameResources {
            fn bind_shadowmap(&mut self);
            fn bind_terrain_heightmap(&mut self);
            fn bind_terrain_lighting(&mut self);
            fn bind_ray_tracing(&mut self);
            fn bind_color_library(&mut self);
        }
    }
}

/// `resources::` game state — OUT OF SCOPE typed gaps (never fabricated).
pub mod resources {
    use crate::typed_gap::Gap;
    /// Water/terrain/roof/grass CPU-side state referenced by the frame passes.
    pub struct Water(pub Gap);
    pub struct Terrain(pub Gap);
    pub struct RoofTiles(pub Gap);
    pub struct Grass(pub Gap);
}

pub mod systems;
