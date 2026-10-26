//! Main target descriptors and the recovered render_world seam order.
//!
//! The sole production consumer is the typed W1..W54 pipeline in
//! tiny-glade/src/world_graph.rs. Missing producers are errors; there is no
//! alternate clear-only world renderer or seam-level host implementation.

use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::render_world;

/// Main colour target name (O: 0x14295d6a8, length 10, `TextureDesc::new_2d(extent, 0x7a)`).
pub const MAIN_COLOR_NAME: &str = "main color";
/// Main depth target name (O: 0x14295d798, length 10, `TextureDesc::new_2d(extent, 0x82)`).
pub const MAIN_DEPTH_NAME: &str = "main depth";
/// Access used when the main graph publishes its final colour for the
/// presentation graph (`production_resolve` imports it sampled at 0x22).
pub const MAIN_EXPORT_ACCESS: u8 = 0x22;
/// O: 0x140bd5460 writes `[desc+0x1c] = 6` (`mov [rdx+0x1c], r8b`); the field
/// at +0x1c is `mip_levels`, not a usage bit. `generate_mips` (W50) reads this
/// value as its SPD mip count, so the target must be created with 6 levels.
pub const MAIN_COLOR_MIP_LEVELS: u8 = 6;
/// Main depth target format id (O: `new_2d(extent, 0x82)`).
pub const MAIN_DEPTH_FORMAT: u32 = 0x82;

/// `main color` description (O: `new_2d(extent, 0x7a)` + `mip_levels(6)` +
/// attachment/sampled usage; contract §3 M2).
pub fn main_color_desc(extent: [u32; 2]) -> TextureDesc {
    TextureDesc::new_2d(extent, render_world::formats::COLOR_HDR)
        .mip_levels(MAIN_COLOR_MIP_LEVELS)
        .color_attachment()
        .sampled()
}

/// `main depth` description (O: `new_2d(extent, 0x82)`; one mip level, matching
/// the native `new_2d` `+0x18 = 1`).
pub fn main_depth_desc(extent: [u32; 2]) -> TextureDesc {
    TextureDesc::new_2d(extent, MAIN_DEPTH_FORMAT).mip_levels(1)
}

/// The 63 `render_world` seam names, in original call order. Reporting /
/// extension registry only; entries are not implemented by this module.
pub const WORLD_FRAME_SEAMS: [&str; 63] = [
    "is_winter",
    "display_dims",
    "dof_mode",
    "flag_c0d8",
    "flag_c098",
    "get_or_create_temporal",
    "get_or_create_temporal_pair",
    "create_raw_resource",
    "upload_raster_staging_to_texture",
    "water_rasterize_from_cpu",
    "rasterize_terrain_stroke",
    "displace_rocky_terrain",
    "read_terrain_data",
    "mouse_read_terrain_stroke_index",
    "rasterize_foliage_exclusion_mask",
    "highlight_active_terrain_stroke",
    "object_grass_push_pass",
    "compute_path_masks",
    "rasterize_buildings",
    "garden_rasterize_from_cpu",
    "water_spawn_vegetation",
    "spawn_vegetation_around_buildings",
    "read_bush_positions_for_cpu",
    "garden_spawn_vegetation",
    "path_pebbles_compute",
    "leaf_particles_compute",
    "roof_tiles_gravity_compute",
    "stairs",
    "generate_wall_draw_lists",
    "gpu_generate_draw_lists",
    "shadow_pass",
    "calc_terrain_lighting_texture",
    "main_pass",
    "generate_hzb",
    "generate_wall_draw_lists_pass2",
    "grass_compute",
    "main_pass_oc2",
    "run_visibility_queries",
    "calculate_dof_autofocus",
    "mouse_depth_compute",
    "late_forward_and_deferred_pass",
    "extract_half_res_depth_normals",
    "calculate_reprojection_map",
    "calculate_ssgi",
    "ray_trace_gi",
    "eval_gi_screen_radiance",
    "bin_lights",
    "restir_sample_direct_lights",
    "apply_screen_shadows",
    "deferred_raster",
    "refraction_copy_pass",
    "water_surface_pass",
    "ice_surface_pass",
    "pre_taa_transparency_pass",
    "compute_shadow_mask_mean",
    "apply_volumetrics",
    "dof_prepare",
    "generate_mips",
    "calc_dof",
    "calculate_taa",
    "calculate_tgsr",
    "post_taa_transparency_pass",
    "render_shader_refactoring",
];

#[cfg(test)]
mod tests {
    use super::*;

    /// P1: the native `+0x1c = 6` is `mip_levels` (0x140bd5460 `mov [rdx+0x1c],
    /// r8b`). W50 `generate_mips` reads `TextureDesc::mip_levels` as its SPD mip
    /// count and asserts `<= 13` + format 0x7a, so the target must carry 6.
    #[test]
    fn main_color_mip_count_matches_generate_mips_input() {
        use crate::systems::render_loop::compute_passes::generate_mips;
        let desc = main_color_desc([1280, 720]);
        assert_eq!(desc.mip_levels, 6);
        assert_eq!(desc.mip_levels, MAIN_COLOR_MIP_LEVELS);
        assert_eq!(desc.format, generate_mips::SPD_FORMAT);
        assert!(desc.mip_levels <= generate_mips::MAX_MIPS);
        assert_eq!(main_depth_desc([1280, 720]).mip_levels, 1);
    }

    #[test]
    fn seam_table_matches_render_world_order() {
        assert_eq!(WORLD_FRAME_SEAMS.len(), 63);
        assert_eq!(WORLD_FRAME_SEAMS[0], "is_winter");
        assert_eq!(WORLD_FRAME_SEAMS[62], "render_shader_refactoring");
    }

}
