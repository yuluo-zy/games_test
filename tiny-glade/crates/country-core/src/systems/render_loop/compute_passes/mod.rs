//! `country_core::systems::render_loop::compute_passes` — frame compute passes.
//!
//! Each module below mirrors the skeleton file tree
//! `compute_passes/<name>.rs` and implements the CPU-side composition:
//! `ComputePass::new<T: AsComputeShaderSource>` + read/write declarations +
//! push_constants + dispatch (see `contracts/compute-passes.md`).

pub mod apply_screen_shadows;
pub mod apply_volumetrics;
pub mod bin_lights;
pub mod blur_pyramid;
pub mod compute_garden;
pub mod compute_path_masks;
pub mod compute_water;
pub mod displace_rocky_terrain;
pub mod dof;
pub mod dof_autofocus;
pub mod eval_gi_screen_radiance;
pub mod generate_hzb;
pub mod generate_mips;
pub mod gpu_generate_draw_lists;
pub mod grass;
pub mod half_res;
pub mod highlight_active_terrain_stroke;
pub mod leaf_particles;
pub mod mouse_depth_value;
pub mod mouse_read_terrain_stroke_index;
pub mod pass_kit;
pub mod path_pebbles;
pub mod prefix_scan;
pub mod primary_rt_debug;
pub mod rasterize_buildings;
pub mod rasterize_terrain_stroke;
pub mod ray_trace_gi;
pub mod read_bush_positions_for_cpu;
pub mod read_terrain_data;
pub mod reprojection;
pub mod reset_indirect_args;
pub mod restir_di;
pub mod roof_tiles_gravity;
pub mod shader_refactoring;
pub mod spawn_boats;
pub mod spawn_vegetation_around_buildings;
pub mod ssgi;
pub mod stairs;
pub mod taa;
pub mod terrain_lighting_texture;
pub mod upload_world_raster;
pub mod visibility_queries;
