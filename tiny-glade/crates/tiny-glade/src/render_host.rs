//! 宿主侧接缝：用 rhapsody 的设备实现 country-core 的 `rhapsody_boundary::GraphicsDevice`。
//!
//! country-core 里的 `Buffer` 是可拷贝的 40 字节值（`id`/`generation` + `byte_size`），对应原版
//! 的托管句柄；这里用槽位表把它映射到 rhapsody 的 [`ManagedBuffer`]，槽位复用时递增代数，
//! 过期句柄按“不属于本设备/已失效”处理。句柄释放见 [`BufferHost::release`]。
use country_core::render::ssbo::rhapsody_boundary::{
    Buffer, BufferDesc, BackendError, GraphicsDevice as CoreGraphicsDevice,
};
use parking_lot::Mutex;
use rhapsody::raw::{
    buffer::{BufferDesc as RhBufferDesc, MemoryLocation},
    device::GraphicsDevice,
    managed::buffer::ManagedBuffer,
};
use ash::vk;
use std::{ops::Range, sync::Arc};

struct Slot {
    generation: u64,
    buffer: Option<ManagedBuffer>,
}

pub struct BufferHost {
    device: Arc<GraphicsDevice>,
    slots: Mutex<Vec<Slot>>,
}

impl BufferHost {
    pub fn new(device: Arc<GraphicsDevice>) -> Self {
        Self {
            device,
            slots: Mutex::new(Vec::new()),
        }
    }

    fn lookup(&self, handle: &Buffer) -> Option<ManagedBuffer> {
        let slots = self.slots.lock();
        let slot = slots.get((handle.id as usize).checked_sub(1)?)?;
        (slot.generation == handle.generation)
            .then(|| slot.buffer.clone())
            .flatten()
    }

    /// 按 core 句柄的 `id` 取（槽位序号，不校验代数；图导入只在句柄刚创建/仍持有时发生）。
    pub fn get_by_id(&self, id: u64) -> Option<ManagedBuffer> {
        let slots = self.slots.lock();
        slots.get((id as usize).checked_sub(1)?)?.buffer.clone()
    }

    /// 释放句柄：槽位清空并递增代数，缓冲随最后一个克隆释放进入延迟销毁。
    pub fn release(&self, handle: &Buffer) -> bool {
        let mut slots = self.slots.lock();
        let Some(slot) = (handle.id as usize)
            .checked_sub(1)
            .and_then(|i| slots.get_mut(i))
        else {
            return false;
        };
        if slot.generation != handle.generation || slot.buffer.is_none() {
            return false;
        }
        slot.buffer = None;
        slot.generation += 1;
        true
    }

    /// 原版 `Buffer` 的 40 字节值（`create_buffer` 的 Ok 载荷）：`ctx`=槽表句柄 id，`slot`=代数；`offset`/`device_id`
    /// 槽表不提供，记 0（U）。其余字段取自真实缓冲（大小、`BufferDesc+0x08` 的 usage/对齐/内存位置）。
    fn record_of(managed: &ManagedBuffer, id: u64, generation: u64) -> rhapsody::raw::buffer::BufferRecord {
        managed.record(id, generation, 0, 0)
    }

    /// 记录 → core 的 `Buffer`（同一 40 字节布局：`+0x00 ctx`、`+0x08 slot`、`+0x10 {offset,device_id,ready}`、
    /// `+0x18 size`、`+0x20 desc_tail`；core 里 `+0x10/+0x20` 原为 U 的占位字段）。
    fn core_buffer(record: &rhapsody::raw::buffer::BufferRecord) -> Buffer {
        Buffer {
            id: record.ctx,
            generation: record.slot,
            _gap10: u64::from(record.offset)
                | (u64::from(record.device_id) << 32)
                | (u64::from(record.ready) << 48)
                | (u64::from(record._pad) << 56),
            byte_size: record.size,
            _gap20: record.desc_tail,
        }
    }

    /// core 句柄 → rhapsody 的 40 字节 `BufferRecord`（过期/未知句柄返回 `None`）。
    pub fn record(&self, handle: &Buffer) -> Option<rhapsody::raw::buffer::BufferRecord> {
        let managed = self.lookup(handle)?;
        Some(Self::record_of(&managed, handle.id, handle.generation))
    }

    pub fn convert(desc: &BufferDesc) -> RhBufferDesc {
        let location = match desc.memory {
            1 => MemoryLocation::GpuOnly,
            _ => MemoryLocation::CpuToGpu,
        };
        let mut out = RhBufferDesc::new(
            desc.size as u64,
            vk::BufferUsageFlags::from_raw(desc.usage),
            location,
        );
        if desc.alignment > 1 {
            out = out.with_alignment(desc.alignment as u16);
        }
        out
    }
}

impl CoreGraphicsDevice for BufferHost {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        name: &str,
        data: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        let managed = self
            .device
            .create_buffer(&Self::convert(desc), name, data)
            .map_err(|_| BackendError)?;
        let mut slots = self.slots.lock();
        let index = slots
            .iter()
            .position(|s| s.buffer.is_none())
            .unwrap_or_else(|| {
                slots.push(Slot {
                    generation: 1,
                    buffer: None,
                });
                slots.len() - 1
            });
        let (id, generation) = (index as u64 + 1, slots[index].generation);
        let record = Self::record_of(&managed, id, generation);
        slots[index].buffer = Some(managed);
        Ok(Self::core_buffer(&record))
    }

    fn update_buffer_bytes(
        &self,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), BackendError> {
        let managed = self.lookup(buffer).ok_or(BackendError)?;
        self.device
            .update_buffer_bytes(&managed, data, ranges, offset)
            .map_err(|_| BackendError)
    }

    fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
        let managed = self.lookup(buffer)?;
        let ptr = managed.persistent_host_ptr()?;
        // SAFETY: 指针来自持久映射且已含偏移；槽位在 `release` 之前一直持有缓冲，
        // 调用方（原版语义）负责与 GPU 的同步及不并发别名。
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, managed.size as usize) })
    }

    fn device_address(&self, buffer: &Buffer) -> u64 {
        self.lookup(buffer)
            .and_then(|b| b.device_address().ok())
            .expect("Buffer device-address backend: invalid handle or buffer without BDA usage")
    }
}

// ---------------------------------------------------------------- texture

use country_core::render::load_texture::boundary::{
    BackendError as TextureBackendError, Texture, TextureDesc, TextureDevice,
};
use rhapsody::raw::{
    managed::texture::ManagedTexture,
    texture::{TextureDesc as RhTextureDesc, TextureKind},
};

