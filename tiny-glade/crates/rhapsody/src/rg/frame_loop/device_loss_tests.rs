use super::*;
use crate::sync::{
    QueueId, SyncSet,
    timeline::{TimelineApi, Timelines},
};
use ash::vk::Handle;

struct NoGpuCalls;
impl TimelineApi for NoGpuCalls {
    fn counter_value(&self, _: vk::Semaphore) -> Result<u64> {
        panic!("a poisoned frame loop must not query the GPU")
    }
    fn wait_all(&self, _: &[vk::Semaphore], _: &[u64], _: u64) -> Result<bool> {
        panic!("a poisoned frame loop must not wait on the GPU")
    }
}
fn timelines() -> Timelines<NoGpuCalls> {
    Timelines::new(NoGpuCalls, &[vk::Semaphore::from_raw(1)]).unwrap()
}

#[test]
fn frame_status_requires_an_actual_generation_change_to_report_recreation() {
    assert_eq!(frame_status(10, 10, false), FrameStatus::Skipped);
    assert_eq!(frame_status(10, 10, true), FrameStatus::Presented);
    assert_eq!(frame_status(10, 11, false), FrameStatus::Recreated);
    assert_eq!(frame_status(10, 11, true), FrameStatus::Recreated);
}

#[test]
fn observed_display_or_cleanup_device_loss_stops_all_queue_operations() {
    let timelines = timelines();
    let error = observe_device_result::<_, ()>(&timelines, Err(Error::DeviceLost)).unwrap_err();
    assert!(error.is_device_lost());
    assert!(timelines.is_device_lost());
    let queue = QueueId::new(0).unwrap();
    assert!(
        timelines
            .submit(queue, |_| -> Result<()> {
                panic!("a display error must prevent the next submit")
            })
            .unwrap_err()
            .is_device_lost()
    );
    assert!(
        timelines
            .with_queue(queue, || -> Result<()> {
                panic!("a display error must prevent the next present")
            })
            .unwrap_err()
            .is_device_lost()
    );
    assert!(timelines.poll().unwrap_err().is_device_lost());
    assert!(
        timelines
            .wait(&SyncSet::EMPTY, 0)
            .unwrap_err()
            .is_device_lost()
    );
    // A later successful CPU cleanup cannot clear the terminal state.
    observe_device_result(&timelines, Ok(())).unwrap();
    assert!(timelines.is_device_lost());
}

