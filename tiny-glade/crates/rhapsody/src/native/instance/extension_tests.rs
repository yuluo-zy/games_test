use super::*;

fn props(name: &str) -> vk::ExtensionProperties {
    let mut p = vk::ExtensionProperties::default();
    for (slot, b) in p.extension_name.iter_mut().zip(name.bytes()) {
        *slot = b as _;
    }
    p
}

fn native_extensions() -> Vec<vk::ExtensionProperties> {
    DEFAULT_INSTANCE_EXTENSIONS
        .iter()
        .copied()
        .chain(["VK_KHR_surface"])
        .map(props)
        .collect()
}

#[test]
fn portability_devices_are_enumerated_only_with_the_matching_extension_and_flag() {
    let mut available = native_extensions();
    let (native, flags) = plan_instance_extensions(&[], &available).unwrap();
    assert!(flags.is_empty());
    assert!(
        !native
            .iter()
            .any(|name| name.as_c_str() == ash::khr::portability_enumeration::NAME)
    );
    available.push(props("VK_KHR_portability_enumeration"));
    let (portable, flags) = plan_instance_extensions(&[], &available).unwrap();
    assert!(flags.contains(vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR));
    assert_eq!(
        portable
            .iter()
            .filter(|name| name.as_c_str() == ash::khr::portability_enumeration::NAME)
            .count(),
        1
    );
    let requested = [
        ash::khr::portability_enumeration::NAME.to_owned(),
        ash::khr::portability_enumeration::NAME.to_owned(),
    ];
    let (explicit, _) = plan_instance_extensions(&requested, &available).unwrap();
    assert_eq!(
        explicit
            .iter()
            .filter(|name| name.as_c_str() == ash::khr::portability_enumeration::NAME)
            .count(),
        1
    );
}

#[test]
fn surface_dependency_does_not_implicitly_enable_headless_wsi_maintenance() {
    let mut available = native_extensions();
    available.push(props("VK_EXT_surface_maintenance1"));
    let (headless, _) = plan_instance_extensions(&[], &available).unwrap();
    assert!(
        !headless
            .iter()
            .any(|name| name.as_c_str() == ash::ext::surface_maintenance1::NAME)
    );
    let (windowed, _) =
        plan_instance_extensions(&[ash::khr::surface::NAME.to_owned()], &available).unwrap();
    assert!(
        windowed
            .iter()
            .any(|name| name.as_c_str() == ash::ext::surface_maintenance1::NAME)
    );
}

#[test]
fn instance_planning_rejects_missing_requested_extensions_and_surface_dependency() {
    let missing_surface: Vec<_> = DEFAULT_INSTANCE_EXTENSIONS
        .iter()
        .copied()
        .map(props)
        .collect();
    assert_eq!(
        plan_instance_extensions(&[], &missing_surface)
            .unwrap_err()
            .kind(),
        crate::ErrorKind::Unsupported
    );
    let missing_portability = [ash::khr::portability_enumeration::NAME.to_owned()];
    assert_eq!(
        plan_instance_extensions(&missing_portability, &native_extensions())
            .unwrap_err()
            .kind(),
        crate::ErrorKind::Unsupported
    );
}

#[test]
fn defaults_first_then_requested_deduped_and_layer_settings_only_when_available() {
    let requested = [
        CString::new("VK_EXT_debug_utils").unwrap(),
        CString::new(DEFAULT_INSTANCE_EXTENSIONS[0]).unwrap(),
    ];
    let names = |v: &[CString]| {
        v.iter()
            .map(|c| c.to_str().unwrap().to_owned())
            .collect::<Vec<_>>()
    };
    let without = effective_extensions(&requested, &[]);
    assert_eq!(
        names(&without),
        [
            "VK_KHR_get_physical_device_properties2",
            "VK_KHR_get_surface_capabilities2",
            "VK_EXT_debug_utils",
            "VK_KHR_surface"
        ]
    );
    let with = effective_extensions(&requested, &[props("VK_EXT_layer_settings")]);
    assert_eq!(names(&with).last().unwrap(), "VK_EXT_layer_settings");
    // 前缀相同但更长的名字不算命中（原版按整个 22 字节含 NUL 比较）。
    assert_eq!(
        effective_extensions(&[], &[props("VK_EXT_layer_settings2")]).len(),
        3
    );
}