/// `Texture.id` 的标记位：图导入入口 `import_unowned(id, access)` 对缓冲和纹理共用，靠它区分。
pub const TEXTURE_ID_TAG: u64 = 1 << 63;

/// 缓冲导入 token 的标记位。rhapsody 后端的 token 空间由 `allocate_token()`（从小整数递增，图内创建的资源也从这里取）
/// 与显式 `bind_import*` 共用；宿主槽位序号是小整数，直接当 token 会与图内资源撞车
/// （“token已经属于其他物理资源”），所以导入图时一律经 [`graph_token`] 打上高位标记。
pub const BUFFER_TOKEN_TAG: u64 = 1 << 62;

/// core 句柄 `id`（缓冲槽位序号，或带 [`TEXTURE_ID_TAG`] 的纹理 id）→ rhapsody 后端的跨帧 token。
/// 帧驱动方用同一个值调 `VulkanGraphBackend::bind_import_named(.., graph_token(id), ..)`。
pub fn graph_token(id: u64) -> u64 {
    if id & TEXTURE_ID_TAG != 0 { id } else { id | BUFFER_TOKEN_TAG }
}

struct TextureSlot {
    generation: u64,
    texture: Option<ManagedTexture>,
}

/// core 的 `TextureDevice` 接缝：`Texture.id` 是槽位序号（从 1 起），代数由 `release` 递增。
pub struct TextureHost {
    device: Arc<GraphicsDevice>,
    slots: Mutex<Vec<TextureSlot>>,
    /// 默认视图缓存（原 `ManagedTexture::view` 按描述缓存，M2.6）：视图随纹理一起存活，`release` 时丢弃。
    views: Mutex<std::collections::HashMap<u64, rhapsody::raw::managed::texture::ManagedTextureView>>,
    /// 宿主附加的用途位（core 的 `TextureDesc` 只有 sampled/storage/附件五个开关，没有 TRANSFER_SRC；
    /// 读回像素的宿主/测试在这里统一加上。原版 `TextureDesc.usage` 是完整位集，此项是宿主策略，不是原版行为）。
    extra_usage: vk::ImageUsageFlags,
}

impl TextureHost {
    pub fn new(device: Arc<GraphicsDevice>) -> Self {
        Self {
            device,
            slots: Mutex::new(Vec::new()),
            views: Mutex::new(Default::default()),
            extra_usage: vk::ImageUsageFlags::empty(),
        }
    }

    /// 之后创建的所有纹理（含图内创建的）额外带上这些用途位。
    pub fn with_extra_usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.extra_usage = usage;
        self
    }

    /// `convert` + 宿主附加用途位（图内创建纹理与 `create_texture` 共用）。
    pub fn convert_for_graph(&self, desc: &TextureDesc) -> Result<RhTextureDesc, TextureBackendError> {
        let mut out = Self::convert(desc)?;
        out.usage |= self.extra_usage;
        Ok(out)
    }

    /// 默认全范围视图的原生句柄（首次创建后缓存）。
    pub fn default_view(&self, id: u64) -> Option<u64> {
        let mut views = self.views.lock();
        if let Some(view) = views.get(&id) {
            return Some(ash::vk::Handle::as_raw(view.raw()));
        }
        let texture = self.get_by_id(id)?;
        let desc = rhapsody::raw::texture::TextureViewDesc::new(&texture.desc);
        let view = self.device.get_vk_image_view(&texture, &desc).ok()?;
        let raw = ash::vk::Handle::as_raw(view.raw());
        views.insert(id, view);
        Some(raw)
    }

    /// rhapsody 描述 → core 描述（`managed_texture_desc`）。
    pub fn core_desc(texture: &ManagedTexture) -> TextureDesc {
        let d = &texture.desc;
        TextureDesc {
            extent: [d.width, d.height, d.depth],
            kind: d.kind as u8,
            sampled: d.usage.contains(vk::ImageUsageFlags::SAMPLED),
            mip_levels: d.mip_levels,
            array_elements: d.array_elements as u16,
            storage: d.usage.contains(vk::ImageUsageFlags::STORAGE),
            color_attachment: d.usage.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT),
            depth_stencil_attachment: d
                .usage
                .contains(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
            format: d.format.as_raw() as u32,
        }
    }

    pub fn get(&self, texture: &Texture) -> Option<ManagedTexture> {
        let slots = self.slots.lock();
        let slot = slots.get(((texture.id & 0xffff_ffff) as usize).checked_sub(1)?)?;
        slot.texture.clone().filter(|_| slot.generation == texture.generation())
    }

    /// 按 `Texture.id`（含标记与代数）取，代数不符返回 `None`。
    pub fn get_by_id(&self, id: u64) -> Option<ManagedTexture> {
        let slots = self.slots.lock();
        let slot = slots.get(((id & 0xffff_ffff) as usize).checked_sub(1)?)?;
        let generation = (id >> 32) & 0x7fff_ffff;
        slot.texture.clone().filter(|_| slot.generation == generation)
    }

    pub fn release(&self, texture: &Texture) -> bool {
        let mut slots = self.slots.lock();
        let Some(slot) = ((texture.id & 0xffff_ffff) as usize)
            .checked_sub(1)
            .and_then(|i| slots.get_mut(i))
        else {
            return false;
        };
        if slot.generation != texture.generation() || slot.texture.is_none() {
            return false;
        }
        slot.texture = None;
        slot.generation += 1;
        self.views.lock().remove(&texture.id);
        true
    }

    /// core 描述 → rhapsody 原布局描述；`kind` 非法时报错而不是猜。
    pub fn convert(desc: &TextureDesc) -> Result<RhTextureDesc, TextureBackendError> {
        let kind = TextureKind::from_raw(desc.kind)
            .ok_or_else(|| TextureBackendError(format!("unknown texture kind {}", desc.kind)))?;
        let extent = vk::Extent3D {
            width: desc.extent[0],
            height: desc.extent[1],
            depth: desc.extent[2],
        };
        let mut out = RhTextureDesc::new(extent, kind, vk::Format::from_raw(desc.format as i32))
            .mip_levels(desc.mip_levels.max(1))
            .array_elements(u32::from(desc.array_elements).max(1));
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
        Ok(out)
    }
}

