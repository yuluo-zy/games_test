//! Convert raw graphics initialization categories into localized display data.

// These tags describe this input format, not a Rust enum discriminant or FFI layout.
const TAG_NO_COMPATIBLE_DEVICES_A: u64 = 0x0b;
const TAG_NO_COMPATIBLE_DEVICES_B: u64 = 0x0c;
const TAG_UNSUPPORTED_WINDOWING_SYSTEM: u64 = 0x12;
const TAG_SWAPCHAIN_ERROR: u64 = 0x14;
const TAG_OUT_OF_VIDEO_MEMORY: u64 = 0x15;
const TAG_INTERNAL_ERROR: u64 = 0x16;
const TAG_UNEXPECTED_ERROR: u64 = 0x17;

/// Caller-provided raw category and diagnostic text from graphics initialization.
/// Unknown tags use the generic missing-driver presentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicsInitError {
    pub tag: u64,
    pub details: String,
}
/// Display keys and optional borrowed diagnostic text. This projection does not load locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicsErrorView<'a> {
    /// A localization key, except the fallback literal `Unexpected error`.
    pub heading: &'static str,
    pub details: Option<&'a str>,
    pub recommendation: Option<&'static str>,
}
/// Select display keys without allocating a second copy of the diagnostic text.
/// Returned details remain valid for the lifetime of `error`.
pub fn localize_graphics_init_error(error: &GraphicsInitError) -> GraphicsErrorView<'_> {
    let mut recommendation = Some("GraphicsInitError.PleaseUpdateDrivers");
    let mut details = None;
    let heading = match error.tag {
        TAG_UNSUPPORTED_WINDOWING_SYSTEM => "GraphicsInitError.UnsupportedWindowingSystem",
        TAG_NO_COMPATIBLE_DEVICES_A | TAG_NO_COMPATIBLE_DEVICES_B => {
            "GraphicsInitError.NoCompatibleDevices"
        }
        TAG_SWAPCHAIN_ERROR => {
            details = Some(error.details.as_str());
            recommendation = Some("GraphicsInitError.SwapchainErrorTip");
            "GraphicsInitError.SwapchainError"
        }
        TAG_OUT_OF_VIDEO_MEMORY => {
            details = Some(error.details.as_str());
            recommendation = Some("GraphicsInitError.OutOfVideoMemoryRecommendation");
            "GraphicsInitError.OutOfVideoMemory"
        }
        TAG_INTERNAL_ERROR => {
            details = Some(error.details.as_str());
            "GraphicsInitError.InternalError"
        }
        TAG_UNEXPECTED_ERROR => {
            details = Some(error.details.as_str());
            recommendation = None;
            "Unexpected error"
        }
        _ => "GraphicsInitError.NoDriver",
    };
    GraphicsErrorView {
        heading,
        details,
        recommendation,
    }
}
