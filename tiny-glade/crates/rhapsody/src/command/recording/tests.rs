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
