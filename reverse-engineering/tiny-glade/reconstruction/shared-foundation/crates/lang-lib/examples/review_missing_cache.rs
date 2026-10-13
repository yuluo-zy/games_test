//! Allocation regression probe for cached lookups and repeated pack lifetimes.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountedSystem;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountedSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            LIVE_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        let next = unsafe { System.realloc(ptr, layout, size) };
        if !next.is_null() {
            LIVE_BYTES.fetch_add(size, Ordering::Relaxed);
            LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
        }
        next
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
        assert!(std::sync::Arc::ptr_eq(
            &std::hint::black_box(library.get(key)),
            &first
        ));
    }
    let allocations = ALLOCATIONS.load(Ordering::Relaxed) - before;
    println!("{{\"cached_lookups\":10000,\"allocations\":{allocations}}}");
    assert_eq!(
        allocations, 0,
        "cached missing keys must reuse their owned string"
    );
    // Warm up parser/thread-local state, then measure complete pack lifetimes.
    fn load_and_drop() {
        let pack = lang_lib::LanguageLibrary::parse_yml("key: value\n", "probe".into()).unwrap();
        assert_eq!(&*pack.get("key"), "value");
        let _ = pack.get("missing");
    }
    load_and_drop();
    let live_before = LIVE_BYTES.load(Ordering::Relaxed);
    for _ in 0..1000 {
        load_and_drop();
    }
    let live_after = LIVE_BYTES.load(Ordering::Relaxed);
    assert_eq!(
        live_after, live_before,
        "discarded packs must reclaim all owned allocations"
    );
    println!("{{\"pack_loads\":1000,\"retained_bytes\":0}}");
}
