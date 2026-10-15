//! 渲染统计快照。
//!
//! 原版 `RenderStats::snapshot_and_reset`（0x140c02da0，机器码已读）：对 `[stats+0x00/+0x08/+0x10]`
//! 三个 u64 各做一次 `xchg`（原子取走并清零），再把三个值写入返回槽（`rcx`）。共 3 字段；
//! 字段语义名称未证实（生产端写入点未逐一追到），以 `counter0..2` 中性命名。
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct RenderStats {
    pub counter0: AtomicU64,
    pub counter1: AtomicU64,
    pub counter2: AtomicU64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StatsSnapshot {
    pub counter0: u64,
    pub counter1: u64,
    pub counter2: u64,
}

impl RenderStats {
    /// 0x140c02da0：3×u64 原子 xchg(0)。
    pub fn snapshot_and_reset(&self) -> StatsSnapshot {
        StatsSnapshot {
            counter0: self.counter0.swap(0, Ordering::SeqCst),
            counter1: self.counter1.swap(0, Ordering::SeqCst),
            counter2: self.counter2.swap(0, Ordering::SeqCst),
        }
    }
}
