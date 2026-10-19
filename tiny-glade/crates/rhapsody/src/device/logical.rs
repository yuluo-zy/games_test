//! The logical [`Device`]: owns its components and fixes their drop order.
//!
//! # Structure
//! `Device` is a thin owner of five components, each with its own state and
//! locks; methods are implemented next to the component they use:
//! - [`Caches`](super::caches) — managed-resource store, built-in samplers,
//!   shared setup context;
//! - [`Lifetime`](super::lifetime) — retirement queue and reclamation policy;
//! - [`Memory`](super::memory) — allocation accounting, suballocator, driver
//!   budgets, eviction registry;
//! - [`Queues`](super::queues) — queues, timelines, pending acquires, deferred
//!   host flushes, submission;
//! - [`DeviceCore`](super::native) — ash device/instance, capabilities, policy.
//!
//! # Lock order
//! Components never call into each other while holding one of their own
//! locks, with these documented nestings (outer → inner):
//! `Caches` (sampler table / setup slot) → `Memory` and managed-store locks;
//! `Queues` pending acquires → resource sync cells;
//! `Queues` per-queue locks (ascending queue index) → the native queue call only.
//! Retirement and eviction run their callbacks with no lock held.
//!
//! # Drop order
//! `Device::drop` waits for the GPU and drains the retirement queue (the only
//! step needing several components). Fields then drop in declaration order:
//! caches → lifetime → memory (suballocator blocks freed) → queues (timeline
//! semaphores destroyed) → core (`VkDevice` destroyed, then the instance).
use ash::vk;
use std::sync::Arc;

use super::caches::Caches;
use super::lifetime::Lifetime;
use super::memory::Memory;
use super::native::DeviceCore;
use super::queues::Queues;

/// Logical Vulkan device. See the module documentation for its components,
/// lock order and teardown order.
pub struct Device {
    self_ref: std::sync::Weak<Self>,
    pub(crate) caches: Caches,
    pub(crate) lifetime: Lifetime,
    pub(crate) memory: Memory,
    pub(crate) queues: Queues,
    /// Must stay the last field: owns the `VkDevice` and the instance.
    pub(crate) core: DeviceCore,
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
    pub(crate) fn create_with_runtime(
        instance: Arc<crate::raw::instance::VulkanInstance>,
        physical: vk::PhysicalDevice,
        info: &vk::DeviceCreateInfo<'_>,
        plan: crate::device::QueuePlan,
        runtime: crate::device::init::RuntimeConfig,
    ) -> crate::Result<Arc<Self>> {
        // Prepare fallible host metadata before creating a native VkDevice.
        let stats = Memory::new_stats(&runtime)?;
        let caches = Caches::new(stats.clone())?;
        // Each component releases what it created when a later step fails.
        let core = unsafe { DeviceCore::create(instance, physical, info, &runtime)? };
        let queues = unsafe { Queues::create(&core.device, info, plan)? };
        let memory = Memory::create(&core, stats, &runtime)?;
        let device = Arc::new_cyclic(|weak| Self {
            self_ref: weak.clone(),
            caches,
            lifetime: Lifetime::new(runtime.reclamation),
            memory,
            queues,
            core,
        });
        if let Err(error) = device.refresh_memory_budget() {
            log::warn!("initial memory budget query failed: {error}");
        }
        Ok(device)
    }

    pub fn wait_idle(&self) -> crate::Result<()> {
        self.wait_idle_and_collect().map(|_| ())
    }

