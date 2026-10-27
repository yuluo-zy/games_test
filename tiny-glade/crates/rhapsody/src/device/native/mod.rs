//! [`DeviceCore`]: the ash handles, enabled capabilities and negotiated
//! policy of one logical device. Lock-free and immutable after creation.
use ash::vk;
use std::sync::Arc;

use crate::native::instance::VulkanInstance;
use crate::native::physical_device::PhysicalDevice;

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

    /// Read the features actually enabled by `info` (core features and the
    /// supported `pNext` feature structures).
    ///
    /// # Safety
    /// `info` and its `pNext` chain must be valid Vulkan structures.
    unsafe fn from_create_info(info: &vk::DeviceCreateInfo<'_>) -> Self {
        let mut capabilities = Self::default();
        unsafe {
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
        }
        capabilities
    }
}

/// Owns the `VkDevice`. Dropping it destroys the logical device and then
/// releases the instance (`instance` is the last field), so every other
/// device component must be dropped first.
pub(crate) struct DeviceCore {
    pub(crate) generation: crate::sync::DeviceGeneration,
    pub(crate) caps: crate::device::DeviceCaps,
    pub(crate) policy: crate::device::ExecutionPolicy,
    pub(crate) frames_in_flight: u32,
    pub(crate) properties: vk::PhysicalDeviceProperties,
    pub(crate) memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub(crate) physical_info: Arc<PhysicalDevice>,
    pub(crate) capabilities: DeviceCapabilities,
    /// 创建时实际启用的设备扩展名（`vkCreateDevice` 的 `ppEnabledExtensionNames`）。
    pub(crate) enabled_extensions: Vec<String>,
    /// Loaded once at creation when `VK_KHR_synchronization2` is enabled.
    pub(crate) synchronization2: Option<ash::khr::synchronization2::Device>,
    /// Number of `vkUpdateDescriptorSets` calls (statistics only).
    pub(crate) descriptor_update_count: std::sync::atomic::AtomicU64,
    pub(crate) device: ash::Device,
    pub(crate) physical: vk::PhysicalDevice,
    /// Must stay the last field: the instance outlives the logical device.
    pub(crate) instance: Arc<VulkanInstance>,
}

impl DeviceCore {
    /// Create the `VkDevice` for `info`; the returned core owns it.
    ///
    /// # Safety
    /// `info` must be a valid create info for `physical` of `instance`.
    pub(crate) unsafe fn create(
        physical_info: Arc<PhysicalDevice>,
        info: &vk::DeviceCreateInfo<'_>,
        runtime: &crate::device::init::RuntimeConfig,
    ) -> crate::Result<Self> {
        unsafe {
            let instance = physical_info.instance().clone();
            let physical = physical_info.raw_handle();
            let capabilities = DeviceCapabilities::from_create_info(info);
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
            let memory_properties = physical_info.memory;
            let properties = physical_info.properties;
            Ok(Self {
                generation: crate::sync::DeviceGeneration::next(),
                caps: runtime.caps,
                policy: runtime.policy,
                frames_in_flight: runtime.frames_in_flight,
                properties,
                memory_properties,
                physical_info,
                capabilities,
                enabled_extensions,
                synchronization2,
                descriptor_update_count: std::sync::atomic::AtomicU64::new(0),
                device,
                physical,
                instance,
            })
        }
    }

    pub(crate) fn has_extension(&self, name: &str) -> bool {
        self.enabled_extensions
            .iter()
            .any(|enabled| enabled == name)
    }
}

impl Drop for DeviceCore {
    fn drop(&mut self) {
        // SAFETY: every child object was destroyed by the components dropped
        // before this one; `instance` is released after this body.
        unsafe { self.device.destroy_device(None) };
    }
}

#[cfg(test)]
mod enabled_stage_tests;
