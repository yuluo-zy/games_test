//! process_tree_cmds.rs — 树命令处理器（B 0x142267430 process_tree_cmds /
//! 0x1422686d0 despawn_trees，O；re-analyst-A 契约 §2）。
//!
//! - `process_tree_cmds`：消费 MoveTreeCmd/DeleteTreeCmd/InsertTreeCmd →
//!   prefab 路径前缀匹配（'clearing/naked_tree/naked1_no_mushrooms' 0x143034f28
//!   区等）→ 分派 spawn/替换/删除（TreeStorage::insert/remove +
//!   Commands::spawn_empty + random_tree_prefab）；log
//!   "Tree  already exists. Replacing it with a new tree"（0x143034f4f 区）。
//! - `despawn_trees`：`Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>`，
//!   α ≤ 0.01（0x142960674 = 0x3c23d70a）→ `DespawnPrefabCmd`。

use bevy_ecs::prelude::*;

use country_core::components::{PrefabInstanceData, TreeAnimation, TreeComponent, TreeMarkedForDeletion};
use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd};
use country_core::resources::session::ActiveSession;
use country_core::resources::tree_storage::{MoveTreeCmd, TreeStorage};
use country_core::resources::world_raster_defs::WaterRaster;
use country_core::systems::audio::PlayTreeSoundCmd;
use country_core::systems::prefab::{DespawnPrefabCmd, SpawnPrefabCmd};

/// despawn α 阈值（O：0x142960674 = 0x3c23d70a = 0.01）。
pub const DESPAWN_ALPHA_THRESHOLD: f32 = 0.01;

/// `process_tree_cmds` @0x142267430（O 面；37KB 的 prefab 分派细部——命令变体
/// 全表（0x19/0x1a/0x27 串分派）U，re-analyst §6；本实现落地命令消费 + 日志 +
/// 事件转发）。
#[allow(clippy::too_many_arguments)]
pub fn process_tree_cmds<S: Send + Sync + 'static>(
    mut storage: ResMut<TreeStorage>,
    mut instances: Query<&mut PrefabInstanceData>,
    mut moves: EventReader<MoveTreeCmd>,
    mut deletes: EventReader<DeleteTreeCmd>,
    mut inserts: EventReader<InsertTreeCmd>,
    mut spawns: EventWriter<SpawnPrefabCmd>,
    mut commands: Commands,
    mut sounds: EventWriter<PlayTreeSoundCmd>,
    _session: Res<ActiveSession>,
    _water: Res<WaterRaster>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
) {
    for _cmd in moves.read() {
        // O：移动命令 → 实例位置更新（0x8B/0x20B 命令记录字段 U）。
    }
    for cmd in deletes.read() {
        // O：删除命令 → TreeStorage::remove + despawn（0x58B 命令变体 2；
        // 真实树 id）。
        if let Some(entry) = storage.remove(country_core::resources::tree_storage::TreeStorageIndex(cmd.id))
        {
            despawns.write(DespawnPrefabCmd {
                entity: Some(entry.entity),
                _gap: cmd.id,
            });
        }
    }
    for cmd in inserts.read() {
        // O：插入命令 → spawn 真实树实体（树组件束）+ TreeStorage::insert
        // （真实 id/实体/位置/缩放）；替换冲突时打日志（O：0x143034f4f 区）。
        let entity = commands
            .spawn((
                TreeComponent,
                PrefabInstanceData {
                    position: cmd.position,
                    scale: cmd.scale,
                    rotation_degrees: 0.0,
                    _gap: 0,
                },
            ))
            .id();
        let replaced = storage.insert(
            country_core::resources::tree_storage::TreeStorageIndex(cmd.id),
            country_core::resources::tree_storage::TreeStorageEntry {
                entity,
                position: cmd.position,
            },
        );
        if replaced.is_some() {
            log::warn!(
                target: "system_tree::process_tree_cmds",
                "Tree  already exists. Replacing it with a new tree"
            );
        }
        spawns.write(SpawnPrefabCmd { _gap: cmd.id });
        // O：PlayTreeSoundCmd kind 0 = TreePlace（tree_audio 域）。
        sounds.write(PlayTreeSoundCmd {
            kind: 0,
            position: cmd.position,
        });
    }
}

/// `despawn_trees` @0x1422686d0（O：标记树 α≤0.01 回收——`DespawnPrefabCmd`）。
pub fn despawn_trees(
    mut trees: Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>,
    mut despawns: EventWriter<DespawnPrefabCmd>,
) {
    for (entity, animation, _marked) in trees.iter_mut() {
        // O：α ≤ 0.01 阈值门控（0x142960674 = 0.01 字节 ✓；α = TreeAnimation
        // 消退进度，字段 I——真实门控落地，不再无条件回收）。
        if animation.alpha <= DESPAWN_ALPHA_THRESHOLD {
            despawns.write(DespawnPrefabCmd {
                entity: Some(entity),
                _gap: 0,
            });
        }
    }
}
