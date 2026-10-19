//! rhapsody 设备的游戏侧包装及兼容类型。
//! 纹理资产需要由 TextureHost 解析托管身份；描述符表使用 rhapsody 的真实资源租约。
//! 其他尚未完成的 ABI 兼容边界分别在对应接口中注明。

use std::marker::PhantomData;
use std::sync::Arc;

use super::render::shader::{ComputeShaderProgram, RasterShaderProgram};

/// 全 crate 唯一的 rhapsody 设备包装（WIRING §1）。
/// 原 `GraphicsDevice(pub u64)` → `Arc<rhapsody::device::Device>`。
#[derive(Clone)]
pub struct GraphicsDevice {
    inner: Arc<rhapsody::device::Device>,
    // Same host that owns the App's library buffers; never a second registry.
    buffer_host:
        Option<Arc<dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice + Send + Sync>>,
}

impl GraphicsDevice {
    /// 从真实 rhapsody 设备构造本包装。
    pub fn new(inner: Arc<rhapsody::device::Device>) -> Self {
        Self {
            inner,
            buffer_host: None,
        }
    }
    pub fn with_buffer_host(
        mut self,
        host: Arc<dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice + Send + Sync>,
    ) -> Self {
        self.buffer_host = Some(host);
        self
    }
    pub(crate) fn buffer_host(
        &self,
    ) -> Option<&(dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice + Send + Sync)> {
        self.buffer_host.as_deref()
    }
    /// 底层 rhapsody 设备。
    pub fn inner(&self) -> &Arc<rhapsody::device::Device> {
        &self.inner
    }

    // ---------------------------------------------------------------- 真实调用（WIRING §2.1）

    /// Full execution and memory dependency on a recording command.
    ///
    /// # Safety
    /// `command` must be recording on this device.
    pub unsafe fn issue_full_pipeline_barrier(
        &self,
        command: ash::vk::CommandBuffer,
    ) -> rhapsody::Result<()> {
        unsafe { self.inner.issue_full_pipeline_barrier(command) }
    }

    /// Record one barrier batch.
    ///
    /// # Safety
    /// `command` must be recording on this device; every barrier resource
    /// must stay alive through the command's completion.
    pub unsafe fn issue_pipeline_barrier(
        &self,
        command: ash::vk::CommandBuffer,
        scope: &rhapsody::raw::barrier::SynchronizationScope,
    ) -> rhapsody::Result<()> {
        unsafe { self.inner.issue_pipeline_barrier(command, scope) }
    }

    /// Record one barrier batch, then clear it for reuse.
    ///
    /// # Safety
    /// As [`Self::issue_pipeline_barrier`].
    pub unsafe fn issue_and_reset_pipeline_barrier(
        &self,
        command: ash::vk::CommandBuffer,
        scope: &mut rhapsody::raw::barrier::SynchronizationScope,
    ) -> rhapsody::Result<()> {
        unsafe { self.inner.issue_and_reset_pipeline_barrier(command, scope) }
    }

    /// 真实 `GraphicsDevice::wait_idle_and_recycle_resources()`（device.rs:699；WIRING §2.7）。
    pub fn wait_idle_and_recycle_resources(&self) -> rhapsody::Result<()> {
        self.inner.wait_idle_and_recycle_resources()
    }

    /// 真实 `GraphicsDevice::wait_idle()`（device.rs:680）。
    pub fn wait_idle(&self) -> rhapsody::Result<()> {
        self.inner.wait_idle()
    }

    /// 真实 `GraphicsDevice::create_command_buffer(family, name)`（device.rs:719）。
    pub fn create_command_buffer(
        &self,
        family: u32,
        name: &str,
    ) -> rhapsody::Result<rhapsody::raw::command_buffer::VulkanCommandBuffer> {
        self.inner.create_command_buffer(family, name)
    }

    /// 真实 `GraphicsDevice::get_built_in_sampler_by_name(name)`（device.rs:830）。
    pub fn get_built_in_sampler_by_name(
        &self,
        name: &str,
    ) -> rhapsody::Result<Arc<rhapsody::raw::sampler::Sampler>> {
        self.inner.get_built_in_sampler_by_name(name)
    }

    /// 真实 `GraphicsDevice::max_bindless_descriptor_count()`（device.rs:669）。
    pub fn max_bindless_descriptor_count(&self) -> u32 {
        self.inner.max_bindless_descriptor_count()
    }

    /// 真实 `GraphicsDevice::refresh_memory_budget()`（device.rs:611）。
    pub fn refresh_memory_budget(
        &self,
    ) -> rhapsody::raw::error::Result<
        Option<ash::vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static>>,
    > {
        self.inner.refresh_memory_budget()
    }

    // ---- raw::device_api 适配层（device_api.rs，WIRING §2.1/§2.3）----

    /// 真实 `create_buffer<D>`（device_api.rs:71）→ `ManagedBuffer`。
    pub fn create_buffer<D: rhapsody::raw::device_api::BufferInitialData>(
        &self,
        desc: &rhapsody::raw::buffer::BufferDesc,
        name: &str,
        data: D,
    ) -> rhapsody::raw::error::Result<rhapsody::raw::managed::buffer::ManagedBuffer> {
        self.inner.create_buffer(desc, name, data)
    }

