use std::io::{self, BufRead};
use tiny_glade_temporal_lifecycle_r8::{should_reset_temporal, snapshot_fixture};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let v: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if v[0] == 2 {
            println!("{}", u8::from(should_reset_temporal(v[1] as u32)));
        } else {
            println!(
                "{}",
                snapshot_fixture(v[0] as u32, v[1] != 0, v[2] != 0, v[3] as u8, v[4] as u8)
            );
        }
    }
}
