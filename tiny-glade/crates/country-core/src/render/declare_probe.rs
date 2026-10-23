//! Diagnostic split of the declaration phase (reports/render-perf-architecture,
//! step 0). Counters are relaxed atomics; wall-clock sampling only happens when
//! `TG_FRAME_TIMING=1`, so a disabled run reads no `Instant`. `take` returns
//! the totals since the previous `take` and resets them.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

static CROSS_PASS_TASKS: AtomicU64 = AtomicU64::new(0);
static LIST_PASSES: AtomicU64 = AtomicU64::new(0);
static LIST_DRAWS: AtomicU64 = AtomicU64::new(0);
static LIST_LOOP_NS: AtomicU64 = AtomicU64::new(0);
static RENDER_NS: AtomicU64 = AtomicU64::new(0);
static TABLE_READS: AtomicU64 = AtomicU64::new(0);
static TABLE_NS: AtomicU64 = AtomicU64::new(0);
static DRAW_ACCESS_NS: AtomicU64 = AtomicU64::new(0);
static REC_DRAWS: AtomicU64 = AtomicU64::new(0);
static REC_PIPELINE_NS: AtomicU64 = AtomicU64::new(0);
static REC_RESOURCES_NS: AtomicU64 = AtomicU64::new(0);
static REC_PREPARE_NS: AtomicU64 = AtomicU64::new(0);
static REC_BIND_NS: AtomicU64 = AtomicU64::new(0);
static REC_DRAW_NS: AtomicU64 = AtomicU64::new(0);
static SUB_NS: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static STAGE_NS: [AtomicU64; 6] = [const { AtomicU64::new(0) }; 6];
static COLLECT_NS: [AtomicU64; 3] = [const { AtomicU64::new(0) }; 3];
static COLLECT_PASSES: AtomicU64 = AtomicU64::new(0);
static COLLECT_TASKS: AtomicU64 = AtomicU64::new(0);

/// Totals accumulated since the previous [`take`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeclareProbe {
    pub cross_pass_tasks: u64,
    pub collect_passes: u64,
    /// Number of shared-pool jobs; inline Collect contributes zero.
    pub collect_tasks: u64,
    /// Wall time for Prepare, Collect (including task wait), and Commit.
    pub collect_ns: [u64; 3],
    /// `run_list_pass` invocations and the draws they declared.
    pub list_passes: u64,
    pub list_draws: u64,
    /// Per-draw builder loop: `DrawBuilder::new`, `bind_common`, material callback.
    pub list_loop_ns: u64,
    /// `RasterPass::render`, which includes the host's pass declaration.
    pub render_ns: u64,
    /// Texture-table access declarations per raster pass (count and time).
    pub table_reads: u64,
    pub table_ns: u64,
    /// Per-draw indirect/index/vertex/external access declarations.
    pub draw_access_ns: u64,
    /// Record-side per-draw split of the list raster passes (summed over
    /// recording threads, so it can exceed wall time): draws, pipeline bind,
    /// resource gathering, `snapshot.prepare`, `bind_pass` + push, draw call.
    pub rec_draws: u64,
    pub rec_pipeline_ns: u64,
    pub rec_resources_ns: u64,
    pub rec_prepare_ns: u64,
    pub rec_bind_ns: u64,
    pub rec_draw_ns: u64,
    /// `submit_raster_pass` stages: 0 pipeline resolve, 1 imports, 2 intents and
    /// ranges, 3 texture table, 4 draw accesses (also counted above), 5 rest.
    pub stage_ns: [u64; 6],
    /// `ShaderBindingSnapshot::prepare` phases (summed over threads): requirements,
    /// globals page, sampler page, reserve+new, sampler write, globals write,
    /// resource writes, finish+commit.
    pub sub_ns: [u64; 8],
}

fn enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("TG_FRAME_TIMING").is_some_and(|value| value == "1"))
}

pub fn start() -> Option<Instant> {
    enabled().then(Instant::now)
}

