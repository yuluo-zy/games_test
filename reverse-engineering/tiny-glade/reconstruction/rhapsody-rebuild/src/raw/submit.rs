// 候选实现迁移来源：reconstruction/render/work-packages/queue-submit-r8/src/lib.rs.
// 这是行为重建，不是原作者源码。
//! 原队列提交顺序来自 0x140c04e20；设备创建、Vulkan 调用和错误转换由实际后端提供。
use ash::vk;
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
    /// 全局提交时间线分配器，区别于 Command.sequence。
    pub next_timeline: u64,
    pub oldest_pending: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct SubmissionInput {
    /// 设备活动主命令的 CPU 投影；原 end 在范围注册之前运行，此字段代表成功结束后的命令。
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
    /// 仅在所有数组和 pNext 仍存活的回调期间提供真实 ash 结构。
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
    /// 保留原 panic 停止点的状态；Rust 展开过程不在此 oracle 覆盖范围。
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
    /// 后端失败转换是显式设备边界。恢复数组构造、所有权转移与局部 CPU 写入；此模型不是同步原语。
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
