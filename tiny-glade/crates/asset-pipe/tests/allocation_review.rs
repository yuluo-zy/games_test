use asset::texture::desc::{CompressionMethod, Gamma, MipChain, TextureOpts};
use asset_pipe::texture::{compress, process_decoded_image};
use image::{DynamicImage, Rgba, RgbaImage};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct ObservedAllocator;
static OBSERVE: AtomicBool = AtomicBool::new(false);
static FULL_RGBA_ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
const SIDE: u32 = 512;
const RGBA_BYTES: usize = (SIDE * SIDE * 4) as usize;

// This integration test is a separate, single-test process. The counter observes
// allocations of exactly one source RGBA image, excluding fixture construction.
unsafe impl GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if OBSERVE.load(Ordering::Relaxed) && layout.size() == RGBA_BYTES {
            FULL_RGBA_ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        // SAFETY: preserve the caller's allocation layout.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: forward the unchanged pointer/layout pair to its allocator.
        unsafe { System.dealloc(ptr, layout) }
    }
}
#[global_allocator]
static ALLOCATOR: ObservedAllocator = ObservedAllocator;

#[test]
fn linear_encoding_reuses_owned_rgba_pixels_without_a_full_image_copy() {
    let source = RgbaImage::from_fn(SIDE, SIDE, |x, y| {
        Rgba([x as u8, y as u8, (x ^ y) as u8, 255])
    });
    let expected = compress(&source, 2).unwrap();
    let opts = TextureOpts {
        gamma: Gamma::Linear,
        compression: CompressionMethod::Color,
        mips: MipChain::None,
        resize: None,
    };
    FULL_RGBA_ALLOCATIONS.store(0, Ordering::Relaxed);
    OBSERVE.store(true, Ordering::Relaxed);
    let output = process_decoded_image(DynamicImage::ImageRgba8(source), &opts).unwrap();
    OBSERVE.store(false, Ordering::Relaxed);
    let copies = FULL_RGBA_ALLOCATIONS.load(Ordering::Relaxed);
    println!(
        "full_rgba_allocations={copies}; avoided_copy_bytes={} per image",
        RGBA_BYTES * 2
    );
    assert_eq!(output.data, expected);
    assert_eq!(output.dimensions, [SIDE, SIDE, 1]);
    assert_eq!(output.format, 145);
    assert_eq!(
        copies, 0,
        "an already-owned linear RGBA image needs no copy before BC7"
    );
    let source = RgbaImage::from_pixel(SIDE, SIDE, Rgba([13, 27, 41, 255]));
    let source_pointer = source.as_raw().as_ptr();
    let output = process_decoded_image(
        DynamicImage::ImageRgba8(source),
        &TextureOpts {
            compression: CompressionMethod::None,
            ..opts
        },
    )
    .unwrap();
    assert_eq!(
        output.data.as_ptr(),
        source_pointer,
        "uncompressed single mip transfers the source allocation"
    );
}
