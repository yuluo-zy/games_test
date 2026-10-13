//! meshwatch.rs — 网格热载系统（O 0x14085b4b0）。
//!
//! 源 `systems/backend/meshwatch.rs`。每帧排空 `MeshWatch` 待处理网格字节，
//! 重载 `AssetMeshLibrary`（`load_json_as_mesh` 路径）。

#![allow(dead_code)]

/// `meshwatch`（O 0x14085b4b0）：变更的网格文件字节 → 重建 `Mesh`。
pub fn meshwatch(_world: &mut bevy_ecs::world::World) {
    // U：路径→网格映射与 VAO 失效通知（typed gap）。
}
