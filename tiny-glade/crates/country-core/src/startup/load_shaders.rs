//! 启动期 UI 和通用着色器注册。路径表与业务名称保持分离。
use crate::{
    asset_libraries::{Handle, shader_library::AssetShaderLibrary},
    render::shader::{RasterShaderProgram, boundary::ShaderBackend},
    rhapsody_types::RasterPipelineDesc,
};

pub fn load_shader_into_library(
    library: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    path: &str,
    name: &'static str,
) -> Handle<RasterShaderProgram> {
    super::load_shader_into_library(
        name,
        RasterPipelineDesc {
            path: path.to_owned(),
            entry: String::new(),
            defines: Vec::new(),
        },
        backend,
        library,
    )
    .expect("load_shader_into_library")
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
    for (path, name) in UI_SHADERS {
        load_shader_into_library(library, backend, path, name);
    }
}

pub const COMMON_SHADERS: [(&str, &str); 14] = [
    ("/clearing/meadow_flower.raster.hlsl", "meadow_flower"),
    ("/vertex_color.raster.hlsl", "vertex_color_shader"),
    ("/dashed_line.raster.hlsl", "dashed_line"),
    ("/timer_circle.raster.hlsl", "timer_circle"),
    ("/final_post.raster.hlsl", "final_post"),
    ("/egui.raster.hlsl", "egui"),
    ("/solid_vertex_color.raster.hlsl", "solid_vertex_color"),
    (
        "/solid_vertex_color_albedo_map.raster.hlsl",
        "solid_vertex_color_albedo_map",
    ),
    (
        "/solid_vertex_color_wooden_gate.raster.hlsl",
        "solid_vertex_color_wooden_gate",
    ),
    ("/flag_roof_cloth.raster.hlsl", "flag_roof_cloth"),
    ("/flag_wall_cloth.raster.hlsl", "flag_wall_cloth"),
    ("/flag_small.raster.hlsl", "flag_small"),
    (
        "/stairs_foliage_exclusion.raster.hlsl",
        "stairs_foliage_exclusion",
    ),
    // O `logo_overlay::startup` 0x1414b4b10（`LogoOverlayPlugin::build` 0x1408a5f40 尾调用 add_systems，
    // Startup）：内联 new_vertex_fragment("/logo_overlay.raster.hlsl") + new_draw("logo_overlay") + add_raster。
    // 与本表其它项等价，追加在末尾以保持 final_post 的槽位下标 4。
    ("/logo_overlay.raster.hlsl", "logo_overlay"),
];

/// 第五项 final_post 的句柄立即交给宿主槽位，后续注册不改变其身份。
pub fn load_shaders(
    library: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    final_post_shader: &mut Option<Handle<RasterShaderProgram>>,
) {
    for (index, (path, name)) in COMMON_SHADERS.into_iter().enumerate() {
        let handle = load_shader_into_library(library, backend, path, name);
        if index == 4 {
            *final_post_shader = Some(handle);
        }
    }
}
