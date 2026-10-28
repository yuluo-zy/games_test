//! 并行录制：pass 分块策略与任务执行器。
//!
//! 一个 batch 的 pass 按**上一帧**各 pass 的 CPU 录制耗时切成连续块，每块一条命令缓冲，
//! 多块时经 `bevy_tasks::ComputeTaskPool` 并行录制；只有一块时直接在调用线程录制。
//!
//! 进入线程池本身有固定成本（`tests/parallel_dispatch_perf.rs` 实测：空任务约 27 µs，
//! 之后每多一块约 4 µs），所以 [`ChunkStrategy::Adaptive`] 只在预计收益超过该成本时才切块。
use super::executor::PassPgoStats;
use crate::resource::ResourceName;
use smallvec::SmallVec;
use std::ops::Range;

/// 一个 pass 的上一帧耗时；`None` 表示上一帧没有该 pass 的记录。
pub type PassCost = Option<u64>;

/// 一个 batch 的连续块。八块以内全部内联，覆盖生产环境的所有 batch。
pub type Chunks = SmallVec<[Range<usize>; 8]>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkStrategy {
    /// 至多 `max_chunks` 块，每块计入 `chunk_overhead_ns` 固定成本，
    /// 未知耗时按已知均值估计，二分预算求最小化最大块的连续切分。
    /// 不考虑线程池调度代价：块数总是尽可能多（压力测试与基准用）。
    Balanced {
        max_chunks: usize,
        chunk_overhead_ns: u64,
    },
    /// 在 [`Self::Balanced`] 的切分上加入调度代价模型：
    /// 并行耗时估计为 `dispatch_overhead_ns + chunk_overhead_ns * 块数 + 最重块耗时`，
    /// 不比串行（单块、调用线程内联）明显更快时就不切。没有上一帧数据时同样不切。
    Adaptive {
        max_chunks: usize,
        /// 每多一块在调用线程上增加的成本。
        chunk_overhead_ns: u64,
        /// 进入线程池并等待它的固定成本。
        dispatch_overhead_ns: u64,
    },
}

/// 未知耗时的估计值：已知耗时的均值；完全没有数据时视为等价。
fn fallback_cost(costs: &[PassCost]) -> u64 {
    let (sum, count) = costs
        .iter()
        .flatten()
        .fold((0u128, 0u128), |(sum, count), &cost| (sum + u128::from(cost), count + 1));
    if count == 0 { 1 } else { (sum / count).max(1) as u64 }
}

