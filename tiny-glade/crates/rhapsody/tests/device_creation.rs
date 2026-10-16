//! 设备创建规则：必需扩展/特性/上限与队列配置。无 Vulkan 设备时跳过。
use ash::vk;
use rhapsody::raw::init::{GraphicsInstance, first_missing_required_feature};

#[test]
fn required_feature_chain_reports_the_first_missing_item_in_original_order() {
    let core_all = vk::PhysicalDeviceFeatures::default()
        .robust_buffer_access(true)
        .draw_indirect_first_instance(true)
        .fill_mode_non_solid(true)
        .sampler_anisotropy(true)
        .texture_compression_bc(true)
        .shader_storage_image_extended_formats(true)
        .shader_storage_image_write_without_format(true)
        .dual_src_blend(true)
        .multi_draw_indirect(true)
        .fragment_stores_and_atomics(true)
        .vertex_pipeline_stores_and_atomics(true);
    let v12_all = vk::PhysicalDeviceVulkan12Features::default()
        .timeline_semaphore(true)
        .descriptor_indexing(true)
        .descriptor_binding_partially_bound(true)
        .descriptor_binding_sampled_image_update_after_bind(true)
        .shader_sampled_image_array_non_uniform_indexing(true)
        .descriptor_binding_update_unused_while_pending(true)
        .descriptor_binding_variable_descriptor_count(true)
        .runtime_descriptor_array(true)
        .buffer_device_address(true);
    let demote_all = vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures::default()
        .shader_demote_to_helper_invocation(true);
    assert_eq!(first_missing_required_feature(&core_all, &v12_all, &demote_all), None);

    // 同时缺 core 与 v12 时先报 core；core 里先报排在前面的。
    let none12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut core = core_all;
    core.multi_draw_indirect = 0;
    core.sampler_anisotropy = 0;
    assert_eq!(
        first_missing_required_feature(&core, &none12, &demote_all),
        Some(("core", "sampler_anisotropy"))
    );
    assert_eq!(
        first_missing_required_feature(&core_all, &none12, &demote_all),
        Some(("PhysicalDeviceVulkan12Features", "timeline_semaphore"))
    );
    assert_eq!(
        first_missing_required_feature(&core_all, &v12_all, &Default::default()),
        Some((
            "PhysicalDeviceShaderDemoteToHelperInvocationFeatures",
            "shader_demote_to_helper_invocation"
        ))
    );
}

#[test]
fn the_default_device_is_created_with_the_original_queue_layout() {
    let Ok(graphics) = GraphicsInstance::new() else { return };
    let Ok(devices) = graphics.available_for_surface(None, Default::default()) else { return };
    match devices.create_device(&graphics) {
        Ok(device) => {
            // 原版通用队列 1 个；独立 compute 族（若有）全部队列都被取出。
            assert!(device.capabilities.runtime_descriptor_array);
            assert!(device.capabilities.buffer_device_address);
        }
        Err(errors) => panic!("默认设备创建失败：\n{}", errors.join("\n")),
    }
}
