//! PGO full-content identity, scoped ownership and frame statistics.
use rhapsody::rg::executor::{PassPgoDuration, PassPgoFrames, PassPgoNameId, PassPgoStats};
use std::sync::Arc;
mod gpu_test_support;
#[test]
fn separately_allocated_names_compare_by_complete_content() {
    let first: Arc<str> = Arc::from("Shadows");
    let second: Arc<str> = Arc::from(format!("Shad{}", "ows"));
    assert!(!Arc::ptr_eq(&first, &second));
    let a = PassPgoNameId::from_shared(first);
    let b = PassPgoNameId::from_shared(second);
    assert_eq!(a, b);
    assert_eq!(a, PassPgoNameId::from_static("Shadows"));
    assert_ne!(a, PassPgoNameId::from_name("Shadow"));
    let mut stats = PassPgoStats::default();
    stats.record(PassPgoDuration::new(a, 40));
    stats.record(PassPgoDuration::new(b, 10));
    stats.record(PassPgoDuration::new(PassPgoNameId::from_name("Shadow"), 70));
    assert_eq!(stats.duration_ns("Shadows"), 10);
    assert_eq!(stats.duration_ns("Shadow"), 70);
}
#[test]
fn obsolete_names_are_reclaimed_when_real_double_buffer_statistics_reset() {
    let mut frames = PassPgoFrames::default();
    let mut previous = None;
    for index in 0..1000 {
        let name: Arc<str> = format!("dynamic-pass-{index}").into();
        let weak = Arc::downgrade(&name);
        frames
            .current
            .record(PassPgoDuration::new(PassPgoNameId::from_shared(name), 1));
        frames.end_frame();
        if let Some(old) = previous.replace(weak.clone()) {
            assert!(std::sync::Weak::<str>::upgrade(&old).is_none());
        }
        assert!(weak.upgrade().is_some());
    }
    frames.end_frame();
    assert!(previous.unwrap().upgrade().is_none());
}
#[test]
fn production_compilation_and_observations_share_the_declaration_name() {
    use rhapsody::rg::{
        graph::RenderGraph,
        plan::{GraphDefinition, PlanContext, compile_definition},
        vulkan::AccessTable,
    };
    let name: Arc<str> = "shared producer".into();
    let weak = Arc::downgrade(&name);
    let mut graph = RenderGraph::new();
    graph
        .add_pass(name.clone())
        .with_side_effect()
        .render(|_| true);
    let (declaration, _, _) = graph.build().into_parts();
    assert!(Arc::ptr_eq(&name, &declaration.passes[0].name));
    let definition = GraphDefinition::from_declaration(
        &declaration,
        &mut AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    assert!(Arc::ptr_eq(&name, &definition.passes[0].name));
    let plan = compile_definition(definition).unwrap();
    let mut frames = PassPgoFrames::default();
    frames.current.record(PassPgoDuration::new(
        PassPgoNameId::from_shared(plan.definition.passes[0].name.clone()),
        10,
    ));
    drop(name);
    drop(declaration);
    drop(plan);
    assert!(weak.upgrade().is_some());
    frames.current.reset();
    assert!(weak.upgrade().is_none());
}
#[test]
fn pgo_lookup_and_frame_swap_keep_final_duration_and_total_observations() {
    let id = PassPgoNameId::from_name("main_pass");
    let mut frames = PassPgoFrames::default();
    assert_eq!(frames.previous.get(&id), None);
    assert_eq!(frames.previous.duration_ns(&id), 0);
    frames.current.record(PassPgoDuration::new(id.clone(), 700));
    frames.current.record(PassPgoDuration::new(id.clone(), 300));
    frames.end_frame();
    assert_eq!(frames.previous.get(&id), Some(300));
    assert_eq!(frames.previous.total_ns(), 1000);
    assert_eq!(frames.previous.max_ns(), 700);
    assert!(frames.current.is_empty());
    let mut stats = PassPgoStats::default();
    stats.extend([PassPgoDuration::new(id.clone(), 1000)]);
    assert_eq!(stats.get(&id), Some(1000));
    stats.reset();
    assert_eq!(stats.get(&id), None);
}

/// 空图在 lavapipe 上走完 begin → execute → end：账本记到提交，GPU 时间戳时长只在 < 1s 时给出。
#[test]
fn session_ledger_counts_the_submission_and_gpu_duration_is_option() {
    use rhapsody::raw::init::create_headless_device;
    use rhapsody::rg::vulkan::{AccessTable, VulkanGraphBackend, VulkanGraphSession};
    let Some(device) = gpu_test_support::initialize("PGO session ledger", create_headless_device())
    else {
        return;
    };
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    assert_eq!(session.backend.gpu_timings().count(), 0);
    let mut graph = session.begin_frame().unwrap();
    assert_eq!(session.frame_ledger().submissions(), 0);
    graph.add_pass("noop").with_side_effect().render(|_| true);
    session.execute_frame(graph.build()).unwrap();
    assert!(session.frame_ledger().has_ended());
    assert!(session.frame_ledger().submissions() >= 1);
    session.wait_submits_done(10_000_000_000).unwrap();
}
