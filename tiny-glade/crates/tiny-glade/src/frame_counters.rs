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

fn env_flag(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|value| value == "1")
}

/// `TG_GPU_ISOLATE=1` (with `TG_GPU_TIMING=1`): drain the GPU before every
/// pass's first timestamp so each span is that pass's own work. The frame runs
/// serialised, so spans are exclusive costs, not production timings.
pub(crate) fn gpu_isolate_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| env_flag("TG_GPU_ISOLATE"))
}

/// `TG_GPU_STATS=1` (with `TG_GPU_TIMING=1`): pipeline-statistics query per
/// pass (input vertices, fragment and compute invocations).
pub(crate) fn gpu_statistics_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| env_flag("TG_GPU_STATS"))
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
    /// Per pass: summed input vertices, fragment invocations, compute invocations.
    statistics: std::collections::HashMap<&'static str, [u64; 3]>,
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

    pub(crate) fn add_statistics(
        &mut self,
        pass_name: &'static str,
        input_vertices: u64,
        fragment_invocations: u64,
        compute_invocations: u64,
    ) {
        let entry = self.statistics.entry(pass_name).or_default();
        for (total, add) in entry.iter_mut().zip([input_vertices, fragment_invocations, compute_invocations]) {
            *total = total.saturating_add(add);
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
                let mut entry = format!("{name}={:.3}ms/frame(x{:.1})", ms(*ns) / frames, *n as f64 / frames);
                if let Some([vertices, fragments, compute]) = self.statistics.get(*name) {
                    // Per-frame averages; fragments in millions, the others in thousands.
                    entry.push_str(&format!(
                        "[vert={:.1}k frag={:.2}M comp={:.1}k]",
                        *vertices as f64 / frames / 1e3,
                        *fragments as f64 / frames / 1e6,
                        *compute as f64 / frames / 1e3,
                    ));
                }
                entry
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

    #[test]
    fn gpu_window_appends_statistics_per_frame_and_resets_them() {
        let mut window = GpuSpanWindow::default();
        for _ in 0..2 {
            window.add(4_000_000, Some("G-buffer"));
            window.add_statistics("G-buffer", 3_000, 2_000_000, 0);
            window.add(1_000_000, Some("DOF"));
            let _ = window.end_frame(3);
        }
        window.add(4_000_000, Some("G-buffer"));
        window.add_statistics("G-buffer", 3_000, 2_000_000, 500);
        let line = window.end_frame(3).expect("window closes on the third frame");
        assert!(line.contains("G-buffer=4.000ms/frame(x1.0)[vert=3.0k frag=2.00M comp=0.2k]"), "{line}");
        // A pass without statistics keeps the plain format.
        assert!(line.contains("DOF=0.667ms/frame(x0.7),") || line.ends_with("DOF=0.667ms/frame(x0.7)]; pass spans overlap command spans"), "{line}");
        window.add(1_000_000, Some("G-buffer"));
        let next = window.end_frame(1).unwrap();
        assert!(!next.contains("vert="), "statistics must reset with the window: {next}");
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

