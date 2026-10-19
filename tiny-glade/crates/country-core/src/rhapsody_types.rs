//! 外部库 `rhapsody::raw::{texture,shader}` / `shader_pipeline` 在本 crate 里的**唯一**占位类型。
//!
//! 此前 `lib.rs::rhapsody_contract`、`render/shader.rs::boundary`、`render/load_texture.rs::boundary`
//! 各写了一套互不兼容的 `Texture` / `TextureDesc` / `CompiledShaderPipeline` / `RasterPipelineDesc` /
//! `RasterPipeline`；现在三处都从这里再导出。字段以 L0 机器码核对过的版本为准
//! （`load_texture` 的文件尾描述、`shader` 的缓存键与布局）；旧占位里的 `width/height/format`
//! 保留为 U 字段，供 `texture_library` 等上层继续使用。
//! 本文件不依赖 crate 其余部分，测试台可原样挂载。

use std::sync::Arc;

pub type Error = anyhow::Error;

// ---------------------------------------------------------------- texture

/// `rhapsody::resource::TextureDesc`。`TextureDesc::new(extent, kind, format)`（0x140bd5270：`r8b → +0x1e`、
/// `r9d → +8`）；纹理文件尾 −3 字节是 `kind`（0 1D … 2 2D、3 2D 数组、4 3D、5 立方体、6 立方体数组），−4 字节是 `format`（原始 VkFormat，零扩展）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TextureDesc {
    /// Native TextureDesc+4 image-create flags; independent of usage bits.
    pub flags: u32,
    pub extent: [u32; 3],
    /// `TextureDesc+0x1e`：图像种类枚举（O）。
    pub kind: u8,
    pub sampled: bool,
    pub mip_levels: u8,
    pub array_elements: u16,
    /// B 阶段追加（i_api）：`TextureDesc::storage`（O 0x140bd5390：`usage |= 8`）。
    pub storage: bool,
    /// `TextureDesc::color_attachment`（O 0x140bd53d0：`usage |= 0x10`）。
    pub color_attachment: bool,
    /// `TextureDesc::depth_stencil_attachment`（O 0x140bd53f0：`usage |= 0x20`）。
    pub depth_stencil_attachment: bool,
    /// B 阶段追加（i_api）：像素格式（O：`TextureDesc::new_2d(extent, format)` 0x140bd52a0 的第二参数；
    /// `format(f)` 0x140bd5430 覆写）。`Default` 为 0。
    pub format: u32,
}

