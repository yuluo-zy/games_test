use std::io::{self, BufRead};
use tiny_glade_record_resources_r7::{fixture, Command};

fn main() {
    for line in io::stdin().lock().lines() {
        let values: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (mut recorder, mut passes) = fixture(&values);
        let command = Command {
            handle: 100,
            sequence: 7,
            state: 1,
        };
        let mut out =
            recorder.record_batch(&mut passes, values[8] != 0, command, |p| p != values[7]);
        if values[11] != 0 && !out.panic {
            recorder.submit_pending(&mut out, values[12] != 0, (0, 41));
        }
        let states = out.taken_command_states.as_ref().map_or_else(
            || {
                recorder
                    .commands
                    .iter()
                    .map(|c| c.state.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            },
            |states| {
                states
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            },
        );
        let sequence = recorder
            .resources
            .iter()
            .map(|r| r.last_command_sequence.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let receipt = out
            .receipt
            .map_or("none".into(), |(q, v)| format!("{q}:{v}"));
        println!(
            "{}|{}|{}|{}|{}|{}|{}",
            u8::from(out.panic),
            out.events.join(","),
            recorder.pending_initial.len(),
            states,
            out.consumed_passes,
            sequence,
            receipt
        );
    }
}
