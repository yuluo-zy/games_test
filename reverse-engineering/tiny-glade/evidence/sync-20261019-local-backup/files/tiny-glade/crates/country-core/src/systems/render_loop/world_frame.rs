//! Explicit minimal world-frame closure + seam registry.
//!
//! # Status (2026-10-19, G2 review revision)
//!
//! The recovered world frame's **production** consumer is the typed W1..W54
//! pipeline in `crates/tiny-glade/src/world_graph.rs` (`build_world`), fed by
//! `compile_world_graph`'s real targets and main prelude. There is deliberately
//! **no** second, seam-level `FramePasses` implementation here: an earlier
//! `ProductionFramePasses` adapter in this module had 63 methods of which 56
//! only returned `unsupported`, and nothing outside its own tests instantiated
//! it. That adapter was removed rather than presented as a completed
//! production body.
//!
//! What remains is real and used:
//!
//! * [`build_minimal_world_frame`] — the explicit minimal graph (main colour
//!   target + recovered `Clear image` transfer pass + export). It is **not** a
//!   fallback that hides missing world producers: `compile_world_graph` only
//!   selects it when `TG_ALLOW_MINIMAL_WORLD_FRAME=1` is set or in demo mode;
//!   otherwise a missing producer is a hard, named error.
//! * [`WORLD_FRAME_SEAMS`] — the 63 `render_world` seam names in original call
//!   order, for reports and for a future seam-level host. This table makes no
//!   claim that the seams are implemented.
//!
//! The real clear pass is `rg_helpers::clear_rg_color_image` (0x1409dde90,
//! pass `"Clear image"`, access 0x34). Final presentation (resolve/blit) is the
//! already-real `production_resolve` → `final_post::resolve_to_swapchain`
//! graph, not part of this module.

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

/// Failure category carried back to the compile boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldFrameErrorKind {
    /// The operation has no recovered production body yet.
    Unsupported,
    /// A recovered body reached a backend/graph failure.
    Backend,
}

/// A named world-frame failure. `seam` identifies the operation so the compile
/// boundary can report exactly which portion is still unconnected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldFrameError {
    pub seam: &'static str,
    pub kind: WorldFrameErrorKind,
    pub message: String,
}

impl WorldFrameError {
    pub fn unsupported(seam: &'static str) -> Self {
        Self {
            seam,
            kind: WorldFrameErrorKind::Unsupported,
            message: format!("world frame operation `{seam}` has no recovered production body"),
        }
    }
    pub fn backend(seam: &'static str, error: impl core::fmt::Display) -> Self {
        Self {
            seam,
            kind: WorldFrameErrorKind::Backend,
            message: error.to_string(),
        }
    }
}

impl core::fmt::Display for WorldFrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({:?}, seam `{}`)", self.message, self.kind, self.seam)
    }
}

impl std::error::Error for WorldFrameError {}

