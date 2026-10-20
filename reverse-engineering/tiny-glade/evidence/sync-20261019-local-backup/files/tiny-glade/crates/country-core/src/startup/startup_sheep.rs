//! 羊资源载入（B `startup_sheep` @0x140a093e0）。
//!
//! 证据（decomp 0x140a093e0 字串池实读）：
//! * 文件式网格：`assets/meshes/sheep_animation/1.json`（名 `sheep_mesh`）、
//!   `sheep_animation/delighted.json`、`sheep_heart.json`（名 `sheep_heart`）、
//!   `umbrella.json`（名 `umbrella_mesh`）、`floatie.json`（名 `floatie_mesh`）。
//! * 着色器：`/sheep.raster.hlsl`→`sheep_shader`、`/solid_vertex_color.raster.hlsl`
//!   →`solid_vertex_color`。
//! * 顶点属性名：`Vertex_Position`、`Vertex_Normal`、`Vertex_UV`、`Vertex_Color`
//!   （第 5 次属性提取的字段名 U）。
//! * 音效：`event:/SFX/Sheep/Baah`，参数 `Pitch`。
//!
//! U：delighted 的注册名、第 5 个属性字段、动画网格注册链
//! （FUN_14130efe0/FUN_1412e23e0/FUN_1416f0ea0）的内部类型未恢复。
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use super::{StartupAssetHost, register_mesh_file};
use bevy_ecs::prelude::Res;

pub const SHEEP_MESHES: [(&str, Option<&str>); 5] = [
    ("assets/meshes/sheep_animation/1.json", Some("sheep_mesh")),
    // U: delighted.json 的注册名未在字串池单独出现。
    ("assets/meshes/sheep_animation/delighted.json", None),
    ("assets/meshes/sheep_heart.json", Some("sheep_heart")),
    ("assets/meshes/umbrella.json", Some("umbrella_mesh")),
    ("assets/meshes/floatie.json", Some("floatie_mesh")),
];

pub const SHEEP_SHADERS: [(&str, &str); 2] = [
    ("/sheep.raster.hlsl", "sheep_shader"),
    ("/solid_vertex_color.raster.hlsl", "solid_vertex_color"),
];

/// O：字串池 0x142ac6553 区；第 5 项 U。
pub const SHEEP_ATTRIBUTES: [&str; 4] = [
    "Vertex_Position",
    "Vertex_Normal",
    "Vertex_UV",
    "Vertex_Color",
];

pub const SHEEP_BAAH_EVENT: &str = "event:/SFX/Sheep/Baah";
pub const SHEEP_PITCH_PARAMETER: &str = "Pitch";

/// 5 个网格与 2 个着色器注册；羊是持久实体，由会话/存档生产者 spawn。
pub fn startup_sheep<H: StartupAssetHost>(host: &mut H) {
    for (path, name) in SHEEP_MESHES {
        host.load_mesh(path, name);
    }
    for (path, name) in SHEEP_SHADERS {
        host.load_shader(path, name);
    }
    // U: 5 次属性提取与动画网格注册链未恢复，不能伪造属性数据。
}

pub fn startup_sheep_meshes(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in SHEEP_MESHES {
        register_mesh_file(&meshes, path, name);
    }
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
        fn spawn_drawable(&mut self, _mesh: u64, _shader: u64) {}
    }

    #[test]
    fn registers_five_meshes_and_two_shaders_in_original_tables() {
        let mut host = Rec::default();
        startup_sheep(&mut host);
        assert_eq!(host.meshes, SHEEP_MESHES.to_vec());
        assert_eq!(host.shaders, SHEEP_SHADERS.to_vec());
        assert_eq!(SHEEP_ATTRIBUTES[0], "Vertex_Position");
        assert_eq!(SHEEP_BAAH_EVENT, "event:/SFX/Sheep/Baah");
        assert_eq!(SHEEP_PITCH_PARAMETER, "Pitch");
    }
}
