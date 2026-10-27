//! 渲染统计快照。
//!
//! 原版 `RenderStats::snapshot_and_reset`（0x140c02da0，机器码已读）：对 `[stats+0x00/+0x08/+0x10]`
//! 三个 u64 各做一次 `xchg`（原子取走并清零），再把三个值写入返回槽（`rcx`）。共 3 字段；
//! 字段语义名称未证实（生产端写入点未逐一追到），以 `counter0..2` 中性命名。
use std::sync::atomic::Ordering;

crate::collections::atomic_snapshot::atomic_snapshot! {
    pub RenderStats, StatsSnapshot { pub counter0, pub counter1, pub counter2 }
    snapshot_and_reset => swap(0, Ordering::SeqCst);
}
