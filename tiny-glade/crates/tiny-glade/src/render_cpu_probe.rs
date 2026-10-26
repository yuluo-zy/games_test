//! Clean per-frame render-thread CPU sampling, independent of the frame-timing
//! log and its `frame_timing_every` sampling gate.
//!
//! The legacy `ProductionRenderer::record_cpu_interval` read
//! `CLOCK_THREAD_CPUTIME_ID` but only on the full-timing path, emitted through
//! `log::info!`, and was gated by log sampling — so its per-frame distribution
//! carried log formatting and control/poll work and could not be read as a
//! clean production p95. This recorder instead:
//!
//! - allocates one bounded record buffer up front (`try_reserve_exact`), so the
//!   measurement never allocates on the hot path and never grows unbounded;
//! - on each render completion point reads the thread CPU clock and stores the
//!   interval against the real observed frame (World tick / main-graph receipt /
//!   attempt), with no logging and no sampling gate;
//! - advances the baseline on failed/cancelled/app-switch frames instead of
//!   assuming their gameplay counter equals a world observation frame;
//! - truncates with an explicit error flag when the buffer is full, and records
//!   a clock-regression sample as an error rather than panicking;
//! - resets its baseline across App switches.
//!
//! Compiled only with `cpu-probe` (also implied by `profile`). Windows keeps
//! GetThreadTimes cumulative CPU for aggregate checks; its percentiles require
//! ETW scheduling intervals because FILETIME units do not guarantee resolution.
//! macOS uses CLOCK_THREAD_CPUTIME_ID. Completion points retain QPC timestamps
//! for offline ETW alignment without hot-path event/log serialization.
#![cfg(feature = "cpu-probe")]

use serde_json::{Value, json};

pub use crate::cpu_clock::{current_thread_id, process_cpu_ns, read_thread_cpu_ns, wall_frequency, wall_ticks};

/// Why a completion point did not yield a clean gameplay CPU sample. A recorded
/// sample is either [`Clean`](SampleKind::Clean) or one of these error kinds;
/// error kinds still advance the baseline so the next clean interval does not
/// absorb the gap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleKind {
    /// A real observed frame: the interval is owner CPU between two consecutive
    /// completion points with a valid, monotonic clock.
    Clean,
    /// The thread CPU clock went backwards between completions. The interval is
    /// discarded; the baseline is re-seeded from the new reading.
    ClockRegressed,
    /// `clock_gettime` failed or is unavailable on this host; the baseline is
    /// dropped and re-seeded next frame.
    ClockError,
    /// The frame failed, was cancelled, or the App was switched: there is no
    /// world observation frame to attribute CPU to. The baseline advances.
    NotObserved,
}

impl SampleKind {
    fn as_str(self) -> &'static str {
        match self {
            SampleKind::Clean => "clean",
            SampleKind::ClockRegressed => "clock_regressed",
            SampleKind::ClockError => "clock_error",
            SampleKind::NotObserved => "not_observed",
        }
    }
}

/// One completion-point sample. `frame` is the gameplay tick at the completion
/// point; `receipt`/`attempt` tie it to the real main-graph observation. For
/// clean samples `cpu_ns` is the owner CPU since the previous completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuSample {
    pub frame: u64,
    pub receipt: Option<u64>,
    pub attempt: u64,
    pub interval_frames: u64,
    pub cpu_ns: u64,
    pub kind: SampleKind,
}

/// A bounded, allocation-free-on-the-hot-path recorder for render-thread owner
/// CPU intervals. Construct once per measurement session with a fixed capacity.
pub struct CpuIntervalRecorder {
    /// `(frame, cumulative_cpu_ns)` of the previous completion point, or `None`
    /// before the first reading / after a reset or clock error.
    baseline: Option<(u64, u64)>,
    samples: Vec<CpuSample>,
    capacity: usize,
    /// Set once the buffer is full and a sample had to be dropped. The caller
    /// reads this as an explicit truncation error rather than silently losing
    /// tail samples.
    truncated: bool,
    /// Count of completion points offered after the buffer filled.
    dropped: u64,
}