/// Minimal world-frame host: the three real graph operations needed by the
/// explicit minimal closure.
pub trait MinimalWorldFrameHost {
    /// Create the main colour target (`create_raw_resource`, 0x140be94d0).
    fn create_color_target(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<render_world::RgTexture, WorldFrameError>;
    /// Record the recovered `Clear image` pass (`clear_rg_color_image`, access 0x34).
    fn clear_color_target(
        &mut self,
        color: &render_world::RgTexture,
    ) -> Result<(), WorldFrameError>;
    /// Publish the final colour for the presentation graph.
    fn export_texture(
        &mut self,
        color: &render_world::RgTexture,
        access: u8,
    ) -> Result<(), WorldFrameError>;
}

/// Explicit minimal world frame: main colour target + recovered clear pass +
/// export. The presentation graph (`production_resolve`) performs the final
/// blit.
///
/// This is a real frame, but it is **not** a world renderer. Callers must
/// select it explicitly (diagnostic switch / demo), never as a silent fallback.
pub fn build_minimal_world_frame<H: MinimalWorldFrameHost + ?Sized>(
    host: &mut H,
    extent: [u32; 2],
) -> Result<render_world::RgTexture, WorldFrameError> {
    assert!(
        extent[0] > 0 && extent[1] > 0,
        "assertion failed: world_render_dims[0] > 0 && world_render_dims[1] > 0"
    );
    // O: `main color` = new_2d(extent, 0x7a).mip_levels(6).
    let desc = main_color_desc(extent);
    let color = host.create_color_target(MAIN_COLOR_NAME, &desc)?;
    host.clear_color_target(&color)?;
    host.export_texture(&color, MAIN_EXPORT_ACCESS)?;
    Ok(color)
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

    struct Minimal {
        created: Option<render_world::RgTexture>,
        seen_desc: Option<TextureDesc>,
        cleared: bool,
        exported: Option<u8>,
    }

    impl MinimalWorldFrameHost for Minimal {
        fn create_color_target(
            &mut self,
            name: &str,
            desc: &TextureDesc,
        ) -> Result<render_world::RgTexture, WorldFrameError> {
            assert_eq!(name, MAIN_COLOR_NAME);
            self.seen_desc = Some(*desc);
            let color = render_world::RgTexture {
                id: 42,
                desc: render_world::TextureDesc::new_2d(
                    [desc.extent[0], desc.extent[1]],
                    desc.format,
                ),
            };
            self.created = Some(color);
            Ok(color)
        }
        fn clear_color_target(
            &mut self,
            color: &render_world::RgTexture,
        ) -> Result<(), WorldFrameError> {
            assert_eq!(color.id, 42);
            self.cleared = true;
            Ok(())
        }
        fn export_texture(
            &mut self,
            _color: &render_world::RgTexture,
            access: u8,
        ) -> Result<(), WorldFrameError> {
            self.exported = Some(access);
            Ok(())
        }
    }

    #[test]
    fn minimal_frame_creates_clears_and_exports_main_color() {
        let mut host = Minimal {
            created: None,
            seen_desc: None,
            cleared: false,
            exported: None,
        };
        let color = build_minimal_world_frame(&mut host, [800, 600]).unwrap();
        assert_eq!(color.id, 42);
        assert_eq!(color.desc.format, render_world::formats::COLOR_HDR);
        assert_eq!(color.desc.extent, [800, 600]);
        // The graph creation description is what carries the native mip count.
        let seen = host.seen_desc.expect("host saw the creation description");
        assert_eq!(seen.mip_levels, MAIN_COLOR_MIP_LEVELS);
        assert_eq!(seen.format, render_world::formats::COLOR_HDR);
        assert!(host.cleared);
        assert_eq!(host.exported, Some(MAIN_EXPORT_ACCESS));
    }

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
    fn backend_errors_keep_their_name() {
        struct Failing;
        impl MinimalWorldFrameHost for Failing {
            fn create_color_target(
                &mut self,
                _name: &str,
                _desc: &TextureDesc,
            ) -> Result<render_world::RgTexture, WorldFrameError> {
                Err(WorldFrameError::backend("create_color_target", "no device"))
            }
            fn clear_color_target(
                &mut self,
                _color: &render_world::RgTexture,
            ) -> Result<(), WorldFrameError> {
                unreachable!()
            }
            fn export_texture(
                &mut self,
                _color: &render_world::RgTexture,
                _access: u8,
            ) -> Result<(), WorldFrameError> {
                unreachable!()
            }
        }
        let error = build_minimal_world_frame(&mut Failing, [8, 8]).unwrap_err();
        assert_eq!(error.seam, "create_color_target");
        assert_eq!(error.kind, WorldFrameErrorKind::Backend);
        assert!(error.to_string().contains("no device"));
    }

    #[test]
    fn seam_table_matches_render_world_order() {
        assert_eq!(WORLD_FRAME_SEAMS.len(), 63);
        assert_eq!(WORLD_FRAME_SEAMS[0], "is_winter");
        assert_eq!(WORLD_FRAME_SEAMS[62], "render_shader_refactoring");
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: world_render_dims[0] > 0 && world_render_dims[1] > 0"
    )]
    fn minimal_frame_requires_nonzero_extent() {
        let mut host = Minimal {
            created: None,
            seen_desc: None,
            cleared: false,
            exported: None,
        };
        let _ = build_minimal_world_frame(&mut host, [0, 600]);
    }
}