#[test]
fn recoverable_display_errors_keep_the_original_error_and_allow_the_next_submit() {
    for original in [
        Error::Timeout,
        Error::OutOfHostMemory,
        Error::SurfaceOutOfDate,
    ] {
        let timelines = timelines();
        let kind = original.kind();
        let result = observe_device_result::<_, ()>(&timelines, Err(original));
        assert_eq!(result.unwrap_err().kind(), kind);
        assert!(!timelines.is_device_lost());
        let (point, ()) = timelines
            .submit(QueueId::new(0).unwrap(), |_| Ok(()))
            .unwrap();
        assert_eq!(point.value, 1);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Failure {
    NoImage,
    Acquire,
    GraphFirstSubmit,
    GraphPartialSubmit,
    PresentationGraph,
    PresentationPanic,
    Present,
}
struct ScriptedOperations {
    timelines: Timelines<NoGpuCalls>,
    failure: Option<Failure>,
    inject_device_loss: bool,
    events: Vec<&'static str>,
    pending_retire: Vec<super::super::graph::RetireCallback>,
}
impl ScriptedOperations {
    fn new(failure: Failure) -> Self {
        Self {
            timelines: timelines(),
            failure: Some(failure),
            inject_device_loss: false,
            events: Vec::new(),
            pending_retire: Vec::new(),
        }
    }
    fn take_failure(&mut self, failure: Failure) -> bool {
        if self.failure == Some(failure) {
            self.failure = None;
            true
        } else {
            false
        }
    }
    fn error(&self) -> Error {
        if self.inject_device_loss {
            Error::DeviceLost
        } else {
            Error::OutOfDeviceMemory
        }
    }
    fn complete_graphs(&mut self) {
        super::super::vulkan::run_retire_callbacks(std::mem::take(&mut self.pending_retire))
            .unwrap();
    }
}
impl FrameOperations for ScriptedOperations {
    type Lease = ();
    type GraphReceipt = ();
    type Target = ();
    type PresentationReceipt = ();
    fn is_device_lost(&self) -> bool {
        self.timelines.is_device_lost()
    }
    fn observe_error(&mut self, error: &Error) {
        if error.is_device_lost() {
            self.timelines.mark_device_lost();
        }
    }
    fn acquire(&mut self) -> Result<Option<()>> {
        self.events.push("acquire");
        if self.take_failure(Failure::NoImage) {
            Ok(None)
        } else if self.take_failure(Failure::Acquire) {
            Err(self.error())
        } else {
            Ok(Some(()))
        }
    }
    fn execute_graph(&mut self, built: BuiltGraph) -> Result<()> {
        self.events.push("graph");
        let (_, _, retire, declaration_error) = built.into_parts();
        if let Some(error) = declaration_error {
            let _ = super::super::vulkan::run_retire_callbacks(retire);
            return Err(error);
        }
        if self.take_failure(Failure::GraphFirstSubmit) {
            let _ = super::super::vulkan::run_retire_callbacks(retire);
            return Err(self.error());
        }
        // This is the Session's ownership boundary, not another GPU
        // transaction model. Both successful and partial submits retain
        // their callbacks until an actual completion is supplied.
        self.pending_retire.extend(retire);
        if self.take_failure(Failure::GraphPartialSubmit) {
            Err(self.error())
        } else {
            Ok(())
        }
    }
    fn render_presentation(&mut self, _: ()) -> Result<((), ())> {
        self.events.push("presentation_graph");
        assert!(
            !self.take_failure(Failure::PresentationPanic),
            "injected presentation panic"
        );
        if self.take_failure(Failure::PresentationGraph) {
            // The session returns the image before reporting the error.
            self.events.push("return_image");
            Err(self.error())
        } else {
            Ok(((), ()))
        }
    }
    fn present(&mut self, _: ()) -> Result<()> {
        self.events.push("present");
        if self.take_failure(Failure::Present) {
            Err(self.error())
        } else {
            Ok(())
        }
    }
}
/// `Cell<usize>`-like counter that retire callbacks (now `Send`) may capture.
struct RetireCount(std::sync::atomic::AtomicUsize);
impl RetireCount {
    fn new(value: usize) -> Self {
        Self(std::sync::atomic::AtomicUsize::new(value))
    }
    fn get(&self) -> usize {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
    fn set(&self, value: usize) {
        self.0.store(value, std::sync::atomic::Ordering::Relaxed)
    }
}
fn graph_with_retire(calls: &std::sync::Arc<RetireCount>) -> BuiltGraph {
    let mut graph = super::super::graph::RenderGraph::new();
    let calls = calls.clone();
    graph.on_retire(move || calls.set(calls.get() + 1));
    graph.build()
}

fn run(operations: &mut ScriptedOperations, graph: BuiltGraph) -> Result<Option<()>> {
    execute_frame_protocol(operations, graph)
        .and_then(|output| output.presentation)
        .map(|presented| presented.map(|_| ()))
}

#[test]
fn graph_is_submitted_before_acquire_and_survives_skipped_presentation() {
    let calls = std::sync::Arc::new(RetireCount::new(0));
    let mut operations = ScriptedOperations::new(Failure::NoImage);
    let output = execute_frame_protocol(&mut operations, graph_with_retire(&calls)).unwrap();
    assert!(matches!(output.presentation, Ok(None)));
    assert_eq!(operations.events, ["graph", "acquire"]);
    assert_eq!(calls.get(), 0, "a skipped frame keeps its submitted graph");
    operations.complete_graphs();
    assert_eq!(calls.get(), 1);
    operations.events.clear();
    assert!(
        run(&mut operations, graph_with_retire(&calls))
            .unwrap()
            .is_some()
    );
    assert_eq!(
        operations.events,
        ["graph", "acquire", "presentation_graph", "present"]
    );
}

#[test]
fn production_frame_protocol_fault_matrix_preserves_retirement_and_recovers_next_frame() {
    for failure in [
        Failure::NoImage,
        Failure::Acquire,
        Failure::GraphFirstSubmit,
        Failure::GraphPartialSubmit,
        Failure::PresentationGraph,
        Failure::PresentationPanic,
        Failure::Present,
    ] {
        let calls = std::sync::Arc::new(RetireCount::new(0));
        let mut operations = ScriptedOperations::new(failure);
        let output = execute_frame_protocol(&mut operations, graph_with_retire(&calls));
        let graph_failed = matches!(
            failure,
            Failure::GraphFirstSubmit | Failure::GraphPartialSubmit
        );
        assert_eq!(output.is_err(), graph_failed, "{failure:?}");
        let result = output
            .and_then(|output| output.presentation)
            .map(|presented| presented.map(|_| ()));
        if failure == Failure::NoImage {
            assert_eq!(result.unwrap(), None);
        } else if failure == Failure::PresentationPanic {
            assert_eq!(result.unwrap_err().kind(), crate::ErrorKind::InvalidUsage);
        } else {
            assert!(
                matches!(result, Err(Error::OutOfDeviceMemory)),
                "{failure:?}"
            );
        }
        // A failed graph never acquires; every later stage keeps its graph.
        assert_eq!(operations.events.contains(&"acquire"), !graph_failed);
        let unsubmitted = failure == Failure::GraphFirstSubmit;
        assert_eq!(calls.get(), usize::from(unsubmitted), "{failure:?}");
        // A failed presentation graph never reaches present.
        let presentation_failed = matches!(
            failure,
            Failure::PresentationGraph | Failure::PresentationPanic
        );
        assert_eq!(
            operations.events.contains(&"present"),
            !graph_failed
                && !presentation_failed
                && !matches!(failure, Failure::NoImage | Failure::Acquire),
            "{failure:?}"
        );
        operations.complete_graphs();
        assert_eq!(
            calls.get(),
            1,
            "the first frame retires exactly once: {failure:?}"
        );
        assert!(
            run(&mut operations, graph_with_retire(&calls))
                .unwrap()
                .is_some()
        );
        assert_eq!(calls.get(), 1, "submitted work remains retained");
        operations.complete_graphs();
        operations.complete_graphs();
        assert_eq!(
            calls.get(),
            2,
            "next frame retires exactly once: {failure:?}"
        );
    }
}

#[test]
fn every_device_loss_stage_stops_the_next_frame_without_rescue_submission() {
    for failure in [
        Failure::Acquire,
        Failure::GraphFirstSubmit,
        Failure::GraphPartialSubmit,
        Failure::PresentationGraph,
        Failure::Present,
    ] {
        let calls = std::sync::Arc::new(RetireCount::new(0));
        let mut operations = ScriptedOperations::new(failure);
        operations.inject_device_loss = true;
        assert!(
            run(&mut operations, graph_with_retire(&calls))
                .unwrap_err()
                .is_device_lost()
        );
        assert!(operations.is_device_lost());
        assert!(
            !operations.events.contains(&"present") || failure == Failure::Present,
            "no present after device loss: {failure:?}"
        );
        let before = operations.events.len();
        assert!(
            run(&mut operations, graph_with_retire(&calls))
                .unwrap_err()
                .is_device_lost()
        );
        assert_eq!(operations.events.len(), before);
        operations.complete_graphs();
        assert_eq!(calls.get(), 2);
    }
}

#[test]
fn device_loss_keeps_its_error_even_when_an_unsubmitted_retire_callback_panics() {
    let calls = std::sync::Arc::new(RetireCount::new(0));
    let mut graph = super::super::graph::RenderGraph::new();
    graph.on_retire(|| panic!("injected unsubmitted retirement panic"));
    let retired = calls.clone();
    graph.on_retire(move || retired.set(retired.get() + 1));
    let mut operations = ScriptedOperations::new(Failure::NoImage);
    operations.timelines.mark_device_lost();
    let error = run(&mut operations, graph.build()).unwrap_err();
    assert!(error.is_device_lost());
    assert_eq!(calls.get(), 1);
    assert!(operations.events.is_empty());
}
