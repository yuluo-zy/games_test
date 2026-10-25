//! Private queue submission order recovered from original 0x140c04e20.
//! Device constructors, Vulkan execution and device-error conversion are supplied.
use ash::vk::{self, Handle};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QueueId {
    pub tag: u32,
    pub index: u32,
}
impl QueueId {
    pub fn packed(self) -> u64 {
        u64::from(self.tag) | (u64::from(self.index) << 32)
    }
}

#[derive(Clone, Debug)]
pub struct Command {
    pub state: u64,
    pub sequence: u64,
    pub queue: QueueId,
    pub handle: vk::CommandBuffer,
}
#[derive(Clone, Debug)]
pub struct PendingCommand {
    pub command: Command,
    pub timeline: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Batch {
    pub wait_semaphores: Vec<vk::Semaphore>,
    pub wait_stages: Vec<vk::PipelineStageFlags>,
    pub wait_values: Vec<u64>,
    pub signal_semaphores: Vec<vk::Semaphore>,
    pub signal_values: Vec<u64>,
    pub commands: Vec<Command>,
}

#[derive(Clone, Debug)]
pub struct QueueState {
    pub handle: vk::Queue,
    pub timeline_semaphore: vk::Semaphore,
    pub pending_timelines: VecDeque<u64>,
    pub pending_commands: VecDeque<PendingCommand>,
}

#[derive(Clone, Debug)]
pub struct DeviceState {
    pub primary: QueueState,
    pub auxiliary: Vec<QueueState>,
    /// Global queue-submission allocator, distinct from Command.sequence.
    pub next_timeline: u64,
    pub oldest_pending: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct SubmissionInput {
    /// CPU projection of the device's active primary command. Its native end
    /// helper runs before range registration; this API represents end success.
    /// End errors and real synchronization are not represented by this adapter.
    pub flushed_command: Option<Command>,
    pub prior: Option<Batch>,
    pub current: Batch,
}

#[derive(Clone, Debug)]
pub struct PreparedSubmit {
    pub wait_semaphores: Vec<vk::Semaphore>,
    pub wait_stages: Vec<vk::PipelineStageFlags>,
    pub wait_values: Vec<u64>,
    pub signal_semaphores: Vec<vk::Semaphore>,
    pub signal_values: Vec<u64>,
    pub commands: Vec<vk::CommandBuffer>,
}
impl PreparedSubmit {
    /// Supplies real ash structures only while all backing arrays and pNext live.
    pub fn with_vk<R>(&self, call: impl FnOnce(&vk::SubmitInfo<'_>) -> R) -> R {
        let mut timeline = vk::TimelineSemaphoreSubmitInfo::default()
            .wait_semaphore_values(&self.wait_values)
            .signal_semaphore_values(&self.signal_values);
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&self.wait_semaphores)
            .wait_dst_stage_mask(&self.wait_stages)
            .command_buffers(&self.commands)
            .signal_semaphores(&self.signal_semaphores)
            .push_next(&mut timeline);
        call(&submit)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Outcome {
    pub panic: Option<&'static str>,
    pub receipt: Option<(QueueId, u64)>,
    pub prepared: Vec<PreparedSubmit>,
    pub backend_queue: Option<vk::Queue>,
    pub events: Vec<&'static str>,
    /// State at the native panic stop; Rust unwinding is outside this oracle.
    pub outer_lock_at_stop: bool,
}

impl DeviceState {
    fn selected(&mut self, id: QueueId) -> Option<&mut QueueState> {
        if id.tag == 0 {
            Some(&mut self.primary)
        } else {
            self.auxiliary.get_mut(id.index as usize)
        }
    }
    /// Backend failure conversion is an explicit device boundary. This function
    /// restores array construction, ownership transfer and partial CPU mutations.
    /// It is a serialized state projection, not a synchronization primitive.
    pub fn queue_submit(
        &mut self,
        id: QueueId,
        input: SubmissionInput,
        mut backend: impl FnMut(vk::Queue, &[PreparedSubmit]) -> (i32, bool),
    ) -> Outcome {
        let mut out = Outcome {
            outer_lock_at_stop: true,
            ..Outcome::default()
        };
        let mut batches = Vec::new();
        if let Some(prior) = input.prior {
            batches.push(prior);
        }
        if let Some(mut command) = input.flushed_command {
            out.events.push("flush-end");
            command.state = 2;
            batches.push(Batch {
                commands: vec![command],
                ..Batch::default()
            });
        }
        batches.push(input.current);
        let total = batches.iter().map(|b| b.commands.len() as u64).sum::<u64>();
        let start = self.next_timeline;
        self.next_timeline = self.next_timeline.wrapping_add(total);
        let end = self.next_timeline;
        let Some(queue) = self.selected(id) else {
            out.panic = Some("queue-index");
            return out;
        };
        queue.pending_timelines.extend(start..end);
        out.events.push("register-range");
        self.oldest_pending = std::iter::once(&self.primary)
            .chain(&self.auxiliary)
            .filter_map(|q| q.pending_timelines.front().copied())
            .min();
        let queue = self.selected(id).expect("selected above");
        let mut value = start;
        for mut batch in batches {
            let mut handles = Vec::new();
            for mut command in std::mem::take(&mut batch.commands) {
                if command.state != 2 {
                    out.panic = Some("command-state");
                    return out;
                }
                if command.queue.tag != id.tag || (id.tag != 0 && command.queue.index != id.index) {
                    out.panic = Some("queue-identity");
                    return out;
                }
                batch.signal_semaphores.push(queue.timeline_semaphore);
                batch.signal_values.push(value);
                handles.push(command.handle);
                command.state = 3;
                queue.pending_commands.push_back(PendingCommand {
                    command,
                    timeline: value,
                });
                value = value.wrapping_add(1);
            }
            out.prepared.push(PreparedSubmit {
                wait_semaphores: batch.wait_semaphores,
                wait_stages: batch.wait_stages,
                wait_values: batch.wait_values,
                signal_semaphores: batch.signal_semaphores,
                signal_values: batch.signal_values,
                commands: handles,
            });
        }
        out.backend_queue = Some(queue.handle);
        out.events.push("backend");
        let (error, converted_success) = backend(queue.handle, &out.prepared);
        if error != 0 {
            out.events.push("convert-error");
            if !converted_success {
                out.panic = Some("backend-error");
                return out;
            }
        }
        out.events.push("post-submit");
        if value == 0 {
            out.panic = Some("zero-receipt");
            return out;
        }
        out.receipt = Some((id, value - 1));
        out.outer_lock_at_stop = false;
        out
    }
}

pub fn fixture(v: &[u64]) -> (DeviceState, QueueId, SubmissionInput) {
    let queue = |i: u64| QueueState {
        handle: vk::Queue::from_raw(0xABCD + i),
        timeline_semaphore: vk::Semaphore::from_raw(0xF000 + i),
        pending_timelines: VecDeque::new(),
        pending_commands: VecDeque::new(),
    };
    let device = DeviceState {
        primary: queue(0),
        auxiliary: vec![queue(1), queue(2)],
        next_timeline: v[11],
        oldest_pending: None,
    };
    let id = QueueId {
        tag: v[1] as u32,
        index: v[2] as u32,
    };
    let commands = (0..v[0])
        .map(|i| Command {
            state: if i == v[9] { 1 } else { 2 },
            sequence: 100 + i,
            queue: QueueId {
                tag: id.tag ^ u32::from(i == 0 && v[10] != 0),
                index: id.index,
            },
            handle: vk::CommandBuffer::from_raw(0xC000 + i),
        })
        .collect();
    let current = Batch {
        commands,
        wait_semaphores: if v[5] != 0 {
            vec![vk::Semaphore::from_raw(0xA000)]
        } else {
            vec![]
        },
        wait_stages: if v[5] != 0 {
            vec![vk::PipelineStageFlags::from_raw(0x400)]
        } else {
            vec![]
        },
        signal_semaphores: if v[6] != 0 {
            vec![vk::Semaphore::from_raw(0xE000)]
        } else {
            vec![]
        },
        signal_values: if v[6] != 0 { vec![0] } else { vec![] },
        ..Batch::default()
    };
    let prior = (v[7] != 0).then(|| Batch {
        commands: (0..v[7])
            .map(|i| Command {
                state: 2,
                sequence: 80 + i,
                queue: id,
                handle: vk::CommandBuffer::from_raw(0xB000 + i),
            })
            .collect(),
        wait_semaphores: vec![vk::Semaphore::from_raw(0xA100)],
        wait_stages: vec![vk::PipelineStageFlags::from_raw(0x400)],
        wait_values: vec![0],
        signal_semaphores: vec![vk::Semaphore::from_raw(0xE100)],
        signal_values: vec![0],
    });
    let flushed_command = (v[8] != 0).then(|| Command {
        state: 1,
        sequence: 70,
        queue: QueueId::default(),
        handle: vk::CommandBuffer::from_raw(0xB100),
    });
    (
        device,
        id,
        SubmissionInput {
            flushed_command,
            prior,
            current,
        },
    )
}

pub fn format_outcome(d: &DeviceState, o: &Outcome) -> String {
    let nums = |v: Vec<u64>| v.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
    let qs = std::iter::once(&d.primary)
        .chain(&d.auxiliary)
        .map(|q| {
            format!(
                "{}/{}",
                nums(q.pending_timelines.iter().copied().collect()),
                q.pending_commands
                    .iter()
                    .map(|p| format!("{}:{}:{}", p.command.state, p.command.sequence, p.timeline))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join(";");
    let backend = o.backend_queue.map_or("none".into(), |q| {
        format!(
            "{}:{}",
            q.as_raw(),
            o.prepared
                .iter()
                .map(|s| format!(
                    "{}/{}/{}/{}/{}/{}",
                    nums(s.wait_semaphores.iter().map(|s| s.as_raw()).collect()),
                    nums(
                        s.wait_stages
                            .iter()
                            .map(|s| u64::from(s.as_raw()))
                            .collect()
                    ),
                    nums(s.commands.iter().map(|s| s.as_raw()).collect()),
                    nums(s.signal_semaphores.iter().map(|s| s.as_raw()).collect()),
                    nums(s.wait_values.clone()),
                    nums(s.signal_values.clone())
                ))
                .collect::<Vec<_>>()
                .join(";")
        )
    });
    let receipt = o
        .receipt
        .map_or("none".into(), |(id, v)| format!("{},{v}", id.packed()));
    format!(
        "{}|{}|{}|{}|{}|{}|{}:0|{}",
        u8::from(o.panic.is_some()),
        d.next_timeline,
        receipt,
        qs,
        backend,
        o.events.join(","),
        u8::from(o.outer_lock_at_stop),
        d.oldest_pending.map_or("none".into(), |v| v.to_string())
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prior_flush_current_order_reaches_ash_without_lifetime_loss() {
        let (mut d, id, input) = fixture(&[2, 0, 0, 0, 0, 1, 1, 2, 1, 999, 0, 10]);
        let o = d.queue_submit(id, input, |_, batches| {
            assert_eq!(batches.len(), 3);
            for batch in batches {
                batch.with_vk(|vk| {
                    assert_eq!(vk.s_type, vk::StructureType::SUBMIT_INFO);
                    assert_eq!(vk.command_buffer_count as usize, batch.commands.len());
                    assert_eq!(
                        vk.signal_semaphore_count as usize,
                        batch.signal_values.len()
                    );
                    // The callback is inside with_vk, while pNext and arrays live.
                    let timeline =
                        unsafe { &*(vk.p_next as *const vk::TimelineSemaphoreSubmitInfo<'_>) };
                    assert_eq!(
                        timeline.s_type,
                        vk::StructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO
                    );
                    let values = unsafe {
                        std::slice::from_raw_parts(
                            timeline.p_signal_semaphore_values,
                            timeline.signal_semaphore_value_count as usize,
                        )
                    };
                    assert_eq!(values, batch.signal_values);
                });
            }
            (0, false)
        });
        assert_eq!(o.receipt, Some((id, 14)));
        assert_eq!(
            d.primary
                .pending_commands
                .iter()
                .map(|c| c.command.sequence)
                .collect::<Vec<_>>(),
            [80, 81, 70, 100, 101]
        );
        assert_eq!(o.prepared[0].signal_values, [0, 10, 11]);
        assert_eq!(o.prepared[1].signal_values, [12]);
        assert_eq!(o.prepared[2].signal_values, [0, 13, 14]);
        assert!(o.prepared[2].wait_values.is_empty());
    }

    #[test]
    fn partial_failure_keeps_registered_range_and_prior_moved_command() {
        let (mut d, id, input) = fixture(&[4, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 10]);
        let o = d.queue_submit(id, input, |_, _| panic!("backend must not run"));
        assert_eq!(o.panic, Some("command-state"));
        assert_eq!(d.next_timeline, 15);
        assert_eq!(d.oldest_pending, Some(10));
        assert_eq!(
            d.auxiliary[1]
                .pending_timelines
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            [10, 11, 12, 13, 14]
        );
        assert_eq!(
            d.auxiliary[1]
                .pending_commands
                .iter()
                .map(|p| (p.command.sequence, p.command.state, p.timeline))
                .collect::<Vec<_>>(),
            [(80, 3, 10), (100, 3, 11)]
        );
        assert!(o.outer_lock_at_stop);
        assert!(o.receipt.is_none());
    }
}
