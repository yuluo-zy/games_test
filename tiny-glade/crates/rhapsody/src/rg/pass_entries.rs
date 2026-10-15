//! 3 个 `RenderGraphState::compile` 单态里内联游戏闭包的 pass 入口：**只有清单与接口占位，没有业务**。
//!
//! 证据：`evidence/render-passes/cpu-frame/evidence.json`（`render_world_passes_ordered`，来自 `render_world`
//! 0x1404ec110 的调用顺序 + resolved-calls 的地址→PDB 名映射）、`producer-closure/g3/g6/g7` 的各 pass 取证、
//! `reports/rg-pass-entries.md`。游戏侧真正的 pass 逻辑在 `country-core`（`FramePasses` trait），不在 rhapsody。
//!
//! 原回调签名：`PassBuilder::render` 收 `FnOnce(u64) -> bool`（见 `pass_builder.rs`，本 crate 另有
//! `FnOnce(PassContext) -> bool` 兼容入口）。
use super::pass_builder::PassBuilder;

/// `RenderGraphState::compile<Export, UD, F>` 的三个单态实例。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompileInstance {
    /// 0x1401647b0：`compile_loading_screen_render_graph::closure_env$0`，Export = `ResourceExportHandle<Texture>`。
    LoadingScreen,
    /// 0x140161cd0：`compile_main_render_graph::closure_env$0`，Export = `ResourceExportHandle<Texture>`；
    /// 闭包内调 `render_world`（0x1404ec110，下表 59 个调用点）。
    World,
    /// 0x140166510：`execute_presentation_render_graph::closure_env$0`，Export = `()`；
    /// 由 `GraphExecutionRecorder::execute`（0x1400fb5b0）内部调用。
    Presentation,
}
impl CompileInstance {
    pub const fn address(self) -> u64 {
        match self {
            Self::LoadingScreen => 0x1401647b0,
            Self::World => 0x140161cd0,
            Self::Presentation => 0x140166510,
        }
    }
    /// 是否返回 `ResourceExportHandle<Texture>`（展示图返回 `()`）。
    pub const fn returns_export_handle(self) -> bool {
        !matches!(self, Self::Presentation)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassKind {
    Compute,
    Draw,
    DrawAndCompute,
    Unknown,
}

/// 一个 pass 入口调用点。`addresses` 里 `/` 分隔的是互斥分支（如水面/冰面）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassEntryInfo {
    pub index: u32,
    pub name: &'static str,
    pub addresses: &'static str,
    pub kind: PassKind,
    pub source: &'static str,
    pub note: &'static str,
}
impl PassEntryInfo {
    pub fn first_address(&self) -> u64 {
        let first = self.addresses.split('/').next().unwrap().trim();
        u64::from_str_radix(first.trim_start_matches("0x"), 16).unwrap()
    }
}

/// 世界图闭包（0x140161cd0 → `render_world` 0x1404ec110）按调用顺序登记的 pass 入口。
pub const WORLD_GRAPH_PASS_ENTRIES: &[PassEntryInfo] = &[
    PassEntryInfo { index: 1, name: "upload_world_raster#0", addresses: "0x1403d8bd0", kind: PassKind::Compute, source: "country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture", note: "upload CPU raster staging to texture" },
    PassEntryInfo { index: 2, name: "upload_world_raster#1", addresses: "0x1403da050", kind: PassKind::Compute, source: "upload_world_raster::upload_raster_staging_to_texture", note: "" },
    PassEntryInfo { index: 3, name: "upload_world_raster#2", addresses: "0x1403da570", kind: PassKind::Compute, source: "upload_world_raster::upload_raster_staging_to_texture", note: "" },
    PassEntryInfo { index: 4, name: "upload_world_raster#3", addresses: "0x1403d90f0", kind: PassKind::Compute, source: "upload_world_raster::upload_raster_staging_to_texture", note: "" },
    PassEntryInfo { index: 5, name: "upload_world_raster#4", addresses: "0x1403d9610", kind: PassKind::Compute, source: "upload_world_raster::upload_raster_staging_to_texture", note: "" },
    PassEntryInfo { index: 6, name: "upload_world_raster#5", addresses: "0x1403d9b30", kind: PassKind::Compute, source: "upload_world_raster::upload_raster_staging_to_texture", note: "" },
    PassEntryInfo { index: 7, name: "water_rasterize_from_cpu", addresses: "0x140a12680", kind: PassKind::Compute, source: "compute_water::water_rasterize_from_cpu", note: "" },
    PassEntryInfo { index: 8, name: "rasterize_terrain_stroke", addresses: "0x1408c7120", kind: PassKind::Compute, source: "rasterize_terrain_stroke::rasterize_terrain_stroke", note: "" },
    PassEntryInfo { index: 9, name: "displace_rocky_terrain", addresses: "0x140a3ee80", kind: PassKind::Compute, source: "displace_rocky_terrain::displace_rocky_terrain", note: "" },
    PassEntryInfo { index: 10, name: "read_terrain_data", addresses: "0x14098c680", kind: PassKind::Compute, source: "read_terrain_data::read_terrain_data", note: "" },
    PassEntryInfo { index: 11, name: "mouse_read_terrain_stroke_index", addresses: "0x1409768b0", kind: PassKind::Compute, source: "mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index", note: "" },
    PassEntryInfo { index: 12, name: "rasterize_foliage_exclusion_mask", addresses: "0x1409cb7b0", kind: PassKind::Draw, source: "draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask", note: "" },
    PassEntryInfo { index: 13, name: "highlight_active_terrain_stroke", addresses: "0x140a58450", kind: PassKind::Compute, source: "highlight_active_terrain_stroke::highlight_active_terrain_stroke", note: "" },
    PassEntryInfo { index: 14, name: "object_grass_push_pass", addresses: "0x1409daaa0", kind: PassKind::Draw, source: "draw_passes::object_grass_push::object_grass_push_pass", note: "" },
    PassEntryInfo { index: 15, name: "compute_path_masks", addresses: "0x1409014e0", kind: PassKind::Compute, source: "compute_path_masks::compute_path_masks", note: "" },
    PassEntryInfo { index: 16, name: "rasterize_buildings", addresses: "0x140a32770", kind: PassKind::Compute, source: "rasterize_buildings::rasterize_buildings", note: "" },
    PassEntryInfo { index: 17, name: "garden_rasterize_from_cpu", addresses: "0x140a22140", kind: PassKind::Compute, source: "compute_garden::garden_rasterize_from_cpu", note: "" },
    PassEntryInfo { index: 18, name: "water_spawn_vegetation", addresses: "0x140a13970", kind: PassKind::Compute, source: "compute_water::water_spawn_vegetation", note: "" },
    PassEntryInfo { index: 19, name: "spawn_vegetation_around_buildings", addresses: "0x1408e4cc0", kind: PassKind::Compute, source: "spawn_vegetation_around_buildings::spawn_vegetation_around_buildings", note: "" },
    PassEntryInfo { index: 20, name: "read_bush_positions_for_cpu", addresses: "0x140866540", kind: PassKind::Compute, source: "read_bush_positions_for_cpu::read_bush_positions_for_cpu", note: "" },
    PassEntryInfo { index: 21, name: "garden_spawn_vegetation", addresses: "0x140a22b80", kind: PassKind::Compute, source: "compute_garden::garden_spawn_vegetation", note: "" },
    PassEntryInfo { index: 22, name: "path_pebbles_compute", addresses: "0x140958f70", kind: PassKind::Compute, source: "path_pebbles::path_pebbles_compute", note: "" },
    PassEntryInfo { index: 23, name: "leaf_particles_compute", addresses: "0x1408f4d00", kind: PassKind::Compute, source: "leaf_particles::leaf_particles_compute", note: "" },
    PassEntryInfo { index: 24, name: "roof_tiles_gravity_compute", addresses: "0x140a29b10", kind: PassKind::Compute, source: "roof_tiles_gravity::roof_tiles_gravity_compute", note: "" },
    PassEntryInfo { index: 25, name: "stairs", addresses: "0x140a68a50", kind: PassKind::Compute, source: "stairs::stairs", note: "" },
    PassEntryInfo { index: 26, name: "generate_wall_draw_lists", addresses: "0x14098aaf0", kind: PassKind::Compute, source: "gpu_generate_draw_lists::generate_wall_draw_lists", note: "round1 wall draw lists" },
    PassEntryInfo { index: 27, name: "gpu_generate_draw_lists#1", addresses: "0x140987dd0", kind: PassKind::Compute, source: "gpu_generate_draw_lists::gpu_generate_draw_lists", note: "third arg 0 = first round; creates gpu-only buffers, clears draw lists, cull_and_bucket" },
    PassEntryInfo { index: 28, name: "shadow_pass", addresses: "0x1409cc270", kind: PassKind::Draw, source: "draw_passes::shadow_pass::shadow_pass", note: "draws shadow map (callees: render/bind_common/get_rg_texture/extent_2d)" },
    PassEntryInfo { index: 29, name: "calc_terrain_lighting_texture", addresses: "0x140a51440", kind: PassKind::Compute, source: "terrain_lighting_texture::calc_terrain_lighting_texture", note: "" },
    PassEntryInfo { index: 30, name: "main_pass", addresses: "0x140833bc0", kind: PassKind::Draw, source: "draw_passes::main_pass::main_pass", note: "main depth/albedo/velocity draw; two create_raw_resource (depth+hzb) precede it" },
    PassEntryInfo { index: 31, name: "generate_hzb", addresses: "0x1409286f0", kind: PassKind::Compute, source: "generate_hzb::generate_hzb", note: "bind_individual_mips + write_rg_storage_buffer + get_or_create_temporal; all_mip_levels" },
    PassEntryInfo { index: 32, name: "generate_wall_draw_lists_pass2", addresses: "0x14098b2a0", kind: PassKind::Compute, source: "gpu_generate_draw_lists::generate_wall_draw_lists_pass2", note: "CONDITIONAL: global *0x14362c0d8 == 0" },
    PassEntryInfo { index: 33, name: "gpu_generate_draw_lists#2", addresses: "0x140987dd0", kind: PassKind::Compute, source: "gpu_generate_draw_lists::gpu_generate_draw_lists", note: "CONDITIONAL: same global; third arg != 0 = second round (post-HZB)" },
    PassEntryInfo { index: 34, name: "grass_compute", addresses: "0x140a60280", kind: PassKind::Compute, source: "grass::grass_compute", note: "dispatch_indirect_raw; writes DrawIndexedIndirectBuffer/DrawIndirectBuffer/AtomicCounterBuffer; inclusive_prefix_scan_u32_1m" },
    PassEntryInfo { index: 35, name: "main_pass_oc2", addresses: "0x140835a40", kind: PassKind::Draw, source: "draw_passes::main_pass::main_pass_oc2", note: "second main pass after HZB" },
    PassEntryInfo { index: 36, name: "run_visibility_queries", addresses: "0x140a2f2e0", kind: PassKind::Compute, source: "visibility_queries::run_visibility_queries", note: "" },
    PassEntryInfo { index: 37, name: "calculate_dof_autofocus", addresses: "0x140a17880", kind: PassKind::Compute, source: "dof_autofocus::calculate_dof_autofocus", note: "" },
    PassEntryInfo { index: 38, name: "mouse_depth_compute", addresses: "0x140a01620", kind: PassKind::Compute, source: "mouse_depth_value::mouse_depth_compute", note: "" },
    PassEntryInfo { index: 39, name: "late_forward_and_deferred_pass", addresses: "0x1408373b0", kind: PassKind::Draw, source: "draw_passes::main_pass::late_forward_and_deferred_pass", note: "lighting pass (bind_common+render)" },
    PassEntryInfo { index: 40, name: "extract_half_res_depth_normals", addresses: "0x1408f6180", kind: PassKind::Compute, source: "half_res::extract_half_res_depth_normals", note: "" },
    PassEntryInfo { index: 41, name: "calculate_reprojection_map", addresses: "0x140a0d190", kind: PassKind::Compute, source: "reprojection::calculate_reprojection_map", note: "" },
    PassEntryInfo { index: 42, name: "calculate_ssgi", addresses: "0x1408743b0", kind: PassKind::Compute, source: "ssgi::calculate_ssgi", note: "" },
    PassEntryInfo { index: 43, name: "ray_trace_gi", addresses: "0x140a01cf0", kind: PassKind::Compute, source: "ray_trace_gi::ray_trace_gi", note: "get_or_create_temporal_pair; bind_ray_tracing(ComputePass); div_up_extent" },
    PassEntryInfo { index: 44, name: "eval_gi_screen_radiance", addresses: "0x140a44cd0", kind: PassKind::Compute, source: "eval_gi_screen_radiance::eval_gi_screen_radiance", note: "" },
    PassEntryInfo { index: 45, name: "bin_lights", addresses: "0x1409e7770", kind: PassKind::Compute, source: "bin_lights::bin_lights", note: "" },
    PassEntryInfo { index: 46, name: "restir_sample_direct_lights", addresses: "0x140a6fec0", kind: PassKind::Compute, source: "restir_di::restir_sample_direct_lights", note: "" },
    PassEntryInfo { index: 47, name: "apply_screen_shadows", addresses: "0x140a39e70", kind: PassKind::Compute, source: "apply_screen_shadows::apply_screen_shadows", note: "" },
    PassEntryInfo { index: 48, name: "deferred_raster", addresses: "0x1409d20b0", kind: PassKind::Draw, source: "draw_passes::deferred_raster::deferred_raster", note: "draw_no_vao; bind_color_library; reads LanternSsbo+SpatialResolveOffsetsSsbo; bind_ray_tracing(DrawBuilder)" },
    PassEntryInfo { index: 49, name: "refraction_copy_pass", addresses: "0x1409bf320", kind: PassKind::Draw, source: "draw_passes::transparency_pass::refraction_copy_pass", note: "CONDITIONAL: global *0x14362c098 == 0" },
    PassEntryInfo { index: 50, name: "water_surface_pass OR ice_surface_pass", addresses: "0x1409bf800 / 0x1409c08d0", kind: PassKind::DrawAndCompute, source: "water_surface_pass / ice_surface_pass", note: "CONDITIONAL dispatch by is_winter flag (0x140ae8320): non-winter water_surface_pass, winter ice_surface_pass; both use dispatch_indirect_raw + bind_ray_tracing" },
    PassEntryInfo { index: 51, name: "pre_taa_transparency_pass", addresses: "0x1409c3a80", kind: PassKind::Draw, source: "transparency_pass::pre_taa_transparency_pass", note: "" },
    PassEntryInfo { index: 52, name: "compute_shadow_mask_mean", addresses: "0x140a3a6f0", kind: PassKind::Compute, source: "apply_screen_shadows::compute_shadow_mask_mean", note: "" },
    PassEntryInfo { index: 53, name: "apply_volumetrics", addresses: "0x1408a44c0", kind: PassKind::Compute, source: "apply_volumetrics::apply_volumetrics", note: "" },
    PassEntryInfo { index: 54, name: "dof_prepare", addresses: "0x140a5a020", kind: PassKind::Compute, source: "dof::dof_prepare", note: "reads DofAutofocusSsbo" },
    PassEntryInfo { index: 55, name: "generate_mips", addresses: "0x1400fb890", kind: PassKind::Compute, source: "generate_mips::generate_mips", note: "CONDITIONAL: config byte param6[0xa9 + (res_type==0x13)] != 0" },
    PassEntryInfo { index: 56, name: "calc_dof", addresses: "0x140a5a570", kind: PassKind::Compute, source: "dof::calc_dof", note: "CONDITIONAL: same byte; uses half_res" },
    PassEntryInfo { index: 57, name: "calculate_taa OR calculate_tgsr", addresses: "0x140a3b2f0 / 0x140a3ba60", kind: PassKind::Compute, source: "taa::calculate_taa / taa::calculate_tgsr", note: "TAA vs TGSR choice by byte param6[0xab]: 0->TAA else TGSR (tgsr uses min_subgroup_size + get_or_create_temporal_pair)" },
    PassEntryInfo { index: 58, name: "post_taa_transparency_pass", addresses: "0x1409c40b0", kind: PassKind::Draw, source: "transparency_pass::post_taa_transparency_pass", note: "" },
    PassEntryInfo { index: 59, name: "render_shader_refactoring", addresses: "0x14098f9f0", kind: PassKind::Unknown, source: "shader_refactoring::render_shader_refactoring", note: "callees only RwLock read (read_contended/wake_writer_or_readers) + unwrap; role unknown" },
];

/// pass 入口的接口占位：给定图构建器，登记自己的 pass 并返回是否成功。具体业务由游戏侧实现。
pub trait PassEntry {
    fn info(&self) -> &'static PassEntryInfo;
    fn declare(&mut self, pass: PassBuilder<'_>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_table_has_59_call_sites_in_order() {
        assert_eq!(WORLD_GRAPH_PASS_ENTRIES.len(), 59);
        for (i, e) in WORLD_GRAPH_PASS_ENTRIES.iter().enumerate() {
            assert_eq!(e.index as usize, i + 1);
            assert!(e.first_address() >= 0x140000000);
        }
        // 两个互斥分支：水面/冰面、TAA/TGSR
        assert_eq!(
            WORLD_GRAPH_PASS_ENTRIES.iter().filter(|e| e.addresses.contains('/')).count(),
            2
        );
    }
    #[test]
    fn compile_instances_have_the_original_addresses() {
        assert_eq!(CompileInstance::LoadingScreen.address(), 0x1401647b0);
        assert_eq!(CompileInstance::World.address(), 0x140161cd0);
        assert_eq!(CompileInstance::Presentation.address(), 0x140166510);
        assert!(!CompileInstance::Presentation.returns_export_handle());
    }
}
