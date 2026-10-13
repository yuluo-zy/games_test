//! clouds.rs — 云层 startup（startup 域，非帧 draw）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/clouds.rs`
//! （编译单元 479/542/1585）。机器码（O）：
//! - `impl$0::build` 0x14085b8a0（size 8，line 19）：`SubApp::add_systems<
//!   Startup, startup_clouds>`。
//! - `startup_clouds` 0x14085b8b0（size 585，line 24）：Startup 系统；候选接口
//!   `(Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>, Res<MeshWatch>,
//!   Res<AssetShaderLibrary>, Res<ShaderWatch>, Commands)`。
//! - `closure$0`（line 35，inlinee）：TupleWindows 云面片索引闭包。
//!
//! startup 逻辑（O）：quad 网格索引（TupleWindows）→ `load_mesh_into_library` +
//! `load_shader_into_library<&str>` → `spawn_empty` + insert
//! `(DrawableMeshBundle, DontCastShadow, KeepOnLevelUnload, DesignatedRenderPass)`
//! （含 `DesignatedRenderPass`，与 billboard_forest 不同——云走指定 render pass）。
//!
//! shader URI（O）：`/cloud.raster.hlsl`（compiled `_cloud`，b903…/f0ad…）。
//! 云动画时间 uniform 与 draw 由 `draw_lists.rs` 闭包族承载（WP5）。

#![allow(dead_code, unused_imports)]

use crate::consume::core::AssetMeshLibrary;
use crate::consume::core::AssetShaderLibrary;
use crate::render::GraphicsDeviceResource;
use crate::typed_gap::Gap;

/// `CloudsPlugin`（impl$0）bevy plugin `build`（O：0x14085b8a0）。
pub struct CloudsPlugin;

impl CloudsPlugin {
    /// `build(&mut self, app: &mut SubApp)`（O：`add_systems<Startup, startup_clouds>`）。
    pub fn build(&mut self, _app: &mut Gap) {
        unimplemented!()
    }
}

/// `startup_clouds`（O：0x14085b8b0）。
/// 云层 quad 网格 + `/cloud.raster.hlsl` shader 入库 + spawn 云实体
/// （`DesignatedRenderPass` 标记）。
pub fn startup_clouds(
    _gpu: &GraphicsDeviceResource,
    _meshes: &AssetMeshLibrary,
    _mesh_watch: &Gap,
    _shaders: &AssetShaderLibrary,
    _shader_watch: &Gap,
    _commands: &Gap,
) {
    unimplemented!()
}

/// `startup_clouds::closure$0`（O：line 35）。TupleWindows 云面片索引闭包。
pub fn closure_0(_window: (u32, u32)) -> [u32; 6] {
    unimplemented!()
}
