//! Render-facing core types recovered from the original Tiny Glade PDB skeleton.
//!
//! WP4 (mesh / shader / render-config / view-frame constants / asset-library wiring).
//! Recovered from version B (patched `tiny-glade.exe`, PDB `2d9686b8…`).
//!
//! Evidence:
//!   contracts/*.md in this package
//!   `_analysis/decomp/*.c` (Ghidra `--worker shared` decompiles)
//!   `evidence/render-architecture/symbol-seeds.json`, `public-symbols.json`
//!
//! Marking legend used across this package:
//!   O = observed (machine code / PDB), I = inferred, U = unknown (typed gap).

pub mod adaptive_render_state;
pub mod atmosphere;
pub mod debug_info_buffers;
pub mod directional_light;
pub mod frame_constants;
pub mod fsr_plugin;
pub mod load_texture;
pub mod mesh;
pub mod render_config;
pub mod shader;
pub mod view_constants;

/// Bevy resource wrapping the raw graphics device.
///
/// Deref implementations (`impl$0::deref`, `impl$1::deref`) were observed as
/// procedure hosts in module 521 (0x140987b30 / 0x140987b40); the inner device is
/// the REUSED `rhapsody::raw::device::GraphicsDevice`.
pub struct GraphicsDeviceResource(pub crate::rhapsody_contract::GraphicsDevice);

impl core::ops::Deref for GraphicsDeviceResource {
    type Target = crate::rhapsody_contract::GraphicsDevice;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A physical graphics adapter as enumerated at startup.
///
/// 160 bytes / element (O: `AvailableGraphicsDevices::from_preference_order`
/// swaps 0xa0-byte elements). Exact field layout is a typed gap.
pub struct AvailableGraphicsDevice {
    /// O: opaque 160-byte record; fields unknown.
    _raw: [u64; 20],
}

/// Ordered list of available graphics devices.
pub struct AvailableGraphicsDevices(pub Vec<AvailableGraphicsDevice>);

impl AvailableGraphicsDevices {
    /// `from_preference_order(devices)` — reverses the preference list so the
    /// first entry is the most-preferred device (O: pairwise 0xa0-byte element
    /// swap loop, 0x140987cb0).
    pub fn from_preference_order(mut devices: Vec<AvailableGraphicsDevice>) -> Self {
        let mid = devices.len() / 2;
        for i in 0..mid {
            let j = devices.len() - 1 - i;
            let (a, b) = devices.split_at_mut(j);
            core::mem::swap(&mut a[i], &mut b[0]);
        }
        Self(devices)
    }
}