impl TextureDevice for TextureHost {
    fn create_texture(
        &self,
        name: &str,
        desc: &TextureDesc,
        data: &[u8],
    ) -> Result<Texture, TextureBackendError> {
        let rh = self.convert_for_graph(desc)?;
        let managed = self
            .device
            .create_texture(&rh, name, data)
            .map_err(|e| TextureBackendError(format!("{e:?}")))?;
        let mut slots = self.slots.lock();
        let index = slots
            .iter()
            .position(|s| s.texture.is_none())
            .unwrap_or_else(|| {
                slots.push(TextureSlot {
                    generation: 1,
                    texture: None,
                });
                slots.len() - 1
            });
        slots[index].texture = Some(managed);
        let generation = slots[index].generation;
        Ok(Texture {
            id: TEXTURE_ID_TAG | (generation << 32) | (index as u64 + 1),
            width: desc.extent[0],
            height: desc.extent[1],
            format: desc.format,
        })
    }
}

trait TextureGeneration {
    fn generation(&self) -> u64;
}
impl TextureGeneration for Texture {
    /// core 的 `Texture` 只有 `id: u64`：bit 63 是纹理标记，bits 32..63 是代数，低 32 位是槽位序号。
    fn generation(&self) -> u64 {
        (self.id >> 32) & 0x7fff_ffff
    }
}

// ---------------------------------------------------------------- compute pipeline

use country_core::render::shader::boundary::{
    CompiledShaderPipeline as CoreCompiled, ComputePipeline, ComputePipelineDesc, Error as ShaderError,
    RasterPipeline, RasterPipelineDesc, RasterPipelineKey, ShaderBackend,
};
use rhapsody::{
    raw::{
        descriptor_set::DescriptorSetState,
        sampler::Sampler,
        shader::{
            ComputePipelineDesc as RhComputeDesc, PipelineCache, RasterPipelineDesc as RhRasterDesc,
            RasterPipelineOpts, ShaderInterfacePlan, ShaderModule, VulkanPipeline,
            vertex_input_from_reflection,
        },
        render_pass::{RenderPass, RenderPassKey},
    },
    shader_pipeline::{
        compiled::{CompiledShaderPipeline as RhCompiled, ShaderStage},
        pipeline_desc::ComputePipelineDesc as FileComputeDesc,
        shader_defines::ShaderDefines,
    },
};
use country_core::render::pass_builder::{
    compute_pass_builder::{ComputePassDevice, ResourceRegistry},
    uniform_binder::{UniformBlockInfo, UniformReflection},
};
use rhapsody::shader_pipeline::reflection::ShaderReflection;
use std::{collections::BTreeMap, path::PathBuf};

/// 发行 .bin 里存的反射 → core 的 `ResourceRegistry`：名字哈希 → (set, binding)、uniform 块大小与所在 set、
/// uniform 字段名哈希 → 偏移。`UniformBlockInfo.set_index` 取块绑定位置的 set（原偏移 `+0x130` 的 u32，字段名未证实）。
pub fn registry_from_reflection(reflection: &ShaderReflection) -> ResourceRegistry {
    ResourceRegistry {
        bindings: reflection
            .binding_name_hashes
            .iter()
            .map(|(hash, location)| (*hash, (location.set, location.binding)))
            .collect(),
        uniform_block: reflection.uniform_block.as_ref().map(|block| UniformBlockInfo {
            size: block.byte_size,
            set_index: block.binding.set,
        }),
        uniform: UniformReflection {
            valid: reflection.uniform_block.is_some(),
            fields: reflection
                .uniform_block
                .iter()
                .flat_map(|block| block.name_hash_offsets.iter().copied())
                .collect(),
        },
    }
}

/// core 的 `ShaderBackend` 接缝（计算管线完整，光栅管线只到“读发行 .bin”）。
///
/// 发行 .bin 的路径由 shader-pipeline 的 `compiled_file_path` 给出（相对 `root`），解码、反射布局、
/// 描述符/管线布局和管线创建全部走 rhapsody。编译产物里的 Sampler 绑定需要宿主提供名字 → 采样器的表
/// （原版取自设备内置采样器，名字映射未恢复，所以缺失时明确报错而不是猜）。
pub struct ShaderHost {
    device: Arc<GraphicsDevice>,
    root: PathBuf,
    cache: PipelineCache,
    samplers: BTreeMap<String, Arc<Sampler>>,
    compute: Mutex<Vec<Arc<ComputeEntry>>>,
    raster: Mutex<Vec<Arc<RasterEntry>>>,
    passes: Arc<RenderPassHost>,
}

/// 渲染通道表：core 的 `RasterPipelineKey.handle` 就是这里的 id（原版是托管渲染通道句柄，kind=4）。
/// 原 `create_raster_pipeline` 从通道对象里读到三样东西：`VkRenderPass`（+0x10）、颜色附件数（+0x18）、
/// 采样数（+0x20），这里照样保存。
#[derive(Default)]
pub struct RenderPassHost {
    table: Mutex<Vec<Arc<RenderPassEntry>>>,
    /// `create_from_desc` 的键表（原 `GraphicsDevice` `+0x188` 的 `HashMap<RenderPassDesc, RenderPass>` 缓存）。
    by_desc: Mutex<std::collections::HashMap<rhapsody::raw::render_pass::RenderPassDesc, u64>>,
}

pub struct RenderPassEntry {
    pub pass: Arc<RenderPass>,
    pub color_attachments: usize,
    pub samples: vk::SampleCountFlags,
}

impl RenderPassHost {
    /// 按原版 0x98B `RenderPassDesc` 键建通道（`create_render_pass` 0x140bfc0d0）：命中缓存返回同一 id。
    /// 颜色附件数/采样数取自 `RenderPass::from_desc` 填的槽负载（`+0x18`/`+0x20`）。
    pub fn create_from_desc(
        &self,
        device: &Arc<GraphicsDevice>,
        desc: &rhapsody::raw::render_pass::RenderPassDesc,
    ) -> Result<u64, rhapsody::raw::error::Error> {
        let mut by_desc = self.by_desc.lock();
        if let Some(id) = by_desc.get(desc) {
            return Ok(*id);
        }
        let pass = Arc::new(RenderPass::from_desc(device.inner.clone(), desc)?);
        let entry = RenderPassEntry {
            color_attachments: pass.color_count as usize,
            samples: vk::SampleCountFlags::from_raw(pass.samples),
            pass,
        };
        let mut table = self.table.lock();
        table.push(Arc::new(entry));
        let id = table.len() as u64;
        by_desc.insert(*desc, id);
        Ok(id)
    }

