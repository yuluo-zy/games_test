use super::*;

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
