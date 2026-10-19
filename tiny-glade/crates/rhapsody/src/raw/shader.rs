//! Shader assets and reflection, pipeline layouts, pipelines and the pipeline
//! cache. SPIR-V parsing uses rspirv; reflection comes from the stored shader
//! container catalog.
use super::allocator::{
    AllocatorStats, CpuMemoryReservation, cpu_vec_bytes, reserve_shared_cpu_metadata,
    shared_cpu_bytes,
};
use super::temp_list::TempList;
use crate::Result;
use crate::raw::descriptor_set::VulkanDescriptorSetLayout;
use crate::raw::descriptor_set::{Binding, LayoutPlan, descriptor_set_layout};
use crate::raw::device::Device;
use ash::vk;
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ffi::{CStr, CString},
    io::Cursor,
    sync::Arc,
};

fn invalid_shader() -> crate::Error {
    crate::Error::invalid_usage("着色器数据、SPIR-V 入口或反射无效")
}
fn observe_vulkan<T>(device: &Device, result: std::result::Result<T, vk::Result>) -> Result<T> {
    result.map_err(|error| {
        if error == vk::Result::ERROR_DEVICE_LOST {
            device.timelines.mark_device_lost();
        }
        crate::Error::from(error)
    })
}

/// Releases owned CPU buffers before returning their reservation on any
/// pre-native failure as well as when the final published object is dropped.
struct BudgetedMetadata<T> {
    data: T,
    memory: CpuMemoryReservation,
}
pub struct ShaderModule {
    pub device: Arc<Device>,
    pub handle: vk::ShaderModule,
    interfaces: Arc<[super::shader_interface::EntryContract]>,
    _memory: CpuMemoryReservation,
}
impl ShaderModule {
    pub fn from_spirv(device: Arc<Device>, bytes: &[u8]) -> Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let mut words_memory = device.allocator_stats.reserve_cpu_bytes(
            cpu_vec_bytes::<u32>(bytes.len() / 4)?,
            crate::device::BudgetCategory::Live,
        )?;
        let words = ash::util::read_spv(&mut Cursor::new(bytes)).map_err(|_| invalid_shader())?;
        words_memory.resize(cpu_vec_bytes::<u32>(words.capacity())?)?;
        if words.len() < 5 || words.first().copied() != Some(0x07230203) {
            return Err(invalid_shader());
        }
        let interfaces = super::shader_interface::contracts(&words)?;
        let (interface_layout, _) = std::alloc::Layout::new::<[usize; 2]>()
            .extend(
                std::alloc::Layout::array::<super::shader_interface::EntryContract>(
                    interfaces.len(),
                )
                .map_err(|_| crate::Error::OutOfHostMemory)?,
            )
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        let interface_bytes = interfaces.iter().try_fold(
            interface_layout.pad_to_align().size() as u64,
            |bytes, entry| {
                bytes
                    .checked_add(entry.bytes() as u64)
                    .ok_or(crate::Error::OutOfHostMemory)
            },
        )?;
        let memory = reserve_shared_cpu_metadata::<Self>(&device.allocator_stats, interface_bytes)?;
        let interfaces = BudgetedMetadata {
            data: interfaces,
            memory,
        };
        let info = vk::ShaderModuleCreateInfo::default().code(&words);
        let handle =
            unsafe { device.device.create_shader_module(&info, None) }.map_err(|error| {
                if error == vk::Result::ERROR_DEVICE_LOST {
                    device.timelines.mark_device_lost();
                }
                crate::Error::from(error)
            })?;
        Ok(Arc::new(Self {
            device,
            handle,
            interfaces: interfaces.data,
            _memory: interfaces.memory,
        }))
    }
}
impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.device.device.destroy_shader_module(self.handle, None);
        }
    }
}

pub struct PipelineLayout {
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub device: Arc<Device>,
    pub handle: vk::PipelineLayout,
    pub(crate) sets: Vec<Arc<VulkanDescriptorSetLayout>>,
    pub(crate) push_constants: Vec<vk::PushConstantRange>,
    bindless_abi: Option<Arc<crate::bindless::BindlessShaderAbi>>,
    _memory: CpuMemoryReservation,
}
impl PipelineLayout {
    pub fn sets(&self) -> &[Arc<VulkanDescriptorSetLayout>] {
        &self.sets
    }
    pub fn push_constants(&self) -> &[vk::PushConstantRange] {
        &self.push_constants
    }
    pub fn new(
        device: Arc<Device>,
        sets: Vec<Arc<VulkanDescriptorSetLayout>>,
        push_constants: Vec<vk::PushConstantRange>,
    ) -> Result<Arc<Self>> {
        Self::create(device, sets, push_constants, None)
    }
    pub(crate) fn new_bindless(
        device: Arc<Device>,
        sets: Vec<Arc<VulkanDescriptorSetLayout>>,
        push_constants: Vec<vk::PushConstantRange>,
        abi: Arc<crate::bindless::BindlessShaderAbi>,
    ) -> Result<Arc<Self>> {
        Self::create(device, sets, push_constants, Some(abi))
    }
    pub fn bindless_abi(&self) -> Option<&Arc<crate::bindless::BindlessShaderAbi>> {
        self.bindless_abi.as_ref()
    }
    fn create(
        device: Arc<Device>,
        sets: Vec<Arc<VulkanDescriptorSetLayout>>,
        push_constants: Vec<vk::PushConstantRange>,
        bindless_abi: Option<Arc<crate::bindless::BindlessShaderAbi>>,
    ) -> Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if sets.iter().any(|s| !Arc::ptr_eq(&device, &s.device)) {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let limits = *device.limits();
        if sets.len() > limits.max_bound_descriptor_sets as usize
            || push_constants.iter().any(|range| {
                range.size == 0
                    || range.size % 4 != 0
                    || range.offset % 4 != 0
                    || range.stage_flags.is_empty()
                    || range
                        .offset
                        .checked_add(range.size)
                        .is_none_or(|end| end > limits.max_push_constants_size)
            })
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        for (index, range) in push_constants.iter().enumerate() {
            if push_constants[..index]
                .iter()
                .any(|prior| prior.stage_flags.intersects(range.stage_flags))
            {
                return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
            }
        }
        let bytes = cpu_vec_bytes::<Arc<VulkanDescriptorSetLayout>>(sets.capacity())?
            .checked_add(cpu_vec_bytes::<vk::PushConstantRange>(
                push_constants.capacity(),
            )?)
            .ok_or(crate::Error::OutOfHostMemory)?;
        let memory = reserve_shared_cpu_metadata::<Self>(&device.allocator_stats, bytes)?;
        let arrays = BudgetedMetadata {
            data: (sets, push_constants),
            memory,
        };
        let mut raw = TempList::new(arrays.data.0.len().max(1), device.allocator_stats.clone())?;
        raw.reserve(arrays.data.0.len())?;
        for set in &arrays.data.0 {
            raw.add(set.handle)?;
        }
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(raw.as_slice())
            .push_constant_ranges(&arrays.data.1);
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.allocator_stats)?;
        let handle = observe_vulkan(&device, unsafe {
            device.device.create_pipeline_layout(&info, None)
        })?;
        Ok(Arc::new(Self {
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            handle,
            sets: arrays.data.0,
            push_constants: arrays.data.1,
            bindless_abi,
            _memory: arrays.memory,
        }))
    }
}
impl Drop for PipelineLayout {
    fn drop(&mut self) {
        self.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::PipelineLayout(self.handle),
        );
    }
}

/// Vulkan driver cache with bounded object tables. Parallel creations use a
/// shared guard; cache-data extraction takes an exclusive guard so it cannot
/// overlap creation. Each key has a separate creation slot.
///
/// 每个键一个槽位：有界表只在查找/插入时短暂持锁，创建在槽位锁内进行——同键的并发调用者
/// 排队等待同一次创建（去重），不同键互不阻塞；创建失败不缓存，下次调用重试。
type ComputePipelineKey = (u64, u64, Vec<u8>);
type PipelineSlot = Arc<Mutex<PipelineSlotState>>;
struct PipelineSlotState {
    pipeline: Option<Arc<VulkanPipeline>>,
    _memory: CpuMemoryReservation,
}
struct BudgetedKey<K> {
    value: K,
    memory: CpuMemoryReservation,
}
struct PipelineCacheEntry<K> {
    key: K,
    slot: PipelineSlot,
    _key_memory: CpuMemoryReservation,
}
pub struct PipelineCache {
    pub device: Arc<Device>,
    handle: vk::PipelineCache,
    lock: RwLock<()>,
    capacity: usize,
    compute: Mutex<TempList<PipelineCacheEntry<ComputePipelineKey>>>,
    raster: Mutex<TempList<PipelineCacheEntry<Vec<u64>>>>,
}
/// Driver cache bytes retain their CPU budget until the last payload owner is
/// dropped, including when a caller keeps them after the cache itself is gone.
pub struct PipelineCacheData {
    bytes: Vec<u8>,
    _memory: super::allocator::CpuMemoryReservation,
}
impl PipelineCacheData {
    pub(crate) fn zeroed(
        budget: &Arc<super::allocator::AllocatorStats>,
        size: usize,
    ) -> Result<Self> {
        let mut memory =
            budget.reserve_cpu_bytes(size as u64, crate::device::BudgetCategory::Live)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        memory.resize(bytes.capacity() as u64)?;
        bytes.resize(size, 0);
        Ok(Self {
            bytes,
            _memory: memory,
        })
    }
    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}
impl std::ops::Deref for PipelineCacheData {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}
/// Full borrowed equality is checked before allocating a key. Slot metadata is
/// owned by the final Arc, including compilers that outlive table eviction.
fn slot_for<K>(
    table: &Mutex<TempList<PipelineCacheEntry<K>>>,
    matches: impl Fn(&K) -> bool,
    create_key: impl FnOnce() -> Result<BudgetedKey<K>>,
    capacity: usize,
    budget: &Arc<AllocatorStats>,
) -> Result<PipelineSlot> {
    let mut slots = table.lock();
    if let Some(entry) = slots.as_slice().iter().find(|entry| matches(&entry.key)) {
        return Ok(entry.slot.clone());
    }
    let victim = if slots.as_slice().len() >= capacity {
        Some(
            slots
                .as_slice()
                .iter()
                .position(|entry| {
                    Arc::strong_count(&entry.slot) == 1 && entry.slot.try_lock().is_some()
                })
                .ok_or_else(|| {
                    crate::Error::exhausted("every pipeline cache slot is referenced")
                })?,
        )
    } else {
        let needed = slots.as_slice().len() + 1;
        slots.reserve(needed)?;
        None
    };
    // All fallible admission precedes eviction. Failure preserves every old key.
    let key = create_key()?;
    let memory = reserve_shared_cpu_metadata::<Mutex<PipelineSlotState>>(budget, 0)?;
    let slot = Arc::new(Mutex::new(PipelineSlotState {
        pipeline: None,
        _memory: memory,
    }));
    if let Some(victim) = victim {
        let mut index = 0;
        slots.retain(|_| {
            let keep = index != victim;
            index += 1;
            keep
        });
    }
    slots.add(PipelineCacheEntry {
        key: key.value,
        slot: slot.clone(),
        _key_memory: key.memory,
    })?;
    Ok(slot)
}
fn budgeted_key_vec<T: Copy>(
    budget: &Arc<AllocatorStats>,
    values: &[T],
) -> Result<BudgetedKey<Vec<T>>> {
    let mut memory = budget.reserve_cpu_bytes(
        cpu_vec_bytes::<T>(values.len())?,
        crate::device::BudgetCategory::Live,
    )?;
    let mut key = Vec::new();
    key.try_reserve_exact(values.len())
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    memory.resize(cpu_vec_bytes::<T>(key.capacity())?)?;
    key.extend_from_slice(values);
    Ok(BudgetedKey { value: key, memory })
}
fn budgeted_raster_key(
    budget: &Arc<AllocatorStats>,
    desc: &RasterPipelineDesc,
) -> Result<BudgetedKey<Vec<u64>>> {
    let size = raster_pipeline_key_len(desc);
    let mut memory = budget.reserve_cpu_bytes(
        cpu_vec_bytes::<u64>(size)?,
        crate::device::BudgetCategory::Live,
    )?;
    let mut key = Vec::new();
    key.try_reserve_exact(size)
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    memory.resize(cpu_vec_bytes::<u64>(key.capacity())?)?;
    visit_raster_pipeline_key(desc, &mut |word| key.push(word));
    Ok(BudgetedKey { value: key, memory })
}
impl PipelineCache {
    pub fn new(device: Arc<Device>, initial_data: &[u8]) -> Result<Self> {
        Self::with_capacity(device, initial_data, 256)
    }
    /// Bounds each in-memory pipeline table, independently of driver cache data.
    pub fn with_capacity(
        device: Arc<Device>,
        initial_data: &[u8],
        capacity: usize,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if capacity == 0 || capacity > 65_536 {
            return Err(crate::Error::invalid_usage(
                "pipeline cache capacity must be in 1..=65536",
            ));
        }
        if !initial_data.is_empty() && !valid_pipeline_cache_data(&device, initial_data) {
            return Err(crate::Error::invalid_usage(
                "pipeline cache header does not match this device",
            ));
        }
        let compute = TempList::new(capacity, device.allocator_stats.clone())?;
        let raster = TempList::new(capacity, device.allocator_stats.clone())?;
        let info = vk::PipelineCacheCreateInfo::default().initial_data(initial_data);
        let handle = observe_vulkan(&device, unsafe {
            device.device.create_pipeline_cache(&info, None)
        })?;
        Ok(Self {
            device,
            handle,
            lock: RwLock::new(()),
            capacity,
            compute: Mutex::new(compute),
            raster: Mutex::new(raster),
        })
    }
    pub fn data(&self) -> Result<PipelineCacheData> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let _lock = self.lock.write();
        let get = self.device.device.fp_v1_0().get_pipeline_cache_data;
        // Query and copy under the same exclusive cache lock. Native creation
        // holds a read guard, so a compiler cannot invalidate the queried size.
        for _ in 0..3 {
            let mut size = 0;
            let result = unsafe {
                get(
                    self.device.device.handle(),
                    self.handle,
                    &mut size,
                    std::ptr::null_mut(),
                )
            };
            observe_vulkan(&self.device, result.result())?;
            if size as u64 > MAX_PIPELINE_CACHE_BYTES {
                return Err(crate::Error::invalid_usage(
                    "pipeline cache data exceeds its byte limit",
                ));
            }
            let mut data = PipelineCacheData::zeroed(&self.device.allocator_stats, size)?;
            let mut written = size;
            let result = unsafe {
                get(
                    self.device.device.handle(),
                    self.handle,
                    &mut written,
                    data.bytes.as_mut_ptr().cast(),
                )
            };
            if result == vk::Result::INCOMPLETE {
                continue;
            }
            observe_vulkan(&self.device, result.result())?;
            if written > size {
                return Err(crate::Error::invalid_usage(
                    "pipeline cache returned an invalid data length",
                ));
            }
            data.bytes.truncate(written);
            return Ok(data);
        }
        Err(crate::Error::Timeout)
    }
    /// 管线缓存持有布局与 shader；显式失效后已有提交仍由外部 Arc 保持有效。
    pub fn invalidate(&self) {
        self.compute
            .lock()
            .retain(|entry| Arc::strong_count(&entry.slot) > 1 || entry.slot.try_lock().is_none());
        self.raster
            .lock()
            .retain(|entry| Arc::strong_count(&entry.slot) > 1 || entry.slot.try_lock().is_none());
    }
    pub fn cached_compute_count(&self) -> usize {
        self.compute
            .lock()
            .as_slice()
            .iter()
            .filter(|entry| entry.slot.lock().pipeline.is_some())
            .count()
    }
}
/// Upper bound for driver pipeline-cache blobs, in memory and on disk.
pub(crate) const MAX_PIPELINE_CACHE_BYTES: u64 = 64 << 20;
pub(crate) fn valid_pipeline_cache_data(device: &Device, bytes: &[u8]) -> bool {
    if bytes.len() < 32 || bytes.len() as u64 > MAX_PIPELINE_CACHE_BYTES {
        return false;
    }
    let word = |offset| {
        u32::from_le_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .expect("checked Vulkan cache header"),
        )
    };
    let properties = device.properties();
    word(0) >= 32
        && word(0) as usize <= bytes.len()
        && word(4) == vk::PipelineCacheHeaderVersion::ONE.as_raw() as u32
        && word(8) == properties.vendor_id
        && word(12) == properties.device_id
        && bytes[16..32] == properties.pipeline_cache_uuid
}
impl Drop for PipelineCache {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_pipeline_cache(self.handle, None) }
    }
}

