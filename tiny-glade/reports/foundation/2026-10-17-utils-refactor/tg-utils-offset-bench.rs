use std::{
    hint::black_box,
    sync::{
        atomic::{AtomicI32, Ordering},
        RwLock,
    },
    time::Instant,
};
static LOCK: RwLock<i32> = RwLock::new(28_800);
static ATOMIC: AtomicI32 = AtomicI32::new(28_800);
fn main() {
    const N: u32 = 5_000_000;
    for _ in 0..3 {
        let started = Instant::now();
        for _ in 0..N {
            black_box(*black_box(&LOCK).read().unwrap());
        }
        let lock = started.elapsed();
        let started = Instant::now();
        for _ in 0..N {
            black_box(black_box(&ATOMIC).load(Ordering::Relaxed));
        }
        let atomic = started.elapsed();
        println!(
            "reads={N} rwlock_ns={} atomic_ns={}",
            lock.as_nanos(),
            atomic.as_nanos()
        );
    }
}