impl TextureDesc {
    pub fn new(extent: [u32; 3], kind: u8, format: u8) -> Self {
        Self {
            extent,
            kind,
            format: u32::from(format),
            ..Self::default()
        }
    }
    pub fn sampled(mut self) -> Self {
        self.sampled = true;
        self
    }
    /// Native 0x140bd5410 ORs usage4 and image-create flags0x108.
    /// Mutable format and extended usage must reach the actual
    /// image producer before the ice radiance0x62 -> sampled0x7b view is made.
    pub fn sampled_with_different_format(mut self) -> Self {
        self.sampled = true;
        self.flags |= 0x108;
        self
    }
    pub fn mip_levels(mut self, n: u8) -> Self {
        self.mip_levels = n;
        self
    }
    pub fn array_elements(mut self, n: u16) -> Self {
        self.array_elements = n;
        self
    }
    /// B 阶段追加（i_api）：`TextureDesc::storage`（O 0x140bd5390，`usage |= 8`）。
    pub fn storage(mut self) -> Self {
        self.storage = true;
        self
    }
    /// `TextureDesc::color_attachment`（O 0x140bd53d0，`usage |= 0x10`）。
    pub fn color_attachment(mut self) -> Self {
        self.color_attachment = true;
        self
    }
    /// `TextureDesc::depth_stencil_attachment`（O 0x140bd53f0，`usage |= 0x20`）。
    pub fn depth_stencil_attachment(mut self) -> Self {
        self.depth_stencil_attachment = true;
        self
    }
    /// `TextureDesc::new_2d_array(extent, layers, format)`（O 0x140bd52e0）：深度 1、层数 = `layers`、mip 1。
    /// 与 `new_2d` 的差别还有 `+0x1e` 维度码（2D=2、2D 数组=3、3D=4，O），本结构体不建模该字节。
    pub fn new_2d_array(extent: [u32; 2], layers: u32, format: u32) -> Self {
        Self {
            extent: [extent[0], extent[1], 1],
            kind: 3,
            array_elements: layers as u16,
            format,
            ..Self::default()
        }
    }
    /// B 阶段追加（i_api）：`TextureDesc::new_2d(extent, format)`（O 0x140bd52a0）：深度 1、层 1。
    pub fn new_2d(extent: [u32; 2], format: u32) -> Self {
        Self {
            extent: [extent[0], extent[1], 1],
            kind: 2,
            format,
            ..Self::default()
        }
    }
    /// `TextureDesc::new_3d`（O 0x140bd5310，与 `new_2d` 逐字节对照）：extent 三维原样、`format` 为第三参。
    /// 与 `new_2d` 的唯一差别是 `+0x1e` 维度码（2D=2，3D=4，O）；本结构体没有该字段（U，未建模，避免波及
    /// 其它不带 `..Default` 的字面量），调用方只能靠 `extent[2]` 区分。
    pub fn new_3d(extent: [u32; 3], format: u32) -> Self {
        Self {
            extent,
            kind: 4,
            format,
            ..Self::default()
        }
    }
    /// B 阶段追加（i_api）：`TextureDesc::format`（O 0x140bd5430）。
    pub fn with_format(mut self, format: u32) -> Self {
        self.format = format;
        self
    }
    /// B 阶段追加（i_api）：`TextureDesc::extent_2d`（O 0x140bd5790）：`(width, height)`。
    pub fn extent_2d(&self) -> [u32; 2] {
        [self.extent[0], self.extent[1]]
    }
}

/// `rhapsody::resource::TextureViewDesc`（O：0x3c B；B 阶段追加，i_api）。
///
/// 布局（逐个构造器反汇编，0x140bd57a0..0x140bd59f0）：
/// `+0x00 view_type: Option<u32>`、`+0x08 usage`、`+0x10 format`、`+0x18 aspect_mask`、`+0x20 level_count`、
/// `+0x28 layer_count`（均 `Option<u32>`：dword 标签 + dword 值）、`+0x30 base_mip_level: u32`、
/// `+0x34 base_layer: u32`、`+0x38 bind_individual_mips: bool`。`new()` 全零（`Option` 全 None）。
/// `level_count`/`layer_count` 的参数是 `Option<u32>`（`r8d` 标签、`r9d` 值）；其余 `Option` 字段的构造器取 `u32`。
/// `Ref::bind_view`（0x140bd6c00 读 / 0x140bd6c30 写）把整个 0x3c B 描述原样放进 `RenderPassBinding`，
/// 其后 `+0x3c` 资源号、`+0x40` 写标志。`calculate_hash<TextureViewDesc>`（0x14145baf0）为 derive(Hash)。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextureViewDesc {
    pub view_type: Option<u32>,
    pub usage: Option<u32>,
    pub format: Option<u32>,
    pub aspect_mask: Option<u32>,
    pub level_count: Option<u32>,
    pub layer_count: Option<u32>,
    pub base_mip_level: u32,
    pub base_layer: u32,
    pub bind_individual_mips: bool,
}

