//! Synthetic allocation regression probe, not an original game API.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

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

fn main() {
    let library = lang_lib::LanguageLibrary::new("allocation-probe".into(), Default::default());
    let key = "review.missing.translation";
    let first = library.get(key);
    let before = ALLOCATIONS.load(Ordering::Relaxed);
    for _ in 0..10_000 {
        assert!(std::ptr::eq(std::hint::black_box(library.get(key)), first));
    }
    let allocations = ALLOCATIONS.load(Ordering::Relaxed) - before;
    println!("{{\"cached_lookups\":10000,\"allocations\":{allocations}}}");
    assert_eq!(
        allocations, 0,
        "cached missing keys must reuse borrowed lookup"
    );
}
