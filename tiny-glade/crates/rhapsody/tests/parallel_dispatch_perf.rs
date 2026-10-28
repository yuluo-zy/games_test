//! Manual probe: cost of dispatching recording chunks on the compute pool
//! versus running them inline.
//! `cargo test -p rhapsody --release --test parallel_dispatch_perf -- --ignored --nocapture`
use rhapsody::rg::map_chunks;
use std::time::{Duration, Instant};

/// Busy-wait: models CPU recording work without sleeping the thread.
fn spin(work: Duration) {
    let end = Instant::now() + work;
    while Instant::now() < end {
        std::hint::spin_loop();
    }
}

fn mean_ns(iterations: u32, mut run: impl FnMut()) -> u128 {
    for _ in 0..200 {
        run();
    }
    let started = Instant::now();
    for _ in 0..iterations {
        run();
    }
    started.elapsed().as_nanos() / u128::from(iterations)
}

#[test]
#[ignore = "manual performance probe"]
fn dispatch_cost_versus_inline() {
    println!("chunks work/chunk_us inline_ns pooled_ns overhead_ns");
    for chunks in [1usize, 2, 4, 8] {
        for work_us in [0u64, 2, 5, 10, 25] {
            let work = Duration::from_micros(work_us);
            let mut items = vec![0u8; chunks];
            let inline = mean_ns(2000, || {
                // The serial alternative: every chunk on the calling thread.
                for item in items.iter_mut() {
                    spin(work);
                    *item = item.wrapping_add(1);
                }
            });
            let pooled = mean_ns(2000, || {
                map_chunks(&mut items, |_, item| {
                    spin(work);
                    *item = item.wrapping_add(1);
                });
            });
            // Overhead is what pooled costs beyond the ideal parallel time
            // (one chunk's work, all chunks running concurrently).
            let ideal = u128::from(work_us) * 1000;
            println!(
                "{chunks:>6} {work_us:>13} {inline:>9} {pooled:>9} {:>11}",
                pooled.saturating_sub(ideal)
            );
        }
    }
}
