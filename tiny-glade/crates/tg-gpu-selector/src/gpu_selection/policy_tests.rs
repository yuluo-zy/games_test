use super::*;

#[test]
fn driver_links_follow_the_b_vendor_mapping() {
    let cases = [
        (
            Vendor::Nvidia,
            "NVIDIA downloads",
            "https://www.nvidia.com/Download/index.aspx",
        ),
        (
            Vendor::Intel,
            "Intel Arc, Iris Xe & 11th+ Gen Processor Graphics downloads",
            "https://www.intel.com/content/www/us/en/download/785597/intel-arc-iris-xe-graphics-windows.html",
        ),
        (
            Vendor::Amd,
            "AMD downloads",
            "https://www.amd.com/en/support/download/drivers.html",
        ),
        (
            Vendor::Qualcomm,
            "Qualcomm Snapdragon X driver downloads",
            "https://softwarecenter.qualcomm.com/catalog/item/Windows_Graphics_Driver",
        ),
    ];
    for (vendor, label, url) in cases {
        assert_eq!(
            get_vendor_driver_update_links(vendor),
            [DriverLink { label, url }]
        );
    }
    assert!(get_vendor_driver_update_links(Vendor::Other).is_empty());
    assert!(get_vendor_driver_update_links(Vendor::Unknown).is_empty());
}

#[test]
fn every_driver_support_class_applies_its_own_advice_override() {
    let cases = [
        (
            DriverSupport::Unsupported {
                minimum: "1.2".into(),
            },
            "Driver.UnsupportedDriver",
            Some(DriverRequirement::MinimumVersion("1.2")),
            false,
        ),
        (
            DriverSupport::UnsupportedLegacyIntel {
                minimum: "2.3".into(),
            },
            "Driver.UnsupportedDriver",
            Some(DriverRequirement::MinimumVersion("2.3")),
            false,
        ),
        (DriverSupport::Emulated, "Driver.EmulatedDriver", None, true),
        (
            DriverSupport::UnknownVendor(0x1234),
            "Driver.UnknownVendor",
            Some(DriverRequirement::VendorId(0x1234)),
            true,
        ),
        (
            DriverSupport::KnownBad {
                known_good: "3.4".into(),
            },
            "Driver.KnownBad",
            Some(DriverRequirement::KnownGoodVersion("3.4")),
            false,
        ),
        (
            DriverSupport::Supported,
            "Driver.SupportedDriver",
            None,
            true,
        ),
    ];

    for (support, key, requirement, clears_links) in cases {
        let advice = device_driver_advice(Vendor::Nvidia, "driver-version", &support);
        assert_eq!(advice.version, "driver-version");
        assert_eq!(advice.key, key);
        assert_eq!(advice.requirement, requirement);
        assert_eq!(advice.links.is_empty(), clears_links);
    }

    let support = DriverSupport::UnsupportedLegacyIntel {
        minimum: "2".into(),
    };
    let legacy = device_driver_advice(Vendor::Amd, "1", &support);
    assert_eq!(
        legacy.links[0].label,
        "Intel 7th-10th Gen Processor Graphics downloads"
    );
    assert_eq!(
        legacy.links[0].url,
        "https://www.intel.com/content/www/us/en/download/776137/intel-7th-10th-gen-processor-graphics-windows.html"
    );
}

#[test]
fn graphics_error_projection_preserves_b_tags_details_and_recommendations() {
    let cases = [
        (
            0x0b,
            "GraphicsInitError.NoCompatibleDevices",
            None,
            Some("GraphicsInitError.PleaseUpdateDrivers"),
        ),
        (
            0x0c,
            "GraphicsInitError.NoCompatibleDevices",
            None,
            Some("GraphicsInitError.PleaseUpdateDrivers"),
        ),
        (
            0x10,
            "GraphicsInitError.NoDriver",
            None,
            Some("GraphicsInitError.PleaseUpdateDrivers"),
        ),
        (
            0x11,
            "GraphicsInitError.NoDriver",
            None,
            Some("GraphicsInitError.PleaseUpdateDrivers"),
        ),
        (
            0x12,
            "GraphicsInitError.UnsupportedWindowingSystem",
            None,
            Some("GraphicsInitError.PleaseUpdateDrivers"),
        ),
        (
            0x14,
            "GraphicsInitError.SwapchainError",
            Some("original details"),
            Some("GraphicsInitError.SwapchainErrorTip"),
        ),
        (
            0x15,
            "GraphicsInitError.OutOfVideoMemory",
            Some("original details"),
            Some("GraphicsInitError.OutOfVideoMemoryRecommendation"),
        ),
        (
            0x16,
            "GraphicsInitError.InternalError",
            Some("original details"),
            Some("GraphicsInitError.PleaseUpdateDrivers"),
        ),
        (0x17, "Unexpected error", Some("original details"), None),
        (
            0x18,
            "GraphicsInitError.NoDriver",
            None,
            Some("GraphicsInitError.PleaseUpdateDrivers"),
        ),
    ];

    for (tag, heading, details, recommendation) in cases {
        assert_eq!(
            localize_graphics_init_error(&GraphicsInitError {
                tag,
                details: "original details".into(),
            }),
            GraphicsErrorView {
                heading,
                details,
                recommendation,
            },
            "unexpected projection for graphics error tag {tag:#x}"
        );
    }
}
