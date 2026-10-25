//! 屋顶启动资源（B `startup_roof` @0x1414547d0）。
//!
//! 证据（expansion-b2 decomp + 字串池 0x142c615d8 区）：
//! * 文件式网格 4 个：`assets/meshes/roof_spire.json`（名 `roof_spire`）、
//!   `roof_support_beam.json`（名 `roof_support_beam`）、`roof_button.json`
//!   （名 `roof_button`）、`roof_delete_scribble.json`（名 `roof_delete_scribble`）。
//! * 着色器 2 个：`/instanced_roof_spire.raster.hlsl`→
//!   `instanced_roof_spire_shader`、`/instanced_roof_beams.raster.hlsl`→
//!   `support_beam_roof_shader`。
//! * 2 个 spawn 记录（O）；与 spire/beam 的配对为 I。
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use super::{StartupAssetHost, register_mesh_file};
use bevy_ecs::prelude::Res;

pub const ROOF_MESHES: [(&str, &str); 4] = [
    ("assets/meshes/roof_spire.json", "roof_spire"),
    ("assets/meshes/roof_support_beam.json", "roof_support_beam"),
    ("assets/meshes/roof_button.json", "roof_button"),
    (
        "assets/meshes/roof_delete_scribble.json",
        "roof_delete_scribble",
    ),
];

pub const ROOF_SHADERS: [(&str, &str); 2] = [
    (
        "/instanced_roof_spire.raster.hlsl",
        "instanced_roof_spire_shader",
    ),
    (
        "/instanced_roof_beams.raster.hlsl",
        "support_beam_roof_shader",
    ),
];

/// I：两条 spawn 记录的配对。
pub const ROOF_DRAWABLES: [(&str, &str); 2] = [
    ("roof_spire", "instanced_roof_spire_shader"),
    ("roof_support_beam", "support_beam_roof_shader"),
];

pub fn startup_roof<H: StartupAssetHost>(host: &mut H) {
    let mut meshes = Vec::new();
    for (path, name) in ROOF_MESHES {
        meshes.push((name, host.load_mesh(path, Some(name))));
    }
    let mut shaders = Vec::new();
    for (path, name) in ROOF_SHADERS {
        shaders.push((name, host.load_shader(path, name)));
    }
    for (mesh_name, shader_name) in ROOF_DRAWABLES {
        let mesh = meshes.iter().find(|(n, _)| *n == mesh_name).unwrap().1;
        let shader = shaders.iter().find(|(n, _)| *n == shader_name).unwrap().1;
        host.spawn_drawable(mesh, shader);
    }
}

pub fn startup_roof_meshes(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in ROOF_MESHES {
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
        spawns: usize,
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
            assert!(mesh >= 1 && shader >= 101);
            self.spawns += 1;
        }
    }

    #[test]
    fn roof_registers_four_meshes_two_shaders_and_two_spawns() {
        let mut host = Rec::default();
        startup_roof(&mut host);
        assert_eq!(host.meshes, ROOF_MESHES.map(|(p, n)| (p, Some(n))).to_vec());
        assert_eq!(host.shaders, ROOF_SHADERS.to_vec());
        assert_eq!(host.spawns, 2);
    }
}
