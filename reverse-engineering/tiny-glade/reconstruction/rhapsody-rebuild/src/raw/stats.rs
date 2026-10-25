//! 渲染统计快照。字段是候选统计接口，原作字段名字和完整布局尚待核对。
use std::sync::atomic::{AtomicU64, Ordering};
#[derive(Default)]
pub struct RenderStats {
    pub draws: AtomicU64,
    pub dispatches: AtomicU64,
    pub submitted_commands: AtomicU64,
    pub allocated_bytes: AtomicU64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StatsSnapshot {
    pub draws: u64,
    pub dispatches: u64,
    pub submitted_commands: u64,
    pub allocated_bytes: u64,
}
impl RenderStats {
    pub fn snapshot_and_reset(&self) -> StatsSnapshot {
        StatsSnapshot {
            draws: self.draws.swap(0, Ordering::Relaxed),
            dispatches: self.dispatches.swap(0, Ordering::Relaxed),
            submitted_commands: self.submitted_commands.swap(0, Ordering::Relaxed),
            allocated_bytes: self.allocated_bytes.swap(0, Ordering::Relaxed),
        }
    }
}
