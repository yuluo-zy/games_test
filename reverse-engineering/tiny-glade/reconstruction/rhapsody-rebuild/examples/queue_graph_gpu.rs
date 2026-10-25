//! 图级时间线生产者与消费者；--cpu 输出由真实图/counter生成的原队列差分输入。
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        init::create_headless_device,
        managed::lifecycle,
        semaphore::TimelineSemaphore,
        submit,
    },
    rg::{
        executor::{
            ExecutionBackend, ExecutionChain, GraphBatchSync, GraphTimelineSignal,
            GraphTimelineWait,
        },
        graph::{self, ResourceBackend},
        pass_api::{AshCommandBackend, RenderPassApi},
        vulkan::{
            AccessSpec, AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend,
            VulkanGraphSession,
        },
    },
};
use serde_json::{Value, json};
use std::{collections::VecDeque, sync::Arc};

fn submit_json(queue: vk::Queue, prepared: &[submit::PreparedSubmit]) -> Value {
    json!({"queue":queue.as_raw(),"fence":0,"submits":prepared.iter().map(|p|json!({"stype":4,"wait_semaphores":p.wait_semaphores.iter().map(|s|s.as_raw()).collect::<Vec<_>>(),"wait_stages":p.wait_stages.iter().map(|s|s.as_raw()).collect::<Vec<_>>(),"commands":p.commands.iter().map(|c|c.as_raw()).collect::<Vec<_>>(),"signal_semaphores":p.signal_semaphores.iter().map(|s|s.as_raw()).collect::<Vec<_>>(),"timeline":{"stype":1000207003,"wait_values":p.wait_values,"signal_values":p.signal_values}})).collect::<Vec<_>>()})
}

struct CpuBackend {
    queue: lifecycle::Queue,
    sync: Vec<GraphBatchSync>,
    next_command: u64,
    resource: graph::EventBackend,
    calls: Vec<Value>,
    error: i32,
}
impl ResourceBackend for CpuBackend {
    fn create_without_initial_data(
        &mut self,
        d: graph::Descriptor,
        name: &str,
    ) -> Result<u64, graph::BackendFailure> {
        self.resource.create_without_initial_data(d, name)
    }
    fn set_debug_name(&mut self, token: u64, d: graph::Descriptor, name: &str) {
        self.resource.set_debug_name(token, d, name)
    }
}
impl ExecutionBackend for CpuBackend {
    fn graph_queue(&self) -> (lifecycle::Queue, u32) {
        (
            self.queue,
            match self.queue {
                lifecycle::Queue::Primary => 0,
                lifecycle::Queue::Auxiliary(i) => i + 1,
            },
        )
    }
    fn batch_synchronization(&self, index: usize, _total: usize) -> GraphBatchSync {
        self.sync.get(index).cloned().unwrap_or_default()
    }
    fn managed_resource(
        &mut self,
        token: u64,
        runtime: &mut lifecycle::SequenceRuntime,
    ) -> Option<lifecycle::ResourceHandle> {
        Some(runtime.create_resource(token, 1))
    }
    fn command_handle(&mut self) -> Result<u64, lifecycle::Failure> {
        let handle = self.next_command;
        self.next_command += 1;
        Ok(handle)
    }
    fn begin_command(&mut self, _handle: u64) -> Result<(), lifecycle::Failure> {
        Ok(())
    }
    fn end_command(&mut self, _handle: vk::CommandBuffer) -> bool {
        true
    }
    fn submit(&mut self, queue: vk::Queue, prepared: &[submit::PreparedSubmit]) -> (i32, bool) {
        self.calls.push(submit_json(queue, prepared));
        (self.error, false)
    }
}

