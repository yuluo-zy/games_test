//! 默认调色板上传和配置监听。返回的 CPU 调色板由宿主插入同一个 World。
use crate::{
    asset_libraries::{
        color_library::ColorLibrarySsbo, configwatch::ConfigWatch, ssbo_library::AssetSsboLibrary,
    },
    render::ssbo::rhapsody_boundary::GraphicsDevice,
};

/// 使用渲染图所属设备上传默认调色板，再监听全局颜色配置。
pub fn init_color_library(
    device: &dyn GraphicsDevice,
    ssbos: &AssetSsboLibrary,
    config_watch: &mut ConfigWatch,
) -> ColorLibrarySsbo {
    let palette = ColorLibrarySsbo::default();
    ssbos.add_new_with_data(device, &[palette]);
    config_watch.watch("assets/colors.json".to_owned());
    palette
}
