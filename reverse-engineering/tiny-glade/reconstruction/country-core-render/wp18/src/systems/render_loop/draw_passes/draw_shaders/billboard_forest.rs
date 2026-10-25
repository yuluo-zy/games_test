//! billboard_forest.rs — 远景 billboard 森林（startup 域，非帧 draw）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs`
//! （编译单元 499/570）。机器码（O）：
//! - `impl$0::build` 0x140ae8070（size 8，line 18）：`SubApp::add_systems<Startup,
//!   startup_billboard_forest>`（bevy plugin build）。
//! - `startup_billboard_forest` 0x140ae8080（size 578，line 27）：bevy Startup 系统；
//!   候选接口（O）：`(Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>,
//!   Res<MeshWatch>, Res<AssetShaderLibrary>, Res<ShaderWatch>, Commands)`。
//! - `closure$0`（line 38，inlinee）：TupleWindows 网格索引闭包。
//!
//! startup 逻辑（O）：`from_iter<u32, FlatMap<TupleWindows<RangeInclusive<u32>,
//!   (u32,u32)>, [u32;6], closure>>` 生成 quad 网格索引 → HashMap（RandomState）
//!   → `load_mesh_into_library`（0x140b1aca0）+ `load_shader_into_library<&str>`
//!   （0x140b3fe40，billboard shader 入库）→ `Commands::spawn_empty` → insert
//!   `(DrawableMeshBundle, DontCastShadow, KeepOnLevelUnload)`。
//!
//! shader URI（O，pass-builder shader-uri-list.json）：`/billboard_forest.raster.hlsl`
//!     + `/billboard_plant.raster.hlsl`（compiled `_billboard_forest`/`_billboard_plant`，
//!     变体 b903…/f0ad…）。draw 绑定在 `draw_lists.rs` 闭包族（WP5 命名边界）。

#![allow(dead_code, unused_imports)]

use country_core::consume::core::AssetMeshLibrary;
use country_core::consume::core::AssetShaderLibrary;
use country_core::render::GraphicsDeviceResource;
use country_core::typed_gap::Gap;

/// `BillboardForestPlugin`（impl$0）bevy plugin `build`（O：0x140ae8070）。
/// 仅注册 `startup_billboard_forest` 到 `Startup` 调度。
pub struct BillboardForestPlugin;

impl BillboardForestPlugin {
    /// `build(&mut self, app: &mut SubApp)`（O：`SubApp::add_systems`）。
    pub fn build(&mut self, _app: &mut Gap) {
        // O：add_systems<Startup, startup_billboard_forest>。
        unimplemented!()
    }
}

/// `startup_billboard_forest`（O：0x140ae8080）。
/// Startup 系统：生成 billboard 森林 quad 网格索引（TupleWindows 闭包）→
/// `load_mesh_into_library` + `load_shader_into_library<&str>` → spawn 空实体 +
/// insert `(DrawableMeshBundle, DontCastShadow, KeepOnLevelUnload)`。
/// 参数为 bevy 系统参数（O 候选接口）；本束以 typed gap 承载。
pub fn startup_billboard_forest(
    _gpu: &GraphicsDeviceResource,
    _meshes: &AssetMeshLibrary,
    _mesh_watch: &Gap,
    _shaders: &AssetShaderLibrary,
    _shader_watch: &Gap,
    _commands: &Gap,
) {
    unimplemented!()
}

/// `startup_billboard_forest::closure$0`（O：line 38）。
/// TupleWindows quad 索引闭包（`(i, j)` → `[u32; 6]` 两三角索引）。
pub fn closure_0(_window: (u32, u32)) -> [u32; 6] {
    unimplemented!()
}