pub fn cpu_scenario(
    queue: lifecycle::Queue,
    separate: bool,
    sync: Vec<GraphBatchSync>,
    error: i32,
) -> Value {
    let mut scenario: rhapsody::app::Scenario =
        serde_json::from_str(include_str!("two-pass.json")).unwrap();
    scenario.batch_per_pass = separate;
    let (declaration, mut callbacks, _) = scenario.build_graph().unwrap().into_parts();
    let queue_state = |index: u64| submit::QueueState {
        handle: vk::Queue::from_raw(0xabcd + index),
        timeline_semaphore: vk::Semaphore::from_raw(0xf000 + index),
        pending_timelines: VecDeque::new(),
        pending_commands: VecDeque::new(),
    };
    let mut chain = ExecutionChain::new(
        2,
        3,
        submit::DeviceState {
            primary: queue_state(0),
            auxiliary: vec![queue_state(1), queue_state(2)],
            next_timeline: 41,
            oldest_pending: None,
        },
    );
    let mut backend = CpuBackend {
        queue,
        sync: sync.clone(),
        next_command: 0xc000,
        resource: Default::default(),
        calls: vec![],
        error,
    };
    let result = chain.execute_graph_with_context(
        declaration,
        &mut graph::Pool::default(),
        &mut rhapsody::app::AccessFixture,
        &mut backend,
        &mut callbacks,
    );
    let mut receipts = vec![];
    let mut failure = None;
    match result {
        Ok(execution) => {
            receipts = execution
                .receipts
                .iter()
                .map(|r| {
                    let (t, i) = r.queue.raw();
                    json!([t, i, r.last_timeline.0])
                })
                .collect();
            for h in execution.resources {
                chain.lifecycle.cpu_retire(h);
                chain.lifecycle.drop_handle(h).unwrap();
            }
            chain.lifecycle.collect_waste();
            for receipt in execution.receipts {
                chain
                    .complete(receipt.queue, receipt.last_timeline)
                    .unwrap();
            }
        }
        Err(e) => failure = Some(format!("{e:?}")),
    }
    let queues=std::iter::once(&chain.device.primary).chain(chain.device.auxiliary.iter()).map(|q|json!({"pending_timelines":q.pending_timelines,"pending_commands":q.pending_commands.iter().map(|p|json!({"state":p.command.state,"sequence":p.command.sequence,"timeline":p.timeline})).collect::<Vec<_>>()})).collect::<Vec<_>>();
    let (tag, index) = queue.raw();
    json!({"status":if failure.is_none(){"ok"}else{"error"},"failure":failure,"queue":[tag,index],"separate":separate,"sync":sync.iter().map(|s|json!({"waits":s.waits.iter().map(|w|json!([w.semaphore.as_raw(),w.value,w.stage.as_raw()])).collect::<Vec<_>>(),"signals":s.signals.iter().map(|s|json!([s.semaphore.as_raw(),s.value])).collect::<Vec<_>>()})).collect::<Vec<_>>(),"calls":backend.calls,"receipts":receipts,"next_timeline":chain.device.next_timeline,"events":chain.lifecycle.events,"pool_sequences":chain.lifecycle.pool_sequences(),"queues":queues,"backend_error":error})
}

fn cpu_cases() -> Value {
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
    json!([
        cpu_scenario(lifecycle::Queue::Primary, false, vec![], 0),
        cpu_scenario(lifecycle::Queue::Auxiliary(0), false, vec![], 0),
        cpu_scenario(
            lifecycle::Queue::Auxiliary(1),
            true,
            vec![
                sync.clone(),
                GraphBatchSync {
                    signals: vec![GraphTimelineSignal {
                        semaphore: vk::Semaphore::from_raw(0xe100),
                        value: 13
                    }],
                    ..Default::default()
                }
            ],
            0
        ),
        cpu_scenario(lifecycle::Queue::Auxiliary(1), false, vec![sync], -4)
    ])
}

fn access_table() -> AccessTable {
    let mut table = AccessTable::default();
    for (code, write, stage, access) in [
        (
            40,
            true,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
        ),
        (
            4,
            false,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_READ,
        ),
        (
            55,
            false,
            vk::PipelineStageFlags::HOST,
            vk::AccessFlags::HOST_READ,
        ),
    ] {
        table.0.insert(
            code,
            AccessSpec {
                original_mask: if write { 0x1000 } else { 0x800 },
                write,
                stages: stage,
                access,
                layout: vk::ImageLayout::GENERAL,
            },
        );
    }
    table
}

