//! B screenshot orchestration; one per-App request owner in screenshot_capture.
//! Native tag 6 means awaiting an opportunity. It is never completion.
//! File images capture into GpuToCpu buffers; menu/LUT capture into shared textures.
use anyhow::{Result, anyhow};
use image::ImageEncoder;
use std::io::Write;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
#[path = "screenshot_capture.rs"]
pub mod shared_texture_capture;
pub type Completion = Arc<dyn Fn(std::result::Result<[u32; 2], String>) + Send + Sync>;
/// Byte values are native, including the three opportunities in compile_main.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ScreenshotPurpose {
    GradingReference = 0,
    LutPreview = 1,
    MenuBackground = 2,
    SceneBeforeUi = 3,
    SceneWithUi = 4,
    FinalFrame = 6,
}
impl ScreenshotPurpose {
    pub fn as_byte(self) -> u8 {
        self as u8
    }
}
#[derive(Clone)]
pub enum ScreenshotSink {
    File {
        path: PathBuf,
        jpeg_quality: u8,
    },
    /// An actual pixel consumer, never a fabricated Steam success result.
    Rgba(Arc<dyn Fn(u32, u32, Vec<u8>) + Send + Sync>),
}
pub struct ScreenshotRequest {
    pub purpose: ScreenshotPurpose,
    pub dimensions: Option<[u32; 2]>,
    pub sink: ScreenshotSink,
    pub completion: Option<Completion>,
}
/// Native gather's map closure initializes state=0, remaining=3, tag=6.
/// Allocation happens at the matching graph opportunity, never here.
pub fn gather_screenshot_requests(
    world: &mut bevy_ecs::world::World,
    requests: impl IntoIterator<Item = ScreenshotRequest>,
) -> rhapsody::Result<()> {
    for request in requests {
        shared_texture_capture::enqueue(world, request)?;
    }
    Ok(())
}
pub fn active_capture_purpose(world: &bevy_ecs::world::World) -> u8 {
    shared_texture_capture::active_capture_purpose(world)
}
pub fn has_active_screenshot_requests(world: &bevy_ecs::world::World) -> bool {
    shared_texture_capture::has_requests(world)
}
pub fn finish_screenshot_capture(world: &mut bevy_ecs::world::World) -> rhapsody::Result<()> {
    shared_texture_capture::finish(world)
}
pub fn hurry_active_screenshots_on_shutdown(
    world: &mut bevy_ecs::world::World,
) -> rhapsody::Result<()> {
    shared_texture_capture::shutdown(world)
}
pub fn write_image_with_callback_impl(
    path: &Path,
    width: u32,
    height: u32,
    rgba: &[u8],
    jpeg_quality: u8,
    callback: &dyn Fn(std::result::Result<[u32; 2], String>),
) {
    let result = write_image(path, width, height, rgba, jpeg_quality)
        .map(|()| [width, height])
        .map_err(|error| format!("Error writing screenshot image: {error}"));
    callback(result);
}

fn write_image(path: &Path, width: u32, height: u32, rgba: &[u8], jpeg_quality: u8) -> Result<()> {
    let expected = (width as usize)
        .checked_mul(height as usize)
        .and_then(|n| n.checked_mul(4))
        .ok_or_else(|| anyhow!("Failed to create ImageBuffer"))?;
    if rgba.len() < expected {
        return Err(anyhow!("Failed to create ImageBuffer"));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // The original requires a second `parent()`; `None` is an error, not a
    // fallback to the system temp directory.
    let parent = path.parent().ok_or_else(|| anyhow!("path has no parent"))?;
    let temp = tempfile::NamedTempFile::new_in(parent)?;

    if path.extension().and_then(|extension| extension.to_str()) == Some("jpg") {
        let mut rgb = Vec::with_capacity(width as usize * height as usize * 3);
        for pixel in rgba[..expected].chunks_exact(4) {
            rgb.extend_from_slice(&pixel[..3]);
        }
        let mut writer = std::io::BufWriter::new(temp.as_file());
        {
            let encoder =
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, jpeg_quality);
            encoder.write_image(&rgb, width, height, image::ExtendedColorType::Rgb8)?;
        }
        writer.flush()?;
    } else {
        let format = image::ImageFormat::from_path(path)?;
        let image = image::RgbaImage::from_raw(width, height, rgba[..expected].to_vec())
            .ok_or_else(|| anyhow!("Failed to create ImageBuffer"))?;
        image.write_to(&mut temp.as_file(), format)?;
    }

    temp.persist(path).map_err(|error| error.error)?;
    Ok(())
}
