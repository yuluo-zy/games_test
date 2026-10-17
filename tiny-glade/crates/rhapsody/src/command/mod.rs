//! 命令录制与 CPU 临时内存基础设施。
//!
//! CPU arena 依赖 CPU lease；真实帧槽及其 Vulkan 命令池由图后端管理，
//! 只有相关队列全部完成后才能复用。

pub mod cpu_arena;
pub mod recording;

pub use cpu_arena::{CpuArenaConfig, CpuArenaLease, CpuArenaPool, CpuArenaStats};
