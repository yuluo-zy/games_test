//! Logical Vulkan device, queues, allocation and retirement.
use ash::vk;
use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

/// 记录实际启用的功能，不能以驱动支持代替设备已启用。
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceCapabilities {
    pub buffer_device_address: bool,
    pub wide_lines: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    pub runtime_descriptor_array: bool,
    pub descriptor_binding_partially_bound: bool,
    pub descriptor_binding_variable_descriptor_count: bool,
    pub sampled_image_update_after_bind: bool,
    pub storage_buffer_update_after_bind: bool,
    /// 实际启用的采样图片数组非一致索引，不能由 runtime array 推断。
    pub shader_sampled_image_array_non_uniform_indexing: bool,
    /// runtime sampled-image 布局 flags15 所需的实际启用特性。
    pub descriptor_binding_update_unused_while_pending: bool,
}

impl DeviceCapabilities {
    fn record_enabled_core(&mut self, enabled: &vk::PhysicalDeviceFeatures) {
        self.wide_lines = enabled.wide_lines != 0;
        self.geometry_shader = enabled.geometry_shader != 0;
        self.tessellation_shader = enabled.tessellation_shader != 0;
    }
}

/// Logical Vulkan device.
///
/// Teardown order is fixed by `Drop`: queued retirements are drained, cached
/// objects and the suballocator release their memory, then the `VkDevice` is
/// destroyed; `instance` is the last field so the `VkInstance` outlives it.
pub struct Device {
    generation: crate::device::DeviceGeneration,
    caps: crate::device::DeviceCaps,
    policy: crate::device::ExecutionPolicy,
    frames_in_flight: u32,
    pub(crate) reclamation: crate::device::ReclamationConfig,
    self_ref: std::sync::Weak<Self>,
    properties: vk::PhysicalDeviceProperties,
    physical_info: std::sync::OnceLock<Arc<super::physical_device::PhysicalDevice>>,
    pub resources: super::managed::resources::ManagedResources,
    builtin_samplers:
        Mutex<[Option<BuiltinSamplerEntry>; super::sampler::BUILTIN_SAMPLER_SPECS.len()]>,
    pub device: ash::Device,
    pub physical: vk::PhysicalDevice,
    pub memory: vk::PhysicalDeviceMemoryProperties,
    pub queue: vk::Queue,
    pub queue_family: u32,
    pub timelines: crate::sync::timeline::Timelines<ash::Device>,
    pub capabilities: DeviceCapabilities,
    /// 创建时实际启用的设备扩展名（`vkCreateDevice` 的 `ppEnabledExtensionNames`）。
    pub enabled_extensions: Vec<String>,
    pub additional_queues: Vec<DeviceQueue>,
    /// 配置入口创建时保留的类别映射；直接提供 VkDeviceCreateInfo 的底层入口无此映射。
    pub queue_plan: crate::device::QueuePlan,
    pub allocator_stats: Arc<super::allocator::AllocatorStats>,
    pub(crate) retirement: crate::sync::retire::Retirement<super::graveyard::Doomed>,
    pub(crate) idle_waits: AtomicU64,
    /// Number of `vkUpdateDescriptorSets` calls (statistics only).
    pub descriptor_update_count: AtomicU64,
    /// gpu-allocator suballocation; `None` means one dedicated allocation per resource.
    #[cfg(feature = "suballocation")]
    pub suballocator: Option<Arc<super::suballoc::SubAllocator>>,
    /// Loaded once at creation when `VK_KHR_synchronization2` is enabled.
    pub(crate) synchronization2: Option<ash::khr::synchronization2::Device>,
    memory_budget_extension: bool,
    /// Must stay the last field: the instance outlives the logical device.
    pub instance: Arc<super::instance::VulkanInstance>,
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceQueue {
    pub handle: vk::Queue,
    pub family: u32,
    pub index: u32,
}

struct BuiltinSamplerEntry {
    sampler: std::sync::Weak<super::sampler::Sampler>,
    // Share the ticket with the payload: a concurrent last-owner Drop may still
    // be running after this entry's Weak can no longer be upgraded.
    _storage: Arc<super::sampler::SamplerStorage>,
}

impl Device {
    /// Every public device is constructed in an Arc. Internal caches hold only weak
    /// references; this upgrade is used while an external caller already owns it.
    pub(crate) fn shared(&self) -> Arc<Self> {
        // Every Device lives in an Arc and `&self` proves an owner exists.
        self.self_ref
            .upgrade()
            .expect("device method called with a live owner")
    }
    /// 此逻辑设备的生命周期标识；同一物理 GPU 上创建的设备也拥有不同代际。
    pub fn generation(&self) -> crate::device::DeviceGeneration {
        self.generation
    }
    pub fn caps(&self) -> &crate::device::DeviceCaps {
        &self.caps
    }
    pub fn policy(&self) -> &crate::device::ExecutionPolicy {
        &self.policy
    }
    pub fn frames_in_flight(&self) -> u32 {
        self.frames_in_flight
    }
    /// Physical command limit; callers must also check the actually enabled
    /// MultiDrawIndirect / DrawIndirectCount features in this device's caps.
    pub fn max_draw_indirect_count(&self) -> u32 {
        self.properties.limits.max_draw_indirect_count
    }
    pub fn reclamation_config(&self) -> crate::device::ReclamationConfig {
        self.reclamation
    }

