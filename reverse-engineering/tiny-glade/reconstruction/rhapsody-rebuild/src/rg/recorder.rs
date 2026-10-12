// 候选实现迁移来源：reconstruction/render/work-packages/record-resources-r7/src/lib.rs.
// 这是行为重建，不是原作者源码。
//! 原顺序 record_batch 与私有 pass 辅助控制流投影；设备、屏障、回调和队列由实际后端消费。
use crate::raw::barrier::{Access, Resource, pre};

#[derive(Clone, Debug)]
pub struct LogicalResource {
    pub metadata: Resource,
    pub first: Option<Access>,
    pub last: Option<Access>,
    pub export_access: Option<u8>,
    pub last_command_sequence: u64,
}

#[derive(Clone, Debug)]
pub struct PassAccess {
    pub resource: usize,
    pub previous: Option<Access>,
    pub current: Access,
}

#[derive(Clone, Debug)]
pub struct Pass {
    pub accesses: Vec<PassAccess>,
    /// 原 FnOnce 数据指针在调用之前取走。
    pub callback_present: bool,
}

#[derive(Clone, Debug)]
pub struct Command {
    pub handle: u64,
    pub sequence: u64,
    pub state: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    pub events: Vec<String>,
    pub panic: bool,
    pub consumed_passes: usize,
    pub receipt: Option<(u32, u64)>,
    pub taken_command_states: Option<Vec<u32>>,
}

#[derive(Clone, Debug)]
pub struct Recorder {
    pub resources: Vec<LogicalResource>,
    pub pending_initial: Vec<usize>,
    pub commands: Vec<Command>,
}

fn validate(resource: Resource) -> bool {
    resource.kind <= 1 && resource.validity == 0
}

fn event(out: &mut Outcome, text: impl Into<String>) {
    out.events.push(text.into());
}

fn barrier(out: &mut Outcome, phase: &str, resource: usize, from: u8, to: u8) {
    event(out, format!("barrier:{phase}:{resource}:{from}:{to}"));
}

fn close_scope(out: &mut Outcome, phase: &str, command: u64, nonempty: bool) {
    if nonempty {
        event(out, format!("emit:{phase}:{command}"));
    }
    event(out, format!("reset:{phase}"));
}

/// 录制器向实际后端提交的操作；资源序号始终是图内逻辑索引。
#[derive(Clone, Copy, Debug)]
pub enum RecordAction {
    Pass(usize),
    End(u64),
    Transition {
        command: u64,
        resource: usize,
        from: u8,
        to: u8,
    },
}

