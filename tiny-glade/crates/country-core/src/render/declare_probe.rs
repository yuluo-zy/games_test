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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct DeclareProbe {
    pub cross_pass_tasks: u64,
    pub collect_passes: u64,
    /// Number of shared-pool jobs recorded while diagnostics are enabled.
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
    /// ranges, 3 texture table, 4 session import time, 5 session import count (not nanoseconds).
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
    if enabled() {
        LIST_PASSES.fetch_add(1, Relaxed);
        LIST_DRAWS.fetch_add(draws as u64, Relaxed);
    }
    add(&LIST_LOOP_NS, started);
}
pub fn render(started: Option<Instant>) {
    add(&RENDER_NS, started);
}
pub fn table(started: Option<Instant>, reads: usize) {
    if enabled() { TABLE_READS.fetch_add(reads as u64, Relaxed); }
    add(&TABLE_NS, started);
}
pub fn draw_access(started: Option<Instant>) {
    add(&DRAW_ACCESS_NS, started);
}

pub fn rec_draw() {
    if enabled() { REC_DRAWS.fetch_add(1, Relaxed); }
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
    if enabled() { STAGE_NS[index].fetch_add(1, Relaxed); }
}
pub fn collect_stage(index: usize, started: Option<Instant>) { add(&COLLECT_NS[index], started); }
pub fn cross_pass_collected(tasks: usize) {
    if enabled() { CROSS_PASS_TASKS.fetch_add(tasks as u64, Relaxed); }
}
pub fn collected(tasks: usize) {
    if enabled() {
        COLLECT_PASSES.fetch_add(1, Relaxed);
        COLLECT_TASKS.fetch_add(tasks as u64, Relaxed);
    }
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

/// Count-only Collect workload sampling, scoped to the fixture's owner thread.
/// This is independent of timing diagnostics and is absent without cpu-probe.
#[cfg(feature = "cpu-probe")]
mod count_only {
    use std::cell::Cell;
    use std::marker::PhantomData;
    use std::rc::Rc;

    /// Cumulative actual joined Collect work, never proof of GPU submission.
    /// Cross-pass values are subsets of tasks/waves, not additional work.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
    pub struct CollectCounts {
        pub tasks: u64,
        pub waves: u64,
        pub cross_pass_tasks: u64,
        pub cross_pass_waves: u64,
        /// Waves whose guard observed a new failure or did not complete.
        /// Admission/Commit failures outside a wave still invalidate the session.
        pub failed_waves: u64,
        pub overflow: bool,
        /// False when inactive, reentered, overflowed, failed, or left mid-wave.
        pub valid: bool,
    }

    impl CollectCounts {
        fn add(value: &mut u64, amount: u64, overflow: &mut bool) {
            if let Some(next) = value.checked_add(amount) {
                *value = next;
            } else {
                *overflow = true;
            }
        }

        fn joined(&mut self, tasks: usize, cross_pass: bool) {
            let Ok(tasks) = u64::try_from(tasks) else {
                self.overflow = true;
                self.valid = false;
                return;
            };
            Self::add(&mut self.tasks, tasks, &mut self.overflow);
            Self::add(&mut self.waves, 1, &mut self.overflow);
            if cross_pass {
                Self::add(&mut self.cross_pass_tasks, tasks, &mut self.overflow);
                Self::add(&mut self.cross_pass_waves, 1, &mut self.overflow);
            }
            self.valid &= !self.overflow;
        }

        fn failed_wave(&mut self) {
            Self::add(&mut self.failed_waves, 1, &mut self.overflow);
            self.valid = false;
        }
    }

    #[derive(Clone, Copy, Default)]
    struct State {
        enabled: bool,
        generation: u64,
        errors: u64,
        active_waves: u64,
        counts: CollectCounts,
    }

    thread_local! {
        static COUNTS: Cell<State> = const { Cell::new(State {
            enabled: false, generation: 0, errors: 0, active_waves: 0,
            counts: CollectCounts { tasks: 0, waves: 0, cross_pass_tasks: 0,
                cross_pass_waves: 0, failed_waves: 0, overflow: false, valid: false },
        }) };
    }

    /// Start/reset only this owner thread. A rejected nested begin preserves
    /// accumulated data and makes the existing session invalid. Only a caller
    /// whose begin returned true owns the matching end operation.
    pub fn collect_counts_begin() -> bool {
        COUNTS.with(|cell| {
            let mut state = cell.get();
            if state.enabled {
                state.counts.valid = false;
                cell.set(state);
                return false;
            }
            let Some(generation) = state.generation.checked_add(1) else {
                state.counts.overflow = true;
                state.counts.valid = false;
                cell.set(state);
                return false;
            };
            cell.set(State { enabled: true, generation,
                counts: CollectCounts { valid: true, ..Default::default() },
                ..Default::default() });
            true
        })
    }

    /// Fixed Copy snapshot, cumulative since begin; no allocation or reset.
    pub fn collect_counts_snapshot() -> CollectCounts {
        COUNTS.with(|cell| {
            let state = cell.get();
            CollectCounts { valid: state.enabled && state.counts.valid && state.active_waves == 0, ..state.counts }
        })
    }

    pub fn collect_counts_end() -> CollectCounts {
        COUNTS.with(|cell| {
            let mut state = cell.get();
            state.counts.valid &= state.enabled && state.active_waves == 0;
            let result = state.counts;
            state.enabled = false;
            // Retain raw final counters even after an erroneous extra end;
            // inactive reads never turn them into valid measurements.
            cell.set(state);
            result
        })
    }

    /// Admission, worker-result or Commit refusal. Does not invent joined work.
    pub fn collect_counts_failed() {
        COUNTS.with(|cell| {
            let mut state = cell.get();
            if !state.enabled { return; }
            state.counts.valid = false;
            CollectCounts::add(&mut state.errors, 1, &mut state.counts.overflow);
            cell.set(state);
        });
    }

    /// Owner-only guard: no Rc is allocated; PhantomData prevents Send/Sync.
    pub(crate) struct CollectCountWave {
        generation: u64,
        initial_errors: u64,
        cross_pass: bool,
        active: bool,
        owner: PhantomData<Rc<()>>,
    }

    pub(crate) fn collect_count_wave(cross_pass: bool) -> CollectCountWave {
        COUNTS.with(|cell| {
            let mut state = cell.get();
            let active = state.enabled;
            if active {
                CollectCounts::add(&mut state.active_waves, 1, &mut state.counts.overflow);
                state.counts.valid &= !state.counts.overflow;
                cell.set(state);
            }
            CollectCountWave { generation: state.generation, initial_errors: state.errors,
                cross_pass, active, owner: PhantomData }
        })
    }

    impl CollectCountWave {
        /// Called once only after the real map_chunks joined (and, for the
        /// single-pass path, owner Commit returned). Errors remain sticky even
        /// when actual task work is included in the cumulative totals.
        pub(crate) fn complete(mut self, tasks: usize) {
            if self.active {
                COUNTS.with(|cell| {
                    let mut state = cell.get();
                    if !state.enabled || state.generation != self.generation { return; }
                    state.counts.joined(tasks, self.cross_pass);
                    if state.errors != self.initial_errors {
                        state.counts.failed_wave();
                    }
                    state.active_waves -= 1;
                    cell.set(state);
                });
                self.active = false;
            }
        }
    }

    impl Drop for CollectCountWave {
        fn drop(&mut self) {
            if !self.active { return; }
            COUNTS.with(|cell| {
                let mut state = cell.get();
                if !state.enabled || state.generation != self.generation { return; }
                state.counts.failed_wave();
                state.active_waves -= 1;
                cell.set(state);
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn reset() { COUNTS.with(|cell| cell.set(State::default())); }

        #[test]
        fn disabled_does_not_collect_or_create_valid_zero_samples() {
            reset();
            collect_count_wave(false).complete(8);
            collect_counts_failed();
            assert_eq!(collect_counts_snapshot(), CollectCounts::default());
            assert_eq!(collect_counts_end(), CollectCounts::default());
        }

        #[test]
        fn multiple_waves_count_cross_pass_as_subset_exactly_once() {
            reset();
            assert!(collect_counts_begin());
            collect_count_wave(false).complete(8);
            collect_count_wave(true).complete(8);
            collect_count_wave(true).complete(4);
            let a = collect_counts_snapshot();
            assert_eq!((a.tasks, a.waves, a.cross_pass_tasks, a.cross_pass_waves), (20, 3, 12, 2));
            assert_eq!(collect_counts_snapshot(), a, "snapshot does not reset");
            assert_eq!(collect_counts_end(), a);
            assert!(!collect_counts_snapshot().valid);
        }

        #[test]
        fn failure_and_cancellation_never_report_success_or_fake_joined_tasks() {
            reset(); assert!(collect_counts_begin());
            let wave = collect_count_wave(false);
            collect_counts_failed();
            collect_counts_failed();
            wave.complete(3);
            drop(collect_count_wave(true));
            let counts = collect_counts_end();
            assert_eq!((counts.tasks, counts.waves, counts.cross_pass_tasks), (3, 1, 0));
            assert_eq!(counts.failed_waves, 2, "two errors in one wave count only one failed wave");
            assert!(!counts.valid);
            assert!(!counts.overflow);
        }

        #[test]
        fn admission_failure_without_dispatched_wave_only_invalidates() {
            reset(); assert!(collect_counts_begin()); collect_counts_failed();
            let counts = collect_counts_end();
            assert_eq!((counts.tasks, counts.waves, counts.failed_waves), (0, 0, 0));
            assert!(!counts.valid);
        }

        #[test]
        fn overflow_is_sticky_and_does_not_wrap() {
            reset(); assert!(collect_counts_begin());
            COUNTS.with(|cell| {
                let mut state = cell.get(); state.counts.tasks = u64::MAX; cell.set(state);
            });
            collect_count_wave(false).complete(1);
            let counts = collect_counts_end();
            assert_eq!(counts.tasks, u64::MAX);
            assert!(counts.overflow && !counts.valid);
        }

        #[test]
        fn nested_begin_preserves_invalid_outer_data_until_its_end() {
            reset(); assert!(collect_counts_begin()); collect_count_wave(false).complete(3);
            assert!(!collect_counts_begin());
            collect_count_wave(false).complete(2);
            let counts = collect_counts_end();
            assert_eq!(counts.tasks, 5);
            assert!(!counts.valid);
            assert_eq!(collect_counts_end().tasks, 5, "extra end preserves raw final data");
        }

        #[test]
        fn unfinished_old_session_cannot_mutate_a_new_session() {
            reset(); assert!(collect_counts_begin()); let old = collect_count_wave(false);
            assert!(!collect_counts_snapshot().valid, "unfinished wave is not a valid completion snapshot");
            assert!(!collect_counts_end().valid);
            assert!(collect_counts_begin()); drop(old);
            collect_count_wave(true).complete(2);
            let counts = collect_counts_end();
            assert_eq!((counts.tasks, counts.waves), (2, 1));
            assert!(counts.valid);
        }

        #[test]
        fn worker_tls_cannot_accidentally_publish_owner_counts() {
            reset(); assert!(collect_counts_begin());
            let worker = std::thread::spawn(|| {
                collect_count_wave(false).complete(5); collect_counts_failed();
                collect_counts_snapshot()
            }).join().unwrap();
            assert!(!worker.valid && worker.tasks == 0);
            collect_count_wave(false).complete(1);
            assert_eq!(collect_counts_end().tasks, 1);
        }
    }
}

#[cfg(feature = "cpu-probe")]
pub use count_only::{CollectCounts, collect_counts_begin, collect_counts_end,
    collect_counts_failed, collect_counts_snapshot};
#[cfg(feature = "cpu-probe")]
pub(crate) use count_only::collect_count_wave;
