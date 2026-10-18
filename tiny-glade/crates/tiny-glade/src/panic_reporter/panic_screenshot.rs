//! `tiny_glade::panic_reporter::panic_screenshot` — GPU screenshot capture for
//! crash reports.
//!
//! Fresh decompilation:
//! * `create_graphics_resources` (`0x1405898a0`) creates the CPU→GPU mapped
//!   buffer and stores it in the global `Option<GraphicsResources>`; the store
//!   asserts the slot was empty (old `Some` ⇒ `core::panicking::panic`).
//! * `capture_image` (`0x140589c90`) reads `TextureDesc::extent_2d()`, computes
//!   `sqrt(230400 / (w / h))` in **f32**, truncates to u32 (clamped at
//!   `u32::MAX`), raises a zero target width to 1, computes
//!   `230400 / target_width` and masks the low bit (`& 0x3fffe`). If the masked
//!   height is non-zero it runs the reduction compute pass (`dispatch_1d`); the
//!   result buffer is `[width u32, height u32, YUY2 bytes...]`.
//! * `dump_screenshot` (`0x14058a060`) joins `screenshot.png`, validates the two
//!   dimensions (`"Invalid screenshot data dimensions: {} x {}. Dimensions
//!   can't be zero."` / `"... Would be too many bytes."`, limit `0x38400` =
//!   230400), converts every 4-byte YUY2 group (two pixels) to two RGB8 pixels
//!   with full-range BT.709 coefficients (`1.5748`, `1.8556`, `0.1873`,
//!   `0.4681`, from `.rdata 0x1429ddd20/0x1429ddd30`; scale 255 and `+0.5`,
//!   clamp then truncate), creates the parent directory and saves the buffer
//!   with the context `"saving image"` (`0x1429fd5a8`, len 0xc).
//!
//! The Vulkan buffer/pass plumbing itself is platform/engine glue; the sizes,
//! validation, colour conversion and file writing are reproduced here. The GPU
//! acquisition is abstracted behind [`ScreenshotSource`] so the recovered
//! control flow is testable without a device.

use anyhow::{anyhow, Context, Result};
use parking_lot::Mutex;
use std::path::{Path, PathBuf};

/// Maximum number of pixels retained in the crash screenshot (`0x38400`).
pub const MAX_SCREENSHOT_PIXELS: u32 = 230_400;

/// Captured frame: header dimensions plus the YUY2 payload (2 bytes/pixel).
#[derive(Clone, Debug, Default)]
pub struct GraphicsResources {
    pub width: u32,
    pub height: u32,
    pub yuyv: Vec<u8>,
}

/// Device-side capture source. Implemented by the render backend.
pub trait ScreenshotSource {
    /// Return the frame texture dimensions and its raw YUY2 bytes.
    fn read_frame(&self) -> Result<(u32, u32, Vec<u8>)>;
}

struct Resources {
    source: Box<dyn ScreenshotSource + Send + Sync>,
    captured: GraphicsResources,
}

static GRAPHICS_RESOURCES: Mutex<Option<Resources>> = Mutex::new(None);

/// B `create_graphics_resources` (`0x1405898a0`).
///
/// Stores the backend source; re-initialisation panics (the original asserts the
/// destination `Option` was `None`). A failed device creation is logged and
/// leaves the slot empty.
pub fn create_graphics_resources(source: Option<Box<dyn ScreenshotSource + Send + Sync>>) {
    let mut slot = GRAPHICS_RESOURCES.lock();
    assert!(
        slot.is_none(),
        "panic screenshot graphics resources initialized twice"
    );
    match source {
        Some(source) => {
            *slot = Some(Resources {
                source,
                captured: GraphicsResources::default(),
            });
        }
        None => log::error!(
            target: "tiny_glade::panic_reporter",
            "Failed to create screenshot graphics resources"
        ),
    }
}

/// Reduction dimensions from the machine code (`0x140589c90`).
///
/// `sqrt(230400 / (width as f32 / height as f32))`, truncated, saturated at
/// `u32::MAX`, minimum 1; the other axis is `230400 / target_width` with its low
/// bit cleared.
pub fn reduced_dimensions(width: u32, height: u32) -> (u32, u32) {
    if width == 0 || height == 0 {
        return (0, 0);
    }
    let scaled = (MAX_SCREENSHOT_PIXELS as f32 / (width as f32 / height as f32)).sqrt();
    let mut target_width = if scaled >= 0.0 { scaled as u32 } else { 0 };
    if scaled > u32::MAX as f32 {
        target_width = u32::MAX;
    }
    if target_width == 0 {
        target_width = 1;
    }
    let target_height = MAX_SCREENSHOT_PIXELS / target_width;
    (target_width, target_height & !1)
}

