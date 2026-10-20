//! 墙启动网格/着色器（B `startup_walls::glade_startup_walls` @0x140a92670）。
//!
//! 证据（R10 契约 §2.13，O）：
//! * shader `/wall/wall_brick_lod0.raster.hlsl`→`wall_brick_lod0`、
//!   `/wall/wall_brick_lod1.raster.hlsl`→`wall_brick_lod1`。
//! * 文件式网格 `assets/meshes/brick.json`（name `brick`）。
//! * 程序化网格 `half_cube`（`create_half_cube_mesh`，源码行 120 内联）。
//! * 7 个 spawn（LOD0/LOD1 与 pass 组合，逐项映射 U）。
//!
//! U：`create_half_cube_mesh` 的顶点/索引常量未恢复；本文件只创建有正确名字的
//! 空网格骨架，绝不编造几何。`once_startup_walls` 属 batch1/PreStartup，由宿主
//! `PreStartup` 表持有，本模块不重复注册。
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::render::mesh::Mesh;
use super::{StartupAssetHost, register_mesh_file};
use bevy_ecs::prelude::Res;

pub const WALL_SHADERS: [(&str, &str); 2] = [
    ("/wall/wall_brick_lod0.raster.hlsl", "wall_brick_lod0"),
    ("/wall/wall_brick_lod1.raster.hlsl", "wall_brick_lod1"),
];

pub const BRICK_MESH_NAME: &str = "brick";
pub const BRICK_MESH_PATH: &str = "assets/meshes/brick.json";
pub const HALF_CUBE_NAME: &str = "half_cube";

/// O：`glade_startup_walls` 的 spawn 记录数。
pub const WALL_DRAWABLE_RECORDS: usize = 7;

/// U：原作的顶点/索引常量载荷未恢复。
pub fn create_half_cube_mesh() -> Mesh {
    Mesh::new(HALF_CUBE_NAME)
}

/// 2 个 LOD 着色器 + brick 网格；程序化 half_cube 由宿主直接登记。
pub fn glade_startup_walls<H: StartupAssetHost>(host: &mut H) {
    host.load_shader(WALL_SHADERS[0].0, WALL_SHADERS[0].1);
    host.load_shader(WALL_SHADERS[1].0, WALL_SHADERS[1].1);
    host.load_mesh(BRICK_MESH_PATH, Some(BRICK_MESH_NAME));
    // U: `half_cube` 的几何数据未恢复，宿主负责把 `create_half_cube_mesh()`
    //    登记进真实网格库；7 条 drawable 的 pass 编号未恢复。
}

/// ECS 网格产者：只写入真实存在的 `brick.json`。
///
/// U：`create_half_cube_mesh` 的几何常量未恢复，因此**不**把空网格写入生产
/// `AssetMeshLibrary`；`TerrainHeightsData::subscribe` 的订阅者所有者未恢复，
/// 同样不在这里创建。
pub fn startup_walls_meshes(meshes: Res<AssetMeshLibrary>) {
    register_mesh_file(&meshes, BRICK_MESH_PATH, Some(BRICK_MESH_NAME));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        shaders: Vec<(&'static str, &'static str)>,
        meshes: Vec<(&'static str, Option<&'static str>)>,
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
    fn wall_lods_and_brick_mesh_are_loaded_as_original_tables() {
        let mut host = Rec::default();
        glade_startup_walls(&mut host);
        assert_eq!(host.shaders, WALL_SHADERS.to_vec());
        assert_eq!(
            host.meshes,
            [(BRICK_MESH_PATH, Some(BRICK_MESH_NAME))].to_vec()
        );
        assert_eq!(WALL_DRAWABLE_RECORDS, 7);
    }

    #[test]
    fn procedural_half_cube_keeps_the_original_name_without_fake_geometry() {
        let mesh = create_half_cube_mesh();
        assert_eq!(mesh.label, HALF_CUBE_NAME);
        assert!(mesh.attributes.is_empty());
        assert!(mesh.indices.is_empty());
    }
}
