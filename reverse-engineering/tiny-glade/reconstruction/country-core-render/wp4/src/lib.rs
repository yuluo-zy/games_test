//! WP4 — `country_core` render-core reconstruction (mesh / shader / render-config
//! / asset-library wiring), aligned to the original PDB skeleton module tree.
//!
//! Version B (patched `tiny-glade.exe` cf048e27…, PDB `tiny_glade.pdb`
//! 2d9686b8…). See `README.md` and `contracts/*.md`.
//!
//! Reused dependencies (NOT reimplemented): `bevy`, `glam`, `ash`, `parking_lot`,
//! `hashbrown`, `frozen`, `rhapsody` (render graph), `shader-pipeline` /
//! `shader-variants` / `shader-compiler` (shader toolchain), `serde`/`ron`,
//! `bytemuck`. The game-owned logic implemented here calls those via the typed
//! contract modules below.

pub mod asset_libraries;
pub mod render;

/// Typed-gap marker: a field whose layout/semantics were not recovered from the
/// original machine code. Never fabricated.
#[derive(Clone, Copy, Default, Debug)]
pub struct Gap;

/// Alias module so modules can write `crate::typed_gap::Gap`.
pub mod typed_gap {
    pub use crate::Gap;
}

/// Contract for the REUSED `rhapsody` crate surface that the render layer calls.
pub mod rhapsody_contract {
    use std::marker::PhantomData;
    use std::sync::Arc;

    use super::render::shader::{ComputeShaderProgram, RasterShaderProgram};

    /// Vulkan device handle (reused `rhapsody::raw::device::GraphicsDevice`).
    #[derive(Clone, Default)]
    pub struct GraphicsDevice(pub u64);

    impl GraphicsDevice {
        pub fn create_compute_pipeline(
            &self,
            _artifacts: Arc<CompiledShaderPipeline>,
        ) -> Result<Arc<CompiledShaderPipeline>, PipelineError> {
            Ok(_artifacts)
        }
        pub fn create_raster_pipeline(
            &self,
            artifacts: Arc<CompiledShaderPipeline>,
        ) -> Box<RasterPipeline> {
            Box::new(RasterPipeline(
                artifacts,
                ManagedHandle(self.0),
                PhantomData,
            ))
        }
        pub fn create_persistent_descriptor_set(&self, _name: &str) -> DescriptorSet {
            DescriptorSet
        }
        pub fn create_texture(
            &self,
            _desc: TextureDesc,
            _data: &[u8],
            _name: u64,
        ) -> Result<Texture, TextureError> {
            Ok(Texture::default())
        }
        pub fn update_buffer_bytes(
            &self,
            _handle: ResourceHandle,
            _bytes: &[u8],
        ) -> Result<(), TextureError> {
            Ok(())
        }
        pub fn update_persistent_descriptor_set(&self) {}
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct DescriptorSet;

    /// Compiled shader artifacts (reused `shader_pipeline::compiled`).
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CompiledShaderPipeline(pub Vec<u8>);

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ComputePipelineDesc;
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct RasterPipelineDesc {
        /// O: desc field at +0x18 (a feature/binding selector int) used in
        /// `get_or_generate` equality.
        pub selector: i32,
    }

    /// Runtime raster pipeline (0x38 bytes / Box, O): holds the compiled
    /// artifacts + a managed handle.
    pub struct RasterPipeline(
        pub Arc<CompiledShaderPipeline>,
        pub ManagedHandle,
        pub PhantomData<()>,
    );

    /// Managed GPU-object handle (reused `rhapsody::raw::managed::handle`).
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct ManagedHandle(pub u64);

    /// RG resource handle.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ResourceHandle(pub u64);
    impl ResourceHandle {
        pub const NONE: ResourceHandle = ResourceHandle(0);
    }

    /// Opaque texture (reused `rhapsody::raw::texture::Texture`, 0x40 bytes).
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct Texture {
        pub width: u32,
        pub height: u32,
        pub format: u32,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct TextureDesc {
        pub sampled: bool,
        pub mip_levels: u32,
        pub array_elements: u32,
    }
    impl TextureDesc {
        pub fn new() -> Self {
            TextureDesc::default()
        }
        pub fn sampled(mut self) -> Self {
            self.sampled = true;
            self
        }
        pub fn mip_levels(mut self, m: u32) -> Self {
            self.mip_levels = m;
            self
        }
        pub fn array_elements(mut self, a: u32) -> Self {
            self.array_elements = a;
            self
        }
    }

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

    /// `import_export` (O: `rhapsody::rg::import_export::export` /
    /// `import_unowned`).
    pub fn import_export_export(_handle: ResourceHandle) {}
    pub fn import_unowned(_data: *const u8, _size: usize) -> ResourceHandle {
        ResourceHandle(1)
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

/// Contract for `puffin` / `ustr` (reused).
pub mod puffin_contract {
    pub struct Scope;
    impl Drop for Scope {
        fn drop(&mut self) {}
    }
    pub fn scope(_name: &str) -> Scope {
        Scope
    }
}
pub mod ustr_contract {
    pub fn intern(_path: &std::path::Path) -> u64 {
        0
    }
}
