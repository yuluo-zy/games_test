use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering},
};
#[path = "../../crates/utils/src/subtractive_1d_boolean.rs"]
mod intervals;
struct Counter;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counter {
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
static COUNTER: Counter = Counter;
fn main() {
    let mut ranges = intervals::SubtractiveBoolean1D::new_range(0.0, 10.0);
    let cut = intervals::Interval {
        start: 2.0,
        end: 8.0,
        upper_exclusive: false,
    };
    ranges.subtract(cut);
    ranges.subtract(cut);
    let before = ALLOCATIONS.load(Ordering::Relaxed);
    for _ in 0..20_000 {
        ranges.subtract(std::hint::black_box(cut));
    }
    let allocations = ALLOCATIONS.load(Ordering::Relaxed) - before;
    assert_eq!(ranges.result().len(), 2);
    assert_eq!(allocations, 0);
    println!("20,000 warmed repeated interval cuts: {allocations} allocations/reallocations");
}
