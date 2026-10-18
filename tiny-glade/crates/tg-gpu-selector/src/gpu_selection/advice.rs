//! Driver advice uses borrowed version data and static vendor links.
use super::{DriverSupport, Vendor};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Static download destination and display label; opening it is a consumer action.
pub struct DriverLink {
    pub label: &'static str,
    pub url: &'static str,
}
/// Return the generic vendor destinations without allocation; unknown vendors have no links.
/// Legacy Intel overrides are applied by `device_driver_advice`, not this vendor-only lookup.
pub fn get_vendor_driver_update_links(vendor: Vendor) -> &'static [DriverLink] {
    match vendor {
        Vendor::Nvidia => &[DriverLink {
            label: "NVIDIA downloads",
            url: "https://www.nvidia.com/Download/index.aspx",
        }],
        Vendor::Intel => &[DriverLink {
            label: "Intel Arc, Iris Xe & 11th+ Gen Processor Graphics downloads",
            url: "https://www.intel.com/content/www/us/en/download/785597/intel-arc-iris-xe-graphics-windows.html",
        }],
        Vendor::Amd => &[DriverLink {
            label: "AMD downloads",
            url: "https://www.amd.com/en/support/download/drivers.html",
        }],
        Vendor::Qualcomm => &[DriverLink {
            label: "Qualcomm Snapdragon X driver downloads",
            url: "https://softwarecenter.qualcomm.com/catalog/item/Windows_Graphics_Driver",
        }],
        Vendor::Other | Vendor::Unknown => &[],
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Distinguish a minimum version, a known-working version and an unrecognized vendor identifier.
pub enum DriverRequirement<'a> {
    MinimumVersion(&'a str),
    KnownGoodVersion(&'a str),
    VendorId(u32),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Borrowed driver display data. Localization keys are resolved only by the eventual view.
pub struct DriverAdvice<'a> {
    pub version: &'a str,
    pub key: &'static str,
    pub requirement: Option<DriverRequirement<'a>>,
    pub links: &'static [DriverLink],
}
/// Project the supplied support assessment into display data without copying version strings.
/// The result borrows `version`/`support`; neither argument is parsed or independently verified.
pub fn device_driver_advice<'a>(
    vendor: Vendor,
    version: &'a str,
    support: &'a DriverSupport,
) -> DriverAdvice<'a> {
    let vendor_links = get_vendor_driver_update_links(vendor);
    let (key, requirement, links) = match support {
        DriverSupport::Unsupported { minimum } => (
            "Driver.UnsupportedDriver",
            Some(DriverRequirement::MinimumVersion(minimum)),
            vendor_links,
        ),
        DriverSupport::UnsupportedLegacyIntel { minimum } => (
            "Driver.UnsupportedDriver",
            Some(DriverRequirement::MinimumVersion(minimum)),
            &[DriverLink {
                label: "Intel 7th-10th Gen Processor Graphics downloads",
                url: "https://www.intel.com/content/www/us/en/download/776137/intel-7th-10th-gen-processor-graphics-windows.html",
            }][..],
        ),
        DriverSupport::Emulated => ("Driver.EmulatedDriver", None, &[][..]),
        DriverSupport::UnknownVendor(vendor_id) => (
            "Driver.UnknownVendor",
            Some(DriverRequirement::VendorId(*vendor_id)),
            &[][..],
        ),
        DriverSupport::KnownBad { known_good } => (
            "Driver.KnownBad",
            Some(DriverRequirement::KnownGoodVersion(known_good)),
            vendor_links,
        ),
        DriverSupport::Supported => ("Driver.SupportedDriver", None, &[][..]),
    };
    DriverAdvice {
        version,
        key,
        requirement,
        links,
    }
}