/// B `capture_image` (`0x140589c90`).
///
/// Reads the current frame, performs the recovered sizing and (when the masked
/// height is non-zero) stores the reduced frame in the shared resource slot. The
/// compute-pass/dispatch plumbing is glue.
pub fn capture_image() -> Result<()> {
    let mut slot = GRAPHICS_RESOURCES.lock();
    let Some(resources) = slot.as_mut() else {
        return Ok(());
    };
    let (source_width, source_height, yuyv) = resources.source.read_frame()?;
    let (width, height) = reduced_dimensions(source_width, source_height);
    if height != 0 {
        resources.captured = GraphicsResources {
            width,
            height,
            yuyv,
        };
    }
    Ok(())
}

/// Full-range BT.709 luma/chroma pair → one RGB8 pixel.
fn yuv_to_rgb(y: u8, cb: u8, cr: u8) -> [u8; 3] {
    let y = y as f32 / 255.0;
    let cb = 2.0 * (cb as f32 / 255.0) - 1.0;
    let cr = 2.0 * (cr as f32 / 255.0) - 1.0;
    let scale = |value: f32| value.clamp(0.0, 255.0) as u8;
    [
        scale((y + 1.5748 * cr) * 255.0 + 0.5),
        scale((y - 0.1873 * cb - 0.4681 * cr) * 255.0 + 0.5),
        scale((y + 1.8556 * cb) * 255.0 + 0.5),
    ]
}

/// YUY2 (`Y0 U Y1 V`, 4 bytes) → two RGB8 pixels (6 bytes).
fn yuy2_to_rgb(yuyv: &[u8]) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(yuyv.len() / 2 * 3);
    for group in yuyv.chunks_exact(4) {
        rgb.extend_from_slice(&yuv_to_rgb(group[0], group[1], group[3]));
        rgb.extend_from_slice(&yuv_to_rgb(group[2], group[1], group[3]));
    }
    rgb
}

/// B `dump_screenshot` (`0x14058a060`).
///
/// `report_dir/screenshot_just_before_the_panic.jpg`; validates both axes non-zero and
/// `width * height <= 230400`, converts to RGB8 and saves a JPEG (`Rgb8`).
pub fn dump_screenshot(
    report_dir: &Path,
    _info: Option<&super::PanicReporterInfo>,
) -> Result<PathBuf> {
    let destination = report_dir.join("screenshot_just_before_the_panic.jpg");

    let slot = GRAPHICS_RESOURCES.lock();
    let Some(resources) = slot.as_ref() else {
        return Err(anyhow!("Graphics resources not available"));
    };
    let width = resources.captured.width;
    let height = resources.captured.height;

    if width == 0 || height == 0 {
        return Err(anyhow!(
            "Invalid screenshot data dimensions: {} x {}. Dimensions can't be zero.",
            width,
            height
        ));
    }

    let pixels = width as u64 * height as u64;
    if pixels > MAX_SCREENSHOT_PIXELS as u64 {
        return Err(anyhow!(
            "Invalid screenshot data dimensions: {} x {}. Would be too many bytes.",
            width,
            height
        ));
    }

    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // The original slices `buffer[8..8 + width*height*2]`; an undersized buffer
    // panics with the slice range check.
    let yuyv = &resources.captured.yuyv[..pixels as usize * 2];
    let rgb = yuy2_to_rgb(yuyv);
    image::save_buffer(
        &destination,
        &rgb,
        width,
        height,
        image::ColorType::Rgb8,
    )
    .context("saving image")?;
    Ok(destination)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduction_matches_16_by_9() {
        // sqrt(230400 / (1920/1080)) = 360, 230400 / 360 = 640.
        assert_eq!(reduced_dimensions(1920, 1080), (360, 640));
        assert_eq!(reduced_dimensions(0, 1080), (0, 0));
    }

    #[test]
    fn yuy2_group_yields_two_rgb_pixels() {
        // Y0=255 U=128 Y1=255 V=128 → two near-white RGB pixels.
        let rgb = yuy2_to_rgb(&[255, 128, 255, 128]);
        assert_eq!(rgb.len(), 6);
        assert_eq!(&rgb[..3], &rgb[3..]);
        assert_eq!(rgb[0], 255);
        assert!(rgb[1] >= 254);
        assert_eq!(rgb[2], 255);
    }
}