    /// 已建通道数（测试/诊断）。
    pub fn len(&self) -> usize {
        self.table.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 按 `RenderPassKey` 建一个渲染通道并登记，返回 id（从 1 起）。
    pub fn create(
        &self,
        device: &Arc<GraphicsDevice>,
        key: &RenderPassKey,
    ) -> Result<u64, rhapsody::raw::error::Error> {
        let pass = Arc::new(RenderPass::from_key(device.inner.clone(), key)?);
        let color_attachments = key.attachments.iter().filter(|a| !a.depth_stencil).count();
        let samples = key
            .attachments
            .first()
            .map_or(vk::SampleCountFlags::TYPE_1, |a| vk::SampleCountFlags::from_raw(a.samples));
        let mut table = self.table.lock();
        table.push(Arc::new(RenderPassEntry { pass, color_attachments, samples }));
        Ok(table.len() as u64)
    }

    pub fn get(&self, id: u64) -> Option<Arc<RenderPassEntry>> {
        self.table.lock().get((id as usize).checked_sub(1)?).cloned()
    }
}

/// 一条已创建的光栅管线及绘制时需要的反射信息。
pub struct RasterEntry {
    pub pipeline: Arc<VulkanPipeline>,
    pub registry: Arc<ResourceRegistry>,
    pub pass: Arc<RenderPassEntry>,
    /// set 0 的描述符集分配器（与计算管线一致，I）。
    pub set0: Option<DescriptorSetState>,
    pub uniform_binding: Option<u32>,
}

/// 一条已创建的计算管线及执行它所需的反射信息。
pub struct ComputeEntry {
    pub pipeline: Arc<VulkanPipeline>,
    pub registry: Arc<ResourceRegistry>,
    /// 着色器 `local_size`（反射 `local_size`，缺省 1×1×1）。
    pub local_size: [u32; 3],
    /// set 0 的描述符集分配器（原版 `assert_eq!(set_index, 0)`：计算 Pass 只用 set 0）。
    pub set0: Option<DescriptorSetState>,
    /// uniform 块所在的 binding 号（反射 `uniform_block.binding.binding`）。
    pub uniform_binding: Option<u32>,
}

impl ShaderHost {
    pub fn new(
        device: Arc<GraphicsDevice>,
        root: impl Into<PathBuf>,
    ) -> Result<Self, vk::Result> {
        let cache = PipelineCache::new(device.inner.clone(), &[])?;
        Ok(Self {
            device,
            root: root.into(),
            cache,
            samplers: BTreeMap::new(),
            compute: Mutex::new(Vec::new()),
            raster: Mutex::new(Vec::new()),
            passes: Arc::new(RenderPassHost::default()),
        })
    }

    pub fn passes(&self) -> &Arc<RenderPassHost> {
        &self.passes
    }

    pub fn raster_entry(&self, handle: &RasterPipeline) -> Option<Arc<RasterEntry>> {
        self.raster.lock().get((handle.id as usize).checked_sub(1)?).cloned()
    }

    pub fn with_sampler(mut self, name: &str, sampler: Arc<Sampler>) -> Self {
        self.samplers.insert(name.to_owned(), sampler);
        self
    }

    pub fn compute_pipeline(&self, handle: &ComputePipeline) -> Option<Arc<VulkanPipeline>> {
        self.compute_entry(handle).map(|e| e.pipeline.clone())
    }

    pub fn compute_entry(&self, handle: &ComputePipeline) -> Option<Arc<ComputeEntry>> {
        self.compute.lock().get((handle.id as usize).checked_sub(1)?).cloned()
    }

    /// 该管线的反射登记表（`ComputePass::new` 里 `[arc+0x10]` 一族偏移读到的那份）。
    pub fn compute_registry(&self, handle: &ComputePipeline) -> Option<Arc<ResourceRegistry>> {
        self.compute_entry(handle).map(|e| e.registry.clone())
    }

    fn file_desc(desc: &ComputePipelineDesc) -> FileComputeDesc {
        FileComputeDesc {
            source_path: desc.path.clone(),
            entry_point: desc.entry.clone(),
            defines: ShaderDefines(desc.defines.clone()),
        }
    }

    fn read(&self, relative: PathBuf) -> Result<CoreCompiled, ShaderError> {
        let path = self.root.join(relative);
        let bytes = std::fs::read(&path)
            .map_err(|e| anyhow::anyhow!("Error opening {}: {e}", path.display()))?;
        Ok(CoreCompiled(bytes))
    }
}

impl ShaderBackend for ShaderHost {
    fn load_compiled_compute(&self, desc: &ComputePipelineDesc) -> Result<CoreCompiled, ShaderError> {
        self.read(Self::file_desc(desc).compiled_file_path())
    }

    fn load_compiled_raster(&self, desc: &RasterPipelineDesc) -> Result<CoreCompiled, ShaderError> {
        let file = rhapsody::shader_pipeline::pipeline_desc::RasterPipelineDesc {
            source_path: desc.path.clone(),
            vertex_entry: String::new(),
            fragment_entry: desc.entry.clone(),
            defines: ShaderDefines(desc.defines.clone()),
        };
        self.read(file.compiled_file_path())
    }

    fn raster_shader_variants(&self, desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
        // O 0x1417388a0 `get_raster_shader_variants`：把 desc 克隆两份，各在 defines 末尾追加
        // `("SHADER_VARIANT_DEPTH_ONLY", "0")` / `("SHADER_VARIANT_DEPTH_ONLY", "1")`
        // （25 字节名字在 0x142ccd739，值字节 0x30 / 0x31 是立即数）。
        let with = |value: &str| {
            let mut variant = desc.clone();
            variant.defines.push(("SHADER_VARIANT_DEPTH_ONLY".to_owned(), value.to_owned()));
            variant
        };
        [with("0"), with("1")]
    }

