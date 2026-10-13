//! 并行录制：pass 分块策略与任务执行器。
//!
//! 原作 `CompiledRenderGraph::record_batch`（RVA 0x2c1d40）按**上一帧**各 pass 的 CPU 录制耗时
//! 把一个 batch 的 pass 切成变长连续块，经 `bevy_tasks::ComputeTaskPool` 的
//! `par_chunk_map_mut` 并行录制，每块一条命令缓冲。证据见
//! `evidence/render-recovery/parallel-recording/FINDINGS.md`。
//!
//! [`ChunkStrategy::Original`] 逐条复刻原作的预算与贪心切分；[`ChunkStrategy::Balanced`]
//! 是候选改进（块数上限、未知耗时按均值估计、最小化最大块），默认不启用。
use super::executor::{PassPgoNameId, PassPgoStats};
use std::ops::Range;

/// 一个 pass 的上一帧耗时；`None` 表示上一帧没有该 pass 的记录。
pub type PassCost = Option<u64>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkStrategy {
    /// 不分块，整个 batch 一条命令缓冲（原作并行开关关闭时的串行路径 0x1402c3240）。
    Serial,
    /// 原作算法：`budget = clamp(max, total/16, total/2)`，贪心累加到预算即切块。
    Original,
    /// 候选改进：至多 `max_chunks` 块，每块计入 `chunk_overhead_ns` 固定成本，
    /// 未知耗时按已知均值估计，二分预算求最小化最大块的连续切分。
    Balanced {
        max_chunks: usize,
        chunk_overhead_ns: u64,
    },
}

/// 原作预算：`total/16 > total/2` 只在溢出边界成立，原作此时 panic，这里同样以 `clamp` 断言。
pub fn original_budget(total_ns: u64, max_ns: u64) -> u64 {
    max_ns.clamp(total_ns / 16, total_ns / 2)
}

/// 原作贪心切分（0x1402c22a0..0x1402c22e6）：缺失耗时记 0；累计 `>= budget` 时把当前 pass
/// 收进本块并关闭。`split_allowed[i]` 为 false 时禁止在 pass `i` 之后切开（候选约束，原作未见）。
pub fn original_chunks(costs: &[PassCost], budget: u64, split_allowed: &[bool]) -> Vec<Range<usize>> {
    let mut chunks = Vec::new();
    let (mut start, mut accumulated) = (0, 0u64);
    for (index, cost) in costs.iter().enumerate() {
        accumulated = accumulated.saturating_add(cost.unwrap_or(0));
        let allowed = split_allowed.get(index).copied().unwrap_or(true);
        if accumulated >= budget && allowed {
            chunks.push(start..index + 1);
            start = index + 1;
            accumulated = 0;
        }
    }
    if start < costs.len() {
        chunks.push(start..costs.len());
    }
    chunks
}

