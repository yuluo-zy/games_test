//! `systems/world_raster.rs` — world-raster history / GPU-staging upload and
//! the generic `draw_raster` stroke rasterizer.
//!
//! PDB: `record_raster_history @0x140957990`, `upload_world_raster_buffers_to_
//! staging @0x140957bf0`, `draw_raster<...>` (14 monomorph instances:
//! Path/Cobblestone1..3/Garden/Water; water instance @0x142260ae0),
//! `RasterDrawResources`, `RasterHistoryRecordResources`, `RasterStrokeCmd`.
//!
//! O: `record_raster_history` takes a variant index (`idx·0x470`) into the
//! stroke-history resources, Arc-clones the 0xc-strided stroke Vec and records
//! it via `resources::history::History::add`, then resets the Vec (cap 4);
//! `upload_world_raster_buffers_to_staging` runs when the dirty input stream is
//! non-empty → `AllWorldRasterWrite::upload_to_staging`;
//! `draw_raster` rasterizes the stroke (`rasterize_stroke<512>` /
//! `rasterize_world_space_stroke`) and `copy_to_staging<512>` after looking up
//! the staging SSBO (`AssetSsboLibrary`, name `...RasterStagingSsbo`).

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::resources::history::History;
use crate::resources::world_raster::WorldRaster;
use crate::resources::world_raster_defs::AllWorldRasterWrite;
use crate::Gap;

/// `RasterStrokeCmd` (O named entity; stroke record resource, elem 0xc B).
#[derive(Resource, Clone, Debug, Default)]
pub struct RasterStrokeCmd {
    pub raw: Gap,
}

/// `RasterDrawResources` (O named entity).
#[derive(Resource, Clone, Debug, Default)]
pub struct RasterDrawResources {
    pub raw: Gap,
}

/// `RasterHistoryRecordResources` (O named entity; per-variant 0x470-strided
/// record slots).
#[derive(Resource, Clone, Debug, Default)]
pub struct RasterHistoryRecordResources {
    pub raw: Gap,
}

/// `record_raster_history` @0x140957990 (O).
pub fn record_raster_history(
    _variant: Res<RasterHistoryRecordResources>,
    mut _history: ResMut<History>,
    _commands: Commands,
) {
    todo!("recover record_raster_history (variant·0x470 slot, Arc clone, History::add)")
}

/// `upload_world_raster_buffers_to_staging` @0x140957bf0 (O).
pub fn upload_world_raster_buffers_to_staging(
    _dirty: Res<Gap>,
    mut _write: ResMut<AllWorldRasterWrite>,
) {
    let _scope = crate::reused::puffin::Scope::new("upload_world_raster_buffers_to_staging");
    todo!("recover upload_world_raster_buffers_to_staging (dirty-stream gate + upload_to_staging)")
}

/// `draw_raster<Ssbo, WorldRaster<512>, C0, C1, C2>` (O generic template).
pub fn draw_raster(
    _stroke: Vec<glam::Vec3>,
    _raster: &mut WorldRaster<512>,
    _ssbos: Res<AssetSsboLibrary>,
    _variant: u8,
) {
    todo!("recover draw_raster (rasterize_stroke<512> + copy_to_staging<512>)")
}
