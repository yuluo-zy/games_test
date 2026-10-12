use std::io::{self, BufRead};
use tiny_glade_record_submit_r6::{queue_call, record, submit};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("input");
        let values: Vec<usize> = line
            .split_whitespace()
            .map(|field| field.parse().expect("integer"))
            .collect();
        let outcome = match values[0] {
            0 => record(
                values[1],
                (values[2] != 999).then_some(values[2]),
                values[3] != 0,
            ),
            1 => submit(
                (0..values[1])
                    .map(|index| {
                        if index + 1 == values[1] {
                            values[2] as u32
                        } else {
                            2
                        }
                    })
                    .collect(),
                values[3] != 0,
                values[4] != 0,
            ),
            2 => queue_call(values[1] as u32, values[2] != 0, values[3] != 0),
            3 => {
                let recorded = record(values[1], None, values[3] != 0);
                let mut submitted = submit(recorded.command_states, values[2] != 0, false);
                let mut events = recorded.events;
                events.append(&mut submitted.events);
                submitted.events = events;
                submitted.consumed_passes = recorded.consumed_passes;
                submitted
            }
            _ => panic!("mode"),
        };
        let states = outcome
            .command_states
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let receipt = outcome
            .receipt
            .map_or("none".into(), |(queue, serial)| format!("{queue}:{serial}"));
        let events = outcome
            .events
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{}|{}|{}|{}|{}|{}",
            u8::from(outcome.panic),
            events,
            outcome.pending_length,
            states,
            outcome.consumed_passes,
            receipt
        );
    }
}
