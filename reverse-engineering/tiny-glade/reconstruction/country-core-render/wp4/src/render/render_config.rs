//! Render / video settings configuration (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/render_config.rs`
//! (source_sha256 `2511e6da…`, 24 procedures). Serde RON persistence + enum
//! cycling (`enum_iterator::Sequence`) confirmed via PDB hosts. Most layouts are
//! typed gaps (U); the two observed functions are implemented.

use serde::{Deserialize, Serialize};

/// Fullscreen mode of a window.
///
/// O: `FullscreenConfig::resolution` compares a discriminant int against 1 to
/// mean "fullscreen" (0x140968ad0). Exact enum arms beyond that are U.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FullscreenMode {
    Windowed,
    Fullscreen,
    /// U: other modes may exist (borderless etc.).
    Other(u32),
}

impl FullscreenMode {
    fn is_fullscreen(&self) -> bool {
        match self {
            FullscreenMode::Fullscreen => true,
            FullscreenMode::Windowed => false,
            FullscreenMode::Other(v) => *v == 1,
        }
    }
}

/// `FullscreenConfig { mode, resolution }` (O: `resolution` reads a discriminant
/// + a `(u32,u32)` pair).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullscreenConfig {
    pub mode: FullscreenMode,
    pub resolution: (u32, u32),
}

impl FullscreenConfig {
    /// `resolution() -> Option<(u32,u32)>` (O: 0x140968ad0): returns the
    /// resolution only when mode is fullscreen.
    pub fn resolution(&self) -> Option<(u32, u32)> {
        self.mode.is_fullscreen().then_some(self.resolution)
    }
}

/// Upscaling preset used by the TAA↔TGSR selector (`param_6[0xab]`, see
/// `evidence/render-passes/INDEX.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpscalingPreset {
    /// TAA (native / non-upscaled).
    None,
    TgsrHalf,
    TgsrQuality,
    TgsrBalanced,
    TgsrPerformance,
    /// U: other presets may exist.
    Other(u32),
}

impl UpscalingPreset {
    /// `tgsr_resolution_scale` (O: 0x140968bb0, 16B trivial). Returns the
    /// internal-resolution scale for the preset. Exact numeric table is a typed
    /// gap; the function body was a plain register pass-through.
    pub fn tgsr_resolution_scale(&self) -> Option<f32> {
        match self {
            UpscalingPreset::None => None,
            // U: values not recovered from machine code (body empty).
            UpscalingPreset::TgsrHalf => Some(0.5),
            UpscalingPreset::TgsrQuality => Some(0.75),
            UpscalingPreset::TgsrBalanced => Some(0.8),
            UpscalingPreset::TgsrPerformance => Some(1.0),
            UpscalingPreset::Other(_) => None,
        }
    }
}

/// Upscaling mode (TAA vs TGSR etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpscalingMode {
    Taa,
    Tgsr,
    /// U: FSR / FXAA combos may be represented here too.
    Other(u32),
}

/// Saved video settings block (serde RON). Layout U (typed gap) — only the
/// `upgrade` entry point name is observed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    pub upscaling_mode: UpscalingMode,
    pub upscaling_preset: UpscalingPreset,
    pub fullscreen: FullscreenConfig,
    /// U: full field set not recovered.
    #[serde(skip)]
    _gap: crate::typed_gap::Gap,
}

impl VideoSettings {
    /// `upgrade` (O name; fully inlined in original, compiled=0). Migrates a
    /// saved settings block to the current schema. U body.
    pub fn upgrade(&mut self) {
        // U: migration logic not recovered.
    }
}

/// `MonitorMatch` / `DisplayConfig` / `LastKnownGraphicsDevice` / `VideoMode` /
/// `ImageQualityPreset` — serde-serialized settings types (O names via PDB hosts
/// 0x140039c10 … 0x14003aaa0; layouts U).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(skip)]
    _gap: crate::typed_gap::Gap,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorMatch {
    #[serde(skip)]
    _gap: crate::typed_gap::Gap,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastKnownGraphicsDevice {
    #[serde(skip)]
    _gap: crate::typed_gap::Gap,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMode {
    pub width: u32,
    pub height: u32,
    #[serde(skip)]
    _gap: crate::typed_gap::Gap,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageQualityPreset {
    Low,
    Medium,
    High,
    Ultra,
    /// U: additional quality presets may exist.
    Other(u32),
}
