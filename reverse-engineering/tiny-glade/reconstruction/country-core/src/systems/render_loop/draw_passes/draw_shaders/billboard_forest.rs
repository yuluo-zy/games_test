//! billboard_forest.rs — 远景 billboard 森林（startup 域，非帧 draw）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs`（编译单元 499/570）。机器码（O）：
//! - `BillboardForest::build` 0x140ae8070（8 B）：`SubApp::add_systems<Startup, startup_billboard_forest>`；
//! - `startup_billboard_forest` 0x140ae8080（662 B）：骨架见 `startup_quad`。与 stars / clouds 的差别：
//!   * 网格自带的名字是 "billboard forest"（带空格，长 0x10）；传给库的名字是 "billboard_forest"（下划线，长 0x10）；
//!   * 32 B 常量 `(87.093262f32, 1u32, …)`（0x142ae2fd0）；
//!   * shader 路径 "/billboard_forest.raster.hlsl"（长 0x1d）、名 "billboard_forest_shader"（长 0x17）；
//!   * 捆绑里**没有** `DesignatedRenderPass`（`bool` 紧跟在两个句柄之后，位于 `+0x48`）。
//! - `Plugin` trait 胶水：`type_id` 0x140295390、`name` 0x140295750 等。
//!
//! shader 变体见 pass-builder shader-uri-list：`/billboard_forest.raster.hlsl` + `/billboard_plant.raster.hlsl`
//! （后者由 `draw_lists` 闭包族使用，不在本 startup 里）。

use crate::systems::render_loop::draw_passes::draw_shaders::startup_quad::{
    AddStartupSystem, QuadSpec, StartupHost, run_startup,
};

pub const BILLBOARD_MESH_NAME: &str = "billboard forest";
pub const BILLBOARD_LIBRARY_NAME: &str = "billboard_forest";
/// 0x142ae2fd0 = 0x42ae2fc0 → 87.093262。
pub const BILLBOARD_FAR_PARAM: f32 = 87.093262;
pub const BILLBOARD_SHADER_PATH: &str = "/billboard_forest.raster.hlsl";
pub const BILLBOARD_SHADER_NAME: &str = "billboard_forest_shader";
pub const STARTUP_SYSTEM_NAME: &str = "startup_billboard_forest";

pub const SPEC: QuadSpec = QuadSpec {
    mesh_name: BILLBOARD_MESH_NAME,
    library_name: BILLBOARD_LIBRARY_NAME,
    far_param: BILLBOARD_FAR_PARAM,
    shader_path: BILLBOARD_SHADER_PATH,
    shader_name: BILLBOARD_SHADER_NAME,
    designated_pass: None,
};

/// `BillboardForest`（impl$0）。
#[derive(Clone, Copy, Debug, Default)]
pub struct BillboardForest;

impl BillboardForest {
    /// `build`（O 0x140ae8070）。
    pub fn build(&self, app: &mut impl AddStartupSystem) {
        app.add_startup_system(STARTUP_SYSTEM_NAME);
    }
}

/// `startup_billboard_forest`（O 0x140ae8080）。
pub fn startup_billboard_forest<H: StartupHost>(host: &mut H) {
    run_startup(&SPEC, host);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::draw_passes::draw_shaders::startup_quad::test_host::{
        Rec, Systems,
    };

    #[test]
    fn build_registers_exactly_one_startup_system() {
        let mut app = Systems::default();
        BillboardForest.build(&mut app);
        assert_eq!(app.0, vec!["startup_billboard_forest"]);
    }

    #[test]
    fn mesh_name_has_a_space_but_the_library_key_has_an_underscore() {
        let mut h = Rec::default();
        startup_billboard_forest(&mut h);
        let (lib_name, mesh) = &h.meshes[0];
        assert_eq!(lib_name, "billboard_forest");
        assert_eq!(mesh.name, "billboard forest");
        assert_eq!(BILLBOARD_MESH_NAME.len(), 0x10);
        assert_eq!(BILLBOARD_LIBRARY_NAME.len(), 0x10);
        assert_eq!(mesh.far_param.to_bits(), 0x42ae2fc0);
        assert_eq!(mesh.indices, vec![0, 1, 2, 2, 1, 3]);
    }

    #[test]
    fn no_designated_render_pass_and_longer_shader_names() {
        let mut h = Rec::default();
        startup_billboard_forest(&mut h);
        assert_eq!(h.spawned[0].designated_pass, None);
        assert_eq!(h.shaders[0].0, "/billboard_forest.raster.hlsl");
        assert_eq!(h.shaders[0].1, "billboard_forest_shader");
        assert_eq!(BILLBOARD_SHADER_PATH.len(), 0x1d);
        assert_eq!(BILLBOARD_SHADER_NAME.len(), 0x17);
    }
}