    fn create_compute_pipeline(&self, name: &str, compiled: Arc<CoreCompiled>) -> ComputePipeline {
        let build = || -> Result<Arc<ComputeEntry>, ShaderError> {
            let decoded = RhCompiled::decode(&compiled.0)?;
            let stage = decoded
                .stages
                .iter()
                .find(|s| s.stage == ShaderStage::Compute)
                .ok_or_else(|| anyhow::anyhow!("{name}: no compute stage"))?;
            let mut names = BTreeMap::new();
            for (n, s) in &self.samplers {
                names.insert(n.clone(), ash::vk::Handle::as_raw(s.handle));
            }
            let scalar = self.device.inner.max_descriptor_set_update_after_bind_sampled_images();
            let plan = ShaderInterfacePlan::from_compiled_pipeline(&decoded, &names, scalar)
                .map_err(|e| anyhow::anyhow!("{name}: interface plan: {e:?}"))?;
            let retained: Vec<_> = self.samplers.values().cloned().collect();
            let layout = plan
                .create_pipeline_layout(self.device.inner.clone(), &retained)
                .map_err(|e| anyhow::anyhow!("{name}: pipeline layout: {e:?}"))?;
            let bytes = stage.spirv.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>();
            let shader = ShaderModule::from_spirv(self.device.inner.clone(), &bytes)
                .map_err(|e| anyhow::anyhow!("{name}: shader module: {e:?}"))?;
            let mut desc = RhComputeDesc::new(shader, layout);
            desc.entry = std::ffi::CString::new(stage.entry_point.as_str())?;
            let registry = Arc::new(registry_from_reflection(&decoded.reflection));
            let pipeline = VulkanPipeline::create_compute_pipeline(&self.cache, desc)
                .map_err(|e| anyhow::anyhow!("{name}: pipeline: {e:?}"))?;
            let set0 = match pipeline.layout.sets.first() {
                Some(layout) => Some(
                    DescriptorSetState::for_layout(layout.clone())
                        .map_err(|e| anyhow::anyhow!("{name}: descriptor state: {e:?}"))?,
                ),
                None => None,
            };
            Ok(Arc::new(ComputeEntry {
                pipeline,
                registry,
                local_size: decoded.reflection.local_size.unwrap_or([1, 1, 1]),
                set0,
                uniform_binding: decoded.reflection.uniform_block.as_ref().map(|b| b.binding.binding),
            }))
        };
        // 原调用点不检查结果（0x1416a4b20）；失败在使用句柄时才暴露，这里按原样 panic 更早更清楚。
        let entry = build().unwrap_or_else(|e| panic!("create_compute_pipeline: {e}"));
        let mut table = self.compute.lock();
        table.push(entry);
        ComputePipeline { id: table.len() as u64 }
    }

    fn create_raster_pipeline(
        &self,
        compiled: Arc<CoreCompiled>,
        key: &RasterPipelineKey,
    ) -> Result<RasterPipeline, ShaderError> {
        // O 0x1416a3720：key.handle 是渲染通道句柄，key.selector 是 4 字节 `RasterPipelineOpts`。
        let pass = self
            .passes
            .get(key.handle)
            .ok_or_else(|| anyhow::anyhow!("raster pipeline: render pass handle {} is not live", key.handle))?;
        let decoded = RhCompiled::decode(&compiled.0)?;
        let stage_of = |wanted: ShaderStage| {
            decoded
                .stages
                .iter()
                .find(|s| s.stage == wanted)
                .ok_or_else(|| anyhow::anyhow!("raster pipeline: missing {wanted:?} stage"))
        };
        let vertex = stage_of(ShaderStage::Vertex)?;
        let fragment = stage_of(ShaderStage::Fragment)?;

        let mut names = BTreeMap::new();
        for (n, s) in &self.samplers {
            names.insert(n.clone(), ash::vk::Handle::as_raw(s.handle));
        }
        let scalar = self.device.inner.max_descriptor_set_update_after_bind_sampled_images();
        let plan = ShaderInterfacePlan::from_compiled_pipeline(&decoded, &names, scalar)
            .map_err(|e| anyhow::anyhow!("raster interface plan: {e:?}"))?;
        let retained: Vec<_> = self.samplers.values().cloned().collect();
        let layout = plan
            .create_pipeline_layout(self.device.inner.clone(), &retained)
            .map_err(|e| anyhow::anyhow!("raster pipeline layout: {e:?}"))?;
        let module = |stage: &rhapsody::shader_pipeline::compiled::CompiledShaderStage| {
            let bytes = stage.spirv.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>();
            ShaderModule::from_spirv(self.device.inner.clone(), &bytes)
                .map_err(|e| anyhow::anyhow!("raster shader module: {e:?}"))
        };
        let mut desc = RhRasterDesc::new(module(vertex)?, layout);
        desc.fragment = Some(module(fragment)?);
        desc.entry = std::ffi::CString::new(vertex.entry_point.as_str())?;
        desc.fragment_entry = Some(std::ffi::CString::new(fragment.entry_point.as_str())?);
        desc.render_pass = pass.pass.handle;
        desc.render_pass_owner = pass.pass.clone();
        desc.subpass = 0;
        desc.samples = pass.samples;
        let (bindings, attributes) = vertex_input_from_reflection(&decoded.reflection)
            .map_err(|e| anyhow::anyhow!("raster vertex input: {e:?}"))?;
        desc.vertex_bindings = bindings;
        desc.vertex_attributes = attributes;
        RasterPipelineOpts(key.selector).apply_to(&mut desc, pass.color_attachments);

        let pipeline = VulkanPipeline::create_raster_pipeline(&self.cache, desc)
            .map_err(|e| anyhow::anyhow!("raster pipeline: {e:?}"))?;
        let set0 = match pipeline.layout.sets.first() {
            Some(layout) => Some(
                DescriptorSetState::for_layout(layout.clone())
                    .map_err(|e| anyhow::anyhow!("raster descriptor state: {e:?}"))?,
            ),
            None => None,
        };
        let entry = Arc::new(RasterEntry {
            pipeline,
            registry: Arc::new(registry_from_reflection(&decoded.reflection)),
            pass,
            set0,
            uniform_binding: decoded.reflection.uniform_block.as_ref().map(|b| b.binding.binding),
        });
        let mut table = self.raster.lock();
        table.push(entry);
        Ok(RasterPipeline { id: table.len() as u64, compiled })
    }
}

// ---------------------------------------------------------------- render graph

use country_core::rhapsody_types::{BufferGraphBackend, ImportedBuffer};
use rhapsody::rg::{graph::RenderGraph as RhGraph, resource::ResourceHandle, vulkan::ResourceSpec};
use std::{cell::RefCell, rc::Rc};

/// core `RenderGraph` 的后端：把 `import_unowned(id, access)` / `export` 落到 rhapsody 图声明上。
///
/// `id` 是 core 句柄里的 `id`（缓冲槽位序号，或带 [`TEXTURE_ID_TAG`] 的纹理 id），描述符取自宿主表里
/// 的真实资源；图里记录的 token 就是这个 id，供之后的物理资源映射反查。
pub struct GraphHost {
    graph: Rc<RefCell<RhGraph>>,
    buffers: Arc<BufferHost>,
    textures: Arc<TextureHost>,
}

impl GraphHost {
    pub fn new(
        graph: Rc<RefCell<RhGraph>>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
    ) -> Self {
        Self { graph, buffers, textures }
    }
}

impl BufferGraphBackend for GraphHost {
    fn import_unowned(&mut self, id: u64, access: u8) -> ImportedBuffer {
        let descriptor = if id & TEXTURE_ID_TAG != 0 {
            let texture = self
                .textures
                .get_by_id(id)
                .expect("import_unowned: texture handle is not live");
            ResourceSpec::Image(texture.desc).descriptor()
        } else {
            let buffer = self
                .buffers
                .get_by_id(id)
                .expect("import_unowned: buffer handle is not live");
            ResourceSpec::Buffer(buffer.desc).descriptor()
        };
        let handle = self
            .graph
            .borrow_mut()
            // 名字按 id 区分：同一张图里导入多个资源时，同名会让后端的名字解析器串位。
            .import_unowned(format!("imported-{id:x}"), descriptor, graph_token(id), access)
            .expect("import_unowned: graph rejected the resource");
        ImportedBuffer {
            resource: u64::from(handle.raw()),
            generation: 0,
            access: u32::from(access),
        }
    }

