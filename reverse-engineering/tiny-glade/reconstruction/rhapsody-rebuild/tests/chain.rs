use rhapsody::app::{Scenario, run};
fn scenario() -> Scenario {
    serde_json::from_str(include_str!("../examples/two-pass.json")).unwrap()
}
#[test]
fn legacy_record_rejects_short_resource_array_before_callbacks_or_tracking() {
    use rhapsody::{
        raw::{barrier, managed::lifecycle, submit},
        rg::{executor, recorder},
    };
    let mut chain = executor::ExecutionChain::new(
        0,
        1,
        submit::DeviceState {
            primary: submit::QueueState {
                handle: ash::vk::Queue::null(),
                timeline_semaphore: ash::vk::Semaphore::null(),
                pending_timelines: Default::default(),
                pending_commands: Default::default(),
            },
            auxiliary: vec![],
            next_timeline: 1,
            oldest_pending: None,
        },
    );
    let mut recorder = recorder::Recorder {
        resources: vec![recorder::LogicalResource {
            metadata: barrier::Resource {
                kind: 1,
                initial: 0,
                reused: false,
                validity: 0,
            },
            first: None,
            last: None,
            export_access: None,
            last_command_sequence: 0,
        }],
        pending_initial: vec![],
        commands: vec![],
    };
    let mut called = false;
    let result = chain.record(
        &mut recorder,
        &mut [],
        false,
        &lifecycle::Command::new(1, 0),
        &[],
        |_| {
            called = true;
            true
        },
    );
    assert!(matches!(
        result,
        Err(executor::Error::ResourceCountMismatch {
            expected: 1,
            actual: 0
        })
    ));
    assert!(!called);
    assert!(chain.lifecycle.events.is_empty());
}
#[test]
fn unused_create_resource_keeps_logical_indices_without_physical_allocation() {
    let mut input = scenario();
    input.resources.insert(
        1,
        rhapsody::app::ResourceInput {
            name: "unused".into(),
            kind: "buffer".into(),
            size: 128,
            imported: None,
            initial_access: 0,
        },
    );
    for pass in &mut input.passes {
        for (resource, _, _) in &mut pass.accesses {
            if *resource >= 1 {
                *resource += 1;
            }
        }
    }
    for (resource, _) in &mut input.exports {
        if *resource >= 1 {
            *resource += 1;
        }
    }
    let result = run(&input).unwrap();
    assert_eq!(result["status"], "ok");
    assert_eq!(result["receipts"], serde_json::json!([42]));
    assert_eq!(result["compiled"]["accesses"].as_array().unwrap().len(), 4);
    assert!(
        result["lifecycle_events"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value
                .as_str()
                .is_some_and(|value| value.starts_with("dispose:5000")))
    );
}
#[test]
fn declarations_reach_submit_and_completion_without_internal_fixture_inputs() {
    let out = run(&scenario()).unwrap();
    assert_eq!(out["status"], "ok");
    assert_eq!(out["compiled"]["accesses"].as_array().unwrap().len(), 4);
    assert_eq!(out["receipts"], serde_json::json!([42]));
    assert_eq!(out["pool_sequences"], serde_json::json!([1, 2]));
    let begin = out["lifecycle_events"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|e| e.as_str())
        .filter(|e| e.starts_with("begin:"))
        .collect::<Vec<_>>();
    assert_eq!(begin.len(), 2);
    assert!(begin[0].contains(":1:0:0"));
    assert!(begin[1].contains(":2:0:0"));
    let api = out["api"].as_array().unwrap();
    let begun = api
        .iter()
        .filter(|e| e["op"] == "begin")
        .map(|e| e["handle"].clone())
        .collect::<Vec<_>>();
    let ended = api
        .iter()
        .filter(|e| e["op"] == "end")
        .map(|e| e["handle"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        begun, ended,
        "every submitted command must reach the backend end boundary"
    );
}
#[test]
fn callback_failure_has_no_submission_or_timeline_allocation() {
    let mut s = scenario();
    s.fail_callback = Some(1);
    let out = run(&s).unwrap();
    assert_eq!(out["status"], "error");
    assert_eq!(out["next_timeline"], 41);
    assert!(
        out["api"]
            .as_array()
            .unwrap()
            .iter()
            .all(|e| e["op"] != "queue_submit")
    );
}
#[test]
fn backend_failure_retains_timeline_reservation_without_receipt() {
    let mut s = scenario();
    s.queue_error = -4;
    s.complete = false;
    let out = run(&s).unwrap();
    assert_eq!(out["status"], "error");
    assert_eq!(out["next_timeline"], 43);
    assert_eq!(out["receipts"], serde_json::json!([]));
    assert_eq!(out["pool_sequences"], serde_json::json!([]));
}
#[test]
fn multiple_batches_use_monotonic_real_sequences() {
    let mut s = scenario();
    s.batch_per_pass = true;
    let out = run(&s).unwrap();
    assert_eq!(out["status"], "ok");
    assert_eq!(out["receipts"], serde_json::json!([42, 44]));
    assert_eq!(out["pool_sequences"], serde_json::json!([1, 2, 3, 4]));
}

// 复用示例中的 CPU 捕获后端；图/counter/录制器仍执行工程生产代码。
#[path = "../examples/queue_graph_gpu.rs"]
#[allow(dead_code)]
mod queue_graph;

#[test]
fn auxiliary_graph_produces_sequences_external_timeline_batches_and_completion() {
    use ash::vk::{self, Handle};
    use rhapsody::{
        raw::managed::lifecycle::Queue,
        rg::executor::{GraphBatchSync, GraphTimelineSignal, GraphTimelineWait},
    };
    let sync = GraphBatchSync {
        waits: vec![GraphTimelineWait {
            semaphore: vk::Semaphore::from_raw(0xa100),
            value: 7,
            stage: vk::PipelineStageFlags::TRANSFER,
        }],
        signals: vec![GraphTimelineSignal {
            semaphore: vk::Semaphore::from_raw(0xe100),
            value: 9,
        }],
    };
    let result = queue_graph::cpu_scenario(
        Queue::Auxiliary(1),
        true,
        vec![
            sync,
            GraphBatchSync {
                signals: vec![GraphTimelineSignal {
                    semaphore: vk::Semaphore::from_raw(0xe100),
                    value: 13,
                }],
                ..Default::default()
            },
        ],
        0,
    );
    assert_eq!(result["status"], "ok");
    assert_eq!(
        result["receipts"],
        serde_json::json!([[1, 1, 42], [1, 1, 44]])
    );
    assert_eq!(result["pool_sequences"], serde_json::json!([1, 2, 3, 4]));
    assert_eq!(result["calls"][0]["queue"], 0xabcf);
    assert_eq!(
        result["calls"][0]["submits"][0]["timeline"]["wait_values"],
        serde_json::json!([7])
    );
    assert_eq!(
        result["calls"][1]["submits"][0]["timeline"]["signal_values"],
        serde_json::json!([13, 43, 44])
    );
    assert!(
        result["events"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .filter(|s| s.starts_with("begin:"))
            .all(|s| s.ends_with(":1:1"))
    );
}

#[test]
fn auxiliary_graph_queue_failure_keeps_reserved_pending_commands_without_receipt() {
    use rhapsody::raw::managed::lifecycle::Queue;
    let result = queue_graph::cpu_scenario(Queue::Auxiliary(1), false, vec![], -4);
    assert_eq!(result["status"], "error");
    assert_eq!(result["next_timeline"], 43);
    assert_eq!(result["receipts"], serde_json::json!([]));
    assert_eq!(
        result["queues"][2]["pending_timelines"],
        serde_json::json!([41, 42])
    );
    assert_eq!(
        result["queues"][2]["pending_commands"],
        serde_json::json!([{"state":3,"sequence":1,"timeline":41},{"state":3,"sequence":2,"timeline":42}])
    );
    assert_eq!(result["pool_sequences"], serde_json::json!([]));
    assert_eq!(
        result["queues"][0]["pending_commands"],
        serde_json::json!([])
    );
}

#[test]
fn missing_queue_or_invalid_sync_has_no_begin_submit_or_timeline_reservation() {
    use ash::vk;
    use rhapsody::{
        raw::managed::lifecycle::Queue,
        rg::executor::{GraphBatchSync, GraphTimelineWait},
    };
    for result in [
        queue_graph::cpu_scenario(Queue::Auxiliary(2), false, vec![], 0),
        queue_graph::cpu_scenario(
            Queue::Primary,
            false,
            vec![GraphBatchSync {
                waits: vec![GraphTimelineWait {
                    semaphore: vk::Semaphore::null(),
                    value: 7,
                    stage: vk::PipelineStageFlags::TRANSFER,
                }],
                ..Default::default()
            }],
            0,
        ),
    ] {
        assert_eq!(result["status"], "error");
        assert_eq!(result["next_timeline"], 41);
        assert_eq!(result["calls"], serde_json::json!([]));
        assert!(
            result["events"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|v| v.as_str())
                .all(|s| !s.starts_with("begin:"))
        );
    }
}
