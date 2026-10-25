//! Original sequential record_batch and private pass helper control-flow projection.
//! Device, synchronization, callback and queue calls remain external boundaries.
use tiny_glade_barrier_record_r5::{pre, Access, Resource};

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
    /// The original FnOnce data pointer is taken before invocation.
    pub callback_present: bool,
}

#[derive(Clone, Debug)]
pub struct Command {
    pub handle: u64,
    pub sequence: u64,
    pub state: u32,
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

impl Recorder {
    /// `callback` is the user-supplied boundary, returning success/failure.
    /// Ordering and partial mutation are recovered from 0x1402c1d40/0x1402c3240.
    pub fn record_batch(
        &mut self,
        passes: &mut [Pass],
        final_batch: bool,
        command: Command,
        mut callback: impl FnMut(usize) -> bool,
    ) -> Outcome {
        let mut out = Outcome::default();
        let Some(initial_command) = self.commands.last() else {
            out.panic = true;
            return out;
        };
        let initial_handle = initial_command.handle;
        // Original length is reset before resolving even the first resource.
        let drained = std::mem::take(&mut self.pending_initial);
        let mut nonempty = false;
        for id in drained {
            let Some(resource) = self.resources.get(id) else {
                out.panic = true;
                return out;
            };
            let Some(first) = resource.first else {
                continue;
            };
            if resource.metadata.reused {
                continue;
            }
            if !validate(resource.metadata) {
                out.panic = true;
                return out;
            }
            event(&mut out, format!("track-barrier:initial:{id}"));
            barrier(
                &mut out,
                "initial",
                id,
                resource.metadata.initial,
                first.code,
            );
            nonempty = true;
        }
        close_scope(&mut out, "initial", initial_handle, nonempty);

        // The supplied command is the result of the device create/begin boundary.
        event(&mut out, format!("create:{}", command.handle));
        event(&mut out, format!("begin:{}", command.handle));
        for (index, pass) in passes.iter_mut().enumerate() {
            for access in &pass.accesses {
                let Some(resource) = self.resources.get_mut(access.resource) else {
                    out.panic = true;
                    return out;
                };
                event(&mut out, format!("track-pass:{}", access.resource));
                // Original tracking silently ignores stale generation. The
                // subsequent barrier validation may still reject that handle.
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
                    nonempty = true;
                }
            }
            close_scope(&mut out, "pass", command.handle, nonempty);
            // No separate post-pass scope exists in this sequential helper.
            if !std::mem::take(&mut pass.callback_present) {
                out.panic = true;
                return out;
            }
            out.consumed_passes += 1;
            event(&mut out, format!("pass:{index}"));
            if !callback(index) {
                out.panic = true;
                return out;
            }
        }
        self.commands.push(command);
        let len = self.commands.len();
        // The old last command and newly appended commands except the last end.
        let command = &mut self.commands[len - 2];
        event(&mut out, format!("end:{}", command.handle));
        command.state = 2;
        if final_batch {
            let mut nonempty = false;
            for (id, resource) in self.resources.iter().enumerate() {
                let Some(target) = resource.export_access else {
                    continue;
                };
                if !validate(resource.metadata) {
                    out.panic = true;
                    return out;
                }
                event(&mut out, format!("track-barrier:export:{id}"));
                barrier(
                    &mut out,
                    "export",
                    id,
                    resource
                        .last
                        .map_or(resource.metadata.initial, |access| access.code),
                    target,
                );
                nonempty = true;
            }
            close_scope(&mut out, "export", self.commands[len - 1].handle, nonempty);
        }
        out
    }

    /// Original submit_pending_cbs take/end/delegate projection. Queue construction
    /// is a capture fixture, not a recovered full queue_submit implementation.
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
}

