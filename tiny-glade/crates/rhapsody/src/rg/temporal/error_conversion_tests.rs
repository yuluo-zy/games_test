use super::*;
use crate::Error;

#[test]
fn temporal_backend_errors_preserve_driver_categories() {
    for original in [
        Error::DeviceLost,
        Error::OutOfHostMemory,
        Error::OutOfDeviceMemory,
        Error::Timeout,
        Error::unsupported("test capability"),
        Error::Vulkan(ash::vk::Result::ERROR_UNKNOWN),
    ] {
        let kind = original.kind();
        let message = original.to_string();
        let build = crate::rg::graph::BuildError::from(original);
        assert_eq!(build, build.clone());
        let actual = Error::from(TemporalError::Backend(build));
        assert_eq!(actual.kind(), kind);
        assert_eq!(actual.to_string(), message);
    }
}
