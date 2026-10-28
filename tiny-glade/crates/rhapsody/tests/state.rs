//! 图回调与退役边界测试；跨帧完整合同见 temporal 测试。
use rhapsody::rg::graph::RenderGraph;
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
    graph.add_pass("run").with_side_effect().render(move |_| {
        calls.fetch_add(1, Ordering::Relaxed);
        true
    });
    let (decl, mut callbacks, _, declaration_error) = graph.build().into_parts();
    assert!(declaration_error.is_none());
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
        fn access_mask(&self, _: u8) -> ash::vk::AccessFlags {
            ash::vk::AccessFlags::empty()
        }
        fn is_write(&self, _: u8) -> bool {
            false
        }
    }
    let compiled = rhapsody::rg::graph::compile_graph(
        decl,
        &mut Default::default(),
        &mut Fixture,
        &Fixture,
    )
    .unwrap();
    use rhapsody::command::recording::Queue;
    let mut chain = rhapsody::rg::executor::ExecutionChain::new(ash::vk::Queue::null(), vec![], 1);
    let command = chain.begin(10, 0, Queue::Primary, Ok(())).unwrap();
    let owners = std::cell::RefCell::new(rhapsody::rg::owner_storage::ResourceOwners::default());
    let errors = std::cell::RefCell::new(None);
    #[cfg(feature = "cpu-probe")]
    let command_workload = std::cell::Cell::new(rhapsody::profile::CommandWorkload::default());
    let context = || rhapsody::rg::executor::PassContext {
        #[cfg(feature = "cpu-probe")]
        command_workload: &command_workload,
        cpu: None,
        pass_index: 0,
        command: &command,
        compiled: &compiled,
        managed: None,
        owners: &owners,
        errors: &errors,
        coverage: Default::default(),
    };
    assert!(callbacks[0](context()));
    assert!(!callbacks[0](context()));
    assert_eq!(count.load(Ordering::Relaxed), 1);
}
