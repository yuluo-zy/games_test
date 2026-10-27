//! 设备与队列配置。

pub use graveyard::RetirementStats;
pub use transient_backing::TransientBackingCounters;
pub use stats::{RenderStats, StatsSnapshot};
pub use memory::GpuMemoryUsage;
#[cfg(feature = "cpu-probe")]
mod texture_upload_probe;
#[cfg(feature = "cpu-probe")]
pub use texture_upload_probe::{TextureUploadProbeKind, TextureUploadProbeSnapshot};
internal_mod!(
    allocator,
    caps,
    debug,
    graveyard,
    host_flush,
    init,
    memory,
    policy,
    pools,
    queue_config,
    reclamation,
    selection,
    semaphore,
    stats,
    transient_backing
);
#[cfg(feature = "suballocation")]
internal_mod!(suballoc);
pub(crate) mod extensions;
mod lifetime;
pub(crate) use lifetime::{ResourceStorage, RetireTicket};
mod logical;
mod native;
mod queues;
mod submission;
pub use submission::{Signal, SubmitBatch, Wait};

pub use caps::{DeviceCaps, Feature, FeatureSet};
pub use init::{DeviceConfig, DevicePreference};
pub use logical::Device;
pub(crate) use logical::observe_vulkan;
pub use native::DeviceCapabilities;
pub use debug::DebugUtils;
pub use selection::{
    AvailableGraphicsDevice, AvailableGraphicsDeviceInitState, GraphicsInstance,
    create_headless_device, create_headless_device_configured,
};
pub use policy::{
    AllocationStrategy, DescriptorStrategy, ExecutionPolicy, PolicyPreferences, plan_policy,
};
pub use queue_config::{
    FamilyInfo, PhysicalQueue, QueueClass, QueueConfig, QueuePlan, QueuePreference, plan_queues,
};
pub use queues::DeviceQueue;
pub use reclamation::{ReclamationBudget, ReclamationConfig};
