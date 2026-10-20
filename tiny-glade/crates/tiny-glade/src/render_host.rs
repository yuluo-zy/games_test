//! 宿主侧接缝：用 rhapsody 的设备实现 country-core 的 `rhapsody_boundary::GraphicsDevice`。
//!
//! country-core 里的 `Buffer` 是可拷贝的 40 字节值（`id`/`generation` + `byte_size`），对应原版
//! 的托管句柄；这里用槽位表把它映射到 rhapsody 的 [`ManagedBuffer`]，槽位复用时递增代数，
//! 过期句柄按“不属于本设备/已失效”处理。句柄释放见 [`BufferHost::release`]。
use ash::vk;
use country_core::render::ssbo::rhapsody_boundary::{
    BackendError, Buffer, BufferDesc, GraphicsDevice as CoreGraphicsDevice,
};
use parking_lot::Mutex;
use rhapsody::{
    device::Device, resource::BufferDesc as RhBufferDesc, resource::ManagedBuffer,
    resource::MemoryLocation,
};
use std::{ops::Range, sync::Arc};

struct Slot {
    generation: u64,
    buffer: Option<ManagedBuffer>,
}

pub struct BufferHost {
    device: Arc<Device>,
    slots: Mutex<Vec<Slot>>,
    uploads: Mutex<Option<BufferUploads>>,
}
struct BufferUploads {
    manager: rhapsody::transfer::TransferManager,
    epoch: u64,
}

impl BufferHost {
    pub fn graphics_device(&self) -> &Arc<Device> {
        &self.device
    }
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
            slots: Mutex::new(Vec::new()),
            uploads: Mutex::new(None),
        }
    }

    fn lookup(&self, handle: &Buffer) -> Option<ManagedBuffer> {
        let slots = self.slots.lock();
        let slot = slots.get((handle.id as usize).checked_sub(1)?)?;
        (slot.generation == handle.generation)
            .then(|| slot.buffer.clone())
            .flatten()
    }

    /// Resolve a retained core buffer only while its slot generation is live.
    pub fn managed_buffer(&self, handle: &Buffer) -> Option<ManagedBuffer> {
        self.lookup(handle)
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

    /// Return the real description only while the host handle is live.
    pub fn description(&self, handle: &Buffer) -> Option<RhBufferDesc> {
        Some(self.lookup(handle)?.desc)
    }

    /// Diagnostics for the actual bounded device-local sparse update service.
    pub fn queued_upload_stats(&self) -> Option<rhapsody::transfer::UploadQueueStats> {
        self.uploads
            .lock()
            .as_ref()
            .map(|service| service.manager.upload_queue_stats())
    }

    pub fn convert(desc: &BufferDesc) -> rhapsody::Result<RhBufferDesc> {
        let location = MemoryLocation::from_raw(desc.memory)
            .ok_or_else(|| rhapsody::Error::invalid_usage("unknown buffer memory location"))?;
        let alignment = u16::try_from(desc.alignment)
            .map_err(|_| rhapsody::Error::invalid_usage("buffer alignment exceeds u16"))?;
        if alignment != 0 && !alignment.is_power_of_two() {
            return Err(rhapsody::Error::invalid_usage(
                "buffer alignment must be a power of two",
            ));
        }
        // The current shader heap lowers original uniform bindings to storage
        // reads. Keep the core/native descriptor intact and admit that physical
        // use on this host's allocation, including persistent library buffers.
        let mut usage = vk::BufferUsageFlags::from_raw(desc.usage);
        if usage.contains(vk::BufferUsageFlags::UNIFORM_BUFFER) {
            usage |= vk::BufferUsageFlags::STORAGE_BUFFER;
        }
        Ok(RhBufferDesc::new(
            u64::try_from(desc.size)
                .map_err(|_| rhapsody::Error::invalid_usage("buffer size exceeds u64"))?,
            usage,
            location,
        )
        .with_alignment(alignment))
    }
}

impl CoreGraphicsDevice for BufferHost {
    fn release_buffer(&self, buffer: &Buffer) {
        self.release(buffer);
    }

    fn create_buffer(
        &self,
        desc: &BufferDesc,
        name: &str,
        data: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        let managed = self
            .device
            .create_buffer(&Self::convert(desc)?, name, data)?;
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
        let byte_size = managed.size;
        slots[index].buffer = Some(managed);
        Ok(Buffer {
            id,
            generation,
            byte_size,
            _gap10: 0,
            _gap20: 0,
        })
    }

    fn update_buffer_bytes(
        &self,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), BackendError> {
        let managed = self.lookup(buffer).ok_or_else(|| {
            BackendError::invalid_usage("buffer host handle is stale or belongs to another host")
        })?;
        if managed.desc.memory_location != MemoryLocation::GpuOnly
            && managed.persistent_host_ptr().is_some()
        {
            return self
                .device
                .update_buffer_bytes(&managed, data, ranges, offset);
        }
        // Reject invalid ranges before allocating the lazy transfer service.
        rhapsody::resource::plan_buffer_update(&managed.desc, data.len(), ranges, offset as u64)?;
        // This host method promises completion on return. Device-local sparse
        // updates use the same bounded queued uploader as streaming consumers,
        // then wait only for these batches' actual points.
        let mut uploads = self.uploads.lock();
        if uploads.is_none() {
            *uploads = Some(BufferUploads {
                manager: rhapsody::transfer::TransferManager::new_with_queue(
                    self.device.clone(),
                    rhapsody::transfer::TransferConfig {
                        slots: 1,
                        chunk_bytes: 1 << 20,
                        max_bytes: 4 << 20,
                    },
                    rhapsody::transfer::UploadQueueConfig {
                        max_keys: 1024,
                        ..Default::default()
                    },
                )?,
                epoch: 0,
            });
        }
        let service = uploads
            .as_mut()
            .expect("initialized bounded upload service");
        service.epoch = service
            .epoch
            .checked_add(1)
            .ok_or_else(|| BackendError::invalid_usage("buffer upload epoch exhausted"))?;
        service.manager.upload_buffer_ranges_wait(
            managed.shared(),
            data,
            ranges,
            offset as u64,
            rhapsody::transfer::UploadOptions {
                key: rhapsody::transfer::UploadKey(buffer.id),
                epoch: service.epoch,
                priority: rhapsody::transfer::UploadPriority::High,
            },
            u64::MAX,
        )
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
use rhapsody::{
    resource::ManagedTexture, resource::TextureDesc as RhTextureDesc, resource::TextureKind,
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
    if id & TEXTURE_ID_TAG != 0 {
        id
    } else {
        id | BUFFER_TOKEN_TAG
    }
}

struct TextureSlot {
    generation: u64,
    texture: Option<ManagedTexture>,
}

/// core 的 `TextureDevice` 接缝：`Texture.id` 是槽位序号（从 1 起），代数由 `release` 递增。
#[derive(Clone)]
pub struct TextureHost {
    device: Arc<Device>,
    slots: Arc<Mutex<Vec<TextureSlot>>>,
    /// 默认视图缓存（原 `ManagedTexture::view` 按描述缓存，M2.6）：视图随纹理一起存活，`release` 时丢弃。
    views: Arc<Mutex<std::collections::HashMap<u64, rhapsody::resource::ManagedTextureView>>>,
    /// Native explicit subresource views, retained until the texture owner retires.
    view_variants: Arc<
        Mutex<
            std::collections::HashMap<
                (u64, country_core::rhapsody_types::TextureViewDesc),
                rhapsody::resource::ManagedTextureView,
            >,
        >,
    >,
    /// 宿主附加的用途位（core 的 `TextureDesc` 只有 sampled/storage/附件五个开关，没有 TRANSFER_SRC；
    /// 读回像素的宿主/测试在这里统一加上。原版 `TextureDesc.usage` 是完整位集，此项是宿主策略，不是原版行为）。
    extra_usage: vk::ImageUsageFlags,
}

impl TextureHost {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
            slots: Arc::new(Mutex::new(Vec::new())),
            views: Arc::new(Mutex::new(Default::default())),
            view_variants: Arc::new(Mutex::new(Default::default())),
            extra_usage: vk::ImageUsageFlags::empty(),
        }
    }

