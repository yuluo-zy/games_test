//! `crate::systems::render_loop::draw_passes::draw_shaders` — draw-shader
//! pass modules（WP7：水域 draw pass `water_planes_draw`；WP18：per-shader
//! raster draw 组合 `billboard_forest` … `stars`）。

pub mod billboard_forest;
pub mod clearing_shaders;
pub mod clouds;
pub mod common_includes;
pub mod draw_bricks;
pub mod floor_shader;
pub mod grass_shader;
pub mod instanced_critters;
pub mod instanced_ivy;
pub mod leaf_particle_draw;
pub mod path_pebble_shader;
pub mod rocky_terrain_draw;
pub mod stairs_draw;
pub mod stars;
pub mod water_planes_draw;
