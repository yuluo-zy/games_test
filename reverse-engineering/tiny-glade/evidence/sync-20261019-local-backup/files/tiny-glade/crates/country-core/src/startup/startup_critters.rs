//! 小动物资源载入（B `startup_critters` @0x1409d49a0）。
//!
//! 证据（decomp 0x1409d49a0 字串池实读）：
//! * 文件式网格：`assets/meshes/butterfly.json`（名 `butterfly`）、
//!   `duck_flying.json`（名 `duck`）、`duck_heart.json`（名 `duck_heart`）、
//!   `splash.json`（名 `splash`）。
//! * 着色器：`/critters/butterfly.raster.hlsl`→`butterfly_shader`、
//!   `/critters/firefly.raster.hlsl`→`firefly_shader`、
//!   `/critters/firefly_glow.raster.hlsl`→`firefly_glow_shader`、
//!   `/critters/duck_animated.raster.hlsl`→`duck_shader`、
//!   `/splash_particle.raster.hlsl`→`splash_particle`。
//! * 音效：`event:/SFX/Butterfly/Shoosh`。
//! * 4 个 spawn 记录。
//!
//! I：网格→着色器的配对与 spawn 顺序由同族命名推断（原件 4 条记录未逐条
//! 恢复字段）；U：firefly 网格加载条目未在字串池单独出现（可能复用 butterfly/
//! splash 网格），其余 bundle 字段未恢复。
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use super::{StartupAssetHost, register_mesh_file};
use bevy_ecs::prelude::Res;

pub const CRITTER_MESHES: [(&str, &str); 4] = [
    ("assets/meshes/butterfly.json", "butterfly"),
    ("assets/meshes/duck_flying.json", "duck"),
    ("assets/meshes/duck_heart.json", "duck_heart"),
    ("assets/meshes/splash.json", "splash"),
];

/// 契约 §2.7 的名式网格：`firefly` 没有对应 `assets/meshes/firefly.json`，
/// 只能经 mesh atlas/prefab 通道按名查找；该通道未恢复，故不注册伪造文件。
pub const CRITTER_NAMED_MESHES: [&str; 1] = ["firefly"];

pub const CRITTER_SHADERS: [(&str, &str); 5] = [
    ("/critters/butterfly.raster.hlsl", "butterfly_shader"),
    ("/critters/firefly.raster.hlsl", "firefly_shader"),
    ("/critters/firefly_glow.raster.hlsl", "firefly_glow_shader"),
    ("/critters/duck_animated.raster.hlsl", "duck_shader"),
    ("/splash_particle.raster.hlsl", "splash_particle"),
];

/// O（0x142ac22f1 区）。
pub const BUTTERFLY_SHOOSH_EVENT: &str = "event:/SFX/Butterfly/Shoosh";

/// I：4 条 spawn 记录的网格/着色器配对。
pub const CRITTER_DRAWABLES: [(&str, &str); 4] = [
    ("butterfly", "butterfly_shader"),
    ("duck", "duck_shader"),
    ("duck_heart", "duck_shader"),
    ("splash", "splash_particle"),
];

/// O：4 条 spawn 记录的 bundle 尺寸分组。
pub const CRITTER_SPAWN_BUNDLES: [(usize, usize); 3] = [(0x78, 2), (0x88, 1), (0xa8, 1)];

/// 网格/着色器注册与 4 个实体 spawn；音效事件由宿主音频域消费常量。
pub fn startup_critters<H: StartupAssetHost>(host: &mut H) {
    let mut meshes = Vec::new();
    for (path, name) in CRITTER_MESHES {
        meshes.push((name, host.load_mesh(path, Some(name))));
    }
    let mut shaders = Vec::new();
    for (path, name) in CRITTER_SHADERS {
        shaders.push((name, host.load_shader(path, name)));
    }
    for (mesh_name, shader_name) in CRITTER_DRAWABLES {
        let mesh = meshes
            .iter()
            .find(|(name, _)| *name == mesh_name)
            .unwrap_or_else(|| panic!("missing critter mesh {mesh_name}"))
            .1;
        let shader = shaders
            .iter()
            .find(|(name, _)| *name == shader_name)
            .unwrap_or_else(|| panic!("missing critter shader {shader_name}"))
            .1;
        host.spawn_drawable(mesh, shader);
    }
    // U: `event:/SFX/Butterfly/Shoosh` 的 FMOD 注册对象类型未恢复。
}

pub fn startup_critters_meshes(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in CRITTER_MESHES {
        register_mesh_file(&meshes, path, Some(name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        meshes: Vec<(&'static str, Option<&'static str>)>,
        shaders: Vec<(&'static str, &'static str)>,
        spawns: Vec<(u64, u64)>,
    }

    impl StartupAssetHost for Rec {
        type Mesh = u64;
        type Shader = u64;
        fn load_mesh(&mut self, path: &'static str, name: Option<&'static str>) -> u64 {
            self.meshes.push((path, name));
            self.meshes.len() as u64
        }
        fn load_shader(&mut self, path: &'static str, name: &'static str) -> u64 {
            self.shaders.push((path, name));
            100 + self.shaders.len() as u64
        }
        fn spawn_drawable(&mut self, mesh: u64, shader: u64) {
            self.spawns.push((mesh, shader));
        }
    }

    #[test]
    fn loads_four_file_meshes_five_shaders_and_spawns_four() {
        let mut host = Rec::default();
        startup_critters(&mut host);
        assert_eq!(host.meshes, CRITTER_MESHES.map(|(p, n)| (p, Some(n))).to_vec());
        assert_eq!(host.shaders, CRITTER_SHADERS.to_vec());
        assert_eq!(host.spawns.len(), 4);
        // butterfly mesh=1, butterfly shader=101
        assert_eq!(host.spawns[0], (1, 101));
        assert_eq!(CRITTER_NAMED_MESHES, ["firefly"]);
        let bundles: usize = CRITTER_SPAWN_BUNDLES.iter().map(|(_, c)| c).sum();
        assert_eq!(bundles, 4);
        assert_eq!(BUTTERFLY_SHOOSH_EVENT, "event:/SFX/Butterfly/Shoosh");
    }
}