    /// 之后创建的所有纹理（含图内创建的）额外带上这些用途位。
    pub fn with_extra_usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.extra_usage = usage;
        self
    }

    /// `convert` + 宿主附加用途位（图内创建纹理与 `create_texture` 共用）。
    pub fn convert_for_graph(
        &self,
        desc: &TextureDesc,
    ) -> Result<RhTextureDesc, TextureBackendError> {
        let mut out = Self::convert(desc)?;
        out.usage |= self.extra_usage;
        Ok(out)
    }

    /// 默认全范围视图的原生句柄（首次创建后缓存）。
    pub fn default_view(&self, id: u64) -> rhapsody::Result<u64> {
        if self.device.is_device_lost() {
            return Err(rhapsody::Error::DeviceLost);
        }
        let texture = self
            .get_by_id(id)
            .ok_or_else(|| rhapsody::Error::invalid_usage("texture host handle is not live"))?;
        let mut views = self.views.lock();
        if let Some(view) = views.get(&id) {
            return Ok(ash::vk::Handle::as_raw(view.raw()));
        }
        let desc = rhapsody::resource::TextureViewDesc::new(&texture.desc)?;
        let view = self.device.get_vk_image_view(&texture, &desc)?;
        let raw = ash::vk::Handle::as_raw(view.raw());
        views.insert(id, view);
        Ok(raw)
    }

    /// Select the native disk texture subresource with the recovered descriptor.
    pub fn explicit_view(
        &self,
        id: u64,
        desc: &country_core::rhapsody_types::TextureViewDesc,
    ) -> rhapsody::Result<u64> {
        let texture = self
            .get_by_id(id)
            .ok_or_else(|| rhapsody::Error::invalid_usage("texture host handle is not live"))?;
        let mut variants = self.view_variants.lock();
        if let Some(view) = variants.get(&(id, *desc)) {
            return Ok(ash::vk::Handle::as_raw(view.raw()));
        }
        let native = rh_view_desc(&texture, desc)?;
        let view = self.device.get_vk_image_view(&texture, &native)?;
        let raw = ash::vk::Handle::as_raw(view.raw());
        variants.insert((id, *desc), view);
        Ok(raw)
    }

    pub(crate) fn owned_view(
        &self,
        id: u64,
        raw: u64,
    ) -> rhapsody::Result<Arc<rhapsody::resource::TextureView>> {
        if let Some(view) = self.views.lock().get(&id) {
            if ash::vk::Handle::as_raw(view.raw()) == raw {
                return Ok(view.view.clone());
            }
        }
        self.view_variants
            .lock()
            .iter()
            .find(|((owner, _), view)| *owner == id && ash::vk::Handle::as_raw(view.raw()) == raw)
            .map(|(_, view)| view.view.clone())
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("external texture view is not owned by this host")
            })
    }

    /// rhapsody 描述 → core 描述（`managed_texture_desc`）。
    pub fn core_desc(texture: &ManagedTexture) -> TextureDesc {
        let d = &texture.desc;
        TextureDesc {
            flags: d.flags.as_raw(),
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
        slot.texture
            .clone()
            .filter(|_| slot.generation == texture.generation())
    }

    /// 按 `Texture.id`（含标记与代数）取，代数不符返回 `None`。
    pub fn get_by_id(&self, id: u64) -> Option<ManagedTexture> {
        let slots = self.slots.lock();
        let slot = slots.get(((id & 0xffff_ffff) as usize).checked_sub(1)?)?;
        let generation = (id >> 32) & 0x7fff_ffff;
        slot.texture
            .clone()
            .filter(|_| slot.generation == generation)
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
        self.view_variants
            .lock()
            .retain(|(id, _), _| *id != texture.id);
        true
    }

    /// core 描述 → rhapsody 原布局描述；`kind` 非法时报错而不是猜。
    pub fn convert(desc: &TextureDesc) -> Result<RhTextureDesc, TextureBackendError> {
        let kind = TextureKind::from_raw(desc.kind).ok_or_else(|| {
            TextureBackendError::invalid_usage(format!("unknown texture kind {}", desc.kind))
        })?;
        let extent = vk::Extent3D {
            width: desc.extent[0],
            height: desc.extent[1],
            depth: desc.extent[2],
        };
        let mut out = RhTextureDesc::new(extent, kind, vk::Format::from_raw(desc.format as i32))
            .mip_levels(desc.mip_levels.max(1))
            .array_elements(u32::from(desc.array_elements).max(1))?;
        out.flags = vk::ImageCreateFlags::from_raw(desc.flags);
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
    fn release_texture(&self, texture: &Texture) {
        self.release(texture);
    }
    fn create_texture(
        &self,
        name: &str,
        desc: &TextureDesc,
        data: &[u8],
    ) -> Result<Texture, TextureBackendError> {
        let rh = self.convert_for_graph(desc)?;
        let managed = if data.is_empty() && rh.usage.contains(vk::ImageUsageFlags::SAMPLED) {
            self.device.create_texture_zeroed(&rh, name)?
        } else {
            self.device.create_texture(&rh, name, data)?
        };
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

impl country_core::asset_libraries::texture_library::TextureLibraryDevice for TextureHost {
    fn create_persistent_descriptor_set(
        &self,
        _name: &str,
    ) -> rhapsody::Result<country_core::rhapsody_contract::DescriptorSet> {
        use country_core::asset_libraries::texture_library::{
            REQUIRED_BINDLESS_DESCRIPTOR_COUNT, RESERVED_DESCRIPTOR_COUNT,
        };
        let mut sampled_capacity = self
            .device
            .max_bindless_descriptor_count()
            .min(REQUIRED_BINDLESS_DESCRIPTOR_COUNT - RESERVED_DESCRIPTOR_COUNT);
        loop {
            match rhapsody::bindless::BindlessHeap::new(
                self.device.clone(),
                rhapsody::bindless::BindlessConfig {
                    sampled_images: sampled_capacity,
                    storage_images: 1,
                    storage_buffers: 1,
                    samplers: 1,
                },
            ) {
                Ok(heap) => {
                    let host = self.clone();
                    return Ok(country_core::rhapsody_contract::DescriptorSet::from_heap(
                        heap,
                        sampled_capacity,
                        move |texture| {
                            let raw = country_core::asset_libraries::texture_library::TextureLibraryDevice::get_vk_image_view(&host, texture, 0)?;
                            host.owned_view(texture.id, raw.0)
                        },
                    ));
                }
                Err(error)
                    if error.kind() == rhapsody::ErrorKind::Unsupported && sampled_capacity > 1 =>
                {
                    // 复用后端的真实布局支持检查，逐步收缩到设备可用容量。
                    sampled_capacity /= 2;
                }
                Err(error) => return Err(error),
            }
        }
    }

    fn update_persistent_descriptor_set(
        &self,
        descriptors: &country_core::rhapsody_contract::DescriptorSet,
        updates: &[country_core::asset_libraries::texture_library::DescriptorImageUpdate],
    ) -> rhapsody::Result<()> {
        for update in updates {
            if update.binding != 0
                || update.layout != vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL.as_raw() as u64
            {
                return Err(rhapsody::Error::invalid_usage("纹理库只接受只读采样描述符"));
            }
            // 使用资产 ID 查表并核对视图，避免把任意整数作为 Vulkan 句柄。
            self.get_by_id(update.texture_id)
                .ok_or_else(|| rhapsody::Error::invalid_usage("纹理已释放"))?;
            let view = self.owned_view(update.texture_id, update.image_view.0)?;
            descriptors.validate_sampled_image(update.index.0, &view)?;
        }
        Ok(())
    }
    fn get_vk_image_view(
        &self,
        texture: &Texture,
        mip: u32,
    ) -> rhapsody::Result<country_core::asset_libraries::texture_library::ImageView> {
        if mip != 0 || self.get(texture).is_none() {
            return Err(rhapsody::Error::invalid_usage("纹理或 mip 无效"));
        }
        self.default_view(texture.id)
            .map(country_core::asset_libraries::texture_library::ImageView)
    }
    fn update_texture(
        &self,
        texture: &Texture,
        update: &country_core::asset_libraries::texture_library::TextureUpdate,
    ) -> rhapsody::Result<()> {
        let managed = self
            .get(texture)
            .ok_or_else(|| rhapsody::Error::invalid_usage("纹理已释放"))?;
        if update.buffer_offset != 0 || update.array_layers == 0 {
            return Err(rhapsody::Error::unsupported(
                "纹理更新要求零偏移和非零数组层数",
            ));
        }
        let layer_count = u32::try_from(update.array_layers)
            .map_err(|_| rhapsody::Error::invalid_usage("纹理层数溢出"))?;
        if update.data.len() % layer_count as usize != 0 {
            return Err(rhapsody::Error::invalid_usage("纹理数据层长度不一致"));
        }
        let bytes_per_layer = update.data.len() / layer_count as usize;
        let regions: Vec<_> = (0..layer_count)
            .map(|layer| {
                let start = layer as usize * bytes_per_layer;
                rhapsody::resource::TextureUpdateRegion {
                    data: &update.data[start..start + bytes_per_layer],
                    extent: update.extent,
                    mip_level: update.mip_level,
                    array_layer: layer,
                }
            })
            .collect();
        rhapsody::resource::update_texture_regions(&self.device, &managed, &regions)
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

use country_core::render::pass_builder::{
    compute_pass_builder::{ComputePassDevice, ResourceRegistry},
    uniform_binder::{UniformBlockInfo, UniformReflection},
};
use country_core::render::shader::boundary::{
    CompiledShaderPipeline as CoreCompiled, ComputePipeline, ComputePipelineDesc,
    Error as ShaderError, RasterPipeline, RasterPipelineDesc, RasterPipelineKey, ShaderBackend,
};
use rhapsody::bindless::BindlessShaderAbi as BindlessAbi;
use rhapsody::bindless::{
    BindlessConfig, BindlessHeap, BindlessLease, BindlessParameters, BindlessResource,
    BindlessShaderPackage,
};
use rhapsody::pipeline::{
    AsyncPipelineCompiler, CompilerConfig, PersistentPipelineCache, PipelineTicket,
};
use rhapsody::rg::RenderPassKey;
use rhapsody::{
    pipeline::DynamicRenderingFormats, pipeline::PipelineCache,
    pipeline::RasterPipelineDesc as RhRasterDesc, pipeline::RasterPipelineOpts,
    pipeline::ShaderModule, pipeline::VulkanPipeline, pipeline::vertex_input_from_reflection,
    resource::Sampler,
};
use shader_pipeline::{
    compiled::CompiledShaderPipeline as RhCompiled, compiled::ShaderStage,
    pipeline_desc::ComputePipelineDesc as FileComputeDesc, shader_defines::ShaderDefines,
};
use shader_pipeline::{
    reflection::BindingKind, reflection::BindingLocation, reflection::ShaderReflection,
};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    time::{Duration, Instant},
};

/// 发行 .bin 里存的反射 → core 的 `ResourceRegistry`：名字哈希 → (set, binding)、uniform 块大小与所在 set、
/// uniform 字段名哈希 → 偏移。`UniformBlockInfo.set_index` 取块绑定位置的 set（原偏移 `+0x130` 的 u32，字段名未证实）。
pub fn registry_from_reflection(reflection: &ShaderReflection) -> ResourceRegistry {
    ResourceRegistry {
        bindings: reflection
            .binding_name_hashes
            .iter()
            .map(|(hash, location)| (*hash, (location.set, location.binding)))
            .collect(),
        uniform_block: reflection
            .uniform_block
            .as_ref()
            .map(|block| UniformBlockInfo {
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

/// core 的 `ShaderBackend` 接缝，创建计算/光栅管线及其共享 bindless ABI。
///
/// 发行 .bin 的路径由 shader-pipeline 的 `compiled_file_path` 给出（相对 `root`），解码、反射布局、
/// 描述符/管线布局和管线创建全部走 rhapsody。Sampler 先取宿主覆盖，随后按反射名字查
/// rhapsody 已恢复的内置采样器表；未知名字保留设备返回的错误。
pub struct ShaderHost {
    device: Arc<Device>,
    root: PathBuf,
    compiler: AsyncPipelineCompiler,
    disk_cache: Option<PersistentPipelineCache>,
    samplers: BTreeMap<String, Arc<Sampler>>,
    heap: Arc<BindlessHeap>,
    sampler_leases: Mutex<BTreeMap<String, Arc<BindlessLease>>>,
    compute: Mutex<Vec<Arc<ComputeEntry>>>,
    raster: Mutex<Vec<Arc<RasterEntry>>>,
    passes: Arc<RenderPassHost>,
    frame_globals: Arc<Mutex<Option<Arc<crate::frame_setup::FrameGlobals>>>>,
    indexed_count_adapter: Mutex<Option<Arc<rhapsody::rg::IndexedCountAdapter>>>,
}

/// Bounded attachment signatures used to request dynamic-rendering pipelines.
#[derive(Default)]
pub struct RenderPassHost {
    table: Mutex<Vec<Arc<RenderPassEntry>>>,
    by_desc: Mutex<std::collections::HashMap<rhapsody::rg::RenderPassDesc, u64>>,
}

pub struct RenderPassEntry {
    pub formats: DynamicRenderingFormats,
    pub color_attachments: usize,
    pub samples: vk::SampleCountFlags,
}

impl RenderPassHost {
    /// Resolve a complete attachment signature without creating native render objects.
    pub fn create_from_desc(
        &self,
        device: &Arc<Device>,
        desc: &rhapsody::rg::RenderPassDesc,
    ) -> Result<u64, rhapsody::Error> {
        let mut by_desc = self.by_desc.lock();
        if let Some(id) = by_desc.get(desc) {
            return Ok(*id);
        }
        let mut attachments = desc
            .colors
            .as_slice()
            .iter()
            .map(|color| color.to_attachment_key())
            .collect::<Vec<_>>();
        if let Some((depth, access)) = desc.depth {
            let mut attachment = depth.to_attachment_key();
            attachment.initial_layout = access.layout().as_raw();
            attachment.final_layout = access.layout().as_raw();
            attachment.subpass_layout = access.layout().as_raw();
            attachments.push(attachment);
        }
        let entry = Self::entry(device, &RenderPassKey { attachments })?;
        let mut table = self.table.lock();
        if table.len() >= 256 {
            return Err(rhapsody::Error::Timeout);
        }
        table.push(Arc::new(entry));
        let id = table.len() as u64;
        by_desc.insert(*desc, id);
        Ok(id)
    }

    fn entry(device: &Arc<Device>, key: &RenderPassKey) -> rhapsody::Result<RenderPassEntry> {
        let first = key
            .attachments
            .first()
            .ok_or_else(|| rhapsody::Error::invalid_usage("raster target has no attachments"))?;
        let samples = vk::SampleCountFlags::from_raw(first.samples);
        if !first.samples.is_power_of_two()
            || key.attachments.iter().any(|a| a.samples != first.samples)
            || key.attachments.iter().filter(|a| a.depth_stencil).count() > 1
        {
            return Err(rhapsody::Error::invalid_usage(
                "raster target sample/depth contract is inconsistent",
            ));
        }
        let mut formats = DynamicRenderingFormats {
            colors: Vec::new(),
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            view_mask: 0,
        };
        for attachment in &key.attachments {
            let format = vk::Format::from_raw(attachment.format);
            if attachment.depth_stencil {
                let aspects = rhapsody::resource::vk_format_valid_aspect_flags(format);
                if aspects.contains(vk::ImageAspectFlags::DEPTH) {
                    formats.depth = format;
                }
                if aspects.contains(vk::ImageAspectFlags::STENCIL) {
                    formats.stencil = format;
                }
            } else {
                formats.colors.push(format);
            }
        }
        device.caps().require_all_enabled(
            rhapsody::device::FeatureSet::EMPTY.with(rhapsody::device::Feature::DynamicRendering),
        )?;
        Ok(RenderPassEntry {
            color_attachments: formats.colors.len(),
            formats,
            samples,
        })
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
        device: &Arc<Device>,
        key: &RenderPassKey,
    ) -> Result<u64, rhapsody::Error> {
        let entry = Self::entry(device, key)?;
        let mut table = self.table.lock();
        if table.len() >= 256 {
            return Err(rhapsody::Error::Timeout);
        }
        table.push(Arc::new(entry));
        Ok(table.len() as u64)
    }

    pub fn get(&self, id: u64) -> Option<Arc<RenderPassEntry>> {
        self.table
            .lock()
            .get((id as usize).checked_sub(1)?)
            .cloned()
    }
}

/// 一条已创建的光栅管线及绘制时需要的反射信息。
pub struct RasterEntry {
    pub pipeline: PipelineTicket,
    pub layout: Arc<rhapsody::pipeline::PipelineLayout>,
    pub registry: Arc<ResourceRegistry>,
    pub pass: Arc<RenderPassEntry>,
    pub bindings: ShaderBindings,
    pub uniform_binding: Option<u32>,
    /// Exact names and locations from the same original compiled reflection
    /// used by vertex_input_from_reflection to build this Vulkan pipeline.
    /// `(属性名, location, 顶点步长字节)`；步长取反射类型 `nbyte`，与管线 binding stride 一致。
    pub vertex_inputs: Vec<(String, u32, u32)>,
}

/// 一条已创建的计算管线及执行它所需的反射信息。
pub struct ComputeEntry {
    pub pipeline: PipelineTicket,
    pub registry: Arc<ResourceRegistry>,
    /// 着色器 `local_size`（反射 `local_size`，缺省 1×1×1）。
    pub local_size: [u32; 3],
    pub bindings: ShaderBindings,
    /// uniform 块所在的 binding 号（反射 `uniform_block.binding.binding`）。
    pub uniform_binding: Option<u32>,
}

pub struct ShaderBindings {
    pub heap: Arc<BindlessHeap>,
    pub abi: Arc<BindlessAbi>,
    samplers: Vec<(BindingLocation, u32, Arc<BindlessLease>)>,
    frame_globals: Arc<Mutex<Option<Arc<crate::frame_setup::FrameGlobals>>>>,
}
impl ShaderBindings {
    /// Owner and range of the FrameConstants block behind set 1/b1.
    pub fn frame_storage_view(
        &self,
    ) -> rhapsody::Result<(Arc<rhapsody::resource::Buffer>, u64, u64)> {
        self.frame_globals
            .lock()
            .as_ref()
            .map(|globals| globals.storage_view())
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage(
                    "frame constants storage binding requires the frame globals",
                )
            })
    }
    /// 全局纹理表（set 2/b0）本帧快照的视图；无帧全局时为空。
    pub fn frame_sampled_table(&self) -> Vec<Arc<rhapsody::resource::TextureView>> {
        self.frame_globals
            .lock()
            .as_ref()
            .map(|globals| {
                globals
                    .sampled_table()
                    .iter()
                    .map(|(_, view)| view.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
    pub fn parameters(&self, push: &[u8]) -> rhapsody::Result<BindlessParameters> {
        let mut parameters = BindlessParameters::new(self.heap.clone(), self.abi.clone(), push)?;
        for (location, element, lease) in &self.samplers {
            parameters.write_lease(*location, *element, lease.clone())?;
        }
        if let Some(globals) = &*self.frame_globals.lock() {
            globals.write_to(&mut parameters, &self.abi, &self.heap)?;
        }
        Ok(parameters)
    }
    pub fn image(
        &self,
        binding: u32,
        view: Arc<rhapsody::resource::TextureView>,
    ) -> rhapsody::Result<BindlessResource> {
        let layout = match self
            .abi
            .slot(BindingLocation { set: 0, binding })
            .map(|slot| slot.kind)
        {
            Some(BindingKind::StorageImage) => vk::ImageLayout::GENERAL,
            Some(BindingKind::SampledImage) => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            _ => {
                return Err(rhapsody::Error::invalid_usage(
                    "image binding differs from shader ABI",
                ));
            }
        };
        self.image_with_layout(binding, view, layout)
    }

    fn image_with_layout(
        &self,
        binding: u32,
        view: Arc<rhapsody::resource::TextureView>,
        layout: vk::ImageLayout,
    ) -> rhapsody::Result<BindlessResource> {
        let kind = self
            .abi
            .slot(BindingLocation { set: 0, binding })
            .map(|slot| slot.kind)
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("image binding is absent from shader ABI")
            })?;
        validate_image_binding_layout(kind, layout)?;
        match kind {
            BindingKind::StorageImage => Ok(BindlessResource::StorageImage(view)),
            BindingKind::SampledImage => Ok(BindlessResource::SampledImage { view, layout }),
            _ => unreachable!("validated image ABI kind"),
        }
    }

    fn image_for_pass(
        &self,
        ctx: &rhapsody::rg::PassContext<'_>,
        resource: u32,
        binding: u32,
        view: Arc<rhapsody::resource::TextureView>,
    ) -> rhapsody::Result<BindlessResource> {
        let selected = view.desc;
        let layout = ctx.image_layout_for_range(
            resource,
            rhapsody::rg::AccessRange::Texture {
                aspect_mask: selected.aspect.as_raw(),
                base_mip_level: selected.base_mip_level,
                level_count: selected.level_count,
                base_array_layer: selected.base_layer,
                layer_count: selected.layer_count,
            },
        )?;
        self.image_with_layout(binding, view, layout)
    }
}

fn validate_image_binding_layout(
    kind: BindingKind,
    layout: vk::ImageLayout,
) -> rhapsody::Result<()> {
    let valid = match kind {
        BindingKind::SampledImage => matches!(
            layout,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL | vk::ImageLayout::GENERAL
        ),
        BindingKind::StorageImage => layout == vk::ImageLayout::GENERAL,
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(rhapsody::Error::invalid_usage(
            "planned image layout differs from shader ABI",
        ))
    }
}

impl ShaderHost {
    pub fn new(device: Arc<Device>, root: impl Into<PathBuf>) -> rhapsody::Result<Self> {
        Self::new_with_pipeline_cache(device, root, None)
    }
    /// Pipeline compilation runs on bounded workers. No frame callback waits.
    pub fn new_with_pipeline_cache(
        device: Arc<Device>,
        root: impl Into<PathBuf>,
        cache_path: Option<PathBuf>,
    ) -> rhapsody::Result<Self> {
        let disk_cache = cache_path
            .map(|path| PersistentPipelineCache::open(device.clone(), path, 256))
            .transpose()?;
        let cache = match &disk_cache {
            Some(disk) => disk.cache.clone(),
            None => Arc::new(PipelineCache::new(device.clone(), &[])?),
        };
        let compiler = AsyncPipelineCompiler::new(cache, CompilerConfig::default())?;
        // A complete original Nani round retains immutable parameter blocks
        // for hundreds of sublist dispatches until its submission completes.
        // The library's small default (256 storage descriptors) exhausted in
        // the actual fill-unsorted-indirection pass. This is an explicit host
        // budget, not an original game constant or a change to shader ABI;
        // it adapts to the enabled device's descriptor-indexing limits.
        let heap = BindlessHeap::new(
            device.clone(),
            adaptive_bindless_config(&device),
        )?;
        Ok(Self {
            device,
            root: root.into(),
            compiler,
            disk_cache,
            samplers: BTreeMap::new(),
            heap,
            sampler_leases: Mutex::new(BTreeMap::new()),
            compute: Mutex::new(Vec::new()),
            raster: Mutex::new(Vec::new()),
            passes: Arc::new(RenderPassHost::default()),
            frame_globals: Arc::new(Mutex::new(None)),
            indexed_count_adapter: Mutex::new(None),
        })
    }
    /// Uses this host's existing heap/compiler. Device support policy is
    /// selected by the raster host; no second descriptor backend is created.
    pub fn indexed_count_adapter(
        &self,
    ) -> rhapsody::Result<Arc<rhapsody::rg::IndexedCountAdapter>> {
        let mut cached = self.indexed_count_adapter.lock();
        if let Some(adapter) = cached.as_ref() {
            return Ok(adapter.clone());
        }
        let adapter = rhapsody::rg::IndexedCountAdapter::new(
            self.device.clone(),
            self.heap.clone(),
            &self.compiler,
        )?;
        *cached = Some(adapter.clone());
        Ok(adapter)
    }
    pub fn set_frame_globals(&self, globals: Arc<crate::frame_setup::FrameGlobals>) {
        *self.frame_globals.lock() = Some(globals);
    }
    /// Explicit startup/prewarm operation. Rendering uses ticket.try_get().
    pub fn prewarm(&self, timeout: Duration) -> rhapsody::Result<()> {
        let deadline = Instant::now()
            .checked_add(timeout)
            .ok_or(rhapsody::Error::Timeout)?;
        let mut tickets = self
            .compute
            .lock()
            .iter()
            .map(|entry| entry.pipeline.clone())
            .collect::<Vec<_>>();
        tickets.extend(
            self.raster
                .lock()
                .iter()
                .map(|entry| entry.pipeline.clone()),
        );
        if let Some(adapter) = self.indexed_count_adapter.lock().as_ref() {
            tickets.push(adapter.ticket());
        }
        for ticket in tickets {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .ok_or(rhapsody::Error::Timeout)?;
            ticket.wait(remaining).map_err(rhapsody::Error::from)?;
        }
        Ok(())
    }
    pub fn save_pipeline_cache(&self) -> rhapsody::Result<()> {
        self.disk_cache
            .as_ref()
            .map_or(Ok(()), PersistentPipelineCache::save)
    }
    pub fn compiler_stats(&self) -> rhapsody::pipeline::CompilerStats {
        self.compiler.stats()
    }

    pub fn passes(&self) -> &Arc<RenderPassHost> {
        &self.passes
    }

    pub fn raster_entry(&self, handle: &RasterPipeline) -> Option<Arc<RasterEntry>> {
        self.raster
            .lock()
            .get((handle.id as usize).checked_sub(1)?)
            .cloned()
    }

    pub fn with_sampler(mut self, name: &str, sampler: Arc<Sampler>) -> Self {
        self.samplers.insert(name.to_owned(), sampler);
        self.sampler_leases.get_mut().remove(name);
        self
    }

    pub fn compute_pipeline(&self, handle: &ComputePipeline) -> Option<Arc<VulkanPipeline>> {
        self.compute_entry(handle)
            .and_then(|e| e.pipeline.try_get().ok())
    }

    pub fn compute_entry(&self, handle: &ComputePipeline) -> Option<Arc<ComputeEntry>> {
        self.compute
            .lock()
            .get((handle.id as usize).checked_sub(1)?)
            .cloned()
    }

    /// 该管线的反射登记表（`ComputePass::new` 里 `[arc+0x10]` 一族偏移读到的那份）。
    pub fn compute_registry(&self, handle: &ComputePipeline) -> Option<Arc<ResourceRegistry>> {
        self.compute_entry(handle).map(|e| e.registry.clone())
    }

    pub fn bindless_stats(&self) -> rhapsody::bindless::BindlessStats {
        self.heap.stats()
    }
    fn bindings(
        &self,
        original: &RhCompiled,
        abi: Arc<BindlessAbi>,
    ) -> rhapsody::Result<ShaderBindings> {
        let mut samplers = Vec::new();
        let mut cached = self.sampler_leases.lock();
        for slot in &abi.bindings {
            if slot.kind != BindingKind::Sampler {
                continue;
            }
            let name = original
                .reflection
                .descriptor_sets
                .get(slot.location.set as usize)
                .and_then(|set| {
                    set.iter()
                        .rev()
                        .find(|(binding, _)| *binding == slot.location.binding)
                })
                .map(|(_, binding)| binding.name.as_str())
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "sampler location absent from original reflection",
                    )
                })?;
            let lease = if let Some(lease) = cached.get(name) {
                lease.clone()
            } else {
                let sampler = match self.samplers.get(name) {
                    Some(sampler) => sampler.clone(),
                    None => self.device.get_built_in_sampler_by_name(name)?,
                };
                let lease = self.heap.sampler(sampler)?;
                cached.insert(name.to_owned(), lease.clone());
                lease
            };
            samplers
                .try_reserve(slot.parameter_count as usize)
                .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
            for element in 0..slot.parameter_count {
                samplers.push((slot.location, element, lease.clone()));
            }
        }
        Ok(ShaderBindings {
            heap: self.heap.clone(),
            abi,
            samplers,
            frame_globals: self.frame_globals.clone(),
        })
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
    fn load_compiled_compute(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<CoreCompiled, ShaderError> {
        self.read(Self::file_desc(desc).compiled_file_path())
    }

    fn load_compiled_raster(&self, desc: &RasterPipelineDesc) -> Result<CoreCompiled, ShaderError> {
        let file = shader_pipeline::pipeline_desc::RasterPipelineDesc {
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
            variant
                .defines
                .push(("SHADER_VARIANT_DEPTH_ONLY".to_owned(), value.to_owned()));
            variant
        };
        [with("0"), with("1")]
    }

    fn create_compute_pipeline(
        &self,
        name: &str,
        compiled: Arc<CoreCompiled>,
    ) -> rhapsody::Result<ComputePipeline> {
        if self.compute.lock().len() >= 4096 {
            return Err(rhapsody::Error::Timeout);
        }
        let decoded = RhCompiled::decode(&compiled.0).map_err(|error| {
            rhapsody::Error::invalid_usage(format!("{name}: compiled shader decode: {error}"))
        })?;
        let package = BindlessShaderPackage::new(&self.heap, &decoded, 64)?;
        let stage = package
            .stages
            .iter()
            .find(|stage| stage.stage == ShaderStage::Compute)
            .ok_or_else(|| rhapsody::Error::invalid_usage("no compute stage in shader package"))?;
        let bytes = stage
            .spirv
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect::<Vec<_>>();
        let entry = std::ffi::CString::new(stage.entry_point.as_str())
            .map_err(|error| rhapsody::Error::invalid_usage_with("compute entry point", error))?;
        let bindings = self.bindings(&decoded, package.abi)?;
        let pipeline = self
            .compiler
            .request_compute_spirv(bytes, entry, package.layout)?;
        let entry = Arc::new(ComputeEntry {
            pipeline,
            registry: Arc::new(registry_from_reflection(&decoded.reflection)),
            local_size: decoded.reflection.local_size.unwrap_or([1, 1, 1]),
            bindings,
            uniform_binding: decoded
                .reflection
                .uniform_block
                .as_ref()
                .map(|block| block.binding.binding),
        });
        let mut table = self.compute.lock();
        if table.len() >= 4096 {
            return Err(rhapsody::Error::Timeout);
        }
        table
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        table.push(entry);
        Ok(ComputePipeline {
            id: table.len() as u64,
        })
    }

    fn create_raster_pipeline(
        &self,
        compiled: Arc<CoreCompiled>,
        key: &RasterPipelineKey,
    ) -> Result<RasterPipeline, ShaderError> {
        if self.raster.lock().len() >= 4096 {
            return Err(rhapsody::Error::Timeout.into());
        }
        // O 0x1416a3720：key.handle 是渲染通道句柄，key.selector 是 4 字节 `RasterPipelineOpts`。
        let pass = self.passes.get(key.handle).ok_or_else(|| {
            anyhow::anyhow!(
                "raster pipeline: render pass handle {} is not live",
                key.handle
            )
        })?;
        let decoded = RhCompiled::decode(&compiled.0)?;
        let package = BindlessShaderPackage::new(&self.heap, &decoded, 64)?;
        // The native Egui producer stores its actual 8-byte screen extent in
        // DrawPacket.unknown_bytes (B 0x1400def7e..0x1400defd0). Preserve the
        // package's reflected push ABI here; RasterHost forwards the packet's
        // bytes to ShaderBindings::parameters, and BindlessParameters::new
        // rejects any byte count differing from abi.original_push_bytes before
        // recording a draw. A pipeline with a push block is not itself missing
        // a producer; a draw with missing/wrong bytes still fails precisely.
        let stage_of = |wanted: ShaderStage| {
            package
                .stages
                .iter()
                .find(|s| s.stage == wanted)
                .ok_or_else(|| anyhow::anyhow!("raster pipeline: missing {wanted:?} stage"))
        };
        let vertex = stage_of(ShaderStage::Vertex)?;
        let fragment = stage_of(ShaderStage::Fragment)?;

        let layout = package.layout.clone();
        let bindings = self.bindings(&decoded, package.abi.clone())?;
        let module = |stage: &shader_pipeline::compiled::CompiledShaderStage| {
            let bytes = stage
                .spirv
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>();
            ShaderModule::from_spirv(self.device.clone(), &bytes).map_err(ShaderError::from)
        };
        let mut desc = RhRasterDesc::new(module(vertex)?, layout);
        desc.fragment = Some(module(fragment)?);
        desc.entry = std::ffi::CString::new(vertex.entry_point.as_str())?;
        desc.fragment_entry = Some(std::ffi::CString::new(fragment.entry_point.as_str())?);
        desc.dynamic_rendering = Some(pass.formats.clone());
        desc.samples = pass.samples;
        let (vertex_bindings, attributes) = vertex_input_from_reflection(&decoded.reflection)
            .map_err(|e| anyhow::anyhow!("raster vertex input: {e:?}"))?;
        desc.vertex_bindings = vertex_bindings;
        desc.vertex_attributes = attributes;
        RasterPipelineOpts(key.selector).apply_to(&mut desc, pass.color_attachments)?;

        let layout = desc.layout.clone();
        let pipeline = self.compiler.request_raster(desc)?;
        let entry = Arc::new(RasterEntry {
            pipeline,
            layout,
            registry: Arc::new(registry_from_reflection(&decoded.reflection)),
            pass,
            bindings,
            vertex_inputs: decoded
                .reflection
                .effective_vertex_inputs()
                .into_iter()
                .map(|(name, input)| {
                    let stride = input
                        .ty
                        .nbyte()
                        .and_then(|bytes| u32::try_from(bytes).ok())
                        .ok_or_else(|| {
                            anyhow::anyhow!("vertex input {name} has no byte size")
                        })?;
                    Ok((name.to_owned(), input.location, stride))
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
            uniform_binding: decoded
                .reflection
                .uniform_block
                .as_ref()
                .map(|b| b.binding.binding),
        });
        let mut table = self.raster.lock();
        if table.len() >= 4096 {
            return Err(rhapsody::Error::Timeout.into());
        }
        table
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        table.push(entry);
        Ok(RasterPipeline {
            id: table.len() as u64,
            compiled,
        })
    }
}

// ---------------------------------------------------------------- render graph

use country_core::rhapsody_types::{BufferGraphBackend, ImportedBuffer};
use rhapsody::{rg::RenderGraph as RhGraph, rg::ResourceSpec};
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
        Self {
            graph,
            buffers,
            textures,
        }
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
            .import_unowned(
                format!("imported-{id:x}"),
                descriptor,
                graph_token(id),
                access,
            )
            .expect("import_unowned: graph rejected the resource");
        ImportedBuffer {
            resource: u64::from(handle.raw()),
            generation: 0,
            access: u32::from(access),
        }
    }

    fn export(&mut self, imported: ImportedBuffer, access: u8) {
        let mut graph = self.graph.borrow_mut();
        let handle = graph
            .resource_handle(imported.resource as u32)
            .expect("export: handle does not belong to this graph");
        graph
            .export(handle, access)
            .expect("export: handle does not belong to this graph");
    }
}

// ---------------------------------------------------------------- compute pass device

use country_core::{render::shader::ComputeShaderProgram, rhapsody_contract::ManagedHandle};

/// core `ComputePassDevice` 的宿主实现：把缓冲/纹理/着色器三张表合在一起。
pub struct DeviceHost {
    pub buffers: Arc<BufferHost>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
}

impl TextureDevice for DeviceHost {
    fn release_texture(&self, texture: &Texture) {
        self.textures.release(texture);
    }
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
        self.textures.default_view(handle.0)
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
    RenderFn, RenderGraph as CorePassGraph, RenderPassApi as CoreRenderPassApi, ResourceLease,
    StagingAlloc, StagingCopy,
};
use rhapsody::{
    resource::DynamicConstants, resource::DynamicConstantsChunk, rg::AshCommandBackend,
    rg::CommandBackend, rg::ManagedPassResources, rg::RenderPassApi as RhRenderPassApi,
};
/// core `rg_boundary::RenderGraph` 的宿主实现：pass 声明落到 rhapsody 图，动态常量走 rhapsody 的
/// `DynamicConstants`（原版 256 KiB 页、线性分配）。
pub struct GraphPasses<'session> {
    graph: Rc<RefCell<RhGraph>>,
    device: Arc<Device>,
    dynamic: Arc<Mutex<DynamicConstants>>,
    tables: Option<HostTables>,
    cache_session: Option<GraphSession<'session>>,
    // Same Arc table -> same real constants range within this declaration
    // host. Actual pages remain owned by the existing DynamicConstants pool;
    // the graph/session and commands retain each imported allocation.
    uniform_uploads: Vec<(Arc<[u8]>, DynamicConstantsChunk)>,
}
enum GraphSession<'a> {
    Exclusive(&'a mut rhapsody::rg::VulkanGraphSession),
    Shared(Rc<RefCell<rhapsody::rg::VulkanGraphSession>>),
}
enum SessionBorrow<'a> {
    Exclusive(&'a mut rhapsody::rg::VulkanGraphSession),
    Shared(std::cell::RefMut<'a, rhapsody::rg::VulkanGraphSession>),
}
impl GraphSession<'_> {
    fn borrow_mut(&mut self) -> rhapsody::Result<SessionBorrow<'_>> {
        match self {
            Self::Exclusive(session) => Ok(SessionBorrow::Exclusive(session)),
            Self::Shared(session) => {
                session
                    .try_borrow_mut()
                    .map(SessionBorrow::Shared)
                    .map_err(|_| {
                        rhapsody::Error::invalid_usage(
                            "frame session is already borrowed during a graph operation",
                        )
                    })
            }
        }
    }
}
impl std::ops::Deref for SessionBorrow<'_> {
    type Target = rhapsody::rg::VulkanGraphSession;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Exclusive(s) => s,
            Self::Shared(s) => s,
        }
    }
}
impl std::ops::DerefMut for SessionBorrow<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Exclusive(s) => s,
            Self::Shared(s) => s,
        }
    }
}

