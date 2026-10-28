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
