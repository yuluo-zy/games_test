//! 交换链解析与拉伸所需的三个 blit 着色器。
use super::load_shaders::load_shader_into_library;
use crate::{
    asset_libraries::shader_library::AssetShaderLibrary, render::shader::boundary::ShaderBackend,
};

pub const BLIT_SHADERS: [(&str, &str); 3] = [
    (
        "/blit/resolve_to_swapchain.raster.hlsl",
        "raster_resolve_to_swapchain",
    ),
    (
        "/blit/resolve_msaa_to_swapchain.raster.hlsl",
        "raster_resolve_msaa_to_swapchain",
    ),
    (
        "/blit/blit_stretch_decode_srgb.raster.hlsl",
        "blit_stretch_decode_srgb",
    ),
];

pub fn startup_blit(library: &AssetShaderLibrary, backend: &dyn ShaderBackend) {
    for (path, name) in BLIT_SHADERS {
        load_shader_into_library(library, backend, path, name);
    }
}