/// 计算 Pass 执行时需要按 id 反查的三张宿主表。
#[derive(Clone)]
pub struct HostTables {
    pub buffers: Arc<BufferHost>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
}

impl<'session> GraphPasses<'session> {
    pub fn new(graph: Rc<RefCell<RhGraph>>, device: Arc<Device>) -> Result<Self, rhapsody::Error> {
        let dynamic = DynamicConstants::new(device.clone())?;
        Ok(Self {
            graph,
            device,
            dynamic: Arc::new(Mutex::new(dynamic)),
            tables: None,
            cache_session: None,
            uniform_uploads: Vec::new(),
        })
    }

    pub fn with_tables(mut self, tables: HostTables) -> Self {
        self.tables = Some(tables);
        self
    }

    /// Borrow the frame driver's real session only for declaration building.
    /// Drop this host before executing the frame; neither a second graph nor a
    /// second cache is introduced.
    pub fn with_session(
        mut self,
        session: &'session mut rhapsody::rg::VulkanGraphSession,
    ) -> rhapsody::Result<Self> {
        if !Arc::ptr_eq(&self.device, &session.backend.device()) {
            return Err(rhapsody::Error::invalid_usage(
                "upload cache belongs to another device",
            ));
        }
        self.cache_session = Some(GraphSession::Exclusive(session));
        Ok(self)
    }

