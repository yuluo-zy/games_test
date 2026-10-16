//! 命令录制与帧环基础设施（P3）。
//!
//! 目前只落地不依赖 Vulkan 的纯 CPU 部分：帧环 [`FrameRing`]。命令池的「帧 × worker ×
//! 队列族」整池复位与 CPU arena 在后续接线时补齐。

pub mod frame_ring;

pub use frame_ring::{FrameRing, FrameRingError, FrameSlotState};
