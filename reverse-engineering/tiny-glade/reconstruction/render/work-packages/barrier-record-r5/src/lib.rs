#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Access {
    pub code: u8,
    pub group: u32,
    pub write: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resource {
    pub kind: u8,
    pub initial: u8,
    pub reused: bool,
    pub validity: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transition {
    pub kind: u8,
    pub from: u8,
    pub to: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    pub transitions: Vec<Transition>,
    pub panic: Option<&'static str>,
    pub pending_length: usize,
    pub tracked: usize,
    pub emitted: usize,
    pub resets: usize,
}

pub fn concurrent(current: Access, previous: Access) -> bool {
    current.code == previous.code
        && (!(current.write || previous.write)
            || (current.group != 0 && current.group == previous.group))
}

fn validate(resource: Resource) -> Result<(), &'static str> {
    if resource.kind > 1 {
        return Err("kind");
    }
    match resource.validity {
        1 => Err("device"),
        2 => Err("managed-kind"),
        3 => Err("generation"),
        _ => Ok(()),
    }
}

pub fn pre(resource: Resource, previous: Option<Access>, current: Access) -> Outcome {
    let mut outcome = Outcome::default();
    let from = match previous {
        Some(previous) => {
            if resource.kind > 1 {
                outcome.panic = Some("kind");
                return outcome;
            }
            if concurrent(current, previous) {
                return outcome;
            }
            previous.code
        }
        None => {
            if !resource.reused {
                return outcome;
            }
            if resource.initial != 0 {
                outcome.panic = Some("initial-nonzero");
                return outcome;
            }
            0
        }
    };
    if let Err(panic) = validate(resource) {
        outcome.panic = Some(panic);
        return outcome;
    }
    outcome.transitions.push(Transition {
        kind: resource.kind,
        from,
        to: current.code,
    });
    outcome
}

pub fn initial(pending: &mut Vec<(Resource, Option<Access>)>, command_present: bool) -> Outcome {
    let mut outcome = Outcome::default();
    if !command_present {
        outcome.panic = Some("command-empty");
        outcome.pending_length = pending.len();
        return outcome;
    }
    let drained = std::mem::take(pending);
    for (resource, first) in drained {
        let Some(first) = first else { continue };
        if resource.reused {
            continue;
        }
        if let Err(panic) = validate(resource) {
            outcome.panic = Some(panic);
            return outcome;
        }
        outcome.tracked += 1;
        outcome.transitions.push(Transition {
            kind: resource.kind,
            from: resource.initial,
            to: first.code,
        });
    }
    outcome.emitted = usize::from(!outcome.transitions.is_empty());
    outcome.resets = 1;
    outcome
}

pub fn export(resource: Resource, last: Option<Access>, target: u8, final_batch: bool) -> Outcome {
    let mut outcome = Outcome::default();
    if !final_batch {
        return outcome;
    }
    if let Err(panic) = validate(resource) {
        outcome.panic = Some(panic);
        return outcome;
    }
    outcome.tracked = 1;
    outcome.transitions.push(Transition {
        kind: resource.kind,
        from: last.map_or(resource.initial, |access| access.code),
        to: target,
    });
    outcome.emitted = 1;
    outcome.resets = 1;
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(kind: u8) -> Resource {
        Resource {
            kind,
            initial: 3,
            reused: false,
            validity: 0,
        }
    }

    fn access(code: u8) -> Access {
        Access {
            code,
            group: 0,
            write: false,
        }
    }

    #[test]
    fn pending_is_drained_before_partial_panic() {
        let mut invalid = resource(1);
        invalid.validity = 3;
        let mut pending = vec![(resource(0), Some(access(4))), (invalid, Some(access(5)))];
        let result = initial(&mut pending, true);
        assert!(pending.is_empty());
        assert_eq!(result.panic, Some("generation"));
        assert_eq!(result.transitions.len(), 1);
        assert_eq!(result.resets, 0);
    }

    #[test]
    fn empty_command_panics_before_drain() {
        let mut pending = vec![(resource(0), Some(access(4)))];
        assert_eq!(initial(&mut pending, false).pending_length, 1);
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn reused_initial_and_pre_have_opposite_roles() {
        let mut reused = resource(0);
        reused.reused = true;
        reused.initial = 0;
        let mut pending = vec![(reused, Some(access(4)))];
        assert!(initial(&mut pending, true).transitions.is_empty());
        assert_eq!(pre(reused, None, access(4)).transitions.len(), 1);
        assert!(pre(resource(0), None, access(4)).transitions.is_empty());
    }

    #[test]
    fn concurrent_write_needs_nonzero_equal_group() {
        let mut current = access(4);
        let mut previous = current;
        assert!(concurrent(current, previous));
        current.write = true;
        assert!(!concurrent(current, previous));
        current.group = 7;
        previous.group = 7;
        assert!(concurrent(current, previous));
    }
}