    /// The frame facade may lazily import a library SSBO while a compute pass
    /// is being constructed. Hold the actual session identity, taking a short
    /// borrow only inside each declaration/upload operation, never around the
    /// caller's whole ComputePass construction closure.
    pub fn with_shared_session(
        mut self,
        session: Rc<RefCell<rhapsody::rg::VulkanGraphSession>>,
    ) -> rhapsody::Result<Self> {
        let device = session
            .try_borrow()
            .map_err(|_| rhapsody::Error::invalid_usage("frame session is already borrowed"))?
            .backend
            .device()
            .clone();
        if !Arc::ptr_eq(&self.device, &device) {
            return Err(rhapsody::Error::invalid_usage(
                "frame session belongs to another device",
            ));
        }
        self.cache_session = Some(GraphSession::Shared(session));
        Ok(self)
    }
}

use country_core::render::pass_builder::{
    compute_pass_builder::RgBuffer,
    pass_resource_access::RgTexture,
    rg_resources::{RgError, RgResources, TemporalBuffer, TemporalTexture, TemporalTexturePair},
};

// Compute PassEnv requests must use the same session that owns pass imports,
// uploads, named history and final submission. A second GraphRgResources with a
// separate session would make otherwise valid resource ids refer to another frame.
/// 原版渲染图里资源名只是调试标签，同一帧内同名资源可以多次创建（例如每个 Nani 外部子集各建一个
/// "Dispatch args"）；rhapsody 的 rg 要求名字唯一，所以这里在重名时追加 `#n` 后缀重试。
/// `declare` 在发现重名时尚未修改任何状态，重试是安全的。
fn declare_label_unique(
    backend: &mut rhapsody::rg::VulkanGraphBackend,
    graph: &mut rhapsody::rg::RenderGraph,
    name: &str,
    spec: rhapsody::rg::ResourceSpec,
) -> Result<rhapsody::rg::ResourceHandle, rhapsody::rg::BuildError> {
    use rhapsody::rg::BuildError;
    let mut attempt = 1u32;
    loop {
        let label = if attempt == 1 { name.to_owned() } else { format!("{name}#{attempt}") };
        match backend.declare(graph, &label, spec.clone()) {
            Err(BuildError::DuplicateResourceName(_)) => attempt += 1,
            other => return other,
        }
    }
}

impl RgResources for GraphPasses<'_> {
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture {
        let tables = self
            .tables
            .as_ref()
            .expect("frame resources require HostTables");
        let native = tables
            .textures
            .convert_for_graph(desc)
            .unwrap_or_else(|error| panic!("create_raw_texture({name}): {error:?}"));
        let mut session = self
            .cache_session
            .as_mut()
            .expect("frame resources require the frame driver's session")
            .borrow_mut()
            .expect("frame session already borrowed during declaration");
        let handle = declare_label_unique(
            &mut session.backend,
            &mut self.graph.borrow_mut(),
            name,
            ResourceSpec::Image(native),
        )
        .unwrap_or_else(|error| panic!("create_raw_texture({name}): {error:?}"));
        RgTexture {
            resource: handle.raw(),
            desc: *desc,
        }
    }

    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer {
        let native = BufferHost::convert(desc)
            .unwrap_or_else(|error| panic!("create_raw_buffer({name}): {error}"));
        let mut session = self
            .cache_session
            .as_mut()
            .expect("frame resources require the frame driver's session")
            .borrow_mut()
            .expect("frame session already borrowed during declaration");
        let handle = declare_label_unique(
            &mut session.backend,
            &mut self.graph.borrow_mut(),
            name,
            ResourceSpec::Buffer(native),
        )
        .unwrap_or_else(|error| panic!("create_raw_buffer({name}): {error:?}"));
        RgBuffer {
            resource: handle.raw(),
        }
    }

    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        let tables = self
            .tables
            .as_ref()
            .ok_or_else(|| rhapsody::Error::invalid_usage("frame resources require HostTables"))?;
        let native = tables.textures.convert_for_graph(desc).map_err(|error| {
            rhapsody::Error::invalid_usage(format!("invalid temporal texture: {error:?}"))
        })?;
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("frame resources require the frame driver's session")
            })?;
        let named = session.named_texture(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalTexture {
            texture: RgTexture {
                resource: named.texture.raw().raw(),
                desc: *desc,
            },
            // B get_or_create_temporal @1404f702e / 1404f72d6:
            // 1 means reusable history, 0 means a new allocation. Preserve
            // the current session's stronger committed-content validity.
            flag: u8::from(named.history_valid),
        })
    }

    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        let tables = self
            .tables
            .as_ref()
            .ok_or_else(|| rhapsody::Error::invalid_usage("frame resources require HostTables"))?;
        let native = tables.textures.convert_for_graph(desc).map_err(|error| {
            rhapsody::Error::invalid_usage(format!("invalid temporal texture: {error:?}"))
        })?;
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("frame resources require the frame driver's session")
            })?;
        let pair = session.temporal_texture(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalTexturePair {
            current: RgTexture {
                resource: pair.current().raw().raw(),
                desc: *desc,
            },
            previous: RgTexture {
                resource: pair.previous().raw().raw(),
                desc: *desc,
            },
            // The core field retains its old name, but native consumers send
            // this byte directly to the shader's `history_valid` uniform.
            fresh: pair.history_valid(),
        })
    }

    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        desc: &BufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        let native = BufferHost::convert(desc)?;
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("frame resources require the frame driver's session")
            })?;
        // Temporal buffers of the game are zero-filled on creation.
        let named = session.named_buffer_zeroed(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalBuffer {
            buffer: RgBuffer {
                resource: named.buffer.raw().raw(),
            },
            // B buffer instance @1404f643d / 1404f665a has the same polarity.
            flag: u8::from(named.history_valid),
        })
    }
}

