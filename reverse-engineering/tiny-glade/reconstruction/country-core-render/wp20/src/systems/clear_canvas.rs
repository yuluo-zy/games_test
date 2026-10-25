//! `systems/clear_canvas.rs` — clear the whole canvas.
//!
//! PDB: `clear_canvas @0x140a7abd0` (6525 B).
//!
//! O: logs "country_core::systems::clear_canvas", then
//! `resources::walls::inner_walls::InnerWalls::remove_all()`, 7×
//! `AssetTextureLibrary::clear_preset_texture()`, and the world rasters
//! (`resources::world_raster::impl$2::clear_all<512>` for each variant).

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::asset_libraries::texture_library::AssetTextureLibrary;
use crate::resources::world_raster::WorldRaster;
use crate::Gap;

/// `clear_canvas` @0x140a7abd0 (O).
pub fn clear_canvas(
    mut _walls: ResMut<Gap>,
    mut _textures: ResMut<AssetTextureLibrary>,
    mut _rasters: (ResMut<WorldRaster<512>>, ResMut<Gap>),
    _commands: Commands,
) {
    todo!("recover clear_canvas (InnerWalls::remove_all + clear_preset_texture + clear_all)")
}
