//! Graph recording and submission using typed CPU identities and actual queue completion points.
use super::transaction::FrameTransaction;
use crate::command::recording;
use crate::command::CpuArenaLease;
use crate::rg::recorder as record;
use ash::vk;
use std::collections::BTreeMap;
mod error;
mod backend_api;
mod pass_context;
mod pgo;
mod entry;
mod batch_recording;
mod completion;
pub use error::Error;
pub use backend_api::{GraphTimelineWait, GraphBatchSync, ChunkTransition, ChunkLease, ChunkRecording, ChunkRecorder, QueuePrelude, ExecutionBackend};
pub use pass_context::{PassRecord, PassContext, PassCoverage};
pub(crate) use pass_context::CoverageEntry;
pub use pgo::{PassPgoNameId, PassPgoDuration, PassPgoStats, PassPgoFrames, PgoKey};

pub struct ExecutionChain {
    subgraph_cache: super::subgraph::SubgraphPlanCache,
    plan_cache: crate::collections::cache::BoundedCache<PlanKey, ()>,
    plan_cache_enabled: bool,
    recording_ids: recording::RecordingCounters,
    pending_commands: Vec<(crate::sync::SyncPoint, recording::Command)>,
    completed_commands: Vec<recording::Command>,
    queues: Vec<vk::Queue>,
    command_records: BTreeMap<u64, recording::Command>,
    recorded_commands: u32,
    pool_family_count: u32,
    /// 并行录制分块配置；所有批次均由工作线程录制。
    pub parallel: crate::rg::parallel::ParallelRecording,
    /// 双缓冲 PGO：分块读 `previous`，本帧耗时写入 `current`，每次执行结束交换。
    ///
    /// This is the slot of the graph executed last (`pgo_key`). Slots of other
    /// graphs are parked in `pgo_parked`, so the main graph never plans its
    /// chunks from the much lighter presentation graph's timings.
    pub pgo: PassPgoFrames,
    /// Identity of the graph whose statistics are in `pgo`; `None` before the
    /// first execution (any pre-seeded statistics are then adopted by that graph).
    pgo_key: Option<PgoKey>,
    /// Statistics of recently executed other graphs, least recently used evicted.
    pgo_parked: Vec<(PgoKey, PassPgoFrames, u64)>,
    pgo_clock: u64,
    /// Per-batch chunk recording storage reused across batches and frames.
    chunk_scratch: ChunkScratch,
}
#[derive(Default)]
struct ChunkScratch {
    /// Previous-frame cost of each pass of the batch being planned.
    plan_costs: Vec<crate::rg::parallel::PassCost>,
    begun: Vec<recording::Command>,
    chunk_of_pass: Vec<usize>,
    planned: Vec<super::sync_plan::BoundTransition>,
    ranges: Vec<std::ops::Range<usize>>,
    transitions: Vec<ChunkTransition>,
}

/// Plan cache key: the compiled plan itself, compared and hashed by its
/// normalized definition, so the definition is stored once.
#[derive(Clone)]
struct PlanKey(std::sync::Arc<super::plan::CompiledPlan>);
impl PartialEq for PlanKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.definition == other.0.definition
    }
}
impl Eq for PlanKey {}
impl std::hash::Hash for PlanKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.definition.hash(state);
    }
}

/// 一个 batch 的并行录制输入。
struct ChunkedBatch<'a, C> {
    recorder: &'a mut record::Recorder,
    passes: &'a mut [record::Pass],
    callbacks: &'a mut [C],
    first_pass: usize,
    chunks: &'a [std::ops::Range<usize>],
    final_batch: bool,
    compiled: &'a crate::rg::graph::FrameInstance,
    managed: Option<&'a std::sync::Arc<crate::rg::pass_bindings::ManagedPassResources>>,
    pass_names: &'a [&'a crate::resource::ResourceName],
    cpu: Option<&'a CpuArenaLease>,
    family: u32,
    queue: recording::Queue,
}
/// 单块的独占工作单元。
struct ChunkWork<'a, 'l, C> {
    command: &'a recording::Command,
    /// Exclusive recording right for `command`'s pool on this worker.
    lease: ChunkLease<'l>,
    start: usize,
    callbacks: &'a mut [C],
    durations: &'a mut [Option<u64>],
    /// Flat barrier list of the whole batch; `ranges[offset]` selects one pass.
    transitions: &'a [ChunkTransition],
    ranges: &'a [std::ops::Range<usize>],
    gpu_timings: &'a [Option<super::profiling::GpuPassTimestamp>],
}
struct ChunkResult {
    ok: bool,
    error: Option<crate::Error>,
    recorded_durations: usize,
    #[cfg(feature = "cpu-probe")]
    command_workload: crate::profile::CommandWorkload,
    owners: super::owner_storage::ResourceOwners,
}

