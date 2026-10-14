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

/// 规划好的 pass 前屏障；资源序号为图内逻辑索引。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlannedTransition {
    pub resource: usize,
    pub from: u8,
    pub to: u8,
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
        if !self.initial_barriers(&mut out, &mut boundary) {
            out.panic = true;
            return out;
        }

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
        if !self.finish_batch(&mut out, vec![command], final_batch, &mut boundary) {
            out.panic = true;
        }
        out
    }

    /// 并行录制的串行段（对应原 record_batch 本体在派发任务前的部分）：
    /// 初始屏障立即发往上一条活动命令；逐 pass 按其所在块的命令序号做资源跟踪，并规划 pass 前屏障。
    /// 屏障由 `pre(metadata, previous, current)` 纯函数得出，与录制时机无关，所以可以先规划、后在各块并行发出。
    /// 返回每个 pass 的屏障列表；`out.panic` 为真时调用方必须中止。
    pub fn plan_chunked_batch(
        &mut self,
        passes: &mut [Pass],
        commands: &[Command],
        chunk_of_pass: &[usize],
        mut boundary: impl FnMut(RecordAction) -> bool,
    ) -> (Vec<Vec<PlannedTransition>>, Outcome) {
        let mut out = Outcome::default();
        let mut plan = Vec::with_capacity(passes.len());
        if chunk_of_pass.len() != passes.len() || !self.initial_barriers(&mut out, &mut boundary) {
            out.panic = true;
            return (plan, out);
        }
        for command in commands {
            event(&mut out, format!("create:{}", command.handle));
            event(&mut out, format!("begin:{}", command.handle));
        }
        for (index, pass) in passes.iter_mut().enumerate() {
            let Some(command) = commands.get(chunk_of_pass[index]) else {
                out.panic = true;
                return (plan, out);
            };
            for access in &pass.accesses {
                let Some(resource) = self.resources.get_mut(access.resource) else {
                    out.panic = true;
                    return (plan, out);
                };
                event(&mut out, format!("track-pass:{}", access.resource));
                if resource.metadata.validity != 3 {
                    resource.last_command_sequence =
                        resource.last_command_sequence.max(command.sequence);
                }
            }
            let mut transitions = Vec::new();
            for access in &pass.accesses {
                let resource = &self.resources[access.resource];
                let projection = pre(resource.metadata, access.previous, access.current);
                if projection.panic.is_some() {
                    out.panic = true;
                    return (plan, out);
                }
                for transition in projection.transitions {
                    barrier(
                        &mut out,
                        "pass",
                        access.resource,
                        transition.from,
                        transition.to,
                    );
                    transitions.push(PlannedTransition {
                        resource: access.resource,
                        from: transition.from,
                        to: transition.to,
                    });
                }
            }
            close_scope(&mut out, "pass", command.handle, !transitions.is_empty());
            if !std::mem::take(&mut pass.callback_present) {
                out.panic = true;
                return (plan, out);
            }
            out.consumed_passes += 1;
            event(&mut out, format!("pass:{index}"));
            plan.push(transitions);
        }
        (plan, out)
    }

    /// 并行段结束后串行收尾：追加本批各块命令、结束除最后一条外的命令、最终批次发导出屏障。
    pub fn finish_chunked_batch(
        &mut self,
        out: &mut Outcome,
        commands: Vec<Command>,
        final_batch: bool,
        mut boundary: impl FnMut(RecordAction) -> bool,
    ) {
        if !self.finish_batch(out, commands, final_batch, &mut boundary) {
            out.panic = true;
        }
    }

    /// 初始屏障进入上一条活动命令；返回 false 表示 panic。
    fn initial_barriers(
        &mut self,
        out: &mut Outcome,
        boundary: &mut impl FnMut(RecordAction) -> bool,
    ) -> bool {
        let Some(initial_command) = self.commands.last() else {
            return false;
        };
        let initial_handle = initial_command.handle;
        // 解析第一个资源之前就清空原 pending 长度。
        let drained = std::mem::take(&mut self.pending_initial);
        let mut nonempty = false;
        for id in drained {
            let Some(resource) = self.resources.get(id) else {
                return false;
            };
            let Some(first) = resource.first else {
                continue;
            };
            if resource.metadata.reused {
                continue;
            }
            if !validate(resource.metadata) {
                return false;
            }
            event(out, format!("track-barrier:initial:{id}"));
            barrier(out, "initial", id, resource.metadata.initial, first.code);
            if !boundary(RecordAction::Transition {
                command: initial_handle,
                resource: id,
                from: resource.metadata.initial,
                to: first.code,
            }) {
                return false;
            }
            nonempty = true;
        }
        close_scope(out, "initial", initial_handle, nonempty);
        true
    }

    /// 追加本批新命令，结束除最后一条外的所有命令；最终批次再发导出屏障。返回 false 表示 panic。
    fn finish_batch(
        &mut self,
        out: &mut Outcome,
        new_commands: Vec<Command>,
        final_batch: bool,
        boundary: &mut impl FnMut(RecordAction) -> bool,
    ) -> bool {
        self.commands.extend(new_commands);
        let len = self.commands.len();
        // 结束旧最后命令及新追加序列中除最后一条以外的命令。
        for command in &mut self.commands[..len - 1] {
            event(out, format!("end:{}", command.handle));
            if !boundary(RecordAction::End(command.handle)) {
                return false;
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
                    return false;
                }
                event(out, format!("track-barrier:export:{id}"));
                barrier(
                    out,
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
                    return false;
                }
                nonempty = true;
            }
            close_scope(out, "export", self.commands[len - 1].handle, nonempty);
        }
        true
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

    /// 原 ensure_active_cb：确保最后一条命令处于录制态（state==1）。
    /// 没有活动命令时经 begin 回调创建一条新的并追加；返回活动命令句柄。
    pub fn ensure_active_cb(&mut self, mut begin: impl FnMut() -> Command) -> u64 {
        if let Some(active) = self.commands.last()
            && active.state == 1
        {
            return active.handle;
        }
        let command = begin();
        let handle = command.handle;
        self.commands.push(command);
        handle
    }
}

