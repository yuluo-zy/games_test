//! Shader modules and pipeline layouts.
use ash::vk;
use crate::Result;
use crate::accounting::CpuMemoryReservation;
use crate::accounting::cpu_vec_bytes;
use crate::accounting::reserve_shared_cpu_metadata;
use crate::collections::temp_list::TempList;
use crate::command::descriptor_set::VulkanDescriptorSetLayout;
use crate::device::Device;
use crate::device::observe_vulkan;
use std::io::Cursor;
use std::sync::Arc;

pub(super) fn invalid_shader() -> crate::Error {
    crate::Error::invalid_usage("着色器数据、SPIR-V 入口或反射无效")
}

/// Releases owned CPU buffers before returning their reservation on any
/// pre-native failure as well as when the final published object is dropped.
pub(super) struct BudgetedMetadata<T> {
    pub(super) data: T,
    pub(super) memory: CpuMemoryReservation,
}

pub struct ShaderModule {
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::ShaderModule,
    pub(super) interfaces: Arc<[crate::pipeline::shader_interface::EntryContract]>,
    pub(super) _memory: CpuMemoryReservation,
}

impl ShaderModule {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::ShaderModule {
        self.handle
    }
}

impl ShaderModule {
    pub fn from_spirv(device: Arc<Device>, bytes: &[u8]) -> Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let mut words_memory = device.memory.stats.reserve_cpu_bytes(
            cpu_vec_bytes::<u32>(bytes.len() / 4)?,
            crate::accounting::BudgetCategory::Live,
        )?;
        let words = ash::util::read_spv(&mut Cursor::new(bytes)).map_err(|_| invalid_shader())?;
        words_memory.resize(cpu_vec_bytes::<u32>(words.capacity())?)?;
        if words.len() < 5 || words.first().copied() != Some(0x07230203) {
            return Err(invalid_shader());
        }
        let interfaces = crate::pipeline::shader_interface::contracts(&words)?;
        let (interface_layout, _) = std::alloc::Layout::new::<[usize; 2]>()
            .extend(
                std::alloc::Layout::array::<crate::pipeline::shader_interface::EntryContract>(
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
        let memory = reserve_shared_cpu_metadata::<Self>(&device.memory.stats, interface_bytes)?;
        let interfaces = BudgetedMetadata {
            data: interfaces,
            memory,
        };
        let info = vk::ShaderModuleCreateInfo::default().code(&words);
        let handle =
            unsafe { device.core.device.create_shader_module(&info, None) }.map_err(|error| {
                if error == vk::Result::ERROR_DEVICE_LOST {
                    device.queues.timelines.mark_device_lost();
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
            self.device
                .core
                .device
                .destroy_shader_module(self.handle, None);
        }
    }
}

pub struct PipelineLayout {
    pub(super) retirement: Option<crate::device::RetireTicket>,
    pub(crate) last_use: Arc<crate::tracking::SyncCell>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::PipelineLayout,
    pub(crate) sets: Vec<Arc<VulkanDescriptorSetLayout>>,
    pub(crate) push_constants: Vec<vk::PushConstantRange>,
    pub(super) bindless_abi: Option<Arc<crate::pipeline::shader::BindlessShaderAbi>>,
    pub(super) _memory: CpuMemoryReservation,
}

impl PipelineLayout {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::PipelineLayout {
        self.handle
    }
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
        abi: Arc<crate::pipeline::shader::BindlessShaderAbi>,
    ) -> Result<Arc<Self>> {
        Self::create(device, sets, push_constants, Some(abi))
    }
    pub fn bindless_abi(&self) -> Option<&Arc<crate::pipeline::shader::BindlessShaderAbi>> {
        self.bindless_abi.as_ref()
    }
    fn create(
        device: Arc<Device>,
        sets: Vec<Arc<VulkanDescriptorSetLayout>>,
        push_constants: Vec<vk::PushConstantRange>,
        bindless_abi: Option<Arc<crate::pipeline::shader::BindlessShaderAbi>>,
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
        let memory = reserve_shared_cpu_metadata::<Self>(&device.memory.stats, bytes)?;
        let arrays = BudgetedMetadata {
            data: (sets, push_constants),
            memory,
        };
        let mut raw = TempList::new(arrays.data.0.len().max(1), device.memory.stats.clone())?;
        raw.reserve(arrays.data.0.len())?;
        for set in &arrays.data.0 {
            raw.add(set.handle)?;
        }
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(raw.as_slice())
            .push_constant_ranges(&arrays.data.1);
        let sync_storage = crate::tracking::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let retirement = device.reserve_retirement()?;
        let handle = observe_vulkan(&device, unsafe {
            device.core.device.create_pipeline_layout(&info, None)
        })?;
        Ok(Arc::new(Self {
            last_use: crate::tracking::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            handle,
            sets: arrays.data.0,
            retirement: Some(retirement),
            push_constants: arrays.data.1,
            bindless_abi,
            _memory: arrays.memory,
        }))
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        self.device.retire_reserved(
            self.last_use.snapshot(),
            crate::device::graveyard::Doomed::PipelineLayout(self.handle),
            self.retirement
                .take()
                .expect("pipeline layout retirement node"),
        );
    }
}

impl crate::tracking::GpuUse for PipelineLayout {
    fn visit_uses(&self, visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        for set in &self.sets {
            set.visit_uses(visitor)?;
        }

        Ok(())
    }
}
