//! stars.rs — 星空（startup 域，非帧 draw）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/stars.rs`（编译单元 502/1598）。机器码（O）：
//! - `StarsPlugin::build` 0x140903c90（8 B）：`SubApp::add_systems<Startup, startup_stars>`（一次尾调用）；
//! - `startup_stars` 0x140903ca0（662 B）：共用骨架见 `startup_quad`。本文件常量：
//!   网格名 / 库名 "star"（长 4）；32 B 常量 `(85.283447f32, 1u32, 0, …)`（0x142aa9130）；
//!   shader 路径 "/star.raster.hlsl"（长 0x11）、名 "star_shader"（长 0xb）；`DesignatedRenderPass = 0x10`。
//! - `Plugin::{name,ready,finish,is_unique}` / `as_any*` / `type_id`：bevy `Plugin` 的 trait 胶水（0x1400c7cc0 起），
//!   无业务逻辑。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::excessive_precision)]
use crate::systems::render_loop::draw_passes::draw_shaders::startup_quad::{
    AddStartupSystem, QuadSpec, StartupHost, run_startup,
};

/// 网格名 = 库名（O：同一个指针）。
pub const STAR_MESH_NAME: &str = "star";
/// 32 B 常量的 f32（O：0x142aa9130 = 0x42aa9120 → 85.283447）。
pub const STAR_FAR_PARAM: f32 = 85.283447;
pub const STAR_SHADER_PATH: &str = "/star.raster.hlsl";
pub const STAR_SHADER_NAME: &str = "star_shader";
/// `DesignatedRenderPass` 的载荷（O：`mov dword ptr [rax+r14+0x48], 0x10`）。
pub const STAR_DESIGNATED_PASS: u32 = 0x10;
/// `startup_stars` 的系统名（`add_systems` 的参数）。
pub const STARTUP_SYSTEM_NAME: &str = "startup_stars";

pub const SPEC: QuadSpec = QuadSpec {
    mesh_name: STAR_MESH_NAME,
    library_name: STAR_MESH_NAME,
    far_param: STAR_FAR_PARAM,
    shader_path: STAR_SHADER_PATH,
    shader_name: STAR_SHADER_NAME,
    designated_pass: Some(STAR_DESIGNATED_PASS),
};

/// `StarsPlugin`（impl$0）。
#[derive(Clone, Copy, Debug, Default)]
pub struct StarsPlugin;

impl StarsPlugin {
    /// `build`（O 0x140903c90）：`add_systems<Startup, startup_stars>`。
    pub fn build(&self, app: &mut impl AddStartupSystem) {
        app.add_startup_system(STARTUP_SYSTEM_NAME);
    }
}

/// `startup_stars`（O 0x140903ca0）。
pub fn startup_stars<H: StartupHost>(host: &mut H) {
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
        StarsPlugin.build(&mut app);
        assert_eq!(app.0, vec!["startup_stars"]);
    }

    #[test]
    fn startup_loads_the_star_mesh_and_shader_and_spawns_with_designated_pass() {
        let mut h = Rec::default();
        startup_stars(&mut h);
        let (lib_name, mesh) = &h.meshes[0];
        assert_eq!(lib_name, "star");
        assert_eq!(mesh.name, "star");
        assert_eq!(mesh.indices, vec![0, 1, 2, 2, 1, 3]);
        assert_eq!(mesh.far_param.to_bits(), 0x42aa9120, "0x142aa9130 处的 f32");
        assert_eq!(
            h.shaders,
            vec![("/star.raster.hlsl".to_string(), "star_shader".to_string())]
        );
        assert_eq!(STAR_SHADER_PATH.len(), 0x11);
        assert_eq!(STAR_SHADER_NAME.len(), 0xb);
        assert_eq!(h.spawned[0].designated_pass, Some(0x10));
    }
}
