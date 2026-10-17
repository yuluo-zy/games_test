//! Bounded acceptance measurements. These are wall-clock CPU phases; waiting
//! for GPU completion and WSI are recorded separately from graph CPU phases.
use serde::Serialize;
use std::collections::VecDeque;

pub const MAX_SAMPLES: usize = 10000;
pub const COLD_START_FRAMES: usize = 300;
#[derive(Clone, Copy, Default)]
pub struct FrameSample {
    pub gpu_command_spans_ns: [Option<u64>; rhapsody::sync::MAX_QUEUES],
    pub acquire_ns: u64,
    pub display_record_ns: u64,
    pub display_submit_present_ns: u64,
    pub total_ns: u64,
    pub build_ns: u64,
    pub compile_ns: u64,
    pub record_ns: u64,
    pub submit_ns: u64,
    pub protocol_ns: u64,
    pub completion_wait_ns: u64,
    pub readback_ns: u64,
    pub reference_ns: u64,
}
#[derive(Default, Serialize)]
pub struct Distribution {
    pub samples: usize,
    pub min_ms: f64,
    pub mean_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
}
impl Distribution {
    fn from_ns(values: impl Iterator<Item = u64>) -> Self {
        let mut values: Vec<_> = values.map(|ns| ns as f64 / 1e6).collect();
        values.sort_by(f64::total_cmp);
        if values.is_empty() {
            return Self::default();
        }
        let percentile = |p: f64| values[((values.len() - 1) as f64 * p).ceil() as usize];
        Self {
            samples: values.len(),
            min_ms: values[0],
            mean_ms: values.iter().sum::<f64>() / values.len() as f64,
            p50_ms: percentile(0.50),
            p95_ms: percentile(0.95),
            p99_ms: percentile(0.99),
            max_ms: values[values.len() - 1],
        }
    }
}
#[derive(Serialize)]
pub struct PerformanceReport {
    pub cold_start: ColdStartReport,
    pub gpu_command_spans_by_queue: Vec<QueueGpuTimes>,
    pub acquire_call: Distribution,
    pub display_record: Distribution,
    pub display_submit_present: Distribution,
    pub warmup_frames: u64,
    pub requested_samples: Option<usize>,
    pub samples_seen_after_warmup: u64,
    pub sampling: &'static str,
    pub scope: &'static str,
    pub total_frame: Distribution,
    pub build_and_begin_frame: Distribution,
    pub graph_compile: Distribution,
    pub graph_record: Distribution,
    pub graph_submit: Distribution,
    pub graph_framework: Distribution,
    pub framework_including_declarations: Distribution,
    pub frame_protocol_including_wsi: Distribution,
    pub completion_wait: Distribution,
    pub host_readback: Distribution,
    pub cpu_reference: Distribution,
}
#[derive(Serialize)]
pub struct ColdStartReport {
    pub maximum_frames: usize,
    pub scope: &'static str,
    pub total_frame: Distribution,
    pub graph_framework: Distribution,
    pub gpu_command_spans_by_queue: Vec<QueueGpuTimes>,
}
#[derive(Serialize)]
pub struct QueueGpuTimes {
    pub queue_index: usize,
    pub sum_of_command_spans_per_frame: Distribution,
}
pub struct Measurements {
    warmup: u64,
    requested: Option<usize>,
    seen: u64,
    samples: VecDeque<FrameSample>,
    cold: Vec<FrameSample>,
}
impl Measurements {
    pub fn new(warmup: u64, requested: Option<usize>) -> Self {
        Self {
            warmup,
            requested,
            seen: 0,
            samples: VecDeque::with_capacity(requested.unwrap_or(MAX_SAMPLES)),
            cold: Vec::with_capacity(COLD_START_FRAMES),
        }
    }
    pub fn record(&mut self, presented_frame: u64, sample: FrameSample) {
        if self.cold.len() < COLD_START_FRAMES {
            self.cold.push(sample);
        }
        if presented_frame <= self.warmup {
            return;
        }
        self.seen += 1;
        if let Some(count) = self.requested {
            if self.samples.len() == count {
                return;
            }
        } else if self.samples.len() == MAX_SAMPLES {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }
    pub fn report(&self) -> PerformanceReport {
        let measure =
            |field: fn(&FrameSample) -> u64| Distribution::from_ns(self.samples.iter().map(field));
        PerformanceReport {
            cold_start: ColdStartReport {
                maximum_frames: COLD_START_FRAMES,
                scope: "First up to 300 presented frames, independent of warmup and steady samples. Initialization/prewarm and first-present delay are separate top-level measurements. Framework percentiles use each frame's compile+record+submit sum.",
                total_frame: Distribution::from_ns(self.cold.iter().map(|s| s.total_ns)),
                graph_framework: Distribution::from_ns(self.cold.iter().map(|s| {
                    s.compile_ns
                        .saturating_add(s.record_ns)
                        .saturating_add(s.submit_ns)
                })),
                gpu_command_spans_by_queue: (0..rhapsody::sync::MAX_QUEUES)
                    .filter_map(|queue_index| {
                        let distribution = Distribution::from_ns(
                            self.cold
                                .iter()
                                .filter_map(|s| s.gpu_command_spans_ns[queue_index]),
                        );
                        (distribution.samples != 0).then_some(QueueGpuTimes {
                            queue_index,
                            sum_of_command_spans_per_frame: distribution,
                        })
                    })
                    .collect(),
            },
            gpu_command_spans_by_queue: (0..rhapsody::sync::MAX_QUEUES)
                .filter_map(|queue_index| {
                    let distribution = Distribution::from_ns(
                        self.samples
                            .iter()
                            .filter_map(|s| s.gpu_command_spans_ns[queue_index]),
                    );
                    (distribution.samples != 0).then_some(QueueGpuTimes {
                        queue_index,
                        sum_of_command_spans_per_frame: distribution,
                    })
                })
                .collect(),
            acquire_call: measure(|s| s.acquire_ns),
            display_record: measure(|s| s.display_record_ns),
            display_submit_present: measure(|s| s.display_submit_present_ns),
            warmup_frames: self.warmup,
            requested_samples: self.requested,
            samples_seen_after_warmup: self.seen,
            sampling: if self.requested.is_some() {
                "first requested samples after warmup"
            } else {
                "latest bounded 10000 samples after warmup"
            },
            scope: "CPU wall time. Graph framework is compile+record+submit; framework_including_declarations also includes graph building and begin_frame. Frame protocol also includes acquire/present/display recording; completion wait and CPU reference are separate. GPU values are sums of command timestamp spans per physical queue; spans can overlap and exclude idle gaps, display work and WSI. Their sum is not frame latency, GPU active time, or pure vsync latency. Missing GPU measurements remain absent.",
            total_frame: measure(|s| s.total_ns),
            build_and_begin_frame: measure(|s| s.build_ns),
            graph_compile: measure(|s| s.compile_ns),
            graph_record: measure(|s| s.record_ns),
            graph_submit: measure(|s| s.submit_ns),
            graph_framework: measure(|s| {
                s.compile_ns
                    .saturating_add(s.record_ns)
                    .saturating_add(s.submit_ns)
            }),
            framework_including_declarations: measure(|s| {
                s.build_ns
                    .saturating_add(s.compile_ns)
                    .saturating_add(s.record_ns)
                    .saturating_add(s.submit_ns)
            }),
            frame_protocol_including_wsi: measure(|s| s.protocol_ns),
            completion_wait: measure(|s| s.completion_wait_ns),
            host_readback: measure(|s| s.readback_ns),
            cpu_reference: measure(|s| s.reference_ns),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cold_prefix_is_bounded_and_does_not_change_requested_warm_sampling() {
        let mut samples = Measurements::new(300, Some(3000));
        for frame in 1..=3400 {
            samples.record(
                frame,
                FrameSample {
                    total_ns: frame * 1_000_000,
                    ..Default::default()
                },
            );
        }
        let report = samples.report();
        assert_eq!(samples.cold.len(), 300);
        assert_eq!(report.cold_start.total_frame.samples, 300);
        assert_eq!(report.cold_start.total_frame.min_ms, 1.0);
        assert_eq!(report.cold_start.total_frame.max_ms, 300.0);
        assert_eq!(report.total_frame.samples, 3000);
        assert_eq!(report.total_frame.min_ms, 301.0);
        assert_eq!(report.total_frame.max_ms, 3300.0);
        assert_eq!(report.samples_seen_after_warmup, 3100);
    }
    #[test]
    fn framework_percentiles_measure_each_frame_sum_instead_of_sum_of_percentiles() {
        let mut samples = Measurements::new(0, None);
        for frame in 0..100 {
            samples.record(
                frame + 1,
                FrameSample {
                    compile_ns: frame * 1_000_000,
                    record_ns: (99 - frame) * 1_000_000,
                    ..Default::default()
                },
            );
        }
        let report = samples.report();
        assert_eq!(report.graph_framework.p99_ms, 99.0);
        assert_eq!(report.cold_start.graph_framework.p99_ms, 99.0);
        assert!(
            report.graph_compile.p99_ms + report.graph_record.p99_ms
                > report.graph_framework.p99_ms
        );
    }
}
