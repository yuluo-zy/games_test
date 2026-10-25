use std::io::{self, BufRead};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use tg_gpu_lifecycle_r3::{
    CommandState, ControlBlock, clone_reference, drop_reference, track_handle, track_sequence,
};
fn number(text: &str) -> u64 {
    u64::from_str_radix(text, 16).unwrap()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split_whitespace().collect();
        let old = number(fields[1]);
        match fields[0] {
            "track" => {
                let control = ControlBlock {
                    last_sequence: AtomicU64::new(old),
                    references: AtomicU32::new(1),
                    generation: number(fields[2]) as u32,
                };
                let result = track_handle(
                    &control,
                    number(fields[3]) as u32,
                    CommandState {
                        tag: number(fields[4]) as u32,
                        sequence: number(fields[5]),
                    },
                );
                println!(
                    "{} {:x}",
                    if result.is_ok() { "ok" } else { "panic" },
                    control.last_sequence.load(Ordering::SeqCst)
                );
            }
            "seq" => {
                let tracker = AtomicU64::new(old);
                let result = track_sequence(
                    &tracker,
                    CommandState {
                        tag: number(fields[2]) as u32,
                        sequence: number(fields[3]),
                    },
                );
                println!(
                    "{} {:x}",
                    if result.is_ok() { "ok" } else { "panic" },
                    tracker.load(Ordering::SeqCst)
                );
            }
            "clone" => {
                let control = ControlBlock {
                    last_sequence: AtomicU64::new(0),
                    references: AtomicU32::new(old as u32),
                    generation: 0,
                };
                println!("{:x}", clone_reference(&control));
            }
            "drop" => {
                let control = ControlBlock {
                    last_sequence: AtomicU64::new(0),
                    references: AtomicU32::new(old as u32),
                    generation: 0,
                };
                let result = drop_reference(&control, number(fields[2]) != 0);
                println!(
                    "{} {:x}",
                    if result.is_err() { "panic" } else { "ok" },
                    control.references.load(Ordering::SeqCst)
                );
            }
            _ => panic!("unknown operation"),
        }
    }
}