fn gpu_run() -> Result<Value, Box<dyn std::error::Error>> {
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    let external = TimelineSemaphore::new(device.clone(), 0)?;
    let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
    let mut producer = VulkanGraphBackend::new(device.clone(), access_table())?;
    producer.set_graph_synchronization(vec![], vec![(external.clone(), 7)])?;
    let mut graph = graph::RenderGraph::new();
    let source = producer
        .declare(
            &mut graph,
            "source",
            ResourceSpec::Buffer(BufferDesc::new_gpu_only(256, usage)),
        )
        .unwrap();
    let resolver = producer.resolver.clone();
    let d = device.clone();
    let mut pass = graph.add_pass("produce");
    pass.add_resource_access(source, 40, 0).unwrap();
    pass.render(move |handle| {
        let mut backend = unsafe { AshCommandBackend::new(&d.device) };
        let mut api =
            RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(handle), 0, false);
        api.fill_buffer(resolver.buffer("source").unwrap(), None, 0x11223344)
            .is_ok()
    });
    graph.export(source, 4).unwrap();
    let mut produced = producer
        .execute(graph.build())
        .map_err(std::io::Error::other)?;
    let source_token = produced.execution.compiled.snapshot.physical_mapping[source.raw() as usize]
        .token
        .unwrap();
    let source_resource = produced.backend.resource_clone(source_token).unwrap();
    let readback = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, usage),
    )?);
    let mut consumer = VulkanGraphBackend::new(device.clone(), access_table())?;
    consumer.set_graph_synchronization(
        vec![(external.clone(), 7, vk::PipelineStageFlags::TRANSFER)],
        vec![(external.clone(), 13)],
    )?;
    consumer
        .bind_import_named_owned_by("source", source_token, source_resource, device.queue_family)
        .unwrap();
    let readback_token = consumer.bind_import(GpuResource::Buffer(readback.clone()));
    let mut graph = graph::RenderGraph::new();
    graph.batch_per_pass(true);
    let descriptor = ResourceSpec::Buffer(BufferDesc::new_gpu_only(256, usage)).descriptor();
    let source = graph
        .import_owned("source", descriptor, source_token, 4)
        .unwrap();
    let middle = consumer
        .declare(
            &mut graph,
            "middle",
            ResourceSpec::Buffer(BufferDesc::new_gpu_only(256, usage)),
        )
        .unwrap();
    let output = graph
        .import_owned("readback", descriptor, readback_token, 0)
        .unwrap();
    let region = vk::BufferCopy {
        src_offset: 0,
        dst_offset: 0,
        size: 256,
    };
    let resolver = consumer.resolver.clone();
    let d = device.clone();
    let mut pass = graph.add_pass("copy-from-producer");
    pass.add_resource_access(source, 4, 0).unwrap();
    pass.add_resource_access(middle, 40, 0).unwrap();
    pass.render(move |handle| {
        unsafe {
            d.device.cmd_copy_buffer(
                vk::CommandBuffer::from_raw(handle),
                resolver.buffer("source").unwrap(),
                resolver.buffer("middle").unwrap(),
                &[region],
            );
        }
        true
    });
    let resolver = consumer.resolver.clone();
    let d = device.clone();
    let destination = readback.handle;
    let mut pass = graph.add_pass("readback");
    pass.add_resource_access(middle, 4, 0).unwrap();
    pass.add_resource_access(output, 40, 0).unwrap();
    pass.render(move |handle| {
        unsafe {
            d.device.cmd_copy_buffer(
                vk::CommandBuffer::from_raw(handle),
                resolver.buffer("middle").unwrap(),
                destination,
                &[region],
            );
        }
        true
    });
    graph.export(output, 55).unwrap();
    let mut consumed = consumer
        .execute(graph.build())
        .map_err(std::io::Error::other)?;
    external.wait(13, 10_000_000_000)?;
    consumed
        .wait(10_000_000_000)
        .map_err(std::io::Error::other)?;
    produced
        .wait(10_000_000_000)
        .map_err(std::io::Error::other)?;
    let mut bytes = [0; 256];
    readback.read_bytes(0, &mut bytes)?;
    if !bytes
        .chunks_exact(4)
        .all(|b| b == 0x11223344u32.to_le_bytes())
    {
        return Err("图级同步读回不一致".into());
    }
    let aux_unavailable = if device.additional_queues.is_empty() {
        VulkanGraphBackend::new_for_queue(
            device.clone(),
            access_table(),
            lifecycle::Queue::Auxiliary(0),
        )
        .is_err()
    } else {
        false
    };
    let mut rejected = VulkanGraphBackend::new(device.clone(), access_table())?;
    rejected
        .bind_import_named_owned_by(
            "wrong-family",
            100,
            GpuResource::Buffer(readback.clone()),
            device.queue_family.wrapping_add(1),
        )
        .unwrap();
    let mut invalid = graph::RenderGraph::new();
    invalid
        .import_owned("wrong-family", descriptor, 100, 0)
        .unwrap();
    let wrong_family_rejected = rejected.execute(invalid.build()).is_err();
    assert!(wrong_family_rejected);
    let persistent = persistent_gpu_run(device.clone())?;
    Ok(
        json!({"status":"passed","real_gpu":true,"device":name,"available_auxiliary_queues":device.additional_queues.len(),"real_auxiliary_graph_executed":false,"missing_auxiliary_rejected":aux_unavailable,"cross_family_import_without_transfer_rejected":wrong_family_rejected,"external_timeline":external.completed()?,"pattern":"11223344","bytes":256,"consumer_batches":consumed.execution.receipts.len(),"producer_calls":produced.backend.submissions.iter().map(|s|submit_json(s.queue,&s.batches)).collect::<Vec<_>>(),"consumer_calls":consumed.backend.submissions.iter().map(|s|submit_json(s.queue,&s.batches)).collect::<Vec<_>>(),"persistent_session":persistent }),
    )
}
fn persistent_gpu_run(
    device: Arc<rhapsody::raw::device::VulkanDevice>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
    let output = Arc::new(Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(256, usage),
    )?);
    let output_weak = Arc::downgrade(&output);
    let mut session =
        VulkanGraphSession::new(VulkanGraphBackend::new(device.clone(), access_table())?);
    let output_token = session
        .backend
        .bind_import(GpuResource::Buffer(output.clone()));
    let descriptor = ResourceSpec::Buffer(BufferDesc::new_gpu_only(256, usage)).descriptor();
    let mut frames = vec![];
    let mut weak = std::collections::BTreeMap::new();
    let internal_timeline = session.backend.timeline.handle;
    for frame in 0..3u32 {
        let mut graph = session.begin_frame().map_err(std::io::Error::other)?;
        let out = graph
            .import_owned(
                "output",
                descriptor,
                output_token,
                if frame == 0 { 0 } else { 55 },
            )
            .unwrap();
        let size = if frame < 2 { 256 } else { 512 };
        let work = session
            .backend
            .declare(
                &mut graph,
                "work",
                ResourceSpec::Buffer(BufferDesc::new_gpu_only(size, usage)),
            )
            .unwrap();
        let resolver = session.backend.resolver.clone();
        let d = device.clone();
        let pattern = 0x11223344u32 + frame;
        let mut pass = graph.add_pass("fill");
        pass.add_resource_access(work, 40, 0).unwrap();
        pass.render(move |handle| {
            let mut backend = unsafe { AshCommandBackend::new(&d.device) };
            let mut api =
                RenderPassApi::new(&mut backend, vk::CommandBuffer::from_raw(handle), 0, false);
            api.fill_buffer(resolver.buffer("work").unwrap(), None, pattern)
                .is_ok()
        });
        let resolver = session.backend.resolver.clone();
        let d = device.clone();
        let out_handle = output.handle;
        let mut pass = graph.add_pass("readback");
        pass.add_resource_access(work, 4, 0).unwrap();
        pass.add_resource_access(out, 40, 0).unwrap();
        pass.render(move |handle| {
            unsafe {
                d.device.cmd_copy_buffer(
                    vk::CommandBuffer::from_raw(handle),
                    resolver.buffer("work").unwrap(),
                    out_handle,
                    &[vk::BufferCopy {
                        src_offset: 0,
                        dst_offset: 0,
                        size: 256,
                    }],
                );
            }
            true
        });
        graph.export(out, 55).unwrap();
        let receipt = session
            .execute_frame(graph.build())
            .map_err(std::io::Error::other)?;
        assert!(session.clear_after_completion().is_err());
        let work_token = receipt.snapshot.physical_mapping[work.raw() as usize]
            .token
            .unwrap();
        if let Some(GpuResource::Buffer(resource)) = session.backend.resource_clone(work_token) {
            weak.entry(work_token)
                .or_insert_with(|| Arc::downgrade(&resource));
        }
        session
            .wait_frame(receipt.frame, 10_000_000_000)
            .map_err(std::io::Error::other)?;
        let mut bytes = [0; 256];
        output.read_bytes(0, &mut bytes)?;
        assert!(bytes.chunks_exact(4).all(|b| b == pattern.to_le_bytes()));
        assert_eq!(session.backend.timeline.handle, internal_timeline);
        assert_eq!(
            receipt.command_sequences,
            vec![u64::from(frame) * 2 + 1, u64::from(frame) * 2 + 2]
        );
        assert_eq!(
            session.backend.timeline.completed()?,
            u64::from(frame) * 2 + 2
        );
        assert_eq!(session.command_count(), 2);
        frames.push(json!({"frame":receipt.frame,"sequences":receipt.command_sequences,"timeline":session.backend.timeline.completed()?,"work_token":work_token,"pool_hits":receipt.snapshot.pool_hits,"resource_count":session.resource_count(),"command_count":session.command_count(),"readback_passed":true}));
    }
    assert_eq!(frames[0]["work_token"], frames[1]["work_token"]);
    assert_ne!(frames[1]["work_token"], frames[2]["work_token"]);
    let collected = session
        .collect_after_completion(&[output_token])
        .map_err(std::io::Error::other)?;
    assert_eq!(collected, 2);
    assert!(weak.values().all(|r| r.upgrade().is_none()));
    assert_eq!(session.resource_count(), 1);
    let mut graph = session.begin_frame().map_err(std::io::Error::other)?;
    let out = graph
        .import_owned("output", descriptor, output_token, 55)
        .unwrap();
    let mut pass = graph.add_pass("failure");
    pass.add_resource_access(out, 40, 0).unwrap();
    pass.render(|_| false);
    let timeline_before = session.chain.device.next_timeline;
    let failure = session.execute_frame(graph.build()).is_err();
    assert!(failure && session.is_poisoned());
    assert_eq!(session.chain.device.next_timeline, timeline_before);
    assert!(session.begin_frame().is_err());
    let next_sequence = session
        .chain
        .counters
        .next_value(rhapsody::raw::device::QueueIdentity::Primary)
        .unwrap();
    assert_eq!(next_sequence, 9);
    drop(output);
    let cleared = session
        .clear_after_completion()
        .map_err(std::io::Error::other)?;
    assert_eq!(cleared, 1);
    assert!(output_weak.upgrade().is_none());
    Ok(
        json!({"status":"passed","persistent_backend":true,"persistent_chain_and_pool":true,"same_timeline":true,"frames":frames,"resize_collected":collected,"old_resources_reclaimed":true,"commands_reused":2,"callback_failure_poisoned":failure,"failure_timeline_unchanged":true,"next_sequence_after_failure":next_sequence,"next_timeline_after_failure":session.chain.device.next_timeline,"resources_after_clear":session.resource_count(),"limits":["串行候选session，不证明原游戏并行调度","失败录制的原完整unwind仍未知；停止后wait_idle清除实际Arc"]}),
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--cpu") {
        println!("{}", cpu_cases());
    } else {
        println!("{}", gpu_run()?);
    }
    Ok(())
}
