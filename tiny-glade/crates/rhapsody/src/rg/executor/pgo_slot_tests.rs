use super::*;

fn chain() -> ExecutionChain {
    ExecutionChain::new(vk::Queue::null(), Vec::new(), 1)
}
fn record(chain: &mut ExecutionChain, name: &str, ns: u64) {
    chain
        .pgo
        .current
        .record(PassPgoDuration::new(PassPgoNameId::from_name(name), ns));
}

#[test]
fn alternating_graphs_keep_their_own_previous_frame() {
    let mut chain = chain();
    let (main, presentation) = (PgoKey(1), PgoKey(2));
    for _ in 0..3 {
        chain.activate_pgo(main);
        // The main graph plans from its own previous frame, never the presentation's.
        assert!(chain.pgo.previous.is_empty() || chain.pgo.previous.get("main pass").is_some());
        assert!(chain.pgo.previous.get("resolve").is_none());
        record(&mut chain, "main pass", 900_000);
        chain.pgo.end_frame();
        chain.activate_pgo(presentation);
        assert!(chain.pgo.previous.get("main pass").is_none());
        record(&mut chain, "resolve", 30_000);
        chain.pgo.end_frame();
    }
    chain.activate_pgo(main);
    assert_eq!(chain.pgo.previous.duration_ns("main pass"), 900_000);
    assert!(chain.pgo.previous.total_ns() >= 200_000, "heavy enough to chunk");
}

#[test]
fn first_graph_adopts_preseeded_statistics_and_slots_are_bounded() {
    let mut chain = chain();
    record(&mut chain, "seeded", 5);
    chain.pgo.end_frame();
    chain.activate_pgo(PgoKey(100));
    assert_eq!(chain.pgo.previous.duration_ns("seeded"), 5, "first graph keeps the seed");
    for key in 0..20u64 {
        chain.activate_pgo(PgoKey(key));
    }
    assert!(chain.pgo_parked.len() <= 8);
}
