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
fn production_compilation_and_observations_share_the_interned_declaration_name() {
    use rhapsody::rg::{
        graph::RenderGraph,
        plan::{GraphDefinition, PlanContext, compile_definition},
        vulkan::AccessTable,
    };
    // Names are interned process-wide: the declaration, the plan definition and
    // the PGO observation all carry one pointer, however the name was supplied.
    let name: Arc<str> = "shared producer".into();
    let mut graph = RenderGraph::new();
    graph
        .add_pass(name.clone())
        .with_side_effect()
        .render(|_| true);
    let (declaration, _, _, declaration_error) = graph.build().into_parts();
    assert!(declaration_error.is_none());
    let interned = declaration.passes[0].name;
    assert_eq!(interned, "shared producer");
    let definition = GraphDefinition::from_declaration(
        &declaration,
        &AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    assert!(std::ptr::eq(
        interned.as_str(),
        definition.passes[0].name.as_str()
    ));
    let plan = compile_definition(definition).unwrap();
    let mut frames = PassPgoFrames::default();
    frames.current.record(PassPgoDuration::new(
        PassPgoNameId::from_resource_name(plan.definition.passes[0].name),
        10,
    ));
    assert_eq!(
        frames
            .current
            .get(&PassPgoNameId::from_name("shared producer")),
        Some(10)
    );
    // A second graph naming the pass differently-allocated still hits the same identity.
    let mut again = RenderGraph::new();
    again
        .add_pass("shared producer")
        .with_side_effect()
        .render(|_| true);
    let (second, _, _, declaration_error) = again.build().into_parts();
    assert!(declaration_error.is_none());
    assert!(std::ptr::eq(
        interned.as_str(),
        second.passes[0].name.as_str()
    ));
    frames.current.reset();
    assert!(frames.current.is_empty());
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
    use rhapsody::device::selection::create_headless_device;
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
