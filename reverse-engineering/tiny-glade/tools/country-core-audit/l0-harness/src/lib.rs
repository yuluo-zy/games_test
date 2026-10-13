//! 把 `reconstruction/country-core/src/render/` 里的第 0 层文件用 `#[path]` 原样挂进来编译，
//! 再提供它们依赖的最小外部占位（`utils_contract`）。运行：
//!   CARGO_TARGET_DIR=/tmp/l0h-target cargo test --offline
extern crate alloc;

#[path = "@SRC@/rhapsody_types.rs"]
pub mod rhapsody_types;

pub mod utils_contract {
    use glam::Vec3;
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Aabb3 {
        pub min: Vec3,
        pub max: Vec3,
    }
    impl Aabb3 {
        pub fn empty() -> Self {
            Aabb3 { min: Vec3::splat(f32::MAX), max: Vec3::splat(f32::MIN) }
        }
        pub fn expand_to_contain_point(&mut self, p: Vec3) {
            self.min = self.min.min(p);
            self.max = self.max.max(p);
        }
        /// O（0x141240460）：`max - min`。
        pub fn extent(&self) -> Vec3 {
            self.max - self.min
        }
        /// O（0x141240420）：`(min + max) * 0.5`。
        pub fn center(&self) -> Vec3 {
            (self.min + self.max) * 0.5
        }
    }

    /// `utils::geometry::rectangle::Rectangle2d`（O 0x140c98900，24 B）：方向 (1,0) @0、中心 @8、
    /// 宽 @0x10、长 @0x14；`new_axis_aligned` 断言 width / length 有限。
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rectangle2d {
        pub direction: glam::Vec2,
        pub center: glam::Vec2,
        pub width: f32,
        pub length: f32,
    }
    impl Rectangle2d {
        pub fn new_axis_aligned(width: f32, length: f32, center: glam::Vec2) -> Self {
            assert!(width.is_finite(), "assertion failed: width.is_finite()");
            assert!(length.is_finite(), "assertion failed: length.is_finite()");
            Rectangle2d { direction: glam::Vec2::X, center, width, length }
        }
    }
    pub fn load_json_as_mesh(_p: &std::path::Path) -> Result<crate::render::mesh::Mesh, ()> { Err(()) }
    #[derive(Clone, Copy, Debug)]
    pub struct Triangle(pub [Vec3; 3]);
    impl Triangle {
        pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
            Triangle([a, b, c])
        }
    }
}