impl CorePassGraph for GraphPasses<'_> {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn) {
        let device = self.device.clone();
        let mut graph = self.graph.borrow_mut();
        let mut pass = graph.add_pass(name);
        // 原 `add_resource_access(id, access, 0)`：同步组 0。
        let resource = pass
            .resource_handle(resource)
            .expect("add_resource_access: resource does not belong to this graph");
        pass.add_resource_access(resource, access, 0)
            .expect("add_resource_access: resource does not belong to this graph");
        pass.render_with_result_context(move |ctx| {
            let managed = ctx
                .managed
                .expect("pass callback without managed resources");
            // SAFETY: 命令缓冲、资源与设备在回调期间由 recorder 持有存活。
            let mut backend = unsafe { AshCommandBackend::new(&device.raw_device()) };
            let command =
                <ash::vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());
            let mut host = PassApiHost {
                api: RhRenderPassApi::new(&mut backend, command, 0, false),
                managed,
                error: None,
            };
            render(&mut host);
            host.error.map_or(Ok(()), Err)
        });
    }

    fn copy_from_staging(
        &mut self,
        name: &'static str,
        copies: Vec<StagingCopy>,
    ) -> rhapsody::Result<()> {
        use rhapsody::{rg::AccessRange, rg::Descriptor};
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::unsupported("staging copies require the frame driver's session")
            })?;
        if copies.is_empty() {
            return Err(rhapsody::Error::invalid_usage("empty staging copy batch"));
        }
        let mut graph = self
            .graph
            .try_borrow_mut()
            .map_err(|_| rhapsody::Error::invalid_usage("staging graph is already borrowed"))?;
        let mut declared = Vec::new();
        declared
            .try_reserve_exact(copies.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        for copy in copies {
            let chunk = copy
                .source
                .lease
                .as_ref()
                .and_then(ResourceLease::downcast_ref::<DynamicConstantsChunk>)
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage("staging copy has no allocation lease")
                })?;
            let size = copy.source.bytes.len() as u64;
            if size == 0
                || ash::vk::Handle::as_raw(chunk.buffer.raw_handle()) != copy.source.buffer
                || chunk.offset != u64::from(copy.source.offset)
                || chunk.size != size
            {
                return Err(rhapsody::Error::invalid_usage(
                    "staging metadata differs from its allocation",
                ));
            }
            let target = graph.resource_handle(copy.target)?;
            let Descriptor::Buffer(desc) =
                graph.declaration().resources[target.raw() as usize].descriptor
            else {
                return Err(rhapsody::Error::invalid_usage(
                    "staging target is not a buffer",
                ));
            };
            if copy
                .dst_offset
                .checked_add(size)
                .is_none_or(|end| end > desc.word_at_00)
                || !chunk.offset.is_multiple_of(4)
                || !copy.dst_offset.is_multiple_of(4)
                || !size.is_multiple_of(4)
            {
                return Err(rhapsody::Error::invalid_usage(
                    "staging copy range is invalid",
                ));
            }
            chunk.write_bytes(&copy.source.bytes)?;
            let source = session.import_frame_buffer(&mut graph, chunk.buffer.clone(), 0x25)?;
            declared.push((source, target, chunk.offset, copy.dst_offset, size));
        }
        let mut pass = graph.add_pass(name);
        for &(source, target, src_offset, dst_offset, size) in &declared {
            pass.add_resource_access_range(
                source,
                0x25,
                0,
                AccessRange::Buffer {
                    offset: src_offset,
                    size,
                },
            )?;
            pass.add_resource_access_range(
                target,
                0x34,
                0,
                AccessRange::Buffer {
                    offset: dst_offset,
                    size,
                },
            )?;
        }
        let device = self.device.clone();
        pass.render_with_result_context(move |ctx| {
            let managed = ctx
                .managed
                .ok_or_else(|| rhapsody::Error::invalid_usage("staging copy has no resources"))?;
            for &(source, target, src_offset, dst_offset, size) in &declared {
                let source = managed.buffer_for(source.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("staging source", error)
                })?;
                let target = managed.buffer_for(target.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("staging target", error)
                })?;
                unsafe {
                    device.raw_device().cmd_copy_buffer(
                        <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle()),
                        source.raw_handle(),
                        target.raw_handle(),
                        &[vk::BufferCopy {
                            src_offset,
                            dst_offset,
                            size,
                        }],
                    );
                }
            }
            Ok(())
        });
        Ok(())
    }

    fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc> {
        let chunk = self.dynamic.lock().allocate(size as u64)?;
        let buffer = ash::vk::Handle::as_raw(chunk.buffer.raw_handle());
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        bytes.resize(size, 0);
        Ok(StagingAlloc {
            buffer,
            offset: u32::try_from(chunk.offset).map_err(|_| {
                rhapsody::Error::invalid_usage("staging offset exceeds the core allocation domain")
            })?,
            bytes,
            lease: Some(ResourceLease::new(Arc::new(chunk))),
        })
    }

    fn upload_complete_bytes(
        &mut self,
        buffer: u32,
        bytes: &[u8],
        source: u64,
        version: u64,
    ) -> rhapsody::Result<bool> {
        use rhapsody::{
            rg::AccessRange, rg::ContentDependency, rg::DependencyId, rg::ResultDependencies,
            rg::ResultNodeId, rg::ResultRequest,
        };
        let Some(mut session) = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
        else {
            return Ok(false);
        };
        if bytes.is_empty() || !bytes.len().is_multiple_of(4) {
            return Err(rhapsody::Error::invalid_usage(
                "cached u32 upload must be nonempty and word aligned",
            ));
        }
        let mut graph = self
            .graph
            .try_borrow_mut()
            .map_err(|_| rhapsody::Error::invalid_usage("upload graph is already borrowed"))?;
        let target = graph.resource_handle(buffer)?;
        let size = bytes.len() as u64;
        let mut parameters = Vec::new();
        parameters
            .try_reserve_exact(bytes.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        parameters.extend_from_slice(bytes);
        session.invalidate_result_dependency(DependencyId::External(source), version);
        let device = self.device.clone();
        let dynamic = self.dynamic.clone();
        let result = session.cached_result(
            &mut graph,
            ResultRequest {
                node: ResultNodeId(source),
                dependencies: ResultDependencies::Complete(vec![ContentDependency::external(
                    source, version,
                )]),
                parameters,
                shader_implementation: 1,
                shader_interface: 1,
            },
            ResourceSpec::Buffer(RhBufferDesc::new_gpu_only(
                size,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            )),
            0x25,
            move |session, graph, output| {
                let chunk = dynamic.lock().allocate(size)?;
                chunk.write_bytes(bytes)?;
                let source = session.import_frame_buffer(graph, chunk.buffer.clone(), 0x25)?;
                graph.subgraph("country.cpu-upload.produce", |scope| {
                    scope.input(
                        source,
                        AccessRange::Buffer {
                            offset: chunk.offset,
                            size,
                        },
                        0x25,
                    )?;
                    scope.interface_mut().implementation_signature = 1;
                    let mut pass = scope.graph().add_pass("Produce immutable CPU upload");
                    pass.add_resource_access_range(
                        source,
                        0x25,
                        0,
                        AccessRange::Buffer {
                            offset: chunk.offset,
                            size,
                        },
                    )?;
                    pass.add_resource_access(output, 0x34, 0)?;
                    let source_offset = chunk.offset;
                    pass.render_with_result_context(move |ctx| {
                        let managed = ctx.managed.ok_or_else(|| {
                            rhapsody::Error::invalid_usage("upload producer has no resources")
                        })?;
                        let output = managed.buffer_for(output.raw()).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("upload output", error)
                        })?;
                        let source = managed.buffer_for(source.raw()).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("upload staging source", error)
                        })?;
                        unsafe {
                            device.raw_device().cmd_copy_buffer(
                                <vk::CommandBuffer as ash::vk::Handle>::from_raw(
                                    ctx.command.handle(),
                                ),
                                source.raw_handle(),
                                output.raw_handle(),
                                &[vk::BufferCopy {
                                    src_offset: source_offset,
                                    dst_offset: 0,
                                    size,
                                }],
                            );
                        }
                        Ok(())
                    });
                    scope.output(output, AccessRange::Whole, 0x34)
                })
            },
        )?;
        let device = self.device.clone();
        graph.subgraph("country.cpu-upload.copy", |scope| {
            scope.interface_mut().implementation_signature = 1;
            scope.input(result.handle, AccessRange::Whole, 0x25)?;
            let mut pass = scope.graph().add_pass("Copy immutable CPU upload");
            pass.add_resource_access(result.handle, 0x25, 0)?;
            pass.add_resource_access_range(
                target,
                0x34,
                0,
                AccessRange::Buffer { offset: 0, size },
            )?;
            pass.render_with_result_context(move |ctx| {
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("upload consumer has no resources")
                })?;
                let source = managed.buffer_for(result.handle.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("cached upload source", error)
                })?;
                let target = managed.buffer_for(target.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("cached upload target", error)
                })?;
                unsafe {
                    device.raw_device().cmd_copy_buffer(
                        <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle()),
                        source.raw_handle(),
                        target.raw_handle(),
                        &[vk::BufferCopy {
                            src_offset: 0,
                            dst_offset: 0,
                            size,
                        }],
                    );
                }
                Ok(())
            });
            scope.output(target, AccessRange::Buffer { offset: 0, size }, 0x34)
        })?;
        Ok(true)
    }
}

struct PassApiHost<'a, B: CommandBackend> {
    api: RhRenderPassApi<'a, B>,
    managed: &'a ManagedPassResources,
    error: Option<rhapsody::Error>,
}

impl<B: CommandBackend> CoreRenderPassApi for PassApiHost<'_, B> {
    fn fill_buffer(&mut self, buffer: u32, range: Option<std::ops::Range<u64>>, value: u32) {
        let buffer = self
            .managed
            .buffer_for(buffer)
            .expect("fill_buffer: unresolved buffer");
        self.api
            .fill_buffer(
                buffer.raw_handle(),
                range.map(|r| (r.start, r.end - r.start)),
                value,
            )
            .expect("fill_buffer: unaligned range");
    }

    fn clear_color_image(&mut self, image: u32) {
        let texture = self
            .managed
            .texture_for(image)
            .expect("clear_color_image: unresolved image");
        let layers = match texture.desc.array_layers() {
            Ok(layers) => layers,
            Err(error) => {
                self.error.get_or_insert(error);
                return;
            }
        };
        self.api
            .clear_color_image(texture.raw_handle(), texture.desc.mip_levels, layers);
    }
}

// ---------------------------------------------------------------- compute pass execution

use country_core::render::pass_builder::compute_pass_builder::{
    ComputeExec, ComputeExecution, ExternalSlot, RecordedComputePass,
};
use country_core::render::pass_builder::pass_resource_access::PassBinding as CorePassBinding;

/// 核心的视图描述（各字段 `Option`）叠加到纹理的默认视图上。
pub(crate) fn rh_view_desc(
    texture: &rhapsody::resource::Texture,
    view: &country_core::rhapsody_types::TextureViewDesc,
) -> rhapsody::Result<rhapsody::resource::TextureViewDesc> {
    project_view_desc(&texture.desc, view)
}

fn project_view_desc(
    texture: &rhapsody::resource::TextureDesc,
    view: &country_core::rhapsody_types::TextureViewDesc,
) -> rhapsody::Result<rhapsody::resource::TextureViewDesc> {
    let mut out = rhapsody::resource::TextureViewDesc::new(texture)?;
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
    out.level_count = remaining_view_count(out.level_count, view.base_mip_level, view.level_count)?;
    out.layer_count = remaining_view_count(out.layer_count, view.base_layer, view.layer_count)?;
    out.base_mip_level = view.base_mip_level;
    out.base_layer = view.base_layer;
    out.validate(texture)?;
    Ok(out)
}

/// Native None means the remaining subresource count after the selected base,
/// not the whole image count plus that base (rg binding @14205f263..297).
fn remaining_view_count(total: u32, base: u32, explicit: Option<u32>) -> rhapsody::Result<u32> {
    let remaining = total.checked_sub(base).ok_or_else(|| {
        rhapsody::Error::invalid_usage("image view base exceeds its actual image")
    })?;
    let count = explicit.unwrap_or(remaining);
    if count == 0 || count > remaining {
        return Err(rhapsody::Error::invalid_usage(
            "image view count exceeds its actual remaining range",
        ));
    }
    Ok(count)
}

