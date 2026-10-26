//! Per-frame diagnostic counters for the render-performance work
//! (reports/render-perf-architecture-2026-10-21, step 0).
//!
//! Counters are plain relaxed atomics. `take`
//! returns the totals since the previous `take` and resets them; the renderer
//! calls it once per frame from `finish_render`.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

static UPLOAD_PENDING_YIELDS: AtomicU64 = AtomicU64::new(0);
/// Totals accumulated since the previous [`take`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameCounters {
    /// Times upload admission yielded to the owner loop without waiting.
    pub upload_pending_yields: u64,
}

pub(crate) fn record_upload_pending_yield() {
    UPLOAD_PENDING_YIELDS.fetch_add(1, Relaxed);
}

/// Returns and resets the counters.
pub fn take() -> FrameCounters {
    FrameCounters {
        upload_pending_yields: UPLOAD_PENDING_YIELDS.swap(0, Relaxed),
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
    passes: std::collections::HashMap<&'static str, (u64, u64)>,
}

impl GpuSpanWindow {
    pub(crate) fn add(&mut self, duration_ns: u64, pass_name: Option<&'static str>) {
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
        record_upload_pending_yield();
        record_upload_pending_yield();
        let first = take();
        assert_eq!(first.upload_pending_yields, 2);
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

/// `TG_PROFILE_OUT=<file>` samples every thread's CPU time (997 Hz) for
/// `TG_PROFILE_SECS` (default 30) starting `TG_PROFILE_DELAY` seconds (default
/// 100) after launch, then writes collapsed stacks (`thread;frame;...;leaf N`,
/// flamegraph input) so one run supports any offline analysis.
#[cfg(feature = "profile")]
pub fn start_cpu_profiler() {
    let Some(out) = std::env::var_os("TG_PROFILE_OUT") else {
        return;
    };
    let secs = |name: &str, default: u64| {
        std::env::var(name)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(default)
    };
    let (delay, length) = (secs("TG_PROFILE_DELAY", 100), secs("TG_PROFILE_SECS", 30));
    std::thread::spawn(move || {
        use std::io::Write;
        std::thread::sleep(std::time::Duration::from_secs(delay));
        let Ok(guard) = pprof::ProfilerGuardBuilder::default()
            .frequency(997)
            .blocklist(&["libc", "libgcc", "pthread", "vdso"])
            .build()
        else {
            eprintln!("profiler failed to start");
            return;
        };
        std::thread::sleep(std::time::Duration::from_secs(length));
        let Ok(report) = guard.report().build() else {
            return;
        };
        let Ok(mut file) = std::fs::File::create(&out) else {
            return;
        };
        for (frames, count) in &report.data {
            let mut line = frames.thread_name.clone();
            for frame in frames.frames.iter().rev() {
                // Inlined frames come innermost first; keep the outermost first.
                for symbol in frame.iter().rev() {
                    line.push(';');
                    line.push_str(&symbol.name().replace(';', ":"));
                    if symbol.lineno() != 0 {
                        line.push_str(&format!(" ({}:{})", symbol.filename(), symbol.lineno()));
                    }
                }
            }
            let _ = writeln!(file, "{line} {count}");
        }
        eprintln!("profile written to {}", std::path::Path::new(&out).display());
    });
}