#[derive(Clone)]
pub struct ComputePipelineDesc {
    pub shader: Arc<ShaderModule>,
    pub entry: CString,
    pub layout: Arc<PipelineLayout>,
}
impl ComputePipelineDesc {
    pub fn new(shader: Arc<ShaderModule>, layout: Arc<PipelineLayout>) -> Self {
        Self {
            shader,
            entry: c"main".to_owned(),
            layout,
        }
    }
}

/// Formats of a dynamic-rendering pipeline; graphics state is explicit.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DynamicRenderingFormats {
    pub colors: Vec<vk::Format>,
    pub depth: vk::Format,
    pub stencil: vk::Format,
    /// Multiview is deliberately not enabled by this implementation yet.
    pub view_mask: u32,
}
impl DynamicRenderingFormats {
    pub fn color(format: vk::Format) -> Self {
        Self {
            colors: vec![format],
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            view_mask: 0,
        }
    }
    pub fn validate(&self, max_colors: u32) -> Result<()> {
        if self.view_mask != 0 {
            return Err(crate::Error::unsupported(
                "dynamic rendering multiview is not enabled",
            ));
        }
        if self.colors.len() > max_colors as usize {
            return Err(crate::Error::invalid_usage(
                "dynamic rendering color count exceeds maxColorAttachments",
            ));
        }
        for &format in &self.colors {
            if format != vk::Format::UNDEFINED
                && super::format::vk_format_valid_aspect_flags(format)
                    != vk::ImageAspectFlags::COLOR
            {
                return Err(crate::Error::invalid_usage(
                    "dynamic rendering color format has depth/stencil aspects",
                ));
            }
        }
        for (format, aspect) in [
            (self.depth, vk::ImageAspectFlags::DEPTH),
            (self.stencil, vk::ImageAspectFlags::STENCIL),
        ] {
            if format != vk::Format::UNDEFINED
                && !super::format::vk_format_valid_aspect_flags(format).contains(aspect)
            {
                return Err(crate::Error::invalid_usage(
                    "dynamic rendering depth/stencil format lacks its aspect",
                ));
            }
        }
        if self.depth != vk::Format::UNDEFINED
            && self.stencil != vk::Format::UNDEFINED
            && self.depth != self.stencil
        {
            return Err(crate::Error::invalid_usage(
                "dynamic depth and stencil formats must match the shared attachment",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RasterTargetSignature {
    Dynamic {
        formats: DynamicRenderingFormats,
        samples: vk::SampleCountFlags,
    },
}

#[derive(Clone)]
pub struct RasterPipelineDesc {
    pub vertex: Arc<ShaderModule>,
    pub fragment: Option<Arc<ShaderModule>>,
    pub entry: CString,
    /// 原发行 vertex/fragment 入口通常分别为 vs_main/ps_main，不能误用同一名称。
    pub fragment_entry: Option<CString>,
    pub layout: Arc<PipelineLayout>,
    /// Required color/depth/stencil signature for dynamic rendering.
    pub dynamic_rendering: Option<DynamicRenderingFormats>,
    pub vertex_bindings: Vec<vk::VertexInputBindingDescription>,
    pub vertex_attributes: Vec<vk::VertexInputAttributeDescription>,
    pub topology: vk::PrimitiveTopology,
    pub polygon_mode: vk::PolygonMode,
    pub cull_mode: vk::CullModeFlags,
    pub front_face: vk::FrontFace,
    pub samples: vk::SampleCountFlags,
    pub depth_test: bool,
    pub depth_write: bool,
    pub depth_compare: vk::CompareOp,
    pub stencil_test: bool,
    pub stencil: vk::StencilOpState,
    pub blend_attachments: Vec<vk::PipelineColorBlendAttachmentState>,
}
impl RasterPipelineDesc {
    /// 构造带必需 vertex/layout 的默认图形管线描述；其余固定状态由调用方填写。
    pub fn new(vertex: Arc<ShaderModule>, layout: Arc<PipelineLayout>) -> Self {
        Self {
            vertex,
            fragment: None,
            entry: c"vs_main".to_owned(),
            fragment_entry: None,
            layout,
            dynamic_rendering: None,
            vertex_bindings: Vec::new(),
            vertex_attributes: Vec::new(),
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            samples: vk::SampleCountFlags::TYPE_1,
            depth_test: false,
            depth_write: false,
            depth_compare: vk::CompareOp::LESS_OR_EQUAL,
            stencil_test: false,
            stencil: vk::StencilOpState::default(),
            blend_attachments: Vec::new(),
        }
    }
    pub fn use_dynamic_rendering(&mut self, formats: DynamicRenderingFormats) {
        self.dynamic_rendering = Some(formats);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RasterPipelineOpts(pub u32);
impl RasterPipelineOpts {
    pub fn new() -> Self {
        Self(0)
    }
    fn byte(self, index: u32) -> u32 {
        (self.0 >> (index * 8)) & 0xff
    }
    fn set_bits(&mut self, shift: u32, width: u32, value: u32) {
        let mask = ((1u32 << width) - 1) << shift;
        self.0 = (self.0 & !mask) | ((value << shift) & mask);
    }
    pub fn topology(self) -> Result<vk::PrimitiveTopology> {
        match self.byte(0) & 3 {
            0 => Ok(vk::PrimitiveTopology::TRIANGLE_LIST),
            1 => Ok(vk::PrimitiveTopology::LINE_LIST),
            2 => Ok(vk::PrimitiveTopology::LINE_STRIP),
            _ => Err(crate::Error::invalid_usage(
                "RasterPipelineOpts 拓扑编码无效",
            )),
        }
    }
    pub fn set_topology(&mut self, value: vk::PrimitiveTopology) -> Result<()> {
        let code = match value {
            vk::PrimitiveTopology::TRIANGLE_LIST => 0,
            vk::PrimitiveTopology::LINE_LIST => 1,
            vk::PrimitiveTopology::LINE_STRIP => 2,
            other => {
                return Err(crate::Error::invalid_usage(format!(
                    "RasterPipelineOpts 不支持拓扑 {other:?}"
                )));
            }
        };
        self.set_bits(0, 2, code);
        Ok(())
    }
    pub fn cull_mode(self) -> vk::CullModeFlags {
        vk::CullModeFlags::from_raw((self.byte(0) >> 2) & 3)
    }
    pub fn set_cull_mode(&mut self, value: vk::CullModeFlags) {
        self.set_bits(2, 2, value.as_raw());
    }
    pub fn depth_compare_op(self) -> Result<vk::CompareOp> {
        match (self.byte(0) >> 4) & 3 {
            0 => Ok(vk::CompareOp::LESS),
            1 => Ok(vk::CompareOp::GREATER),
            2 => Ok(vk::CompareOp::EQUAL),
            _ => Err(crate::Error::invalid_usage(
                "RasterPipelineOpts 深度比较编码无效",
            )),
        }
    }
    pub fn set_depth_compare_op(&mut self, value: vk::CompareOp) -> Result<()> {
        let code = match value {
            vk::CompareOp::LESS => 0,
            vk::CompareOp::GREATER => 1,
            vk::CompareOp::EQUAL => 2,
            other => {
                return Err(crate::Error::invalid_usage(format!(
                    "RasterPipelineOpts 不支持深度比较 {other:?}"
                )));
            }
        };
        self.set_bits(4, 2, code);
        Ok(())
    }
    pub fn depth_write(self) -> bool {
        (self.byte(0) >> 6) & 1 != 0
    }
    pub fn set_depth_write(&mut self, value: bool) {
        self.set_bits(6, 1, u32::from(value));
    }
    pub fn depth_test(self) -> bool {
        (self.byte(0) >> 7) & 1 != 0
    }
    pub fn set_depth_test_enable(&mut self, value: bool) {
        self.set_bits(7, 1, u32::from(value));
    }
    pub fn stencil_test(self) -> bool {
        self.byte(1) & 1 != 0
    }
    pub fn set_stencil_test_enable(&mut self, value: bool) {
        self.set_bits(8, 1, u32::from(value));
    }
    pub fn stencil_fail_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 1) & 7) as i32)
    }
    pub fn set_stencil_fail_op(&mut self, value: vk::StencilOp) {
        self.set_bits(9, 3, value.as_raw() as u32);
    }
    pub fn stencil_pass_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 4) & 7) as i32)
    }
    pub fn set_stencil_pass_op(&mut self, value: vk::StencilOp) {
        self.set_bits(12, 3, value.as_raw() as u32);
    }
    /// 3 位横跨 b1 bit7（低位）与 b2 bit0–1（高两位）。
    pub fn stencil_depth_fail_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 7) + (self.byte(2) & 3) * 2) as i32)
    }
    pub fn set_stencil_depth_fail_op(&mut self, value: vk::StencilOp) {
        let raw = value.as_raw() as u32;
        self.set_bits(15, 1, raw & 1);
        self.set_bits(16, 2, raw >> 1);
    }
    pub fn stencil_compare_op(self) -> vk::CompareOp {
        vk::CompareOp::from_raw(((self.byte(2) >> 2) & 7) as i32)
    }
    pub fn set_stencil_compare_op(&mut self, value: vk::CompareOp) {
        self.set_bits(18, 3, value.as_raw() as u32);
    }
    /// 8 位横跨 b2 bit5–7（低 3 位）与 b3 bit0–4（高 5 位）。
    pub fn stencil_compare_mask(self) -> u32 {
        ((self.byte(3) << 3) | (self.byte(2) >> 5)) & 0xff
    }
    pub fn set_stencil_compare_mask(&mut self, value: u32) {
        self.set_bits(21, 3, value & 7);
        self.set_bits(24, 5, (value >> 3) & 0x1f);
    }
    pub fn blend_mode(self) -> u8 {
        ((self.byte(3) >> 5) & 3) as u8
    }
    pub fn set_blend_mode(&mut self, value: u8) {
        self.set_bits(29, 2, u32::from(value));
    }
    pub fn polygon_mode(self) -> vk::PolygonMode {
        if self.byte(3) >> 7 != 0 {
            vk::PolygonMode::LINE
        } else {
            vk::PolygonMode::FILL
        }
    }
    pub fn set_polygon_mode(&mut self, value: vk::PolygonMode) -> Result<()> {
        let code = match value {
            vk::PolygonMode::FILL => 0,
            vk::PolygonMode::LINE => 1,
            other => {
                return Err(crate::Error::invalid_usage(format!(
                    "RasterPipelineOpts 不支持多边形模式 {other:?}"
                )));
            }
        };
        self.set_bits(31, 1, code);
        Ok(())
    }
    /// front/back 共用的模板状态；write mask 恒 `0xffffffff`，reference 0（动态）。
    pub fn stencil_state(self) -> vk::StencilOpState {
        vk::StencilOpState::default()
            .fail_op(self.stencil_fail_op())
            .pass_op(self.stencil_pass_op())
            .depth_fail_op(self.stencil_depth_fail_op())
            .compare_op(self.stencil_compare_op())
            .compare_mask(self.stencil_compare_mask())
            .write_mask(0xffff_ffff)
            .reference(0)
    }
    /// 把位域展开到管线描述（固定状态部分；`front_face` 恒 COUNTER_CLOCKWISE=0，线宽 1.0）。
    /// The blend attachment state is replicated for every color attachment.
    pub fn apply_to(self, desc: &mut RasterPipelineDesc, color_attachments: usize) -> Result<()> {
        // Decode every fallible field before mutating the caller's description.
        let topology = self.topology()?;
        let depth_compare = self.depth_compare_op()?;
        desc.topology = topology;
        desc.polygon_mode = self.polygon_mode();
        desc.cull_mode = self.cull_mode();
        desc.front_face = vk::FrontFace::COUNTER_CLOCKWISE;
        desc.depth_test = self.depth_test();
        desc.depth_write = self.depth_write();
        desc.depth_compare = depth_compare;
        desc.stencil_test = self.stencil_test();
        desc.stencil = self.stencil_state();
        desc.blend_attachments = vec![get_vk_blend_mode(self.blend_mode() << 5); color_attachments];
        Ok(())
    }
}
struct PipelineArrays {
    shaders: Vec<Arc<ShaderModule>>,
    entries: Vec<usize>,
    formats: Option<DynamicRenderingFormats>,
    target: Option<RasterTargetSignature>,
}
pub struct VulkanPipeline {
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub device: Arc<Device>,
    pub handle: vk::Pipeline,
    pub layout: Arc<PipelineLayout>,
    pub point: vk::PipelineBindPoint,
    shaders: Vec<Arc<ShaderModule>>,
    shader_entries: Vec<usize>,
    dynamic_rendering: Option<DynamicRenderingFormats>,
    raster_target: Option<RasterTargetSignature>,
    _memory: CpuMemoryReservation,
}
impl VulkanPipeline {
    pub fn create_compute_pipeline(
        cache: &PipelineCache,
        desc: ComputePipelineDesc,
    ) -> Result<Arc<Self>> {
        if cache.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if !Arc::ptr_eq(&cache.device, &desc.layout.device)
            || !Arc::ptr_eq(&cache.device, &desc.shader.device)
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let shader_entry = super::shader_interface::validate(
            &desc.shader.interfaces,
            &desc.entry,
            vk::ShaderStageFlags::COMPUTE,
            &desc.layout,
        )?;
        use ash::vk::Handle;
        let shader = desc.shader.handle.as_raw();
        let layout = desc.layout.handle.as_raw();
        let slot = slot_for(
            &cache.compute,
            |key| key.0 == shader && key.1 == layout && key.2.as_slice() == desc.entry.as_bytes(),
            || {
                let key = budgeted_key_vec(&cache.device.allocator_stats, desc.entry.as_bytes())?;
                Ok(BudgetedKey {
                    value: (shader, layout, key.value),
                    memory: key.memory,
                })
            },
            cache.capacity,
            &cache.device.allocator_stats,
        )?;
        let mut created = slot.lock();
        if let Some(pipeline) = created.pipeline.as_ref() {
            return Ok(pipeline.clone());
        }
        let count = 1;
        let mut memory = reserve_shared_cpu_metadata::<Self>(
            &cache.device.allocator_stats,
            pipeline_array_bytes(count, count, 0, 0)?,
        )?;
        let mut shaders = try_vec(count)?;
        shaders.push(desc.shader.clone());
        let mut shader_entries = try_vec(count)?;
        shader_entries.push(shader_entry);
        memory.resize(
            shared_cpu_bytes::<Self>()?
                .checked_add(pipeline_array_bytes(
                    shaders.capacity(),
                    shader_entries.capacity(),
                    0,
                    0,
                )?)
                .ok_or(crate::Error::OutOfHostMemory)?,
        )?;
        let metadata = BudgetedMetadata {
            data: PipelineArrays {
                shaders,
                entries: shader_entries,
                formats: None,
                target: None,
            },
            memory,
        };
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(desc.shader.handle)
            .name(&desc.entry);
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(desc.layout.handle);
        let sync_storage =
            crate::resource::SyncCell::reserve_arc_storage(&cache.device.allocator_stats)?;
        let _cache_use = cache.lock.read();
        let result = unsafe {
            cache
                .device
                .device
                .create_compute_pipelines(cache.handle, &[info], None)
        };
        let handle = finish_pipeline(&cache.device, result)?;
        let pipeline = Arc::new(Self {
            last_use: crate::resource::SyncCell::for_device(cache.device.generation())
                .into_reserved_arc(sync_storage),
            device: cache.device.clone(),
            handle,
            layout: desc.layout,
            point: vk::PipelineBindPoint::COMPUTE,
            shaders: metadata.data.shaders,
            shader_entries: metadata.data.entries,
            dynamic_rendering: metadata.data.formats,
            raster_target: metadata.data.target,
            _memory: metadata.memory,
        });
        created.pipeline = Some(pipeline.clone());
        Ok(pipeline)
    }
    pub fn create_raster_pipeline(
        cache: &PipelineCache,
        desc: RasterPipelineDesc,
    ) -> Result<Arc<Self>> {
        if cache.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if !Arc::ptr_eq(&cache.device, &desc.layout.device)
            || !Arc::ptr_eq(&cache.device, &desc.vertex.device)
            || desc
                .fragment
                .as_ref()
                .is_some_and(|s| !Arc::ptr_eq(&cache.device, &s.device))
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let vertex_entry = super::shader_interface::validate(
            &desc.vertex.interfaces,
            &desc.entry,
            vk::ShaderStageFlags::VERTEX,
            &desc.layout,
        )?;
        let fragment_entry = desc
            .fragment
            .as_ref()
            .map(|fragment| {
                super::shader_interface::validate(
                    &fragment.interfaces,
                    desc.fragment_entry.as_deref().unwrap_or(&desc.entry),
                    vk::ShaderStageFlags::FRAGMENT,
                    &desc.layout,
                )
            })
            .transpose()?;
        {
            let formats = desc.dynamic_rendering.as_ref().ok_or_else(|| {
                crate::Error::invalid_usage("raster pipeline requires dynamic attachment formats")
            })?;
            if !cache
                .device
                .caps()
                .is_enabled(crate::device::Feature::DynamicRendering)
            {
                return Err(crate::Error::unsupported(
                    "dynamicRendering feature was not enabled",
                ));
            }
            let properties = cache.device.properties();
            if properties
                .api_version
                .min(cache.device.instance.api_version)
                < vk::API_VERSION_1_3
                && !cache
                    .device
                    .enabled_extensions
                    .iter()
                    .any(|extension| extension == "VK_KHR_dynamic_rendering")
            {
                return Err(crate::Error::unsupported(
                    "dynamic rendering needs effective Vulkan 1.3 or enabled VK_KHR_dynamic_rendering",
                ));
            }
            let limits = properties.limits;
            formats.validate(limits.max_color_attachments)?;
            if formats.colors.len() != desc.blend_attachments.len() {
                return Err(crate::Error::invalid_usage(
                    "dynamic rendering color formats and blend attachments differ",
                ));
            }
            if (desc.depth_test || desc.depth_write) && formats.depth == vk::Format::UNDEFINED {
                return Err(crate::Error::invalid_usage(
                    "depth-enabled dynamic pipeline has no depth format",
                ));
            }
            if desc.stencil_test && formats.stencil == vk::Format::UNDEFINED {
                return Err(crate::Error::invalid_usage(
                    "stencil-enabled dynamic pipeline has no stencil format",
                ));
            }
            if desc.samples.as_raw().count_ones() != 1
                || (!formats.colors.is_empty()
                    && !limits
                        .framebuffer_color_sample_counts
                        .contains(desc.samples))
                || (formats.depth != vk::Format::UNDEFINED
                    && !limits
                        .framebuffer_depth_sample_counts
                        .contains(desc.samples))
                || (formats.stencil != vk::Format::UNDEFINED
                    && !limits
                        .framebuffer_stencil_sample_counts
                        .contains(desc.samples))
            {
                return Err(crate::Error::unsupported(
                    "dynamic rendering sample count exceeds framebuffer limits",
                ));
            }
        }
        let slot = slot_for(
            &cache.raster,
            |key| raster_pipeline_key_matches(&desc, key),
            || budgeted_raster_key(&cache.device.allocator_stats, &desc),
            cache.capacity,
            &cache.device.allocator_stats,
        )?;
        let mut created = slot.lock();
        if let Some(pipeline) = created.pipeline.as_ref() {
            return Ok(pipeline.clone());
        }
        let count = 1 + usize::from(desc.fragment.is_some());
        let formats = desc
            .dynamic_rendering
            .as_ref()
            .expect("validated dynamic formats");
        let mut memory = reserve_shared_cpu_metadata::<Self>(
            &cache.device.allocator_stats,
            pipeline_array_bytes(
                count,
                count,
                formats.colors.capacity(),
                formats.colors.len(),
            )?,
        )?;
        let mut shaders = try_vec(count)?;
        shaders.push(desc.vertex.clone());
        let mut shader_entries = try_vec(count)?;
        shader_entries.push(vertex_entry);
        let mut stages = smallvec::SmallVec::<[vk::PipelineShaderStageCreateInfo<'_>; 2]>::new();
        stages.push(stage(
            desc.vertex.handle,
            vk::ShaderStageFlags::VERTEX,
            &desc.entry,
        ));
        if let Some(fragment) = &desc.fragment {
            stages.push(stage(
                fragment.handle,
                vk::ShaderStageFlags::FRAGMENT,
                desc.fragment_entry.as_deref().unwrap_or(&desc.entry),
            ));
            shaders.push(fragment.clone());
            shader_entries.push(fragment_entry.expect("validated fragment entry"));
        }
        // The public target signature owns a distinct color Vec. Construct and
        // charge both actual allocations before entering Vulkan.
        let mut colors = try_vec(formats.colors.len())?;
        colors.extend_from_slice(&formats.colors);
        let target_formats = DynamicRenderingFormats {
            colors,
            depth: formats.depth,
            stencil: formats.stencil,
            view_mask: formats.view_mask,
        };
        memory.resize(
            shared_cpu_bytes::<Self>()?
                .checked_add(pipeline_array_bytes(
                    shaders.capacity(),
                    shader_entries.capacity(),
                    formats.colors.capacity(),
                    target_formats.colors.capacity(),
                )?)
                .ok_or(crate::Error::OutOfHostMemory)?,
        )?;
        let raster_target = Some(RasterTargetSignature::Dynamic {
            formats: target_formats,
            samples: desc.samples,
        });
        let metadata = BudgetedMetadata {
            data: PipelineArrays {
                shaders,
                entries: shader_entries,
                formats: desc.dynamic_rendering,
                target: raster_target,
            },
            memory,
        };
        let vertex = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&desc.vertex_bindings)
            .vertex_attribute_descriptions(&desc.vertex_attributes);
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(desc.topology);
        let viewport = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(desc.polygon_mode)
            .cull_mode(desc.cull_mode)
            .front_face(desc.front_face)
            .line_width(1.0);
        let multisample =
            vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(desc.samples);
        let depth = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(desc.depth_test)
            .depth_write_enable(desc.depth_write)
            .depth_compare_op(desc.depth_compare)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(desc.stencil_test)
            .front(desc.stencil)
            .back(desc.stencil)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0);
        let blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&desc.blend_attachments);
        let line_width_dynamic = cache.device.capabilities.wide_lines;
        let mut dynamic_values: smallvec::SmallVec<[vk::DynamicState; 4]> = smallvec::smallvec![
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::STENCIL_REFERENCE,
        ];
        if line_width_dynamic {
            dynamic_values.push(vk::DynamicState::LINE_WIDTH);
        }
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_values);
        let mut info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex)
            .input_assembly_state(&assembly)
            .viewport_state(&viewport)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .depth_stencil_state(&depth)
            .color_blend_state(&blend)
            .dynamic_state(&dynamic)
            .layout(desc.layout.handle);
        let mut rendering = vk::PipelineRenderingCreateInfo::default();
        if let Some(formats) = &metadata.data.formats {
            rendering = rendering
                .color_attachment_formats(&formats.colors)
                .depth_attachment_format(formats.depth)
                .stencil_attachment_format(formats.stencil)
                .view_mask(formats.view_mask);
            info = info.push_next(&mut rendering);
        }
        let sync_storage =
            crate::resource::SyncCell::reserve_arc_storage(&cache.device.allocator_stats)?;
        let _cache_use = cache.lock.read();
        let result = unsafe {
            cache
                .device
                .device
                .create_graphics_pipelines(cache.handle, &[info], None)
        };
        let handle = finish_pipeline(&cache.device, result)?;
        let pipeline = Arc::new(Self {
            last_use: crate::resource::SyncCell::for_device(cache.device.generation())
                .into_reserved_arc(sync_storage),
            device: cache.device.clone(),
            handle,
            layout: desc.layout,
            point: vk::PipelineBindPoint::GRAPHICS,
            shaders: metadata.data.shaders,
            shader_entries: metadata.data.entries,
            raster_target: metadata.data.target,
            dynamic_rendering: metadata.data.formats,
            _memory: metadata.memory,
        });
        created.pipeline = Some(pipeline.clone());
        Ok(pipeline)
    }
    /// # Safety
    /// command 必须处于正确录制状态，上层提交链应保持本对象至 GPU 完成。
    pub unsafe fn bind_pipeline(&self, command: vk::CommandBuffer) {
        unsafe {
            self.device
                .device
                .cmd_bind_pipeline(command, self.point, self.handle)
        }
    }
    pub fn shader_count(&self) -> usize {
        self.shaders.len()
    }
    pub fn raster_target_signature(&self) -> Option<&RasterTargetSignature> {
        self.raster_target.as_ref()
    }
    pub(crate) fn has_same_shader_interface(&self, other: &Self) -> bool {
        self.shaders.len() == other.shaders.len()
            && self
                .shaders
                .iter()
                .zip(&self.shader_entries)
                .zip(other.shaders.iter().zip(&other.shader_entries))
                .all(|((shader, &entry), (old, &old_entry))| {
                    shader.interfaces[entry].same_interface(&old.interfaces[old_entry])
                })
    }
    pub fn dynamic_rendering_formats(&self) -> Option<&DynamicRenderingFormats> {
        self.dynamic_rendering.as_ref()
    }
}
fn try_vec<T>(capacity: usize) -> Result<Vec<T>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    Ok(values)
}
fn pipeline_array_bytes(
    shaders: usize,
    entries: usize,
    formats: usize,
    target_formats: usize,
) -> Result<u64> {
    cpu_vec_bytes::<Arc<ShaderModule>>(shaders)?
        .checked_add(cpu_vec_bytes::<usize>(entries)?)
        .and_then(|bytes| bytes.checked_add(cpu_vec_bytes::<vk::Format>(formats).ok()?))
        .and_then(|bytes| bytes.checked_add(cpu_vec_bytes::<vk::Format>(target_formats).ok()?))
        .ok_or(crate::Error::OutOfHostMemory)
}
fn stage<'a>(
    module: vk::ShaderModule,
    kind: vk::ShaderStageFlags,
    entry: &'a CStr,
) -> vk::PipelineShaderStageCreateInfo<'a> {
    vk::PipelineShaderStageCreateInfo::default()
        .module(module)
        .stage(kind)
        .name(entry)
}
fn finish_pipeline(
    device: &Device,
    result: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
) -> Result<vk::Pipeline> {
    match result {
        Ok(values) => values
            .first()
            .copied()
            .ok_or_else(|| crate::Error::invalid_usage("着色器资源、接口或设备不匹配")),
        Err((values, error)) => {
            if error == vk::Result::ERROR_DEVICE_LOST {
                device.timelines.mark_device_lost();
            }
            for handle in values {
                unsafe { device.device.destroy_pipeline(handle, None) }
            }
            Err(error.into())
        }
    }
}
#[cfg(test)]
fn append_dynamic_rendering_key(key: &mut Vec<u64>, formats: Option<&DynamicRenderingFormats>) {
    match formats {
        None => key.push(0),
        Some(formats) => {
            key.extend([1, u64::from(formats.view_mask), formats.colors.len() as u64]);
            key.extend(formats.colors.iter().map(|format| format.as_raw() as u64));
            key.extend([
                formats.depth.as_raw() as u64,
                formats.stencil.as_raw() as u64,
            ]);
        }
    }
}