/// Original bind_individual_mips enumerates the exact selected range and
/// creates a level_count=1 view per element (@1418b4740..47f7). The current
/// fully-bound shader ABI additionally fills unused array slots with the last
/// actual view; it creates no owner or mip beyond the declared original range.
fn individual_mip_views(
    view: rhapsody::resource::TextureViewDesc,
    array_count: u32,
) -> rhapsody::Result<Vec<rhapsody::resource::TextureViewDesc>> {
    if view.level_count == 0 || array_count < view.level_count {
        return Err(rhapsody::Error::invalid_usage(
            "individual mip views exceed the actual shader array",
        ));
    }
    let mut views = Vec::new();
    views
        .try_reserve_exact(array_count as usize)
        .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    for element in 0..array_count {
        let mut selected = view;
        selected.base_mip_level = view
            .base_mip_level
            .checked_add(element.min(view.level_count - 1))
            .ok_or_else(|| rhapsody::Error::invalid_usage("individual mip index overflow"))?;
        selected.level_count = 1;
        views.push(selected);
    }
    Ok(views)
}

/// Declaration-time projection of the same view used by rh_view_desc during
/// recording. The logical descriptor is available before GPU allocation; no
/// physical image or substitute owner is created to calculate this range.
pub(crate) fn compute_view_range(
    descriptor: rhapsody::rg::Descriptor,
    view: &country_core::rhapsody_types::TextureViewDesc,
) -> rhapsody::Result<rhapsody::rg::AccessRange> {
    use rhapsody::{rg::AccessRange, rg::Descriptor};
    let Descriptor::Image(image) = descriptor else {
        return Err(rhapsody::Error::invalid_usage(
            "compute image view refers to a buffer",
        ));
    };
    let AccessRange::Texture {
        level_count,
        layer_count,
        ..
    } = AccessRange::Whole.normalized(descriptor).map_err(|error| {
        rhapsody::Error::invalid_usage(format!("compute image shape: {error:?}"))
    })?
    else {
        unreachable!("image descriptor normalizes to a texture range")
    };
    let range = AccessRange::Texture {
        aspect_mask: view.aspect_mask.unwrap_or_else(|| {
            rhapsody::resource::vk_format_valid_aspect_flags(vk::Format::from_raw(
                image.words_at_00_through_18[2] as i32,
            ))
            .as_raw()
        }),
        base_mip_level: view.base_mip_level,
        level_count: remaining_view_count(level_count, view.base_mip_level, view.level_count)?,
        base_array_layer: view.base_layer,
        layer_count: remaining_view_count(layer_count, view.base_layer, view.layer_count)?,
    };
    range.validate(descriptor).map_err(|error| {
        rhapsody::Error::invalid_usage(format!("compute image view range: {error:?}"))
    })?;
    Ok(range)
}

/// 整帧实测需要的 bindless 描述符数(存储缓冲 600 不够 4096 够；采样图像 256 不够)。
/// 宿主预算，非原版常量；实际取值按设备上限收敛，见 [`adaptive_bindless_config`]。
const BINDLESS_STORAGE_BUFFER_DEMAND: u32 = 4096;
const BINDLESS_SAMPLED_IMAGE_DEMAND: u32 = 2048;

/// 各类描述符数取需求与设备 update-after-bind 上限(单 set、单 stage)的较小者，
/// 再把总数收进 stage 资源总数与全池总数：超出时先削存储缓冲，再削采样图像。
/// 上限足够的设备(如 lavapipe)得到完整需求；上限小的设备(如 Mac 的 1212)按自身能力
/// 收敛，不强行超限。
fn adaptive_bindless_config(device: &rhapsody::device::Device) -> BindlessConfig {
    let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
    // SAFETY: the instance and physical device belong to this live device.
    unsafe {
        device
            .instance()
            .raw_instance()
            .get_physical_device_properties2(device.physical_device(), &mut properties);
    }
    let base = BindlessConfig::default();
    let mut sampled_images = BINDLESS_SAMPLED_IMAGE_DEMAND
        .min(indexing.max_descriptor_set_update_after_bind_sampled_images)
        .min(indexing.max_per_stage_descriptor_update_after_bind_sampled_images)
        .max(base.sampled_images);
    let mut storage_buffers = BINDLESS_STORAGE_BUFFER_DEMAND
        .min(indexing.max_descriptor_set_update_after_bind_storage_buffers)
        .min(indexing.max_per_stage_descriptor_update_after_bind_storage_buffers)
        .max(base.storage_buffers);
    let budget = indexing
        .max_per_stage_update_after_bind_resources
        .min(indexing.max_update_after_bind_descriptors_in_all_pools);
    let fixed = base.storage_images + base.samplers;
    let mut excess = (sampled_images + storage_buffers + fixed).saturating_sub(budget);
    let cut = excess.min(storage_buffers.saturating_sub(base.storage_buffers));
    storage_buffers -= cut;
    excess -= cut;
    sampled_images -= excess.min(sampled_images.saturating_sub(base.sampled_images));
    BindlessConfig {
        sampled_images,
        storage_buffers,
        ..base
    }
}

/// 宿主偏差(DEVIATION)：原版整缓冲绑定（range = 缓冲全长），原机 maxStorageBufferRange
/// 足够大(Apple M3 为 4 GiB)。设备上限更小时(lavapipe 固定 128 MiB)整缓冲绑定会被
/// 描述符校验拒绝，此处把绑定范围钳到设备上限；设备上限足够时返回原值，行为与原版一致。
/// 钳制后着色器越界访问由 robustBufferAccess 兜底，仅影响小上限设备上超出部分的数据。
pub(crate) fn clamp_storage_binding_range(device: &rhapsody::device::Device, size: u64) -> u64 {
    size.min(u64::from(device.limits().max_storage_buffer_range))
}

pub(crate) fn upsert_shader_resource(
    resources: &mut Vec<(u32, BindlessResource)>,
    binding: u32,
    resource: BindlessResource,
) -> rhapsody::Result<()> {
    if let Some((_, old)) = resources.iter_mut().find(|(index, _)| *index == binding) {
        *old = resource;
    } else {
        resources
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        resources.push((binding, resource));
    }
    Ok(())
}

/// Dispatch admission is shared by direct and indirect commands. In particular,
/// an empty initial payload is not a valid zero-filled replacement for a shader
/// whose first payload is supplied inside `dispatch_with`.
struct ComputeParameterState {
    ready: bool,
    error: Option<rhapsody::Error>,
}

impl ComputeParameterState {
    fn new(
        push_bytes: u32,
        initial: &[u8],
        bind: &mut dyn FnMut(&[u8]) -> rhapsody::Result<()>,
    ) -> rhapsody::Result<Self> {
        let deferred = push_bytes != 0 && initial.is_empty();
        if !deferred {
            bind(initial)?;
        }
        Ok(Self {
            ready: !deferred,
            error: None,
        })
    }

    fn push(&mut self, bytes: &[u8], bind: &mut dyn FnMut(&[u8]) -> rhapsody::Result<()>) {
        if self.error.is_none() {
            match bind(bytes) {
                Ok(()) => self.ready = true,
                Err(error) => self.error = Some(error),
            }
        }
    }

    fn can_dispatch(&mut self) -> bool {
        if self.error.is_some() {
            return false;
        }
        if !self.ready {
            self.error = Some(rhapsody::Error::invalid_usage(
                "compute dispatch requires a valid shader push parameter payload",
            ));
            return false;
        }
        true
    }
}

struct ComputeExecHost<'a> {
    device: &'a ash::Device,
    command: vk::CommandBuffer,
    local_size: [u32; 3],
    managed: &'a ManagedPassResources,
    bind_parameters: &'a mut dyn FnMut(&[u8]) -> rhapsody::Result<()>,
    parameters: ComputeParameterState,
}

impl ComputeExec for ComputeExecHost<'_> {
    /// `[device+0xaba]` 的调试开关（每次 dispatch 前全管线屏障）；本宿主不开启。
    fn full_barrier_enabled(&self) -> bool {
        false
    }
    fn issue_full_pipeline_barrier(&mut self) {}
    fn count_dispatch(&mut self) {}
    fn cmd_dispatch(&mut self, x: u32, y: u32, z: u32) {
        if self.parameters.can_dispatch() {
            unsafe { self.device.cmd_dispatch(self.command, x, y, z) };
        }
    }
    fn cmd_dispatch_indirect(&mut self, buffer: u32, offset: u64) -> Option<()> {
        if !self.parameters.can_dispatch() {
            return Some(());
        }
        let buffer = self.managed.buffer_for(buffer).ok()?;
        unsafe {
            self.device
                .cmd_dispatch_indirect(self.command, buffer.raw_handle(), offset)
        };
        Some(())
    }
    fn local_size(&self) -> [u32; 3] {
        self.local_size
    }
    fn push_constants(&mut self, bytes: &[u8]) {
        self.parameters.push(bytes, self.bind_parameters);
    }
}

