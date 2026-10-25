//! Only DeviceConfig negotiates production device creation.
mod gpu_test_support;
use rhapsody::device::{Device, DeviceConfig, Feature};
#[test]
fn configured_creation_enables_the_required_rendering_features() {
    let Some(device) = gpu_test_support::initialize("configured device creation", Device::new_headless(&DeviceConfig::headless())) else { return; };
    for feature in [Feature::TimelineSemaphore, Feature::DynamicRendering, Feature::Synchronization2] {
        assert!(device.caps().enabled().contains(feature));
    }
}
