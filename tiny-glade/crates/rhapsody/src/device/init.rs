//! 配置驱动的设备选择与功能启用；生产创建入口不依赖旧驱动排序或固定扩展清单。
use super::{
    AllocationStrategy, DeviceCaps, ExecutionPolicy, Feature, FeatureSet, QueueConfig,
    RecordingPolicy,
};
use crate::raw::{
    device::Device,
    instance::{InstanceBuilder, VulkanInstance},
};
use crate::{Error, Result};
use ash::vk;
use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DevicePreference {
    #[default]
    Discrete,
    Integrated,
    ByName(String),
    ByUuid([u8; vk::UUID_SIZE]),
}

/// Native submission strategy. Legacy disables synchronization2 while retaining
/// device timeline completion, dynamic rendering and descriptor capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SubmissionApi {
    /// Enable synchronization2 when the selected device supports it.
    #[default]
    Auto,
    /// Use vkQueueSubmit with timeline semaphore submission information.
    Legacy,
}

#[derive(Clone, Debug)]
pub struct DeviceConfig {
    /// Device memory budget; cached and retiring allocations still count.
    /// Per-heap `VK_EXT_memory_budget` limits apply in addition when enabled.
    pub memory_budget_bytes: u64,
    /// Budget for renderer-owned host metadata, enforced separately from
    /// device memory (`0` = unbounded).
    pub host_memory_budget_bytes: u64,
    pub queues: QueueConfig,
    /// 使用现有实例时要求验证层已启用；headless 入口负责创建它。
    pub validation: bool,
    pub required_features: FeatureSet,
    pub preferred_device: DevicePreference,
    pub frames_in_flight: u32,
    pub recording: RecordingPolicy,
    pub allocation: AllocationStrategy,
    pub submission_api: SubmissionApi,
    /// Per-maintenance-tick limits; explicit completed-object drains and
    /// shutdown are exempt from the normal/pressure quotas.
    pub reclamation: super::ReclamationConfig,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            memory_budget_bytes: 1024 * 1024 * 1024,
            host_memory_budget_bytes: 512 * 1024 * 1024,
            queues: QueueConfig::default(),
            validation: false,
            required_features: super::policy::bindless_features()
                .with(Feature::TimelineSemaphore)
                .with(Feature::DynamicRendering)
                .with(Feature::ShaderDemoteToHelperInvocation),
            preferred_device: DevicePreference::default(),
            frames_in_flight: 2,
            recording: RecordingPolicy::default(),
            submission_api: SubmissionApi::Auto,
            reclamation: super::ReclamationConfig::default(),
            allocation: if cfg!(feature = "suballocation") {
                AllocationStrategy::Suballocated
            } else {
                AllocationStrategy::Dedicated
            },
        }
    }
}