fn balanced_into(
    costs: &[PassCost],
    max_chunks: usize,
    chunk_overhead_ns: u64,
    split_allowed: &[bool],
    chunks: &mut Chunks,
) {
    chunks.clear();
    let n = costs.len();
    let max_chunks = max_chunks.max(1);
    if n == 0 {
        return;
    }
    let fallback = fallback_cost(costs);
    let weight = |cost: &PassCost| u128::from(cost.unwrap_or(fallback).max(1));
    let chunk_overhead_ns = u128::from(chunk_overhead_ns);
    let allowed = |i: usize| split_allowed.get(i).copied().unwrap_or(true);
    // 给定预算时的贪心切分：块成本 = 固定开销 + 块内耗时；单个超预算 pass 独占一块。
    // Search only counts partitions. Materialize ranges once, at the final budget.
    let split = |budget: u128, mut output: Option<&mut Chunks>| {
        let (mut start, mut load, mut count) = (0, chunk_overhead_ns, 1usize);
        for (i, cost) in costs.iter().enumerate() {
            let w = weight(cost);
            if i > start && load + w > budget && allowed(i - 1) {
                if let Some(chunks) = output.as_deref_mut() {
                    chunks.push(start..i);
                }
                count += 1;
                start = i;
                load = chunk_overhead_ns;
            }
            load += w;
        }
        if let Some(chunks) = output {
            chunks.push(start..n);
        }
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
    split(low, Some(chunks));
}

/// 连续切分：在块数不超过 `max_chunks` 的前提下最小化最重块。
/// `split_allowed[i]` 为 false 时禁止在 pass `i` 之后切开。
pub fn balanced_chunks(
    costs: &[PassCost],
    max_chunks: usize,
    chunk_overhead_ns: u64,
    split_allowed: &[bool],
) -> Vec<Range<usize>> {
    let mut chunks = Chunks::new();
    balanced_into(costs, max_chunks, chunk_overhead_ns, split_allowed, &mut chunks);
    chunks.into_vec()
}

fn adaptive_into(
    costs: &[PassCost],
    max_chunks: usize,
    chunk_overhead_ns: u64,
    dispatch_overhead_ns: u64,
    split_allowed: &[bool],
    chunks: &mut Chunks,
) {
    chunks.clear();
    let n = costs.len();
    if n == 0 {
        return;
    }
    let fallback = fallback_cost(costs);
    let weight = |cost: &PassCost| cost.unwrap_or(fallback).max(1);
    let total = costs.iter().map(weight).fold(0u64, u64::saturating_add);
    let limit = max_chunks.clamp(1, n);
    // `k` chunks cost about `chunk_overhead * k + total / k`, minimal at `sqrt(total / overhead)`.
    let ideal = match chunk_overhead_ns {
        0 => limit,
        overhead => ((total as f64 / overhead as f64).sqrt().round() as usize).clamp(1, limit),
    };
    let known = costs.iter().any(Option::is_some);
    // Without statistics, or when even a perfect split could not repay entering the pool,
    // record the batch inline as one chunk.
    if !known || ideal < 2 || total <= dispatch_overhead_ns {
        chunks.push(0..n);
        return;
    }
    balanced_into(costs, ideal, chunk_overhead_ns, split_allowed, chunks);
    let heaviest = chunks
        .iter()
        .map(|chunk| costs[chunk.clone()].iter().map(weight).fold(0u64, u64::saturating_add))
        .max()
        .unwrap_or(0);
    let pooled = dispatch_overhead_ns
        .saturating_add(chunk_overhead_ns.saturating_mul(chunks.len() as u64))
        .saturating_add(heaviest);
    // Pool latency varies with the host and its load: demand a clear win (>10%).
    if chunks.len() < 2 || pooled.saturating_mul(10) >= total.saturating_mul(9) {
        chunks.clear();
        chunks.push(0..n);
    }
}

fn plan_into(strategy: ChunkStrategy, costs: &[PassCost], split_allowed: &[bool], chunks: &mut Chunks) {
    match strategy {
        ChunkStrategy::Balanced {
            max_chunks,
            chunk_overhead_ns,
        } => balanced_into(costs, max_chunks, chunk_overhead_ns, split_allowed, chunks),
        ChunkStrategy::Adaptive {
            max_chunks,
            chunk_overhead_ns,
            dispatch_overhead_ns,
        } => adaptive_into(
            costs,
            max_chunks,
            chunk_overhead_ns,
            dispatch_overhead_ns,
            split_allowed,
            chunks,
        ),
    }
}

/// 按策略切分一个 batch；`names` 是 batch 内各 pass 的 PGO 名。
pub fn plan_chunks<N: AsRef<str>>(
    strategy: ChunkStrategy,
    names: &[N],
    previous: &PassPgoStats,
    split_allowed: &[bool],
) -> Vec<Range<usize>> {
    let costs: SmallVec<[PassCost; 32]> = names.iter().map(|name| previous.get(name)).collect();
    let mut chunks = Chunks::new();
    plan_into(strategy, &costs, split_allowed, &mut chunks);
    chunks.into_vec()
}

/// [`plan_chunks`] for the executor's hot path: the pass names are already interned, so the
/// previous frame is read by identity (no string hashing or global intern lookup per pass),
/// and `costs` and `chunks` are caller-owned buffers reused across batches and frames.
pub fn plan_chunks_into(
    strategy: ChunkStrategy,
    names: &[&ResourceName],
    previous: &PassPgoStats,
    split_allowed: &[bool],
    costs: &mut Vec<PassCost>,
    chunks: &mut Chunks,
) {
    costs.clear();
    costs.extend(names.iter().map(|name| previous.get_interned(name)));
    plan_into(strategy, costs, split_allowed, chunks);
}

/// Record chunks on the ECS compute pool and return results in chunk order.
///
/// A single chunk runs on the calling thread: entering the pool costs tens of
/// microseconds even for an empty task, which would otherwise dominate light batches.
pub fn map_chunks<T, R>(
    items: &mut [T], f: impl Fn(usize, &mut T) -> R + Send + Sync,
) -> smallvec::SmallVec<[R; 8]>
where T: Send, R: Send + 'static {
    use bevy_tasks::{ComputeTaskPool, TaskPool};
    if let [only] = items {
        return smallvec::smallvec![f(0, only)];
    }
    let pool = ComputeTaskPool::get_or_init(TaskPool::default);
    let mut results = smallvec::SmallVec::<[Option<R>; 8]>::new();
    results.resize_with(items.len(), || None);
    // Each worker owns its disjoint output slot. Scope completion joins every
    // worker, preserving input order without a result Vec or an index sort.
    // Tick only this recording scope: unrelated ECS producers must not run
    // under the render owner's thread identity while it owns a frame boundary.
    let _ = pool.scope_with_executor(false, None, |scope| {
        for ((index, item), result) in items.iter_mut().enumerate().zip(&mut results) {
            let f = &f;
            scope.spawn(async move { *result = Some(f(index, item)); });
        }
    });
    results.into_iter().map(|result| result.expect("joined chunk worker returned no result")).collect()
}

#[cfg(test)]
mod chunk_results_tests;

#[cfg(test)]
mod adaptive_tests;

/// 执行器的并行录制配置。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParallelRecording {
    pub strategy: ChunkStrategy,
}
impl ParallelRecording {
    /// One production worker path. PGO changes chunk weights; light or unmeasured batches
    /// record inline on the calling thread instead of paying for a pool dispatch.
    pub const PARALLEL: Self = Self {
        strategy: ChunkStrategy::Adaptive {
            max_chunks: 8,
            chunk_overhead_ns: 4_000,
            dispatch_overhead_ns: 27_000,
        },
    };
    /// The former production policy: every batch is dispatched to the pool, split into as
    /// many chunks as the weights allow (at most eight), even when the work is tiny.
    /// For callers that need worker recording regardless of cost, such as thread-safety
    /// checks; it pays the pool dispatch latency (~27 µs) on every batch.
    pub const ALWAYS_PARALLEL: Self = Self {
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
