//! B `init_color_library` at 0x14082c000: default palette → GPU buffer →
//! deferred CPU resource insertion → watch `assets/colors.json`. This producer
//! does not read the JSON or select a theme. Native evidence is retained in
//! task-index/work/TG-07278b540192d36392/main-chain-frame-20261018.

use crate::{
    asset_libraries::{
        color_library::ColorLibrarySsbo,
        configwatch::ConfigWatch,
        ssbo_library::AssetSsboLibrary,
    },
    render::ssbo::rhapsody_boundary::GraphicsDevice,
};

/// The host supplies the same graphics-device buffer table used by the App's
/// render graph. The returned palette is the original Commands payload and
/// must be inserted into that same App's World after successful registration.
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