    #[cfg(not(feature = "suballocation"))]
    pub fn suballocation_enabled(&self) -> bool {
        false
    }
    /// `maxDescriptorSetUpdateAfterBindSampledImages` of the physical device.
    pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
        let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
        unsafe {
            self.instance
                .instance
                .get_physical_device_properties2(self.physical, &mut properties);
        }
        indexing.max_descriptor_set_update_after_bind_sampled_images
    }
    pub(crate) fn create_with_runtime(
        instance: Arc<super::instance::VulkanInstance>,
        physical: vk::PhysicalDevice,
        info: &vk::DeviceCreateInfo<'_>,
        plan: crate::device::QueuePlan,
        runtime: crate::device::init::RuntimeConfig,
    ) -> crate::Result<Arc<Self>> {
        let queue_family = plan.queues[0].family;
        let allocator_stats = Arc::new(super::allocator::AllocatorStats::default());
        allocator_stats.set_device_budget(runtime.memory_budget_bytes)?;
        allocator_stats.set_host_budget(runtime.host_memory_budget_bytes)?;
        // Prepare fallible host metadata before creating a native VkDevice.
        let resources = super::managed::resources::ManagedResources::new(allocator_stats.clone())?;
        unsafe {
            let mut capabilities = DeviceCapabilities::default();
            if !info.p_enabled_features.is_null() {
                capabilities.record_enabled_core(&*info.p_enabled_features);
            }
            let mut feature = info.p_next.cast::<vk::BaseInStructure<'_>>();
            while !feature.is_null() {
                match (*feature).s_type {
                    vk::StructureType::PHYSICAL_DEVICE_FEATURES_2 => {
                        let enabled_core =
                            (*feature.cast::<vk::PhysicalDeviceFeatures2<'_>>()).features;
                        capabilities.record_enabled_core(&enabled_core);
                    }
                    vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => {
                        let enabled = &*feature.cast::<vk::PhysicalDeviceVulkan12Features<'_>>();
                        capabilities.buffer_device_address = enabled.buffer_device_address != 0;
                        capabilities.runtime_descriptor_array =
                            enabled.runtime_descriptor_array != 0;
                        capabilities.descriptor_binding_partially_bound =
                            enabled.descriptor_binding_partially_bound != 0;
                        capabilities.descriptor_binding_variable_descriptor_count =
                            enabled.descriptor_binding_variable_descriptor_count != 0;
                        capabilities.sampled_image_update_after_bind =
                            enabled.descriptor_binding_sampled_image_update_after_bind != 0;
                        capabilities.storage_buffer_update_after_bind =
                            enabled.descriptor_binding_storage_buffer_update_after_bind != 0;
                        capabilities.shader_sampled_image_array_non_uniform_indexing =
                            enabled.shader_sampled_image_array_non_uniform_indexing != 0;
                        capabilities.descriptor_binding_update_unused_while_pending =
                            enabled.descriptor_binding_update_unused_while_pending != 0;
                    }
                    vk::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => {
                        let enabled =
                            &*feature.cast::<vk::PhysicalDeviceDescriptorIndexingFeatures<'_>>();
                        capabilities.runtime_descriptor_array =
                            enabled.runtime_descriptor_array != 0;
                        capabilities.descriptor_binding_partially_bound =
                            enabled.descriptor_binding_partially_bound != 0;
                        capabilities.descriptor_binding_variable_descriptor_count =
                            enabled.descriptor_binding_variable_descriptor_count != 0;
                        capabilities.sampled_image_update_after_bind =
                            enabled.descriptor_binding_sampled_image_update_after_bind != 0;
                        capabilities.storage_buffer_update_after_bind =
                            enabled.descriptor_binding_storage_buffer_update_after_bind != 0;
                        capabilities.shader_sampled_image_array_non_uniform_indexing =
                            enabled.shader_sampled_image_array_non_uniform_indexing != 0;
                        capabilities.descriptor_binding_update_unused_while_pending =
                            enabled.descriptor_binding_update_unused_while_pending != 0;
                    }
                    vk::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => {
                        capabilities.buffer_device_address =
                            (*feature.cast::<vk::PhysicalDeviceBufferDeviceAddressFeatures<'_>>())
                                .buffer_device_address
                                != 0;
                    }
                    _ => {}
                }
                feature = (*feature).p_next;
            }
            let enabled_extensions = if info.pp_enabled_extension_names.is_null() {
                Vec::new()
            } else {
                std::slice::from_raw_parts(
                    info.pp_enabled_extension_names,
                    info.enabled_extension_count as usize,
                )
                .iter()
                .map(|name| {
                    std::ffi::CStr::from_ptr(*name)
                        .to_string_lossy()
                        .into_owned()
                })
                .collect()
            };
            let device = instance.instance.create_device(physical, info, None)?;
            let synchronization2 = enabled_extensions
                .iter()
                .any(|name| name == "VK_KHR_synchronization2")
                .then(|| ash::khr::synchronization2::Device::new(&instance.instance, &device));
            let memory_budget_extension = enabled_extensions
                .iter()
                .any(|name| name == "VK_EXT_memory_budget");
            let queue = device.get_device_queue(queue_family, 0);
            let memory = instance
                .instance
                .get_physical_device_memory_properties(physical);
            let requested = std::slice::from_raw_parts(
                info.p_queue_create_infos,
                info.queue_create_info_count as usize,
            );
            let mut additional_queues = Vec::new();
            for request in requested {
                for index in 0..request.queue_count {
                    if request.queue_family_index == queue_family && index == 0 {
                        continue;
                    }
                    additional_queues.push(DeviceQueue {
                        handle: device.get_device_queue(request.queue_family_index, index),
                        family: request.queue_family_index,
                        index,
                    });
                }
            }
            {
                // VkDeviceQueueCreateInfo 按族排序，逻辑队列按 Graphics/Compute/Transfer 分配。
                // 统一顺序，确保 QueueId 与设备时间线、additional_queues 一一对应。
                additional_queues.sort_by_key(|queue| {
                    plan.queues
                        .iter()
                        .position(|physical| {
                            physical.family == queue.family && physical.index == queue.index
                        })
                        .unwrap_or(usize::MAX)
                });
            }
            #[cfg(feature = "suballocation")]
            let suballocator = match runtime.policy.allocation {
                crate::device::AllocationStrategy::Dedicated => None,
                crate::device::AllocationStrategy::Suballocated => Some(Arc::new(
                    super::suballoc::SubAllocator::new(
                        &instance.instance,
                        &device,
                        physical,
                        capabilities.buffer_device_address,
                        allocator_stats.clone(),
                    )
                    .inspect_err(|_| {
                        device.destroy_device(None);
                    })?,
                )),
            };
            if additional_queues.len() + 1 > crate::sync::MAX_QUEUES {
                #[cfg(feature = "suballocation")]
                if let Some(sub) = &suballocator {
                    sub.shutdown();
                }
                device.destroy_device(None);
                return Err(crate::Error::invalid_usage("队列配置超过设备时间线容量"));
            }
            let mut semaphores = Vec::new();
            for _ in 0..=additional_queues.len() {
                let mut ty = vk::SemaphoreTypeCreateInfo::default()
                    .semaphore_type(vk::SemaphoreType::TIMELINE);
                match device
                    .create_semaphore(&vk::SemaphoreCreateInfo::default().push_next(&mut ty), None)
                {
                    Ok(semaphore) => semaphores.push(semaphore),
                    Err(error) => {
                        for semaphore in semaphores {
                            device.destroy_semaphore(semaphore, None);
                        }
                        #[cfg(feature = "suballocation")]
                        if let Some(sub) = &suballocator {
                            sub.shutdown();
                        }
                        device.destroy_device(None);
                        return Err(error.into());
                    }
                }
            }
            let timelines = match crate::sync::timeline::Timelines::new(device.clone(), &semaphores)
            {
                Ok(timelines) => timelines,
                Err(error) => {
                    for semaphore in semaphores {
                        device.destroy_semaphore(semaphore, None);
                    }
                    #[cfg(feature = "suballocation")]
                    if let Some(sub) = &suballocator {
                        sub.shutdown();
                    }
                    device.destroy_device(None);
                    return Err(error);
                }
            };
            let properties = instance.instance.get_physical_device_properties(physical);
            allocator_stats
                .set_max_native_objects(u64::from(properties.limits.max_memory_allocation_count));
            Ok(Arc::new_cyclic(|weak| Self {
                self_ref: weak.clone(),
                properties,
                physical_info: Default::default(),
                resources,
                builtin_samplers: Mutex::new(std::array::from_fn(|_| None)),
                generation: crate::device::DeviceGeneration::next(),
                caps: runtime.caps,
                policy: runtime.policy,
                frames_in_flight: runtime.frames_in_flight,
                reclamation: runtime.reclamation,
                timelines,
                descriptor_update_count: AtomicU64::new(0),
                device,
                physical,
                memory,
                queue,
                queue_family,
                instance,

                capabilities,
                enabled_extensions,
                additional_queues,
                queue_plan: plan,
                allocator_stats,
                retirement: Default::default(),
                idle_waits: AtomicU64::new(0),
                #[cfg(feature = "suballocation")]
                suballocator,
                synchronization2,
                memory_budget_extension,
            }))
            .inspect(|device| {
                if let Err(error) = device.refresh_memory_budget() {
                    log::warn!("initial memory budget query failed: {error}");
                }
            })
        }
    }

    pub fn wait_idle(&self) -> crate::Result<()> {
        self.wait_idle_and_collect().map(|_| ())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // Owned Vulkan children hold a device Arc and are already gone; only
        // queued retirements and device-owned caches remain.
        self.drain_retirement_for_drop();
        // Cached entries only hold weak handles and host metadata tickets.
        for entry in self.builtin_samplers.get_mut().iter_mut() {
            entry.take();
        }
        // Native memory blocks must be freed before the VkDevice.
        #[cfg(feature = "suballocation")]
        if let Some(sub) = self.suballocator.take() {
            sub.shutdown();
        }
        unsafe {
            for index in 0..self.timelines.queue_count() {
                if let Some(semaphore) = crate::sync::QueueId::new(index)
                    .and_then(|queue| self.timelines.semaphore(queue).ok())
                {
                    self.device.destroy_semaphore(semaphore, None);
                }
            }
            self.device.destroy_device(None);
        }
        // `instance` is dropped after this body, after the VkDevice.
    }
}

impl Device {
    pub fn min_subgroup_size_option(&self) -> Option<u32> {
        if !self
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_subgroup_size_control")
        {
            return None;
        }
        let mut control = vk::PhysicalDeviceSubgroupSizeControlProperties::default();
        let mut props = vk::PhysicalDeviceProperties2::default().push_next(&mut control);
        unsafe {
            self.instance
                .instance
                .get_physical_device_properties2(self.physical, &mut props);
        }
        Some(control.min_subgroup_size)
    }
    pub fn device_uuid(&self) -> [u8; 16] {
        let mut id = vk::PhysicalDeviceIDProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
        unsafe {
            self.instance
                .instance
                .get_physical_device_properties2(self.physical, &mut properties);
        }
        id.device_uuid
    }
    pub fn pipeline_cache_uuid(&self) -> [u8; 16] {
        self.properties.pipeline_cache_uuid
    }
    pub fn limits(&self) -> &vk::PhysicalDeviceLimits {
        &self.properties.limits
    }

    /// Minimum subgroup size, or 1 when `VK_EXT_subgroup_size_control` is absent.
    pub fn min_subgroup_size(&self) -> u32 {
        self.min_subgroup_size_option().unwrap_or(1)
    }
    /// Raw physical device handle; see [`Self::physical_device_info`].
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical
    }
    /// Physical device wrapper attached by [`Self::attach_physical_device_info`].
    pub fn physical_device_info(&self) -> Option<&Arc<super::physical_device::PhysicalDevice>> {
        self.physical_info.get()
    }
    /// 挂接物理设备包装（只能设置一次）。
    pub fn attach_physical_device_info(
        &self,
        info: Arc<super::physical_device::PhysicalDevice>,
    ) -> Result<(), Arc<super::physical_device::PhysicalDevice>> {
        self.physical_info.set(info)
    }
    /// Query `VK_EXT_memory_budget` and apply the per-heap budgets to the
    /// allocator (driver budget minus memory this device did not allocate).
    /// Call once per frame or maintenance tick. Returns `Ok(None)` when the
    /// extension is not enabled.
    pub fn refresh_memory_budget(
        &self,
    ) -> super::error::Result<Option<vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static>>> {
        if !self.memory_budget_extension {
            return Ok(None);
        }
        let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        let mut properties = vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut budget);
        unsafe {
            self.instance
                .instance
                .get_physical_device_memory_properties2(self.physical, &mut properties);
        }
        let heaps = (self.memory.memory_heap_count as usize).min(vk::MAX_MEMORY_HEAPS);
        let mut snapshot = [(0u64, 0u64); vk::MAX_MEMORY_HEAPS];
        for (index, entry) in snapshot.iter_mut().enumerate().take(heaps) {
            *entry = (budget.heap_budget[index], budget.heap_usage[index]);
        }
        self.allocator_stats
            .apply_driver_heap_budgets(&snapshot[..heaps]);
        let budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT {
            heap_budget: budget.heap_budget,
            heap_usage: budget.heap_usage,
            ..Default::default()
        };
        if let Some(physical) = self.physical_info.get() {
            physical.publish_memory_budget(&budget);
        }
        Ok(Some(budget))
    }
    /// Whether `VK_EXT_memory_budget` was enabled at creation.
    pub fn memory_budget_enabled(&self) -> bool {
        self.memory_budget_extension
    }
    pub fn queue_family_index_for(&self, kind: u8) -> crate::Result<u32> {
        match kind {
            0 => Ok(self.queue_family),
            1 => self
                .additional_queues
                .iter()
                .map(|queue| queue.family)
                .find(|family| *family != self.queue_family)
                .ok_or_else(|| crate::Error::unsupported("设备未创建独立队列族")),
            _ => Err(crate::Error::invalid_usage("未知队列族类别")),
        }
    }
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family
    }
    /// Debug utils when the instance enabled `VK_EXT_debug_utils`.
    pub fn debug_utils_option(&self) -> Option<super::debug::DebugUtils> {
        self.instance
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_debug_utils")
            .then(|| self.debug_utils())
    }
    pub fn debug_utils(&self) -> super::debug::DebugUtils {
        super::debug::DebugUtils::new(&self.shared().instance.instance, self.shared())
    }

    /// Runtime bindless descriptor limit.
    pub fn max_bindless_descriptor_count(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_sampled_images()
    }
    /// Waits for the device to become idle and destroys all completed
    /// retirements; same as [`Self::wait_idle`].
    pub fn wait_idle_and_recycle_resources(&self) -> crate::Result<()> {
        self.wait_idle()
    }
    /// Allocate a primary command buffer from a new pool for `family` and name
    /// it when debug utils are enabled.
    pub fn create_command_buffer(
        &self,
        family: u32,
        name: &str,
    ) -> crate::Result<super::command_buffer::VulkanCommandBuffer> {
        let command =
            super::command_buffer::VulkanCommandPool::new_for_family(self.shared(), family)?
                .allocate()?;
        if let Some(utils) = self.debug_utils_option() {
            let _ = unsafe { utils.set_object_name(command.handle, name) };
        }
        Ok(command)
    }
    pub fn create_buffer_impl(
        &self,
        desc: &super::buffer::BufferDesc,
    ) -> super::error::Result<super::buffer::Buffer> {
        super::buffer::Buffer::new(self.shared(), desc)
    }
    /// 创建 GPU 分析输出缓冲。
    pub fn create_gpu_profiler_output_buffer(
        &self,
        size: u64,
    ) -> super::error::Result<super::buffer::Buffer> {
        super::buffer::Buffer::new(
            self.shared(),
            &super::buffer::BufferDesc::new_gpu_to_cpu(
                size,
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
    }
    /// 初始化纹理数据：经暂存缓冲 + setup 命令缓冲同步上传（见 `raw::setup`）。
    pub fn init_texture_data(
        &self,
        texture: &super::texture::Texture,
        bytes: &[u8],
    ) -> super::error::Result<()> {
        super::setup::init_texture_data(&self.shared(), texture, bytes)
    }
    pub fn update_texture(
        &self,
        texture: &super::texture::Texture,
        bytes: &[u8],
    ) -> super::error::Result<()> {
        super::setup::update_texture(&self.shared(), texture, bytes)
    }

    /// Built-ins are cached weakly so live resources can own the device without a cycle.
    pub fn get_built_in_sampler(
        &self,
        index: usize,
    ) -> crate::Result<Option<Arc<super::sampler::Sampler>>> {
        if self.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let Some(spec) = super::sampler::BUILTIN_SAMPLER_SPECS.get(index) else {
            return Ok(None);
        };
        let mut cache = self.builtin_samplers.lock();
        if let Some(sampler) = cache[index]
            .as_ref()
            .and_then(|entry| entry.sampler.upgrade())
        {
            return Ok(Some(sampler));
        }
        // Discard the dead weak allocation before admitting its replacement.
        cache[index] = None;
        let (sampler, storage) =
            super::sampler::Sampler::new_builtin(self.shared(), &spec.create_info())?;
        cache[index] = Some(BuiltinSamplerEntry {
            sampler: Arc::downgrade(&sampler),
            _storage: storage,
        });
        Ok(Some(sampler))
    }
    pub fn try_get_built_in_sampler_by_name(
        &self,
        name: &str,
    ) -> crate::Result<Option<Arc<super::sampler::Sampler>>> {
        match super::sampler::BUILTIN_SAMPLER_SPECS
            .iter()
            .position(|s| s.name == name)
        {
            Some(index) => self.get_built_in_sampler(index),
            None => Ok(None),
        }
    }
    pub fn get_built_in_sampler_by_name(
        &self,
        name: &str,
    ) -> crate::Result<Arc<super::sampler::Sampler>> {
        self.try_get_built_in_sampler_by_name(name)?
            .ok_or_else(|| crate::Error::invalid_usage(format!("unknown built-in sampler: {name}")))
    }
    /// 查询设备是否支持给定扩展名。
    pub fn query_device_extension_support(&self, names: &[&str]) -> Vec<bool> {
        let available = match unsafe {
            self.instance
                .instance
                .enumerate_device_extension_properties(self.physical)
        } {
            Ok(extensions) => extensions
                .iter()
                .map(|e| {
                    unsafe { std::ffi::CStr::from_ptr(e.extension_name.as_ptr()) }
                        .to_string_lossy()
                        .into_owned()
                })
                .collect::<Vec<_>>(),
            Err(_) => Vec::new(),
        };
        names
            .iter()
            .map(|name| available.iter().any(|have| have == name))
            .collect()
    }
    /// Whether the device-local heaps hold at least `min_device_local_bytes`.
    pub fn validate_physical_device_requirements(&self, min_device_local_bytes: u64) -> bool {
        let memory = unsafe {
            self.instance
                .instance
                .get_physical_device_memory_properties(self.physical)
        };
        let device_local = memory.memory_heaps[..memory.memory_heap_count as usize]
            .iter()
            .filter(|heap| heap.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL))
            .map(|heap| heap.size)
            .sum::<u64>();
        device_local >= min_device_local_bytes
    }
    /// Run `body` with the device allocator statistics.
    pub fn with_global_allocator<R>(
        &self,
        body: impl FnOnce(&super::allocator::AllocatorStats) -> R,
    ) -> R {
        body(&self.shared().allocator_stats)
    }
    /// 下发一次管线屏障（memory/buffer/image 三类数组按 scope 汇总）。
    pub fn issue_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
        scope: &super::barrier::SynchronizationScope,
    ) {
        unsafe {
            self.device.cmd_pipeline_barrier(
                command,
                scope.source_stages,
                scope.destination_stages,
                vk::DependencyFlags::empty(),
                &scope.memory,
                &scope.buffers,
                &scope.images,
            )
        }
    }
    /// 下发后立即清空 scope，供下一批复用。
    pub fn issue_and_reset_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
        scope: &mut super::barrier::SynchronizationScope,
    ) {
        self.issue_pipeline_barrier(command, scope);
        scope.memory.clear();
        scope.buffers.clear();
        scope.images.clear();
    }
    /// 全同步屏障（所有命令→所有命令）。
    pub fn issue_full_pipeline_barrier(&self, command: vk::CommandBuffer) {
        unsafe {
            self.device.cmd_pipeline_barrier(
                command,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[],
            )
        }
    }
    /// 在 setup 命令缓冲上录制并同步执行一次回调；结尾自动补全量内存屏障。
    pub fn with_setup_resources<R>(
        &self,
        resources: &[&dyn crate::resource::GpuUse],
        body: impl FnOnce(vk::CommandBuffer) -> R,
    ) -> super::error::Result<R> {
        super::setup::with_setup_resources(&self.shared(), resources, body)
    }
}

#[cfg(test)]
mod enabled_stage_tests {
    use super::*;

    #[test]
    fn optional_shader_stages_follow_enabled_core_features_and_can_be_disabled() {
        let mut capabilities = DeviceCapabilities::default();
        capabilities.record_enabled_core(
            &vk::PhysicalDeviceFeatures::default()
                .geometry_shader(true)
                .tessellation_shader(true)
                .wide_lines(true),
        );
        assert!(
            capabilities.geometry_shader
                && capabilities.tessellation_shader
                && capabilities.wide_lines
        );
        // A supported-feature structure is never involved in this policy. The
        // exact enabled structure, including explicit false fields, is authoritative.
        capabilities.record_enabled_core(&vk::PhysicalDeviceFeatures::default());
        assert!(
            !capabilities.geometry_shader
                && !capabilities.tessellation_shader
                && !capabilities.wide_lines
        );
    }
}