/// The key covers every RasterPipelineDesc field that affects the Vulkan pipeline;不使用不稳定 Debug 字串或哈希碰撞替代相等性。
pub(crate) fn raster_pipeline_key_len(desc: &RasterPipelineDesc) -> usize {
    let fragment = desc.fragment_entry.as_deref().unwrap_or(&desc.entry);
    // Nineteen fixed state words and four following sequence lengths. Native
    // render-pass/subpass fields were removed with the dynamic-only contract.
    23usize
        .saturating_add(
            desc.dynamic_rendering
                .as_ref()
                .map_or(1, |f| 5usize.saturating_add(f.colors.len())),
        )
        .saturating_add(desc.entry.as_bytes().len())
        .saturating_add(fragment.to_bytes().len())
        .saturating_add(desc.vertex_bindings.len().saturating_mul(3))
        .saturating_add(desc.vertex_attributes.len().saturating_mul(4))
        .saturating_add(desc.blend_attachments.len().saturating_mul(8))
}
pub(crate) fn raster_pipeline_key_matches(desc: &RasterPipelineDesc, key: &[u64]) -> bool {
    if key.len() != raster_pipeline_key_len(desc) {
        return false;
    }
    let mut index = 0;
    let mut equal = true;
    visit_raster_pipeline_key(desc, &mut |word| {
        equal &= key[index] == word;
        index += 1;
    });
    equal
}
pub(crate) fn raster_pipeline_key(desc: &RasterPipelineDesc) -> Vec<u64> {
    let mut key = Vec::with_capacity(raster_pipeline_key_len(desc));
    visit_raster_pipeline_key(desc, &mut |word| key.push(word));
    key
}
fn visit_raster_pipeline_key(desc: &RasterPipelineDesc, emit: &mut dyn FnMut(u64)) {
    struct Writer<'a>(&'a mut dyn FnMut(u64));
    impl Writer<'_> {
        fn push(&mut self, value: u64) {
            (self.0)(value);
        }
        fn extend(&mut self, values: impl IntoIterator<Item = u64>) {
            for value in values {
                self.push(value);
            }
        }
    }
    use ash::vk::Handle;
    let mut key = Writer(emit);
    key.extend([
        desc.vertex.handle.as_raw(),
        desc.fragment
            .as_ref()
            .map_or(0, |shader| shader.handle.as_raw()),
        desc.layout.handle.as_raw(),
        desc.topology.as_raw() as u64,
        desc.polygon_mode.as_raw() as u64,
        u64::from(desc.cull_mode.as_raw()),
        desc.front_face.as_raw() as u64,
        u64::from(desc.samples.as_raw()),
        u64::from(desc.depth_test),
        u64::from(desc.depth_write),
        desc.depth_compare.as_raw() as u64,
        u64::from(desc.stencil_test),
        desc.stencil.fail_op.as_raw() as u64,
        desc.stencil.pass_op.as_raw() as u64,
        desc.stencil.depth_fail_op.as_raw() as u64,
        desc.stencil.compare_op.as_raw() as u64,
        u64::from(desc.stencil.compare_mask),
        u64::from(desc.stencil.write_mask),
        desc.entry.as_bytes().len() as u64,
    ]);
    match desc.dynamic_rendering.as_ref() {
        None => key.push(0),
        Some(formats) => {
            key.extend([1, u64::from(formats.view_mask), formats.colors.len() as u64]);
            key.extend(formats.colors.iter().map(|format| format.as_raw() as u64));
            key.extend([
                formats.depth.as_raw() as u64,
                formats.stencil.as_raw() as u64,
            ]);
        }
    }
    key.extend(desc.entry.as_bytes().iter().map(|byte| u64::from(*byte)));
    let fragment_entry = desc.fragment_entry.as_deref().unwrap_or(&desc.entry);
    key.push(fragment_entry.to_bytes().len() as u64);
    key.extend(
        fragment_entry
            .to_bytes()
            .iter()
            .map(|byte| u64::from(*byte)),
    );
    key.push(desc.vertex_bindings.len() as u64);
    for binding in &desc.vertex_bindings {
        key.extend([
            u64::from(binding.binding),
            u64::from(binding.stride),
            binding.input_rate.as_raw() as u64,
        ]);
    }
    key.push(desc.vertex_attributes.len() as u64);
    for attribute in &desc.vertex_attributes {
        key.extend([
            u64::from(attribute.location),
            u64::from(attribute.binding),
            attribute.format.as_raw() as u64,
            u64::from(attribute.offset),
        ]);
    }
    key.push(desc.blend_attachments.len() as u64);
    for blend in &desc.blend_attachments {
        key.extend([
            u64::from(blend.blend_enable),
            blend.src_color_blend_factor.as_raw() as u64,
            blend.dst_color_blend_factor.as_raw() as u64,
            blend.color_blend_op.as_raw() as u64,
            blend.src_alpha_blend_factor.as_raw() as u64,
            blend.dst_alpha_blend_factor.as_raw() as u64,
            blend.alpha_blend_op.as_raw() as u64,
            u64::from(blend.color_write_mask.as_raw()),
        ]);
    }
}
impl Drop for VulkanPipeline {
    fn drop(&mut self) {
        self.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::Pipeline(self.handle),
        );
    }
}

