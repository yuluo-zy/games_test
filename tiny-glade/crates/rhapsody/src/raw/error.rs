//! 兼容路径：错误类型已统一到 [`crate::error`]。
pub use crate::error::*;

/// The observed driver failure is terminal across every resource entry point.
pub(crate) fn observe_vulkan<T>(
    device: &super::device::Device,
    result: std::result::Result<T, ash::vk::Result>,
) -> crate::Result<T> {
    result.map_err(|error| {
        if error == ash::vk::Result::ERROR_DEVICE_LOST {
            device.queues.timelines.mark_device_lost();
        }
        crate::Error::from(error)
    })
}
