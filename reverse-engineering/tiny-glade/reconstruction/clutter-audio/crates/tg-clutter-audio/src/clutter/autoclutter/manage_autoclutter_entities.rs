//! manage_autoclutter_entities 业务恢复草稿（B 0x1422d04e0，size 10301，源行 0–247）
//!
//! 证据：reva 反编译（55KB）+ 全函数反汇编（1706 指令）+ PDB 行号。
//! 闭包：closure$0（1422d2d20×2）、closure$1=L56、closure$3=L85、closure$4=L95、
//! closure$5=L100、update_shutter=L249。

use bevy_ecs::prelude::*;
use bevy_ecs::system::Commands;
use glam::Quat;

use super::{AutoClutterObject, AutoClutterState, GlobalId};
use crate::glue::Transform;

/// 现存实体集（from_iter<u64, &AutoClutterObject>；+ Map<QueryIter<Entity,...>>）。
pub fn manage_autoclutter_entities(
    mut state: ResMut<AutoClutterState>,
    mut commands: Commands,
    entities: Query<(Entity, &AutoClutterObject)>,
    _grid: Query<&Transform>,
) {
    // 证据：现存实体键集。
    let existing: std::collections::HashSet<u64> =
        entities.iter().map(|(_, o)| o.key).collect();
    // 证据：目标键集 = state.spawn_candidates 键。
    let expected: std::collections::HashSet<u64> =
        state.spawn_candidates.iter().map(|s| s.key).collect();

    // 新增：Commands::spawn_empty（B 140d146a0）+ insert<ColorId>。
    for key in expected.difference(&existing) {
        let _entity = commands.spawn_empty().id();
        // TODO(evidence): 组件集合（ColorId、位置、朝向）与 insert 命令 closure 精确内容。
        let _ = key;
    }
    // 删除：insert<ClutterMarkedForDeletion> 命令（B 经 entity_command insert closure）。
    for key in existing.difference(&expected) {
        let _ = key;
    }

    // 更新：per-entity closure_0 + 四元数朝向（两次叉积+归一化）+ init_rng 抖动。
    // 证据：byte@+0x30 分支（==2 或 -3 后 <4 且 !=1）触发更新命令。
    for spawn in &state.spawn_candidates {
        let _orient: Quat = orient_look(spawn.position);
        let _ = &mut commands;
    }
}

/// 朝向计算（证据：manage 内联两次叉积 + 1/SQRT 归一化；fVar18=1.0、fVar56=0.5 常量）。
fn orient_look(_pos: u64) -> Quat {
    // TODO(evidence): 目标向量来源（墙面/区域法向）。
    Quat::IDENTITY
}

/// 需要删除的杂物标记组件（country_core resources clutter_meshes）。
pub struct ClutterMarkedForDeletion;

/// 颜色 id 组件（insert<ColorId> 命令）。
pub struct ColorId;