/// Deterministic synthetic fixture generator; no claim about natural producer
/// reachability is made for the reused marker.
pub fn fixture(v: &[usize]) -> (Recorder, Vec<Pass>) {
    let n = v[0];
    let count = v[1];
    let code = |p: usize| if v[4] == 0 || p & 1 == 0 { 4 } else { 23 };
    let access = |p| Access {
        code: code(p),
        group: v[5] as u32,
        write: v[6] != 0,
    };
    let recorder = Recorder {
        resources: (0..n)
            .map(|id| LogicalResource {
                metadata: Resource {
                    kind: (id % 2) as u8,
                    initial: v[2] as u8,
                    reused: v[3] != 0,
                    validity: if id == v[9] { v[10] as u8 } else { 0 },
                },
                first: (count != 0).then(|| access(0)),
                last: (count != 0).then(|| access(count - 1)),
                export_access: Some(11),
                last_command_sequence: 2,
            })
            .collect(),
        pending_initial: if v[13] != 0 {
            (0..n).collect()
        } else {
            Vec::new()
        },
        commands: vec![Command {
            handle: 99,
            sequence: 6,
            state: 1,
        }],
    };
    let passes = (0..count)
        .map(|p| Pass {
            accesses: (0..n)
                .map(|id| PassAccess {
                    resource: id,
                    previous: (p != 0).then(|| access(p - 1)),
                    current: access(p),
                })
                .collect(),
            callback_present: !(v[7] == 998 && p == 0),
        })
        .collect();
    (recorder, passes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn created() -> Command {
        Command {
            handle: 100,
            sequence: 7,
            state: 1,
        }
    }

    #[test]
    fn failure_preserves_earlier_barriers_and_consumes_callback() {
        let values = [2, 2, 3, 0, 1, 0, 1, 1, 1, 999, 0, 0, 0, 1];
        let (mut recorder, mut passes) = fixture(&values);
        let out = recorder.record_batch(&mut passes, true, created(), |p| p != 1);
        assert!(out.panic);
        assert_eq!(recorder.commands.len(), 1);
        assert!(passes.iter().all(|p| !p.callback_present));
        assert!(out.events.contains(&"emit:pass:100".into()));
        assert!(!out.events.contains(&"end:99".into()));
    }

    #[test]
    fn reused_initial_is_deferred_until_pass_and_callback_sees_barrier() {
        let values = [1, 1, 0, 1, 0, 0, 0, 999, 1, 999, 0, 0, 0, 1];
        let (mut recorder, mut passes) = fixture(&values);
        let out = recorder.record_batch(&mut passes, true, created(), |_| true);
        let barrier = out
            .events
            .iter()
            .position(|e| e == "barrier:pass:0:0:4")
            .unwrap();
        let callback = out.events.iter().position(|e| e == "pass:0").unwrap();
        assert!(barrier < callback);
        assert_eq!(recorder.resources[0].last_command_sequence, 7);
        assert_eq!(
            recorder
                .commands
                .iter()
                .map(|c| c.state)
                .collect::<Vec<_>>(),
            [2, 1]
        );
    }

    #[test]
    fn invalid_later_initial_drains_before_fail_and_skips_create() {
        let values = [2, 2, 3, 0, 1, 0, 1, 999, 1, 1, 3, 0, 0, 1];
        let (mut recorder, mut passes) = fixture(&values);
        let out = recorder.record_batch(&mut passes, true, created(), |_| true);
        assert!(out.panic);
        assert!(recorder.pending_initial.is_empty());
        assert_eq!(
            out.events,
            ["track-barrier:initial:0", "barrier:initial:0:3:4"]
        );
    }

    #[test]
    fn empty_pass_export_uses_initial_and_preserves_tracker_sequence() {
        let values = [2, 0, 255, 0, 0, 0, 0, 999, 1, 999, 0, 0, 0, 1];
        let (mut recorder, mut passes) = fixture(&values);
        let out = recorder.record_batch(&mut passes, true, created(), |_| true);
        assert!(!out.panic);
        assert!(out.events.contains(&"barrier:export:1:255:11".into()));
        assert_eq!(recorder.resources[0].last_command_sequence, 2);
    }

    #[test]
    fn submit_error_keeps_last_open_in_taken_storage_and_clears_executor() {
        let values = [1, 2, 3, 0, 1, 0, 1, 999, 1, 999, 0, 1, 1, 1];
        let (mut recorder, mut passes) = fixture(&values);
        let mut out = recorder.record_batch(&mut passes, true, created(), |_| true);
        recorder.submit_pending(&mut out, true, (0, 41));
        assert!(out.panic);
        assert!(recorder.commands.is_empty());
        assert_eq!(out.taken_command_states, Some(vec![2, 1]));
        assert_eq!(out.receipt, None);
    }
}
