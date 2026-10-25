//! Typed consumers of observed per-queue CB sequence production. Vulkan and
//! object allocation remain external boundaries. Global buckets use plain u64.
pub use device_counters_r6::DeviceCounters;
use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use tg_gpu_lifecycle_r3::{CommandState, ControlBlock, TrackOutcome, drop_reference, track_handle};

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
    pub fn producer(self) -> device_counters_r6::QueueIdentity {
        match self {
            Self::Primary => device_counters_r6::QueueIdentity::Primary,
            Self::Auxiliary(i) => device_counters_r6::QueueIdentity::Auxiliary(i),
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
    /// Keeps the observed sequence; there is no +/-1 fixture translation.
    pub fn record_command(&self) -> tiny_glade_record_resources_r7::Command {
        tiny_glade_record_resources_r7::Command {
            handle: self.handle(),
            sequence: self.sequence().0,
            state: self.state(),
        }
    }
    pub fn caller_snapshot(&self) -> tiny_glade_queue_callers_r7::CommandSnapshot {
        tiny_glade_queue_callers_r7::CommandSnapshot(self.bytes)
    }
    pub fn from_caller_snapshot(snapshot: tiny_glade_queue_callers_r7::CommandSnapshot) -> Self {
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
    /// Called after the actual backend's reset/begin result. This function consumes
    /// the validated R6A device producer, rather than a default zero seed.
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
    /// Entry for an already begun native/backend command. This does NOT increment
    /// a counter again. Duplicate insertion replaces before the original assertion.
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
                control: ControlBlock {
                    last_sequence: AtomicU64::new(0),
                    references: AtomicU32::new(1),
                    generation,
                },
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
    /// The recorder's resource sequence is an observed tracker output, not a
    /// queue timeline. This lets the R7 recorder bind its real Command sequence.
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
        tg_gpu_lifecycle_r3::clone_reference(&r.control);
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
    /// Queue submission adapter receives actual per-CB thresholds assigned by the
    /// queue backend. It never makes CB sequences into semaphore values. The
    /// pending command bytes may be transferred unchanged from a native backend.
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
    /// Successful queue_submit turns ended CBs to tag3. This convenience adapter
    /// only materializes the recovered callback result; no Vulkan submit is made.
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
    /// Original query stores the reported value directly, including a synthetic
    /// regressing query fixture. Monotonicity is a Vulkan/runtime precondition.
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
    /// Pop pending BEFORE validation, and pool the CB immediately after sequence
    /// disposal returns, even if an out-of-order ready gap defers resource drain.
    pub fn reclaim_ready(&mut self, queue: Queue) -> Result<(), Failure> {
        loop {
            let q = self.queues.entry(queue).or_default();
            if !q
                .pending
                .front()
                .is_some_and(|p| p.threshold.0 <= q.completed)
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

#[cfg(test)]
mod tests {
    use super::*;
    fn begin(r: &mut SequenceRuntime, c: &DeviceCounters, handle: u64) -> Command {
        r.begin_after_backend(c, Command::new(handle, 0), Queue::Primary, Ok(()))
            .unwrap()
    }
    #[test]
    fn real_one_flows_without_fixture_translation() {
        let c = DeviceCounters::new(0);
        let mut r = SequenceRuntime::new(1);
        let cmd = begin(&mut r, &c, 100);
        assert_eq!(cmd.sequence(), Sequence(1));
        let h = r.create_resource(0, 7);
        r.track(h, &cmd).unwrap();
        r.cpu_retire(h);
        r.drop_handle(h).unwrap();
        r.collect_waste();
        let mut ended = cmd;
        ended.set_state(2);
        r.accept_queue_success(Queue::Primary, vec![(ended, Timeline(41))]);
        r.observe_timeline(Queue::Primary, Timeline(40));
        r.reclaim_ready(Queue::Primary).unwrap();
        assert!(r.pool_sequences().is_empty());
        assert!(!r.events.contains(&"dispose:0".into()));
        r.observe_timeline(Queue::Primary, Timeline(41));
        r.reclaim_ready(Queue::Primary).unwrap();
        assert_eq!(r.pool_sequences(), [1]);
        assert_eq!(r.prefix(Queue::Primary), 1);
        assert!(r.events.contains(&"dispose:0".into()));
    }
    #[test]
    fn out_of_order_recycles_cb_before_resource_gap_closes() {
        let c = DeviceCounters::new(0);
        let mut r = SequenceRuntime::new(1);
        let one = begin(&mut r, &c, 100);
        let two = begin(&mut r, &c, 101);
        let h = r.create_resource(0, 7);
        r.track(h, &two).unwrap();
        r.drop_handle(h).unwrap();
        r.collect_waste();
        r.accept_queue_success(Queue::Primary, vec![(two, Timeline(4))]);
        r.observe_timeline(Queue::Primary, Timeline(4));
        r.reclaim_ready(Queue::Primary).unwrap();
        assert_eq!(r.pool_sequences(), [2]);
        assert_eq!(r.ready_markers(Queue::Primary), [0, 2]);
        assert!(r.bucket_keys().contains(&2));
        r.accept_queue_success(Queue::Primary, vec![(one, Timeline(9))]);
        r.observe_timeline(Queue::Primary, Timeline(9));
        r.reclaim_ready(Queue::Primary).unwrap();
        assert_eq!(r.prefix(Queue::Primary), 2);
        assert!(r.bucket_keys().is_empty());
    }
    #[test]
    fn duplicate_replaces_bucket_before_failure() {
        let c = DeviceCounters::new(1);
        let mut r = SequenceRuntime::new(1);
        let one = begin(&mut r, &c, 100);
        let h = r.create_resource(0, 7);
        r.track(h, &one).unwrap();
        r.drop_handle(h).unwrap();
        r.collect_waste();
        let error = r
            .begin_after_backend(&c, Command::new(101, 0), Queue::Auxiliary(0), Ok(()))
            .unwrap_err();
        assert_eq!(error.reason, Failure::DuplicateBucket);
        assert_eq!(error.command.sequence(), Sequence(1));
        assert_eq!(
            c.next_value(device_counters_r6::QueueIdentity::Auxiliary(0)),
            Some(2)
        );
        assert!(r.buckets[&1].is_empty());
    }
    #[test]
    fn failed_reset_does_not_consume_counter() {
        let c = DeviceCounters::new(0);
        let mut r = SequenceRuntime::new(1);
        assert_eq!(
            r.begin_after_backend(
                &c,
                Command::new(100, 0),
                Queue::Primary,
                Err(Failure::BackendReset)
            )
            .unwrap_err()
            .reason,
            Failure::BackendReset
        );
        assert_eq!(
            c.next_value(device_counters_r6::QueueIdentity::Primary),
            Some(1)
        );
        assert!(r.bucket_keys().is_empty());
    }
    #[test]
    fn validation_failure_is_after_pending_pop() {
        let c = DeviceCounters::new(0);
        let mut r = SequenceRuntime::new(1);
        let mut cmd = begin(&mut r, &c, 100);
        cmd.set_state(2);
        r.accept_submission(
            SubmissionReceipt {
                queue: Queue::Primary,
                last_timeline: Timeline(1),
            },
            vec![PendingCommand {
                command: cmd,
                threshold: Timeline(1),
            }],
        );
        r.observe_timeline(Queue::Primary, Timeline(1));
        assert_eq!(
            r.reclaim_ready(Queue::Primary),
            Err(Failure::InvalidPendingState)
        );
        assert_eq!(r.pending_len(Queue::Primary), 0);
        assert_eq!(r.bucket_keys(), [1]);
    }
    #[test]
    fn recorder_receives_actual_command_and_tracker() {
        let c = DeviceCounters::new(0);
        let mut r = SequenceRuntime::new(1);
        let initial = begin(&mut r, &c, 99);
        let new = begin(&mut r, &c, 100);
        let h = r.create_resource(0, 7);
        let (mut recorder, mut passes) = tiny_glade_record_resources_r7::fixture(&[
            1, 1, 3, 0, 0, 0, 0, 999, 1, 999, 0, 0, 0, 1,
        ]);
        recorder.commands = vec![initial.record_command()];
        recorder.resources[0].last_command_sequence = 0;
        let out = recorder.record_batch(&mut passes, true, new.record_command(), |_| true);
        assert!(!out.panic);
        assert_eq!(recorder.resources[0].last_command_sequence, 2);
        r.import_record_tracker(h, Sequence(recorder.resources[0].last_command_sequence))
            .unwrap();
        r.drop_handle(h).unwrap();
        r.collect_waste();
        assert_eq!(r.buckets[&2], [h]);
        assert_eq!(Command::from_caller_snapshot(new.caller_snapshot()), new);
    }
}
