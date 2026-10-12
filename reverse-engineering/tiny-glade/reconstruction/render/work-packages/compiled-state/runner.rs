#[path = "lib.rs"]
mod candidate;

use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let values: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        let mut compiler = candidate::BatchCompiler {
            batches: Vec::new(),
            current_range: values[0]..values[1],
            current_export_barriers: values[2] != 0,
        };
        compiler.flush_batch(values[3]);
        if let Some(batch) = compiler.batches.first() {
            println!(
                "{} {} {} {} {} {} {}",
                compiler.batches.len(),
                compiler.current_range.start,
                compiler.current_range.end,
                usize::from(compiler.current_export_barriers),
                batch.pass_range.start,
                batch.pass_range.end,
                usize::from(batch.export_barriers)
            );
        } else {
            println!(
                "0 {} {} {}",
                compiler.current_range.start,
                compiler.current_range.end,
                usize::from(compiler.current_export_barriers)
            );
        }
    }
}
