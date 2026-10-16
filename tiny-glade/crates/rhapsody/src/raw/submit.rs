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

#[derive(Clone, Debug, Default)]
pub struct PreparedSubmit {
    pub wait_semaphores: Vec<vk::Semaphore>,
    pub wait_stages: Vec<vk::PipelineStageFlags>,
    pub wait_values: Vec<u64>,
    pub signal_semaphores: Vec<vk::Semaphore>,
    pub signal_values: Vec<u64>,
    pub commands: Vec<vk::CommandBuffer>,
}

/// 提交信息构建器；字段收集后转成真实 ash 结构。
#[derive(Clone, Debug, Default)]
pub struct SubmitInfo {
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub wait_semaphores: Vec<vk::Semaphore>,
    pub wait_stages: Vec<vk::PipelineStageFlags>,
    pub wait_values: Vec<u64>,
    pub signal_semaphores: Vec<vk::Semaphore>,
    pub signal_values: Vec<u64>,
}
impl SubmitInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn command_buffer(mut self, command: vk::CommandBuffer) -> Self {
        self.command_buffers.push(command);
        self
    }
    pub fn command_buffers(mut self, commands: &[vk::CommandBuffer]) -> Self {
        self.command_buffers.extend_from_slice(commands);
        self
    }
    pub fn wait_semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.wait_semaphores.push(semaphore);
        self
    }
    pub fn signal_semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.signal_semaphores.push(semaphore);
        self
    }
    /// 与 timeline 值数组长度一致的阶段掩码；wait_stages 为空时按值数组置默认。
    pub fn with_wait_stages(mut self, stages: vk::PipelineStageFlags) -> Self {
        self.wait_stages = vec![stages; self.wait_values.len().max(self.wait_semaphores.len())];
        self
    }
    pub fn with_wait_values(mut self, values: &[u64]) -> Self {
        self.wait_values.extend_from_slice(values);
        self
    }
    pub fn with_signal_values(mut self, values: &[u64]) -> Self {
        self.signal_values.extend_from_slice(values);
        self
    }
    /// 仅在回调期间提供真实 ash 结构；timeline 扩展挂在本对象值数组上。
    pub fn with_vk<R>(&self, call: impl FnOnce(&vk::SubmitInfo<'_>) -> R) -> R {
        let mut timeline = vk::TimelineSemaphoreSubmitInfo::default()
            .wait_semaphore_values(&self.wait_values)
            .signal_semaphore_values(&self.signal_values);
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&self.wait_semaphores)
            .wait_dst_stage_mask(&self.wait_stages)
            .command_buffers(&self.command_buffers)
            .signal_semaphores(&self.signal_semaphores)
            .push_next(&mut timeline);
        call(&submit)
    }
}

