//! Explicit escape hatches for hosts that integrate with other Vulkan code.
//!
//! These types expose native handles (`ash::Instance`, debug messengers,
//! physical-device queries). Native objects created through them are outside
//! this crate's synchronization, retirement and memory accounting; the caller
//! owns their lifetime. For the logical device see
//! [`Device::raw_device`](crate::device::Device::raw_device),
//! [`Device::instance`](crate::device::Device::instance) and
//! [`Device::primary_queue`](crate::device::Device::primary_queue).
pub use crate::device::DeviceQueue;
pub use crate::raw::debug::DebugUtils;
pub use crate::raw::init::{
    AvailableGraphicsDevice, AvailableGraphicsDeviceInitState, GraphicsInstance,
    GraphicsInstanceLayerAllowlist, GraphicsInstanceLayerAllowlistEntry, create_headless_device,
    create_headless_device_configured,
};
pub use crate::raw::instance::{
    DebugMessage, GraphicsInstanceLoader, InstanceBuilder, KHRONOS_VALIDATION_LAYER,
    ValidationMessenger, VulkanInstance,
};
pub use crate::raw::physical_device::{PhysicalDevice, enumerate_physical_devices};
