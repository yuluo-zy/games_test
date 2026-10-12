//! Chain-level candidate scenarios. External device/managed allocation
//! providers are explicit fixtures; these are not whole-original-graph oracles.
use ash::vk::{self, Handle};
use std::collections::VecDeque;
use tg_rhapsody_reconstruction::{
    execution::{Error, ExecutionBackend, ExecutionChain},
    graph, lifecycle, queue_submit,
};

struct Backend {
    next: u64,
    submissions: Vec<Vec<u64>>,
    signal_values: Vec<Vec<u64>>,
    fail_submit: bool,
}
impl graph::ResourceBackend for Backend {
    fn create_without_initial_data(
        &mut self,
        _: graph::Descriptor,
        _: &str,
    ) -> Result<u64, graph::BackendFailure> {
        self.next += 1;
        Ok(self.next)
    }
    fn set_debug_name(&mut self, _: u64, _: graph::Descriptor, _: &str) {}
}
impl ExecutionBackend for Backend {
    fn managed_resource(
        &mut self,
        token: u64,
        lifecycle: &mut lifecycle::SequenceRuntime,
    ) -> Option<lifecycle::ResourceHandle> {
        // Explicit unresolved physical-control-block provider fixture. Token is
        // supplied by graph Registry; generation is NOT inferred by shared code.
        Some(lifecycle.create_resource(token, 7))
    }
    fn command_handle(&mut self) -> Result<u64, lifecycle::Failure> {
        self.next += 1;
        Ok(self.next)
    }
    fn begin_command(&mut self, _: u64) -> Result<(), lifecycle::Failure> {
        Ok(())
    }
    fn end_command(&mut self, _: vk::CommandBuffer) -> bool {
        true
    }
    fn submit(&mut self, _: vk::Queue, submits: &[queue_submit::PreparedSubmit]) -> (i32, bool) {
        self.submissions.extend(
            submits
                .iter()
                .map(|s| s.commands.iter().map(|c| c.as_raw()).collect()),
        );
        self.signal_values
            .extend(submits.iter().map(|s| s.signal_values.clone()));
        (if self.fail_submit { -4 } else { 0 }, false)
    }
}
struct Dependency;
impl graph::AccessDependency for Dependency {
    // External dependency classifier fixture, deliberately supplying a supported
    // storage mask. This does not reconstruct the original AccessType-to-mask API.
    fn access_mask(&mut self, _: u8) -> u32 {
        0x40
    }
    fn is_write(&mut self, code: u8) -> bool {
        (40..=55).contains(&code)
    }
}
fn declarations() -> graph::GraphDeclaration {
    let buffer = graph::Descriptor::Buffer(graph::BufferDesc {
        word_at_00: 256,
        word_at_08: 0x20,
        bytes_at_0c_through_0e: [0; 3],
    });
    let image = graph::Descriptor::Image(graph::TextureDesc {
        words_at_00_through_18: [0x20, 1, 1, 1, 1, 1, 1],
        bytes_at_1c_through_1e: [0; 3],
    });
    graph::GraphDeclaration {
        resources: vec![
            graph::ResourceDeclaration {
                name: "buffer".into(),
                descriptor: buffer,
                source: graph::OriginSource::Create,
            },
            graph::ResourceDeclaration {
                name: "image".into(),
                descriptor: image,
                source: graph::OriginSource::ImportOwned {
                    token: 5000,
                    initial_access: 3,
                },
            },
        ],
        passes: [4, 40]
            .into_iter()
            .enumerate()
            .map(|(i, code)| graph::PassDeclaration {
                name: format!("pass-{i}"),
                accesses: (0..2)
                    .map(|r| graph::AccessDeclaration {
                        resource_index: r,
                        synchronization_group: 0,
                        access_code: code,
                    })
                    .collect(),
            })
            .collect(),
        exports: vec![
            graph::ExportDeclaration {
                resource_index: 0,
                tag: 1,
                access_code: 55,
            },
            graph::ExportDeclaration {
                resource_index: 1,
                tag: 0,
                access_code: 55,
            },
        ],
        batch_per_pass: false,
    }
}
fn setup(fail_submit: bool) -> (ExecutionChain, Backend) {
    let q = queue_submit::QueueState {
        handle: vk::Queue::from_raw(1),
        timeline_semaphore: vk::Semaphore::from_raw(2),
        pending_timelines: VecDeque::new(),
        pending_commands: VecDeque::new(),
    };
    (
        ExecutionChain::new(
            0,
            1,
            queue_submit::DeviceState {
                primary: q,
                auxiliary: vec![],
                next_timeline: 41,
                oldest_pending: None,
            },
        ),
        Backend {
            next: 100,
            submissions: vec![],
            signal_values: vec![],
            fail_submit,
        },
    )
}
fn good(_: u64) -> bool {
    true
}
fn bad(_: u64) -> bool {
    false
}

