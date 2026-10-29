use super::*;
use ash::vk;

#[test]
fn shared_errors_compare_by_identity_not_by_value() {
    let first = SharedError::from(Error::DeviceLost);
    assert_eq!(first, first.clone());
    assert_ne!(first, SharedError::from(Error::DeviceLost));
}

#[test]
fn descriptor_pool_exhaustion_is_distinct_from_device_memory_exhaustion() {
    assert_eq!(
        Error::from(vk::Result::ERROR_OUT_OF_POOL_MEMORY).kind(),
        ErrorKind::Exhausted
    );
    assert!(matches!(
        Error::from(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY),
        Error::OutOfDeviceMemory
    ));
}

#[test]
fn vulkan_results_map_to_actionable_categories() {
    let cases = [
        (vk::Result::ERROR_DEVICE_LOST, ErrorKind::DeviceLost),
        (
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
            ErrorKind::OutOfMemory,
        ),
        (vk::Result::ERROR_OUT_OF_HOST_MEMORY, ErrorKind::OutOfMemory),
        (
            vk::Result::ERROR_OUT_OF_DATE_KHR,
            ErrorKind::SurfaceOutOfDate,
        ),
        (vk::Result::ERROR_SURFACE_LOST_KHR, ErrorKind::SurfaceLost),
        (vk::Result::TIMEOUT, ErrorKind::Timeout),
        (
            vk::Result::ERROR_EXTENSION_NOT_PRESENT,
            ErrorKind::Unsupported,
        ),
        (vk::Result::ERROR_INITIALIZATION_FAILED, ErrorKind::Vulkan),
    ];
    for (result, kind) in cases {
        assert_eq!(Error::from(result).kind(), kind, "{result:?}");
    }
}

#[test]
fn invalid_usage_keeps_its_source() {
    #[derive(Debug, thiserror::Error)]
    #[error("细节")]
    struct Detail;
    let error = Error::invalid_usage_with("越界", Detail);
    assert_eq!(error.kind(), ErrorKind::InvalidUsage);
    assert_eq!(
        std::error::Error::source(&error).unwrap().to_string(),
        "细节"
    );
}
