//! system-water CPU state chain (water-cpu-analysis.md, water-extra-analysis.md).
//!
//! draw_water (0x141ba9130) -> draw_raster (0x142260ae0) -> rasterize_stroke<512> (0x142261d10)
//! -> copy_to_staging (0x142261760) -> water_rasterize_from_cpu (0x140a12680);
//! replay_raster (0x1409f4650); update_water_audio_grid (0x141baefe0).
//! Each is a bevy_ecs system registered by [`WaterPlugin`].

pub mod audio_grid;
pub mod plugin;
pub mod raster;
pub mod replay;
pub mod stroke;
pub mod tool;

pub use plugin::WaterPlugin;
pub use raster::{BufferSel, RasterBufferRef, WorldRaster, GRID, WORLD_SCALE};
pub use replay::{RecalculateHeightMapCmd, ReplayQueue, ScheduledTerrainReadback};
pub use stroke::{StrokeParams, WriteMode};
pub use tool::{WaterDrawEvent, WaterEraseEvent};

/// `ResMut<WaterRaster>` of the docs: the `WorldRaster<512>` resource.
pub type WaterRaster = WorldRaster;