/// 原 spirq 分叉生成的已保存反射条目；通用 SPIR-V 解析不在本模块重写。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedEntry {
    pub entry_point: String,
    pub stage: String,
    /// Concrete default workgroup dimensions for this compute entry. None
    /// means no supported execution-size contract was resolved; hot reload
    /// cannot assume compatibility for such an entry.
    #[serde(default)]
    pub local_size: Option<[u32; 3]>,
    pub descriptors: Vec<ReflectedDescriptor>,
    #[serde(default)]
    pub push_constants: Vec<ReflectedPushConstant>,
    #[serde(default)]
    pub inputs: Vec<ReflectedLocation>,
    #[serde(default)]
    pub outputs: Vec<ReflectedLocation>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedLocation {
    pub location: u32,
    pub component: u32,
    #[serde(rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedDescriptor {
    pub set: u32,
    pub binding: u32,
    pub name: String,
    pub count: u32,
    pub descriptor_type: String,
    #[serde(default, rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedPushConstant {
    #[serde(rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterfaceError {
    Stage(String),
    Descriptor(String),
    IncompatibleBinding(u32, u32),
    MissingSampler(String),
    InvalidPushConstant,
    RuntimeBindingNotLast(u32, u32),
    InvalidLayout,
    IncompatibleLocation(u32, u32),
    /// A vertex input has no byte size (`Type::nbyte()` is None, e.g. an
    /// opaque resource type).
    VertexInputByteCount,
    /// Unsupported vertex attribute type; the payload is the type's Debug text.
    UnsupportedVertexAttributeType(String),
    /// A compute shader declares stage inputs.
    ComputeHasVertexInputs,
    /// A compute shader has no workgroup size (`local_size` is None).
    MissingGroupSize,
}
pub mod original_panics {
    pub const COMPUTE_INPUTS_NOT_EMPTY: &str =
        "assertion failed: compiled.reflection.inputs.is_empty()";
    pub const VALID_GROUP_SIZE: &str = "a valid group size";
    pub const FAILED_CREATING_SHADER_PIPELINE_PREFIX: &str = "Failed creating shader pipeline ";
    pub const FAILED_CREATING_SHADER_PIPELINE_SEP: &str = ": ";
    pub const UNABLE_TO_CREATE_GRAPHICS_PIPELINE: &str = "Unable to create graphics pipeline";
    pub const VERTEX_INPUT_ATTRIB_BYTE_COUNT: &str = "vertex input attrib byte count";
    pub const UNSUPPORTED_VERTEX_ATTRIBUTE_TYPE_PREFIX: &str =
        "Unsupported vertex attribute type: ";
    pub const UNWRAP_ON_ERR: &str = "called `Result::unwrap()` on an `Err` value";

    /// compute 管线创建失败（453:39）的完整 panic 文本。
    pub fn failed_creating_shader_pipeline(name: &str, err: &dyn std::fmt::Debug) -> String {
        format!(
            "{FAILED_CREATING_SHADER_PIPELINE_PREFIX}{name}{FAILED_CREATING_SHADER_PIPELINE_SEP}{err:?}"
        )
    }
}
/// Reflection checks before compute pipeline creation: no stage inputs, then
/// a known local size.
pub fn validate_compute_reflection(
    reflection: &crate::shader_pipeline::reflection::ShaderReflection,
) -> Result<[u32; 3]> {
    if !reflection.vertex_inputs.is_empty() {
        return Err(InterfaceError::ComputeHasVertexInputs.into());
    }
    reflection
        .local_size
        .ok_or_else(|| InterfaceError::MissingGroupSize.into())
}
impl std::fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VertexInputByteCount => {
                f.write_str(original_panics::VERTEX_INPUT_ATTRIB_BYTE_COUNT)
            }
            Self::UnsupportedVertexAttributeType(ty) => {
                write!(
                    f,
                    "{}{ty}",
                    original_panics::UNSUPPORTED_VERTEX_ATTRIBUTE_TYPE_PREFIX
                )
            }
            Self::ComputeHasVertexInputs => f.write_str(original_panics::COMPUTE_INPUTS_NOT_EMPTY),
            Self::MissingGroupSize => f.write_str(original_panics::VALID_GROUP_SIZE),
            Self::MissingSampler(name) => write!(f, "Sampler not found: {name}"),
            _ => write!(f, "着色器接口无效：{self:?}"),
        }
    }
}
impl std::error::Error for InterfaceError {}
impl From<InterfaceError> for crate::Error {
    fn from(source: InterfaceError) -> Self {
        Self::invalid_usage_with("着色器接口无效", source)
    }
}