impl TextureViewDesc {
    /// 0x140bd57a0：全零。
    pub fn new() -> Self {
        Self::default()
    }
    /// 0x140bd57e0。
    pub fn view_type(mut self, v: u32) -> Self {
        self.view_type = Some(v);
        self
    }
    /// 0x140bd5820。
    pub fn usage(mut self, v: u32) -> Self {
        self.usage = Some(v);
        self
    }
    /// 0x140bd5860。
    pub fn format(mut self, v: u32) -> Self {
        self.format = Some(v);
        self
    }
    /// 0x140bd58a0（`ash::vk::ImageAspectFlags`：COLOR 1 / DEPTH 2 / STENCIL 4）。
    pub fn aspect_mask(mut self, v: u32) -> Self {
        self.aspect_mask = Some(v);
        self
    }
    /// 0x140bd58e0。
    pub fn base_mip_level(mut self, v: u32) -> Self {
        self.base_mip_level = v;
        self
    }
    /// 0x140bd5960。
    pub fn base_layer(mut self, v: u32) -> Self {
        self.base_layer = v;
        self
    }
    /// 0x140bd5920：参数是 `Option<u32>`。
    pub fn level_count(mut self, v: Option<u32>) -> Self {
        self.level_count = v;
        self
    }
    /// 0x140bd59a0：参数是 `Option<u32>`。
    pub fn layer_count(mut self, v: Option<u32>) -> Self {
        self.layer_count = v;
        self
    }
    /// 0x140bd59f0。
    pub fn bind_individual_mips(mut self, v: bool) -> Self {
        self.bind_individual_mips = v;
        self
    }
}

/// `rhapsody::resource::Texture`（O：0x38/0x40 B）。`width/height/format` 为旧占位字段，U。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Texture {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub format: u32,
}

impl Texture {
    /// `to_texture_binding_key` 调用点读取的句柄字（U，暂返回 id 低 32 位）。
    pub fn handle(&self) -> u32 {
        self.id as u32
    }
}

pub use rhapsody::Error as BackendError;

// ---------------------------------------------------------------- shader

/// `shader_pipeline::compiled::CompiledShaderPipeline`（O：0x130 B 载荷，Arc 块 0x150 B）。
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CompiledShaderPipeline(pub Vec<u8>);

/// `shader_pipeline::pipeline_desc::RasterPipelineDesc`（0x48 B）。字段 U，仅保留可比较的标识。
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct RasterPipelineDesc {
    pub path: String,
    pub entry: String,
    pub defines: Vec<(String, String)>,
}

/// `rhapsody::pipeline::RasterPipelineDesc`（0x20 B）：缓存键。+0 为托管句柄，+0x18 为 u32；
/// 二者相等才算同一键（O：查表比较处的两步判断）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterPipelineKey {
    pub handle: u64,
    pub selector: u32,
}

/// `rhapsody::pipeline::RasterPipeline`（O：`Box` 0x38 B）。内容 U。
#[derive(Debug)]
pub struct RasterPipeline {
    pub id: u64,
    pub compiled: Arc<CompiledShaderPipeline>,
}

/// `rhapsody` 计算管线（O：0x48 B 载荷）。内容 U。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputePipeline {
    pub id: u64,
}

// ---------------------------------------------------------------- render graph

/// `rhapsody::rg` 导入缓冲后的句柄（O：`import_unowned` 返回 20 B，存进 `BufferState`
/// 的 `+0x28`）。
/// **WIRING §4.13 证伪**：旧注释的「`+0xe=5` 未导入判别」不成立——`5` 是 `rg::graph` 后备资源
/// `RegistryResource::Empty` 的 tag（`payload_projection`，graph.rs:749），与 ImportedBuffer 无关。
/// 真实重构（WIRING §3.4）：`rg::graph::RenderGraph::import_unowned(name, descriptor, token, access)
/// -> Result<ResourceHandle>`（rg/import_export.rs:7），token = 物理托管句柄；游戏侧 40B `Buffer`
/// 槽表未恢复（登记缺口 §4.8），此处保留占位作编译桥。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImportedBuffer {
    pub resource: u64,
    pub generation: u64,
    pub access: u32,
}