impl CpuIntervalRecorder {
    /// Allocate a recorder holding at most `capacity` samples. The backing
    /// buffer is reserved exactly, up front; a failed reservation (host OOM) or
    /// a zero capacity returns `None` so the caller can decline to measure
    /// rather than abort.
    pub fn with_capacity(capacity: usize) -> Option<Self> {
        let mut samples = Vec::new();
        if capacity == 0 || samples.try_reserve_exact(capacity).is_err() {
            return None;
        }
        Some(Self {
            baseline: None,
            samples,
            capacity,
            truncated: false,
            dropped: 0,
        })
    }

    /// Drop the baseline across an App switch so the next completion re-seeds it
    /// and the first interval of the new App is not measured against the old
    /// App's clock. Keeps already-recorded samples.
    pub fn reset_baseline(&mut self) {
        self.baseline = None;
    }

    /// Cumulative owner CPU nanoseconds from the shared platform clock.
    /// Retained here for compatibility with existing recorder consumers.
    pub fn read_thread_cpu_ns() -> Option<u64> {
        crate::cpu_clock::read_thread_cpu_ns()
    }

    /// Record a completion point for a real observed frame. Reads the clock and
    /// stores the interval since the previous completion. Call exactly once per
    /// render completion that produced a world observation.
    pub fn record_observed(&mut self, frame: u64, receipt: Option<u64>, attempt: u64) {
        let Some(now) = Self::read_thread_cpu_ns() else {
            // Clock failure/unavailable: drop the baseline, record the error.
            self.baseline = None;
            self.store(CpuSample {
                frame,
                receipt,
                attempt,
                interval_frames: 0,
                cpu_ns: 0,
                kind: SampleKind::ClockError,
            });
            return;
        };
        self.record_reading(frame, receipt, attempt, now, SampleKind::Clean);
    }

    /// Record a completion point that produced no world observation (failed,
    /// cancelled, or App switch). Advances the baseline without emitting a clean
    /// interval, so the next clean sample does not absorb this gap.
    pub fn record_unobserved(&mut self, frame: u64, attempt: u64) {
        let Some(now) = Self::read_thread_cpu_ns() else {
            self.baseline = None;
            self.store(CpuSample {
                frame,
                receipt: None,
                attempt,
                interval_frames: 0,
                cpu_ns: 0,
                kind: SampleKind::ClockError,
            });
            return;
        };
        self.record_reading(frame, None, attempt, now, SampleKind::NotObserved);
    }

    /// Core bookkeeping shared by observed and unobserved readings. Separated
    /// from the clock so it is unit-testable on any host via
    /// [`Self::push_interval_ns`]. `kind` is the attribution for a *successful*
    /// clock read; a regression downgrades it to `ClockRegressed`.
    fn record_reading(
        &mut self,
        frame: u64,
        receipt: Option<u64>,
        attempt: u64,
        now_cpu_ns: u64,
        kind: SampleKind,
    ) {
        let previous = self.baseline.replace((frame, now_cpu_ns));
        let Some((previous_frame, previous_cpu)) = previous else {
            // First reading after seed/reset: establish the baseline only.
            // An unobserved first reading still seeds without a sample.
            return;
        };
        let interval_frames = frame.saturating_sub(previous_frame);
        if kind == SampleKind::NotObserved {
            // The gap frame advanced the baseline (done above); record it as a
            // non-clean sample carrying no owner CPU interval.
            self.store(CpuSample {
                frame,
                receipt,
                attempt,
                interval_frames,
                cpu_ns: 0,
                kind: SampleKind::NotObserved,
            });
            return;
        }
        match now_cpu_ns.checked_sub(previous_cpu) {
            Some(cpu_ns) => self.store(CpuSample {
                frame,
                receipt,
                attempt,
                interval_frames,
                cpu_ns,
                kind,
            }),
            None => {
                // Clock regressed: re-seed (already replaced above) and flag it.
                self.store(CpuSample {
                    frame,
                    receipt,
                    attempt,
                    interval_frames,
                    cpu_ns: 0,
                    kind: SampleKind::ClockRegressed,
                });
            }
        }
    }