    fn export(&mut self, imported: ImportedBuffer, access: u8) {
        self.graph
            .borrow_mut()
            .export(ResourceHandle::from_raw(imported.resource as u32), access)
            .expect("export: handle does not belong to this graph");
    }
}

// ---------------------------------------------------------------- compute pass device

use country_core::{
    render::shader::ComputeShaderProgram,
    rhapsody_contract::ManagedHandle,
};

/// core `ComputePassDevice` 的宿主实现：把缓冲/纹理/着色器三张表合在一起。
pub struct DeviceHost {
    pub buffers: Arc<BufferHost>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
}

impl TextureDevice for DeviceHost {
    fn create_texture(
        &self,
        name: &str,
        desc: &TextureDesc,
        data: &[u8],
    ) -> Result<Texture, TextureBackendError> {
        self.textures.create_texture(name, desc, data)
    }
}

impl ComputePassDevice for DeviceHost {
    /// `GraphicsDevice::get_vk_buffer`：核心的 `ManagedHandle` 里装的是缓冲 `id`。
    fn get_vk_buffer(&self, handle: &ManagedHandle) -> Option<u64> {
        let buffer = self.buffers.get_by_id(handle.0)?;
        Some(ash::vk::Handle::as_raw(buffer.raw()))
    }

    fn pipeline_registry(&self, program: &ComputeShaderProgram) -> Arc<ResourceRegistry> {
        self.shaders
            .compute_registry(&program.pipeline)
            .expect("pipeline_registry: program was not created through this host")
    }
}

use country_core::render::pass_builder::pass_resource_access::TextureDevice as PassTextureDevice;
use country_core::rhapsody_types::BackendError as PassBackendError;

impl PassTextureDevice for DeviceHost {
    /// `GraphicsDevice::get_vk_image_view(handle, None)`：默认视图。
    fn get_vk_image_view(&self, handle: &ManagedHandle) -> Result<u64, PassBackendError> {
        self.textures
            .default_view(handle.0)
            .ok_or_else(|| PassBackendError("get_vk_image_view: texture handle is not live".into()))
    }

    fn managed_texture_desc(&self, handle: &ManagedHandle) -> TextureDesc {
        let texture = self
            .textures
            .get_by_id(handle.0)
            .expect("managed_texture_desc: texture handle is not live");
        TextureHost::core_desc(&texture)
    }
}

// ---------------------------------------------------------------- render graph passes

use country_core::render::rg_helpers::rg_boundary::{
    CopyRegion, RenderFn, RenderGraph as CorePassGraph, RenderPassApi as CoreRenderPassApi,
    StagingAlloc,
};
use rhapsody::{
    raw::{buffer::Buffer as RhBuffer, dynamic_constants::DynamicConstants},
    rg::{
        pass_api::{AshCommandBackend, CommandBackend, RenderPassApi as RhRenderPassApi},
        pass_bindings::ManagedPassResources,
    },
};
use std::collections::HashMap;

/// 暂存缓冲登记：`StagingAlloc.buffer`（Vulkan 句柄原始值）→ 持有该页的缓冲，供回调里回写数据。
type ChunkTable = Arc<Mutex<HashMap<u64, Arc<RhBuffer>>>>;

/// core `rg_boundary::RenderGraph` 的宿主实现：pass 声明落到 rhapsody 图，动态常量走 rhapsody 的
/// `DynamicConstants`（原版 256 KiB 页、线性分配）。
pub struct GraphPasses {
    graph: Rc<RefCell<RhGraph>>,
    device: Arc<GraphicsDevice>,
    dynamic: DynamicConstants,
    chunks: ChunkTable,
    tables: Option<HostTables>,
}

/// 计算 Pass 执行时需要按 id 反查的三张宿主表。
#[derive(Clone)]
pub struct HostTables {
    pub buffers: Arc<BufferHost>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
}

impl GraphPasses {
    pub fn new(graph: Rc<RefCell<RhGraph>>, device: Arc<GraphicsDevice>) -> Result<Self, rhapsody::raw::error::Error> {
        let dynamic = DynamicConstants::new(device.inner.clone())?;
        Ok(Self {
            graph,
            device,
            dynamic,
            chunks: Arc::new(Mutex::new(HashMap::new())),
            tables: None,
        })
    }

    pub fn with_tables(mut self, tables: HostTables) -> Self {
        self.tables = Some(tables);
        self
    }
}

impl CorePassGraph for GraphPasses {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn) {
        let device = self.device.inner.clone();
        let chunks = self.chunks.clone();
        let mut graph = self.graph.borrow_mut();
        let mut pass = graph.add_pass(name);
        // 原 `add_resource_access(id, access, 0)`：同步组 0。
        pass.add_resource_access(ResourceHandle::from_raw(resource), access, 0)
            .expect("add_resource_access: resource does not belong to this graph");
        pass.render_with_context(move |ctx| {
            let managed = ctx.managed.expect("pass callback without managed resources");
            // SAFETY: 命令缓冲、资源与设备在回调期间由 recorder 持有存活。
            let mut backend = unsafe { AshCommandBackend::new(&device.device) };
            let command = <ash::vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());
            let mut host = PassApiHost {
                api: RhRenderPassApi::new(&mut backend, command, 0, false),
                managed,
                chunks: &chunks,
                used: Vec::new(),
            };
            render(&mut host);
            // 回调返回后命令缓冲才提交：回调里用到的暂存页必须活到 GPU 完成，交给命令拥有者保活。
            for page in std::mem::take(&mut host.used) {
                ctx.keep_alive(page);
            }
            true
        });
    }

    fn allocate_dynamic(&mut self, size: usize) -> StagingAlloc {
        let chunk = self
            .dynamic
            .allocate(size as u64)
            .expect("DynamicConstants::allocate failed");
        let buffer = ash::vk::Handle::as_raw(chunk.buffer.handle);
        self.chunks.lock().insert(buffer, chunk.buffer.clone());
        StagingAlloc {
            buffer,
            offset: chunk.offset as u32,
            bytes: vec![0; size],
        }
    }
}

