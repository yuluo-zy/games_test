//! B startup shader producers, recovered from original RNE instructions.
//!
//! `load_ui_shaders` 0x140aaaf80 (6 calls), `load_shaders` 0x140aab0c0
//! (13 calls), shared helper 0x140b3fe40. Each helper builds the path-only
//! descriptor (empty entry/defines), compiles both raster variants, unwraps on
//! failure, then `AssetShaderLibrary::add_raster(program, false)`.
//! Original path/name lengths and call order are retained in
//! TG-0ef1874c29a03d0dec/main-chain-frame-20261018/0x140aaaf80.asm etc.
use crate::{
    asset_libraries::{Handle, shader_library::AssetShaderLibrary},
    render::shader::{RasterShaderProgram, boundary::ShaderBackend},
    rhapsody_types::RasterPipelineDesc,
};

pub fn load_shader_into_library(
    library: &AssetShaderLibrary, backend: &dyn ShaderBackend,
    path: &str, name: &'static str,
) -> Handle<RasterShaderProgram> {
    super::load_shader_into_library(name, RasterPipelineDesc {
        path: path.to_owned(), entry: String::new(), defines: Vec::new(),
    }, backend, library).expect("load_shader_into_library")
}

pub const UI_SHADERS: [(&str, &str); 6] = [
    ("/ui_2d.raster.hlsl", "ui_2d"),
    ("/ui_2d_ws.raster.hlsl", "ui_2d_ws"),
    ("/ui_2d_color_button.raster.hlsl", "ui_2d_color_button"),
    ("/text.raster.hlsl", "text"),
    ("/mei_shadow.raster.hlsl", "mei_shadow"),
    ("/custom_cursor.raster.hlsl", "custom_cursor"),
];

pub fn load_ui_shaders(library: &AssetShaderLibrary, backend: &dyn ShaderBackend) {
    for (path, name) in UI_SHADERS { load_shader_into_library(library, backend, path, name); }
}

pub const COMMON_SHADERS: [(&str, &str); 13] = [
    ("/clearing/meadow_flower.raster.hlsl", "meadow_flower"),
    ("/vertex_color.raster.hlsl", "vertex_color_shader"),
    ("/dashed_line.raster.hlsl", "dashed_line"),
    ("/timer_circle.raster.hlsl", "timer_circle"),
    ("/final_post.raster.hlsl", "final_post"),
    ("/egui.raster.hlsl", "egui"),
    ("/solid_vertex_color.raster.hlsl", "solid_vertex_color"),
    ("/solid_vertex_color_albedo_map.raster.hlsl", "solid_vertex_color_albedo_map"),
    ("/solid_vertex_color_wooden_gate.raster.hlsl", "solid_vertex_color_wooden_gate"),
    ("/flag_roof_cloth.raster.hlsl", "flag_roof_cloth"),
    ("/flag_wall_cloth.raster.hlsl", "flag_wall_cloth"),
    ("/flag_small.raster.hlsl", "flag_small"),
    ("/stairs_foliage_exclusion.raster.hlsl", "stairs_foliage_exclusion"),
];

/// The original also writes the fifth returned handle into FinalPostResources
/// immediately before loading Egui (0x140aab1c7..1da). The caller supplies that
/// actual slot here so the side effect and its ordering are preserved.
pub fn load_shaders(
    library: &AssetShaderLibrary, backend: &dyn ShaderBackend,
    final_post_shader: &mut Option<Handle<RasterShaderProgram>>,
) {
    for (index, (path, name)) in COMMON_SHADERS.into_iter().enumerate() {
        let handle = load_shader_into_library(library, backend, path, name);
        if index == 4 { *final_post_shader = Some(handle); }
    }
}