    /// 真实 `create_texture<D>`（device_api.rs:151）→ `ManagedTexture`。
    pub fn create_texture<D: rhapsody::raw::device_api::TextureInitialBytes>(
        &self,
        desc: &rhapsody::raw::texture::TextureDesc,
        name: &str,
        data: D,
    ) -> rhapsody::raw::error::Result<rhapsody::raw::managed::texture::ManagedTexture> {
        self.inner.create_texture(desc, name, data)
    }

    /// 真实 `get_vk_image_view(texture, desc)`（device_api.rs:331）。
    pub fn get_vk_image_view(
        &self,
        texture: &rhapsody::raw::managed::texture::ManagedTexture,
        desc: &rhapsody::raw::texture::TextureViewDesc,
    ) -> rhapsody::raw::error::Result<rhapsody::raw::managed::texture::ManagedTextureView> {
        self.inner.get_vk_image_view(texture, desc)
    }

    /// 真实 `get_vk_buffer(buffer)`（device_api.rs:325）。
    pub fn get_vk_buffer(
        &self,
        buffer: &rhapsody::raw::managed::buffer::ManagedBuffer,
    ) -> rhapsody::raw::error::Result<ash::vk::Buffer> {
        self.inner.get_vk_buffer(buffer)
    }

    /// 真实 `update_buffer_bytes(buffer, data, ranges, offset)`（device_api.rs:348）。
    pub fn update_buffer_bytes(
        &self,
        buffer: &rhapsody::raw::managed::buffer::ManagedBuffer,
        data: &[u8],
        ranges: &[std::ops::Range<usize>],
        offset: usize,
    ) -> rhapsody::raw::error::Result<()> {
        self.inner.update_buffer_bytes(buffer, data, ranges, offset)
    }

    /// 真实 `buffer_mapped_slice_mut`（device_api.rs:137）。
    /// # Safety
    /// Callers must exclude GPU/host aliases and flush non-coherent writes.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn buffer_mapped_slice_mut<'a>(
        &self,
        buffer: &'a rhapsody::raw::managed::buffer::ManagedBuffer,
    ) -> Option<&'a mut [u8]> {
        unsafe { self.inner.buffer_mapped_slice_mut(buffer) }
    }

    /// 真实 `buffer_device_address`（device_api.rs:144）。
    pub fn buffer_device_address(
        &self,
        buffer: &rhapsody::raw::managed::buffer::ManagedBuffer,
    ) -> rhapsody::raw::error::Result<ash::vk::DeviceAddress> {
        self.inner.buffer_device_address(buffer)
    }

    // ---------------------------------------------------------------- 登记缺口（WIRING §3.1 / §4）

    /// **登记缺口 §4.3**：`[device+0xaba]` 全屏障开关无公开 getter（rhapsody 结构注释已记录布局）。
    /// 桥内返回 `false`（WIRING §3.1 #3），不得当作真实完成。
    pub fn full_barrier_required(&self) -> bool {
        log::warn!(
            target: "country_core::rhapsody_contract",
            "full_barrier_required: registered gap (WIRING §4.3), full-barrier switch has no public getter; returning false"
        );
        false
    }
}

/// 资产库持有真实描述符堆与资源租约；兼容旧模块导入路径。
pub use crate::asset_libraries::descriptor_table::DescriptorSet;

/// 设备/着色器/纹理占位类型：全 crate 唯一一份，见 `crate::rhapsody_types`。
pub use crate::render::pass_builder::shader_uri::ComputePipelineDesc;
pub use crate::rhapsody_types::{
    CompiledShaderPipeline, ComputePipeline, RasterPipeline, RasterPipelineDesc, RasterPipelineKey,
    Texture, TextureDesc,
};

/// Game-side logical object identity; actual GPU ownership uses `raw::managed::handle::Managed<T>` /
/// `Managed<T>`）。游戏侧以 u64 槽位为 ABI 桥；`gpu_generate_draw_lists` 的 `ExternalBufferHandle`
/// 等消费点经此编译。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ManagedHandle(pub u64);

/// Managed handle wrapper for a typed GPU object
/// (physical resource keys use `rhapsody::resource::Handle<T>`; game identity retained for
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
    /// 真实语义 = `Managed::resource()` / `ManagedRef::get_untracked`（WIRING §1）。
    /// 游戏侧对象槽未恢复（§4.8），保留桥。
    pub fn get(&self) -> &T {
        panic!("Managed handle backend is not connected")
    }
}

/// Temporal read-only handle (reused `rhapsody::rg::temporal::ReadOnlyHandle<T>`).
/// WIRING §4.5：原版无 ReadOnlyHandle；真实跨帧只读面 = `rg::temporal::TemporalLookup`/`TemporalResource`。
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
        panic!("Temporal handle backend is not connected")
    }
}

