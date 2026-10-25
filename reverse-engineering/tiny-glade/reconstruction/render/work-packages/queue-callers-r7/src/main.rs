use std::io::{self, BufRead};
use tiny_glade_queue_callers_r7::{ensure_active_cb, CommandSnapshot, FixtureBackend};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let values: Vec<u64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let [length, family, fill, sequence] = values[..] else {
            panic!("expected length family fill sequence");
        };
        let mut commands = vec![CommandSnapshot([fill as u8; 88]); length as usize];
        let mut backend = FixtureBackend {
            fill: fill as u8,
            sequence,
            ..Default::default()
        };
        ensure_active_cb(&mut commands, family as u32, &mut backend);
        let bytes: String = commands
            .iter()
            .flat_map(|v| v.0)
            .map(|v| format!("{v:02x}"))
            .collect();
        println!("{}|{}|{}", commands.len(), backend.events.join(","), bytes);
    }
}
