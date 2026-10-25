//! stars.rs — 星空 startup（startup 域，非帧 draw）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/stars.rs`
//! （编译单元 502/1598）。机器码（O）：
//! - `impl$0::build` 0x140903c90（size 8，line 19）：`SubApp::add_systems<
//!   Startup, startup_stars>`。
//! - `startup_stars` 0x140903ca0（size 585，line 24）：Startup 系统；候选接口
//!   `(Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>, Res<MeshWatch>,
//!   Res<AssetShaderLibrary>, Res<ShaderWatch>, Commands)`。
//! - `closure$0`（line 35，inlinee）：TupleWindows 星面片索引闭包。
//!
//! startup 逻辑（O）：quad 网格索引（TupleWindows）→ `load_mesh_into_library` +
//! `load_shader_into_library<&str>` → `spawn_empty` + insert
//! `(DrawableMeshBundle, DontCastShadow, KeepOnLevelUnload, DesignatedRenderPass)`。
//!
//! shader URI（O）：`/star.raster.hlsl`（compiled `_star`，b903…/f0ad…）。

#![allow(dead_code, unused_imports)]

use country_core::consume::core::AssetMeshLibrary;
use country_core::consume::core::AssetShaderLibrary;
use country_core::render::GraphicsDeviceResource;
use country_core::typed_gap::Gap;

/// `StarsPlugin`（impl$0）bevy plugin `build`（O：0x140903c90）。
pub struct StarsPlugin;

impl StarsPlugin {
    /// `build(&mut self, app: &mut SubApp)`（O：`add_systems<Startup, startup_stars>`）。
    pub fn build(&mut self, _app: &mut Gap) {
        unimplemented!()
    }
}

/// `startup_stars`（O：0x140903ca0）。星面片网格 + `/star.raster.hlsl` 入库 + spawn。
pub fn startup_stars(
    _gpu: &GraphicsDeviceResource,
    _meshes: &AssetMeshLibrary,
    _mesh_watch: &Gap,
    _shaders: &AssetShaderLibrary,
    _shader_watch: &Gap,
    _commands: &Gap,
) {
    unimplemented!()
}

/// `startup_stars::closure$0`（O：line 35）。TupleWindows 星面片索引闭包。
pub fn closure_0(_window: (u32, u32)) -> [u32; 6] {
    unimplemented!()
}
