//! Diagnostic split of the declaration phase (reports/render-perf-architecture,
//! step 0). Counters are relaxed atomics; wall-clock sampling only happens when
//! `TG_FRAME_TIMING=1`, so a disabled run reads no `Instant`. `take` returns
//! the totals since the previous `take` and resets them.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

static LIST_PASSES: AtomicU64 = AtomicU64::new(0);
static LIST_DRAWS: AtomicU64 = AtomicU64::new(0);
static LIST_LOOP_NS: AtomicU64 = AtomicU64::new(0);
static RENDER_NS: AtomicU64 = AtomicU64::new(0);
static TABLE_READS: AtomicU64 = AtomicU64::new(0);
static TABLE_NS: AtomicU64 = AtomicU64::new(0);
static DRAW_ACCESS_NS: AtomicU64 = AtomicU64::new(0);

/// Totals accumulated since the previous [`take`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeclareProbe {
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

pub fn take() -> DeclareProbe {
    DeclareProbe {
        list_passes: LIST_PASSES.swap(0, Relaxed),
        list_draws: LIST_DRAWS.swap(0, Relaxed),
        list_loop_ns: LIST_LOOP_NS.swap(0, Relaxed),
        render_ns: RENDER_NS.swap(0, Relaxed),
        table_reads: TABLE_READS.swap(0, Relaxed),
        table_ns: TABLE_NS.swap(0, Relaxed),
        draw_access_ns: DRAW_ACCESS_NS.swap(0, Relaxed),
    }
}