/// pass 内单个资源的访问引用；`resource_index` 返回图内逻辑索引。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassResourceRef {
    resource_index: u32,
    access: Access,
}
impl PassResourceRef {
    pub fn new(resource_index: u32, access: Access) -> Self {
        Self {
            resource_index,
            access,
        }
    }
    pub fn resource_index(&self) -> u32 {
        self.resource_index
    }
    pub fn access(&self) -> &Access {
        &self.access
    }
}

/// 原 1.x 风格录制器；`record_pass` 把单个 pass 按批录制进 Recorder。
/// 当前图主链走 `Recorder::record_batch`，本类型保留原名入口供 PDB 名消费者。
pub struct RenderGraphRecorder {
    recorder: Recorder,
}
impl RenderGraphRecorder {
    pub fn create(resources: Vec<LogicalResource>, pending_initial: Vec<usize>) -> Self {
        Self {
            recorder: Recorder {
                resources,
                pending_initial,
                commands: Vec::new(),
            },
        }
    }
    pub fn new(resources: Vec<LogicalResource>, pending_initial: Vec<usize>) -> Self {
        Self::create(resources, pending_initial)
    }
    pub fn recorder(&mut self) -> &mut Recorder {
        &mut self.recorder
    }
    /// 录制单个 pass；返回命令句柄的接受/拒绝结果与事件输出。
    pub fn record_pass(
        &mut self,
        pass: &mut Pass,
        final_batch: bool,
        command: Command,
        boundary: impl FnMut(RecordAction) -> bool,
    ) -> Outcome {
        self.recorder
            .record_batch(std::slice::from_mut(pass), final_batch, command, boundary)
    }
}
