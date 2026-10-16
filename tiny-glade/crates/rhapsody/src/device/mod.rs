//! 设备与队列配置。
pub mod queue_config;

pub use queue_config::{
    FamilyInfo, PhysicalQueue, QueueClass, QueueConfig, QueuePlan, QueuePreference, plan_queues,
};