pub struct ShaderInterfacePlan {
    /// 保留原反射 set/binding 编号；缺失的 set 用空布局填充，绝不重新枚举 sparse binding。
    pub sets: Vec<LayoutPlan>,
    pub push_constants: Vec<vk::PushConstantRange>,
}
pub(super) fn reflected_stage(stage: &str) -> Result<vk::ShaderStageFlags> {
    match stage {
        "GLCompute" => Ok(vk::ShaderStageFlags::COMPUTE),
        "Vertex" => Ok(vk::ShaderStageFlags::VERTEX),
        "Fragment" => Ok(vk::ShaderStageFlags::FRAGMENT),
        other => Err(InterfaceError::Stage(other.to_owned()).into()),
    }
}
impl ShaderInterfacePlan {
    pub fn from_reflection(
        entries: &[ReflectedEntry],
        immutable_samplers: &BTreeMap<String, u64>,
        runtime_device_scalar: u32,
    ) -> Result<Self> {
        if let (Some(vertex), Some(fragment)) = (
            entries.iter().find(|entry| entry.stage == "Vertex"),
            entries.iter().find(|entry| entry.stage == "Fragment"),
        ) {
            for input in &fragment.inputs {
                let output = vertex
                    .outputs
                    .iter()
                    .find(|output| {
                        output.location == input.location && output.component == input.component
                    })
                    .ok_or(InterfaceError::IncompatibleLocation(
                        input.location,
                        input.component,
                    ))?;
                if canonical_type(&input.reflected_type) != canonical_type(&output.reflected_type) {
                    return Err(InterfaceError::IncompatibleLocation(
                        input.location,
                        input.component,
                    )
                    .into());
                }
            }
        }
        let mut merged = BTreeMap::<(u32, u32), (Binding, vk::ShaderStageFlags)>::new();
        let mut pushes = BTreeMap::<(u32, u32), vk::ShaderStageFlags>::new();
        for entry in entries {
            let stage = reflected_stage(&entry.stage)?;
            for descriptor in &entry.descriptors {
                let tag = match descriptor.descriptor_type.as_str() {
                    "StorageBuffer(ReadWrite)" | "StorageBuffer(ReadOnly)" => 0,
                    "UniformBuffer" => 1,
                    "CombinedImageSampler" => 2,
                    "StorageImage(ReadWrite)" | "StorageImage(ReadOnly)" => 3,
                    "SampledImage" => 4,
                    "Sampler" => 5,
                    other => return Err(InterfaceError::Descriptor(other.to_owned()).into()),
                };
                let sampler = if tag == 5 {
                    *immutable_samplers
                        .get(&descriptor.name)
                        .filter(|v| **v != 0)
                        .ok_or_else(|| InterfaceError::MissingSampler(descriptor.name.clone()))?
                } else {
                    0
                };
                let value = Binding {
                    tag,
                    name: descriptor.name.clone(),
                    count: descriptor.count,
                    runtime: descriptor.count == 0,
                    sampler,
                };
                match merged.entry((descriptor.set, descriptor.binding)) {
                    std::collections::btree_map::Entry::Vacant(slot) => {
                        slot.insert((value, stage));
                    }
                    std::collections::btree_map::Entry::Occupied(mut slot) => {
                        let (prior, stages) = slot.get_mut();
                        // 名称仅在 _dyn 分类与 immutable sampler 策略中影响布局；跨 stage 别名允许合并。
                        if prior.tag != value.tag
                            || prior.count != value.count
                            || prior.sampler != value.sampler
                            || prior.name.ends_with("_dyn") != value.name.ends_with("_dyn")
                        {
                            return Err(InterfaceError::IncompatibleBinding(
                                descriptor.set,
                                descriptor.binding,
                            )
                            .into());
                        }
                        *stages |= stage;
                    }
                }
            }
            for push in &entry.push_constants {
                let size = push.reflected_type["minimum_bytes"]
                    .as_u64()
                    .and_then(|size| u32::try_from(size).ok())
                    .filter(|size| *size > 0 && *size % 4 == 0)
                    .ok_or(InterfaceError::InvalidPushConstant)?;
                // 当前原 catalog 的 push block 始于 0；大小来自原 fork 反射，而非猜测成员数量。
                *pushes.entry((0, size)).or_default() |= stage;
            }
        }
        let set_count = merged
            .keys()
            .map(|(set, _)| *set)
            .max()
            .map_or(0, |set| set as usize + 1);
        if set_count > 32 {
            return Err(InterfaceError::InvalidLayout.into());
        }
        let mut sets = (0..set_count)
            .map(|_| LayoutPlan {
                flags: 0,
                bindings: Vec::new(),
                binding_flags: Vec::new(),
            })
            .collect::<Vec<_>>();
        for ((set, binding), (value, stages)) in merged {
            let plan =
                descriptor_set_layout(&[value], u64::from(binding), stages, runtime_device_scalar)
                    .map_err(|_| InterfaceError::InvalidLayout)?;
            let target = &mut sets[set as usize];
            target.flags |= plan.flags;
            target.bindings.extend(plan.bindings);
            target.binding_flags.extend(plan.binding_flags);
        }
        for (set, plan) in sets.iter().enumerate() {
            for (index, flags) in plan.binding_flags.iter().enumerate() {
                if flags & vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw() != 0
                    && index + 1 != plan.bindings.len()
                {
                    return Err(InterfaceError::RuntimeBindingNotLast(
                        set as u32,
                        plan.bindings[index].binding,
                    )
                    .into());
                }
            }
        }
        // Vulkan 不允许同一 stage 在多个 push 范围中出现；合并该 stage 的最大范围。
        let mut by_stage = BTreeMap::<u32, u32>::new();
        for ((_, size), flags) in pushes {
            for stage in [
                vk::ShaderStageFlags::VERTEX,
                vk::ShaderStageFlags::FRAGMENT,
                vk::ShaderStageFlags::COMPUTE,
            ] {
                if flags.contains(stage) {
                    by_stage
                        .entry(stage.as_raw())
                        .and_modify(|s| *s = (*s).max(size))
                        .or_insert(size);
                }
            }
        }
        let mut grouped = BTreeMap::<u32, vk::ShaderStageFlags>::new();
        for (stage, size) in by_stage {
            *grouped.entry(size).or_default() |= vk::ShaderStageFlags::from_raw(stage);
        }
        Ok(Self {
            sets,
            push_constants: grouped
                .into_iter()
                .map(|(size, stage_flags)| vk::PushConstantRange {
                    stage_flags,
                    offset: 0,
                    size,
                })
                .collect(),
        })
    }
    pub fn create_descriptor_set_layouts(
        &self,
        device: Arc<Device>,
        samplers: &[Arc<crate::raw::sampler::Sampler>],
    ) -> Result<Vec<Arc<VulkanDescriptorSetLayout>>> {
        if self.sets.len() > super::descriptor_set::MAX_DESCRIPTOR_SETS {
            return Err(crate::Error::invalid_usage("描述符集合数超过模块上限"));
        }
        self.sets
            .iter()
            .map(|plan| {
                VulkanDescriptorSetLayout::new_with_samplers(
                    device.clone(),
                    plan,
                    samplers.to_vec(),
                )
            })
            .collect()
    }
    pub fn create_pipeline_layout(
        &self,
        device: Arc<Device>,
        samplers: &[Arc<crate::raw::sampler::Sampler>],
    ) -> Result<Arc<PipelineLayout>> {
        let layouts = self.create_descriptor_set_layouts(device.clone(), samplers)?;
        PipelineLayout::new(device, layouts, self.push_constants.clone())
    }
}
/// 去掉反射调试名字后比较结构；仅处理已有 schema，不解析 SPIR-V 或重写公开反射库。
pub(super) fn canonical_type(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .filter(|(key, _)| key.as_str() != "name")
                .map(|(key, child)| (key.clone(), canonical_type(child)))
                .collect(),
        ),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.iter().map(canonical_type).collect())
        }
        other => other.clone(),
    }
}

/// Blend state from the two blend-mode bits (5..=6) of the encoded raster flags.
pub fn get_vk_blend_mode(encoded_raster_flags: u8) -> vk::PipelineColorBlendAttachmentState {
    let mode = usize::from(encoded_raster_flags >> 5 & 3);
    let enabled = [0, 1, 1, 1][mode];
    let dst_color = [0, 1, 7, 16][mode];
    let dst_alpha = [0, 1, 7, 18][mode];
    vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(enabled != 0)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::from_raw(dst_color))
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::from_raw(dst_alpha))
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(vk::ColorComponentFlags::RGBA)
}
/// 原类型 tag 2/3/4 对应 32 位有符号/无符号/浮点，字节宽度仅允许 4/8/12/16。
pub fn map_vertex_attribute_format(type_tag: u32, bytes: u64) -> Result<vk::Format> {
    let base = match type_tag {
        2 => 99,
        3 => 98,
        4 => 100,
        _ => return Err(InterfaceError::InvalidLayout.into()),
    };
    if !(4..=16).contains(&bytes) || !bytes.is_multiple_of(4) {
        return Err(InterfaceError::InvalidLayout.into());
    }
    Ok(vk::Format::from_raw(base + 3 * (bytes as i32 / 4 - 1)))
}

