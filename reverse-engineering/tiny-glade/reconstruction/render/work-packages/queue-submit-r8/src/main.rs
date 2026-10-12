use std::io::{self, BufRead};
use tiny_glade_queue_submit_r8::{fixture, format_outcome};
fn main() {
    for line in io::stdin().lock().lines() {
        let v = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i128>().unwrap() as u64)
            .collect::<Vec<_>>();
        let (mut device, id, input) = fixture(&v);
        let outcome = device.queue_submit(id, input, |_, _| (v[3] as i32, v[4] != 0));
        println!("{}", format_outcome(&device, &outcome));
    }
}