#[test]
fn graph_produces_accesses_record_sequences_submit_and_completion() {
    let (mut runtime, mut backend) = setup(false);
    let result = runtime
        .execute_graph(
            declarations(),
            &mut graph::Pool::default(),
            &mut Dependency,
            &mut backend,
            &mut [good as fn(u64) -> bool; 2],
        )
        .unwrap();
    assert_eq!(result.compiled.info.accesses.len(), 4);
    assert_eq!(backend.signal_values, [vec![41, 42]]);
    assert_eq!(backend.submissions[0].len(), 2);
    let command_handle = backend.submissions[0][1];
    assert!(result.recorder_events.contains(&"pass:1".to_owned()));
    assert!(
        result
            .recorder_events
            .contains(&format!("emit:pass:{command_handle}"))
    );
    for resource in &result.resources {
        assert_eq!(runtime.lifecycle.resource_state(*resource), Some((2, 1)));
        runtime.lifecycle.cpu_retire(*resource);
        runtime.lifecycle.drop_handle(*resource).unwrap();
    }
    runtime.lifecycle.collect_waste();
    runtime
        .complete(lifecycle::Queue::Primary, lifecycle::Timeline(41))
        .unwrap();
    assert_eq!(runtime.lifecycle.pool_sequences(), [1]);
    for resource in &result.resources {
        assert!(
            !runtime
                .lifecycle
                .events
                .contains(&format!("dispose:{}", resource.id))
        );
    }
    runtime
        .complete(lifecycle::Queue::Primary, result.receipts[0].last_timeline)
        .unwrap();
    assert_eq!(runtime.lifecycle.pool_sequences(), [1, 2]);
    for resource in &result.resources {
        assert!(
            runtime
                .lifecycle
                .events
                .contains(&format!("dispose:{}", resource.id))
        );
    }
}
#[test]
fn graph_callback_failure_does_not_construct_submit_arrays() {
    let (mut runtime, mut backend) = setup(false);
    let result = runtime.execute_graph(
        declarations(),
        &mut graph::Pool::default(),
        &mut Dependency,
        &mut backend,
        &mut [good as fn(u64) -> bool, bad],
    );
    assert!(matches!(result, Err(Error::Record(_))));
    assert!(backend.submissions.is_empty());
    assert_eq!(runtime.device.next_timeline, 41);
    assert_eq!(runtime.lifecycle.bucket_keys(), [1, 2]);
}
#[test]
fn graph_queue_failure_preserves_pending_and_prohibits_fake_completion() {
    let (mut runtime, mut backend) = setup(true);
    let result = runtime.execute_graph(
        declarations(),
        &mut graph::Pool::default(),
        &mut Dependency,
        &mut backend,
        &mut [good as fn(u64) -> bool; 2],
    );
    let Err(Error::Queue(outcome)) = result else {
        panic!("expected queue failure")
    };
    assert_eq!(outcome.receipt, None);
    assert_eq!(runtime.device.primary.pending_commands.len(), 2);
    assert_eq!(runtime.device.next_timeline, 43);
    assert!(matches!(
        runtime.complete(lifecycle::Queue::Primary, lifecycle::Timeline(42)),
        Err(Error::CompletionAfterQueueFailure)
    ));
}

#[test]
fn binding_callback_receives_counter_command_and_current_graph_mapping() {
    use std::cell::RefCell;
    use tg_rhapsody_reconstruction::execution::PassContext;

    let (mut runtime, mut backend) = setup(false);
    let observed = RefCell::new(Vec::new());
    let callback = |context: PassContext<'_>| {
        assert_eq!(context.command.state(), 1);
        assert_eq!(context.command.queue().unwrap(), lifecycle::Queue::Primary);
        let logical = context.compiled.info.accesses[context.compiled.info.pass_access_ranges
            [context.pass_index]
            .0
            .0 as usize]
            .resource_index as usize;
        let backing = context.compiled.registry.resources[logical].backing_index as usize;
        let token =
            graph::payload_projection(&context.compiled.registry.backing_resources[backing])
                .1
                .unwrap();
        assert_eq!(context.resources[logical].id, token);
        observed.borrow_mut().push((
            context.pass_index,
            context.command.sequence().0,
            context.command.handle(),
        ));
        true
    };
    let mut declaration = declarations();
    declaration.batch_per_pass = true;
    let result = runtime
        .execute_graph_with_context(
            declaration,
            &mut graph::Pool::default(),
            &mut Dependency,
            &mut backend,
            &mut [callback; 2],
        )
        .unwrap();
    assert_eq!(
        *observed.borrow(),
        [
            (0, 2, backend.submissions[0][1]),
            (1, 4, backend.submissions[1][1]),
        ]
    );
    assert_eq!(result.receipts[1].last_timeline, lifecycle::Timeline(44));
}