/// RG resource handle（游戏侧占位；WIRING §1：真实 = `rg::resource::ResourceHandle(u32)`，
/// `raw()` 取图内逻辑索引，值域 u64 → u32）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResourceHandle(pub u64);
impl ResourceHandle {
    pub const NONE: ResourceHandle = ResourceHandle(0);
}

/// Texture view descriptor (reused `rhapsody::raw::texture::TextureViewDesc`).
pub use crate::rhapsody_types::TextureViewDesc;

/// 2D extent（WIRING §1：真实 = `ash::vk::Extent2D`，同形；保留游戏侧结构体，字段语义一致）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

/// Render pass descriptor used as a dedupe key
/// (reused `rhapsody::rg::attachments::contract::RenderPassDesc`).
/// WIRING §4.16：真实 `RenderPassDesc` 是附件表键（0x98B），`{selector:u32}` 是臆造；真正的
/// “渲染通道句柄 + 选项 u32” 是 `RasterPipelineKey`。此占位不再作真实键，保留给游戏侧编译。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderPassDesc {
    pub selector: u32,
}

/// Game-side pass marker; physical attachments are prepared by the dynamic renderer.
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

/// Pass resource slot（游戏侧占位；WIRING §1：真实 = `rg::binding::PassResourceSlot`，resource 为
/// `ResourceHandle`，缓冲槽带 offset/size、图像槽带 layout）。
#[derive(Clone, Copy, Debug)]
pub enum PassResourceSlot {
    StorageBuffer { resource: u32 },
    UniformBuffer { resource: u32 },
    SampledImage { resource: u32 },
    StorageImage { resource: u32 },
}

/// A recorded pass binding（游戏侧占位；WIRING §1：真实 = `rg::pass_bindings::PassBinding`，
/// 经 `PassBinding::{buffer,image}` 构造）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderPassBinding {
    pub resource_index: u32,
    pub access_type: u8,
    /// 视图描述。原 `RenderPassBinding`（0x44 B）把整块 0x3c B `TextureViewDesc` 放在最前（`Ref::bind_view`），
    /// 此前占位类型丢了它，宿主无法知道光栅 pass 的 RG 纹理该绑哪个视图。`None` = 缓冲或默认视图。
    pub view: Option<crate::rhapsody_types::TextureViewDesc>,
}

/// `rhapsody::rg::pass_api::bind` — record one resource access binding.
/// WIRING §4.6：真实没有 `rg::pass_api::bind` 自由函数；真实构造 = `PassBinding::{buffer,image}` +
/// `PassBuilder::add_resource_access`。此函数保留为游戏侧声明的占位。
pub fn bind(resource_index: u32, access_type: u8) -> RenderPassBinding {
    RenderPassBinding {
        resource_index,
        access_type,
        view: None,
    }
}

/// `rhapsody::rg::pass_builder::PassBuilder<UD>` — pass-declaration builder.
/// 游戏侧占位；真实构图走 `rg::pass_builder::PassBuilder`（`add_resource_access(ResourceHandle, u8, u32)`
/// / `render_with_context`），见 `render_frame.rs` 的 ExecutionChain 接线与 WIRING §2.4/§3.3。
#[derive(Debug)]
pub struct PassBuilder<'a, UD> {
    _marker: PhantomData<&'a mut UD>,
}
impl<'a, UD> PassBuilder<'a, UD> {
    /// `PassBuilder::add_resource_access`（O: resource_index + access_code + group）。
    /// WIRING §4.7：真实 `resolve_resource` 不存在；哈希→(set,resource) typed gap 待取证。
    pub fn add_resource_access(
        &mut self,
        _resource_index: u32,
        _access_type: u8,
        _sync_group: u32,
    ) {
        panic!("Detached pass builder cannot register accesses; supply RecordingPassBuilder")
    }
    /// `PassBuilder::resolve_resource` — hash → (set_index, resource_index) (typed gap).
    pub fn resolve_resource(_hash: u64) -> (u32, u32) {
        panic!("Detached pass builder cannot resolve resources")
    }
    /// `PassBuilder::render<PassResources, closure>` (O path; wired in integration).
    pub fn render(self, _callback: impl FnOnce(u64) -> bool + 'static) {
        panic!("Detached pass builder cannot execute callbacks")
    }
}

// B-phase append (a4_compute): a builder not attached to a recorder — used by
// `ComputePass`'s `BindGameResources::as_pass_builder` view; the live accesses are
// recorded in `pass_resource_access::RecordingPassBuilder`.
impl<'a, UD> PassBuilder<'a, UD> {
    pub fn detached() -> Self {
        PassBuilder {
            _marker: PhantomData,
        }
    }
}

/// Render-graph handle（游戏侧占位；WIRING §1/§3.4：真实 = `rg::graph::RenderGraph`，
/// `import_unowned/import_owned` 返回 `ResourceHandle`）。
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
impl<T: 'static> Default for StorageBufferDescriptorImpl<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: 'static> StorageBufferDescriptorImpl<T> {
    pub fn new() -> Self {
        StorageBufferDescriptorImpl {
            _marker: PhantomData,
        }
    }
    pub fn data_ptr(_data: &T) -> *mut u8 {
        _data as *const T as *mut u8
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
