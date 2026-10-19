mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::semaphore::TimelineSemaphore,
    rg::{
        profiling::GpuTimingConfig,
        vulkan::{AccessTable, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;
struct Unblock(Arc<TimelineSemaphore>, bool);
impl Unblock {
    fn signal(&mut self) {
        if self.1 {
            return;
        }
        unsafe {
            self.0
                .device
                .device
                .signal_semaphore(
                    &vk::SemaphoreSignalInfo::default()
                        .semaphore(self.0.handle)
                        .value(1),
                )
                .unwrap();
        }
        self.1 = true;
    }
}
impl Drop for Unblock {
    fn drop(&mut self) {
        self.signal();
    }
}
fn session() -> Option<VulkanGraphSession> {
    let device = gpu_test_support::initialize(
        "GPU timing device",
        Device::new_headless(&DeviceConfig::headless()),
    )?;
    let mut backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    backend
        .configure_gpu_timing(Some(GpuTimingConfig {
            pass_spans: false,
            pending_commands: 1,
            retained_samples: 2,
        }))
        .unwrap();
    Some(VulkanGraphSession::new(backend))
}
#[test]
fn query_slot_is_not_reset_until_actual_submission_completes_and_capacity_is_nonblocking() {
    let Some(mut session) = session() else {
        return;
    };
    let device = session.backend.device.clone();
    let gate = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let mut unblock = Unblock(gate.clone(), false);
    let mut graph = session.begin_frame().unwrap();
    graph
        .add_pass("first pending span")
        .with_side_effect()
        .render(|_| true);
    session
        .backend
        .set_graph_synchronization(
            vec![(gate, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            vec![],
        )
        .unwrap();
    let first = session.execute_frame(graph.build()).unwrap();
    let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
    assert_eq!(stats.pending, 1);
    assert_eq!(stats.completed, 0);
    assert_eq!(session.backend.gpu_timings().count(), 0);
    assert!(session.backend.configure_gpu_timing(None).is_err());
    let mut graph = session.begin_frame().unwrap();
    graph
        .add_pass("second does not overwrite pending query")
        .with_side_effect()
        .render(|_| true);
    session.execute_frame(graph.build()).unwrap();
    let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
    assert_eq!(stats.pending, 1);
    assert_eq!(stats.capacity_skips, 1);
    assert_eq!(stats.completed, 0);
    unblock.signal();
    session.wait_submits_done(10_000_000_000).unwrap();
    let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
    assert_eq!(stats.pending, 0);
    assert_eq!(stats.completed, 1);
    let samples = session.backend.drain_gpu_timings().collect::<Vec<_>>();
    assert_eq!(samples.len(), 1);
    assert_eq!(samples[0].queue, first.receipts[0].queue.index());
    assert_eq!(samples[0].completion_value, first.receipts[0].value);
    // Repeated frames recycle one query pair, rather than creating per-frame pools.
    for _ in 0..5 {
        let mut graph = session.begin_frame().unwrap();
        graph
            .add_pass("recycled timing pair")
            .with_side_effect()
            .render(|_| true);
        session.execute_frame(graph.build()).unwrap();
        session.wait_submits_done(10_000_000_000).unwrap();
    }
    let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
    assert_eq!(stats.pending, 0);
    assert_eq!(stats.peak_pending, 1);
    assert_eq!(stats.completed, 6);
    assert_eq!(stats.retained_samples, 2);
    session.backend.configure_gpu_timing(None).unwrap();
    assert_eq!(device.garbage_stats().idle_waits, 0);
}
#[test]
fn failed_recording_cancels_unsubmitted_query_and_next_frame_can_reuse_it() {
    let Some(mut session) = session() else {
        return;
    };
    let mut graph = session.begin_frame().unwrap();
    graph
        .add_pass("failed callback")
        .with_side_effect()
        .render(|_| false);
    assert!(session.execute_frame(graph.build()).is_err());
    assert_eq!(
        session.backend.poll_gpu_timings().unwrap().unwrap().pending,
        0
    );
    let mut graph = session.begin_frame().unwrap();
    graph
        .add_pass("recovered callback")
        .with_side_effect()
        .render(|_| true);
    session.execute_frame(graph.build()).unwrap();
    session.wait_submits_done(10_000_000_000).unwrap();
    let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
    assert_eq!(stats.completed, 1);
    assert_eq!(stats.capacity_skips, 0);
}

#[test]
fn per_pass_pairs_follow_actual_serial_and_worker_submissions_and_cancel_as_a_unit() {
    use rhapsody::rg::parallel::ParallelRecording;
    for parallel in [ParallelRecording::SERIAL, ParallelRecording::EVERY_PASS] {
        let config = DeviceConfig {
            recording: rhapsody::device::RecordingPolicy::Parallel,
            ..DeviceConfig::headless()
        };
        let Some(device) = gpu_test_support::initialize(
            "per-pass serial/worker timing",
            Device::new_headless(&config),
        ) else {
            return;
        };
        let mut session = VulkanGraphSession::new(
            VulkanGraphBackend::new(device, AccessTable::original()).unwrap(),
        );
        session.chain.parallel = parallel;
        session
            .backend
            .configure_gpu_timing(Some(GpuTimingConfig {
                pass_spans: true,
                pending_commands: 32,
                retained_samples: 8,
            }))
            .unwrap();
        let mut failed = session.begin_frame().unwrap();
        for (index, name) in ["first", "rejected", "unrecorded"].into_iter().enumerate() {
            failed
                .add_pass(name)
                .with_side_effect()
                .render(move |_| index != 1);
        }
        assert!(session.execute_frame(failed.build()).is_err());
        assert_eq!(
            session.backend.poll_gpu_timings().unwrap().unwrap().pending,
            0
        );
        // The next frame reuses every cancelled pair. Both serial and actual
        // worker commands publish their own queue completion, never a CPU value.
        let mut graph = session.begin_frame().unwrap();
        for name in ["first", "second", "third"] {
            graph.add_pass(name).with_side_effect().render(|_| true);
        }
        session.chain.pgo.previous.reset();
        let frame = session.execute_frame(graph.build()).unwrap();
        if parallel == ParallelRecording::EVERY_PASS {
            assert!(frame.stats.commands_recorded > 1);
        }
        session.wait_submits_done(10_000_000_000).unwrap();
        let samples: Vec<_> = session.backend.drain_gpu_timings().collect();
        let mut passes: Vec<_> = samples
            .iter()
            .filter_map(|sample| sample.pass_index)
            .collect();
        passes.sort_unstable();
        assert_eq!(passes, [0, 1, 2]);
        assert_eq!(
            samples
                .iter()
                .filter(|sample| sample.pass_index.is_none())
                .count(),
            frame.stats.commands_recorded as usize
        );
        for sample in &samples {
            assert!(
                frame
                    .receipts
                    .iter()
                    .any(|point| point.queue.index() == sample.queue
                        && point.value == sample.completion_value)
            );
        }
        let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
        assert_eq!(stats.pending, 0);
        assert_eq!(stats.capacity_skips, 0);
        session.backend.configure_gpu_timing(None).unwrap();
    }
}

#[test]
fn in_flight_pass_queries_keep_their_slots_until_the_actual_timeline_reaches_them() {
    let Some(mut session) = session() else { return };
    session
        .backend
        .configure_gpu_timing(Some(GpuTimingConfig {
            pass_spans: true,
            pending_commands: 2,
            retained_samples: 4,
        }))
        .unwrap();
    let gate = TimelineSemaphore::new(session.backend.device.clone(), 0).unwrap();
    let mut unblock = Unblock(gate.clone(), false);
    let mut graph = session.begin_frame().unwrap();
    session
        .backend
        .set_graph_synchronization(
            vec![(gate, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            vec![],
        )
        .unwrap();
    graph
        .add_pass("blocked pass pair")
        .with_side_effect()
        .render(|_| true);
    let first = session.execute_frame(graph.build()).unwrap();
    let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
    assert_eq!((stats.pending, stats.completed), (2, 0));
    assert!(session.backend.configure_gpu_timing(None).is_err());
    let mut next = session.begin_frame().unwrap();
    next.add_pass("capacity pressure must not reset an in-flight pass pair")
        .with_side_effect()
        .render(|_| true);
    session.execute_frame(next.build()).unwrap();
    let stats = session.backend.poll_gpu_timings().unwrap().unwrap();
    assert_eq!(
        (stats.pending, stats.completed, stats.capacity_skips),
        (2, 0, 1)
    );
    unblock.signal();
    session.wait_submits_done(10_000_000_000).unwrap();
    let samples: Vec<_> = session.backend.drain_gpu_timings().collect();
    assert_eq!(samples.len(), 2);
    assert_eq!(
        samples
            .iter()
            .filter(|sample| sample.pass_index == Some(0))
            .count(),
        1
    );
    assert!(samples.iter().all(
        |sample| {
            first.receipts.iter().any(|point| {
                point.queue.index() == sample.queue && point.value == sample.completion_value
            })
        }
    ));
    assert_eq!(
        session.backend.poll_gpu_timings().unwrap().unwrap().pending,
        0
    );
}
