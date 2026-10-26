//! CPU command recording metadata. RecordingId identifies a recording attempt
//! within an ExecutionChain queue; GPU completion is exclusively SyncPoint/SyncSet.
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Queue {
    Primary,
    Auxiliary(u32),
}
impl Queue {
    fn index(self) -> Option<usize> {
        match self {
            Self::Primary => Some(0),
            Self::Auxiliary(index) => usize::try_from(index).ok()?.checked_add(1),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct RecordingId(pub u64);
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CommandState {
    #[default]
    Initial,
    Recording,
    Recorded,
    Submitted,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command {
    pub handle: u64,
    pub family: u32,
    pub recording_id: RecordingId,
    pub queue: Queue,
    pub state: CommandState,
}
impl Command {
    pub fn new(handle: u64, family: u32, queue: Queue) -> Self {
        Self {
            handle,
            family,
            recording_id: RecordingId::default(),
            queue,
            state: CommandState::Initial,
        }
    }
    pub fn handle(&self) -> u64 {
        self.handle
    }

}
pub struct RecordingCounters {
    slots: [AtomicU64; crate::sync::MAX_QUEUES],
    queue_count: usize,
}
impl RecordingCounters {
    pub fn new(auxiliary_count: u32) -> Self {
        Self {
            slots: std::array::from_fn(|_| AtomicU64::new(1)),
            queue_count: (u64::from(auxiliary_count) + 1).min(crate::sync::MAX_QUEUES as u64)
                as usize,
        }
    }
    pub fn next_id(&self, queue: Queue) -> Option<RecordingId> {
        let index = queue.index()?;
        if index >= self.queue_count {
            return None;
        }
        self.slots
            .get(index)?
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                value.checked_add(1)
            })
            .ok()
            .map(RecordingId)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Failure {
    CounterBounds,
    PoolFamilyBounds,
}
#[derive(Debug)]
pub struct BeginFailure {
    pub reason: Failure,
    pub command: Command,
}
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Failure {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rg::executor::{Error, ExecutionChain};
    #[test]
    fn actual_begin_publishes_typed_metadata_only_after_backend_success() {
        let mut chain =
            ExecutionChain::new(ash::vk::Queue::null(), vec![ash::vk::Queue::null()], 4);
        let failure = chain
            .begin(41, 3, Queue::Auxiliary(0), Err(crate::Error::Timeout))
            .unwrap_err();
        assert!(matches!(failure, Error::Validation(crate::Error::Timeout)));
        assert_eq!(chain.recording_command_count(), 0);
        let command = chain.begin(42, 3, Queue::Auxiliary(0), Ok(())).unwrap();
        assert_eq!(command.state, CommandState::Recording);
        assert_eq!(command.recording_id, RecordingId(1));
        assert_eq!(
            (command.handle, command.family, command.queue),
            (42, 3, Queue::Auxiliary(0))
        );
        let next = chain.begin(43, 3, Queue::Auxiliary(0), Ok(())).unwrap();
        assert_eq!(next.recording_id, RecordingId(2));
        let primary = chain.begin(44, 0, Queue::Primary, Ok(())).unwrap();
        assert_eq!(primary.recording_id, RecordingId(1));
        assert_ne!(primary.queue, command.queue);
    }
    #[test]
    fn unknown_queue_and_exhausted_ids_do_not_wrap_to_an_initial_id() {
        let counters = RecordingCounters::new(0);
        assert!(counters.next_id(Queue::Auxiliary(0)).is_none());
        let bounded = RecordingCounters::new(u32::MAX);
        assert!(
            bounded
                .next_id(Queue::Auxiliary(crate::sync::MAX_QUEUES as u32))
                .is_none()
        );
        counters.slots[0].store(u64::MAX, Ordering::Relaxed);
        assert!(counters.next_id(Queue::Primary).is_none());
        assert_eq!(counters.slots[0].load(Ordering::Relaxed), u64::MAX);
    }
}
