//! clouds.rs — 云层（startup 域，非帧 draw）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/clouds.rs`（编译单元 479/542/1585）。机器码（O）：
//! - `CloudPlugin::build` 0x14085b8a0（8 B）：`SubApp::add_systems<Startup, startup_clouds>`；
//! - `startup_clouds` 0x14085b8b0（662 B）：骨架见 `startup_quad`。常量：网格名 / 库名 "cloud"（长 5）；shader 路径 "/cloud.raster.hlsl"（长 0x12）、名 "cloud_shader"（长 0xc）；
//!   `DesignatedRenderPass = 0x10`。
//! - 注：PDB 里 `from_iter` 单态化实例的闭包名属于 `clouds::startup_clouds::closure_env$0`（ICF 后三个 startup 共用）。

use crate::systems::render_loop::draw_passes::draw_shaders::startup_quad::{
    AddStartupSystem, QuadSpec, StartupHost, run_startup,
};

pub const CLOUD_MESH_NAME: &str = "cloud";
pub const CLOUD_SHADER_PATH: &str = "/cloud.raster.hlsl";
pub const CLOUD_SHADER_NAME: &str = "cloud_shader";
pub const CLOUD_DESIGNATED_PASS: u32 = 0x10;
pub const STARTUP_SYSTEM_NAME: &str = "startup_clouds";

pub const SPEC: QuadSpec = QuadSpec {
    mesh_name: CLOUD_MESH_NAME,
    library_name: CLOUD_MESH_NAME,
    shader_path: CLOUD_SHADER_PATH,
    shader_name: CLOUD_SHADER_NAME,
    designated_pass: Some(CLOUD_DESIGNATED_PASS),
    keep_on_level_unload: true,
};

/// `CloudPlugin`（impl$0）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CloudPlugin;

impl CloudPlugin {
    /// `build`（O 0x14085b8a0）。
    pub fn build(&self, app: &mut impl AddStartupSystem) {
        app.add_startup_system(STARTUP_SYSTEM_NAME);
    }
}

/// `startup_clouds`（O 0x14085b8b0）。
pub fn startup_clouds<H: StartupHost>(host: &mut H) {
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
        CloudPlugin.build(&mut app);
        assert_eq!(app.0, vec!["startup_clouds"]);
    }

    #[test]
    fn startup_loads_the_cloud_mesh_and_shader_and_spawns_with_designated_pass() {
        let mut h = Rec::default();
        startup_clouds(&mut h);
        let (lib_name, mesh) = &h.meshes[0];
        assert_eq!((lib_name.as_str(), mesh.name), ("cloud", "cloud"));
        assert_eq!(mesh.indices, vec![0, 1, 2, 2, 1, 3]);
        assert_eq!(
            h.shaders,
            vec![("/cloud.raster.hlsl".to_string(), "cloud_shader".to_string())]
        );
        assert_eq!(CLOUD_SHADER_PATH.len(), 0x12);
        assert_eq!(CLOUD_SHADER_NAME.len(), 0xc);
        assert_eq!(h.spawned[0].designated_pass, Some(0x10));
    }
}