    /// 此逻辑设备的生命周期标识；同一物理 GPU 上创建的设备也拥有不同代际。
    pub fn generation(&self) -> crate::device::DeviceGeneration {
        self.core.generation
    }
    pub fn caps(&self) -> &crate::device::DeviceCaps {
        &self.core.caps
    }
    pub fn policy(&self) -> &crate::device::ExecutionPolicy {
        &self.core.policy
    }
    pub fn frames_in_flight(&self) -> u32 {
        self.core.frames_in_flight
    }
    /// Features actually enabled at creation.
    pub fn capabilities(&self) -> &super::DeviceCapabilities {
        &self.core.capabilities
    }
    /// Device extensions enabled at creation.
    pub fn enabled_extensions(&self) -> &[String] {
        &self.core.enabled_extensions
    }
    /// Physical command limit; callers must also check the actually enabled
    /// MultiDrawIndirect / DrawIndirectCount features in this device's caps.
    pub fn max_draw_indirect_count(&self) -> u32 {
        self.core.properties.limits.max_draw_indirect_count
    }
    pub fn reclamation_config(&self) -> crate::device::ReclamationConfig {
        self.lifetime.reclamation
    }
    /// Memory types and heaps of the physical device.
    pub fn memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.core.memory_properties
    }
    /// Number of `vkUpdateDescriptorSets` calls issued (statistics).
    pub fn descriptor_update_count(&self) -> u64 {
        self.core
            .descriptor_update_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }
    /// `maxDescriptorSetUpdateAfterBindSampledImages` of the physical device.
    pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
        let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
        unsafe {
            self.core
                .instance
                .instance
                .get_physical_device_properties2(self.core.physical, &mut properties);
        }
        indexing.max_descriptor_set_update_after_bind_sampled_images
    }

    /// Interop escape hatch: the ash dispatch table of the `VkDevice`.
    ///
    /// Every call through it is `unsafe` and bypasses this crate's
    /// synchronization, retirement and accounting; objects created through it
    /// must be destroyed by the caller before the device is dropped.
    pub fn raw_device(&self) -> &ash::Device {
        &self.core.device
    }
    /// Interop escape hatch: the instance this device was created from.
    pub fn instance(&self) -> &Arc<crate::raw::instance::VulkanInstance> {
        &self.core.instance
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // Owned Vulkan children hold a device Arc and are already gone; only
        // queued retirements remain. Draining needs the core (destroy calls)
        // and memory (frees), so it happens before any component drops.
        self.drain_retirement_for_drop();
        // Cached entries only hold weak handles and host metadata tickets.
        self.caches.clear_samplers();
        // Components drop in field order (see the module documentation).
    }
}

