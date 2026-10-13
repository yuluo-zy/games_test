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

/// `rhapsody::raw::texture::TextureDesc`。`arg3/arg4` 取自纹理文件尾 −3/−4 字节（含义 U，O 值）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TextureDesc {
    pub extent: [u32; 3],
    pub arg3: u8,
    pub arg4: u8,
    pub sampled: bool,
    pub mip_levels: u8,
    pub array_elements: u16,
    /// B 阶段追加（i_api）：`TextureDesc::storage`（O 0x140bd5390：`usage |= 8`）。
    pub storage: bool,
    /// B 阶段追加（i_api）：像素格式（O：`TextureDesc::new_2d(extent, format)` 0x140bd52a0 的第二参数；
    /// `format(f)` 0x140bd5430 覆写）。`Default` 为 0。
    pub format: u32,
}

impl TextureDesc {
    pub fn new(extent: [u32; 3], arg3: u8, arg4: u8) -> Self {
        Self {
            extent,
            arg3,
            arg4,
            ..Self::default()
        }
    }
    pub fn sampled(mut self) -> Self {
        self.sampled = true;
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
    /// B 阶段追加（i_api）：`TextureDesc::new_2d(extent, format)`（O 0x140bd52a0）：深度 1、层 1。
    pub fn new_2d(extent: [u32; 2], format: u32) -> Self {
        Self {
            extent: [extent[0], extent[1], 1],
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

/// `rhapsody::raw::texture::TextureViewDesc`（O：0x3c B；B 阶段追加，i_api）。
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

/// `rhapsody::raw::texture::Texture`（O：0x38/0x40 B）。`width/height/format` 为旧占位字段，U。
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

#[derive(Debug)]
pub struct BackendError(pub String);

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

/// `rhapsody::raw::shader::RasterPipelineDesc`（0x20 B）：缓存键。+0 为托管句柄，+0x18 为 u32；
/// 二者相等才算同一键（O：查表比较处的两步判断）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterPipelineKey {
    pub handle: u64,
    pub selector: u32,
}

/// `rhapsody::raw::shader::RasterPipeline`（O：`Box` 0x38 B）。内容 U。
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

/// `rhapsody::rg::import_export` 导入缓冲后的句柄（O：`import_unowned` 返回 20 B，存进 `BufferState`
/// 的 `+0x28`；其中 `+0xe` 处有一个值为 5 的字节在 `BufferState::new` 里作“未导入”判别，见 ssbo_library）。
/// 字段含义 U，仅保留可比较的标识。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImportedBuffer {
    pub resource: u64,
    pub generation: u64,
    pub access: u32,
}

/// 渲染图（`rhapsody::rg::RenderGraph`，外部库）。占位：仅记录导入 / 导出，便于测试观察调用顺序。
#[derive(Debug, Default)]
pub struct RenderGraph {
    pub imports: Vec<(u64, u8)>,
    pub exports: Vec<(ImportedBuffer, u8)>,
}

impl RenderGraph {
    /// `import_export::impl$1::import_unowned(buffer, rg, access)`（O 0x140bd4fc0）。
    pub fn import_unowned(&mut self, buffer: u64, access: u8) -> ImportedBuffer {
        self.imports.push((buffer, access));
        ImportedBuffer {
            resource: buffer,
            generation: self.imports.len() as u64,
            access: access as u32,
        }
    }

    /// `import_export::impl$1::export(handle, rg, access)`（O 0x140bd5120）。
    pub fn export(&mut self, imported: ImportedBuffer, access: u8) {
        self.exports.push((imported, access));
    }
}
