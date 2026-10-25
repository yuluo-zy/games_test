//! vao_update.rs — VAO 构建/重建系统（O）。
//!
//! 源 `systems/backend/vao_update.rs`。
//! `build_missing_vaos`（0x1408646b0）：遍历无 VAO 的网格，`AssetVAOLibrary::add`
//! 建 VAO，成功后给实体插 `Handle<Vao>` 组件（命令）。空网格日志
//! "Mesh is empty. Skip building VAO"。
//! `rebuild_vaos`（0x140864bf0）：遍历全部网格，匹配 `Vao` 绑定变化则
//! `AssetVAOLibrary::rebuild_vao`。

#![allow(dead_code)]

use crate::boundary::{AssetMeshLibrary, Vao};
use crate::typed_gap::Gap;

/// `build_missing_vaos`（O 0x1408646b0）。
pub fn build_missing_vaos(_world: &mut bevy_ecs::world::World) {
    // U：实体查询 + Handle<Vao> 组件插入命令（typed gap）。
}

/// `rebuild_vaos`（O 0x140864bf0）。
pub fn rebuild_vaos(_world: &mut bevy_ecs::world::World) {
    // U：网格→VAO 绑定比对（typed gap）。
}

/// VAO 请求（O：build_missing_vaos 内部逐网格处理；字段 U）。
pub struct VaoBuildRequest {
    pub _mesh: u64,
    pub _gap: Gap,
}

impl VaoBuildRequest {
    /// 空网格跳过（O 日志分支）。
    pub fn mesh_is_empty(&self) -> bool {
        false
    }
}

/// 绑定 `Handle<Vao>` 命令（O：`insert<Handle<Vao>>` command，0x28B 负载）。
pub struct InsertVaoCommand {
    pub mesh_handle: u64,
    pub vao: Vao,
    pub _gap: Gap,
}

/// `AssetVAOLibrary` 协作（O：`add`/`rebuild_vao` 见 asset_libraries::vao_library）。
pub fn note_mesh_library(_lib: &AssetMeshLibrary) {}