impl DeviceConfig {
    pub fn headless() -> Self {
        Self {
            queues: QueueConfig {
                require_present: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn validate(&self, has_surface: bool) -> Result<()> {
        self.reclamation.validate()?;
        if self.submission_api == SubmissionApi::Legacy
            && self.required_features.contains(Feature::Synchronization2)
        {
            return Err(Error::invalid_usage(
                "legacy submission conflicts with required synchronization2",
            ));
        }
        if self.memory_budget_bytes == 0 {
            return Err(Error::invalid_usage("memory budget must be nonzero"));
        }
        if !(1..=3).contains(&self.frames_in_flight) {
            return Err(Error::invalid_usage("frames_in_flight 必须在 1..=3 内"));
        }
        if self.queues.require_present && !has_surface {
            return Err(Error::invalid_usage("要求呈现的设备配置缺少 surface"));
        }
        if !cfg!(feature = "suballocation") && self.allocation == AllocationStrategy::Suballocated {
            return Err(Error::unsupported("未编译 suballocation 功能"));
        }
        // 可选的扩展实现尚未接入生产后端；不以特性位冒充实现策略。
        let supported_request = Feature::ALL
            .into_iter()
            .filter(|feature| {
                !matches!(
                    feature,
                    Feature::DescriptorBuffer
                        | Feature::GraphicsPipelineLibrary
                        | Feature::ShaderObject
                        | Feature::MeshShader
                        | Feature::PresentId
                        | Feature::PresentWait
                )
            })
            .fold(FeatureSet::EMPTY, |set, feature| set.with(feature));
        if !supported_request.is_superset(self.required_features) {
            return Err(Error::unsupported("请求的可选特性尚未接入设备创建协议"));
        }
        Ok(())
    }
}

pub(crate) struct RuntimeConfig {
    pub reclamation: super::ReclamationConfig,
    pub memory_budget_bytes: u64,
    pub host_memory_budget_bytes: u64,
    pub caps: DeviceCaps,
    pub policy: ExecutionPolicy,
    pub frames_in_flight: u32,
}

pub(crate) fn core_features(
    core: &vk::PhysicalDeviceFeatures,
    v12: &vk::PhysicalDeviceVulkan12Features<'_>,
) -> FeatureSet {
    let fields = [
        (Feature::WideLines, core.wide_lines),
        (Feature::ImageCubeArray, core.image_cube_array),
        (Feature::MultiDrawIndirect, core.multi_draw_indirect),
        (
            Feature::DrawIndirectFirstInstance,
            core.draw_indirect_first_instance,
        ),
        (Feature::DrawIndirectCount, v12.draw_indirect_count),
        (
            Feature::SeparateDepthStencilLayouts,
            v12.separate_depth_stencil_layouts,
        ),
        (
            Feature::ShaderStorageBufferArrayDynamicIndexing,
            core.shader_storage_buffer_array_dynamic_indexing,
        ),
        (
            Feature::ShaderSampledImageArrayDynamicIndexing,
            core.shader_sampled_image_array_dynamic_indexing,
        ),
        (
            Feature::ShaderStorageImageArrayDynamicIndexing,
            core.shader_storage_image_array_dynamic_indexing,
        ),
        (Feature::TimelineSemaphore, v12.timeline_semaphore),
        (Feature::BufferDeviceAddress, v12.buffer_device_address),
        (Feature::DescriptorIndexing, v12.descriptor_indexing),
        (
            Feature::RuntimeDescriptorArray,
            v12.runtime_descriptor_array,
        ),
        (
            Feature::DescriptorBindingPartiallyBound,
            v12.descriptor_binding_partially_bound,
        ),
        (
            Feature::DescriptorBindingVariableDescriptorCount,
            v12.descriptor_binding_variable_descriptor_count,
        ),
        (
            Feature::SampledImageUpdateAfterBind,
            v12.descriptor_binding_sampled_image_update_after_bind,
        ),
        (
            Feature::StorageBufferUpdateAfterBind,
            v12.descriptor_binding_storage_buffer_update_after_bind,
        ),
        (
            Feature::StorageImageUpdateAfterBind,
            v12.descriptor_binding_storage_image_update_after_bind,
        ),
        (
            Feature::ShaderStorageImageArrayNonUniformIndexing,
            v12.shader_storage_image_array_non_uniform_indexing,
        ),
        (
            Feature::ShaderStorageBufferArrayNonUniformIndexing,
            v12.shader_storage_buffer_array_non_uniform_indexing,
        ),
        (
            Feature::ShaderSampledImageArrayNonUniformIndexing,
            v12.shader_sampled_image_array_non_uniform_indexing,
        ),
        (
            Feature::DescriptorBindingUpdateUnusedWhilePending,
            v12.descriptor_binding_update_unused_while_pending,
        ),
    ];
    fields
        .into_iter()
        .filter(|(_, enabled)| *enabled != 0)
        .fold(FeatureSet::EMPTY, |set, (feature, _)| set.with(feature))
}

fn selection_rank(
    preference: &DevicePreference,
    kind: vk::PhysicalDeviceType,
    name: &str,
    uuid: [u8; vk::UUID_SIZE],
) -> Option<u8> {
    match preference {
        DevicePreference::ByName(wanted) => (name == wanted).then_some(0),
        DevicePreference::ByUuid(wanted) => (uuid == *wanted).then_some(0),
        DevicePreference::Discrete => Some(if kind == vk::PhysicalDeviceType::DISCRETE_GPU {
            0
        } else {
            1
        }),
        DevicePreference::Integrated => Some(if kind == vk::PhysicalDeviceType::INTEGRATED_GPU {
            0
        } else {
            1
        }),
    }
}

impl Device {
    pub fn new_headless(config: &DeviceConfig) -> Result<Arc<Self>> {
        config.validate(false)?;
        let mut builder = InstanceBuilder::default();
        if config.validation {
            builder
                .layers
                .push(CString::new("VK_LAYER_KHRONOS_validation").expect("静态层名无 NUL"));
        }
        Self::new(builder.build()?, None, config)
    }

    /// surface 必须属于 instance；设备选择、队列规划和特性协商在任何资源创建之前完成。
    pub fn new(
        instance: Arc<VulkanInstance>,
        surface: Option<&crate::raw::swapchain::Surface>,
        config: &DeviceConfig,
    ) -> Result<Arc<Self>> {
        config.validate(surface.is_some())?;
        if let Some(surface) = surface
            && !Arc::ptr_eq(&surface.instance, &instance)
        {
            return Err(Error::invalid_usage("surface 不属于所选实例"));
        }
        if config.validation
            && !instance
                .layers
                .iter()
                .any(|name| name == "VK_LAYER_KHRONOS_validation")
        {
            return Err(Error::invalid_usage("实例未启用请求的验证层"));
        }
        let mut candidates = Vec::new();
        for physical in unsafe { instance.instance.enumerate_physical_devices() }? {
            let mut id = vk::PhysicalDeviceIDProperties::default();
            let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
            unsafe {
                instance
                    .instance
                    .get_physical_device_properties2(physical, &mut properties)
            };
            let properties = properties.properties;
            let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
            if let Some(rank) = selection_rank(
                &config.preferred_device,
                properties.device_type,
                &name,
                id.device_uuid,
            ) {
                candidates.push((rank, physical, properties));
            }
        }
        candidates.sort_by_key(|(rank, _, _)| *rank);
        let mut last_unsupported = None;
        for (_, physical, properties) in candidates {
            match create_candidate(
                instance.clone(),
                physical,
                properties,
                surface,
                config,
                None,
                &[],
            ) {
                Ok(device) => return Ok(device),
                Err(error @ Error::Unsupported(_)) => last_unsupported = Some(error),
                // 真实 OOM/设备丢失不得被聚合成 Unsupported 或字符串。
                Err(error) => return Err(error),
            }
        }
        Err(last_unsupported.unwrap_or_else(|| Error::unsupported("没有符合设备偏好的物理设备")))
    }
}

pub(crate) fn create_candidate(
    instance: Arc<VulkanInstance>,
    physical: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties,
    surface: Option<&crate::raw::swapchain::Surface>,
    config: &DeviceConfig,
    selected_plan: Option<super::QueuePlan>,
    required_extensions: &[&CStr],
) -> Result<Arc<Device>> {
    if properties.api_version < vk::API_VERSION_1_2 || instance.api_version < vk::API_VERSION_1_2 {
        return Err(Error::unsupported("设备创建要求 Vulkan 1.2"));
    }
    let available = unsafe {
        instance
            .instance
            .enumerate_device_extension_properties(physical)
    }?;
    let has = |name: &CStr| {
        available
            .iter()
            .any(|p| unsafe { CStr::from_ptr(p.extension_name.as_ptr()) } == name)
    };
    let core13 = properties.api_version.min(instance.api_version) >= vk::API_VERSION_1_3;
    let has_sync2 = core13 || has(ash::khr::synchronization2::NAME);
    let has_dynamic = core13 || has(ash::khr::dynamic_rendering::NAME);
    let has_demote = core13 || has(ash::ext::shader_demote_to_helper_invocation::NAME);
    let has_maintenance = has(ash::ext::swapchain_maintenance1::NAME)
        && instance
            .enabled_extensions
            .iter()
            .any(|name| name == "VK_EXT_surface_maintenance1");
    let mut maintenance = vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT::default();
    let mut v11 = vk::PhysicalDeviceVulkan11Features::default();
    let mut v12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut sync2 = vk::PhysicalDeviceSynchronization2Features::default();
    let mut dynamic = vk::PhysicalDeviceDynamicRenderingFeatures::default();
    let mut demote = vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures::default();
    let mut descriptor_buffer = vk::PhysicalDeviceDescriptorBufferFeaturesEXT::default();
    let mut pipeline_library = vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT::default();
    let mut shader_object = vk::PhysicalDeviceShaderObjectFeaturesEXT::default();
    let mut mesh = vk::PhysicalDeviceMeshShaderFeaturesEXT::default();
    let mut present_id = vk::PhysicalDevicePresentIdFeaturesKHR::default();
    let mut present_wait = vk::PhysicalDevicePresentWaitFeaturesKHR::default();
    let mut features = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut v11)
        .push_next(&mut v12);
    if has_maintenance {
        features = features.push_next(&mut maintenance);
    }
    if has_sync2 {
        features = features.push_next(&mut sync2);
    }
    if has_dynamic {
        features = features.push_next(&mut dynamic);
    }
    if has_demote {
        features = features.push_next(&mut demote);
    }
    if has(ash::ext::descriptor_buffer::NAME) {
        features = features.push_next(&mut descriptor_buffer);
    }
    if has(ash::ext::graphics_pipeline_library::NAME) {
        features = features.push_next(&mut pipeline_library);
    }
    if has(ash::ext::shader_object::NAME) {
        features = features.push_next(&mut shader_object);
    }
    if has(ash::ext::mesh_shader::NAME) {
        features = features.push_next(&mut mesh);
    }
    if has(ash::khr::present_id::NAME) {
        features = features.push_next(&mut present_id);
    }
    if has(ash::khr::present_wait::NAME) {
        features = features.push_next(&mut present_wait);
    }
    unsafe {
        instance
            .instance
            .get_physical_device_features2(physical, &mut features)
    };
    let core = features.features;
    let mut supported = core_features(&core, &v12);
    if v11.shader_draw_parameters != 0 {
        supported = supported.with(Feature::ShaderDrawParameters);
    }
    if sync2.synchronization2 != 0 {
        supported = supported.with(Feature::Synchronization2);
    }
    if dynamic.dynamic_rendering != 0 {
        supported = supported.with(Feature::DynamicRendering);
    }
    if demote.shader_demote_to_helper_invocation != 0 {
        supported = supported.with(Feature::ShaderDemoteToHelperInvocation);
    }
    if has(ash::ext::memory_budget::NAME) {
        supported = supported.with(Feature::MemoryBudget);
    }
    for (feature, value) in [
        (
            Feature::DescriptorBuffer,
            descriptor_buffer.descriptor_buffer,
        ),
        (
            Feature::GraphicsPipelineLibrary,
            pipeline_library.graphics_pipeline_library,
        ),
        (Feature::ShaderObject, shader_object.shader_object),
        (Feature::MeshShader, mesh.mesh_shader),
        (Feature::PresentId, present_id.present_id),
        (Feature::PresentWait, present_wait.present_wait),
    ] {
        if value != 0 {
            supported = supported.with(feature);
        }
    }
    if maintenance.swapchain_maintenance1 != 0 {
        supported = supported.with(Feature::SwapchainMaintenance1);
    }
    let required = config
        .required_features
        .with(Feature::TimelineSemaphore)
        .with(Feature::DynamicRendering)
        // 原版 rhapsody 底层把该扩展列入必需设备扩展，缺失即拒绝适配器。
        .with(Feature::ShaderDemoteToHelperInvocation);
    for feature in required.iter() {
        DeviceCaps::new(supported, FeatureSet::EMPTY).require_supported(feature)?;
    }
    let use_sync2 = config.submission_api == SubmissionApi::Auto && sync2.synchronization2 != 0;

    let properties = unsafe {
        instance
            .instance
            .get_physical_device_queue_family_properties(physical)
    };
    let mut families = Vec::with_capacity(properties.len());
    for (index, props) in properties.iter().enumerate() {
        let present = if let Some(surface) = surface {
            unsafe {
                surface.loader.get_physical_device_surface_support(
                    physical,
                    index as u32,
                    surface.handle,
                )
            }?
        } else {
            false
        };
        families.push(super::FamilyInfo::from_properties(props, present));
    }
    let plan = match selected_plan {
        Some(plan) => plan,
        None => super::plan_queues(&families, &config.queues)?,
    };
    let counts = plan.create_counts();
    let priorities: Vec<_> = counts.iter().map(|(_, n)| vec![1.0; *n as usize]).collect();
    let queues: Vec<_> = counts
        .iter()
        .zip(&priorities)
        .map(|((family, _), priorities)| {
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(*family)
                .queue_priorities(priorities)
        })
        .collect();
    let mut extensions = Vec::new();
    for name in required_extensions {
        if !has(name) {
            return Err(Error::unsupported(format!(
                "缺少请求的设备扩展 {}",
                name.to_string_lossy()
            )));
        }
        if !extensions.contains(&name.as_ptr()) {
            extensions.push(name.as_ptr());
        }
    }
    if surface.is_some() {
        if !has(ash::khr::swapchain::NAME) {
            return Err(Error::unsupported("设备缺少 VK_KHR_swapchain"));
        }
        extensions.push(ash::khr::swapchain::NAME.as_ptr());
        if cfg!(target_os = "windows")
            && has(ash::ext::full_screen_exclusive::NAME)
            && instance
                .enabled_extensions
                .iter()
                .any(|e| e == "VK_KHR_get_surface_capabilities2")
            && instance
                .enabled_extensions
                .iter()
                .any(|e| e == "VK_KHR_win32_surface")
        {
            extensions.push(ash::ext::full_screen_exclusive::NAME.as_ptr());
        }
    }
    if supported.contains(Feature::MemoryBudget) {
        extensions.push(ash::ext::memory_budget::NAME.as_ptr());
    }
    // 在 1.2 实例上通过扩展入口使用 1.3 特性，不能调用未启用的核心函数。
    if !core13 && use_sync2 {
        extensions.push(ash::khr::synchronization2::NAME.as_ptr());
    }
    if !core13 && dynamic.dynamic_rendering != 0 {
        extensions.push(ash::khr::dynamic_rendering::NAME.as_ptr());
    }
    if !core13 && demote.shader_demote_to_helper_invocation != 0 {
        extensions.push(ash::ext::shader_demote_to_helper_invocation::NAME.as_ptr());
    }
    if has(ash::khr::portability_subset::NAME) {
        extensions.push(ash::khr::portability_subset::NAME.as_ptr());
    }
    // 只启用渲染器实际使用的核心字段；不把完整驱动支持结构原样用作启用结构。
    let enabled_core = vk::PhysicalDeviceFeatures::default()
        .robust_buffer_access(core.robust_buffer_access != 0)
        .wide_lines(core.wide_lines != 0)
        .image_cube_array(core.image_cube_array != 0)
        .shader_storage_buffer_array_dynamic_indexing(
            core.shader_storage_buffer_array_dynamic_indexing != 0,
        )
        .shader_sampled_image_array_dynamic_indexing(
            core.shader_sampled_image_array_dynamic_indexing != 0,
        )
        .shader_storage_image_array_dynamic_indexing(
            core.shader_storage_image_array_dynamic_indexing != 0,
        )
        .sampler_anisotropy(core.sampler_anisotropy != 0)
        .fill_mode_non_solid(core.fill_mode_non_solid != 0)
        .texture_compression_bc(core.texture_compression_bc != 0)
        .shader_storage_image_extended_formats(core.shader_storage_image_extended_formats != 0)
        .shader_storage_image_write_without_format(
            core.shader_storage_image_write_without_format != 0,
        )
        .draw_indirect_first_instance(core.draw_indirect_first_instance != 0)
        .multi_draw_indirect(core.multi_draw_indirect != 0)
        .dual_src_blend(core.dual_src_blend != 0)
        .fragment_stores_and_atomics(core.fragment_stores_and_atomics != 0)
        .vertex_pipeline_stores_and_atomics(core.vertex_pipeline_stores_and_atomics != 0);
    let mut enabled11 = vk::PhysicalDeviceVulkan11Features::default()
        .shader_draw_parameters(v11.shader_draw_parameters != 0);
    let mut enabled12 = vk::PhysicalDeviceVulkan12Features::default()
        .timeline_semaphore(true)
        .draw_indirect_count(v12.draw_indirect_count != 0)
        // Optional: lets the graph track depth and stencil independently.
        .separate_depth_stencil_layouts(v12.separate_depth_stencil_layouts != 0)
        .buffer_device_address(v12.buffer_device_address != 0)
        .descriptor_indexing(v12.descriptor_indexing != 0)
        .runtime_descriptor_array(v12.runtime_descriptor_array != 0)
        .descriptor_binding_partially_bound(v12.descriptor_binding_partially_bound != 0)
        .descriptor_binding_variable_descriptor_count(
            v12.descriptor_binding_variable_descriptor_count != 0,
        )
        .descriptor_binding_sampled_image_update_after_bind(
            v12.descriptor_binding_sampled_image_update_after_bind != 0,
        )
        .descriptor_binding_storage_buffer_update_after_bind(
            v12.descriptor_binding_storage_buffer_update_after_bind != 0,
        )
        .descriptor_binding_storage_image_update_after_bind(
            v12.descriptor_binding_storage_image_update_after_bind != 0,
        )
        .shader_storage_image_array_non_uniform_indexing(
            v12.shader_storage_image_array_non_uniform_indexing != 0,
        )
        .shader_sampled_image_array_non_uniform_indexing(
            v12.shader_sampled_image_array_non_uniform_indexing != 0,
        )
        .shader_storage_buffer_array_non_uniform_indexing(
            v12.shader_storage_buffer_array_non_uniform_indexing != 0,
        )
        .descriptor_binding_update_unused_while_pending(
            v12.descriptor_binding_update_unused_while_pending != 0,
        );
    let mut enabled = core_features(&enabled_core, &enabled12);
    if enabled11.shader_draw_parameters != 0 {
        enabled = enabled.with(Feature::ShaderDrawParameters);
    }
    if use_sync2 {
        enabled = enabled.with(Feature::Synchronization2);
    }
    if dynamic.dynamic_rendering != 0 {
        enabled = enabled.with(Feature::DynamicRendering);
    }
    if demote.shader_demote_to_helper_invocation != 0 {
        enabled = enabled.with(Feature::ShaderDemoteToHelperInvocation);
    }
    if supported.contains(Feature::MemoryBudget) {
        enabled = enabled.with(Feature::MemoryBudget);
    }
    if maintenance.swapchain_maintenance1 != 0 {
        extensions.push(ash::ext::swapchain_maintenance1::NAME.as_ptr());
        enabled = enabled.with(Feature::SwapchainMaintenance1);
    }
    maintenance.p_next = std::ptr::null_mut();
    sync2.p_next = std::ptr::null_mut();
    dynamic.p_next = std::ptr::null_mut();
    demote.p_next = std::ptr::null_mut();
    extensions.sort_by(|a, b| unsafe { CStr::from_ptr(*a).cmp(CStr::from_ptr(*b)) });
    extensions.dedup_by(|a, b| unsafe { CStr::from_ptr(*a) == CStr::from_ptr(*b) });
    let mut info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_extension_names(&extensions)
        .enabled_features(&enabled_core)
        .push_next(&mut enabled11)
        .push_next(&mut enabled12);
    if maintenance.swapchain_maintenance1 != 0 {
        info = info.push_next(&mut maintenance);
    }
    if use_sync2 {
        info = info.push_next(&mut sync2);
    }
    if dynamic.dynamic_rendering != 0 {
        info = info.push_next(&mut dynamic);
    }
    if demote.shader_demote_to_helper_invocation != 0 {
        info = info.push_next(&mut demote);
    }
    let caps = DeviceCaps::new(supported, enabled);
    let policy = super::plan_policy(
        &caps,
        &super::PolicyPreferences {
            recording: config.recording,
            allocation: config.allocation,
        },
    )?;
    let runtime = RuntimeConfig {
        reclamation: config.reclamation,
        memory_budget_bytes: config.memory_budget_bytes,
        host_memory_budget_bytes: config.host_memory_budget_bytes,
        caps,
        policy,
        frames_in_flight: config.frames_in_flight,
    };
    Device::create_with_runtime(instance, physical, &info, plan, runtime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submission_api_defaults_to_auto_and_legacy_rejects_a_required_sync2_conflict() {
        assert_eq!(DeviceConfig::default().submission_api, SubmissionApi::Auto);
        let mut config = DeviceConfig {
            submission_api: SubmissionApi::Legacy,
            ..DeviceConfig::headless()
        };
        assert!(config.validate(false).is_ok());
        config.required_features = config.required_features.with(Feature::Synchronization2);
        assert_eq!(
            config.validate(false).unwrap_err().kind(),
            crate::ErrorKind::InvalidUsage
        );
        config.submission_api = SubmissionApi::Auto;
        assert!(config.validate(false).is_ok());
    }

    #[test]
    fn default_config_requires_shader_demote_to_helper_invocation() {
        let config = DeviceConfig::headless();
        assert!(
            config
                .required_features
                .contains(Feature::ShaderDemoteToHelperInvocation)
        );
        assert!(config.validate(false).is_ok());
    }

    #[test]
    fn core_array_dynamic_indexing_caps_follow_each_actual_core_bit() {
        let features = [
            Feature::ShaderStorageBufferArrayDynamicIndexing,
            Feature::ShaderSampledImageArrayDynamicIndexing,
            Feature::ShaderStorageImageArrayDynamicIndexing,
        ];
        for (index, feature) in features.into_iter().enumerate() {
            let mut core = vk::PhysicalDeviceFeatures::default();
            match index {
                0 => core.shader_storage_buffer_array_dynamic_indexing = vk::TRUE,
                1 => core.shader_sampled_image_array_dynamic_indexing = vk::TRUE,
                _ => core.shader_storage_image_array_dynamic_indexing = vk::TRUE,
            }
            let v12 = vk::PhysicalDeviceVulkan12Features::default()
                .shader_storage_buffer_array_non_uniform_indexing(true)
                .shader_sampled_image_array_non_uniform_indexing(true)
                .shader_storage_image_array_non_uniform_indexing(true);
            let actual = core_features(&core, &v12);
            for other in features {
                assert_eq!(actual.contains(other), other == feature);
            }
        }
    }

    #[test]
    fn indirect_caps_distinguish_supported_from_actually_enabled_features() {
        let core = vk::PhysicalDeviceFeatures::default()
            .multi_draw_indirect(true)
            .draw_indirect_first_instance(true);
        let vulkan12 = vk::PhysicalDeviceVulkan12Features::default().draw_indirect_count(true);
        let supported = core_features(&core, &vulkan12);
        let disabled = core_features(
            &vk::PhysicalDeviceFeatures::default(),
            &vk::PhysicalDeviceVulkan12Features::default(),
        );
        let caps = DeviceCaps::new(supported, disabled);
        for feature in [
            Feature::MultiDrawIndirect,
            Feature::DrawIndirectFirstInstance,
            Feature::DrawIndirectCount,
        ] {
            assert!(caps.is_supported(feature));
            assert!(caps.require_enabled(feature).is_err());
            assert!(
                DeviceCaps::new(supported, core_features(&core, &vulkan12))
                    .require_enabled(feature)
                    .is_ok()
            );
        }
        let only_count = core_features(&vk::PhysicalDeviceFeatures::default(), &vulkan12);
        assert!(!only_count.contains(Feature::MultiDrawIndirect));
        assert!(!only_count.contains(Feature::DrawIndirectFirstInstance));
        assert!(only_count.contains(Feature::DrawIndirectCount));
    }

    #[test]
    fn config_rejects_invalid_frame_counts_and_surface_requirements() {
        assert!(DeviceConfig::default().validate(false).is_err());
        for count in [0, 4, u32::MAX] {
            let config = DeviceConfig {
                frames_in_flight: count,
                ..DeviceConfig::headless()
            };
            assert_eq!(
                config.validate(false).unwrap_err().kind(),
                crate::ErrorKind::InvalidUsage
            );
        }
        for count in 1..=3 {
            assert!(
                DeviceConfig {
                    frames_in_flight: count,
                    ..DeviceConfig::headless()
                }
                .validate(false)
                .is_ok()
            );
        }
    }

    #[test]
    fn explicit_device_identity_does_not_fall_back_to_another_gpu() {
        let name = DevicePreference::ByName("GPU".into());
        assert_eq!(
            selection_rank(&name, vk::PhysicalDeviceType::DISCRETE_GPU, "GPU2", [0; 16]),
            None
        );
        assert_eq!(
            selection_rank(
                &name,
                vk::PhysicalDeviceType::INTEGRATED_GPU,
                "GPU",
                [0; 16]
            ),
            Some(0)
        );
        let id = DevicePreference::ByUuid([7; 16]);
        assert_eq!(
            selection_rank(&id, vk::PhysicalDeviceType::DISCRETE_GPU, "GPU", [0; 16]),
            None
        );
        assert_eq!(
            selection_rank(&id, vk::PhysicalDeviceType::DISCRETE_GPU, "GPU", [7; 16]),
            Some(0)
        );
    }
}