/// 一次图执行的阶段耗时与分块统计（性能追踪用）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExecutionStats {
    /// Native API calls/host copy bytes from complete successful recording.
    /// Not GPU-completed work. Partial/error execution explicitly invalidates it.
    #[cfg(feature = "cpu-probe")]
    pub command_workload: crate::profile::CommandWorkload,
    /// Commands successfully begun for this execution, including ownership preludes.
    pub commands_recorded: u32,
    pub plan_cache_hit: bool,
    pub culled_passes: u32,
    pub compile_ns: u64,
    pub record_ns: u64,
    pub submit_ns: u64,
    pub batches: u32,
    /// 实际并行录制的 batch 数及其总块数。
    pub parallel_batches: u32,
    pub chunks: u32,
    /// Pending cross-family ownership acquires recorded into this frame's
    /// batches (no separate acquire submit).
    pub coalesced_acquires: u32,
    /// 本次执行发出的屏障命令数（每个非空屏障作用域一次 `vkCmdPipelineBarrier`）。
    pub barrier_commands: u32,
    /// 本次执行包含的访问转换总数（屏障命令里的 buffer/image 屏障条目）。
    pub barrier_transitions: u32,
}
impl ExecutionStats {
    fn count_barriers(&mut self, outcome: &record::Outcome) {
        self.barrier_commands += outcome.barrier_commands;
        self.barrier_transitions += outcome.barrier_transitions;
    }
}

/// An allocated graph before command recording. It is bound to its originating
/// backend and resource pool; cancel it there if the consumer abandons the frame.
pub struct PreparedGraph {
    pub compiled: crate::rg::graph::FrameInstance,
    pub stats: ExecutionStats,
    cpu_arena: Option<CpuArenaLease>,
    queue: (recording::Queue, u32),
    original_pass_count: usize,
    pgo_key: PgoKey,
}

pub struct GraphExecution {
    pub compiled: crate::rg::graph::FrameInstance,
    pub transaction: FrameTransaction,
    pub stats: ExecutionStats,
}

impl std::fmt::Debug for GraphExecution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphExecution")
            .field("transaction", &self.transaction)
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}
impl GraphExecution {
    pub fn completion(&self) -> crate::sync::SyncSet {
        self.transaction.completion()
    }
}

#[cfg(test)]
mod exact_image_layout_tests;

impl ExecutionChain {

    pub fn new(primary: vk::Queue, auxiliary: Vec<vk::Queue>, family_count: u32) -> Self {
        let auxiliary_count = auxiliary.len() as u32;
        Self {
            subgraph_cache: super::subgraph::SubgraphPlanCache::default(),
            plan_cache: crate::collections::cache::BoundedCache::new(128, 16 * 1024 * 1024),
            plan_cache_enabled: true,
            recording_ids: recording::RecordingCounters::new(auxiliary_count),
            pending_commands: Vec::new(),
            completed_commands: Vec::new(),
            queues: std::iter::once(primary).chain(auxiliary).collect(),
            command_records: BTreeMap::new(),
            recorded_commands: 0,
            pool_family_count: family_count,
            parallel: Default::default(),
            chunk_scratch: Default::default(),
            pgo: Default::default(),
            pgo_key: None,
            pgo_parked: Vec::new(),
            pgo_clock: 0,
        }
    }

}

/// 执行后按逻辑索引提取的导出资源；物理 token 由后端资源表持有。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExportedResource {
    pub logical: u32,
    pub token: u64,
}
impl ExportedResource {
    pub fn new(logical: u32, token: u64) -> Self {
        Self { logical, token }
    }
}

#[cfg(test)]
mod pgo_slot_tests;