/// 候选改进的连续切分：在块数不超过 `max_chunks` 的前提下最小化最重块。
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
    let known: Vec<u64> = costs.iter().flatten().copied().collect();
    let fallback = if known.is_empty() {
        1
    } else {
        (known.iter().sum::<u64>() / known.len() as u64).max(1)
    };
    let weights: Vec<u64> = costs.iter().map(|c| c.unwrap_or(fallback).max(1)).collect();
    let allowed = |i: usize| split_allowed.get(i).copied().unwrap_or(true);
    // 给定预算时的贪心切分：块成本 = 固定开销 + 块内耗时；单个超预算 pass 独占一块。
    let split = |budget: u64| {
        let mut chunks = Vec::new();
        let (mut start, mut load) = (0, chunk_overhead_ns);
        for (i, &w) in weights.iter().enumerate() {
            if i > start && load + w > budget && allowed(i - 1) {
                chunks.push(start..i);
                start = i;
                load = chunk_overhead_ns;
            }
            load += w;
        }
        chunks.push(start..n);
        chunks
    };
    let (mut low, mut high) = (
        chunk_overhead_ns + weights.iter().copied().max().unwrap_or(1),
        chunk_overhead_ns + weights.iter().sum::<u64>(),
    );
    while low < high {
        let mid = low + (high - low) / 2;
        if split(mid).len() <= max_chunks {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    split(low)
}

/// 按策略切分一个 batch；`names` 是 batch 内各 pass 的 PGO 名。
pub fn plan_chunks(
    strategy: ChunkStrategy,
    names: &[PassPgoNameId],
    previous: &PassPgoStats,
    split_allowed: &[bool],
) -> Vec<Range<usize>> {
    let costs: Vec<PassCost> = names.iter().map(|&name| previous.get(name)).collect();
    match strategy {
        ChunkStrategy::Serial => std::iter::once(0..names.len()).collect(),
        ChunkStrategy::Original => original_chunks(
            &costs,
            original_budget(previous.total_ns(), previous.max_ns()),
            split_allowed,
        ),
        ChunkStrategy::Balanced {
            max_chunks,
            chunk_overhead_ns,
        } => balanced_chunks(&costs, max_chunks, chunk_overhead_ns, split_allowed),
    }
}

/// 块的执行方式。`ComputePool` 与原作一致，使用 `bevy_tasks` 的全局计算池（与 Bevy ECS 共用，
/// 不另起线程池抢核）；`Inline` 在当前线程按块顺序执行，用于确定性测试与对照。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskRunner {
    Inline,
    ComputePool,
}
impl TaskRunner {
    /// 对每个块调用 `f(块序号, 块)`，结果按块顺序返回。
    pub fn map_chunks<T, R>(self, mut items: &mut [T], f: impl Fn(usize, &mut T) -> R + Send + Sync) -> Vec<R>
    where
        T: Send,
        R: Send + 'static,
    {
        match self {
            Self::Inline => items.iter_mut().enumerate().map(|(i, item)| f(i, item)).collect(),
            Self::ComputePool => {
                use bevy_tasks::{ComputeTaskPool, ParallelSliceMut, TaskPool};
                let pool = ComputeTaskPool::get_or_init(TaskPool::default);
                // 原作同样以“块的切片”为元素、每个元素一个任务；bevy 的 scope 按 spawn 顺序返回结果。
                items.par_chunk_map_mut(pool, 1, |index, chunk| f(index, &mut chunk[0]))
            }
        }
    }
}

/// 执行器的并行录制配置。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParallelRecording {
    pub strategy: ChunkStrategy,
    pub runner: TaskRunner,
    /// 候选门槛（原作没有）：上一帧所有 pass 的录制总耗时低于该值时整帧走串行。
    /// 0 表示不设门槛（原作行为）。轻帧的固定开销（命令池、任务调度）会超过并行收益。
    pub min_total_ns: u64,
}
impl ParallelRecording {
    /// 串行：与旧行为、原作开关关闭时一致。
    pub const SERIAL: Self = Self {
        strategy: ChunkStrategy::Serial,
        runner: TaskRunner::Inline,
        min_total_ns: 0,
    };
    /// 原作行为：原分块算法 + 全局计算池，无门槛。
    pub const ORIGINAL: Self = Self {
        strategy: ChunkStrategy::Original,
        runner: TaskRunner::ComputePool,
        min_total_ns: 0,
    };
    /// 原分块算法 + 轻帧退回串行。门槛取自实测：每 pass 约 2µs 时并行与串行持平，
    /// 64 个 pass 即约 128µs；取 200µs 留出余量。第一帧没有统计，也走串行。
    pub const ADAPTIVE: Self = Self {
        strategy: ChunkStrategy::Original,
        runner: TaskRunner::ComputePool,
        min_total_ns: 200_000,
    };
}
impl Default for ParallelRecording {
    fn default() -> Self {
        Self::SERIAL
    }
}