/// 从 `VecDeque` 队首切出一个元素；为空返回 None。
pub fn carve_off_front<T>(deque: &mut VecDeque<T>) -> Option<T> {
    deque.pop_front()
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
    pub backend_error: i32,
    pub panic: Option<&'static str>,
    pub receipt: Option<(QueueId, u64)>,
    pub prepared: Vec<PreparedSubmit>,
    pub backend_queue: Option<vk::Queue>,
    pub events: Vec<&'static str>,
    /// 保留原 panic 停止点的状态；Rust 展开过程不在此 oracle 覆盖范围。
    pub outer_lock_at_stop: bool,
    /// 0x140c04e20 登记范围后发的 profiler 事件 0xd 的载荷：`dev+0x7f8（本次提交后的全局计数）- 全队列最旧待完成值`
    /// （没有任何待完成值时按 0 计，即等于全局计数）。
    pub in_flight: Option<u32>,
    /// 本次提交是否带上了设备级 setup 命令缓冲（`get_setup_cb_submit_info` 返回 Some）。
    pub setup_submitted: bool,
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
        backend: impl FnMut(vk::Queue, &[PreparedSubmit]) -> (i32, bool),
    ) -> Outcome {
        self.queue_submit_with_setup(id, None, input, backend)
    }
    /// 0x140c04e20 `GraphicsDevice::queue_submit` 的 CPU 部分，另带设备级 setup 命令缓冲记录
    /// （原版在 0x140bf84e0 `get_setup_cb_submit_info` 里把 `dev+0x9f8..` 槽取走并结束，成功则多一条 0x90B 记录）。
    ///
    /// ✅ 拼接顺序（wave3 逐指令读通 0x140c04e20 @0x140c051b6..0x140c05597）：先建一个 0x90B 记录的小 Vec——
    /// 请求内 `+0x40` 的可选记录（= `prior`）若存在先压入（0x140c051ee..0x140c05243），随后 `get_setup_cb_submit_info`
    /// 的结果（setup 记录）若存在压在其后（0x140c05255..0x140c052e1），最后才压入由请求命令数组构造的“当前”记录
    /// （0x140c05597）。即最终顺序 = `[prior, setup, current…]`：setup 在 prior 之后、当前命令之前
    /// （不是最先，也不是最后；上传先于使用仍成立）。
    ///
    /// ⚠️ `flushed_command` 的位置仍与本模型不同：原版把它的句柄追加进“当前”记录的命令句柄 Vec 末尾
    /// （0x140c05494..0x140c0551e，值 Vec 压 0），本模型仍把它独立成 `current` 之前的一个批次（既有
    /// `original_oracle*` 断言固定了该形态，未改动）。
    pub fn queue_submit_with_setup(
        &mut self,
        id: QueueId,
        setup: Option<Command>,
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
        if let Some(command) = setup {
            out.setup_submitted = true;
            out.events.push("setup-cb");
            batches.push(Batch {
                commands: vec![command],
                ..Batch::default()
            });
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
        let value = self.next_timeline;
        let Some(next) = value.checked_add(1) else {
            out.panic = Some("timeline-overflow");
            return out;
        };
        let Some(queue) = self.selected(id) else {
            out.panic = Some("queue-index");
            return out;
        };
        let mut pending = Vec::new();
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
                handles.push(command.handle);
                command.state = 3;
                pending.push(PendingCommand {
                    command,
                    timeline: value,
                });
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
        // 一个提交只 signal 一次队列时间线；全部命令共用这个完成点。
        let last = out.prepared.last_mut().expect("current 批次总会存在");
        last.signal_semaphores.push(queue.timeline_semaphore);
        last.signal_values.push(value);
        out.backend_queue = Some(queue.handle);
        let (error, _) = backend(queue.handle, &out.prepared);
        out.backend_error = error;
        if error != 0 {
            out.panic = Some("backend-error");
            return out;
        }
        // Vulkan 成功后才登记；失败不会留下永远无法完成的 pending 项。
        queue.pending_commands.extend(pending);
        queue.pending_timelines.push_back(value);
        self.next_timeline = next;
        self.oldest_pending = std::iter::once(&self.primary)
            .chain(&self.auxiliary)
            .filter_map(|q| q.pending_timelines.front().copied())
            .min();
        out.in_flight = Some((next - self.oldest_pending.unwrap_or(next)) as u32);
        out.receipt = Some((id, value));
        out.outer_lock_at_stop = false;
        out
    }
}

/// 设备级 setup 命令缓冲槽（`GraphicsDevice+0x9f0..+0xa4f`）。
///
/// **更正上一轮结论**：`dev+0xa00..0xa4f` 不是 “wait/signal 模板”，而是 `Mutex<Option<CommandBuffer>>`：
/// - `+0x9f0`：字节自旋锁（0 空闲 / 1 持有；`CAS 0→1`，等待 1_000_000_000 ns 超时参数）；
/// - `+0x9f8`：命令状态（u64）：1 = 录制中，2 = 已结束，3 = 已提交，**4 = None（空槽）**，0 = 新建；
/// - `+0xa00..+0xa4f`：其余 0x50B 命令缓冲字段（与命令环项同布局：`+0x08` 序号、`+0x10/+0x14` 队列标签/索引、
///   `+0x18..` 所有者 Vec，句柄在命令项 `+0x48` = `dev+0xa40`）。
///
/// 写入点（机器码/反编译已读）：
/// - 创建：`GraphicsDevice::create`（0x140bf2560）尾段把栈上 `{字节锁=0, 状态=4}` 拷到 `+0x9f0/+0x9f8`
///   （`stack0x22f0=0`、`stack0x22f8=4`，反编译第 2324 行 → 第 2447–2448 行拷入 `puVar62[0x13e/0x13f]`），
///   其余 0x50B 载荷未初始化（None，不会被读）；
/// - 惰性创建：0x140c03fa0 / 0x140c04840（`setup cb` 访问器）：持锁后若状态 == 4，则
///   `create_command_buffer(dev, "setup cb"(0x142b18788, 长 8), 族 = dev+0xad4)` + `begin`（0x140bed230）→ 写槽（状态 1）；
///   覆盖旧值时旧状态必须是 0 或 3，否则 unreachable（长 0x6e 的 panic 串 0x142b12b58）；
/// - 取走：0x140bf84e0 `get_setup_cb_submit_info`，由 `queue_submit`（0x140c04e20）调用，见 [`SetupCommandSlot::take_for_submit`]。
#[derive(Clone, Debug)]
pub struct SetupCommandSlot {
    pub command: Option<Command>,
}

