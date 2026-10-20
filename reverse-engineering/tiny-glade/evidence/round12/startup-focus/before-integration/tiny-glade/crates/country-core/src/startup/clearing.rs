//! Clearing（林间空地）资产（B `startup::clearing::init_clearing` @0x140a78aa0，
//! `uninit_clearing` @0x140a7a5b0）。
//!
//! 证据（R10 契约 §2.1，O）：
//! * 阶段：`init_clearing` = batch2/OnEnter(Game)；`uninit_clearing` = batch3（同 label）。
//! * 无条件网格 `assets/meshes/distant_billboard.json`（name `distant_billboard`）；
//!   格调判别 4（I：Winter）时才加载 `assets/meshes/ice_cracks.json`（name `ice_cracks`）
//!   与 `/water/ice_cracks.raster.hlsl`（name `ice_cracks`）。
//! * 两次 `Commands::spawn`（bundle 0x78）；容量常量 `min(x, 0x44c)` = 1100。
//! * `uninit_clearing` 遍历 27×0xc0 花园槽并置 `+0x38 = 1`；FMOD `snapshot:/Loading`。
//! * 另：`AudioTriggerLibrary::clear`、`AssetPlacementDataLibrary::clear`（I）。
//!
//! U：`param_10` 资源类型名；`util_add_clearing_shader` 复用方式；spawn bundle 的
//! pass/组件；两库 clear 的调用点类型。
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::resources::session::ActiveSession;
use super::{StartupAssetHost, register_mesh_file};
use bevy_ecs::prelude::Res;

pub const CLEARING_UNLOAD_SNAPSHOT: &str = "snapshot:/Loading";
pub const BILLBOARD_MESH: (&str, &str) = (
    "assets/meshes/distant_billboard.json",
    "distant_billboard",
);
pub const ICE_CRACKS_MESH: (&str, &str) = ("assets/meshes/ice_cracks.json", "ice_cracks");
pub const ICE_CRACKS_SHADER: (&str, &str) = ("/water/ice_cracks.raster.hlsl", "ice_cracks");

/// O：容量 = `min(x, 0x44c)`。
pub const CLEARING_CAPACITY: usize = 0x44c;
/// O：卸载遍历 27 × 0xc0 花园槽，置 +0x38 = 1。
pub const UNINIT_GARDEN_SLOTS: usize = 27;
pub const UNINIT_GARDEN_SLOT_SIZE: usize = 0xc0;
pub const UNINIT_GARDEN_FLAG_OFFSET: usize = 0x38;
/// O：init 的 spawn 记录数。
pub const CLEARING_SPAWN_RECORDS: usize = 2;

/// 无条件 billboard；冬季追加 ice_cracks 与 shader。
pub fn init_clearing<H: StartupAssetHost>(host: &mut H, is_winter: bool) {
    host.load_mesh(BILLBOARD_MESH.0, Some(BILLBOARD_MESH.1));
    if is_winter {
        host.load_mesh(ICE_CRACKS_MESH.0, Some(ICE_CRACKS_MESH.1));
        host.load_shader(ICE_CRACKS_SHADER.0, ICE_CRACKS_SHADER.1);
    }
    log::info!(
        target: "country_core::startup::clearing",
        "Initializing the glade."
    );
    // U: 两条 spawn 的 bundle/pass；主题产者与 trees spawn 不在 init_clearing 证据内。
}

pub fn init_clearing_meshes(meshes: Res<AssetMeshLibrary>, session: Option<Res<ActiveSession>>) {
    register_mesh_file(&meshes, BILLBOARD_MESH.0, Some(BILLBOARD_MESH.1));
    let is_winter = session.map(|session| session.is_winter()).unwrap_or(false);
    if is_winter {
        register_mesh_file(&meshes, ICE_CRACKS_MESH.0, Some(ICE_CRACKS_MESH.1));
    }
}

/// 卸载快照常量；FMOD 调用由音频域消费（`game_loading_audio` 已有同一字符串）。
pub fn uninit_clearing_snapshot() -> &'static str {
    CLEARING_UNLOAD_SNAPSHOT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        meshes: Vec<(&'static str, Option<&'static str>)>,
        shaders: Vec<(&'static str, &'static str)>,
    }
    impl StartupAssetHost for Rec {
        type Mesh = u64;
        type Shader = u64;
        fn load_mesh(&mut self, path: &'static str, name: Option<&'static str>) -> u64 {
            self.meshes.push((path, name));
            1
        }
        fn load_shader(&mut self, path: &'static str, name: &'static str) -> u64 {
            self.shaders.push((path, name));
            2
        }
        fn spawn_drawable(&mut self, _: u64, _: u64) {}
    }

    #[test]
    fn winter_adds_ice_cracks_and_the_shader_while_summer_does_not() {
        let mut summer = Rec::default();
        init_clearing(&mut summer, false);
        assert_eq!(
            summer.meshes,
            [(BILLBOARD_MESH.0, Some(BILLBOARD_MESH.1))].to_vec()
        );
        assert!(summer.shaders.is_empty());

        let mut winter = Rec::default();
        init_clearing(&mut winter, true);
        assert_eq!(winter.meshes.len(), 2);
        assert_eq!(winter.shaders, [ICE_CRACKS_SHADER].to_vec());
        assert_eq!(CLEARING_CAPACITY, 1100);
        assert_eq!(UNINIT_GARDEN_SLOTS * UNINIT_GARDEN_SLOT_SIZE, 0x1440);
        assert_eq!(uninit_clearing_snapshot(), "snapshot:/Loading");
        assert_eq!(
            crate::systems::audio::game_loading_audio::LOADING_SNAPSHOT,
            CLEARING_UNLOAD_SNAPSHOT
        );
    }
}