impl Device {
    /// Flush every queued non-coherent dynamic-constants write with one
    /// `vkFlushMappedMemoryRanges` call. Runs before each queue submit; a
    /// no-op (one atomic load) when nothing is pending, e.g. coherent memory.
    pub(crate) fn flush_deferred_host_writes(&self) -> crate::Result<()> {
        self.queues.host_flush.flush_with(|ranges| {
            crate::raw::error::observe_vulkan(self, unsafe {
                self.core.device.flush_mapped_memory_ranges(ranges)
            })
        })
    }
    /// Number of deferred host-flush calls issued (statistics).
    pub fn deferred_host_flush_count(&self) -> u64 {
        self.queues.host_flush.flush_calls()
    }
    pub fn min_subgroup_size_option(&self) -> Option<u32> {
        if !self
            .core
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_subgroup_size_control")
        {
            return None;
        }
        let mut control = vk::PhysicalDeviceSubgroupSizeControlProperties::default();
        let mut props = vk::PhysicalDeviceProperties2::default().push_next(&mut control);
        unsafe {
            self.core
                .instance
                .instance
                .get_physical_device_properties2(self.core.physical, &mut props);
        }
        Some(control.min_subgroup_size)
    }
    pub fn device_uuid(&self) -> [u8; 16] {
        let mut id = vk::PhysicalDeviceIDProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
        unsafe {
            self.core
                .instance
                .instance
                .get_physical_device_properties2(self.core.physical, &mut properties);
        }
        id.device_uuid
    }
    pub fn pipeline_cache_uuid(&self) -> [u8; 16] {
        self.core.properties.pipeline_cache_uuid
    }
    pub fn limits(&self) -> &vk::PhysicalDeviceLimits {
        &self.core.properties.limits
    }
    /// Physical-device properties captured at device creation.
    pub fn properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.core.properties
    }

    /// Minimum subgroup size, or 1 when `VK_EXT_subgroup_size_control` is absent.
    pub fn min_subgroup_size(&self) -> u32 {
        self.min_subgroup_size_option().unwrap_or(1)
    }
    /// Raw physical device handle; see [`Self::physical_device_info`].
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.core.physical
    }
    /// Physical device wrapper attached by [`Self::attach_physical_device_info`].
    pub fn physical_device_info(
        &self,
    ) -> Option<&Arc<crate::raw::physical_device::PhysicalDevice>> {
        self.core.physical_info.get()
    }
    /// 挂接物理设备包装（只能设置一次）。
    pub fn attach_physical_device_info(
        &self,
        info: Arc<crate::raw::physical_device::PhysicalDevice>,
    ) -> Result<(), Arc<crate::raw::physical_device::PhysicalDevice>> {
        self.core.physical_info.set(info)
    }
    pub fn queue_family_index_for(&self, kind: u8) -> crate::Result<u32> {
        match kind {
            0 => Ok(self.queues.primary_family),
            1 => self
                .queues
                .additional
                .iter()
                .map(|queue| queue.family)
                .find(|family| *family != self.queues.primary_family)
                .ok_or_else(|| crate::Error::unsupported("设备未创建独立队列族")),
            _ => Err(crate::Error::invalid_usage("未知队列族类别")),
        }
    }
    pub fn queue_family_index(&self) -> u32 {
        self.queues.primary_family
    }
    /// Debug utils when the instance enabled `VK_EXT_debug_utils`.
    pub fn debug_utils_option(&self) -> Option<crate::raw::debug::DebugUtils> {
        self.core
            .instance
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_debug_utils")
            .then(|| self.debug_utils())
    }
    pub fn debug_utils(&self) -> crate::raw::debug::DebugUtils {
        crate::raw::debug::DebugUtils::new(&self.shared().core.instance.instance, self.shared())
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
    ) -> crate::Result<crate::raw::command_buffer::VulkanCommandBuffer> {
        let command =
            crate::raw::command_buffer::VulkanCommandPool::new_for_family(self.shared(), family)?
                .allocate()?;
        if let Some(utils) = self.debug_utils_option() {
            let _ = unsafe { utils.set_object_name(command.handle, name) };
        }
        Ok(command)
    }
    pub fn create_buffer_impl(
        &self,
        desc: &crate::raw::buffer::BufferDesc,
    ) -> crate::raw::error::Result<crate::raw::buffer::Buffer> {
        crate::raw::buffer::Buffer::new(self.shared(), desc)
    }
    /// 创建 GPU 分析输出缓冲。
    pub fn create_gpu_profiler_output_buffer(
        &self,
        size: u64,
    ) -> crate::raw::error::Result<crate::raw::buffer::Buffer> {
        crate::raw::buffer::Buffer::new(
            self.shared(),
            &crate::raw::buffer::BufferDesc::new_gpu_to_cpu(
                size,
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
    }
    /// 初始化纹理数据：经暂存缓冲在共享 setup 上下文上异步上传（见 `raw::setup`），不等待 GPU。
    pub fn init_texture_data(
        &self,
        texture: &crate::raw::texture::Texture,
        bytes: &[u8],
    ) -> crate::raw::error::Result<()> {
        crate::raw::setup::init_texture_data(&self.shared(), texture, bytes)
    }
    pub fn update_texture(
        &self,
        texture: &crate::raw::texture::Texture,
        bytes: &[u8],
    ) -> crate::raw::error::Result<()> {
        crate::raw::setup::update_texture(&self.shared(), texture, bytes)
    }

    /// 查询设备是否支持给定扩展名。
    pub fn query_device_extension_support(&self, names: &[&str]) -> Vec<bool> {
        let available = match unsafe {
            self.core
                .instance
                .instance
                .enumerate_device_extension_properties(self.core.physical)
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
    /// Record one barrier batch through the enabled barrier API
    /// (synchronization2 when enabled, legacy otherwise; empty stage masks
    /// keep their empty-scope meaning on both).
    ///
    /// # Safety
    /// `command` must be recording on this device and every barrier resource
    /// must stay alive through the command's completion.
    pub unsafe fn issue_barriers(
        &self,
        command: vk::CommandBuffer,
        scope: &crate::raw::barrier::SynchronizationScope,
    ) -> crate::raw::error::Result<()> {
        unsafe { crate::raw::barrier::BarrierDispatch::new(self).issue(self, command, scope) }
    }
    /// Record one barrier batch; see [`Self::issue_barriers`].
    ///
    /// # Safety
    /// As [`Self::issue_barriers`].
    pub unsafe fn issue_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
        scope: &crate::raw::barrier::SynchronizationScope,
    ) -> crate::raw::error::Result<()> {
        unsafe { self.issue_barriers(command, scope) }
    }
    /// Record the batch, then clear its barriers for reuse.
    ///
    /// # Safety
    /// As [`Self::issue_barriers`].
    pub unsafe fn issue_and_reset_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
        scope: &mut crate::raw::barrier::SynchronizationScope,
    ) -> crate::raw::error::Result<()> {
        let issued = unsafe { self.issue_barriers(command, scope) };
        scope.memory.clear();
        scope.buffers.clear();
        scope.images.clear();
        issued
    }
    /// Full execution and memory dependency (all commands to all commands).
    ///
    /// # Safety
    /// `command` must be recording on this device.
    pub unsafe fn issue_full_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
    ) -> crate::raw::error::Result<()> {
        let mut scope = crate::raw::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::ALL_COMMANDS,
        );
        scope.memory.push(
            vk::MemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
                .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE),
        );
        unsafe { self.issue_barriers(command, &scope) }
    }
    /// Record `body` after the declared accesses on the shared setup context
    /// and submit without waiting (see [`crate::raw::setup::with_setup_resources`]).
    pub fn with_setup_resources<R>(
        &self,
        accesses: &[crate::raw::setup::SetupAccess<'_>],
        body: impl FnOnce(vk::CommandBuffer) -> R,
    ) -> crate::raw::error::Result<crate::raw::setup::SetupSubmission<R>> {
        crate::raw::setup::with_setup_resources(&self.shared(), accesses, body)
    }
}
