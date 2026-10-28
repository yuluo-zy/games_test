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
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
pub enum Failure {
    CounterBounds,
    PoolFamilyBounds,
}
#[derive(Debug)]
pub struct BeginFailure {
    pub reason: Failure,
    pub command: Command,
}

// Recording failures keep their structured detail and actionable category.
impl From<Failure> for crate::Error {
    fn from(source: Failure) -> Self {
        Self::invalid_usage_with("渲染状态无效", source)
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod error_display_tests {
    use super::*;

    #[test]
    fn display_text_is_stable() {
        assert_eq!(Failure::CounterBounds.to_string(), "CounterBounds");
    }
}
