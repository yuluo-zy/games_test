//! 图回调与退役边界测试；跨帧完整合同见 temporal 测试。
use rhapsody::{raw::disposal::CommandBufferDisposalBuckets, rg::graph::RenderGraph};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
#[test]
fn graph_passes_commit_atomically_and_callbacks_are_consumed_once() {
    let mut graph = RenderGraph::new();
    let count = Arc::new(AtomicUsize::new(0));
    drop(graph.add_pass("discard"));
    let calls = count.clone();
    graph.add_pass("run").render(move |_| {
        calls.fetch_add(1, Ordering::Relaxed);
        true
    });
    let (decl, mut callbacks, _) = graph.build().into_parts();
    assert_eq!(decl.passes.len(), 1);
    struct Fixture;
    impl rhapsody::rg::graph::ResourceBackend for Fixture {
        fn create_without_initial_data(
            &mut self,
            _: rhapsody::rg::graph::Descriptor,
            _: &str,
        ) -> Result<u64, rhapsody::rg::graph::BackendFailure> {
            unreachable!()
        }
        fn set_debug_name(&mut self, _: u64, _: rhapsody::rg::graph::Descriptor, _: &str) {}
    }
    impl rhapsody::rg::graph::AccessDependency for Fixture {
        fn access_mask(&mut self, _: u8) -> u32 {
            0
        }
        fn is_write(&mut self, _: u8) -> bool {
            false
        }
    }
    let compiled = rhapsody::rg::graph::compile_graph(
        decl,
        &mut Default::default(),
        &mut Fixture,
        &mut Fixture,
    )
    .unwrap();
    use rhapsody::raw::managed::lifecycle::{Command, DeviceCounters, Queue, SequenceRuntime};
    let command = SequenceRuntime::new(1)
        .begin_after_backend(
            &DeviceCounters::new(0),
            Command::new(10, 0),
            Queue::Primary,
            Ok(()),
        )
        .unwrap();
    let owners = std::cell::RefCell::new(vec![]);
    let context = || rhapsody::rg::executor::PassContext {
        pass_index: 0,
        command: &command,
        compiled: &compiled,
        logical_resources: &[],
        managed: None,
        owners: &owners,
    };
    assert!(callbacks[0](context()));
    assert!(!callbacks[0](context()));
    assert_eq!(count.load(Ordering::Relaxed), 1);
}
#[test]
fn retirement_waits_for_completed_threshold_in_order() {
    // 原语义：每个命令先 begin 桶，完成按序号经 SequentialDisposal 连续推进，乱序完成等待。
    let values = Arc::new(std::sync::Mutex::new(vec![]));
    let buckets = CommandBufferDisposalBuckets::default();
    let mut bucket_ids = [1u64, 2, 3];
    bucket_ids.reverse();
    for sequence in 1..=3u64 {
        buckets.begin(sequence);
        let values = values.clone();
        assert!(
            buckets
                .append_existing(
                    sequence,
                    Box::new(move || values.lock().unwrap().push(sequence))
                )
                .is_ok()
        );
    }
    let mut sequential = rhapsody::raw::disposal::SequentialDisposal::new();
    let mut retire = |sequence: u64| {
        buckets.remove(sequence).dispose();
    };
    sequential.dispose_in_order(3, &mut retire);
    sequential.dispose_in_order(2, &mut retire);
    assert_eq!(buckets.pending(), 3, "1 未完成，2/3 必须等待");
    sequential.dispose_in_order(1, &mut retire);
    assert_eq!(buckets.pending(), 0);
    assert_eq!(*values.lock().unwrap(), vec![1, 2, 3]);
    assert_eq!(sequential.last_retired(), 3);
}