impl Recorder {
    /// 边界回调报告执行成功或失败；顺序和局部写入证据来自 0x1402c1d40 / 0x1402c3240。
    pub fn record_batch(
        &mut self,
        passes: &mut [Pass],
        final_batch: bool,
        command: Command,
        mut boundary: impl FnMut(RecordAction) -> bool,
    ) -> Outcome {
        let mut out = Outcome::default();
        let Some(initial_command) = self.commands.last() else {
            out.panic = true;
            return out;
        };
        let initial_handle = initial_command.handle;
        // 解析第一个资源之前就清空原 pending 长度。
        let drained = std::mem::take(&mut self.pending_initial);
        let mut nonempty = false;
        for id in drained {
            let Some(resource) = self.resources.get(id) else {
                out.panic = true;
                return out;
            };
            let Some(first) = resource.first else {
                continue;
            };
            if resource.metadata.reused {
                continue;
            }
            if !validate(resource.metadata) {
                out.panic = true;
                return out;
            }
            event(&mut out, format!("track-barrier:initial:{id}"));
            barrier(
                &mut out,
                "initial",
                id,
                resource.metadata.initial,
                first.code,
            );
            if !boundary(RecordAction::Transition {
                command: initial_handle,
                resource: id,
                from: resource.metadata.initial,
                to: first.code,
            }) {
                out.panic = true;
                return out;
            }
            nonempty = true;
        }
        close_scope(&mut out, "initial", initial_handle, nonempty);

        // 输入命令来自实际设备 create/begin 边界。
        event(&mut out, format!("create:{}", command.handle));
        event(&mut out, format!("begin:{}", command.handle));
        for (index, pass) in passes.iter_mut().enumerate() {
            for access in &pass.accesses {
                let Some(resource) = self.resources.get_mut(access.resource) else {
                    out.panic = true;
                    return out;
                };
                event(&mut out, format!("track-pass:{}", access.resource));
                // 原跟踪忽略过期 generation；后续屏障校验仍可能拒绝这个句柄。
                if resource.metadata.validity != 3 {
                    resource.last_command_sequence =
                        resource.last_command_sequence.max(command.sequence);
                }
            }
            let mut nonempty = false;
            for access in &pass.accesses {
                let resource = &self.resources[access.resource];
                let projection = pre(resource.metadata, access.previous, access.current);
                if projection.panic.is_some() {
                    out.panic = true;
                    return out;
                }
                for transition in projection.transitions {
                    barrier(
                        &mut out,
                        "pass",
                        access.resource,
                        transition.from,
                        transition.to,
                    );
                    if !boundary(RecordAction::Transition {
                        command: command.handle,
                        resource: access.resource,
                        from: transition.from,
                        to: transition.to,
                    }) {
                        out.panic = true;
                        return out;
                    }
                    nonempty = true;
                }
            }
            close_scope(&mut out, "pass", command.handle, nonempty);
            // 顺序辅助函数没有独立的 pass 后屏障作用域。
            if !std::mem::take(&mut pass.callback_present) {
                out.panic = true;
                return out;
            }
            out.consumed_passes += 1;
            event(&mut out, format!("pass:{index}"));
            if !boundary(RecordAction::Pass(index)) {
                out.panic = true;
                return out;
            }
        }
        self.commands.push(command);
        let len = self.commands.len();
        // 结束旧最后命令及新追加序列中除最后一条以外的命令。
        for command in &mut self.commands[..len - 1] {
            event(&mut out, format!("end:{}", command.handle));
            if !boundary(RecordAction::End(command.handle)) {
                out.panic = true;
                return out;
            }
            command.state = 2;
        }
        if final_batch {
            let mut nonempty = false;
            for (id, resource) in self.resources.iter().enumerate() {
                let Some(target) = resource.export_access else {
                    continue;
                };
                if !validate(resource.metadata) {
                    out.panic = true;
                    return out;
                }
                event(&mut out, format!("track-barrier:export:{id}"));
                barrier(
                    &mut out,
                    "export",
                    id,
                    resource
                        .last
                        .map_or(resource.metadata.initial, |access| access.code),
                    target,
                );
                if !boundary(RecordAction::Transition {
                    command: self.commands[len - 1].handle,
                    resource: id,
                    from: resource.last.map_or(resource.metadata.initial, |a| a.code),
                    to: target,
                }) {
                    out.panic = true;
                    return out;
                }
                nonempty = true;
            }
            close_scope(&mut out, "export", self.commands[len - 1].handle, nonempty);
        }
        out
    }

    /// 原 submit_pending_cbs 的接管、结束、委托投影；完整队列构造在 raw/submit.rs。
    pub fn submit_pending(&mut self, out: &mut Outcome, end_error: bool, receipt: (u32, u64)) {
        let mut commands = std::mem::take(&mut self.commands);
        let Some(last) = commands.last_mut() else {
            return;
        };
        if last.state != 1 {
            out.panic = true;
            out.taken_command_states = Some(commands.iter().map(|c| c.state).collect());
            return;
        }
        event(out, "end:last");
        if end_error {
            out.panic = true;
        } else {
            last.state = 2;
            event(out, format!("queue:{}", commands.len()));
            out.receipt = Some(receipt);
        }
        out.taken_command_states = Some(commands.iter().map(|c| c.state).collect());
    }
}
