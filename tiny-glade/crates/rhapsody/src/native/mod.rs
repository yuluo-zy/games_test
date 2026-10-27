//! Native Vulkan wrappers that know nothing about the logical device, resources or the
//! render graph: instances and their layers, physical-device queries, access-scope tables,
//! owned submission arrays, DXGI adapter probing and the Aftermath crash-dump hook.
//!
//! Native objects created through the public items here are outside this crate's
//! synchronization, retirement and memory accounting; the caller owns their lifetime.
//! For the logical device see [`Device::raw_device`](crate::device::Device::raw_device),
//! [`Device::instance`](crate::device::Device::instance) and
//! [`Device::primary_queue`](crate::device::Device::primary_queue).

pub mod instance;
pub mod instance_layers;
pub mod physical_device;
pub mod surface;
internal_mod!(access_table, aftermath, dxgi, resource_usage, submit);

pub use access_table::{AccessScope, AccessSpec, is_write};
pub use instance::{
    DebugMessage, GraphicsInstanceLoader, InstanceBuilder, KHRONOS_VALIDATION_LAYER,
    ValidationMessenger, VulkanInstance,
};
pub use instance_layers::{GraphicsInstanceLayerAllowlist, GraphicsInstanceLayerAllowlistEntry};
pub use physical_device::{PhysicalDevice, enumerate_physical_devices};
pub use surface::Surface;
