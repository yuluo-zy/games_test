
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
        pub fn issue_full_pipeline_barrier(&self, _command: u64) {}
        /// `dispatch` — vkCmdDispatch groups (O call site).
        pub fn dispatch(&self, _command: u64, _x: u32, _y: u32, _z: u32) {}
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

    /// Texture view descriptor (reused `rhapsody::raw::texture::TextureViewDesc`).
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
