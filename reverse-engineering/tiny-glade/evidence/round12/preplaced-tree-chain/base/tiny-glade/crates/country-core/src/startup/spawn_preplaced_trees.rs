//! 预置树生成（B `spawn_preplaced_trees` @0x140862270）。
//!
//! 证据（R10 契约 §2.6，O + 发行资产实读）：
//! * 不在 `add_game_startup_systems`；由 `save::impl$1::add_save_load_systems`
//!   （0x1409443b0）注册，`run_game_thread` 直调。本模块只提供系统，不改 save。
//! * 输入 `Res<ActiveSession>`（assets root、格调）、`ResMut<TreeStorage>`、`Commands`。
//! * 配置：`<assets_root>/assets/glade/<theme>/trees.json`，条目
//!   `{ prefab, placements }`；placements 文件为
//!   `[{ position:[x,y,z], id, scale, rotation_angle, pad0, pad1 }]`（发行资产实读）。
//! * 每棵树 `TreeStorage::insert` + 实体 spawn；日志 `Converting pre-placed trees`、
//!   `deserialize trees`。
//!
//! U：树实体的完整组件 bundle（TreeComponent/TreeAnimation/TreeMoveBend 等）、
//! prefab 名到实体的消费链、per-glade 路径表其余条目。当前只写已确认的
//! `Transform` + `InteractableForestTree` + `TreeStorage`。
use crate::resources::session::ActiveSession;
use crate::resources::tree_storage::{InteractableForestTree, TreeStorage, TreeStorageEntry};
use crate::utils::components::transform::Transform;
use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};
use std::path::{Path, PathBuf};

pub const TREES_FILE_NAME: &str = "trees.json";

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PreplacedTreeGroup {
    pub prefab: String,
    pub placements: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct TreePlacement {
    pub position: [f32; 3],
    pub id: u64,
    pub scale: f32,
    pub rotation_angle: f32,
    #[serde(default)]
    pub pad0: u32,
    #[serde(default)]
    pub pad1: u32,
}

/// O：`<assets_root>/assets/glade/<theme>/trees.json`。
pub fn trees_json_path(assets_root: impl AsRef<Path>, theme: &str) -> PathBuf {
    assets_root
        .as_ref()
        .join("assets")
        .join("glade")
        .join(theme)
        .join(TREES_FILE_NAME)
}

pub fn load_tree_groups(path: impl AsRef<Path>) -> anyhow::Result<Vec<PreplacedTreeGroup>> {
    let text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn load_tree_placements(
    glade_dir: impl AsRef<Path>,
    group: &PreplacedTreeGroup,
) -> anyhow::Result<Vec<TreePlacement>> {
    let text = std::fs::read_to_string(glade_dir.as_ref().join(&group.placements))?;
    Ok(serde_json::from_str(&text)?)
}

/// 读会话格调的 trees.json，逐条 spawn 并写入 `TreeStorage`。
///
/// 运行位置由 save 的注册点决定（契约 §2.6）；本函数不进入
/// `add_startup_mesh_systems`。
pub fn spawn_preplaced_trees(
    session: Option<Res<ActiveSession>>,
    mut storage: Option<ResMut<TreeStorage>>,
    mut commands: Commands,
) {
    let (Some(session), Some(storage)) = (session, storage.as_mut()) else {
        return;
    };
    let assets_root = session.assets_root();
    let theme = session.meta.glade.assets_folder();
    let groups_path = trees_json_path(&assets_root, theme);
    let groups = match load_tree_groups(&groups_path) {
        Ok(groups) => groups,
        Err(error) => {
            log::error!(
                target: "country_core::startup::spawn_preplaced_trees",
                "deserialize trees: {error:#}"
            );
            return;
        }
    };
    let glade_dir = assets_root.join("assets").join("glade").join(theme);
    log::info!(
        target: "country_core::startup::spawn_preplaced_trees",
        "Converting pre-placed trees"
    );
    for group in &groups {
        let placements = match load_tree_placements(&glade_dir, group) {
            Ok(placements) => placements,
            Err(error) => {
                log::error!(
                    target: "country_core::startup::spawn_preplaced_trees",
                    "deserialize trees ({}): {error:#}",
                    group.placements
                );
                continue;
            }
        };
        for placement in placements {
            let position = Vec3::from_array(placement.position);
            let rotation = Quat::from_rotation_y(placement.rotation_angle.to_radians());
            let entity = commands
                .spawn((
                    Transform {
                        rotation,
                        translation: position,
                        scale: Vec3::splat(placement.scale),
                    },
                    InteractableForestTree {
                        id: placement.id,
                        position,
                        scale: placement.scale,
                        _gap: 0,
                    },
                ))
                .id();
            let index = storage.next_index();
            storage.insert(index, TreeStorageEntry { entity, position });
            // U: prefab 名到渲染/动画组件的消费链未恢复。
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_assets_use_the_glade_theme_path_and_schema() {
        let path = trees_json_path("root", "summer");
        assert_eq!(path, PathBuf::from("root/assets/glade/summer/trees.json"));
        let group: PreplacedTreeGroup =
            serde_json::from_str(r#"{"prefab":"clearing/tree/mid1","placements":"trees/a.json"}"#)
                .unwrap();
        assert_eq!(group.prefab, "clearing/tree/mid1");
        let placement: TreePlacement = serde_json::from_str(
            r#"{"position":[-13.7,0.0,27.4],"id":52,"scale":1.06,"rotation_angle":110.3}"#,
        )
        .unwrap();
        assert_eq!(placement.id, 52);
        assert_eq!(placement.position, [-13.7, 0.0, 27.4]);
    }
}
