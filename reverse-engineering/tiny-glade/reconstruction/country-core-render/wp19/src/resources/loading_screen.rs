//! `loading_screen.rs` — loading screen boot resource + startup/spawn.
//!
//! Business logic (contracts/menu-session.md §3):
//!   - `LoadingScreenPlugin::build` 0x140a1a190: `insert_resource<LoadingScreenProgress>()`;
//!     Startup `startup_loading_screen` (6-param signature);
//!     Update `update_loading_screen` with `run_if` predicate on
//!     `Option<Res<LoadingScreenProgress>>`.
//!   - `startup_loading_screen` 0x140a1a1e0: build a cloud/grid mesh from
//!     `[u32;6]` indices (reuses `draw_shaders::clouds::startup_clouds::closure_env_0`,
//!     O), `load_mesh_into_library` (name hash `0x42ac8c61`, len 14),
//!     `load_shader_into_library`, spawn entity with `(DrawableMeshBundle,
//!     DesignatedRenderPass, DontCastShadow)`.

use crate::consume::core::{AssetMeshLibrary, AssetShaderLibrary, FilteredWallTime, LogoAnimation};
use crate::typed_gap::Gap;

/// 加载屏资源（O 名称；布局 U）。
pub struct LoadingScreen(pub Gap);

/// 加载进度资源（O 名称；布局 U）。
pub struct LoadingScreenProgress(pub Gap);

/// LoadingScreen bevy Plugin。
pub struct LoadingScreenPlugin(pub Gap);

impl LoadingScreenPlugin {
    /// O：0x140a1a190（`impl$0::build`）。
    pub fn build(&mut self) {
        // insert_resource<LoadingScreenProgress>()
        // add_systems<Startup, startup_loading_screen>   (Res<GraphicsDeviceResource>,
        //   Res<AssetMeshLibrary>, Res<MeshWatch>, Res<AssetShaderLibrary>,
        //   Res<ShaderWatch>, Commands)
        // add_systems<Update, update_loading_screen>.run_if(|Option<Res<LoadingScreenProgress>>|)
        unimplemented!()
    }
}

/// O：0x140a1a1e0（Startup 系统，6 参）。
pub fn startup_loading_screen(
    _device: &Gap,
    _meshes: &AssetMeshLibrary,
    _mesh_watch: &Gap,
    _shaders: &AssetShaderLibrary,
    _shader_watch: &Gap,
) {
    // O: from_iter<u32, FlatMap<TupleWindows<RangeInclusive<u32>, (u32,u32)>,
    //        [u32;6], clouds::startup_clouds::closure_env_0>>
    //     load_mesh_into_library(mesh name hash 0x42ac8c61, len 14) +
    //     load_shader_into_library<&str>
    //     Commands::spawn_empty + insert (DrawableMeshBundle, DesignatedRenderPass,
    //       DontCastShadow), unit transform
    unimplemented!()
}

/// O：`update_loading_screen`（inline @29，宿主系统签名 `(ResMut<LoadingScreenProgress>,
/// Res<LogoAnimation>, FilteredWallTime)`）。每帧推进加载进度。
pub fn update_loading_screen(
    _progress: &mut LoadingScreenProgress,
    _logo: &LogoAnimation,
    _time: &FilteredWallTime,
) {
    unimplemented!()
}
