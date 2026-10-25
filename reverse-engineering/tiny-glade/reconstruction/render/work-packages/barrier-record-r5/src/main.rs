use std::io::{self, BufRead};
use tiny_glade_barrier_record_r5::{export, initial, pre, Access, Resource};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("input line");
        let values: Vec<u32> = line
            .split_whitespace()
            .map(|field| field.parse().expect("integer field"))
            .collect();
        let resource = Resource {
            kind: values[1] as u8,
            initial: values[2] as u8,
            reused: values[3] != 0,
            validity: values[11] as u8,
        };
        let previous = (values[4] != 256).then_some(Access {
            code: values[4] as u8,
            group: values[5],
            write: values[6] != 0,
        });
        let current = Access {
            code: values[7] as u8,
            group: values[8],
            write: values[9] != 0,
        };
        let outcome = match values[0] {
            0 => pre(resource, previous, current),
            1 => initial(
                &mut vec![(resource, (values[4] != 256).then_some(current))],
                values[10] != 0,
            ),
            2 => export(resource, previous, current.code, values[10] != 0),
            _ => panic!("mode"),
        };
        let transitions = outcome
            .transitions
            .iter()
            .map(|transition| format!("{}:{}:{}", transition.kind, transition.from, transition.to))
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{}|{}|{}|{}|{}|{}",
            outcome.panic.unwrap_or("ok"),
            transitions,
            outcome.pending_length,
            outcome.tracked,
            outcome.emitted,
            outcome.resets
        );
    }
}
