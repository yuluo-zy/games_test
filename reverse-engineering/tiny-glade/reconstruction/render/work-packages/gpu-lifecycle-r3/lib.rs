use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[derive(Debug)]
pub struct ControlBlock {
    pub last_sequence: AtomicU64,
    pub references: AtomicU32,
    pub generation: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandState {
    pub tag: u32,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackOutcome {
    Updated,
    GenerationSkipped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    InvalidCommandState,
    MovedHandleDrop,
    DuplicateBucket,
    MissingBucket,
    DuplicateCompletion,
    UnknownHandle,
    CrossDomainMappingUnknown,
    TimelineRegression,
    DeviceMismatch,
    ResourceKindMismatch,
    DisposalGenerationMismatch,
    ReferencesRemain,
}

pub fn track_sequence(tracker: &AtomicU64, command: CommandState) -> Result<(), Failure> {
    if command.tag != 1 {
        return Err(Failure::InvalidCommandState);
    }
    tracker.fetch_max(command.sequence, Ordering::SeqCst);
    Ok(())
}

pub fn track_handle(
    control: &ControlBlock,
    handle_generation: u32,
    command: CommandState,
) -> Result<TrackOutcome, Failure> {
    if control.generation != handle_generation {
        return Ok(TrackOutcome::GenerationSkipped);
    }
    track_sequence(&control.last_sequence, command)?;
    Ok(TrackOutcome::Updated)
}

pub fn clone_reference(control: &ControlBlock) -> u32 {
    control
        .references
        .fetch_add(1, Ordering::SeqCst)
        .wrapping_add(1)
}

pub fn drop_reference(control: &ControlBlock, moved: bool) -> Result<bool, Failure> {
    if moved {
        return Err(Failure::MovedHandleDrop);
    }
    Ok(control
        .references
        .fetch_sub(1, Ordering::SeqCst)
        .wrapping_sub(1)
        == 0)
}

pub fn dispose_control(
    control: &mut ControlBlock,
    handle_generation: u32,
    handle_device: u16,
    device: u16,
    resource_kind: u8,
    mut external_dispose: impl FnMut(),
) -> Result<(), Failure> {
    if handle_device != device {
        return Err(Failure::DeviceMismatch);
    }
    if resource_kind != 1 {
        return Err(Failure::ResourceKindMismatch);
    }
    if control.generation != handle_generation {
        return Err(Failure::DisposalGenerationMismatch);
    }
    external_dispose();
    if control.references.load(Ordering::SeqCst) != 0 {
        return Err(Failure::ReferencesRemain);
    }
    control.generation = control.generation.wrapping_add(1);
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommandKey {
    pub domain: u32,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Handle {
    pub identity: u64,
    pub generation: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Create(Handle),
    Track(Handle, CommandKey),
    GenerationSkipped(Handle),
    CpuDrop(Handle, u32),
    WasteQueued(Handle, Option<CommandKey>),
    WasteAttached(Handle, CommandKey),
    CpuPoolReturn(Handle),
    Submit {
        queue: u32,
        timeline: u64,
        commands: Vec<CommandKey>,
    },
    Complete {
        queue: u32,
        timeline: u64,
    },
    ReclaimCommand(CommandKey),
    ExternalDispose(Handle),
}

#[derive(Debug)]
struct Resource {
    control: ControlBlock,
    last_use: Option<CommandKey>,
    dropped: bool,
}

#[derive(Default, Debug)]
pub struct EventOracle {
    next_identity: u64,
    resources: HashMap<u64, Resource>,
    waste: Vec<(Handle, Option<CommandKey>)>,
    buckets: BTreeMap<CommandKey, Vec<Handle>>,
    pending: BTreeMap<u32, Vec<(u64, Vec<CommandKey>)>>,
    completed: BTreeMap<u32, u64>,
    disposed_prefix: BTreeMap<u32, u64>,
    ready: BTreeSet<CommandKey>,
    pub events: Vec<Event>,
}

impl EventOracle {
    pub fn create(&mut self, generation: u32) -> Handle {
        let handle = Handle {
            identity: self.next_identity,
            generation,
        };
        self.next_identity += 1;
        self.resources.insert(
            handle.identity,
            Resource {
                control: ControlBlock {
                    last_sequence: AtomicU64::new(0),
                    references: AtomicU32::new(1),
                    generation,
                },
                last_use: None,
                dropped: false,
            },
        );
        self.events.push(Event::Create(handle));
        handle
    }

    pub fn begin(&mut self, key: CommandKey) -> Result<(), Failure> {
        if self.buckets.insert(key, Vec::new()).is_some() {
            return Err(Failure::DuplicateBucket);
        }
        Ok(())
    }

    pub fn track(
        &mut self,
        handle: Handle,
        key: CommandKey,
        tag: u32,
    ) -> Result<TrackOutcome, Failure> {
        let resource = self
            .resources
            .get_mut(&handle.identity)
            .ok_or(Failure::UnknownHandle)?;
        let outcome = track_handle(
            &resource.control,
            handle.generation,
            CommandState {
                tag,
                sequence: key.sequence,
            },
        )?;
        if outcome == TrackOutcome::GenerationSkipped {
            self.events.push(Event::GenerationSkipped(handle));
            return Ok(outcome);
        }
        if resource
            .last_use
            .is_some_and(|previous| previous.domain != key.domain)
        {
            return Err(Failure::CrossDomainMappingUnknown);
        }
        resource.last_use = Some(CommandKey {
            domain: key.domain,
            sequence: resource.control.last_sequence.load(Ordering::SeqCst),
        });
        self.events.push(Event::Track(handle, key));
        Ok(outcome)
    }

    pub fn clone_handle(&self, handle: Handle) -> Result<u32, Failure> {
        let resource = self
            .resources
            .get(&handle.identity)
            .ok_or(Failure::UnknownHandle)?;
        Ok(clone_reference(&resource.control))
    }

    pub fn cpu_pool_return(&mut self, handle: Handle) {
        self.events.push(Event::CpuPoolReturn(handle));
    }

    pub fn cpu_drop(&mut self, handle: Handle) -> Result<(), Failure> {
        let resource = self
            .resources
            .get_mut(&handle.identity)
            .ok_or(Failure::UnknownHandle)?;
        let last = drop_reference(&resource.control, resource.dropped)?;
        let count = resource.control.references.load(Ordering::SeqCst);
        self.events.push(Event::CpuDrop(handle, count));
        if last {
            resource.dropped = true;
            self.waste.push((handle, resource.last_use));
            self.events
                .push(Event::WasteQueued(handle, resource.last_use));
        }
        Ok(())
    }

    pub fn take_out_waste(&mut self) {
        for (handle, key) in self.waste.drain(..) {
            if let Some(key) = key.and_then(|key| {
                self.buckets.get_mut(&key).map(|bucket| {
                    bucket.push(handle);
                    key
                })
            }) {
                self.events.push(Event::WasteAttached(handle, key));
            } else {
                self.events.push(Event::ExternalDispose(handle));
            }
        }
    }

    pub fn submit(
        &mut self,
        queue: u32,
        timeline: u64,
        commands: Vec<CommandKey>,
    ) -> Result<(), Failure> {
        let pending = self.pending.entry(queue).or_default();
        if pending
            .last()
            .is_some_and(|previous| previous.0 >= timeline)
        {
            return Err(Failure::TimelineRegression);
        }
        pending.push((timeline, commands.clone()));
        self.events.push(Event::Submit {
            queue,
            timeline,
            commands,
        });
        Ok(())
    }

    pub fn complete(&mut self, queue: u32, timeline: u64) -> Result<(), Failure> {
        if self
            .completed
            .get(&queue)
            .is_some_and(|previous| *previous > timeline)
        {
            return Err(Failure::TimelineRegression);
        }
        self.completed.insert(queue, timeline);
        self.events.push(Event::Complete { queue, timeline });
        let pending = self.pending.entry(queue).or_default();
        let count = pending
            .iter()
            .take_while(|record| record.0 <= timeline)
            .count();
        let commands: Vec<_> = pending.drain(..count).flat_map(|record| record.1).collect();
        for key in commands {
            let prefix = self.disposed_prefix.get(&key.domain).copied().unwrap_or(0);
            if key.sequence <= prefix || !self.ready.insert(key) {
                return Err(Failure::DuplicateCompletion);
            }
        }
        let domains: BTreeSet<_> = self.ready.iter().map(|key| key.domain).collect();
        for domain in domains {
            let prefix = self.disposed_prefix.entry(domain).or_default();
            loop {
                let next = CommandKey {
                    domain,
                    sequence: prefix.wrapping_add(1),
                };
                if !self.ready.remove(&next) {
                    break;
                }
                *prefix = next.sequence;
                let bucket = self.buckets.remove(&next).ok_or(Failure::MissingBucket)?;
                self.events.push(Event::ReclaimCommand(next));
                for handle in bucket {
                    self.events.push(Event::ExternalDispose(handle));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn key(sequence: u64) -> CommandKey {
        CommandKey {
            domain: 0,
            sequence,
        }
    }
    #[test]
    fn last_drop_waits_for_contiguous_gpu_completion() {
        let mut oracle = EventOracle::default();
        oracle.begin(key(1)).unwrap();
        oracle.begin(key(2)).unwrap();
        let handle = oracle.create(7);
        oracle.track(handle, key(2), 1).unwrap();
        oracle.cpu_pool_return(handle);
        oracle.cpu_drop(handle).unwrap();
        oracle.take_out_waste();
        assert!(!oracle.events.contains(&Event::ExternalDispose(handle)));
        oracle.submit(0, 10, vec![key(1)]).unwrap();
        oracle.submit(1, 3, vec![key(2)]).unwrap();
        oracle.complete(1, 3).unwrap();
        assert!(!oracle.events.contains(&Event::ExternalDispose(handle)));
        oracle.complete(0, 10).unwrap();
        assert!(oracle.events.contains(&Event::ExternalDispose(handle)));
    }
    #[test]
    fn stale_generation_skips_even_bad_command_state() {
        let control = ControlBlock {
            last_sequence: AtomicU64::new(9),
            references: AtomicU32::new(1),
            generation: 2,
        };
        assert_eq!(
            track_handle(
                &control,
                1,
                CommandState {
                    tag: 2,
                    sequence: 10
                }
            ),
            Ok(TrackOutcome::GenerationSkipped)
        );
        assert_eq!(control.last_sequence.load(Ordering::SeqCst), 9);
        assert_eq!(
            track_handle(
                &control,
                2,
                CommandState {
                    tag: 2,
                    sequence: 10
                }
            ),
            Err(Failure::InvalidCommandState)
        );
    }
    #[test]
    fn drop_order_and_max_tracking() {
        let mut oracle = EventOracle::default();
        oracle.begin(key(1)).unwrap();
        oracle.begin(key(2)).unwrap();
        let first = oracle.create(1);
        let second = oracle.create(2);
        oracle.track(first, key(2), 1).unwrap();
        oracle.track(first, key(1), 1).unwrap();
        oracle.track(second, key(1), 1).unwrap();
        oracle.clone_handle(first).unwrap();
        oracle.cpu_drop(second).unwrap();
        oracle.cpu_drop(first).unwrap();
        oracle.take_out_waste();
        assert!(!oracle.events.contains(&Event::ExternalDispose(first)));
        oracle.cpu_drop(first).unwrap();
        oracle.take_out_waste();
        oracle.submit(0, 1, vec![key(1), key(2)]).unwrap();
        oracle.complete(0, 1).unwrap();
        assert!(oracle.events.contains(&Event::ExternalDispose(first)));
        assert!(oracle.events.contains(&Event::ExternalDispose(second)));
        assert_eq!(oracle.cpu_drop(first), Err(Failure::MovedHandleDrop));
    }
    #[test]
    fn empty_submission_and_duplicate_bucket() {
        let mut oracle = EventOracle::default();
        oracle.submit(0, 1, vec![]).unwrap();
        oracle.complete(0, 1).unwrap();
        assert!(
            !oracle
                .events
                .iter()
                .any(|event| matches!(event, Event::ReclaimCommand(_)))
        );
        oracle.begin(key(1)).unwrap();
        assert_eq!(oracle.begin(key(1)), Err(Failure::DuplicateBucket));
        oracle.submit(0, 2, vec![key(1)]).unwrap();
        oracle.complete(0, 2).unwrap();
        oracle.submit(1, 1, vec![key(1)]).unwrap();
        assert_eq!(oracle.complete(1, 1), Err(Failure::DuplicateCompletion));
    }
    #[test]
    fn late_drop_disposes_after_bucket_is_gone() {
        let mut oracle = EventOracle::default();
        oracle.begin(key(1)).unwrap();
        let handle = oracle.create(1);
        oracle.track(handle, key(1), 1).unwrap();
        oracle.submit(0, 1, vec![key(1)]).unwrap();
        oracle.complete(0, 1).unwrap();
        oracle.cpu_drop(handle).unwrap();
        oracle.take_out_waste();
        assert!(oracle.events.contains(&Event::ExternalDispose(handle)));
    }
    #[test]
    fn disposal_generation_reuse_and_failure_event_order() {
        let mut control = ControlBlock {
            last_sequence: AtomicU64::new(0),
            references: AtomicU32::new(1),
            generation: u32::MAX,
        };
        let mut calls = 0;
        assert_eq!(
            dispose_control(&mut control, u32::MAX, 2, 2, 1, || calls += 1),
            Err(Failure::ReferencesRemain)
        );
        assert_eq!(calls, 1);
        control.references.store(0, Ordering::SeqCst);
        assert_eq!(
            dispose_control(&mut control, u32::MAX, 2, 2, 1, || calls += 1),
            Ok(())
        );
        assert_eq!(control.generation, 0);
        assert_eq!(
            dispose_control(&mut control, u32::MAX, 2, 2, 1, || calls += 1),
            Err(Failure::DisposalGenerationMismatch)
        );
        assert_eq!(calls, 2);
    }
}