pub fn vertex_input_from_reflection(
    reflection: &crate::shader_pipeline::reflection::ShaderReflection,
) -> Result<(
    Vec<vk::VertexInputBindingDescription>,
    Vec<vk::VertexInputAttributeDescription>,
)> {
    use crate::shader_pipeline::reflection_ty::{ScalarType, Type};
    let mut bindings = Vec::new();
    let mut attributes = Vec::new();
    for input in reflection.effective_vertex_inputs().into_values() {
        let bytes = input
            .ty
            .nbyte()
            .ok_or(InterfaceError::VertexInputByteCount)? as u64;
        let scalar = match &input.ty {
            Type::Scalar(s) => s,
            Type::Vector(v) => &v.scalar_ty,
            other => {
                return Err(
                    InterfaceError::UnsupportedVertexAttributeType(format!("{other:?}")).into(),
                );
            }
        };
        // The reflection schema stores scalar *bytes*, not SPIR-V bits: the
        // attribute format comes from scalar.nbyte() * N and the scalar tag,
        // and the vertex stride from the full Type::nbyte.
        let tag = match scalar {
            ScalarType::Signed(_) => 2,
            ScalarType::Unsigned(_) => 3,
            ScalarType::Float(_) => 4,
            _ => return Err(InterfaceError::InvalidLayout.into()),
        };
        let format = map_vertex_attribute_format(tag, bytes)?;
        bindings.push(
            vk::VertexInputBindingDescription::default()
                .binding(input.location)
                .stride(bytes as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
        );
        attributes.push(
            vk::VertexInputAttributeDescription::default()
                .location(input.location)
                .binding(input.location)
                .format(format)
                .offset(0),
        );
    }
    Ok((bindings, attributes))
}

/// 已解析发行容器中的一个模块。容器 offset/word_count 来自原 spirq catalog。
#[derive(Clone)]
pub struct ReflectedShader {
    pub module: Arc<ShaderModule>,
    pub entry: ReflectedEntry,
}
impl ReflectedShader {
    pub fn from_compiled_container(
        device: Arc<Device>,
        container: &[u8],
        metadata: &serde_json::Value,
        entry_name: &str,
    ) -> Result<Self> {
        let offset = metadata["offset"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or_else(invalid_shader)?;
        let len = metadata["word_count"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .and_then(|v| v.checked_mul(4))
            .ok_or_else(invalid_shader)?;
        let payload = container
            .get(offset..offset.checked_add(len).ok_or_else(invalid_shader)?)
            .ok_or_else(invalid_shader)?;
        let entry: ReflectedEntry = serde_json::from_value(
            metadata["entries"]
                .as_array()
                .and_then(|entries| {
                    entries
                        .iter()
                        .find(|entry| entry["entry_point"] == entry_name)
                })
                .cloned()
                .ok_or_else(invalid_shader)?,
        )
        .map_err(|_| invalid_shader())?;
        let parsed = rspirv::dr::load_bytes(payload).map_err(|_| invalid_shader())?;
        let expected = match entry.stage.as_str() {
            "GLCompute" => rspirv::spirv::ExecutionModel::GLCompute,
            "Vertex" => rspirv::spirv::ExecutionModel::Vertex,
            "Fragment" => rspirv::spirv::ExecutionModel::Fragment,
            _ => return Err(invalid_shader()),
        };
        let present = parsed.entry_points.iter().any(|point| matches!(
            (point.operands.first(), point.operands.get(2)),
            (Some(rspirv::dr::Operand::ExecutionModel(stage)), Some(rspirv::dr::Operand::LiteralString(name)))
            if *stage == expected && name == entry_name
        ));
        if !present {
            return Err(invalid_shader());
        }
        Ok(Self {
            module: ShaderModule::from_spirv(device, payload)?,
            entry,
        })
    }
}

/// 按资产键选择真实 shader，自动生产布局与计算管线；不假定文件名就是 country-core pass 键。
pub struct ShaderLibrary {
    pub cache: PipelineCache,
    shaders: BTreeMap<String, ReflectedShader>,
    layouts: BTreeMap<String, Arc<PipelineLayout>>,
    compiled_rasters:
        BTreeMap<(String, String), Arc<crate::shader_pipeline::compiled::CompiledShaderPipeline>>,
    compiled_computes:
        BTreeMap<String, Arc<crate::shader_pipeline::compiled::CompiledShaderPipeline>>,
}
pub struct RasterProgram {
    pub vertex: ReflectedShader,
    pub fragment: ReflectedShader,
    pub layout: Arc<PipelineLayout>,
}
impl RasterProgram {
    /// 图形固定状态仍由材质/附件消费者提供；shader/entry/layout 则由资产选择生产。
    pub fn pipeline_desc(&self, mut state: RasterPipelineDesc) -> Result<RasterPipelineDesc> {
        state.vertex = self.vertex.module.clone();
        state.fragment = Some(self.fragment.module.clone());
        state.layout = self.layout.clone();
        state.entry =
            CString::new(self.vertex.entry.entry_point.as_str()).map_err(|_| invalid_shader())?;
        state.fragment_entry = Some(
            CString::new(self.fragment.entry.entry_point.as_str()).map_err(|_| invalid_shader())?,
        );
        Ok(state)
    }
}
impl ShaderLibrary {
    pub fn new(device: Arc<Device>) -> Result<Self> {
        Ok(Self {
            cache: PipelineCache::new(device, &[])?,
            shaders: BTreeMap::new(),
            layouts: BTreeMap::new(),
            compiled_rasters: BTreeMap::new(),
            compiled_computes: BTreeMap::new(),
        })
    }
    pub fn register(&mut self, key: String, shader: ReflectedShader) -> Result<()> {
        if !Arc::ptr_eq(&self.cache.device, &shader.module.device) {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        self.layouts.clear();
        self.cache.invalidate();
        self.compiled_computes.remove(&key);
        self.compiled_rasters
            .retain(|(vertex, fragment), _| vertex != &key && fragment != &key);
        self.shaders.insert(key, shader);
        Ok(())
    }
    pub fn create_compute_pipeline(
        &mut self,
        key: &str,
        samplers: &BTreeMap<String, Arc<crate::raw::sampler::Sampler>>,
    ) -> Result<Arc<VulkanPipeline>> {
        let runtime_device_scalar = self
            .cache
            .device
            .max_descriptor_set_update_after_bind_sampled_images();
        use ash::vk::Handle;
        if samplers
            .values()
            .any(|sampler| !Arc::ptr_eq(&self.cache.device, &sampler.device))
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let shader = self.shaders.get(key).ok_or_else(invalid_shader)?;
        if shader.entry.stage != "GLCompute" {
            return Err(invalid_shader());
        }
        // sampler 句柄属于布局键；修改 sampler 配置不能复用旧 layout。
        let sampler_map = samplers
            .iter()
            .map(|(name, sampler)| (name.clone(), sampler.handle.as_raw()))
            .collect::<BTreeMap<_, _>>();
        let mut by_name = BTreeMap::new();
        for descriptor in &shader.entry.descriptors {
            if descriptor.descriptor_type == "Sampler" {
                let handle = sampler_map
                    .get(&descriptor.name)
                    .copied()
                    .ok_or_else(|| crate::Error::invalid_usage("着色器资源、接口或设备不匹配"))?;
                by_name.insert(descriptor.name.clone(), handle);
            }
        }
        let plan = if let Some(compiled) = self.compiled_computes.get(key) {
            // Compute reflection checks: no stage inputs, known local size.
            validate_compute_reflection(&compiled.reflection)?;
            ShaderInterfacePlan::from_compiled_pipeline(compiled, &by_name, runtime_device_scalar)?
        } else {
            ShaderInterfacePlan::from_reflection(
                std::slice::from_ref(&shader.entry),
                &by_name,
                runtime_device_scalar,
            )?
        };
        let layout_key = serde_json::to_string(&(key, &plan.sets, runtime_device_scalar))
            .map_err(|error| crate::Error::invalid_usage_with("无法编码计算管线布局键", error))?;
        let layout = match self.layouts.get(&layout_key) {
            Some(layout) => layout.clone(),
            None => {
                let retained = samplers.values().cloned().collect::<Vec<_>>();
                let layout = plan.create_pipeline_layout(self.cache.device.clone(), &retained)?;
                self.layouts.insert(layout_key, layout.clone());
                layout
            }
        };
        VulkanPipeline::create_compute_pipeline(
            &self.cache,
            ComputePipelineDesc {
                shader: shader.module.clone(),
                entry: CString::new(shader.entry.entry_point.as_str())
                    .map_err(|_| invalid_shader())?,
                layout,
            },
        )
    }
    pub fn invalidate(&mut self, key: &str) {
        self.shaders.remove(key);
        self.compiled_computes.remove(key);
        self.compiled_rasters
            .retain(|(vertex, fragment), _| vertex != key && fragment != key);
        self.layouts.clear();
        self.cache.invalidate();
    }
    pub fn prepare_raster_program(
        &mut self,
        vertex_key: &str,
        fragment_key: &str,
        samplers: &BTreeMap<String, Arc<crate::raw::sampler::Sampler>>,
    ) -> Result<RasterProgram> {
        self.prepare_raster_program_with_runtime_limit(vertex_key, fragment_key, samplers, u32::MAX)
    }
    /// Bound runtime arrays to the consumer's real resolver capacity. The
    /// device limit and Vulkan layout-support checks remain authoritative.
    pub fn prepare_raster_program_with_runtime_limit(
        &mut self,
        vertex_key: &str,
        fragment_key: &str,
        samplers: &BTreeMap<String, Arc<crate::raw::sampler::Sampler>>,
        max_runtime_descriptors: u32,
    ) -> Result<RasterProgram> {
        if max_runtime_descriptors == 0 {
            return Err(crate::Error::invalid_usage(
                "runtime descriptor capacity must be nonzero",
            ));
        }
        let runtime_device_scalar = self
            .cache
            .device
            .max_descriptor_set_update_after_bind_sampled_images()
            .min(max_runtime_descriptors.saturating_add(32));
        use ash::vk::Handle;
        if samplers
            .values()
            .any(|sampler| !Arc::ptr_eq(&self.cache.device, &sampler.device))
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let vertex = self
            .shaders
            .get(vertex_key)
            .cloned()
            .ok_or_else(invalid_shader)?;
        let fragment = self
            .shaders
            .get(fragment_key)
            .cloned()
            .ok_or_else(invalid_shader)?;
        if vertex.entry.stage != "Vertex" || fragment.entry.stage != "Fragment" {
            return Err(invalid_shader());
        }
        let sampler_map = samplers
            .iter()
            .map(|(name, sampler)| (name.clone(), sampler.handle.as_raw()))
            .collect();
        let plan = if let Some(compiled) = self
            .compiled_rasters
            .get(&(vertex_key.to_owned(), fragment_key.to_owned()))
        {
            ShaderInterfacePlan::from_compiled_pipeline(
                compiled,
                &sampler_map,
                runtime_device_scalar,
            )?
        } else {
            ShaderInterfacePlan::from_reflection(
                &[vertex.entry.clone(), fragment.entry.clone()],
                &sampler_map,
                runtime_device_scalar,
            )?
        };
        let key = serde_json::to_string(&(
            vertex_key,
            fragment_key,
            &plan.sets,
            runtime_device_scalar,
        ))
        .map_err(|error| crate::Error::invalid_usage_with("无法编码图形管线布局键", error))?;
        let layout = match self.layouts.get(&key) {
            Some(layout) => layout.clone(),
            None => {
                let retained = samplers.values().cloned().collect::<Vec<_>>();
                let layout = plan.create_pipeline_layout(self.cache.device.clone(), &retained)?;
                self.layouts.insert(key, layout.clone());
                layout
            }
        };
        Ok(RasterProgram {
            vertex,
            fragment,
            layout,
        })
    }
}
/// Reflect full module resource schemas using the shared SPIR-Q type/layout implementation.
pub fn reflect_spirv_entries(words: &[u32]) -> Result<Vec<ReflectedEntry>> {
    reflect_spirv_entries_with_resources(words, true)
}
pub(crate) fn reflect_spirv_entries_used(words: &[u32]) -> Result<Vec<ReflectedEntry>> {
    use rspirv::{binary::Assemble, dr::Operand, spirv::Op};
    use std::collections::{HashMap, HashSet};
    let module = rspirv::dr::load_words(words).map_err(|_| invalid_shader())?;
    let functions = module
        .functions
        .iter()
        .map(|function| {
            function
                .def
                .as_ref()
                .and_then(|definition| definition.result_id)
                .map(|id| (id, function))
                .ok_or_else(invalid_shader)
        })
        .collect::<Result<HashMap<_, _>>>()?;
    let globals = module
        .types_global_values
        .iter()
        .filter(|instruction| instruction.class.opcode == Op::Variable)
        .filter_map(|instruction| instruction.result_id)
        .collect::<HashSet<_>>();
    let mut result = Vec::new();
    for entry in &module.entry_points {
        let Some(Operand::IdRef(root)) = entry.operands.get(1) else {
            return Err(invalid_shader());
        };
        let mut pending = vec![*root];
        let mut reached = HashSet::new();
        let mut used = HashSet::new();
        while let Some(id) = pending.pop() {
            if !reached.insert(id) {
                continue;
            }
            let function = functions.get(&id).ok_or_else(invalid_shader)?;
            for instruction in function.all_inst_iter() {
                if instruction.class.opcode == Op::FunctionCall {
                    let Some(Operand::IdRef(callee)) = instruction.operands.first() else {
                        return Err(invalid_shader());
                    };
                    pending.push(*callee);
                }
                for operand in &instruction.operands {
                    if let Operand::IdRef(id) = operand
                        && globals.contains(id)
                    {
                        used.insert(*id);
                    }
                }
            }
        }
        // The pinned SPIR-Q fork consumes function instructions in its definition
        // scan, so ref_all_rscs(false) silently misses every static resource use.
        // Select variables from the real call tree, then reuse its type/layout
        // reflection on this private view. The driver's module stays unchanged.
        let mut view = module.clone();
        view.entry_points = vec![entry.clone()];
        let interface = &mut view.entry_points[0].operands;
        let mut position = 0;
        interface.retain(|operand| {
            position += 1;
            position <= 3 || !matches!(operand, Operand::IdRef(id) if globals.contains(id) && !used.contains(id))
        });
        view.functions.retain(|function| {
            function
                .def
                .as_ref()
                .and_then(|definition| definition.result_id)
                .is_some_and(|id| reached.contains(&id))
        });
        view.types_global_values.retain(|instruction| {
            instruction.class.opcode != Op::Variable
                || instruction.result_id.is_some_and(|id| used.contains(&id))
        });
        result.extend(reflect_spirv_entries_with_resources(
            &view.assemble(),
            true,
        )?);
    }
    Ok(result)
}
fn reflect_spirv_entries_with_resources(
    words: &[u32],
    all_resources: bool,
) -> Result<Vec<ReflectedEntry>> {
    if words.len() < 5 || words.first().copied() != Some(0x07230203) {
        return Err(invalid_shader());
    }
    // This pinned reflector seeks a name section before decorations and types.
    // OpName is optional; add one to a reflection-only copy when absent.
    let mut module = rspirv::dr::load_words(words).map_err(|_| invalid_shader())?;
    let normalized;
    let words = if module.debug_names.is_empty() {
        let function = module
            .entry_points
            .first()
            .and_then(|entry| entry.operands.get(1))
            .and_then(|operand| {
                if let rspirv::dr::Operand::IdRef(id) = operand {
                    Some(*id)
                } else {
                    None
                }
            })
            .ok_or_else(invalid_shader)?;
        module.debug_names.push(rspirv::dr::Instruction::new(
            rspirv::spirv::Op::Name,
            None,
            None,
            vec![
                rspirv::dr::Operand::IdRef(function),
                rspirv::dr::Operand::LiteralString("rhapsody_reflection_entry".into()),
            ],
        ));
        use rspirv::binary::Assemble;
        normalized = module.assemble();
        normalized.as_slice()
    } else {
        words
    };
    use spirq::{ReflectConfig, SpirvBinary, Variable};
    let reflected = ReflectConfig::new()
        .spv(SpirvBinary::from(words))
        .ref_all_rscs(all_resources)
        .combine_img_samplers(false)
        .gen_unique_names(false)
        .reflect()
        .map_err(|_| invalid_shader())?;
    reflected
        .into_iter()
        .map(|entry| {
            let mut local_size = None;
            for mode in &entry.exec_modes {
                if matches!(
                    format!("{:?}", mode.exec_mode).as_str(),
                    "LocalSize" | "LocalSizeId"
                ) {
                    if mode.operands.len() != 3 {
                        return Err(invalid_shader());
                    }
                    // Pipeline descriptors currently supply no specialization
                    // overrides, so reflected constant defaults are effective.
                    let size = [
                        mode.operands[0].value.to_u32(),
                        mode.operands[1].value.to_u32(),
                        mode.operands[2].value.to_u32(),
                    ];
                    if size.contains(&0) || local_size.is_some_and(|old| old != size) {
                        return Err(invalid_shader());
                    }
                    local_size = Some(size);
                }
            }
            let mut descriptors = Vec::new();
            let mut inputs = Vec::new();
            let mut outputs = Vec::new();
            let mut push_constants = Vec::new();
            for variable in entry.vars {
                match variable {
                    Variable::Descriptor {
                        name,
                        desc_bind,
                        desc_ty,
                        ty,
                        nbind,
                    } => {
                        descriptors.push(ReflectedDescriptor {
                            set: desc_bind.set(),
                            binding: desc_bind.bind(),
                            name: name.unwrap_or_default(),
                            count: nbind,
                            descriptor_type: format!("{desc_ty:?}"),
                            reflected_type: reflected_type_json(&ty),
                        });
                    }
                    Variable::Input { location, ty, .. } => inputs.push(ReflectedLocation {
                        location: location.loc(),
                        component: location.comp(),
                        reflected_type: reflected_type_json(&ty),
                    }),
                    Variable::Output { location, ty, .. } => outputs.push(ReflectedLocation {
                        location: location.loc(),
                        component: location.comp(),
                        reflected_type: reflected_type_json(&ty),
                    }),
                    Variable::PushConstant { ty, .. } => {
                        push_constants.push(ReflectedPushConstant {
                            reflected_type: reflected_type_json(&ty),
                        })
                    }
                    // 特殊化常量由调用者另行提供，当前运行时接口计划不生成默认值。
                    Variable::SpecConstant { .. } => {}
                }
            }
            descriptors.sort_by_key(|value| (value.set, value.binding));
            inputs.sort_by_key(|value| (value.location, value.component));
            outputs.sort_by_key(|value| (value.location, value.component));
            Ok(ReflectedEntry {
                entry_point: entry.name,
                stage: format!("{:?}", entry.exec_model),
                local_size,
                descriptors,
                inputs,
                outputs,
                push_constants,
            })
        })
        .collect()
}

/// 复用既有公开fork反射字段投影，类型解析仍由spirq执行。
fn reflected_type_json(ty: &spirq::ty::Type) -> serde_json::Value {
    use serde_json::json;
    use spirq::ty::Type;
    let detail = match ty {
        Type::Struct(value) => {
            json!({"kind":"Struct","members":value.members.iter().map(|member| json!({"name":member.name,"offset":member.offset,"type":reflected_type_json(&member.ty)})).collect::<Vec<_>>()})
        }
        Type::Array(value) => {
            json!({"kind":"Array","count":value.nrepeat,"stride":value.stride,"element":reflected_type_json(&value.proto_ty)})
        }
        Type::Matrix(value) => {
            json!({"kind":"Matrix","axis_order":value.major.map(|major|format!("{major:?}")),"vector":value.vec_ty.to_string(),"count":value.nvec,"stride":value.stride})
        }
        Type::DevicePointer(value) => {
            json!({"kind":"DevicePointer","target":reflected_type_json(&value.pointee_ty)})
        }
        _ => json!({"kind":ty.to_string()}),
    };
    json!({"minimum_bytes":ty.nbyte(),"detail":detail})
}

/// 从真实容器stage记录取得SPIR-V，不使用外部catalog偏移或magic扫描。
/// 序列化stage/入口名与payload不一致时，在Vulkan对象创建前拒绝。
pub fn compiled_pipeline_entries(
    compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
) -> Result<Vec<ReflectedEntry>> {
    use crate::shader_pipeline::compiled::ShaderStage;
    compiled
        .stages
        .iter()
        .map(|stage| {
            let expected = match stage.stage {
                ShaderStage::Compute => "GLCompute",
                ShaderStage::Vertex => "Vertex",
                ShaderStage::Fragment => "Fragment",
            };
            reflect_spirv_entries(&stage.spirv)?
                .into_iter()
                .find(|entry| entry.entry_point == stage.entry_point && entry.stage == expected)
                .ok_or_else(invalid_shader)
        })
        .collect()
}

impl ShaderInterfacePlan {
    /// 直接消费发行容器的stored reflection；SPIR-Q重反射仅用于独立核对。
    pub fn from_compiled_pipeline(
        compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
        samplers: &BTreeMap<String, u64>,
        runtime_device_scalar: u32,
    ) -> Result<Self> {
        use crate::shader_pipeline::{
            compiled::ShaderStage,
            reflection::{BindingArray, BindingKind},
        };
        let compute =
            compiled.stages.len() == 1 && compiled.stages[0].stage == ShaderStage::Compute;
        let raster = compiled.stages.len() == 2
            && compiled
                .stages
                .iter()
                .any(|stage| stage.stage == ShaderStage::Vertex)
            && compiled
                .stages
                .iter()
                .any(|stage| stage.stage == ShaderStage::Fragment);
        if !compute && !raster {
            return Err(invalid_shader());
        }
        for index in 0..compiled.stages.len() {
            stored_pipeline_entry(compiled, index)?;
        }
        let visibility = if compute {
            vk::ShaderStageFlags::COMPUTE
        } else {
            vk::ShaderStageFlags::ALL_GRAPHICS
        };
        let mut sets = Vec::new();
        for stored_set in compiled.reflection.effective_descriptor_sets() {
            let mut set = LayoutPlan {
                flags: 0,
                bindings: Vec::new(),
                binding_flags: Vec::new(),
            };
            for (index, binding) in stored_set {
                let tag = match binding.kind {
                    BindingKind::StorageBuffer => 0,
                    BindingKind::UniformBuffer => 1,
                    BindingKind::CombinedImageSampler => 2,
                    BindingKind::StorageImage => 3,
                    BindingKind::SampledImage => 4,
                    BindingKind::Sampler => 5,
                };
                let (count, runtime) = match binding.array {
                    BindingArray::Fixed(count) => (count, false),
                    BindingArray::Runtime => (0, true),
                };
                let sampler = if tag == 5 {
                    samplers
                        .get(&binding.name)
                        .copied()
                        .filter(|handle| *handle != 0)
                        .ok_or_else(|| {
                            crate::Error::invalid_usage("着色器资源、接口或设备不匹配")
                        })?
                } else {
                    0
                };
                let plan = descriptor_set_layout(
                    &[Binding {
                        tag,
                        name: binding.name.clone(),
                        count,
                        runtime,
                        sampler,
                    }],
                    u64::from(index),
                    visibility,
                    runtime_device_scalar,
                )
                .map_err(|_| crate::Error::invalid_usage("着色器资源、接口或设备不匹配"))?;
                set.flags |= plan.flags;
                set.bindings.extend(plan.bindings);
                set.binding_flags.extend(plan.binding_flags);
            }
            for (index, flags) in set.binding_flags.iter().enumerate() {
                if flags & vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw() != 0
                    && index + 1 != set.bindings.len()
                {
                    return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
                }
            }
            sets.push(set);
        }
        let size = compiled.reflection.push_constant_size;
        if !size.is_multiple_of(4) {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let push_constants = if size == 0 {
            Vec::new()
        } else {
            vec![vk::PushConstantRange {
                stage_flags: visibility,
                offset: 0,
                size,
            }]
        };
        Ok(Self {
            sets,
            push_constants,
        })
    }
}

fn serialized_stage_name(stage: crate::shader_pipeline::compiled::ShaderStage) -> &'static str {
    use crate::shader_pipeline::compiled::ShaderStage;
    match stage {
        ShaderStage::Compute => "GLCompute",
        ShaderStage::Vertex => "Vertex",
        ShaderStage::Fragment => "Fragment",
    }
}

/// 原存储反射直接生产资源名字/编号/数组和push范围；不重新解析公开库类型。
/// SPIR-V只用于入口名/执行模型的一致性检查，完整stage IO独立核对时才重新反射。
pub fn stored_pipeline_entry(
    compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
    index: usize,
) -> Result<ReflectedEntry> {
    use crate::shader_pipeline::{
        compiled::ShaderStage,
        reflection::{BindingArray, BindingKind},
    };
    let stage = compiled.stages.get(index).ok_or_else(invalid_shader)?;
    let expected = match stage.stage {
        ShaderStage::Compute => rspirv::spirv::ExecutionModel::GLCompute,
        ShaderStage::Vertex => rspirv::spirv::ExecutionModel::Vertex,
        ShaderStage::Fragment => rspirv::spirv::ExecutionModel::Fragment,
    };
    let module = rspirv::dr::load_words(&stage.spirv).map_err(|_| invalid_shader())?;
    let present = module.entry_points.iter().any(|point| matches!(
        (point.operands.first(), point.operands.get(2)),
        (Some(rspirv::dr::Operand::ExecutionModel(model)), Some(rspirv::dr::Operand::LiteralString(name)))
        if *model == expected && name == &stage.entry_point
    ));
    if !present {
        return Err(invalid_shader());
    }
    let mut descriptors = Vec::new();
    for (set, bindings) in compiled
        .reflection
        .effective_descriptor_sets()
        .into_iter()
        .enumerate()
    {
        for (binding, value) in bindings {
            let descriptor_type = match value.kind {
                BindingKind::StorageBuffer => "StorageBuffer(ReadWrite)",
                BindingKind::UniformBuffer => "UniformBuffer",
                BindingKind::CombinedImageSampler => "CombinedImageSampler",
                BindingKind::StorageImage => "StorageImage(ReadWrite)",
                BindingKind::SampledImage => "SampledImage",
                BindingKind::Sampler => "Sampler",
            };
            let count = match value.array {
                BindingArray::Fixed(count) => count,
                BindingArray::Runtime => 0,
            };
            let reflected_type = compiled
                .reflection
                .uniform_block
                .as_ref()
                .filter(|block| block.binding.set == set as u32 && block.binding.binding == binding)
                .map_or(
                    serde_json::Value::Null,
                    |block| serde_json::json!({"minimum_bytes":block.byte_size}),
                );
            descriptors.push(ReflectedDescriptor {
                set: set as u32,
                binding,
                name: value.name.clone(),
                count,
                descriptor_type: descriptor_type.into(),
                reflected_type,
            });
        }
    }
    let push_constants = if compiled.reflection.push_constant_size == 0 {
        Vec::new()
    } else {
        vec![ReflectedPushConstant {
            reflected_type: serde_json::json!({"minimum_bytes":compiled.reflection.push_constant_size}),
        }]
    };
    let inputs = if stage.stage == ShaderStage::Vertex {
        compiled
            .reflection
            .effective_vertex_inputs()
            .into_values()
            .map(|input| {
                let ty = input.ty.to_spirq().ok_or_else(invalid_shader)?;
                Ok(ReflectedLocation {
                    location: input.location,
                    component: 0,
                    reflected_type: reflected_type_json(&ty),
                })
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };
    Ok(ReflectedEntry {
        entry_point: stage.entry_point.clone(),
        stage: serialized_stage_name(stage.stage).into(),
        local_size: if stage.stage == ShaderStage::Compute {
            compiled.reflection.local_size
        } else {
            None
        },
        descriptors,
        push_constants,
        inputs,
        outputs: Vec::new(),
    })
}

impl ReflectedShader {
    /// 运行时完整容器入口：消费真实stored reflection，不构造空尾段替身。
    pub fn from_compiled_pipeline_stage(
        device: Arc<Device>,
        compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
        index: usize,
    ) -> Result<Self> {
        let entry = stored_pipeline_entry(compiled, index)?;
        let bytes = compiled.stages[index].spirv_bytes();
        Ok(Self {
            module: ShaderModule::from_spirv(device, &bytes)?,
            entry,
        })
    }
}

impl ShaderLibrary {
    /// 资产owner提供选择键；原存储字段保留至实际layout生产。
    pub fn register_compiled_compute(&mut self, key: String, bytes: &[u8]) -> Result<()> {
        use crate::shader_pipeline::compiled::{CompiledShaderPipeline, ShaderStage};
        let compiled = CompiledShaderPipeline::decode(bytes).map_err(|_| invalid_shader())?;
        if compiled.stages.len() != 1 || compiled.stages[0].stage != ShaderStage::Compute {
            return Err(invalid_shader());
        }
        let shader =
            ReflectedShader::from_compiled_pipeline_stage(self.cache.device.clone(), &compiled, 0)?;
        self.register(key.clone(), shader)?;
        self.compiled_computes.insert(key, Arc::new(compiled));
        Ok(())
    }
    /// 两个stage创建成功后一起注册；fragment失败不破坏旧资产/缓存。
    pub fn register_compiled_raster(
        &mut self,
        vertex_key: String,
        fragment_key: String,
        bytes: &[u8],
    ) -> Result<()> {
        use crate::shader_pipeline::compiled::{CompiledShaderPipeline, ShaderStage};
        let compiled = CompiledShaderPipeline::decode(bytes).map_err(|_| invalid_shader())?;
        if vertex_key == fragment_key || compiled.stages.len() != 2 {
            return Err(invalid_shader());
        }
        let vertex_index = compiled
            .stages
            .iter()
            .position(|stage| stage.stage == ShaderStage::Vertex)
            .ok_or_else(invalid_shader)?;
        let fragment_index = compiled
            .stages
            .iter()
            .position(|stage| stage.stage == ShaderStage::Fragment)
            .ok_or_else(invalid_shader)?;
        let vertex = ReflectedShader::from_compiled_pipeline_stage(
            self.cache.device.clone(),
            &compiled,
            vertex_index,
        )?;
        let fragment = ReflectedShader::from_compiled_pipeline_stage(
            self.cache.device.clone(),
            &compiled,
            fragment_index,
        )?;
        self.register(vertex_key.clone(), vertex)?;
        self.register(fragment_key.clone(), fragment)?;
        self.compiled_rasters
            .insert((vertex_key, fragment_key), Arc::new(compiled));
        Ok(())
    }
}

impl crate::resource::GpuUse for PipelineLayout {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        for set in &self.sets {
            set.visit_uses(visitor)?;
        }

        Ok(())
    }
}

impl crate::resource::GpuUse for VulkanPipeline {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        self.layout.visit_uses(visitor)?;

        Ok(())
    }
}

#[cfg(test)]
mod raster_opts_tests {
    use super::*;

    #[test]
    fn cache_slots_charge_actual_owners_and_join_full_borrowed_keys() {
        use std::cell::Cell;
        let budget = Arc::new(AllocatorStats::default());
        let table = Mutex::new(TempList::new(1, budget.clone()).unwrap());
        let first = slot_for(
            &table,
            |key: &Vec<u8>| key.as_slice() == b"first",
            || budgeted_key_vec(&budget, b"first"),
            1,
            &budget,
        )
        .unwrap();
        let charged = budget.cpu_memory_usage().total();
        let invoked = Cell::new(false);
        budget.set_total_budget(charged).unwrap();
        let same = slot_for(
            &table,
            |key| key.as_slice() == b"first",
            || {
                invoked.set(true);
                budgeted_key_vec(&budget, b"first")
            },
            1,
            &budget,
        )
        .unwrap();
        assert!(Arc::ptr_eq(&same, &first));
        assert!(
            !invoked.get(),
            "cache hit never allocates or builds an owned key"
        );
        assert_eq!(budget.cpu_memory_usage().total(), charged);
        assert!(matches!(
            slot_for(
                &table,
                |_| false,
                || {
                    invoked.set(true);
                    budgeted_key_vec(&budget, b"other")
                },
                1,
                &budget
            ),
            Err(crate::Error::Exhausted(_))
        ));
        assert!(
            !invoked.get(),
            "an in-flight compiler slot is never evicted"
        );
        drop(same);
        table.lock().clear();
        drop(table);
        assert!(
            budget.cpu_memory_usage().total() > 0,
            "external slot Arc retains its metadata credit"
        );
        drop(first);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn cache_key_admission_failure_preserves_the_old_slot_and_credit() {
        let budget = Arc::new(AllocatorStats::default());
        let table = Mutex::new(TempList::new(1, budget.clone()).unwrap());
        let first = slot_for(
            &table,
            |key: &Vec<u8>| key.as_slice() == b"first",
            || budgeted_key_vec(&budget, b"first"),
            1,
            &budget,
        )
        .unwrap();
        drop(first);
        let charged = budget.cpu_memory_usage().total();
        budget.set_total_budget(charged).unwrap();
        assert!(matches!(
            slot_for(
                &table,
                |_| false,
                || budgeted_key_vec(&budget, b"second"),
                1,
                &budget
            ),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(budget.cpu_memory_usage().total(), charged);
        assert_eq!(table.lock().as_slice()[0].key, b"first");
        budget.set_total_budget(1 << 20).unwrap();
        let next = slot_for(
            &table,
            |_| false,
            || budgeted_key_vec(&budget, b"second"),
            1,
            &budget,
        )
        .unwrap();
        assert_eq!(table.lock().as_slice()[0].key, b"second");
        drop((next, table));
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn pipeline_array_accounting_includes_both_format_vectors_and_rejects_overflow() {
        let bytes = pipeline_array_bytes(2, 2, 4, 3).unwrap();
        assert_eq!(
            bytes,
            (2 * std::mem::size_of::<Arc<ShaderModule>>()
                + 2 * std::mem::size_of::<usize>()
                + 7 * std::mem::size_of::<vk::Format>()) as u64
        );
        assert!(matches!(
            pipeline_array_bytes(usize::MAX, 0, 0, 0),
            Err(crate::Error::OutOfHostMemory)
        ));
    }

    #[test]
    fn invalid_raster_options_return_usage_errors_without_changing_encoded_state() {
        assert_eq!(
            RasterPipelineOpts(3).topology().unwrap_err().kind(),
            crate::ErrorKind::InvalidUsage
        );
        assert_eq!(
            RasterPipelineOpts(3 << 4)
                .depth_compare_op()
                .unwrap_err()
                .kind(),
            crate::ErrorKind::InvalidUsage
        );
        let mut options = RasterPipelineOpts(0xa5a5_5a5a);
        let original = options;
        for error in [
            options
                .set_topology(vk::PrimitiveTopology::PATCH_LIST)
                .unwrap_err(),
            options
                .set_depth_compare_op(vk::CompareOp::ALWAYS)
                .unwrap_err(),
            options
                .set_polygon_mode(vk::PolygonMode::POINT)
                .unwrap_err(),
        ] {
            assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
        }
        assert_eq!(options, original);
    }

    #[test]
    fn malformed_shader_and_reflection_use_public_error_categories() {
        assert_eq!(
            reflect_spirv_entries(&[]).unwrap_err().kind(),
            crate::ErrorKind::InvalidUsage
        );
        let reflection = crate::shader_pipeline::reflection::ShaderReflection {
            descriptor_sets: vec![],
            uniform_block: None,
            binding_names: vec![],
            binding_name_hashes: vec![],
            vertex_inputs: vec![],
            local_size: None,
            push_constant_size: 0,
        };
        let error = validate_compute_reflection(&reflection).unwrap_err();
        assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
        assert!(matches!(
            std::error::Error::source(&error)
                .and_then(|source| source.downcast_ref::<InterfaceError>()),
            Some(InterfaceError::MissingGroupSize)
        ));
    }

    #[test]
    fn dynamic_formats_validate_aspects_limits_and_multiview() {
        let color = DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM);
        color.validate(1).unwrap();
        assert!(color.validate(0).is_err());
        let mut value = color.clone();
        value.colors[0] = vk::Format::D32_SFLOAT;
        assert!(value.validate(1).is_err());
        value = color.clone();
        value.depth = vk::Format::R8G8B8A8_UNORM;
        assert!(value.validate(1).is_err());
        value.depth = vk::Format::D24_UNORM_S8_UINT;
        value.stencil = vk::Format::D24_UNORM_S8_UINT;
        value.validate(1).unwrap();
        value.stencil = vk::Format::S8_UINT;
        assert!(value.validate(1).is_err());
        value = color;
        value.view_mask = 1;
        assert_eq!(
            value.validate(1).unwrap_err().kind(),
            crate::ErrorKind::Unsupported
        );
    }
    #[test]
    fn dynamic_cache_signature_includes_actual_target_and_all_formats() {
        let key = |value: Option<&DynamicRenderingFormats>| {
            let mut key = Vec::new();
            append_dynamic_rendering_key(&mut key, value);
            key
        };
        let color = DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM);
        assert_ne!(key(None), key(Some(&color)));
        for changed in 0..5 {
            let mut value = color.clone();
            match changed {
                0 => value.colors[0] = vk::Format::B8G8R8A8_UNORM,
                1 => value.colors.push(vk::Format::R8G8B8A8_UNORM),
                2 => value.depth = vk::Format::D32_SFLOAT,
                3 => value.stencil = vk::Format::S8_UINT,
                _ => value.view_mask = 1,
            }
            assert_ne!(key(Some(&color)), key(Some(&value)));
        }
    }

    #[test]
    fn raster_cache_key_length_matches_the_actual_full_key_and_borrowed_lookup() {
        let device = match crate::raw::init::create_headless_device() {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    std::env::var("RHAPSODY_REQUIRE_VULKAN").as_deref() != Ok("1"),
                    "strict raster key fixture requires Vulkan: {error}"
                );
                eprintln!("SKIP Vulkan raster key fixture: {error}");
                return;
            }
        };
        // Keys only read these owned identities; no null shader/layout is ever
        // passed to native pipeline creation by this metadata regression.
        let module = Arc::new(ShaderModule {
            device: device.clone(),
            handle: vk::ShaderModule::null(),
            interfaces: Arc::from([]),
            _memory: reserve_shared_cpu_metadata::<ShaderModule>(&device.allocator_stats, 16)
                .unwrap(),
        });
        let layout = Arc::new(PipelineLayout {
            last_use: crate::resource::SyncCell::for_device_budgeted(
                device.generation(),
                &device.allocator_stats,
            )
            .unwrap(),
            device: device.clone(),
            handle: vk::PipelineLayout::null(),
            sets: Vec::new(),
            push_constants: Vec::new(),
            bindless_abi: None,
            _memory: reserve_shared_cpu_metadata::<PipelineLayout>(&device.allocator_stats, 0)
                .unwrap(),
        });
        let mut desc = RasterPipelineDesc::new(module, layout);
        for index in 0..5 {
            desc.entry = CString::new(format!("vertex_{index}")).unwrap();
            desc.fragment_entry = Some(CString::new(format!("fragment_entry_{index}")).unwrap());
            desc.dynamic_rendering = (index != 0).then(|| DynamicRenderingFormats {
                colors: vec![vk::Format::R8G8B8A8_UNORM; index],
                depth: vk::Format::UNDEFINED,
                stencil: vk::Format::UNDEFINED,
                view_mask: 0,
            });
            desc.vertex_bindings = vec![vk::VertexInputBindingDescription::default(); index];
            desc.vertex_attributes =
                vec![vk::VertexInputAttributeDescription::default(); index + 1];
            desc.blend_attachments = vec![vk::PipelineColorBlendAttachmentState::default(); index];
            let key = raster_pipeline_key(&desc);
            assert_eq!(key.len(), raster_pipeline_key_len(&desc));
            assert!(raster_pipeline_key_matches(&desc, &key));
            desc.depth_write = !desc.depth_write;
            assert!(!raster_pipeline_key_matches(&desc, &key));
        }
    }

    #[test]
    fn bit_positions_match_the_exe_decode() {
        let mut o = RasterPipelineOpts::new();
        o.set_topology(vk::PrimitiveTopology::LINE_STRIP).unwrap();
        assert_eq!(o.0, 0b10);
        let mut o = RasterPipelineOpts::new();
        o.set_cull_mode(vk::CullModeFlags::BACK);
        assert_eq!(o.0, 0b10 << 2);
        let mut o = RasterPipelineOpts::new();
        o.set_depth_compare_op(vk::CompareOp::EQUAL).unwrap();
        o.set_depth_write(true);
        o.set_depth_test_enable(true);
        assert_eq!(o.0, (2 << 4) | (1 << 6) | (1 << 7));
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_test_enable(true);
        o.set_stencil_fail_op(vk::StencilOp::INCREMENT_AND_CLAMP);
        o.set_stencil_pass_op(vk::StencilOp::REPLACE);
        assert_eq!(o.0, (1 << 8) | (3 << 9) | (2 << 12));
        // depth-fail op 3 位：b1 bit7 为低位，b2 bit0–1 为高两位。
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_depth_fail_op(vk::StencilOp::INVERT); // 5 = 0b101
        assert_eq!(o.0, (1 << 15) | (2 << 16));
        assert_eq!(o.stencil_depth_fail_op(), vk::StencilOp::INVERT);
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_compare_op(vk::CompareOp::ALWAYS); // 7
        assert_eq!(o.0, 7 << 18);
        // compare mask 8 位：b2 bit5–7 低 3 位，b3 bit0–4 高 5 位。
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_compare_mask(0xff);
        assert_eq!(o.0, (7 << 21) | (0x1f << 24));
        assert_eq!(o.stencil_compare_mask(), 0xff);
        let mut o = RasterPipelineOpts::new();
        o.set_blend_mode(3);
        o.set_polygon_mode(vk::PolygonMode::LINE).unwrap();
        assert_eq!(o.0, (3 << 29) | (1 << 31));
    }

    #[test]
    fn split_fields_round_trip() {
        let mut o = RasterPipelineOpts::new();
        for (op, mask) in [(0u32, 0u32), (7, 0xa5), (3, 0x5a), (5, 0xff)] {
            o.set_stencil_depth_fail_op(vk::StencilOp::from_raw(op as i32));
            o.set_stencil_compare_mask(mask);
            o.set_stencil_compare_op(vk::CompareOp::LESS_OR_EQUAL);
            o.set_blend_mode(2);
            assert_eq!(o.stencil_depth_fail_op().as_raw(), op as i32);
            assert_eq!(o.stencil_compare_mask(), mask);
            assert_eq!(o.stencil_compare_op(), vk::CompareOp::LESS_OR_EQUAL);
            assert_eq!(o.blend_mode(), 2);
        }
    }

    #[test]
    fn jump_table_enums_use_the_exe_ordering() {
        for (code, topology) in [
            (0, vk::PrimitiveTopology::TRIANGLE_LIST),
            (1, vk::PrimitiveTopology::LINE_LIST),
            (2, vk::PrimitiveTopology::LINE_STRIP),
        ] {
            assert_eq!(RasterPipelineOpts(code).topology().unwrap(), topology);
        }
        for (code, op) in [
            (0, vk::CompareOp::LESS),
            (1, vk::CompareOp::GREATER),
            (2, vk::CompareOp::EQUAL),
        ] {
            assert_eq!(
                RasterPipelineOpts(code << 4).depth_compare_op().unwrap(),
                op
            );
        }
    }

    #[test]
    fn apply_to_replicates_one_blend_state_per_color_attachment() {
        let device = match crate::raw::init::create_headless_device() {
            Ok(device) => device,
            Err(_) => return,
        };
        let module = Arc::new(ShaderModule {
            device: device.clone(),
            handle: vk::ShaderModule::null(),
            interfaces: Arc::from([]),
            _memory: reserve_shared_cpu_metadata::<ShaderModule>(&device.allocator_stats, 16)
                .unwrap(),
        });
        let layout = Arc::new(PipelineLayout {
            last_use: crate::resource::SyncCell::for_device_budgeted(
                device.generation(),
                &device.allocator_stats,
            )
            .unwrap(),
            device: device.clone(),
            handle: vk::PipelineLayout::null(),
            sets: Vec::new(),
            push_constants: Vec::new(),
            bindless_abi: None,
            _memory: reserve_shared_cpu_metadata::<PipelineLayout>(&device.allocator_stats, 0)
                .unwrap(),
        });
        let mut desc = RasterPipelineDesc::new(module, layout);
        let mut o = RasterPipelineOpts::new();
        o.set_blend_mode(2);
        o.set_stencil_test_enable(true);
        o.set_stencil_compare_mask(0x81);
        o.apply_to(&mut desc, 3).unwrap();
        assert_eq!(desc.blend_attachments.len(), 3);
        assert_eq!(desc.blend_attachments[0].dst_color_blend_factor.as_raw(), 7);
        assert!(desc.stencil_test);
        assert_eq!(desc.stencil.compare_mask, 0x81);
        assert_eq!(desc.stencil.write_mask, 0xffff_ffff);
        assert_eq!(desc.front_face, vk::FrontFace::COUNTER_CLOCKWISE);
    }

    #[test]
    fn map_vertex_attribute_format_follows_the_exe_table() {
        assert_eq!(
            map_vertex_attribute_format(4, 4).unwrap(),
            vk::Format::R32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(4, 8).unwrap(),
            vk::Format::R32G32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(4, 12).unwrap(),
            vk::Format::R32G32B32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(4, 16).unwrap(),
            vk::Format::R32G32B32A32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(3, 4).unwrap(),
            vk::Format::R32_UINT
        );
        assert_eq!(
            map_vertex_attribute_format(2, 16).unwrap(),
            vk::Format::R32G32B32A32_SINT
        );
        assert!(map_vertex_attribute_format(4, 20).is_err());
        assert!(map_vertex_attribute_format(5, 4).is_err());
    }
}