/// Minimal game-facing resource graph calls. Runtime graph ownership and actual
/// handles remain in rhapsody and are supplied by the integration owner.
pub trait BufferGraphBackend {
    fn import_unowned(&mut self, buffer: u64, access: u8) -> ImportedBuffer;
    fn export(&mut self, imported: ImportedBuffer, access: u8);
}

/// Resource-graph adapter used by recovered game buffer import/export rules.
/// Production construction requires a backend; the default recorder exists
/// only for CPU contract tests.
#[derive(Default)]
pub struct RenderGraph {
    backend: Option<Box<dyn BufferGraphBackend>>,
    #[cfg(test)]
    pub imports: Vec<(u64, u8)>,
    #[cfg(test)]
    pub exports: Vec<(ImportedBuffer, u8)>,
}

impl core::fmt::Debug for RenderGraph {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RenderGraph")
            .field("backend_connected", &self.backend.is_some())
            .finish()
    }
}

impl RenderGraph {
    pub fn with_backend(backend: impl BufferGraphBackend + 'static) -> Self {
        Self {
            backend: Some(Box::new(backend)),
            #[cfg(test)]
            imports: Vec::new(),
            #[cfg(test)]
            exports: Vec::new(),
        }
    }

    /// `import_export::impl$1::import_unowned(buffer, rg, access)`（O 0x140bd4fc0）。
    pub fn import_unowned(&mut self, buffer: u64, access: u8) -> ImportedBuffer {
        if let Some(backend) = &mut self.backend {
            return backend.import_unowned(buffer, access);
        }
        #[cfg(test)]
        {
            self.imports.push((buffer, access));
            ImportedBuffer {
                resource: buffer,
                generation: self.imports.len() as u64,
                access: access as u32,
            }
        }
        #[cfg(not(test))]
        panic!("A resource graph backend is required")
    }

    /// `import_export::impl$1::export(handle, rg, access)`（O 0x140bd5120）。
    pub fn export(&mut self, imported: ImportedBuffer, access: u8) {
        if let Some(backend) = &mut self.backend {
            backend.export(imported, access);
            return;
        }
        #[cfg(test)]
        self.exports.push((imported, access));
        #[cfg(not(test))]
        panic!("A resource graph backend is required")
    }
}

// ---------------------------------------------------------------- rhapsody 真实面（D 组接入，WIRING §3.2）

// 本地占位类型 → rhapsody 真实类型的转发层。游戏侧占位类型保留在上一节（作为编译桥，
// 逐项映射见 `D-rhapsody/rhapsody-WIRING.md` §1）；这里按原名转发 rhapsody 真实公开类型，
// 供真实接线点（`rhapsody_contract::GraphicsDevice` 方法、`render/shader.rs`、`render_frame.rs`
// ExecutionChain 等）使用。**绝不修改 `crates/rhapsody`**；rhapsody 缺接口 → 登记缺口。