    /// Clock-free entry for tests: treat `now_cpu_ns` as the reading at this
    /// completion point for a real observed frame. Mirrors `record_observed`
    /// without touching `CLOCK_THREAD_CPUTIME_ID`.
    pub fn push_interval_ns(
        &mut self,
        frame: u64,
        receipt: Option<u64>,
        attempt: u64,
        now_cpu_ns: u64,
    ) {
        self.record_reading(frame, receipt, attempt, now_cpu_ns, SampleKind::Clean);
    }

    /// Clock-free entry for tests: record an unobserved gap at `now_cpu_ns`
    /// (advances the baseline, no clean interval). Mirrors `record_unobserved`.
    pub fn push_unobserved_ns(&mut self, frame: u64, attempt: u64, now_cpu_ns: u64) {
        self.record_reading(frame, None, attempt, now_cpu_ns, SampleKind::NotObserved);
    }

    fn store(&mut self, sample: CpuSample) {
        if self.samples.len() >= self.capacity {
            self.truncated = true;
            self.dropped += 1;
            return;
        }
        self.samples.push(sample);
    }

    pub fn samples(&self) -> &[CpuSample] {
        &self.samples
    }

    /// True once the buffer filled and at least one sample was dropped.
    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn dropped(&self) -> u64 {
        self.dropped
    }

    /// Clean samples only, for distribution statistics.
    fn clean_cpu_ns(&self) -> Vec<u64> {
        self.samples
            .iter()
            .filter(|s| s.kind == SampleKind::Clean)
            .map(|s| s.cpu_ns)
            .collect()
    }

    /// Mean of clean owner-CPU intervals in nanoseconds, or `None` if there are
    /// no clean samples.
    pub fn clean_mean_ns(&self) -> Option<f64> {
        let values = self.clean_cpu_ns();
        if values.is_empty() {
            return None;
        }
        let sum: u128 = values.iter().map(|&v| v as u128).sum();
        Some(sum as f64 / values.len() as f64)
    }

    /// Nearest-rank p95 of clean owner-CPU intervals in nanoseconds.
    pub fn clean_p95_ns(&self) -> Option<u64> {
        let mut values = self.clean_cpu_ns();
        if values.is_empty() {
            return None;
        }
        values.sort_unstable();
        // Nearest-rank: ceil(0.95 * n) clamped into range.
        let rank = ((values.len() as f64) * 0.95).ceil() as usize;
        let index = rank.saturating_sub(1).min(values.len() - 1);
        Some(values[index])
    }

    /// Structured dump aligned with the existing
    /// `cpu-clock-receipt-crosscheck` evidence shape: a header with sample
    /// counts and the clean distribution, plus every per-receipt sample.
    pub fn to_json(&self, label: &str) -> Value {
        let clean = self.clean_cpu_ns();
        let samples: Vec<Value> = self
            .samples
            .iter()
            .map(|s| {
                json!({
                    "frame": s.frame,
                    "receipt": s.receipt,
                    "attempt": s.attempt,
                    "interval_frames": s.interval_frames,
                    "cpu_ns": s.cpu_ns,
                    "kind": s.kind.as_str(),
                })
            })
            .collect();
        json!({
            "schema": "render-cpu-clean-v1",
            "label": label,
            "clock": if cfg!(windows) { "GetThreadTimes(user+kernel)" } else { "CLOCK_THREAD_CPUTIME_ID" },
            "notes": "owner CPU between consecutive render completion points; sleep/GPU excluded, busy-wait included; no log formatting on the hot path",
            "capacity": self.capacity,
            "recorded": self.samples.len(),
            "clean_samples": clean.len(),
            "truncated": self.truncated,
            "dropped": self.dropped,
            "clean_mean_ns": self.clean_mean_ns(),
            "clean_p95_ns": self.clean_p95_ns(),
            "samples": samples,
        })
    }
}