impl GraphPasses<'_> {
    /// 登记一个已声明完毕的计算 Pass：访问声明进图；回调里绑定管线与描述符集（set 0）、
    /// 推 push constants、按模式 dispatch。`pipeline` 是该 Pass 着色器程序创建出的管线句柄。
    pub fn add_compute(
        &mut self,
        pass: RecordedComputePass,
        pipeline: &ComputePipeline,
    ) -> rhapsody::Result<()> {
        let tables = self
            .tables
            .clone()
            .ok_or_else(|| rhapsody::Error::invalid_usage("add_compute requires HostTables"))?;
        let entry = tables.shaders.compute_entry(pipeline).ok_or_else(|| {
            rhapsody::Error::invalid_usage("compute pipeline was not created through this host")
        })?;
        let RecordedComputePass {
            name,
            builder,
            bindings,
            access_ranges,
            inactive_texture_bindings,
            undefined_texture_bindings,
            preserving_buffers,
            preserving_texture_bindings,
            external_bindings,
            device_address_reads,
            uniform_binder,
            push_constants,
            dispatch,
            ..
        } = pass;
        let accesses = builder.accesses.clone();
        drop(builder);

        let device = self.device.clone();
        let dynamic = self.dynamic.clone();
        let mut graph = self
            .graph
            .try_borrow_mut()
            .map_err(|_| rhapsody::Error::invalid_usage("compute graph is already borrowed"))?;
        let mut external_resources = Vec::new();
        let mut external_accesses = Vec::new();
        external_resources
            .try_reserve_exact(external_bindings.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        external_accesses
            .try_reserve_exact(external_bindings.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        let mut external_writes = false;
        // These immutable uniforms are already native graph resources with an
        // exact imported range. Their descriptor lease must retain that role,
        // rather than also treating the same bytes as a private upload read.
        let mut declared_uniform_bindings = Vec::new();
        for external in external_bindings {
            use rhapsody::rg::AccessRange;
            let mut session = self
                .cache_session
                .as_mut()
                .map(GraphSession::borrow_mut)
                .transpose()?
                .ok_or_else(|| {
                    rhapsody::Error::unsupported(
                        "external compute resources require the frame driver's session",
                    )
                })?;
            let kind = entry
                .bindings
                .abi
                .slot(BindingLocation {
                    set: 0,
                    binding: external.binding,
                })
                .map(|slot| slot.kind)
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "external compute binding is absent from shader ABI",
                    )
                })?;
            let (resource, handle, range, code) = match external.slot {
                ExternalSlot::UniformBytes(bytes) => {
                    declared_uniform_bindings.push(external.binding);
                    if !matches!(
                        kind,
                        BindingKind::StorageBuffer | BindingKind::UniformBuffer
                    ) {
                        return Err(rhapsody::Error::invalid_usage(
                            "external uniform differs from shader ABI",
                        ));
                    }
                    if bytes.is_empty() {
                        return Err(rhapsody::Error::invalid_usage(
                            "external uniform has an empty range",
                        ));
                    }
                    let (buffer, offset, size) = if let Some((_, chunk)) = self
                        .uniform_uploads
                        .iter()
                        .find(|(source, _)| Arc::ptr_eq(source, &bytes))
                    {
                        (chunk.buffer.clone(), chunk.offset, chunk.size)
                    } else {
                        // Bounded declaration cache, not another allocator or
                        // descriptor heap. Original DoF uses one shared table.
                        if self.uniform_uploads.len() == 64 {
                            return Err(rhapsody::Error::unsupported(
                                "external uniform declaration budget exceeded",
                            ));
                        }
                        self.uniform_uploads
                            .try_reserve(1)
                            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
                        let chunk = self.dynamic.lock().push_slice(&bytes)?;
                        let range = (chunk.buffer.clone(), chunk.offset, chunk.size);
                        self.uniform_uploads.push((bytes, chunk));
                        range
                    };
                    let expected = if kind == BindingKind::UniformBuffer {
                        vk::BufferUsageFlags::UNIFORM_BUFFER
                    } else {
                        vk::BufferUsageFlags::STORAGE_BUFFER
                    };
                    if !Arc::ptr_eq(&buffer.device(), &self.device)
                        || !buffer.usage.contains(expected)
                        || offset.checked_add(size).is_none_or(|end| end > buffer.size)
                    {
                        return Err(rhapsody::Error::invalid_usage(
                            "external uniform physical range/device mismatch",
                        ));
                    }
                    let handle = session.import_frame_buffer(&mut graph, buffer.clone(), 0x1a)?;
                    (
                        BindlessResource::Buffer {
                            buffer,
                            offset,
                            size,
                        },
                        handle,
                        AccessRange::Buffer { offset, size },
                        0x1a,
                    )
                }
                ExternalSlot::Buffer {
                    vk_buffer,
                    offset,
                    range,
                    handle,
                    ..
                } => {
                    if !matches!(
                        kind,
                        BindingKind::StorageBuffer | BindingKind::UniformBuffer
                    ) {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute buffer differs from shader ABI",
                        ));
                    }
                    let buffer = tables.buffers.get_by_id(handle.0).ok_or_else(|| {
                        rhapsody::Error::invalid_usage("external compute buffer is not live")
                    })?;
                    if ash::vk::Handle::as_raw(buffer.raw_handle()) != vk_buffer {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute buffer differs from its owner",
                        ));
                    }
                    let size = if range == vk::WHOLE_SIZE {
                        buffer.size.checked_sub(offset).ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "external compute buffer offset exceeds size",
                            )
                        })?
                    } else {
                        range
                    };
                    if size == 0 || offset.checked_add(size).is_none_or(|end| end > buffer.size) {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute buffer range exceeds its owner",
                        ));
                    }
                    let buffer = buffer.shared();
                    let handle = session.import_frame_buffer(&mut graph, buffer.clone(), 0x37)?;
                    external_writes = true;
                    (
                        BindlessResource::Buffer {
                            buffer,
                            offset,
                            size: clamp_storage_binding_range(&self.device, size),
                        },
                        handle,
                        AccessRange::Buffer { offset, size },
                        0x37,
                    )
                }
                ExternalSlot::Image {
                    sampler,
                    image_view,
                    layout,
                    handle,
                    ..
                } => {
                    if sampler != 0 {
                        return Err(rhapsody::Error::unsupported(
                            "combined external image sampler is not a separate shader binding",
                        ));
                    }
                    let view = tables.textures.owned_view(handle.0, image_view)?;
                    let (expected, code) = match kind {
                        BindingKind::SampledImage => {
                            (vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL, 0x22)
                        }
                        BindingKind::StorageImage => {
                            external_writes = true;
                            (vk::ImageLayout::GENERAL, 0x37)
                        }
                        _ => {
                            return Err(rhapsody::Error::invalid_usage(
                                "external compute image differs from shader ABI",
                            ));
                        }
                    };
                    if expected.as_raw() != layout as i32 {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute image layout differs from shader ABI",
                        ));
                    }
                    let range = AccessRange::Texture {
                        aspect_mask: view.desc.aspect.as_raw(),
                        base_mip_level: view.desc.base_mip_level,
                        level_count: view.desc.level_count,
                        base_array_layer: view.desc.base_layer,
                        layer_count: view.desc.layer_count,
                    };
                    let handle =
                        session.import_frame_texture(&mut graph, view.texture.clone(), code)?;
                    (
                        entry.bindings.image(external.binding, view)?,
                        handle,
                        range,
                        code,
                    )
                }
                ExternalSlot::Other(_) => {
                    return Err(rhapsody::Error::unsupported(
                        "external compute resource kind is not connected",
                    ));
                }
            };
            external_accesses.push((handle, range, code));
            upsert_shader_resource(&mut external_resources, external.binding, resource)?;
        }
        // Original global set 1/b1 is a storage view of the per-frame FrameConstants
        // block (wall/roof chain). Unlike the uniform b0 view, a storage lease must be
        // covered by a declared graph access of its owner, so import the exact frame
        // constants range as a shader read of this pass.
        if entry
            .bindings
            .abi
            .slot(BindingLocation { set: 1, binding: 1 })
            .is_some()
        {
            let (buffer, offset, size) = entry.bindings.frame_storage_view()?;
            let mut session = self
                .cache_session
                .as_mut()
                .map(GraphSession::borrow_mut)
                .transpose()?
                .ok_or_else(|| {
                    rhapsody::Error::unsupported(
                        "frame constants storage view requires the frame driver's session",
                    )
                })?;
            let handle = session.import_frame_buffer(&mut graph, buffer, 0x1a)?;
            external_accesses.push((
                handle,
                rhapsody::rg::AccessRange::Buffer { offset, size },
                0x1a,
            ));
        }
        // BDA pointers are not descriptor bindings, but their physical reads
        // still participate in this graph's synchronization and owner program.
        // The same import/session owns cancellation and completion retirement.
        let mut device_address_accesses = Vec::new();
        device_address_accesses
            .try_reserve_exact(device_address_reads.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        for chunk in device_address_reads {
            if chunk.size == 0
                || chunk
                    .offset
                    .checked_add(chunk.size)
                    .is_none_or(|end| end > chunk.buffer.size)
            {
                return Err(rhapsody::Error::invalid_usage("invalid BDA upload range"));
            }
            // Validate actual BDA admission even if a future caller bypasses
            // the terrain uploader's own device_address() call.
            chunk.device_address()?;
            let mut session = self
                .cache_session
                .as_mut()
                .map(GraphSession::borrow_mut)
                .transpose()?
                .ok_or_else(|| {
                    rhapsody::Error::unsupported(
                        "BDA upload ranges require the frame driver's session",
                    )
                })?;
            let handle = session.import_frame_buffer(&mut graph, chunk.buffer.clone(), 0x1a)?;
            device_address_accesses.push((
                handle,
                rhapsody::rg::AccessRange::Buffer {
                    offset: chunk.offset,
                    size: chunk.size,
                },
            ));
        }
        for range in &access_ranges {
            if bindings.get(range.access_index as usize).is_none() {
                return Err(rhapsody::Error::invalid_usage(
                    "compute binding range index is invalid",
                ));
            }
        }
        // Native bind_texture_with_view appends its access and its ImageView
        // together. Match each occurrence once, preserving repeated views of
        // the same image (e.g. read mip N / write mip N+1) in original order.
        // Synchronization code/group remain native values; the legacy API
        // does not invent content initialization from a Vulkan RW mask.
        let mut used_image_views = vec![false; bindings.len()];
        let mut declared_accesses = Vec::with_capacity(accesses.len());
        for access in &accesses {
            let view = bindings.iter().enumerate().find_map(|(index, binding)| {
                match binding {
                    CorePassBinding::ImageView { resource, write, view }
                        if !used_image_views[index] && *resource == access.resource
                            && *write == country_core::render::pass_builder::pass_resource_access::access_is_write(access.access) =>
                        Some((index, view)),
                    _ => None,
                }
            });
            let mut contents_unused = false;
            let mut allow_undefined = false;
            let mut preserves_texture = false;
            let range = if let Some((index, view)) = view {
                used_image_views[index] = true;
                contents_unused = inactive_texture_bindings.contains(&(index as u8));
                allow_undefined = undefined_texture_bindings.contains(&(index as u8));
                if allow_undefined
                    && (contents_unused || rhapsody::command::is_write(access.access))
                {
                    return Err(rhapsody::Error::invalid_usage(
                        "undefined texture read metadata conflicts with its actual access",
                    ));
                }
                preserves_texture = preserving_texture_bindings.contains(&(index as u8));
                let resource = graph.resource_handle(access.resource)?;
                // A storage-image (write) view that is not `bind_individual_mips` and
                // names no level count is a single `RWTexture2D[Array]` operand: the
                // shader can only address the view's base mip, so the pass touches
                // exactly one level even though the native view spans the remaining
                // chain. Declaring the whole chain would demand (preserving RMW) or
                // claim (plain write) contents of levels the pass can never reach.
                // Evidence (original 0x140533b10 / ImageBarrier::new 0x140bd65e0): the original
                // declares access per whole image and tracks no mip contents, so this range is a
                // host-side refinement of rhapsody's content contract (shader semantics), not an
                // original behavior; removing it makes read-storage passes (Water deferred rt,
                // Volumetrics) demand mips 1..5 before "Generate mips" writes them.
                let storage_single_level = if let CorePassBinding::ImageView { write: true, .. } =
                    &bindings[index]
                {
                    view.level_count.is_none() && !view.bind_individual_mips
                } else {
                    false
                };
                let view_for_range;
                let view = if storage_single_level {
                    view_for_range = view.level_count(Some(1));
                    &view_for_range
                } else {
                    view
                };
                compute_view_range(
                    graph.declaration().resources[resource.raw() as usize].descriptor,
                    view,
                )?
            } else {
                // Storage buffers share the same existing per-binding inactive
                // intent. Match each native read occurrence independently so an
                // active alias still tightens the graph's content requirement.
                if let Some(index) =
                    bindings
                        .iter()
                        .enumerate()
                        .find_map(|(index, binding)| match binding {
                            CorePassBinding::BufferRead { resource }
                            | CorePassBinding::Buffer { resource }
                                if !used_image_views[index]
                                    && *resource == access.resource
                                    && !rhapsody::command::is_write(access.access) =>
                            {
                                Some(index)
                            }
                            _ => None,
                        })
                {
                    used_image_views[index] = true;
                    contents_unused = inactive_texture_bindings.contains(&(index as u8));
                }
                rhapsody::rg::AccessRange::Whole
            };
            declared_accesses.push((
                *access,
                range,
                contents_unused,
                allow_undefined,
                preserves_texture,
            ));
        }
        if bindings.iter().enumerate().any(|(index, binding)| {
            matches!(binding, CorePassBinding::ImageView { .. }) && !used_image_views[index]
        }) {
            return Err(rhapsody::Error::invalid_usage(
                "compute image view has no matching native resource access",
            ));
        }
        let mut declared = graph.add_pass(name);
        declared.on_queue(rhapsody::device::QueueClass::Compute);
        for (a, range, contents_unused, allow_undefined, preserves_texture) in &declared_accesses {
            let resource = declared.resource_handle(a.resource)?;
            if *contents_unused && !rhapsody::command::is_write(a.access) {
                declared.add_inactive_resource_read_range(resource, a.access, a.group, *range)?;
            } else if *allow_undefined && !rhapsody::command::is_write(a.access) {
                declared.add_undefined_content_read_range(resource, a.access, a.group, *range)?;
            } else if (*preserves_texture || preserving_buffers.contains(&a.resource))
                && rhapsody::command::is_write(a.access)
            {
                declared
                    .add_preserving_resource_access_range(resource, a.access, a.group, *range)?;
            } else {
                declared.add_resource_access_range(resource, a.access, a.group, *range)?;
            }
        }
        for (resource, range, code) in external_accesses {
            declared.add_resource_access_range(resource, code, 0, range)?;
        }
        for (resource, range) in device_address_accesses {
            declared.add_resource_access_range(resource, 0x1a, 1, range)?;
        }
        if external_writes {
            declared.side_effect();
        }
        declared.render_with_result_context(move |ctx| {
            let managed = ctx.managed.ok_or_else(|| {
                rhapsody::Error::invalid_usage("compute callback without managed resources")
            })?;
            let diagnose = |stage: &str, error: &rhapsody::Error| {
                log::error!("compute {name}: {stage}: {error:?}; heap {:?}", entry.bindings.heap.stats());
            };
            let pipeline = entry.pipeline.try_get().inspect_err(|error| diagnose("pipeline.try_get", error))?;
            let command = <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());
            let mut resources = Vec::new();
            let mut image_arrays: Vec<(u32, Vec<BindlessResource>)> = Vec::new();
            for range in &access_ranges {
                let resource = match bindings[range.access_index as usize] {
                    CorePassBinding::Buffer { resource }
                    | CorePassBinding::BufferRead { resource } => {
                        let buffer = managed.buffer_for(resource).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("compute buffer binding", error)
                        })?;
                        BindlessResource::Buffer {
                            buffer: buffer.shared(),
                            offset: 0,
                            size: clamp_storage_binding_range(&device, buffer.size),
                        }
                    }
                    CorePassBinding::ImageView { resource, view, write } => {
                        let texture = managed.texture_for(resource).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("compute image binding", error)
                        })?;
                        // Same single-operand storage view the graph declaration above
                        // narrowed: the lease must be covered by the declared range.
                        let view = if write && view.level_count.is_none() && !view.bind_individual_mips {
                            view.level_count(Some(1))
                        } else {
                            view
                        };
                        let desc = rh_view_desc(&texture, &view)?;
                        if view.bind_individual_mips {
                            let location = BindingLocation { set: 0, binding: range.resource_index };
                            let count = entry.bindings.abi.slot(location)
                                .ok_or_else(|| rhapsody::Error::invalid_usage("individual mip binding is absent from the shader ABI"))?
                                .parameter_count;
                            if image_arrays.iter().any(|(binding, _)| *binding == range.resource_index)
                                || resources.iter().any(|(binding, _)| *binding == range.resource_index) {
                                return Err(rhapsody::Error::invalid_usage("individual mip binding has conflicting values"));
                            }
                            let mut elements = Vec::new();
                            for selected in individual_mip_views(desc, count)? {
                                let selected = texture.image_view(&selected)?;
                                elements.push(entry.bindings.image_for_pass(&ctx, resource, range.resource_index, selected.view.clone())?);
                            }
                            image_arrays.push((range.resource_index, elements));
                            continue;
                        }
                        let view = texture.image_view(&desc)?;
                        entry
                            .bindings
                            .image_for_pass(&ctx, resource, range.resource_index, view.view.clone())?
                    }
                };
                upsert_shader_resource(&mut resources, range.resource_index, resource)?;
            }
            for (binding, resource) in external_resources {
                upsert_shader_resource(&mut resources, binding, resource)?;
            }
            if let (Some(uniform), Some(binding)) = (uniform_binder, entry.uniform_binding) {
                let chunk = dynamic.lock().allocate_with_data(uniform.data())?;
                upsert_shader_resource(
                    &mut resources,
                    binding,
                    BindlessResource::Buffer {
                        buffer: chunk.buffer.clone(),
                        offset: chunk.offset,
                        size: chunk.size,
                    },
                )?;
            }
            // SAFETY: pipeline and command belong to this device; graph declares the accessed resources.
            unsafe {
                device.raw_device().cmd_bind_pipeline(
                    command,
                    vk::PipelineBindPoint::COMPUTE,
                    pipeline.raw_handle(),
                );
            }
            let mut parameters: Option<rhapsody::bindless::PreparedBindlessParameters> = None;
            let mut bind_parameters = |push: &[u8]| -> rhapsody::Result<()> {
                let next = if let Some(previous) = &parameters {
                    previous.with_push(push, &mut dynamic.lock())
                        .inspect_err(|error| diagnose("parameters.with_push", error))?
                } else {
                    let mut first = entry.bindings.parameters(push)
                        .inspect_err(|error| diagnose("parameters.new/globals", error))?;
                    for (binding, resource) in &resources {
                        let location = BindingLocation { set: 0, binding: *binding };
                        let result = if declared_uniform_bindings.contains(binding) {
                            let BindlessResource::Buffer { buffer, offset, size } = resource else {
                                return Err(rhapsody::Error::invalid_usage(
                                    "declared uniform descriptor is not its actual buffer"));
                            };
                            let lease = entry.bindings.heap.storage_buffer(buffer.clone(), *offset, *size)?;
                            first.write_lease(location, 0, lease)
                        } else {
                            first.write(location, 0, resource.clone())
                        };
                        result.map_err(|error| {
                            diagnose("parameters.write", &error);
                            if error.kind() != rhapsody::ErrorKind::InvalidUsage {
                                return error;
                            }
                            let detail = match resource {
                                BindlessResource::Buffer { buffer, offset, size } => format!(
                                    ": buffer {:?}, usage {:#x}, allocation {}, offset {}, range {}",
                                    buffer.raw_handle(), buffer.desc.usage.as_raw(), buffer.desc.size,
                                    offset, size,
                                ),
                                _ => String::new(),
                            };
                            rhapsody::Error::invalid_usage_with(
                                format!("compute {name}, set 0 binding {binding}{detail}"),
                                error,
                            )
                        })?;
                    }
                    for (binding, elements) in &image_arrays {
                        if resources.iter().any(|(scalar, _)| scalar == binding) {
                            return Err(rhapsody::Error::invalid_usage("individual mip binding conflicts with a scalar descriptor"));
                        }
                        let location = BindingLocation { set: 0, binding: *binding };
                        for (element, resource) in elements.iter().enumerate() {
                            first.write(location, element as u32, resource.clone())
                                .inspect_err(|error| diagnose("parameters.write_individual_mips", error))?;
                        }
                    }
                    first.finish(&mut dynamic.lock())
                        .inspect_err(|error| diagnose("parameters.finish", error))?
                };
                // SAFETY: all resources came from this pass's declared binding
                // contract; later payloads retain those exact resource leases.
                unsafe { next.bind_pass(&ctx, pipeline.clone())? };
                parameters = Some(next);
                Ok(())
            };
            let parameter_state = ComputeParameterState::new(
                entry.bindings.abi.original_push_bytes,
                &push_constants,
                &mut bind_parameters,
            )?;
            let mut exec = ComputeExecHost {
                device: &device.raw_device(),
                command,
                local_size: entry.local_size,
                managed,
                bind_parameters: &mut bind_parameters,
                parameters: parameter_state,
            };
            // An explicit initial payload is already bound. Otherwise the first
            // callback push must bind a valid payload before either dispatch kind.
            ComputeExecution {
                push_constants: Vec::new(),
                dispatch,
            }
            .execute(&mut exec);
            exec.parameters.error.map_or(Ok(()), Err)
        });
        Ok(())
    }
}

