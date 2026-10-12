//! 原已知策略与候选多设备consumer验收；不将候选排序当成原 comparator。
use ash::vk;
use rhapsody::raw::{
    init::{
        AvailableGraphicsDevice, DeviceDecision, DeviceMemoryMetrics, DeviceSelectionPolicy,
        DriverSupport, GraphicsDevicesForSurface, GraphicsInstanceLayer,
        GraphicsInstanceLayerAllowlist, GraphicsInstanceLayerAllowlistEntry,
    },
    physical_device::{DeviceDriverVersion, DriverVendor, HeapBudget, PerformanceInfo},
};
pub fn candidate(
    index: usize,
    name: &str,
    vendor: DriverVendor,
    components: [u32; 4],
) -> AvailableGraphicsDevice {
    AvailableGraphicsDevice {
        index,
        handle: index as u64 + 1,
        name: name.into(),
        device_type: vk::PhysicalDeviceType::DISCRETE_GPU,
        driver: DeviceDriverVersion::from_parts(vendor, components),
        memory: PerformanceInfo {
            device_local_bytes: 4_000_000_000,
            host_bytes: 8_000_000_000,
            device_local_budget: Some(3_000_000_000),
            host_budget: Some(6_000_000_000),
            unified_memory: false,
            heaps: vec![HeapBudget {
                index: 0,
                device_local: true,
                size: 4_000_000_000,
                budget: Some(3_000_000_000),
                usage: Some(1_000_000_000),
            }],
            vendor_specific: None,
        },
        original_memory_metrics: Some(DeviceMemoryMetrics {
            local: 1_000_000_000,
            host: 400_000_000,
            shared: false,
        }),
        api_version: vk::API_VERSION_1_2,
        timeline_semaphore: true,
        graphics_families: vec![0],
        present_families: Some(vec![0]),
        extensions: vec!["VK_KHR_swapchain".into()],
        errors: vec![],
    }
}
#[test]
fn real_original_ron_migrations_append_and_keep_exact_layer_vendor_semantics() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../ljxsj_92385/Tiny Glade/assets/vulkan_allowlist_defaults.ron"
    ));
    let mut allowlist = GraphicsInstanceLayerAllowlist::default();
    allowlist.update_from_ron(input).unwrap();
    assert_eq!(allowlist.version, 2);
    assert_eq!(allowlist.entries.len(), 3);
    assert!(allowlist.contains_vendor("VALVE") && allowlist.contains_vendor("OBS"));
    assert!(!allowlist.contains_vendor("Valve"));
    assert!(allowlist.contains_layer("VK_LAYER_reshade"));
    let valve = GraphicsInstanceLayer {
        name: "VK_LAYER_VALVE_steam_overlay".into(),
        description: String::new(),
    };
    assert_eq!(valve.vendor(), Some("VALVE"));
    assert!(allowlist.permits(&valve));
    let reshade = GraphicsInstanceLayer {
        name: "VK_LAYER_reshade".into(),
        description: String::new(),
    };
    assert_eq!(reshade.vendor(), None);
    assert!(allowlist.permits(&reshade));
    let validation = GraphicsInstanceLayer {
        name: "VK_LAYER_KHRONOS_validation".into(),
        description: String::new(),
    };
    assert!(!allowlist.permits(&validation));
    let mut existing = GraphicsInstanceLayerAllowlist {
        version: 1,
        entries: vec![GraphicsInstanceLayerAllowlistEntry::Vendor("USER".into())],
    };
    existing.update_from_ron(input).unwrap();
    assert_eq!(existing.entries.len(), 2);
    assert!(existing.contains_vendor("USER"));
    assert!(!existing.contains_vendor("VALVE"));
    let snapshot = existing.entries.clone();
    existing.update_from_ron(input).unwrap();
    assert_eq!(snapshot, existing.entries);
}
#[test]
fn original_driver_thresholds_old_intel_and_known_nvidia_issue_are_preserved() {
    let old = candidate(
        0,
        "Intel(R) UHD Graphics 630",
        DriverVendor::IntelWindows,
        [101, 2135, 0, 0],
    );
    assert!(old.is_old_intel());
    assert_eq!(old.driver_support(), DriverSupport::Supported);
    assert_eq!(old.is_compatible(), DeviceDecision::Yes);
    assert!(matches!(old.is_supported(), DeviceDecision::No(_)));
    let xe = candidate(
        1,
        "Intel(R) Iris(R) Xe Graphics",
        DriverVendor::IntelWindows,
        [101, 5522, 0, 0],
    );
    assert!(!xe.is_old_intel());
    assert_eq!(xe.is_supported(), DeviceDecision::Yes);
    let bad = candidate(
        2,
        "NVIDIA GeForce RTX 5090",
        DriverVendor::Nvidia,
        [580, 88, 0, 0],
    );
    assert_eq!(
        bad.driver_support(),
        DriverSupport::KnownIssue {
            recommended: "580.97"
        }
    );
    assert_eq!(
        candidate(
            3,
            "NVIDIA GeForce GTX 1080",
            DriverVendor::Nvidia,
            [580, 88, 0, 0]
        )
        .driver_support(),
        DriverSupport::Supported
    );
    assert_eq!(
        candidate(4, "AMD", DriverVendor::Amd, [2, 0, 178, 0]).driver_support(),
        DriverSupport::BelowMinimum {
            recommended: "21.3.1",
            old_intel: false
        }
    );
    assert_eq!(
        candidate(5, "AMD", DriverVendor::Amd, [2, 0, 179, 0]).driver_support(),
        DriverSupport::Supported
    );
}
#[test]
fn original_memory_thresholds_separate_shared_and_missing_budget_differ() {
    let mut d = candidate(0, "NVIDIA", DriverVendor::Nvidia, [550, 0, 0, 0]);
    d.original_memory_metrics = Some(DeviceMemoryMetrics {
        local: 999_999_999,
        host: 4_000_000_000,
        shared: false,
    });
    assert!(matches!(d.is_compatible(), DeviceDecision::No(_)));
    d.original_memory_metrics = Some(DeviceMemoryMetrics {
        local: 999_999_999,
        host: 400_000_001,
        shared: true,
    });
    assert_eq!(d.is_compatible(), DeviceDecision::Yes);
    d.original_memory_metrics = None;
    assert!(matches!(d.is_compatible(), DeviceDecision::Unknown(_)));
    d.driver.encoding_known = false;
    assert_eq!(d.driver_support(), DriverSupport::Unknown);
}
#[test]
fn surface_selection_reports_missing_reasons_and_candidate_order() {
    let mut integrated = candidate(0, "Intel Xe", DriverVendor::IntelWindows, [101, 5522, 0, 0]);
    integrated.device_type = vk::PhysicalDeviceType::INTEGRATED_GPU;
    let discrete = candidate(1, "RTX 4070", DriverVendor::Nvidia, [550, 0, 0, 0]);
    let mut missing = candidate(2, "missing", DriverVendor::Nvidia, [550, 0, 0, 0]);
    missing.present_families = Some(vec![1]);
    missing.timeline_semaphore = false;
    missing.extensions.clear();
    let emulated = candidate(
        3,
        "Microsoft Basic Render Driver",
        DriverVendor::Unknown,
        [0; 4],
    );
    let selected = GraphicsDevicesForSurface::from_candidates(
        vec![integrated, discrete, missing, emulated],
        DeviceSelectionPolicy {
            requires_surface: true,
            required_extensions: vec!["VK_KHR_swapchain".into()],
            ..Default::default()
        },
    );
    assert_eq!(selected.available, vec![1, 0]);
    assert_eq!(selected.missing_devices().len(), 2);
    let reasons = &selected.missing_devices()[0].1;
    assert!(reasons.iter().any(|r| r.contains("timeline")));
    assert!(reasons.iter().any(|r| r.contains("presentation")));
    assert!(reasons.iter().any(|r| r.contains("swapchain")));
}
#[test]
fn strict_unknown_policy_never_turns_unknown_into_supported() {
    let d = candidate(0, "unclassified", DriverVendor::Unknown, [0; 4]);
    assert!(matches!(d.is_supported(), DeviceDecision::Unknown(_)));
    let list = GraphicsDevicesForSurface::from_candidates(
        vec![d],
        DeviceSelectionPolicy {
            allow_unknown_original_policy: false,
            ..Default::default()
        },
    );
    assert!(list.available_devices().next().is_none());
    assert_eq!(list.missing_devices().len(), 1);
}