/// `raw::barrier::SynchronizationScope`。
pub use rhapsody::command::SynchronizationScope as RhapsodySynchronizationScope;
/// 统一设备本体，资源、队列与时间线共用同一设备归属。
pub use rhapsody::device::Device as RhapsodyDevice;
/// `raw::managed::handle::Managed`（物理托管句柄；`key()`/`resource()`/`shared()`，Deref→T）。
pub use rhapsody::resource::Managed as RhapsodyManaged;
/// `raw::managed::buffer::ManagedBuffer`。
pub use rhapsody::resource::ManagedBuffer;
/// `raw::texture::TextureDesc`（设备/纹理创建，32B；`new_2d/new_2d_array/new_3d/new` + builder）。
pub use rhapsody::resource::TextureDesc as RhapsodyTextureDesc;
/// `raw::texture::TextureViewDesc`（resolve 后的实际视图描述，`vk::ImageViewType/Format/AspectFlags`）。
pub use rhapsody::resource::TextureViewDesc as RhapsodyTextureViewDesc;
/// `raw::error::{Error, Result}`。
pub use rhapsody::{Error as RhapsodyError, Result as RhapsodyResult};
/// `raw::shader::{RasterPipelineOpts, ShaderLibrary, VulkanPipeline}`。
pub use rhapsody::{
    pipeline::RasterPipelineOpts, pipeline::ShaderLibrary as RhapsodyShaderLibrary,
    pipeline::VulkanPipeline as RhapsodyVulkanPipeline,
};
/// `raw::buffer::{BufferDesc, MemoryLocation}`。
pub use rhapsody::{
    resource::BufferDesc as RhapsodyBufferDesc, resource::MemoryLocation as RhapsodyMemoryLocation,
};
/// `raw::managed::texture::{ManagedTexture, ManagedTextureView}`。
pub use rhapsody::{resource::ManagedTexture, resource::ManagedTextureView};
/// `raw::texture::{Texture, TextureKind}`。
pub use rhapsody::{
    resource::Texture as RhapsodyTexture, resource::TextureKind as RhapsodyTextureKind,
};
/// Pure dynamic attachment contract; no Vulkan render-pass object.
pub use rhapsody::{
    rg::RenderPassAttachmentDesc as RhapsodyRenderPassAttachmentDesc,
    rg::RenderPassDesc as RhapsodyRenderPassDesc,
};

/// `shader_pipeline::compiled::CompiledShaderPipeline`（`load_compiled(path)`/`decode(bytes)`/`encode()`）。
pub use shader_pipeline::compiled::CompiledShaderPipeline as RhapsodyCompiledShaderPipeline;
/// `shader_pipeline::shader_defines::ShaderDefines`（`artifact_hash()`）。
pub use shader_pipeline::shader_defines::ShaderDefines;
/// `shader_pipeline::pipeline_desc::{ComputePipelineDesc, RasterPipelineDesc}`。
pub use shader_pipeline::{
    pipeline_desc::ComputePipelineDesc as RhapsodyComputePipelineDesc,
    pipeline_desc::RasterPipelineDesc as RhapsodyRasterPipelineDesc,
};

/// `rg::pass_api::AshCommandBackend`（`RenderPassApi::new` 的 Vulkan 后端）。
pub use rhapsody::rg::AshCommandBackend;
/// `rg::pass_bindings::ManagedPassResources`。
pub use rhapsody::rg::ManagedPassResources;
/// `rg::pass_builder::PassBuilder`（`add_resource_access`/`render_with_context`/`rg_mut`）。
pub use rhapsody::rg::PassBuilder as RhapsodyPassBuilder;
/// `rg::graph::PassDeclaration`（`RenderPassApi::find_resource_access_type` 用）。
pub use rhapsody::rg::PassDeclaration as RgPassDeclaration;
/// `rg::graph::Pool`（资源池；`execute_graph_with_context` 的 pool 参数）。
pub use rhapsody::rg::Pool as RgPool;
/// `rg::pass_api::RenderPassApi`（`dispatch_groups_3d`/`fill_buffer`/`clear_color_image`/`copy_buffer`）。
pub use rhapsody::rg::RenderPassApi;
pub use rhapsody::rg::ResourceHandle as RgResourceHandle;
/// `raw::submit::{DeviceState, QueueState}`（`ExecutionChain::new` 的提交状态）。
/// `rg::resource::ResourceOrigin`。
pub use rhapsody::rg::ResourceOrigin;
/// `rg::vulkan::{VulkanGraphBackend, AccessTable, GpuResource, ResourceSpec}`。
pub use rhapsody::{
    rg::AccessTable, rg::GpuResource, rg::ResourceSpec as RgResourceSpec,
    rg::VulkanGraphBackend as RhapsodyVulkanGraphBackend,
};
/// `rg::graph::{RenderGraph, Descriptor}` + `rg::resource::ResourceHandle`。
pub use rhapsody::{rg::Descriptor as RgDescriptor, rg::RenderGraph as RhapsodyRenderGraph};
/// `rg::executor::{ExecutionChain, ExecutionBackend, PassContext, GraphExecution, ExecutionStats}`。
pub use rhapsody::{
    rg::ExecutionBackend, rg::ExecutionChain, rg::ExecutionStats, rg::GraphExecution,
    rg::PassContext,
};
/// `rg::graph::{OriginSource, ResourceDeclaration, GraphDeclaration}`。
pub use rhapsody::{
    rg::GraphDeclaration, rg::OriginSource as RgOriginSource,
    rg::ResourceDeclaration as RgResourceDeclaration,
};

