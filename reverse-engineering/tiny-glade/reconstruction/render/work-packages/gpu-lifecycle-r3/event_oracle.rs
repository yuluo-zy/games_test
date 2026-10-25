use tg_gpu_lifecycle_r3::{CommandKey, Event, EventOracle, Failure};

fn key(sequence: u64) -> CommandKey {
    CommandKey {
        domain: 0,
        sequence,
    }
}

fn event_json(event: &Event) -> String {
    match event {
        Event::Create(handle) => format!(
            "{{\"event\":\"create\",\"identity\":{},\"generation\":{}}}",
            handle.identity, handle.generation
        ),
        Event::Track(handle, key) => format!(
            "{{\"event\":\"track\",\"identity\":{},\"domain\":{},\"sequence\":{}}}",
            handle.identity, key.domain, key.sequence
        ),
        Event::GenerationSkipped(handle) => format!(
            "{{\"event\":\"generation_skipped\",\"identity\":{},\"generation\":{}}}",
            handle.identity, handle.generation
        ),
        Event::CpuDrop(handle, refs) => format!(
            "{{\"event\":\"cpu_drop\",\"identity\":{},\"references\":{refs}}}",
            handle.identity
        ),
        Event::WasteQueued(handle, key) => format!(
            "{{\"event\":\"waste_queued\",\"identity\":{},\"sequence\":{}}}",
            handle.identity,
            key.map_or("null".to_owned(), |key| key.sequence.to_string())
        ),
        Event::WasteAttached(handle, key) => format!(
            "{{\"event\":\"waste_attached\",\"identity\":{},\"domain\":{},\"sequence\":{}}}",
            handle.identity, key.domain, key.sequence
        ),
        Event::CpuPoolReturn(handle) => format!(
            "{{\"event\":\"cpu_pool_return\",\"identity\":{}}}",
            handle.identity
        ),
        Event::Submit {
            queue,
            timeline,
            commands,
        } => format!(
            "{{\"event\":\"submit\",\"queue\":{queue},\"timeline\":{timeline},\"commands\":[{}]}}",
            commands
                .iter()
                .map(|key| format!(
                    "{{\"domain\":{},\"sequence\":{}}}",
                    key.domain, key.sequence
                ))
                .collect::<Vec<_>>()
                .join(",")
        ),
        Event::Complete { queue, timeline } => {
            format!("{{\"event\":\"complete\",\"queue\":{queue},\"timeline\":{timeline}}}")
        }
        Event::ReclaimCommand(key) => format!(
            "{{\"event\":\"reclaim_command\",\"domain\":{},\"sequence\":{}}}",
            key.domain, key.sequence
        ),
        Event::ExternalDispose(handle) => format!(
            "{{\"event\":\"external_dispose\",\"identity\":{}}}",
            handle.identity
        ),
    }
}

fn main() {
    let mut scenarios = Vec::new();
    let mut oracle = EventOracle::default();
    oracle.begin(key(1)).unwrap();
    oracle.begin(key(2)).unwrap();
    let first = oracle.create(10);
    let second = oracle.create(11);
    oracle.track(first, key(2), 1).unwrap();
    oracle.track(second, key(1), 1).unwrap();
    oracle.cpu_pool_return(first);
    oracle.cpu_drop(first).unwrap();
    oracle.cpu_drop(second).unwrap();
    oracle.take_out_waste();
    oracle.submit(0, 5, vec![key(1)]).unwrap();
    oracle.submit(1, 9, vec![key(2)]).unwrap();
    oracle.complete(1, 9).unwrap();
    assert!(
        !oracle
            .events
            .iter()
            .any(|event| matches!(event, Event::ExternalDispose(_)))
    );
    oracle.complete(0, 5).unwrap();
    scenarios.push((
        "multi_queue_out_of_order_cpu_drop_contiguous_gpu_reclaim",
        oracle.events,
    ));
    let mut oracle = EventOracle::default();
    oracle.submit(0, 1, vec![]).unwrap();
    oracle.complete(0, 1).unwrap();
    scenarios.push(("empty_submit", oracle.events));
    let mut oracle = EventOracle::default();
    let handle = oracle.create(4);
    let mut stale = handle;
    stale.generation = 3;
    oracle.track(stale, key(1), 0).unwrap();
    assert_eq!(
        oracle.track(handle, key(1), 0),
        Err(Failure::InvalidCommandState)
    );
    oracle.cpu_drop(handle).unwrap();
    assert_eq!(oracle.cpu_drop(handle), Err(Failure::MovedHandleDrop));
    oracle.take_out_waste();
    scenarios.push(("generation_skip_and_moved_drop_failure", oracle.events));
    println!(
        "{{\"schema_version\":1,\"provenance\":\"candidate event oracle; Vulkan external, not runtime observation\",\"scenarios\":[{}]}}",
        scenarios
            .into_iter()
            .map(|(name, events)| format!(
                "{{\"name\":\"{name}\",\"events\":[{}]}}",
                events.iter().map(event_json).collect::<Vec<_>>().join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
}