struct PassApiHost<'a, B: CommandBackend> {
    api: RhRenderPassApi<'a, B>,
    managed: &'a ManagedPassResources,
    chunks: &'a ChunkTable,
    used: Vec<Arc<RhBuffer>>,
}

impl<B: CommandBackend> CoreRenderPassApi for PassApiHost<'_, B> {
    fn fill_buffer(&mut self, buffer: u32, range: Option<std::ops::Range<u64>>, value: u32) {
        let buffer = self.managed.buffer_for(buffer).expect("fill_buffer: unresolved buffer");
        self.api
            .fill_buffer(buffer.handle, range.map(|r| (r.start, r.end - r.start)), value)
            .expect("fill_buffer: unaligned range");
    }

    fn clear_color_image(&mut self, image: u32) {
        let texture = self.managed.texture_for(image).expect("clear_color_image: unresolved image");
        self.api.clear_color_image(
            texture.handle,
            texture.desc.mip_levels,
            texture.desc.array_layers(),
        );
    }

    fn copy_buffer(&mut self, src: &StagingAlloc, dst: u32, regions: &[CopyRegion]) {
        let dst = self.managed.buffer_for(dst).expect("copy_buffer: unresolved buffer");
        let page = self
            .chunks
            .lock()
            .get(&src.buffer)
            .cloned()
            .expect("copy_buffer: staging allocation was not made by this graph");
        // 暂存区是本帧新分配、GPU 尚未访问：录制期间写入，随后提交即可见。
        page.write_bytes_unsynchronized(u64::from(src.offset), &src.bytes)
            .expect("copy_buffer: staging write failed");
        let copies: Vec<_> = regions
            .iter()
            .map(|r| ash::vk::BufferCopy {
                // core 的 region.src_offset 已含暂存区在页内的偏移（`rg_buffer_copy_from_slice` 取 `staging.offset`）。
                src_offset: r.src_offset,
                dst_offset: r.dst_offset,
                size: src.bytes.len() as u64,
            })
            .collect();
        self.api.copy_buffer(page.handle, dst.handle, &copies);
        self.used.push(page);
    }
}

// ---------------------------------------------------------------- compute pass execution

use country_core::render::pass_builder::compute_pass_builder::{
    ComputeExec, ComputeExecution, ExternalSlot, RecordedComputePass,
};
use country_core::render::pass_builder::pass_resource_access::PassBinding as CorePassBinding;
use rhapsody::raw::{binding::DescriptorSetBinding, descriptor_set::DescriptorWrite};

pub(crate) fn layout_binding_type(
    layout: &rhapsody::raw::descriptor_set::VulkanDescriptorSetLayout,
    binding: u32,
) -> Option<ash::vk::DescriptorType> {
    layout
        .plan
        .bindings
        .iter()
        .find(|b| b.binding == binding)
        .map(|b| ash::vk::DescriptorType::from_raw(b.descriptor_type))
}

/// 核心的视图描述（各字段 `Option`）叠加到纹理的默认视图上。
pub(crate) fn rh_view_desc(
    texture: &rhapsody::raw::texture::Texture,
    view: &country_core::rhapsody_types::TextureViewDesc,
) -> rhapsody::raw::texture::TextureViewDesc {
    let mut out = rhapsody::raw::texture::TextureViewDesc::new(&texture.desc);
    if let Some(v) = view.view_type {
        out.view_type = vk::ImageViewType::from_raw(v as i32);
    }
    if let Some(v) = view.usage {
        out.usage = vk::ImageUsageFlags::from_raw(v);
    }
    if let Some(v) = view.format {
        out.format = vk::Format::from_raw(v as i32);
    }
    if let Some(v) = view.aspect_mask {
        out.aspect = vk::ImageAspectFlags::from_raw(v);
    }
    if let Some(v) = view.level_count {
        out.level_count = v;
    }
    if let Some(v) = view.layer_count {
        out.layer_count = v;
    }
    out.base_mip_level = view.base_mip_level;
    out.base_layer = view.base_layer;
    out
}

struct ComputeExecHost<'a> {
    device: &'a ash::Device,
    command: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    push_stage: vk::ShaderStageFlags,
    local_size: [u32; 3],
    managed: &'a ManagedPassResources,
}

impl ComputeExec for ComputeExecHost<'_> {
    /// `[device+0xaba]` 的调试开关（每次 dispatch 前全管线屏障）；本宿主不开启。
    fn full_barrier_enabled(&self) -> bool {
        false
    }
    fn issue_full_pipeline_barrier(&mut self) {}
    fn count_dispatch(&mut self) {}
    fn cmd_dispatch(&mut self, x: u32, y: u32, z: u32) {
        unsafe { self.device.cmd_dispatch(self.command, x, y, z) };
    }
    fn cmd_dispatch_indirect(&mut self, buffer: u32, offset: u64) -> Option<()> {
        let buffer = self.managed.buffer_for(buffer).ok()?;
        unsafe { self.device.cmd_dispatch_indirect(self.command, buffer.handle, offset) };
        Some(())
    }
    fn local_size(&self) -> [u32; 3] {
        self.local_size
    }
    fn push_constants(&mut self, bytes: &[u8]) {
        unsafe {
            self.device
                .cmd_push_constants(self.command, self.layout, self.push_stage, 0, bytes)
        };
    }
}