impl rhapsody::resource::GpuUse for RasterEntry {
    fn visit_uses(&self, visit: &mut rhapsody::resource::UseVisitor<'_>) -> rhapsody::Result<()> {
        if let Ok(pipeline) = self.pipeline.try_get() {
            pipeline.visit_uses(visit)?;
        }
        Ok(())
    }
}
impl rhapsody::resource::GpuUse for ComputeEntry {
    fn visit_uses(&self, visit: &mut rhapsody::resource::UseVisitor<'_>) -> rhapsody::Result<()> {
        if let Ok(pipeline) = self.pipeline.try_get() {
            pipeline.visit_uses(visit)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod buffer_description_tests {
    use super::*;
    #[test]
    fn uniform_buffer_descriptions_admit_the_current_storage_heap_lowering() {
        for (usage, physical) in [(0x12, 0x32), (0x32, 0x32), (0x20, 0x20), (0, 0)] {
            let desc = BufferDesc::new_gpu_only(432, usage).alignment(4);
            let converted = BufferHost::convert(&desc).unwrap();
            assert_eq!(converted.usage.as_raw(), physical);
            assert_eq!(converted.size, 432);
            assert_eq!(converted.alignment, 4);
            assert_eq!(desc.usage, usage, "native/core descriptor must stay intact");
        }
    }

    #[test]
    fn buffer_description_conversion_preserves_all_memory_locations() {
        for memory in 1..=4 {
            let desc = BufferDesc {
                size: 256,
                usage: 0x20,
                memory,
                alignment: 32,
                gpu_only: false,
            };
            let converted = BufferHost::convert(&desc).unwrap();
            assert_eq!(converted.memory_location.as_raw(), memory);
            assert_eq!(converted.alignment, 32);
            assert_eq!(converted.size, 256);
        }
    }
    #[test]
    fn buffer_description_conversion_rejects_unknown_memory_and_truncating_alignment() {
        for memory in [0, 5, u8::MAX] {
            let error = BufferHost::convert(&BufferDesc {
                size: 16,
                memory,
                ..Default::default()
            })
            .unwrap_err();
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        }
        for alignment in [3, 6, 65_536, usize::MAX] {
            let error = BufferHost::convert(&BufferDesc {
                size: 16,
                memory: 1,
                alignment,
                ..Default::default()
            })
            .unwrap_err();
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        }
        for alignment in [0, 1, 32_768] {
            let converted = BufferHost::convert(&BufferDesc {
                size: 16,
                memory: 1,
                alignment,
                ..Default::default()
            })
            .unwrap();
            assert_eq!(usize::from(converted.alignment), alignment);
        }
    }
}

#[cfg(test)]
mod compute_parameter_tests {
    use super::ComputeParameterState;

    #[test]
    fn callback_payload_is_bound_before_dispatch_without_a_synthetic_initial_value() {
        let mut payloads = Vec::new();
        let mut bind = |bytes: &[u8]| {
            assert_eq!(bytes.len(), 20);
            payloads.push(bytes.to_vec());
            Ok(())
        };
        let mut state = ComputeParameterState::new(20, &[], &mut bind).unwrap();
        assert!(!state.ready);
        state.push(&[7; 20], &mut bind);
        assert!(state.can_dispatch());
        state.push(&[9; 20], &mut bind);
        assert!(state.can_dispatch());
        assert_eq!(payloads, vec![vec![7; 20], vec![9; 20]]);
        assert!(state.error.is_none());
    }

    #[test]
    fn dispatch_before_first_payload_is_rejected_and_a_later_push_cannot_hide_it() {
        let mut bind_calls = 0;
        let mut bind = |_: &[u8]| {
            bind_calls += 1;
            Ok(())
        };
        let mut state = ComputeParameterState::new(20, &[], &mut bind).unwrap();
        assert!(!state.can_dispatch());
        state.push(&[1; 20], &mut bind);
        assert!(!state.can_dispatch());
        assert_eq!(bind_calls, 0);
        assert!(matches!(
            state.error,
            Some(rhapsody::Error::InvalidUsage { .. })
        ));
    }

    #[test]
    fn invalid_callback_payload_keeps_the_binding_failure_and_suppresses_dispatch() {
        let mut bind_calls = 0;
        let mut bind = |bytes: &[u8]| {
            bind_calls += 1;
            if bytes.len() != 20 {
                return Err(rhapsody::Error::invalid_usage(
                    "wrong callback payload size",
                ));
            }
            Ok(())
        };
        let mut state = ComputeParameterState::new(20, &[], &mut bind).unwrap();
        state.push(&[1; 4], &mut bind);
        assert!(!state.can_dispatch());
        state.push(&[2; 20], &mut bind);
        assert!(!state.can_dispatch());
        assert_eq!(bind_calls, 1);
        assert!(matches!(
            state.error,
            Some(rhapsody::Error::InvalidUsage { .. })
        ));
    }

    #[test]
    fn zero_byte_abi_and_explicit_initial_payload_bind_immediately() {
        for payload in [Vec::new(), vec![3; 16]] {
            let mut payloads = Vec::new();
            let mut bind = |bytes: &[u8]| {
                payloads.push(bytes.to_vec());
                Ok(())
            };
            let mut state =
                ComputeParameterState::new(payload.len() as u32, &payload, &mut bind).unwrap();
            assert!(state.can_dispatch());
            assert_eq!(payloads, vec![payload]);
        }
        let error = ComputeParameterState::new(20, &[1; 4], &mut |_| {
            Err(rhapsody::Error::invalid_usage("wrong initial payload size"))
        });
        assert!(matches!(error, Err(rhapsody::Error::InvalidUsage { .. })));
    }
}

#[cfg(test)]
mod original_individual_mip_view_tests {
    use super::*;
    #[test]
    fn selected_remaining_mips_are_individual_and_unused_abi_tail_reuses_the_last_actual_view() {
        let desc = rhapsody::resource::TextureDesc::new_2d_array(
            256,
            256,
            2,
            vk::Format::B10G11R11_UFLOAT_PACK32,
        )
        .mip_levels(9)
        .usage(vk::ImageUsageFlags::STORAGE);
        let source = country_core::rhapsody_types::TextureViewDesc::new()
            .base_mip_level(1)
            .base_layer(1)
            .view_type(5)
            .bind_individual_mips(true);
        let selected = project_view_desc(&desc, &source).unwrap();
        assert_eq!(
            (
                selected.base_mip_level,
                selected.level_count,
                selected.base_layer,
                selected.layer_count
            ),
            (1, 8, 1, 1)
        );
        let views = individual_mip_views(selected, 12).unwrap();
        assert_eq!(views.len(), 12);
        assert_eq!(
            views.iter().map(|v| v.base_mip_level).collect::<Vec<_>>(),
            [1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8]
        );
        assert!(
            views
                .iter()
                .all(|v| v.level_count == 1 && v.layer_count == 1 && v.base_layer == 1)
        );
        for v in views {
            v.validate(&desc).unwrap();
        }
        let logical = rhapsody::rg::ResourceSpec::Image(desc).descriptor();
        assert_eq!(
            compute_view_range(logical, &source).unwrap(),
            rhapsody::rg::AccessRange::Texture {
                aspect_mask: 1,
                base_mip_level: 1,
                level_count: 8,
                base_array_layer: 1,
                layer_count: 1
            }
        );
        assert!(individual_mip_views(selected, 7).is_err());
        assert!(project_view_desc(&desc, &source.base_mip_level(9)).is_err());
        assert!(project_view_desc(&desc, &source.level_count(Some(9))).is_err());
    }
}

#[cfg(test)]
mod original_surface_texture_flags_tests {
    use super::*;
    #[test]
    fn ice_radiance_flags_survive_the_actual_canonical_descriptor_projection() {
        let original = TextureDesc::new_2d([64, 32], 0x62).sampled_with_different_format();
        assert_eq!(original.flags, 0x108);
        assert!(original.sampled);
        let native = TextureHost::convert(&original).unwrap();
        assert_eq!(native.flags.as_raw(), 0x108);
        assert_eq!(native.format, vk::Format::R32_UINT);
        assert_eq!(native.width, 64);
        assert_eq!(native.height, 32);
        assert!(native.usage.contains(vk::ImageUsageFlags::SAMPLED));
        assert_eq!(
            TextureHost::convert(&TextureDesc::new_2d([64, 32], 0x62))
                .unwrap()
                .flags
                .as_raw(),
            0
        );
    }
}

#[cfg(test)]
mod planned_image_layout_abi_tests {
    use super::*;
    #[test]
    fn sampled_general_is_explicit_and_storage_never_accepts_sampled_layout() {
        for layout in [
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            vk::ImageLayout::GENERAL,
        ] {
            validate_image_binding_layout(BindingKind::SampledImage, layout).unwrap();
        }
        validate_image_binding_layout(BindingKind::StorageImage, vk::ImageLayout::GENERAL).unwrap();
        assert!(
            validate_image_binding_layout(
                BindingKind::StorageImage,
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
            )
            .is_err()
        );
        assert!(
            validate_image_binding_layout(BindingKind::SampledImage, vk::ImageLayout::UNDEFINED)
                .is_err()
        );
        assert!(
            validate_image_binding_layout(
                BindingKind::SampledImage,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL
            )
            .is_err()
        );
        assert!(
            validate_image_binding_layout(BindingKind::Sampler, vk::ImageLayout::GENERAL).is_err()
        );
    }
}
