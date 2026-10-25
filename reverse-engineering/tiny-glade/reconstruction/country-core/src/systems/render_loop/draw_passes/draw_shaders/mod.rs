//! `crate::systems::render_loop::draw_passes::draw_shaders` — per-shader draw 组合与 startup 系统。
//!
//! 每个文件对应 PDB 的一个同名模块；`ssbo_tags` / `startup_quad` / `common_includes_compute` / `test_support`
//! 是本目录里为复用而拆出的辅助（PDB 里没有同名模块）。`instanced_roof` 在 PDB 里存在（3 个函数），
//! 之前的骨架漏了它。

pub mod billboard_forest;
pub mod clearing_shaders;
pub mod clouds;
pub mod common_includes;
pub mod common_includes_compute;
pub mod draw_bricks;
pub mod floor_shader;
pub mod grass_shader;
pub mod instanced_critters;
pub mod instanced_ivy;
pub mod instanced_roof;
pub mod leaf_particle_draw;
pub mod path_pebble_shader;
pub mod rocky_terrain_draw;
pub mod ssbo_tags;
pub mod stairs_draw;
pub mod stars;
pub mod startup_quad;
pub mod water_planes_draw;

#[cfg(test)]
pub(crate) mod test_support;