/// A session is installed on the resident owner once and retrieved once.
/// Warmup counts actual world observations, not render attempts or loading frames.
#[derive(Clone, Debug)]
pub struct CpuProbeConfig {
    pub label: String,
    pub warmup_frames: u64,
    pub sample_frames: usize,
    pub capacity: usize,
}

#[derive(Clone, Copy)]
struct CompletionPoint {
    app_epoch: u64,
    world_tick: u64,
    receipt: Option<u64>,
    attempt: u64,
    qpc: u64,
    cumulative_cpu_ns: Option<u64>,
    observed: bool,
}

pub struct CpuProbeSession {
    config: CpuProbeConfig,
    recorder: CpuIntervalRecorder,
    points: Vec<CompletionPoint>,
    warmup_left: u64,
    app_epoch: u64,
    last_receipt: Option<u64>,
    invalid: bool,
    thread_id: u64,
    process_start: Option<u64>,
    process_end: Option<u64>,
}

impl CpuProbeSession {
    pub fn new(config: CpuProbeConfig) -> anyhow::Result<Self> {
        anyhow::ensure!(config.sample_frames > 0 && config.capacity > config.sample_frames,
            "CPU probe needs positive samples and capacity >= samples + 1");
        let recorder = CpuIntervalRecorder::with_capacity(config.capacity)
            .ok_or_else(|| anyhow::anyhow!("CPU probe buffer allocation failed"))?;
        let mut points = Vec::new();
        points.try_reserve_exact(config.capacity)?;
        Ok(Self { warmup_left: config.warmup_frames, config, recorder, points,
            app_epoch: 0, last_receipt: None, invalid: false,
            thread_id: current_thread_id(), process_start: None, process_end: None })
    }

    pub fn reset_baseline(&mut self) {
        self.app_epoch += 1;
        self.last_receipt = None;
        self.recorder.reset_baseline();
    }

    pub fn complete(&self) -> bool {
        self.recorder.samples.len() >= self.config.sample_frames
    }

    pub fn record(&mut self, world_tick: u64, receipt: Option<u64>, attempt: u64, observed: bool) {
        if self.complete() { return; }
        if self.warmup_left != 0 {
            if observed { self.warmup_left -= 1; }
            self.recorder.reset_baseline();
            return;
        }
        self.record_reading(world_tick, receipt, attempt, observed,
            CpuIntervalRecorder::read_thread_cpu_ns(), wall_ticks());
    }

    fn record_reading(&mut self, world_tick: u64, receipt: Option<u64>, attempt: u64,
        observed: bool, cpu: Option<u64>, qpc: u64) {
        if self.complete() { return; }
        if self.points.len() >= self.config.capacity {
            self.invalid = true;
            self.recorder.truncated = true;
            self.recorder.dropped += 1;
            return;
        }
        if self.points.is_empty() { self.process_start = process_cpu_ns(); }
        let observed = observed && receipt.is_some();
        if !observed || (observed && self.last_receipt.is_some_and(|previous| receipt.unwrap() <= previous)) {
            self.invalid = true;
        }
        if observed { self.last_receipt = receipt; }
        self.points.push(CompletionPoint { app_epoch: self.app_epoch, world_tick,
            receipt, attempt, qpc, cumulative_cpu_ns: cpu, observed });
        match cpu {
            Some(now) => self.recorder.record_reading(world_tick, receipt, attempt, now,
                if observed { SampleKind::Clean } else { SampleKind::NotObserved }),
            None => {
                self.invalid = true;
                self.recorder.baseline = None;
                self.recorder.store(CpuSample { frame: world_tick, receipt, attempt,
                    interval_frames: 0, cpu_ns: 0, kind: SampleKind::ClockError });
            }
        }
        if self.complete() { self.process_end = process_cpu_ns(); }
    }