/// `vkCmdPipelineBarrier`（`dev+0x3e0`）在 setup 命令结束前记录的全局内存屏障：
/// `srcStage = dstStage = ALL_COMMANDS(0x10000)`，`VkMemoryBarrier(sType=0x2e){src = dst = 0x18000 = MEMORY_READ|MEMORY_WRITE}`，
/// `dependencyFlags = 0`（0x140bf84e0）。
pub const SETUP_BARRIER_STAGE: vk::PipelineStageFlags = vk::PipelineStageFlags::ALL_COMMANDS;
pub const SETUP_BARRIER_ACCESS: vk::AccessFlags = vk::AccessFlags::from_raw(
    vk::AccessFlags::MEMORY_READ.as_raw() | vk::AccessFlags::MEMORY_WRITE.as_raw(),
);
/// 槽中命令缓冲的调试名（0x142b18788，8 字节）。
pub const SETUP_CB_NAME: &str = "setup cb";
/// `+0x9f8` 空槽标记。
pub const SETUP_SLOT_NONE: u64 = 4;

impl Default for SetupCommandSlot {
    /// 创建尾段的初值：空槽（状态 4）。
    fn default() -> Self {
        Self { command: None }
    }
}

impl SetupCommandSlot {
    /// `+0x9f8` 的原始状态值（None → 4）。
    pub fn raw_state(&self) -> u64 {
        self.command.as_ref().map_or(SETUP_SLOT_NONE, |c| c.state)
    }
    /// 0x140c03fa0/0x140c04840：槽空则用 `create`（族 = `dev+0xad4`，名 "setup cb"）造并开始录制的命令（状态 1）。
    /// 返回槽内命令的可变引用，供调用方录制上传命令。
    ///
    /// # Panics
    /// 槽内已有命令却不处于录制中（状态 ≠ 1）：本重建把 “覆盖旧值” 路径收紧为不覆盖（原版覆盖要求旧状态 0/3，
    /// 但该分支只在 `state == 4` 时进入，实际不可达）。
    pub fn ensure_recording(&mut self, create_and_begin: impl FnOnce() -> Command) -> &mut Command {
        if self.command.is_none() {
            let mut command = create_and_begin();
            command.state = 1;
            self.command = Some(command);
        }
        self.command.as_mut().expect("just filled")
    }
    /// 0x140bf84e0 `get_setup_cb_submit_info`：把槽置为 None（4）；原值为 None 返回 `Ok(None)`。
    /// 否则：记录 [`SETUP_BARRIER_STAGE`]/[`SETUP_BARRIER_ACCESS`] 全局屏障（`record_barrier`，**在状态检查之前**），
    /// 状态必须为 1（否则 `unreachable!` / `Err(Unreachable)`），`end`（`vkEndCommandBuffer`，+0x2c8）返回非 0 → panic
    /// （原版 `Err(VkResult)` 经 0x1428d9760 unwrap 失败，这里以 `Err(End(code))` 返回），成功则状态置 2 并返回该命令。
    pub fn take_for_submit(
        &mut self,
        record_barrier: impl FnOnce(vk::CommandBuffer),
        end: impl FnOnce(vk::CommandBuffer) -> i32,
    ) -> Result<Option<Command>, SetupTakeError> {
        let Some(mut command) = self.command.take() else {
            return Ok(None);
        };
        record_barrier(command.handle);
        if command.state != 1 {
            return Err(SetupTakeError::Unreachable {
                state: command.state,
            });
        }
        let code = end(command.handle);
        if code != 0 {
            return Err(SetupTakeError::End(code));
        }
        command.state = 2;
        Ok(Some(command))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetupTakeError {
    /// 原 `FUN_1428d9390(…0x142b14d78 / FUN_140bed620)`：状态不是 1（panic 串由 0x140bf84e0 的 else 分支格式化 `{:?}` 状态）。
    Unreachable { state: u64 },
    /// `vkEndCommandBuffer` 失败（原 0x142b14d20 长 0x12 的 unwrap 失败串）。
    End(i32),
}

/// `dev+0x9f8` 之后 `dev+0xa50` 起的 `SequentialDisposal`（`dispose_in_order(dev+0xa50, seq)`，0x1418c4100）；
/// `dev+0xa88` 为 “设备已丢失” 读写锁（0x140bf0720 取读锁读标志，`queue_submit` 入口先查）。
pub const DEV_SETUP_SLOT_LOCK: usize = 0x9f0;
pub const DEV_SETUP_SLOT_STATE: usize = 0x9f8;
pub const DEV_SETUP_SLOT_END: usize = 0xa50;

/// `queue_submit` 尾部 `reclaim_command_buffer_resources` 的结果。
#[derive(Clone, Debug, Default)]
pub struct ReclaimOutcome {
    /// 出队命令的句柄（交回池）。
    pub handles: Vec<vk::CommandBuffer>,
    /// 经 `InOrderRetire`（0x1418c7ae0）按序可退役的 `cb_seq`（每个对应一次 `dispose_in_order` + 销毁）。
    pub retired_sequences: Vec<u64>,
}

/// 队列提交引擎（原 `SubmissionEngine`，`GraphicsDevice+0x720`）。
///
/// 原版结构（0x1418c3300 / 0x140bf0b90 / 0x140bf0cc0，机器码已读）：
/// - 图形 `Queue` 内联于 `+0x10`，辅助队列为 `Vec<Queue>`（`+0xc8` 指针 / `+0xd0` 长度，步长 0xb0）；
/// - 每个 `Queue` 有两个 std `VecDeque`（cap-first `{cap, ptr, head, len}`）：
///   `+0x00..0x18` 待完成**时间线值环**（`u64`），`+0x20..0x38` 待回收**命令环**（项 0x60B，
///   命令缓冲 0x58B + 时间线 `@+0x58`）；`+0x80` = 最近一次读到的完成计数，`+0x88` = 队列时间线信号量；
/// - `update_pending_submissions`：`vkGetSemaphoreCounterValue(队列时间线)` → 无条件存 `+0x80` →
///   从时间线环**队首**逐个弹出直到遇到 `> counter` 的值就停（不是 `retain`）；
/// - `reclaim_command_buffer_resources`：命令环队首项时间线 `<= +0x80` 才出队；状态必须为 3（已提交），
///   否则 `unreachable!()`；命令所属队列标签/索引必须与本队列一致（`assert_eq!`），再回池。
/// - `update_queue_pending_submissions` 先推进图形队列，再按 Vec 顺序推进每个辅助队列。
///
/// 本重建的 `QueueState` 没有 `+0x80` 字段（结构体在 `rg/vulkan.rs` 里按字面量构造，不能增字段），
/// 完成计数改存引擎的 `completed`（下标 0 = 图形，`1+i` = 辅助 `i`）。
pub struct SubmissionEngine {
    pub state: DeviceState,
    completed: Vec<u64>,
    /// 每队列的按序退役器（原 `Queue+0x40`，下标同 `completed`：0 = 图形，`1+i` = 辅助）。
    retire: Vec<super::graveyard::InOrderRetire>,
}
impl SubmissionEngine {
    pub fn new(state: DeviceState) -> Self {
        let completed = vec![0; 1 + state.auxiliary.len()];
        let retire = vec![super::graveyard::InOrderRetire::new(); 1 + state.auxiliary.len()];
        Self {
            state,
            completed,
            retire,
        }
    }
    pub fn queue(&self, id: QueueId) -> Option<&QueueState> {
        if id.tag == 0 {
            Some(&self.state.primary)
        } else {
            self.state.auxiliary.get(id.index as usize)
        }
    }
    pub fn queue_mut(&mut self, id: QueueId) -> Option<&mut QueueState> {
        if id.tag == 0 {
            Some(&mut self.state.primary)
        } else {
            self.state.auxiliary.get_mut(id.index as usize)
        }
    }
    fn slot(id: QueueId) -> usize {
        if id.tag == 0 {
            0
        } else {
            1 + id.index as usize
        }
    }
    fn sync_completed_len(&mut self) {
        let want = 1 + self.state.auxiliary.len();
        if self.completed.len() < want {
            self.completed.resize(want, 0);
        }
        if self.retire.len() < want {
            self.retire
                .resize(want, super::graveyard::InOrderRetire::new());
        }
    }
    /// 队列 `+0x80`：最近一次读到的完成计数。
    pub fn completed_value(&self, id: QueueId) -> u64 {
        self.completed.get(Self::slot(id)).copied().unwrap_or(0)
    }
    fn oldest_pending(&self) -> Option<u64> {
        std::iter::once(&self.state.primary)
            .chain(&self.state.auxiliary)
            .filter_map(|q| q.pending_timelines.front().copied())
            .min()
    }
    /// 0x140bf0b90 `Queue::update_pending_submissions`，`counter` 为 `vkGetSemaphoreCounterValue` 的结果。
    pub fn update_pending_submissions(&mut self, id: QueueId, counter: u64) {
        self.sync_completed_len();
        let slot = Self::slot(id);
        let Some(queue) = self.queue_mut(id) else {
            return;
        };
        // 队首弹出：遇到第一个 `counter < 值` 就停。
        while let Some(&front) = queue.pending_timelines.front() {
            if counter < front {
                break;
            }
            queue.pending_timelines.pop_front();
        }
        self.completed[slot] = counter;
        self.state.oldest_pending = self.oldest_pending();
    }
    /// 0x1418c3300：先图形队列，再按 Vec 顺序推进每个辅助队列；`counter_of` 读取各队列的完成计数
    /// （真实后端为 `vkGetSemaphoreCounterValue(queue.timeline_semaphore)`）。
    pub fn update_queue_pending_submissions_with(
        &mut self,
        mut counter_of: impl FnMut(&QueueState) -> u64,
    ) {
        let primary = counter_of(&self.state.primary);
        self.update_pending_submissions(QueueId { tag: 0, index: 0 }, primary);
        for index in 0..self.state.auxiliary.len() {
            let counter = counter_of(&self.state.auxiliary[index]);
            self.update_pending_submissions(
                QueueId {
                    tag: 1,
                    index: index as u32,
                },
                counter,
            );
        }
    }
    /// 以显式完成计数推进所有队列（兼容旧入口；`completed` 同时作为每个队列的计数）。
    pub fn update_queue_pending_submissions(&mut self, completed: u64) {
        self.update_queue_pending_submissions_with(|_| completed);
    }
    /// 更新某队列队首待处理命令的时间戳投影；候选只记录事件，不伪造 GPU 时间。
    pub fn update_pending_submission_timestamp(&mut self, id: QueueId, timeline: u64) {
        let Some(queue) = self.queue_mut(id) else {
            return;
        };
        if let Some(pending) = queue.pending_commands.front_mut() {
            pending.timeline = pending.timeline.max(timeline);
        }
    }
    /// 0x140bf0cc0 `Queue::reclaim_command_buffer_resources`（单队列）：从命令环队首起，
    /// 时间线 `<= 该队列 +0x80` 的项出队；返回其命令缓冲句柄。
    ///
    /// # Panics
    /// 出队项状态不是 3 → `unreachable!()`；命令队列标签/索引与本队列不符 → `assert_eq!`（原版同样 panic）。
    pub fn reclaim_queue(&mut self, id: QueueId) -> Vec<vk::CommandBuffer> {
        self.sync_completed_len();
        let done = self.completed[Self::slot(id)];
        let Some(queue) = self.queue_mut(id) else {
            return Vec::new();
        };
        let mut reclaimed = Vec::new();
        while let Some(front) = queue.pending_commands.front() {
            if done < front.timeline {
                break;
            }
            let pending = queue.pending_commands.pop_front().expect("front checked");
            if pending.command.state != 3 {
                unreachable!("internal error: entered unreachable code");
            }
            // 队列自身的标签值（`+0x68`）未存于 `QueueState`，只能核对“主/辅”类别与辅助索引。
            assert_eq!(pending.command.queue.tag == 0, id.tag == 0);
            if id.tag != 0 {
                assert_eq!(pending.command.queue.index, id.index);
            }
            reclaimed.push(pending.command.handle);
        }
        reclaimed
    }
    /// 0x140bf0cc0 完整语义：[`reclaim_queue`](Self::reclaim_queue) 出队完成命令，并对每条命令的 `cb_seq`
    /// 调用 `FUN_1418c7ae0(queue+0x40, seq, dev)` —— 严格按序号退役该命令登记的资源桶
    /// （见 [`super::graveyard::InOrderRetire`]）。出队顺序即命令环顺序（= 提交顺序）。
    ///
    /// # Panics
    /// 同 `reclaim_queue`；另外序号 `<= 已退役最大序号` 或重复暂存时 panic（原版同）。
    pub fn reclaim_command_buffer_resources(&mut self, id: QueueId) -> ReclaimOutcome {
        self.sync_completed_len();
        let done = self.completed[Self::slot(id)];
        let sequences: Vec<u64> = match self.queue(id) {
            Some(queue) => queue
                .pending_commands
                .iter()
                .take_while(|p| p.timeline <= done)
                .map(|p| p.command.sequence)
                .collect(),
            None => Vec::new(),
        };
        let handles = self.reclaim_queue(id);
        let slot = Self::slot(id);
        let mut retired_sequences = Vec::new();
        for seq in sequences {
            retired_sequences.extend(self.retire[slot].retire(seq));
        }
        ReclaimOutcome {
            handles,
            retired_sequences,
        }
    }
    /// 整条 `queue_submit`（0x140c04e20）：先 `update_queue_pending_submissions`（0x1418c3300，推进所有队列完成值，
    /// 发生在时间线分配之前），再 CPU 部分提交，成功后做尾部回收（`FUN_140bf0cc0`，仅对本次提交的队列）。
    /// `counter_of` 为各队列的 `vkGetSemaphoreCounterValue`；`setup` 由 [`SetupCommandSlot::take_for_submit`] 得到。
    pub fn queue_submit(
        &mut self,
        id: QueueId,
        setup: Option<Command>,
        input: SubmissionInput,
        counter_of: impl FnMut(&QueueState) -> u64,
        backend: impl FnMut(vk::Queue, &[PreparedSubmit]) -> (i32, bool),
    ) -> (Outcome, ReclaimOutcome) {
        self.update_queue_pending_submissions_with(counter_of);
        let out = self
            .state
            .queue_submit_with_setup(id, setup, input, backend);
        if out.panic.is_some() {
            return (out, ReclaimOutcome::default());
        }
        let reclaimed = self.reclaim_command_buffer_resources(id);
        (out, reclaimed)
    }
    /// 图形队列 + 全部辅助队列依次回收（`wait_idle_and_recycle_resources` 的第 4 步）。
    pub fn reclaim_command_buffer_resources_all_queues(
        &mut self,
        completed: u64,
    ) -> Vec<vk::CommandBuffer> {
        // 兼容旧入口：先把显式完成计数并入各队列的 `+0x80`（只增不减）。
        self.sync_completed_len();
        for value in &mut self.completed {
            *value = (*value).max(completed);
        }
        let mut reclaimed = self.reclaim_queue(QueueId { tag: 0, index: 0 });
        for index in 0..self.state.auxiliary.len() {
            reclaimed.extend(self.reclaim_queue(QueueId {
                tag: 1,
                index: index as u32,
            }));
        }
        reclaimed
    }
}
