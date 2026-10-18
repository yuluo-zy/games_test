//! 加载常春藤枝干、叶片和花朵网格；着色器与实体由宿主渲染阶段装配。
use crate::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use crate::utils::load_json::load_json_as_mesh;
use bevy_ecs::prelude::*;

/// 常春藤网格路径与库内业务名称。
pub const IVY_MESHES: [(&str, &str); 3] = [
    ("assets/meshes/ivy_branch.json", "ivy_branch"),
    ("assets/meshes/ivy_leaf.json", "ivy_leaf"),
    ("assets/meshes/ivy_flower.json", "ivy_flower"),
];

/// 一次性注册三类常春藤网格，复用已有 JSON 网格读取和资产库。
pub fn startup_ivy(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in IVY_MESHES {
        let mesh = load_json_as_mesh(path);
        meshes.add(MeshAsset::new(mesh, Some(name.to_string())));
    }
}