    pub fn to_json(&self) -> Value {
        let mut value = self.recorder.to_json(&self.config.label);
        value["schema"] = json!("render-cpu-clean-v2");
        value["thread_id"] = json!(self.thread_id);
        value["process_id"] = json!(std::process::id());
        value["qpc_frequency"] = json!(wall_frequency());
        value["warmup_frames"] = json!(self.config.warmup_frames);
        value["requested_samples"] = json!(self.config.sample_frames);
        value["complete"] = json!(self.complete());
        value["valid"] = json!(self.complete() && !self.invalid && !self.recorder.truncated
            && self.recorder.samples.iter().all(|sample| sample.kind == SampleKind::Clean));
        value["percentiles_require_etw_on_windows"] = json!(cfg!(windows));
        value["process_cpu_start_ns"] = json!(self.process_start);
        value["process_cpu_end_ns"] = json!(self.process_end);
        value["completion_points"] = Value::Array(self.points.iter().map(|point| json!({
            "app_epoch": point.app_epoch, "world_tick": point.world_tick,
            "receipt": point.receipt, "attempt": point.attempt, "qpc": point.qpc,
            "cumulative_cpu_ns": point.cumulative_cpu_ns, "observed": point.observed,
        })).collect());
        value
    }
}

#[cfg(test)]
mod session_tests {
    use super::*;
    fn session(samples: usize) -> CpuProbeSession {
        CpuProbeSession::new(CpuProbeConfig { label: "test".into(), warmup_frames: 0,
            sample_frames: samples, capacity: samples + 2 }).unwrap()
    }
    #[test]
    fn completed_session_is_bounded_and_retains_raw_alignment() {
        let mut session = session(2);
        for index in 0..10 { session.record_reading(index, Some(index), index, true, Some(index * 100), index * 1000); }
        let value = session.to_json();
        assert_eq!(value["complete"], true);
        assert_eq!(value["valid"], true);
        assert_eq!(value["completion_points"].as_array().unwrap().len(), 3);
        assert_eq!(value["clean_samples"], 2);
    }
    #[test]
    fn failed_or_duplicate_receipts_invalidate_acceptance() {
        let mut session = session(2);
        session.record_reading(1, Some(1), 1, true, Some(100), 1);
        session.record_reading(2, None, 2, false, Some(200), 2);
        session.record_reading(3, Some(1), 3, true, Some(300), 3);
        assert_eq!(session.to_json()["valid"], false);
    }
    #[test]
    fn app_epoch_changes_allow_new_receipts_without_cross_app_interval() {
        let mut session = session(2);
        session.record_reading(100, Some(100), 1, true, Some(100), 1);
        session.reset_baseline();
        session.record_reading(1, Some(1), 2, true, Some(10000), 2);
        session.record_reading(2, Some(2), 3, true, Some(10100), 3);
        assert_eq!(session.recorder.samples[0].cpu_ns, 100);
        assert_eq!(session.points[1].app_epoch, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_is_bounded_and_truncation_is_an_explicit_error() {
        let mut rec = CpuIntervalRecorder::with_capacity(2).unwrap();
        // First reading seeds the baseline (no stored sample yet).
        rec.push_interval_ns(0, Some(0), 0, 1_000);
        // Two intervals fit.
        rec.push_interval_ns(1, Some(1), 0, 3_000); // cpu 2000
        rec.push_interval_ns(2, Some(2), 0, 6_000); // cpu 3000
        assert_eq!(rec.samples().len(), 2);
        assert!(!rec.truncated());
        // Third interval overflows the bounded buffer.
        rec.push_interval_ns(3, Some(3), 0, 10_000);
        assert_eq!(rec.samples().len(), 2, "buffer stays bounded");
        assert!(rec.truncated(), "overflow is flagged, not silent");
        assert_eq!(rec.dropped(), 1);
        // Recorded intervals are exactly the clean deltas.
        assert_eq!(rec.samples()[0].cpu_ns, 2_000);
        assert_eq!(rec.samples()[1].cpu_ns, 3_000);
    }

    #[test]
    fn zero_capacity_declines_to_allocate() {
        assert!(CpuIntervalRecorder::with_capacity(0).is_none());
    }

    #[test]
    fn clock_regression_is_recorded_as_an_error_sample_not_a_panic() {
        let mut rec = CpuIntervalRecorder::with_capacity(8).unwrap();
        rec.push_interval_ns(0, Some(0), 0, 5_000); // seed
        rec.push_interval_ns(1, Some(1), 0, 7_000); // clean cpu 2000
        rec.push_interval_ns(2, Some(2), 0, 4_000); // regressed (< previous)
        rec.push_interval_ns(3, Some(3), 0, 6_000); // clean again cpu 2000 from re-seed
        let kinds: Vec<_> = rec.samples().iter().map(|s| s.kind).collect();
        assert_eq!(
            kinds,
            vec![SampleKind::Clean, SampleKind::ClockRegressed, SampleKind::Clean]
        );
        // The regressed sample carries no cpu time and re-seeds the baseline.
        assert_eq!(rec.samples()[1].cpu_ns, 0);
        assert_eq!(rec.samples()[2].cpu_ns, 2_000);
    }

    #[test]
    fn unobserved_frames_advance_the_baseline_without_a_clean_interval() {
        let mut rec = CpuIntervalRecorder::with_capacity(8).unwrap();
        rec.push_interval_ns(0, Some(0), 0, 1_000); // seed
        rec.push_interval_ns(1, Some(1), 0, 3_000); // clean cpu 2000
        // A cancelled frame at frame 2 (completion CPU reading 5_000): advances
        // the baseline, records a NotObserved sample with no owner CPU.
        rec.push_unobserved_ns(2, 0, 5_000);
        rec.push_interval_ns(3, Some(3), 0, 6_000); // clean cpu 1000 from frame 2
        let kinds: Vec<_> = rec.samples().iter().map(|s| s.kind).collect();
        assert_eq!(
            kinds,
            vec![SampleKind::Clean, SampleKind::NotObserved, SampleKind::Clean]
        );
        // The clean interval after the gap is measured from frame 2's reading,
        // not from frame 1 — the cancelled frame's CPU is not folded in.
        assert_eq!(rec.samples()[2].cpu_ns, 1_000);
        assert_eq!(rec.samples()[2].interval_frames, 1);
        // The unobserved sample carries no receipt (no world observation).
        assert_eq!(rec.samples()[1].receipt, None);
    }

    #[test]
    fn reset_baseline_drops_cross_app_interval() {
        let mut rec = CpuIntervalRecorder::with_capacity(8).unwrap();
        rec.push_interval_ns(10, Some(10), 0, 100_000); // App A seed
        rec.push_interval_ns(11, Some(11), 0, 102_000); // clean cpu 2000
        rec.reset_baseline();
        // App B's first reading seeds again; no giant interval across the switch.
        rec.push_interval_ns(0, Some(0), 1, 500_000);
        rec.push_interval_ns(1, Some(1), 1, 503_000); // clean cpu 3000
        let cleans: Vec<_> = rec
            .samples()
            .iter()
            .filter(|s| s.kind == SampleKind::Clean)
            .map(|s| s.cpu_ns)
            .collect();
        assert_eq!(cleans, vec![2_000, 3_000], "no cross-App interval leaked");
    }

    #[test]
    fn receipt_association_and_json_shape_round_trip() {
        let mut rec = CpuIntervalRecorder::with_capacity(8).unwrap();
        rec.push_interval_ns(0, Some(100), 0, 0); // seed
        rec.push_interval_ns(1, Some(101), 0, 8_000_000); // 8ms
        rec.push_interval_ns(2, Some(102), 0, 16_000_000); // 8ms
        rec.push_interval_ns(3, Some(103), 0, 40_000_000); // 24ms
        let value = rec.to_json("static-unit");
        assert_eq!(value["clean_samples"], json!(3));
        assert_eq!(value["recorded"], json!(3));
        assert_eq!(value["truncated"], json!(false));
        // p95 nearest-rank of [8,8,24]ms -> rank ceil(0.95*3)=3 -> 24ms.
        assert_eq!(value["clean_p95_ns"], json!(24_000_000u64));
        // Each sample carries its receipt.
        let receipts: Vec<_> = value["samples"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["receipt"].as_u64().unwrap())
            .collect();
        assert_eq!(receipts, vec![101, 102, 103]);
        // Mean (8+8+24)/3 = 13.33ms.
        let mean = value["clean_mean_ns"].as_f64().unwrap();
        assert!((mean - 13_333_333.33).abs() < 1.0);
    }

    #[test]
    fn mixed_failed_and_cancelled_sequence_maps_clean_samples_to_receipts() {
        // A representative sequence: normal, normal, failed(no receipt),
        // cancelled(no receipt), normal. Only the three real observations are
        // clean, each tied to its receipt; the two gaps advance the baseline.
        let mut rec = CpuIntervalRecorder::with_capacity(16).unwrap();
        rec.push_interval_ns(0, Some(200), 0, 0); // seed
        rec.push_interval_ns(1, Some(201), 0, 8_000_000); // clean 8ms
        rec.push_unobserved_ns(2, 0, 20_000_000); // failed: +12ms gap, baseline->20ms
        rec.push_unobserved_ns(3, 0, 25_000_000); // cancelled: +5ms gap, baseline->25ms
        rec.push_interval_ns(4, Some(204), 0, 34_000_000); // clean 9ms from frame3
        let clean: Vec<_> = rec
            .samples()
            .iter()
            .filter(|s| s.kind == SampleKind::Clean)
            .map(|s| (s.receipt, s.cpu_ns))
            .collect();
        assert_eq!(
            clean,
            vec![(Some(201), 8_000_000), (Some(204), 9_000_000)],
            "only real observations are clean, each tied to its receipt; gap CPU is not folded in"
        );
        // Two unobserved gaps recorded, no receipts.
        let unobserved = rec
            .samples()
            .iter()
            .filter(|s| s.kind == SampleKind::NotObserved)
            .count();
        assert_eq!(unobserved, 2);
    }
}


thread_local! {
    static FRAME_STAGES: std::cell::Cell<Option<[u64; 3]>> = const { std::cell::Cell::new(None) };
}
/// Owner-only fixed storage; does not publish a graph pointer to workers.
pub fn begin_stages() { FRAME_STAGES.with(|slot| slot.set(Some([0; 3]))); }
pub fn take_stages() -> Option<[u64; 3]> { FRAME_STAGES.with(|slot| slot.take()) }
pub(crate) fn stage_boundary(index: usize) {
    FRAME_STAGES.with(|slot| {
        if let Some(mut times) = slot.get() { times[index] = wall_ticks(); slot.set(Some(times)); }
    });
}
pub(crate) fn management_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("TG_CPU_BILL").is_ok_and(|value| value == "1"))
}


#[cfg(test)]
mod stage_tests {
    use super::*;
    #[test]
    fn stage_storage_is_owner_local_and_clears_after_take() {
        assert!(take_stages().is_none());
        begin_stages();
        stage_boundary(0);
        assert!(std::thread::spawn(take_stages).join().unwrap().is_none());
        let times = take_stages().unwrap();
        if cfg!(any(windows, target_os = "macos")) { assert!(times[0] > 0); }
        assert_eq!(times[1..], [0,0]);
        assert!(take_stages().is_none());
        stage_boundary(1);
        assert!(take_stages().is_none());
    }
}
