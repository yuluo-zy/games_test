use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batch {
    pub start_pass: usize,
    pub end_pass: usize,
    pub export_barriers: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandBuffer {
    pub id: u64,
    pub batch: usize,
    pub ended: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendEvent {
    BeginFrame {
        frame: u64,
    },
    RecordBatch {
        batch: usize,
        start_pass: usize,
        end_pass: usize,
        command_buffer: u64,
        export_barriers: bool,
    },
    EndCommandBuffer {
        command_buffer: u64,
    },
    QueueSubmit {
        submit_sequence: u64,
        command_buffers: Vec<u64>,
        wait_sequence: Option<u64>,
        signal_sequence: u64,
    },
    WaitSubmit {
        sequence: u64,
    },
    Export {
        logical_resource: u16,
        command_buffer: u64,
    },
    Retire {
        submit_sequence: u64,
        remaining_resources: usize,
        exported_resources: usize,
    },
    TemporalFlip {
        key: u64,
        old_parity: u32,
        new_parity: u32,
    },
    EndFrame {
        frame: u64,
    },
}
#[derive(Clone, Debug)]
pub struct ExportedResources<R> {
    pub values: HashMap<u16, R>,
}
impl<R> Default for ExportedResources<R> {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}
impl<R> ExportedResources<R> {
    pub fn take_resource(&mut self, logical: u16) -> Option<R> {
        Some(
            self.values
                .remove(&logical)
                .expect("exported resource missing"),
        )
    }
}
#[derive(Clone, Debug)]
pub struct ExecutedRenderGraph<R> {
    pub submit_sequence: u64,
    pub command_buffers: Vec<CommandBuffer>,
    pub remaining_resources: Vec<R>,
    pub exports: ExportedResources<R>,
}
impl<R> ExecutedRenderGraph<R> {
    pub fn new(
        submit_sequence: u64,
        command_buffers: Vec<CommandBuffer>,
        remaining_resources: Vec<R>,
        exports: ExportedResources<R>,
    ) -> Self {
        Self {
            submit_sequence,
            command_buffers,
            remaining_resources,
            exports,
        }
    }
    pub fn retire(self, events: &mut Vec<BackendEvent>) -> (Vec<R>, ExportedResources<R>) {
        events.push(BackendEvent::Retire {
            submit_sequence: self.submit_sequence,
            remaining_resources: self.remaining_resources.len(),
            exported_resources: self.exports.values.len(),
        });
        (self.remaining_resources, self.exports)
    }
}
#[derive(Clone, Debug, Default)]
pub struct TemporalRenderGraphState {
    parity: HashMap<u64, u32>,
}
impl TemporalRenderGraphState {
    pub fn get_and_flip_parity(&mut self, key: u64, events: &mut Vec<BackendEvent>) -> u32 {
        let old = self.parity.get(&key).copied();
        let new = old.map(|parity| 1u32.wrapping_sub(parity)).unwrap_or(0);
        self.parity.insert(key, new);
        events.push(BackendEvent::TemporalFlip {
            key,
            old_parity: old.unwrap_or(0),
            new_parity: new,
        });
        new
    }
}
#[derive(Clone, Debug)]
pub struct GraphExecutor {
    pub frame: u64,
    pub next_submit_sequence: u64,
    pub completed_sequence: u64,
    pub last_frame_duration_ns: u64,
}
impl GraphExecutor {
    pub fn new() -> Self {
        Self {
            frame: 0,
            next_submit_sequence: 1,
            completed_sequence: 0,
            last_frame_duration_ns: 0,
        }
    }
    pub fn begin_frame(&mut self, e: &mut Vec<BackendEvent>) {
        self.frame += 1;
        e.push(BackendEvent::BeginFrame { frame: self.frame })
    }
    pub fn wait_gpu_queue_trim(&mut self, sequence: u64, e: &mut Vec<BackendEvent>) {
        self.completed_sequence = self.completed_sequence.max(sequence);
        e.push(BackendEvent::WaitSubmit { sequence })
    }
    pub fn last_known_frame_duration_ns(&self) -> u64 {
        self.last_frame_duration_ns
    }
    pub fn submit(
        &mut self,
        command_buffers: Vec<u64>,
        wait_sequence: Option<u64>,
        e: &mut Vec<BackendEvent>,
    ) -> u64 {
        let signal = self.next_submit_sequence;
        self.next_submit_sequence += 1;
        e.push(BackendEvent::QueueSubmit {
            submit_sequence: signal,
            command_buffers,
            wait_sequence,
            signal_sequence: signal,
        });
        signal
    }
}
pub struct GraphExecutionRecorder {
    pub active: Option<CommandBuffer>,
    next_cb: u64,
    pub pending: Vec<CommandBuffer>,
}
impl GraphExecutionRecorder {
    pub fn new() -> Self {
        Self {
            active: None,
            next_cb: 1,
            pending: Vec::new(),
        }
    }
    pub fn ensure_active_cb(&mut self, batch: usize) -> &mut CommandBuffer {
        if self.active.is_none() {
            let id = self.next_cb;
            self.next_cb += 1;
            self.active = Some(CommandBuffer {
                id,
                batch,
                ended: false,
            })
        }
        self.active.as_mut().unwrap()
    }
    pub fn execute_compiled<R>(
        &mut self,
        batches: &[Batch],
        resources: Vec<R>,
        executor: &mut GraphExecutor,
        e: &mut Vec<BackendEvent>,
    ) -> ExecutedRenderGraph<R> {
        for (batch_index, batch) in batches.iter().enumerate() {
            assert!(batch.start_pass <= batch.end_pass);
            let cb = self.ensure_active_cb(batch_index).clone();
            e.push(BackendEvent::RecordBatch {
                batch: batch_index,
                start_pass: batch.start_pass,
                end_pass: batch.end_pass,
                command_buffer: cb.id,
                export_barriers: batch.export_barriers,
            });
            if batch_index + 1 != batches.len() {
                self.submit_pending_cbs(executor, e);
            }
        }
        ExecutedRenderGraph::new(
            executor.next_submit_sequence.saturating_sub(1),
            Vec::new(),
            resources,
            ExportedResources::default(),
        )
    }
    pub fn submit_pending_cbs(
        &mut self,
        executor: &mut GraphExecutor,
        events: &mut Vec<BackendEvent>,
    ) -> Option<u64> {
        if let Some(mut command_buffer) = self.active.take() {
            assert!(!command_buffer.ended);
            command_buffer.ended = true;
            events.push(BackendEvent::EndCommandBuffer {
                command_buffer: command_buffer.id,
            });
            self.pending.push(command_buffer);
        }
        if self.pending.is_empty() {
            return None;
        }
        let ids = self
            .pending
            .drain(..)
            .map(|command_buffer| command_buffer.id)
            .collect();
        Some(executor.submit(ids, None, events))
    }
    pub fn end_frame(&mut self, executor: &mut GraphExecutor, e: &mut Vec<BackendEvent>) {
        self.ensure_active_cb(usize::MAX);
        self.submit_pending_cbs(executor, e);
        e.push(BackendEvent::EndFrame {
            frame: executor.frame,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemporalResource<R> {
    Texture(R),
    Buffer(R),
    Exported(u16),
}
pub struct TemporalRenderGraphRecorder<R = u64> {
    pub state: TemporalRenderGraphState,
    pub resources: HashMap<u64, TemporalResource<R>>,
}
impl<R> TemporalRenderGraphRecorder<R> {
    pub fn new() -> Self {
        Self {
            state: TemporalRenderGraphState::default(),
            resources: HashMap::new(),
        }
    }
    pub fn export_temporal(&mut self, mut export: impl FnMut(&TemporalResource<R>) -> u16) {
        for resource in self.resources.values_mut() {
            if !matches!(resource, TemporalResource::Exported(_)) {
                let logical_resource = export(resource);
                *resource = TemporalResource::Exported(logical_resource);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timeline() {
        let mut e = GraphExecutor::new();
        let mut r = GraphExecutionRecorder::new();
        let mut x = Vec::new();
        e.begin_frame(&mut x);
        let g = r.execute_compiled(
            &[Batch {
                start_pass: 0,
                end_pass: 2,
                export_barriers: true,
            }],
            vec![7u8],
            &mut e,
            &mut x,
        );
        assert_eq!(g.submit_sequence, 0);
        let _ = g.retire(&mut x);
        r.end_frame(&mut e, &mut x);
        assert!(matches!(x[2], BackendEvent::Retire { .. }));
        assert!(matches!(x[3], BackendEvent::EndCommandBuffer { .. }));
        assert!(matches!(x[4], BackendEvent::QueueSubmit { .. }));
    }
    #[test]
    fn nonfinal_batches_submit_before_final_cpu_retire() {
        let mut executor = GraphExecutor::new();
        let mut recorder = GraphExecutionRecorder::new();
        let mut events = Vec::new();
        executor.begin_frame(&mut events);
        let graph = recorder.execute_compiled(
            &[
                Batch {
                    start_pass: 0,
                    end_pass: 1,
                    export_barriers: false,
                },
                Batch {
                    start_pass: 1,
                    end_pass: 2,
                    export_barriers: true,
                },
            ],
            vec![7u8],
            &mut executor,
            &mut events,
        );
        assert_eq!(graph.submit_sequence, 1);
        assert_eq!(recorder.active.as_ref().unwrap().id, 2);
        assert!(!recorder.active.as_ref().unwrap().ended);
        let (remaining_resources, _) = graph.retire(&mut events);
        assert_eq!(remaining_resources, vec![7]);
        assert_eq!(executor.completed_sequence, 0);
        assert!(!events
            .iter()
            .any(|event| matches!(event, BackendEvent::WaitSubmit { .. })));
        recorder.end_frame(&mut executor, &mut events);
        assert!(recorder.active.is_none());
        assert!(recorder.pending.is_empty());
        assert!(matches!(
            &events[events.len() - 2],
            BackendEvent::QueueSubmit { command_buffers, .. } if command_buffers == &vec![2]
        ));
    }
    #[test]
    fn parity() {
        let mut state = TemporalRenderGraphState::default();
        let mut x = Vec::new();
        assert_eq!(state.get_and_flip_parity(4, &mut x), 0);
        assert_eq!(state.get_and_flip_parity(4, &mut x), 1);
        assert_eq!(state.get_and_flip_parity(5, &mut x), 0);
        assert_eq!(state.get_and_flip_parity(4, &mut x), 0);
    }
    #[test]
    fn temporal_exports_do_not_flip_parity_or_reexport() {
        let mut recorder = TemporalRenderGraphRecorder::new();
        let mut events = Vec::new();
        assert_eq!(recorder.state.get_and_flip_parity(4, &mut events), 0);
        recorder
            .resources
            .insert(4, TemporalResource::Texture(11u64));
        recorder
            .resources
            .insert(5, TemporalResource::Buffer(12u64));
        recorder.resources.insert(6, TemporalResource::Exported(13));
        let mut exported = Vec::new();
        recorder.export_temporal(|resource| {
            exported.push(resource.clone());
            match resource {
                TemporalResource::Texture(_) => 21,
                TemporalResource::Buffer(_) => 22,
                TemporalResource::Exported(_) => panic!("already exported"),
            }
        });
        assert_eq!(exported.len(), 2);
        assert_eq!(recorder.resources[&4], TemporalResource::Exported(21));
        assert_eq!(recorder.resources[&5], TemporalResource::Exported(22));
        assert_eq!(recorder.resources[&6], TemporalResource::Exported(13));
        recorder.export_temporal(|_| panic!("already exported"));
        assert_eq!(recorder.state.get_and_flip_parity(4, &mut events), 1);
    }
}
