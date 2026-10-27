//! The logical [`Device`]: owns its components and fixes their drop order.
//!
//! # Structure
//! `Device` is a thin owner of five components, each with its own state and
//! locks; methods are implemented next to the component they use:
//! - [`Extensions`](super::extensions) — typed per-device state owned by the
//!   resource and transfer layers (managed-resource store, built-in samplers,
//!   parked setup pools), created on first use;
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
//! extension state (sampler table / setup pool list) → `Memory` and managed-store locks;
//! Pending acquire lists are drained before taking resource sync cells;
//! `Queues` per-queue locks (ascending queue index) → the native queue call only.
//! Retirement and eviction run their callbacks with no lock held.
//!
//! # Drop order
//! `Device::drop` waits for the GPU and drains the retirement queue (the only
//! step needing several components). Fields then drop in declaration order:
//! extensions → lifetime → memory (suballocator blocks freed) → queues (timeline
//! semaphores destroyed) → core (`VkDevice` destroyed, then the instance).
use ash::vk;
use std::sync::Arc;

use super::extensions::Extensions;
use super::lifetime::Lifetime;
use super::memory::Memory;
use super::native::DeviceCore;
use super::queues::Queues;

/// Logical Vulkan device. See the module documentation for its components,
/// lock order and teardown order.
pub struct Device {
    self_ref: std::sync::Weak<Self>,
    #[cfg(feature = "cpu-probe")]
    upload_probe: super::texture_upload_probe::TextureUploadProbeCounters,
    pub(crate) extensions: Extensions,
    pub(crate) lifetime: Lifetime,
    pub(crate) memory: Memory,
    pub(crate) queues: Queues,
    /// Must stay the last field: owns the `VkDevice` and the instance.
    pub(crate) core: DeviceCore,
}

impl Device {
    /// Instrument a known producer only after its real successful upload submit.
    /// Do not duplicate a setup/transfer hook. None invalidates payload evidence
    /// without changing the real submit. Counts never imply GPU completion.
    #[cfg(feature = "cpu-probe")]
    pub fn record_texture_upload_submission(&self, kind: super::TextureUploadProbeKind, payload: Option<u64>) {
        self.upload_probe.record(kind, payload);
    }
    #[cfg(feature = "cpu-probe")]
    pub fn texture_upload_probe_snapshot(&self) -> super::TextureUploadProbeSnapshot {
        self.upload_probe.snapshot(self.generation().value())
    }
    /// Every public device is constructed in an Arc. Internal caches hold only weak
    /// references; this upgrade is used while an external caller already owns it.
    pub(crate) fn shared(&self) -> Arc<Self> {
        // Every Device lives in an Arc and `&self` proves an owner exists.
        self.self_ref
            .upgrade()
            .expect("device method called with a live owner")
    }
    pub(crate) fn create_with_runtime(
        physical: Arc<crate::native::physical_device::PhysicalDevice>,
        info: &vk::DeviceCreateInfo<'_>,
        plan: crate::device::QueuePlan,
        runtime: crate::device::init::RuntimeConfig,
    ) -> crate::Result<Arc<Self>> {
        // Prepare fallible host metadata before creating a native VkDevice.
        let stats = Memory::new_stats(&runtime)?;
        // Each component releases what it created when a later step fails.
        let core = unsafe { DeviceCore::create(physical, info, &runtime)? };
        let queues = unsafe { Queues::create(&core.device, info, plan)? };
        let memory = Memory::create(&core, stats, &runtime)?;
        let device = Arc::new_cyclic(|weak| Self {
            self_ref: weak.clone(),
            #[cfg(feature = "cpu-probe")]
            upload_probe: Default::default(),
            extensions: Extensions::default(),
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
    pub fn generation(&self) -> crate::sync::DeviceGeneration {
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
    pub(crate) fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
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
    pub fn instance(&self) -> &Arc<crate::native::instance::VulkanInstance> {
        &self.core.instance
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // Owned Vulkan children hold a device Arc and are already gone; only
        // queued retirements remain. Draining needs the core (destroy calls)
        // and memory (frees), so it happens before any component drops.
        self.drain_retirement_for_drop();
        // The GPU is idle and every retired command buffer is freed: extensions drop the
        // built-in sampler tickets and destroy the parked setup pools (they hold no
        // device reference).
        self.extensions.release_for_device_drop();
        // Components drop in field order (see the module documentation).
    }
}

impl Device {
    /// Flush every queued non-coherent dynamic-constants write with one
    /// `vkFlushMappedMemoryRanges` call. Runs before each queue submit; a
    /// no-op (one atomic load) when nothing is pending, e.g. coherent memory.
    pub(crate) fn flush_deferred_host_writes(&self) -> crate::Result<()> {
        self.queues.host_flush.flush_with(|ranges| {
            crate::device::observe_vulkan(self, unsafe {
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

    /// Raw physical device handle; see [`Self::physical_device_info`].
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.core.physical
    }
    /// Immutable physical-device metadata captured during device creation.
    pub fn physical_device_info(&self) -> &Arc<crate::native::physical_device::PhysicalDevice> {
        &self.core.physical_info
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queues.primary_family
    }
    /// Debug utils when the instance enabled `VK_EXT_debug_utils`.
    pub(crate) fn debug_utils_option(&self) -> Option<crate::device::debug::DebugUtils> {
        self.core
            .instance
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_debug_utils")
            .then(|| self.debug_utils())
    }
    pub(crate) fn debug_utils(&self) -> crate::device::debug::DebugUtils {
        crate::device::debug::DebugUtils::new(&self.shared().core.instance.instance, self.shared())
    }

    /// Runtime bindless descriptor limit.
    pub fn max_bindless_descriptor_count(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_sampled_images()
    }

}

/// Publish terminal device loss at the native call boundary.
pub(crate) fn observe_vulkan<T>(
    device: &Device,
    result: std::result::Result<T, ash::vk::Result>,
) -> crate::Result<T> {
    result.map_err(|error| {
        if error == ash::vk::Result::ERROR_DEVICE_LOST {
            device.queues.timelines.mark_device_lost();
        }
        crate::Error::from(error)
    })
}