#[path = "@SRC@/frozen_map.rs"]
pub mod frozen_map;
#[path = "@SRC@/name_hash.rs"]
pub mod name_hash;
pub mod asset_libraries {
    include!("asset_handle.rs");
    #[path = "@SRC@/asset_libraries/mesh_library.rs"]
    pub mod mesh_library;
    #[path = "@SRC@/asset_libraries/ssbo_library.rs"]
    pub mod ssbo_library;
    #[path = "@SRC@/asset_libraries/color_library.rs"]
    pub mod color_library;
    #[path = "@SRC@/asset_libraries/placement_data_library.rs"]
    pub mod placement_data_library;
    #[path = "@SRC@/asset_libraries/audio_trigger_library.rs"]
    pub mod audio_trigger_library;
    #[path = "@SRC@/asset_libraries/vao_library.rs"]
    pub mod vao_library;
    #[path = "@SRC@/asset_libraries/meshwatch.rs"]
    pub mod meshwatch;
    #[path = "@SRC@/asset_libraries/texture_library.rs"]
    pub mod texture_library;
    #[path = "@SRC@/asset_libraries/texture_watch.rs"]
    pub mod texture_watch;
    #[path = "@SRC@/asset_libraries/shader_library.rs"]
    pub mod shader_library;
    #[path = "@SRC@/asset_libraries/configwatch.rs"]
    pub mod configwatch;
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gap;
pub mod reused {
    include!("reused_watch.rs");
}
pub mod render;

pub mod rhapsody_contract {
    include!("rc_body.rs");
}
pub mod typed_gap {
    pub use crate::Gap;
}
pub mod render_loop_ctx {
    include!("render_loop_ctx.rs");
}

pub mod utils {
    #[path = "@SRC@/utils/float_grid.rs"]
    pub mod float_grid;
    pub mod components {
        #[path = "@SRC@/utils/components/transform.rs"]
        pub mod transform;
    }
}
pub mod resources {
    pub mod session { #[derive(Clone, Copy)] pub struct GladeType(pub bool); impl GladeType { pub fn is_winter(&self) -> bool { self.0 } } }
    #[path = "@SRC@/resources/texture_presets.rs"]
    pub mod texture_presets;
    pub mod placement_data {
        use crate::render::ssbo::StorageElement;
        use serde::Deserialize;
        #[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
        pub struct PlacementDataSsbo {
            pub x: f32,
            pub y: f32,
        }
        impl StorageElement for PlacementDataSsbo {
            const NAME: &'static str = "PlacementDataSsbo";
        }
    }
    #[path = "@SRC@/resources/render/mod.rs"]
    pub mod render;
    #[path = "@SRC@/resources/render_ssbo.rs"]
    pub mod render_ssbo;
    #[path = "@SRC@/resources/image_formation.rs"]
    pub mod image_formation;
    #[path = "@SRC@/resources/water_mask.rs"]
    pub mod water_mask;
    #[path = "@SRC@/resources/world_raster.rs"]
    pub mod world_raster;
    #[path = "@SRC@/resources/cursor_terrain_raycast.rs"]
    pub mod cursor_terrain_raycast;
}

pub mod systems {
    pub mod render_loop {
        pub mod compute_passes {
            #[path = "@SRC@/systems/render_loop/compute_passes/apply_screen_shadows.rs"]
            pub mod apply_screen_shadows;
            #[path = "@SRC@/systems/render_loop/compute_passes/apply_volumetrics.rs"]
            pub mod apply_volumetrics;
            #[path = "@SRC@/systems/render_loop/compute_passes/bin_lights.rs"]
            pub mod bin_lights;
            #[path = "@SRC@/systems/render_loop/compute_passes/blur_pyramid.rs"]
            pub mod blur_pyramid;
            #[path = "@SRC@/systems/render_loop/compute_passes/compute_garden.rs"]
            pub mod compute_garden;
            #[path = "@SRC@/systems/render_loop/compute_passes/compute_path_masks.rs"]
            pub mod compute_path_masks;
            #[path = "@SRC@/systems/render_loop/compute_passes/compute_water.rs"]
            pub mod compute_water;
            #[path = "@SRC@/systems/render_loop/compute_passes/displace_rocky_terrain.rs"]
            pub mod displace_rocky_terrain;
            #[path = "@SRC@/systems/render_loop/compute_passes/dof.rs"]
            pub mod dof;
            #[path = "@SRC@/systems/render_loop/compute_passes/dof_autofocus.rs"]
            pub mod dof_autofocus;
            #[path = "@SRC@/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs"]
            pub mod eval_gi_screen_radiance;
            #[path = "@SRC@/systems/render_loop/compute_passes/generate_hzb.rs"]
            pub mod generate_hzb;
            #[path = "@SRC@/systems/render_loop/compute_passes/generate_mips.rs"]
            pub mod generate_mips;
            #[path = "@SRC@/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"]
            pub mod gpu_generate_draw_lists;
            #[path = "@SRC@/systems/render_loop/compute_passes/grass.rs"]
            pub mod grass;
            #[path = "@SRC@/systems/render_loop/compute_passes/half_res.rs"]
            pub mod half_res;
            #[path = "@SRC@/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs"]
            pub mod highlight_active_terrain_stroke;
            #[path = "@SRC@/systems/render_loop/compute_passes/leaf_particles.rs"]
            pub mod leaf_particles;
            #[path = "@SRC@/systems/render_loop/compute_passes/mouse_depth_value.rs"]
            pub mod mouse_depth_value;
            #[path = "@SRC@/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs"]
            pub mod mouse_read_terrain_stroke_index;
            #[path = "@SRC@/systems/render_loop/compute_passes/pass_kit.rs"]
            pub mod pass_kit;
            #[path = "@SRC@/systems/render_loop/compute_passes/path_pebbles.rs"]
            pub mod path_pebbles;
            #[path = "@SRC@/systems/render_loop/compute_passes/prefix_scan.rs"]
            pub mod prefix_scan;
            #[path = "@SRC@/systems/render_loop/compute_passes/primary_rt_debug.rs"]
            pub mod primary_rt_debug;
            #[path = "@SRC@/systems/render_loop/compute_passes/rasterize_buildings.rs"]
            pub mod rasterize_buildings;
            #[path = "@SRC@/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs"]
            pub mod rasterize_terrain_stroke;
            #[path = "@SRC@/systems/render_loop/compute_passes/ray_trace_gi.rs"]
            pub mod ray_trace_gi;
            #[path = "@SRC@/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs"]
            pub mod read_bush_positions_for_cpu;
            #[path = "@SRC@/systems/render_loop/compute_passes/read_terrain_data.rs"]
            pub mod read_terrain_data;
            #[path = "@SRC@/systems/render_loop/compute_passes/reprojection.rs"]
            pub mod reprojection;
            #[path = "@SRC@/systems/render_loop/compute_passes/reset_indirect_args.rs"]
            pub mod reset_indirect_args;
            #[path = "@SRC@/systems/render_loop/compute_passes/restir_di.rs"]
            pub mod restir_di;
            #[path = "@SRC@/systems/render_loop/compute_passes/roof_tiles_gravity.rs"]
            pub mod roof_tiles_gravity;
            #[path = "@SRC@/systems/render_loop/compute_passes/shader_refactoring.rs"]
            pub mod shader_refactoring;
            #[path = "@SRC@/systems/render_loop/compute_passes/spawn_boats.rs"]
            pub mod spawn_boats;
            #[path = "@SRC@/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs"]
            pub mod spawn_vegetation_around_buildings;
            #[path = "@SRC@/systems/render_loop/compute_passes/ssgi.rs"]
            pub mod ssgi;
            #[path = "@SRC@/systems/render_loop/compute_passes/stairs.rs"]
            pub mod stairs;
            #[path = "@SRC@/systems/render_loop/compute_passes/taa.rs"]
            pub mod taa;
            #[path = "@SRC@/systems/render_loop/compute_passes/terrain_lighting_texture.rs"]
            pub mod terrain_lighting_texture;
            #[path = "@SRC@/systems/render_loop/compute_passes/upload_world_raster.rs"]
            pub mod upload_world_raster;
            #[path = "@SRC@/systems/render_loop/compute_passes/visibility_queries.rs"]
            pub mod visibility_queries;
        }
        #[path = "@SRC@/systems/render_loop/indirect_params.rs"]
        pub mod indirect_params;
        pub mod resources {
            #[path = "@SRC@/systems/render_loop/resources/critters.rs"]
            pub mod critters;
        }
        pub mod draw_passes {
            pub mod draw_shaders {
                #[cfg(test)]
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/test_support.rs"]
                pub mod test_support;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/ssbo_tags.rs"]
                pub mod ssbo_tags;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"]
                pub mod common_includes;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/common_includes_compute.rs"]
                pub mod common_includes_compute;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs"]
                pub mod grass_shader;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs"]
                pub mod floor_shader;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs"]
                pub mod path_pebble_shader;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs"]
                pub mod rocky_terrain_draw;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs"]
                pub mod leaf_particle_draw;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs"]
                pub mod water_planes_draw;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs"]
                pub mod stairs_draw;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs"]
                pub mod draw_bricks;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs"]
                pub mod instanced_ivy;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs"]
                pub mod instanced_critters;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs"]
                pub mod clearing_shaders;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs"]
                pub mod instanced_roof;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/startup_quad.rs"]
                pub mod startup_quad;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/stars.rs"]
                pub mod stars;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/clouds.rs"]
                pub mod clouds;
                #[path = "@SRC@/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs"]
                pub mod billboard_forest;
            }
        }
    }
}
