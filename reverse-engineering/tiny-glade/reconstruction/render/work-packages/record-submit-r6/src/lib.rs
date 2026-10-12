use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Create(u64),
    Begin(u64),
    Pass(usize),
    End(u64),
    Exports,
    EndLast,
    Queue(usize),
    VulkanSubmit(u32),
    ConvertError,
    PostSubmit,
}

impl fmt::Display for Event {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(handle) => write!(formatter, "create:{handle}"),
            Self::Begin(handle) => write!(formatter, "begin:{handle}"),
            Self::Pass(index) => write!(formatter, "pass:{index}"),
            Self::End(handle) => write!(formatter, "end:{handle}"),
            Self::Exports => formatter.write_str("exports"),
            Self::EndLast => formatter.write_str("end:last"),
            Self::Queue(count) => write!(formatter, "queue:{count}"),
            Self::VulkanSubmit(count) => write!(formatter, "vk-submit:{count}"),
            Self::ConvertError => formatter.write_str("convert-error"),
            Self::PostSubmit => formatter.write_str("post-submit"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    pub events: Vec<Event>,
    pub panic: bool,
    pub pending_length: usize,
    pub command_states: Vec<u32>,
    pub consumed_passes: usize,
    pub receipt: Option<(u32, u64)>,
}

pub fn record(pass_count: usize, fail_at: Option<usize>, final_batch: bool) -> Outcome {
    let mut outcome = Outcome::default();
    outcome.command_states.push(1);
    outcome.events.push(Event::Create(100));
    outcome.events.push(Event::Begin(100));
    for pass in 0..pass_count {
        outcome.consumed_passes += 1;
        outcome.events.push(Event::Pass(pass));
        if fail_at == Some(pass) {
            outcome.panic = true;
            return outcome;
        }
    }
    outcome.command_states = vec![2, 1];
    outcome.events.push(Event::End(99));
    if final_batch {
        outcome.events.push(Event::Exports);
    }
    outcome
}

pub fn submit(mut command_states: Vec<u32>, end_error: bool, wait_only: bool) -> Outcome {
    let mut outcome = Outcome::default();
    if command_states.is_empty() {
        outcome.panic = wait_only;
        return outcome;
    }
    if command_states.last() != Some(&1) {
        outcome.panic = true;
        outcome.command_states = command_states;
        return outcome;
    }
    outcome.events.push(Event::EndLast);
    if end_error {
        outcome.panic = true;
        outcome.command_states = command_states;
        return outcome;
    }
    *command_states.last_mut().expect("nonempty") = 2;
    outcome.events.push(Event::Queue(command_states.len()));
    outcome.command_states = command_states;
    outcome.receipt = Some((0, 41));
    outcome
}

pub fn queue_call(count: u32, backend_error: bool, converted_success: bool) -> Outcome {
    let mut outcome = Outcome::default();
    outcome.events.push(Event::VulkanSubmit(count));
    if backend_error {
        outcome.events.push(Event::ConvertError);
        if !converted_success {
            outcome.panic = true;
            return outcome;
        }
    }
    outcome.events.push(Event::PostSubmit);
    outcome.receipt = Some((0, 41));
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_failure_does_not_append_command() {
        let outcome = record(3, Some(1), true);
        assert!(outcome.panic);
        assert_eq!(outcome.command_states, [1]);
        assert_eq!(outcome.consumed_passes, 2);
    }

    #[test]
    fn submit_ends_only_last_open_command() {
        let outcome = submit(vec![2, 1], false, false);
        assert_eq!(outcome.command_states, vec![2, 2]);
        assert_eq!(outcome.events, [Event::EndLast, Event::Queue(2)]);
    }

    #[test]
    fn empty_wait_only_panics_after_take() {
        assert!(submit(vec![], false, true).panic);
        assert!(!submit(vec![], false, false).panic);
    }
}