fn add(counter: &AtomicU64, started: Option<Instant>) {
    if let Some(started) = started {
        counter.fetch_add(started.elapsed().as_nanos() as u64, Relaxed);
    }
}

pub fn list_loop(started: Option<Instant>, draws: usize) {
    LIST_PASSES.fetch_add(1, Relaxed);
    LIST_DRAWS.fetch_add(draws as u64, Relaxed);
    add(&LIST_LOOP_NS, started);
}
pub fn render(started: Option<Instant>) {
    add(&RENDER_NS, started);
}
pub fn table(started: Option<Instant>, reads: usize) {
    TABLE_READS.fetch_add(reads as u64, Relaxed);
    add(&TABLE_NS, started);
}
pub fn draw_access(started: Option<Instant>) {
    add(&DRAW_ACCESS_NS, started);
}

pub fn rec_draw() {
    REC_DRAWS.fetch_add(1, Relaxed);
}
pub fn rec_pipeline(started: Option<Instant>) {
    add(&REC_PIPELINE_NS, started);
}
pub fn rec_resources(started: Option<Instant>) {
    add(&REC_RESOURCES_NS, started);
}
pub fn rec_prepare(started: Option<Instant>) {
    add(&REC_PREPARE_NS, started);
}
pub fn rec_bind(started: Option<Instant>) {
    add(&REC_BIND_NS, started);
}
pub fn rec_draw_call(started: Option<Instant>) {
    add(&REC_DRAW_NS, started);
}

pub fn sub(index: usize, started: Option<Instant>) {
    add(&SUB_NS[index], started);
}
pub fn stage_count(index: usize) {
    STAGE_NS[index].fetch_add(1, Relaxed);
}
pub fn collect_stage(index: usize, started: Option<Instant>) { add(&COLLECT_NS[index], started); }
pub fn cross_pass_collected(tasks: usize) { CROSS_PASS_TASKS.fetch_add(tasks as u64, Relaxed); }
pub fn collected(tasks: usize) {
    COLLECT_PASSES.fetch_add(1, Relaxed);
    COLLECT_TASKS.fetch_add(tasks as u64, Relaxed);
}
pub fn stage(index: usize, started: Option<Instant>) {
    add(&STAGE_NS[index], started);
}

pub fn take() -> DeclareProbe {
    DeclareProbe {
        cross_pass_tasks: CROSS_PASS_TASKS.swap(0, Relaxed),        collect_passes: COLLECT_PASSES.swap(0, Relaxed),
        collect_tasks: COLLECT_TASKS.swap(0, Relaxed),
        collect_ns: std::array::from_fn(|i| COLLECT_NS[i].swap(0, Relaxed)),
        list_passes: LIST_PASSES.swap(0, Relaxed),
        list_draws: LIST_DRAWS.swap(0, Relaxed),
        list_loop_ns: LIST_LOOP_NS.swap(0, Relaxed),
        render_ns: RENDER_NS.swap(0, Relaxed),
        table_reads: TABLE_READS.swap(0, Relaxed),
        table_ns: TABLE_NS.swap(0, Relaxed),
        draw_access_ns: DRAW_ACCESS_NS.swap(0, Relaxed),
        rec_draws: REC_DRAWS.swap(0, Relaxed),
        rec_pipeline_ns: REC_PIPELINE_NS.swap(0, Relaxed),
        rec_resources_ns: REC_RESOURCES_NS.swap(0, Relaxed),
        rec_prepare_ns: REC_PREPARE_NS.swap(0, Relaxed),
        rec_bind_ns: REC_BIND_NS.swap(0, Relaxed),
        rec_draw_ns: REC_DRAW_NS.swap(0, Relaxed),
        stage_ns: std::array::from_fn(|i| STAGE_NS[i].swap(0, Relaxed)),
        sub_ns: std::array::from_fn(|i| SUB_NS[i].swap(0, Relaxed)),
    }
}