// ---- 适配函数（WIRING §3.2 的转换路径，纯本文件类型 ↔ rhapsody 真实类型）----

/// 游戏侧 `TextureDesc`（builder，`extent:[u32;3]`/`kind:u8`/`format:u32`）→ 真实 `raw::texture::TextureDesc`。
/// WIRING §3.2：`new([w,h,d], kind_u8, format_u8)` →
/// `new(vk::Extent3D, TextureKind::from_raw(kind).unwrap(), vk::Format::from_raw(format))`。
pub fn convert_texture_desc(desc: &TextureDesc) -> rhapsody::Result<RhapsodyTextureDesc> {
    let kind = RhapsodyTextureKind::from_raw(desc.kind)
        .ok_or_else(|| rhapsody::Error::invalid_usage("纹理种类无效"))?;
    let mut out = RhapsodyTextureDesc::new(
        ash::vk::Extent3D {
            width: desc.extent[0],
            height: desc.extent[1],
            depth: desc.extent[2],
        },
        kind,
        ash::vk::Format::from_raw(desc.format as i32),
    );
    if desc.sampled {
        out = out.sampled();
    }
    if desc.storage {
        out = out.storage();
    }
    if desc.color_attachment {
        out = out.color_attachment();
    }
    if desc.depth_stencil_attachment {
        out = out.depth_stencil_attachment();
    }
    if desc.mip_levels != 0 {
        out = out.mip_levels(desc.mip_levels);
    }
    if desc.array_elements != 0 {
        out = out.array_elements(u32::from(desc.array_elements))?;
    }
    Ok(out)
}

/// 真实 `raw::texture::TextureDesc` → `rg::graph::Descriptor::Image`（`rg::vulkan::ResourceSpec` 的
/// 字节布局编码；WIRING §2.6 的 `descriptor_from_texture` 路径）。
pub fn texture_desc_to_rg_descriptor(desc: &RhapsodyTextureDesc) -> RgDescriptor {
    RgResourceSpec::Image(*desc).descriptor()
}

/// 游戏侧 `CompiledShaderPipeline`（字节容器）→ 真实 `CompiledShaderPipeline`（`decode`，speedy 校验）。
pub fn decode_compiled_pipeline(bytes: &[u8]) -> Result<RhapsodyCompiledShaderPipeline, Error> {
    RhapsodyCompiledShaderPipeline::decode(bytes)
        .map_err(|error| anyhow::anyhow!("compiled shader decode failed: {error:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TextureDesc::new_2d/new_2d_array/new_3d` 在原版里写 `+0x1e` 维度码（2D=2、2D 数组=3、3D=4，O：见各构造器文档）。
    /// 此前重建版没建模该字节（恒 0 = 1D），宿主据 `kind` 建 VkImage 时图内创建的 2D 纹理会变成 1D——
    /// 由宿主端到端的图内纹理创建发现。
    #[test]
    fn dimension_constructors_set_the_kind_byte() {
        assert_eq!(TextureDesc::new_2d([8, 4], 37).kind, 2);
        assert_eq!(TextureDesc::new_2d_array([8, 4], 3, 37).kind, 3);
        assert_eq!(TextureDesc::new_3d([8, 4, 2], 37).kind, 4);
        // 链式 builder 不改 kind。
        assert_eq!(TextureDesc::new_2d([8, 4], 37).sampled().storage().kind, 2);
    }
}
