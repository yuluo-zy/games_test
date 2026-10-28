//! Shared actual System allocator instrumentation. Callers choose the counted phase.
#![allow(dead_code)] // each test binary uses a different subset of the shared helpers
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::{Cell, RefCell},
    time::Instant,
};

thread_local! {
    static COUNTING: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static BYTES: Cell<u64> = const { Cell::new(0) };
    static ALLOCATION_SIZES: RefCell<[u64;32]> = const { RefCell::new([0;32]) };
}
#[cfg(unix)]
pub(crate) mod allocation_trace {
    use std::{
        cell::{Cell, RefCell},
        ffi::c_void,
    };
    unsafe extern "C" {
        fn backtrace(frames: *mut *mut c_void, capacity: i32) -> i32;
        fn dladdr(address: *const c_void, info: *mut DlInfo) -> i32;
    }
    #[repr(C)]
    struct DlInfo {
        file: *const std::ffi::c_char,
        base: *mut c_void,
        symbol: *const std::ffi::c_char,
        symbol_address: *mut c_void,
    }
    thread_local! {
        pub static ENABLED: Cell<bool> = const { Cell::new(false) };
        pub static START: Cell<usize> = const { Cell::new(0) };
        static FRAMES: RefCell<[[usize; 32]; 32]> = const { RefCell::new([[0;32];32]) };
    }
    pub fn capture(index: usize) {
        let start = START.with(Cell::get);
        if index < start || index - start >= 32 || !ENABLED.with(Cell::get) {
            return;
        }
        // This is a separate diagnostic frame. Exclude unwinder internals from
        // the production System count, and use only fixed-size trace storage.
        super::COUNTING.with(|counting| counting.set(false));
        let mut frames = [std::ptr::null_mut(); 32];
        let count = unsafe { backtrace(frames.as_mut_ptr(), frames.len() as i32) };
        FRAMES.with(|stored| {
            let mut stored = stored.borrow_mut();
            for (target, frame) in stored[index - start]
                .iter_mut()
                .zip(frames)
                .take(count.max(0) as usize)
            {
                *target = frame as usize;
            }
        });
        super::COUNTING.with(|counting| counting.set(true));
    }
    pub fn report(sample: &super::Sample) {
        FRAMES.with(|frames| {
            for (index, frames) in frames.borrow().iter().enumerate().take(
                sample
                    .allocations
                    .saturating_sub(START.with(Cell::get) as u64)
                    .min(32) as usize,
            ) {
                let mut info = DlInfo {
                    file: std::ptr::null(),
                    base: std::ptr::null_mut(),
                    symbol: std::ptr::null(),
                    symbol_address: std::ptr::null_mut(),
                };
                let first = frames.iter().copied().find(|frame| *frame != 0).unwrap();
                assert_ne!(unsafe { dladdr(first as *const c_void, &mut info) }, 0);
                let addresses = frames
                    .iter()
                    .filter(|frame| **frame != 0)
                    .map(|frame| format!("0x{frame:x}"))
                    .collect::<Vec<_>>()
                    .join(" ");
                println!(
                    "ALLOCATION_TRACE {} bytes={} image_base=0x{:x} addresses={addresses}",
                    index + START.with(Cell::get),
                    sample.allocation_sizes[index],
                    info.base as usize
                );
            }
        });
    }
}

struct CountingAllocator;
fn record(bytes: usize) {
    if COUNTING.try_with(Cell::get).unwrap_or(false) {
        let _ = ALLOCATIONS.try_with(|count| {
            let index = count.get() as usize;
            #[cfg(unix)]
            let start = allocation_trace::START.with(Cell::get);
            #[cfg(not(unix))]
            let start = 0;
            if index >= start && index - start < 32 {
                let _ = ALLOCATION_SIZES
                    .try_with(|sizes| sizes.borrow_mut()[index - start] = bytes as u64);
            }
            count.set(count.get() + 1);
            #[cfg(unix)]
            allocation_trace::capture(index);
        });
        let _ = BYTES.try_with(|count| count.set(count.get() + bytes as u64));
    }
}
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record(layout.size());
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record(size);
        unsafe { System.realloc(pointer, layout, size) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }
}
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;
#[derive(Clone, Copy, Default, serde::Serialize)]
pub(crate) struct Sample {
    pub(crate) allocations: u64,
    pub(crate) allocated_bytes: u64,
    pub(crate) elapsed_ns: u64,
    pub(crate) allocation_sizes: [u64; 32],
}
pub(crate) fn measured<T>(f: impl FnOnce() -> T) -> (T, Sample) {
    struct Stop;
    impl Drop for Stop {
        fn drop(&mut self) {
            COUNTING.with(|enabled| enabled.set(false));
        }
    }
    ALLOCATIONS.with(|count| count.set(0));
    BYTES.with(|count| count.set(0));
    ALLOCATION_SIZES.with(|sizes| *sizes.borrow_mut() = [0; 32]);
    COUNTING.with(|enabled| enabled.set(true));
    let stop = Stop;
    let start = Instant::now();
    let output = f();
    let elapsed_ns = start.elapsed().as_nanos() as u64;
    drop(stop);
    (
        output,
        Sample {
            allocations: ALLOCATIONS.with(Cell::get),
            allocated_bytes: BYTES.with(Cell::get),
            elapsed_ns,
            allocation_sizes: ALLOCATION_SIZES.with(|sizes| *sizes.borrow()),
        },
    )
}
