//! 可运行的构图与 API 捕获适配；此后端使用显式设备和托管分配测试替身。真实 Vulkan 后端见 rg/vulkan.rs。
use crate::raw::{managed::lifecycle, submit};
use crate::rg::{
    executor::{Error, ExecutionBackend, ExecutionChain},
    graph,
};
use ash::vk::{self, Handle};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::VecDeque;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceInput {
    pub name: String,
    pub kind: String,
    pub size: u64,
    #[serde(default)]
    pub imported: Option<u64>,
    #[serde(default)]
    pub initial_access: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PassInput {
    pub name: String,
    pub accesses: Vec<(u32, u8, u32)>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scenario {
    pub resources: Vec<ResourceInput>,
    pub passes: Vec<PassInput>,
    pub exports: Vec<(u32, u8)>,
    #[serde(default)]
    pub batch_per_pass: bool,
    #[serde(default)]
    pub fail_callback: Option<usize>,
    #[serde(default)]
    pub fail_end: bool,
    #[serde(default)]
    pub queue_error: i32,
    pub timeline_start: u64,
    #[serde(default)]
    pub complete: bool,
}

impl Scenario {
    pub fn build_graph(&self) -> Result<graph::BuiltGraph, String> {
        use crate::rg::resource::{ResourceHandle, ResourceOrigin};
        let mut graph = graph::RenderGraph::new();
        graph.batch_per_pass(self.batch_per_pass);
        for r in &self.resources {
            let descriptor = match r.kind.as_str() {
                "buffer" => graph::Descriptor::Buffer(graph::BufferDesc {
                    word_at_00: r.size,
                    word_at_08: 0x20,
                    bytes_at_0c_through_0e: [0; 3],
                }),
                // 示例只构造合法形状；未知描述符字段不赋予未经证实的名称。
                "image" => graph::Descriptor::Image(graph::TextureDesc {
                    words_at_00_through_18: [
                        0x20,
                        u32::try_from(r.size).map_err(|_| "image example size exceeds u32")?,
                        1,
                        1,
                        1,
                        1,
                        1,
                    ],
                    bytes_at_1c_through_1e: [0; 3],
                }),
                other => return Err(format!("unsupported resource kind {other}")),
            };
            let source = r.imported.map_or(graph::OriginSource::Create, |token| {
                graph::OriginSource::ImportOwned {
                    token,
                    initial_access: r.initial_access,
                }
            });
            graph
                .create_raw_resource(r.name.clone(), ResourceOrigin { descriptor, source })
                .map_err(|e| format!("{e:?}"))?;
        }
        for (id, pass) in self.passes.iter().enumerate() {
            let mut builder = graph.add_pass(pass.name.clone());
            for &(index, code, group) in &pass.accesses {
                builder
                    .add_resource_access(ResourceHandle::from_raw(index), code, group)
                    .map_err(|e| format!("{e:?}"))?;
            }
            let success = self.fail_callback != Some(id);
            builder.render(move |_| success);
        }
        for &(index, code) in &self.exports {
            graph
                .export(ResourceHandle::from_raw(index), code)
                .map_err(|e| format!("{e:?}"))?;
        }
        Ok(graph.build())
    }

    pub fn graph(&self) -> Result<graph::GraphDeclaration, String> {
        Ok(self.build_graph()?.declaration)
    }
}

pub struct AccessFixture;
impl graph::AccessDependency for AccessFixture {
    fn access_mask(&mut self, _: u8) -> u32 {
        0x40
    }
    fn is_write(&mut self, code: u8) -> bool {
        (40..=55).contains(&code)
    }
}

#[derive(Default)]
struct CaptureBackend {
    next: u64,
    queue_error: i32,
    fail_end: bool,
    api: Vec<Value>,
}
impl graph::ResourceBackend for CaptureBackend {
    fn create_without_initial_data(
        &mut self,
        descriptor: graph::Descriptor,
        name: &str,
    ) -> Result<u64, graph::BackendFailure> {
        self.next += 1;
        self.api.push(json!({"op":"resource_create", "name":name, "kind":descriptor.kind_code(), "token":self.next}));
        Ok(self.next)
    }
    fn set_debug_name(&mut self, _: u64, _: graph::Descriptor, _: &str) {}
}
impl ExecutionBackend for CaptureBackend {
    fn managed_resource(
        &mut self,
        token: u64,
        runtime: &mut lifecycle::SequenceRuntime,
    ) -> Option<lifecycle::ResourceHandle> {
        // 测试替身显式提供控制块，不从不透明 token 猜测。
        Some(runtime.create_resource(token, 7))
    }
    fn command_handle(&mut self) -> Result<u64, lifecycle::Failure> {
        self.next += 1;
        Ok(self.next)
    }
    fn begin_command(&mut self, handle: u64) -> Result<(), lifecycle::Failure> {
        self.api.push(json!({"op":"begin", "handle":handle}));
        Ok(())
    }
    fn end_command(&mut self, handle: vk::CommandBuffer) -> bool {
        self.api.push(json!({"op":"end", "handle":handle.as_raw()}));
        !self.fail_end
    }
    fn submit(&mut self, queue: vk::Queue, prepared: &[submit::PreparedSubmit]) -> (i32, bool) {
        let submits: Vec<Value> = prepared.iter().map(|s| s.with_vk(|info| {
            // 在 ash 数组及 pNext 仍存活时复制所有捕获字段。
            unsafe {
                let timeline = &*(info.p_next as *const vk::TimelineSemaphoreSubmitInfo);
                let copy64 = |p: *const u64, n: u32| if n == 0 { Vec::new() } else { std::slice::from_raw_parts(p, n as usize).to_vec() };
                json!({"stype":info.s_type.as_raw(),
                    "wait_semaphores":s.wait_semaphores.iter().map(|x|x.as_raw()).collect::<Vec<_>>(),
                    "wait_stages":s.wait_stages.iter().map(|x|x.as_raw()).collect::<Vec<_>>(),
                    "commands":s.commands.iter().map(|x|x.as_raw()).collect::<Vec<_>>(),
                    "signal_semaphores":s.signal_semaphores.iter().map(|x|x.as_raw()).collect::<Vec<_>>(),
                    "timeline":{"stype":timeline.s_type.as_raw(),
                        "wait_values":copy64(timeline.p_wait_semaphore_values,timeline.wait_semaphore_value_count),
                        "signal_values":copy64(timeline.p_signal_semaphore_values,timeline.signal_semaphore_value_count)}})
            }
        })).collect();
        self.api.push(
            json!({"op":"queue_submit", "queue":queue.as_raw(), "fence":0, "submits":submits}),
        );
        (self.queue_error, false)
    }
}

pub fn run(scenario: &Scenario) -> Result<Value, String> {
    let (declarations, mut callbacks, retire_callbacks) = scenario.build_graph()?.into_parts();
    let mut backend = CaptureBackend {
        next: 100,
        queue_error: scenario.queue_error,
        fail_end: scenario.fail_end,
        ..Default::default()
    };
    let primary = submit::QueueState {
        handle: vk::Queue::from_raw(0xabcd),
        timeline_semaphore: vk::Semaphore::from_raw(0xf000),
        pending_timelines: VecDeque::new(),
        pending_commands: VecDeque::new(),
    };
    let mut runtime = ExecutionChain::new(
        0,
        1,
        submit::DeviceState {
            primary,
            auxiliary: vec![],
            next_timeline: scenario.timeline_start,
            oldest_pending: None,
        },
    );
    let result = runtime.execute_graph_with_context(
        declarations,
        &mut graph::Pool::default(),
        &mut AccessFixture,
        &mut backend,
        &mut callbacks,
    );
    let mut errors = Vec::new();
    let (status, compiled, record_events, receipts) = match result {
        Ok(result) => {
            let receipts = result
                .receipts
                .iter()
                .map(|r| r.last_timeline.0)
                .collect::<Vec<_>>();
            let c = json!({"accesses":result.compiled.info.accesses.iter().map(|a|json!({"resource":a.resource_index,"previous":a.previous_access_index,"next":a.next_access_index,"group":a.synchronization_group,"code":a.access_code})).collect::<Vec<_>>(),
                "physical_mapping":result.compiled.snapshot.physical_mapping.iter().map(|p|json!({"logical":p.logical,"physical":p.physical,"token":p.token,"initial":p.initial_access})).collect::<Vec<_>>(),
                "batches":result.compiled.snapshot.batches.iter().map(|b|json!({"start":b.pass_range.start,"end":b.pass_range.end,"export":b.export_barriers})).collect::<Vec<_>>()});
            if scenario.complete {
                for h in result.resources {
                    runtime.lifecycle.cpu_retire(h);
                    runtime
                        .lifecycle
                        .drop_handle(h)
                        .map_err(|e| format!("{e:?}"))?;
                }
                runtime.lifecycle.collect_waste();
                for receipt in &result.receipts {
                    runtime
                        .complete(receipt.queue, receipt.last_timeline)
                        .map_err(|e| format!("{e:?}"))?;
                }
                for callback in retire_callbacks {
                    callback();
                }
            }
            ("ok", c, result.recorder_events, receipts)
        }
        Err(Error::Record(out)) => {
            errors.push("record/callback".into());
            ("error", Value::Null, out.events, vec![])
        }
        Err(err) => {
            errors.push(format!("{err:?}"));
            ("error", Value::Null, vec![], vec![])
        }
    };
    Ok(
        json!({"status":status,"errors":errors,"compiled":compiled,"record_events":record_events,
        "receipts":receipts,"next_timeline":runtime.device.next_timeline,"api":backend.api,
        "lifecycle_events":runtime.lifecycle.events,"pool_sequences":runtime.lifecycle.pool_sequences(),
        "limits":["device/managed allocation are explicit fixtures","access classifier/masks are dependency fixtures","barriers are record projections, no real Vulkan execution","original full compile entry/game construction closure not verified"]}),
    )
}
