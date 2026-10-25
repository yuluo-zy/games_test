// 候选实现迁移来源：reconstruction/render/work-packages/sequence-runtime-r8/src/lib.rs.
// 这是行为重建，不是原作者源码。
//! 消费真实每队列命令序号的类型化生命周期模型。对象分配由后端提供；全局桶使用 u64。
pub use crate::raw::device::DeviceCounters;
use crate::raw::managed::tracking::{
    CommandState, ControlBlock, TrackOutcome, drop_reference, track_handle,
};
use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Queue {
    Primary,
    Auxiliary(u32),
}
impl Queue {
    pub fn raw(self) -> (u32, u32) {
        match self {
            Self::Primary => (0, 0),
            Self::Auxiliary(i) => (1, i),
        }
    }
    pub fn producer(self) -> crate::raw::device::QueueIdentity {
        match self {
            Self::Primary => crate::raw::device::QueueIdentity::Primary,
            Self::Auxiliary(i) => crate::raw::device::QueueIdentity::Auxiliary(i),
        }
    }
    pub fn from_raw(tag: u32, index: u32) -> Result<Self, Failure> {
        match tag {
            0 => Ok(Self::Primary),
            1 => Ok(Self::Auxiliary(index)),
            _ => Err(Failure::InvalidQueueTag),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sequence(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timeline(pub u64);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command {
    pub bytes: [u8; 88],
}
impl Command {
    pub fn new(handle: u64, pool_family: u32) -> Self {
        let mut c = Self { bytes: [0; 88] };
        c.bytes[0x48..0x50].copy_from_slice(&handle.to_le_bytes());
        c.bytes[0x50..0x54].copy_from_slice(&pool_family.to_le_bytes());
        c
    }
    pub fn state(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0..4].try_into().unwrap())
    }
    pub fn set_state(&mut self, state: u32) {
        self.bytes[0..8].copy_from_slice(&u64::from(state).to_le_bytes());
    }
    pub fn sequence(&self) -> Sequence {
        Sequence(u64::from_le_bytes(self.bytes[8..16].try_into().unwrap()))
    }
    pub fn queue(&self) -> Result<Queue, Failure> {
        Queue::from_raw(
            u32::from_le_bytes(self.bytes[16..20].try_into().unwrap()),
            u32::from_le_bytes(self.bytes[20..24].try_into().unwrap()),
        )
    }
    pub fn handle(&self) -> u64 {
        u64::from_le_bytes(self.bytes[0x48..0x50].try_into().unwrap())
    }
    pub fn pool_family(&self) -> u32 {
        u32::from_le_bytes(self.bytes[0x50..0x54].try_into().unwrap())
    }
    /// 保留已观察到的序号，不做加一或减一的测试数据换算。
    pub fn record_command(&self) -> crate::rg::recorder::Command {
        crate::rg::recorder::Command {
            handle: self.handle(),
            sequence: self.sequence().0,
            state: self.state(),
        }
    }
    pub fn caller_snapshot(&self) -> crate::raw::command_buffer::CommandSnapshot {
        crate::raw::command_buffer::CommandSnapshot(self.bytes)
    }
    pub fn from_caller_snapshot(snapshot: crate::raw::command_buffer::CommandSnapshot) -> Self {
        Self { bytes: snapshot.0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Failure {
    InvalidQueueTag,
    InvalidBeginState,
    BackendReset,
    BackendBegin,
    CounterBounds,
    DuplicateBucket,
    InvalidRecordingState,
    UnknownResource,
    ResourceAlreadyDropped,
    InvalidPendingState,
    QueueMismatch,
    SequenceAlreadyDisposed,
    DuplicateReadyMarker,
    MissingBucket,
    PoolFamilyBounds,
}
#[derive(Debug)]
pub struct BeginFailure {
    pub reason: Failure,
    pub command: Command,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubmissionReceipt {
    pub queue: Queue,
    pub last_timeline: Timeline,
}
#[derive(Clone, Debug)]
pub struct PendingCommand {
    pub command: Command,
    pub threshold: Timeline,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceHandle {
    pub id: u64,
    pub generation: u32,
}

#[derive(Debug)]
struct Resource {
    control: ControlBlock,
    dropped: bool,
}
#[derive(Debug, Default)]
struct QueueState {
    pending: VecDeque<PendingCommand>,
    timelines: VecDeque<Timeline>,
    completed: u64,
    prefix: u64,
    ready: VecDeque<u64>,
}

#[derive(Debug)]
pub struct SequenceRuntime {
    buckets: BTreeMap<u64, Vec<ResourceHandle>>,
    queues: BTreeMap<Queue, QueueState>,
    resources: BTreeMap<u64, Resource>,
    waste: Vec<ResourceHandle>,
    pools: BTreeMap<u32, Vec<Command>>,
    pool_family_count: u32,
    pub events: Vec<String>,
}
impl SequenceRuntime {
    pub fn new(pool_family_count: u32) -> Self {
        Self {
            buckets: BTreeMap::new(),
            queues: BTreeMap::new(),
            resources: BTreeMap::new(),
            waste: Vec::new(),
            pools: BTreeMap::new(),
            pool_family_count,
            events: Vec::new(),
        }
    }
    /// 在实际后端 reset/begin 后调用；使用设备序号生产者，不从默认零开始。
    pub fn begin_after_backend(
        &mut self,
        counters: &DeviceCounters,
        mut command: Command,
        queue: Queue,
        backend: Result<(), Failure>,
    ) -> Result<Command, BeginFailure> {
        if !matches!(command.state(), 0 | 3) {
            return Err(BeginFailure {
                reason: Failure::InvalidBeginState,
                command,
            });
        }
        if let Err(reason) = backend {
            return Err(BeginFailure { reason, command });
        }
        let Some(sequence) = counters.begin_sequence(queue.producer()) else {
            return Err(BeginFailure {
                reason: Failure::CounterBounds,
                command,
            });
        };
        command.set_state(1);
        command.bytes[8..16].copy_from_slice(&sequence.to_le_bytes());
        let (tag, index) = queue.raw();
        command.bytes[16..20].copy_from_slice(&tag.to_le_bytes());
        command.bytes[20..24].copy_from_slice(&index.to_le_bytes());
        command.bytes[86] = 0;
        self.events.push(format!(
            "begin:{}:{sequence}:{tag}:{index}",
            command.handle()
        ));
        if let Err(reason) = self.accept_begun(&command) {
            return Err(BeginFailure { reason, command });
        }
        Ok(command)
    }
    /// 接收已开始的后端命令，不再次递增计数器；重复插入先替换再执行原断言。
    pub fn accept_begun(&mut self, command: &Command) -> Result<(), Failure> {
        if command.state() != 1 {
            return Err(Failure::InvalidRecordingState);
        }
        command.queue()?;
        let previous = self.buckets.insert(command.sequence().0, Vec::new());
        self.events.push(format!(
            "insert:{}:{}",
            command.sequence().0,
            u8::from(previous.is_some())
        ));
        if previous.is_some() {
            return Err(Failure::DuplicateBucket);
        }
        Ok(())
    }
    pub fn create_resource(&mut self, id: u64, generation: u32) -> ResourceHandle {
        self.resources.insert(
            id,
            Resource {
                control: ControlBlock::new(generation),
                dropped: false,
            },
        );
        ResourceHandle { id, generation }
    }
    pub fn track(&mut self, handle: ResourceHandle, command: &Command) -> Result<(), Failure> {
        let resource = self
            .resources
            .get(&handle.id)
            .ok_or(Failure::UnknownResource)?;
        let result = track_handle(
            &resource.control,
            handle.generation,
            CommandState {
                tag: command.state(),
                sequence: command.sequence().0,
            },
        )
        .map_err(|_| Failure::InvalidRecordingState)?;
        self.events.push(format!(
            "track:{}:{}:{}",
            handle.id,
            resource.control.last_sequence.load(Ordering::SeqCst),
            u8::from(result == TrackOutcome::GenerationSkipped)
        ));
        Ok(())
    }
    /// 录制器输出的是命令序号，不能当成队列时间线；用于绑定真实 Command sequence。
    pub fn import_record_tracker(
        &mut self,
        handle: ResourceHandle,
        sequence: Sequence,
    ) -> Result<(), Failure> {
        let resource = self
            .resources
            .get(&handle.id)
            .ok_or(Failure::UnknownResource)?;
        resource
            .control
            .last_sequence
            .fetch_max(sequence.0, Ordering::SeqCst);
        Ok(())
    }
    pub fn cpu_retire(&mut self, handle: ResourceHandle) {
        self.events.push(format!("cpu-retire:{}", handle.id));
    }
    pub fn clone_handle(&self, handle: ResourceHandle) -> Result<(), Failure> {
        let r = self
            .resources
            .get(&handle.id)
            .ok_or(Failure::UnknownResource)?;
        crate::raw::managed::tracking::clone_reference(&r.control);
        Ok(())
    }
    pub fn drop_handle(&mut self, handle: ResourceHandle) -> Result<(), Failure> {
        let resource = self
            .resources
            .get_mut(&handle.id)
            .ok_or(Failure::UnknownResource)?;
        let last = drop_reference(&resource.control, resource.dropped)
            .map_err(|_| Failure::ResourceAlreadyDropped)?;
        self.events.push(format!(
            "drop:{}:{}",
            handle.id,
            resource.control.references.load(Ordering::SeqCst)
        ));
        if last {
            resource.dropped = true;
            self.waste.push(handle);
        }
        Ok(())
    }
    pub fn collect_waste(&mut self) {
        for handle in self.waste.drain(..) {
            let sequence = self.resources[&handle.id]
                .control
                .last_sequence
                .load(Ordering::SeqCst);
            if let Some(bucket) = self.buckets.get_mut(&sequence) {
                bucket.push(handle);
                self.events.push(format!("attach:{}:{sequence}", handle.id));
            } else {
                self.events.push(format!("dispose:{}", handle.id));
            }
        }
    }
    /// 提交适配接收队列分配的每条命令时间线阈值，不把命令序号当信号量值。
    pub fn accept_submission(&mut self, receipt: SubmissionReceipt, pending: Vec<PendingCommand>) {
        let q = self.queues.entry(receipt.queue).or_default();
        for item in pending {
            self.events.push(format!(
                "submit:{}:{}",
                item.command.sequence().0,
                item.threshold.0
            ));
            q.timelines.push_back(item.threshold);
            q.pending.push_back(item);
        }
    }
    /// 提交成功后已结束命令变为标签 3；此函数消费后端结果，不自行调用 Vulkan。
    pub fn accept_queue_success(
        &mut self,
        queue: Queue,
        commands: Vec<(Command, Timeline)>,
    ) -> SubmissionReceipt {
        let last = commands.last().map_or(Timeline(0), |(_, t)| *t);
        let receipt = SubmissionReceipt {
            queue,
            last_timeline: last,
        };
        let pending = commands
            .into_iter()
            .map(|(mut command, threshold)| {
                command.set_state(3);
                PendingCommand { command, threshold }
            })
            .collect();
        self.accept_submission(receipt, pending);
        receipt
    }
    /// 原查询直接保存返回值；值单调递增由 Vulkan 运行时保证，测试替身可提供回退值。
    pub fn observe_timeline(&mut self, queue: Queue, value: Timeline) {
        let (tag, index) = queue.raw();
        self.events
            .push(format!("complete:{tag}:{index}:{}", value.0));
        let q = self.queues.entry(queue).or_default();
        q.completed = value.0;
        while q.timelines.front().is_some_and(|t| t.0 <= value.0) {
            q.timelines.pop_front();
        }
    }
    fn drain_bucket(&mut self, sequence: u64) -> Result<(), Failure> {
        let bucket = self
            .buckets
            .remove(&sequence)
            .ok_or(Failure::MissingBucket)?;
        self.events.push(format!("remove:{sequence}"));
        for handle in bucket {
            self.events.push(format!("dispose:{}", handle.id));
        }
        Ok(())
    }
    fn dispose_sequence(&mut self, queue: Queue, sequence: u64) -> Result<(), Failure> {
        let q = self.queues.entry(queue).or_default();
        if sequence <= q.prefix {
            return Err(Failure::SequenceAlreadyDisposed);
        }
        let offset = sequence - q.prefix - 1;
        if offset != 0 {
            let offset = usize::try_from(offset).map_err(|_| Failure::SequenceAlreadyDisposed)?;
            if offset < q.ready.len() {
                if q.ready[offset] != 0 {
                    return Err(Failure::DuplicateReadyMarker);
                }
                q.ready[offset] = sequence;
            } else {
                q.ready.resize(offset, 0);
                q.ready.push_back(sequence);
            }
            return Ok(());
        }
        q.prefix = q.prefix.wrapping_add(1);
        self.drain_bucket(sequence)?;
        self.queues.get_mut(&queue).unwrap().ready.pop_front();
        loop {
            let q = self.queues.get_mut(&queue).unwrap();
            let Some(next) = q.ready.front().copied().filter(|v| *v != 0) else {
                break;
            };
            q.ready.pop_front();
            q.prefix = q.prefix.wrapping_add(1);
            if next != q.prefix {
                return Err(Failure::DuplicateReadyMarker);
            }
            self.drain_bucket(next)?;
        }
        Ok(())
    }
    /// 先弹出 pending 再校验；序号处理返回后立即归还命令，即使乱序缺口仍延迟资源回收。
    pub fn reclaim_ready(&mut self, queue: Queue) -> Result<(), Failure> {
        loop {
            let q = self.queues.entry(queue).or_default();
            if q.pending
                .front()
                .is_none_or(|p| p.threshold.0 > q.completed)
            {
                return Ok(());
            }
            let item = q.pending.pop_front().unwrap();
            let command = item.command;
            self.events.push(format!("pop:{}", command.sequence().0));
            if command.state() != 3 {
                return Err(Failure::InvalidPendingState);
            }
            if command.queue()? != queue {
                return Err(Failure::QueueMismatch);
            }
            self.dispose_sequence(queue, command.sequence().0)?;
            if command.pool_family() >= self.pool_family_count {
                return Err(Failure::PoolFamilyBounds);
            }
            self.events.push(format!("pool:{}", command.sequence().0));
            self.pools
                .entry(command.pool_family())
                .or_default()
                .push(command);
        }
    }
    pub fn recycle(&mut self, family: u32) -> Option<Command> {
        self.pools.get_mut(&family)?.pop()
    }
    pub fn prefix(&self, queue: Queue) -> u64 {
        self.queues.get(&queue).map_or(0, |q| q.prefix)
    }
    pub fn ready_markers(&self, queue: Queue) -> Vec<u64> {
        self.queues
            .get(&queue)
            .map_or_else(Vec::new, |q| q.ready.iter().copied().collect())
    }
    pub fn pending_len(&self, queue: Queue) -> usize {
        self.queues.get(&queue).map_or(0, |q| q.pending.len())
    }
    pub fn pool_sequences(&self) -> Vec<u64> {
        self.pools
            .values()
            .flat_map(|v| v.iter().map(|c| c.sequence().0))
            .collect()
    }
    pub fn bucket_keys(&self) -> Vec<u64> {
        self.buckets.keys().copied().collect()
    }
    pub fn resource_state(&self, handle: ResourceHandle) -> Option<(u64, u32)> {
        let r = self.resources.get(&handle.id)?;
        Some((
            r.control.last_sequence.load(Ordering::SeqCst),
            r.control.references.load(Ordering::SeqCst),
        ))
    }
}
