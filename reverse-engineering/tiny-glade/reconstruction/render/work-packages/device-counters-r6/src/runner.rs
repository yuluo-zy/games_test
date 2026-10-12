use device_counters_r6::raw_begin_projection;
use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let values = line
            .split_whitespace()
            .map(|value| u64::from_str_radix(value, 16).unwrap())
            .collect::<Vec<_>>();
        let tag = values[0] as u32;
        let index = values[1] as u32;
        let mut counters = values[2..].to_vec();
        match raw_begin_projection(&mut counters, tag, index) {
            Some(sequence) => print!("ok {sequence:x}"),
            None => print!("bounds"),
        }
        for counter in counters {
            print!(" {counter:x}");
        }
        println!();
    }
}
