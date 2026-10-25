//! Bounded cost probe: timings are observations, correctness checks count work.
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};
use tabloid::{Publisher, State, StateDelta};
struct CountedSystem;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for CountedSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: CountedSystem = CountedSystem;
static CLONES: AtomicUsize = AtomicUsize::new(0);
static APPLIES: AtomicUsize = AtomicUsize::new(0);
struct Fixed([u64; 64]);
impl Default for Fixed {
    fn default() -> Self {
        Self([0; 64])
    }
}
impl Clone for Fixed {
    fn clone(&self) -> Self {
        CLONES.fetch_add(1, Ordering::Relaxed);
        Self(self.0)
    }
}
impl State for Fixed {
    fn empty() -> Self {
        Self::default()
    }
}
impl StateDelta<Fixed> for u64 {
    fn apply_to(&self, s: &mut Fixed) {
        APPLIES.fetch_add(1, Ordering::Relaxed);
        s.0[0] |= self;
    }
}
#[derive(Default)]
struct Growing(Vec<u64>);
impl Clone for Growing {
    fn clone(&self) -> Self {
        CLONES.fetch_add(1, Ordering::Relaxed);
        Self(self.0.clone())
    }
}
impl State for Growing {
    fn empty() -> Self {
        Self::default()
    }
}
impl StateDelta<Growing> for u64 {
    fn apply_to(&self, s: &mut Growing) {
        APPLIES.fetch_add(1, Ordering::Relaxed);
        s.0.push(*self);
    }
}
fn run<T: State>(name: &str)
where
    u64: StateDelta<T>,
{
    for count in [1, 16, 64] {
        CLONES.store(0, Ordering::Relaxed);
        APPLIES.store(0, Ordering::Relaxed);
        let mut publisher = Publisher::<T>::new();
        let subscribers: Vec<_> = (0..count).map(|_| publisher.subscribe()).collect();
        let allocation_start = ALLOCATIONS.load(Ordering::Relaxed);
        let started = Instant::now();
        for value in 0..1000 {
            publisher.publish(value).unwrap();
        }
        let elapsed = started.elapsed();
        let allocations = ALLOCATIONS.load(Ordering::Relaxed) - allocation_start;
        assert_eq!(APPLIES.load(Ordering::Relaxed), 1000);
        assert_eq!(CLONES.load(Ordering::Relaxed), 999);
        drop(subscribers);
        for _ in 0..10_000 {
            drop(publisher.subscribe());
        }
        publisher.publish(1000).unwrap();
        assert_eq!(
            APPLIES.load(Ordering::Relaxed),
            1000,
            "abandoned subscriptions must do no work"
        );
        println!(
            "state={name} subscribers={count} publications=1000 clones=999 applies=1000 allocations={allocations} elapsed_us={}",
            elapsed.as_micros()
        );
    }
}
fn main() {
    run::<Fixed>("fixed");
    run::<Growing>("growing");
}
