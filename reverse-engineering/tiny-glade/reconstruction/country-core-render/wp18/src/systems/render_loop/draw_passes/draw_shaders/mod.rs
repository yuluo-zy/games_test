//! `draw_shaders` — per-shader raster draw-module 组合（WP18）。
//!
//! 每个模块为对应 shader 族组装一个 raster draw：`DrawBuilder` 绑定纹理
//! （`TexturePreset` / `TextureFromDisk`）+ `read_storage_buffer<T>` +
//! `uniform`/`optional_uniform` + draw 模式（`draw` / `draw_instanced_w_gl_draw_flag` /
//! `multi_draw_indexed_indirect` / `multi_draw_indirect` / `draw_no_index_buffer` /
//! `nani_draw`）。mirror `instanced_roof.rs`（wp9）与 `water_planes_draw.rs`
//! （wp7）为规范样例。

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
