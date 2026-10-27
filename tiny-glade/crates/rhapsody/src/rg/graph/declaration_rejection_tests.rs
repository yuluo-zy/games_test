use super::*;
#[test]
fn first_error_survives_build_replay_and_does_not_poison_next_graph() {
    let mut graph = RenderGraph::new();
    graph.reject(crate::Error::OutOfHostMemory);
    graph.reject(crate::Error::invalid_usage("later malformed consumer"));
    let mut built = graph.build().into_graph().build();
    assert!(matches!(built.take_rejection(), Some(crate::Error::OutOfHostMemory)));
    assert!(built.take_rejection().is_none());
    let (_, _, _, rejection) = RenderGraph::new().build().into_parts();
    assert!(rejection.is_none());
}
#[test]
fn splitting_rejected_declaration_preserves_callbacks_without_executing_them() {
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    let executed = Arc::new(AtomicUsize::new(0));
    let retired = Arc::new(AtomicUsize::new(0));
    let mut graph = RenderGraph::new();
    let calls = executed.clone();
    graph.add_pass("must not execute").with_side_effect()
        .render_with_result_context(move |_| { calls.fetch_add(1, Ordering::Relaxed); Ok(()) });
    let calls = retired.clone();
    graph.on_retire(move || { calls.fetch_add(1, Ordering::Relaxed); });
    graph.reject(crate::Error::OutOfHostMemory);
    let (_, callbacks, retire, rejection) = graph.build().into_parts();
    assert!(matches!(rejection, Some(crate::Error::OutOfHostMemory)));
    drop(callbacks);
    super::super::vulkan::run_retire_callbacks(retire).unwrap();
    assert_eq!(executed.load(Ordering::Relaxed), 0);
    assert_eq!(retired.load(Ordering::Relaxed), 1);
}
