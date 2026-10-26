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
//! The recorder is compiled only for profile builds (`feature = "profile"`), so
//! normal builds carry no recorder type and no clock overhead. The actual
//! `CLOCK_THREAD_CPUTIME_ID` read is additionally `target_os = "macos"` (the
//! only host with that owner-CPU path and the QoS policy this measurement
//! isolates); on other hosts `record_observed`/`record_unobserved` report a
//! clock error. The pure bookkeeping (buffer bounds, baseline reset, receipt
//! association, JSON shape) is unit-testable on any host through
//! [`CpuIntervalRecorder::push_interval_ns`], which does not touch the clock.
#![cfg(feature = "cpu-probe")]

use serde_json::{Value, json};

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

    /// Read the thread CPU clock. Returns cumulative owner CPU nanoseconds, or
    /// `None` on a `clock_gettime` failure. Only macOS has the owner-CPU path;
    /// other hosts return `None` so the sample is recorded as a clock error.
    #[cfg(target_os = "macos")]
    fn read_thread_cpu_ns() -> Option<u64> {
        let mut stamp = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: `stamp` is a valid, fully-initialized timespec; clock_gettime
        // only writes through the pointer on success.
        if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut stamp) } != 0 {
            return None;
        }
        Some(stamp.tv_sec as u64 * 1_000_000_000 + stamp.tv_nsec as u64)
    }

    #[cfg(not(target_os = "macos"))]
    fn read_thread_cpu_ns() -> Option<u64> {
        None
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
            "clock": "CLOCK_THREAD_CPUTIME_ID",
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
