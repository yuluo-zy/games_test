//! 并行录制：pass 分块策略与任务执行器。
//!
//! 一个 batch 的 pass 按**上一帧**各 pass 的 CPU 录制耗时切成连续块，每块一条命令缓冲，
//! 经 `bevy_tasks::ComputeTaskPool` 并行录制。未知耗时按均值估计，冷帧与轻帧同样使用任务池。
use super::executor::PassPgoStats;
use std::ops::Range;

/// 一个 pass 的上一帧耗时；`None` 表示上一帧没有该 pass 的记录。
pub type PassCost = Option<u64>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkStrategy {
    /// 至多 `max_chunks` 块，每块计入 `chunk_overhead_ns` 固定成本，
    /// 未知耗时按已知均值估计，二分预算求最小化最大块的连续切分。
    Balanced {
        max_chunks: usize,
        chunk_overhead_ns: u64,
    },
}

/// 连续切分：在块数不超过 `max_chunks` 的前提下最小化最重块。
/// `split_allowed[i]` 为 false 时禁止在 pass `i` 之后切开。
pub fn balanced_chunks(
    costs: &[PassCost],
    max_chunks: usize,
    chunk_overhead_ns: u64,
    split_allowed: &[bool],
) -> Vec<Range<usize>> {
    let n = costs.len();
    let max_chunks = max_chunks.max(1);
    if n == 0 {
        return Vec::new();
    }
    // 未知耗时按已知均值估计；完全没有数据时视为等价，按 pass 数均分。
    let (known_sum, known_count) = costs.iter().flatten().fold((0u128, 0u128), |(sum, count), &cost| (sum + u128::from(cost), count + 1));
    let fallback = if known_count == 0 { 1 } else { (known_sum / known_count).max(1) as u64 };
    let weight = |cost: &PassCost| u128::from(cost.unwrap_or(fallback).max(1));
    let chunk_overhead_ns = u128::from(chunk_overhead_ns);
    let allowed = |i: usize| split_allowed.get(i).copied().unwrap_or(true);
    // 给定预算时的贪心切分：块成本 = 固定开销 + 块内耗时；单个超预算 pass 独占一块。
    // Search only counts partitions. Materialize ranges once, at the final budget.
    let split = |budget: u128, mut output: Option<&mut Vec<Range<usize>>>| {
        let (mut start, mut load, mut count) = (0, chunk_overhead_ns, 1usize);
        for (i, cost) in costs.iter().enumerate() {
            let w = weight(cost);
            if i > start && load + w > budget && allowed(i - 1) {
                if let Some(chunks) = output.as_deref_mut() { chunks.push(start..i); }
                count += 1;
                start = i;
                load = chunk_overhead_ns;
            }
            load += w;
        }
        if let Some(chunks) = output { chunks.push(start..n); }
        count
    };
    let (mut low, mut high) = (
        chunk_overhead_ns + costs.iter().map(weight).max().unwrap_or(1),
        chunk_overhead_ns + costs.iter().map(weight).sum::<u128>(),
    );
    while low < high {
        let mid = low + (high - low) / 2;
        if split(mid, None) <= max_chunks {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let mut chunks = Vec::with_capacity(max_chunks.min(n));
    split(low, Some(&mut chunks));
    chunks
}

/// 按策略切分一个 batch；`names` 是 batch 内各 pass 的 PGO 名。
pub fn plan_chunks<N: AsRef<str>>(
    strategy: ChunkStrategy,
    names: &[N],
    previous: &PassPgoStats,
    split_allowed: &[bool],
) -> Vec<Range<usize>> {
    let costs: smallvec::SmallVec<[PassCost; 32]> = names.iter().map(|name| previous.get(name)).collect();
    match strategy {
        ChunkStrategy::Balanced {
            max_chunks,
            chunk_overhead_ns,
        } => balanced_chunks(&costs, max_chunks, chunk_overhead_ns, split_allowed),
    }
}

/// Record chunks on the ECS compute pool and return results in chunk order.
pub(crate) fn map_chunks<T, R>(
    items: &mut [T], f: impl Fn(usize, &mut T) -> R + Send + Sync,
) -> Vec<R>
where T: Send, R: Send + 'static {
    use bevy_tasks::{ComputeTaskPool, TaskPool};
    let pool = ComputeTaskPool::get_or_init(TaskPool::default);
    // Tick only this recording scope: unrelated ECS producers must not run
    // under the render owner's thread identity while it owns a frame boundary.
    let mut indexed = pool.scope_with_executor(false, None, |scope| {
        for (index, item) in items.iter_mut().enumerate() {
            let f = &f;
            scope.spawn(async move { (index, f(index, item)) });
        }
    });
    indexed.sort_unstable_by_key(|(index, _)| *index);
    indexed.into_iter().map(|(_, value)| value).collect()
}

/// 执行器的并行录制配置。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParallelRecording {
    pub strategy: ChunkStrategy,
}
impl ParallelRecording {
    /// One production worker path. PGO changes chunk weights, never execution mode.
    pub const PARALLEL: Self = Self {
        strategy: ChunkStrategy::Balanced { max_chunks: 8, chunk_overhead_ns: 20_000 },
    };
    /// 无门槛、块数不限：即使没有统计也每个 pass 一块（压力测试与基准用）。
    pub const EVERY_PASS: Self = Self {
        strategy: ChunkStrategy::Balanced {
            max_chunks: usize::MAX,
            chunk_overhead_ns: 0,
        },

    };
}
impl Default for ParallelRecording {
    fn default() -> Self {
        Self::PARALLEL
    }
}