impl GraphPasses {
    /// 登记一个已声明完毕的计算 Pass：访问声明进图；回调里绑定管线与描述符集（set 0）、
    /// 推 push constants、按模式 dispatch。`pipeline` 是该 Pass 着色器程序创建出的管线句柄。
    pub fn add_compute(&mut self, pass: RecordedComputePass, pipeline: &ComputePipeline) {
        let tables = self.tables.clone().expect("add_compute needs HostTables (with_tables)");
        let entry = tables
            .shaders
            .compute_entry(pipeline)
            .expect("add_compute: pipeline was not created through this host");
        let RecordedComputePass {
            name,
            builder,
            bindings,
            access_ranges,
            external_bindings,
            uniform_binder,
            push_constants,
            dispatch,
            ..
        } = pass;
        // uniform 块：声明期就写进动态常量页（GPU 尚未访问），回调里只绑定。
        let uniform = uniform_binder.map(|u| {
            let chunk = self
                .dynamic
                .allocate(u.data().len() as u64)
                .expect("uniform chunk allocation failed");
            chunk.write_bytes(u.data()).expect("uniform chunk write failed");
            (chunk.buffer.clone(), chunk.offset, chunk.size)
        });
        let accesses = builder.accesses.clone();
        drop(builder);

        let device = self.device.inner.clone();
        let mut graph = self.graph.borrow_mut();
        let mut declared = graph.add_pass(name);
        for a in &accesses {
            declared
                .add_resource_access(ResourceHandle::from_raw(a.resource), a.access, a.group)
                .expect("add_resource_access: resource does not belong to this graph");
        }
        declared.render_with_context(move |ctx| {
            let managed = ctx.managed.expect("pass callback without managed resources");
            let command = <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());
            let layout0 = entry.pipeline.layout.sets.first();
            let ty_of = |binding: u32, default: vk::DescriptorType| {
                layout0
                    .and_then(|l| layout_binding_type(l, binding))
                    .unwrap_or(default)
            };
            let mut writes: Vec<DescriptorWrite> = Vec::new();
            for range in &access_ranges {
                let value = match bindings[range.access_index as usize] {
                    CorePassBinding::Buffer { resource } | CorePassBinding::BufferRead { resource } => {
                        let buffer = managed.buffer_for(resource).expect("compute binding: unresolved buffer");
                        DescriptorSetBinding::Buffer {
                            ty: ty_of(range.resource_index, vk::DescriptorType::STORAGE_BUFFER),
                            info: vk::DescriptorBufferInfo {
                                buffer: buffer.handle,
                                offset: 0,
                                range: vk::WHOLE_SIZE,
                            },
                            owner: Arc::new(buffer),
                        }
                    }
                    CorePassBinding::ImageView { resource, write, view } => {
                        let texture = managed.texture_for(resource).expect("compute binding: unresolved image");
                        let view = texture
                            .image_view(&rh_view_desc(&texture, &view))
                            .expect("compute binding: image view creation failed");
                        let ty = ty_of(
                            range.resource_index,
                            if write {
                                vk::DescriptorType::STORAGE_IMAGE
                            } else {
                                vk::DescriptorType::SAMPLED_IMAGE
                            },
                        );
                        let layout = if ty == vk::DescriptorType::STORAGE_IMAGE {
                            vk::ImageLayout::GENERAL
                        } else {
                            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                        };
                        DescriptorSetBinding::Image {
                            ty,
                            info: vk::DescriptorImageInfo {
                                sampler: vk::Sampler::null(),
                                image_view: view.raw(),
                                image_layout: layout,
                            },
                            owners: vec![Arc::new(view)],
                        }
                    }
                };
                writes.push(DescriptorWrite { binding: range.resource_index, array_element: 0, value });
            }
            for external in &external_bindings {
                let value = match &external.slot {
                    ExternalSlot::Buffer { vk_buffer, offset, range, handle, .. } => {
                        let owner = tables.buffers.get_by_id(handle.0).expect("external buffer is not live");
                        DescriptorSetBinding::Buffer {
                            ty: ty_of(external.binding, vk::DescriptorType::STORAGE_BUFFER),
                            info: vk::DescriptorBufferInfo {
                                buffer: <vk::Buffer as ash::vk::Handle>::from_raw(*vk_buffer),
                                offset: *offset,
                                range: *range,
                            },
                            owner: Arc::new(owner),
                        }
                    }
                    ExternalSlot::Image { sampler, image_view, layout, handle, .. } => {
                        let owner = tables.textures.get_by_id(handle.0).expect("external texture is not live");
                        DescriptorSetBinding::Image {
                            ty: ty_of(external.binding, vk::DescriptorType::SAMPLED_IMAGE),
                            info: vk::DescriptorImageInfo {
                                sampler: <vk::Sampler as ash::vk::Handle>::from_raw(*sampler),
                                image_view: <vk::ImageView as ash::vk::Handle>::from_raw(*image_view),
                                image_layout: vk::ImageLayout::from_raw(*layout as i32),
                            },
                            owners: vec![Arc::new(owner)],
                        }
                    }
                    ExternalSlot::Other(_) => panic!("external slot kind is not connected"),
                };
                writes.push(DescriptorWrite { binding: external.binding, array_element: 0, value });
            }
            if let (Some((page, offset, size)), Some(binding)) = (&uniform, entry.uniform_binding) {
                writes.push(DescriptorWrite {
                    binding,
                    array_element: 0,
                    value: DescriptorSetBinding::Buffer {
                        ty: ty_of(binding, vk::DescriptorType::UNIFORM_BUFFER),
                        info: vk::DescriptorBufferInfo { buffer: page.handle, offset: *offset, range: *size },
                        owner: page.clone(),
                    },
                });
            }

            // 录制的命令缓冲在回调返回后才提交：管线（及其布局）必须活到 GPU 完成。
            ctx.keep_alive(entry.clone());
            let raw = &device.device;
            unsafe { raw.cmd_bind_pipeline(command, vk::PipelineBindPoint::COMPUTE, entry.pipeline.handle) };
            if let Some(state) = &entry.set0 {
                // SAFETY: 写入的句柄属于本设备，资源由 `owner` 持有并随集合交给命令拥有者保活。
                let set = unsafe { state.allocate_descriptor_set(&writes) }
                    .expect("compute pass: descriptor set allocation failed");
                unsafe {
                    raw.cmd_bind_descriptor_sets(
                        command,
                        vk::PipelineBindPoint::COMPUTE,
                        entry.pipeline.layout.handle,
                        0,
                        &[set.handle],
                        &[],
                    )
                };
                ctx.keep_alive(set);
            }
            let push_stage = entry
                .pipeline
                .layout
                .push_constants
                .first()
                .map_or(vk::ShaderStageFlags::COMPUTE, |r| r.stage_flags);
            let mut exec = ComputeExecHost {
                device: raw,
                command,
                layout: entry.pipeline.layout.handle,
                push_stage,
                local_size: entry.local_size,
                managed,
            };
            ComputeExecution { push_constants, dispatch }.execute(&mut exec);
            if let Some((page, ..)) = uniform {
                ctx.keep_alive(page);
            }
            true
        });
    }
}
