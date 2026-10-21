//! Per-frame diagnostic counters for the render-performance work
//! (reports/render-perf-architecture-2026-10-21, step 0).
//!
//! Counters are plain relaxed atomics. Wall-clock sampling happens only when
//! `TG_FRAME_TIMING=1`, so a disabled run adds no `Instant` reads. `take`
//! returns the totals since the previous `take` and resets them; the renderer
//! calls it once per frame from `finish_render`.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::{Duration, Instant};

static UPLOAD_WAIT_CALLS: AtomicU64 = AtomicU64::new(0);
static UPLOAD_WAIT_NS: AtomicU64 = AtomicU64::new(0);
static UPLOAD_WAIT_BYTES: AtomicU64 = AtomicU64::new(0);
static PREWARM_CALLS: AtomicU64 = AtomicU64::new(0);
static PREWARM_TICKETS: AtomicU64 = AtomicU64::new(0);
static PREWARM_NS: AtomicU64 = AtomicU64::new(0);

/// Totals accumulated since the previous [`take`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameCounters {
    /// Synchronous `upload_buffer_ranges_wait` calls (each waits on a GPU fence).
    pub upload_wait_calls: u64,
    pub upload_wait_ns: u64,
    pub upload_wait_bytes: u64,
    /// `ShaderHost::prewarm` invocations.
    pub prewarm_calls: u64,
    /// Tickets scanned across those invocations.
    pub prewarm_tickets: u64,
    pub prewarm_ns: u64,
}

fn enabled() -> bool {
    crate::game_loop::frame_timing_enabled()
}

/// Starts a wall-clock sample only when frame timing is enabled.
pub(crate) fn start() -> Option<Instant> {
    enabled().then(Instant::now)
}

pub(crate) fn record_upload_wait(started: Option<Instant>, bytes: usize) {
    UPLOAD_WAIT_CALLS.fetch_add(1, Relaxed);
    UPLOAD_WAIT_BYTES.fetch_add(bytes as u64, Relaxed);
    if let Some(started) = started {
        UPLOAD_WAIT_NS.fetch_add(nanos(started.elapsed()), Relaxed);
    }
}

pub(crate) fn record_prewarm(started: Option<Instant>, tickets: usize) {
    PREWARM_CALLS.fetch_add(1, Relaxed);
    PREWARM_TICKETS.fetch_add(tickets as u64, Relaxed);
    if let Some(started) = started {
        PREWARM_NS.fetch_add(nanos(started.elapsed()), Relaxed);
    }
}

fn nanos(duration: Duration) -> u64 {
    duration.as_nanos().min(u128::from(u64::MAX)) as u64
}

/// Returns and resets the counters.
pub fn take() -> FrameCounters {
    FrameCounters {
        upload_wait_calls: UPLOAD_WAIT_CALLS.swap(0, Relaxed),
        upload_wait_ns: UPLOAD_WAIT_NS.swap(0, Relaxed),
        upload_wait_bytes: UPLOAD_WAIT_BYTES.swap(0, Relaxed),
        prewarm_calls: PREWARM_CALLS.swap(0, Relaxed),
        prewarm_tickets: PREWARM_TICKETS.swap(0, Relaxed),
        prewarm_ns: PREWARM_NS.swap(0, Relaxed),
    }
}

/// `TG_GPU_TIMING=1` enables per-pass GPU timestamps (needs a driver with
/// timestamp support). Off by default; disabled runs record no queries.
pub(crate) fn gpu_timing_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("TG_GPU_TIMING").is_some_and(|value| value == "1"))
}

/// Aggregates GPU timestamp samples over a window of frames.
///
/// Whole-command spans are summed per frame (single graphics queue, so this
/// approximates GPU time but is not multi-queue latency). Pass spans overlap
/// the command spans and are reported separately, never added to them.
#[derive(Default)]
pub(crate) struct GpuSpanWindow {
    frames: u64,
    command_spans: u64,
    command_ns: u64,
    passes: std::collections::HashMap<String, (u64, u64)>,
}

impl GpuSpanWindow {
    pub(crate) fn add(&mut self, duration_ns: u64, pass_name: Option<String>) {
        match pass_name {
            None => {
                self.command_spans += 1;
                self.command_ns += duration_ns;
            }
            Some(name) => {
                let entry = self.passes.entry(name).or_default();
                entry.0 += duration_ns;
                entry.1 += 1;
            }
        }
    }

    /// Counts a frame and, every `every` frames, returns the log line and resets.
    pub(crate) fn end_frame(&mut self, every: u64) -> Option<String> {
        self.frames += 1;
        if self.frames < every {
            return None;
        }
        let frames = self.frames as f64;
        let ms = |ns: u64| ns as f64 / 1_000_000.0;
        let mut top: Vec<_> = self.passes.iter().collect();
        top.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
        let top = top
            .iter()
            .take(20)
            .map(|(name, (ns, n))| {
                format!("{name}={:.3}ms/frame(x{:.1})", ms(*ns) / frames, *n as f64 / frames)
            })
            .collect::<Vec<_>>()
            .join(", ");
        let line = format!(
            "gpu_spans window_frames={} command_spans_per_frame={:.1} command_sum_ms_per_frame={:.3} top_passes=[{}]; pass spans overlap command spans",
            self.frames,
            self.command_spans as f64 / frames,
            ms(self.command_ns) / frames,
            top,
        );
        *self = Self::default();
        Some(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_returns_totals_and_resets() {
        let _ = take();
        record_upload_wait(None, 100);
        record_upload_wait(None, 28);
        record_prewarm(None, 7);
        let first = take();
        assert_eq!(first.upload_wait_calls, 2);
        assert_eq!(first.upload_wait_bytes, 128);
        assert_eq!(first.prewarm_calls, 1);
        assert_eq!(first.prewarm_tickets, 7);
        assert_eq!(take(), FrameCounters::default());
    }

    #[test]
    fn gpu_window_reports_every_n_frames_and_resets() {
        let mut window = GpuSpanWindow::default();
        window.add(2_000_000, None);
        window.add(1_500_000, Some("Shadows".into()));
        window.add(500_000, Some("Shadows".into()));
        assert!(window.end_frame(2).is_none());
        window.add(2_000_000, None);
        let line = window.end_frame(2).expect("window closes on the second frame");
        assert!(line.contains("command_sum_ms_per_frame=2.000"), "{line}");
        assert!(line.contains("Shadows=1.000ms/frame(x1.0)"), "{line}");
        assert!(window.end_frame(2).is_none(), "window must reset");
    }
}
