//! `systems/debug/sheep_controls.rs` — B 真实 Sheep query 所有者。
//!
//! 证据：
//! - `sheep_controls` @0x1416f11a0（query `(Entity, &mut Transform, &mut Sheep,
//!   &mut CritterGoal)`；步进 `uVar9 * 0x68` = Sheep stride；调用 capture iterator
//!   0x1412f03f0 / apply 0x1416f24f0）——[O，sheep-SPEC §2.5]。
//! - `SheepControl` 调试资源 @0x1416f0ea0（native 持有 `Vec<SheepSnapshot>`
//!   cap-first + 跟随位置；本模型用 `Option<SheepControlSnapshot>` 表示捕获输出）。
//! - 原生系统还内联 picking/相机以驱动 UI 交互（`PickingMode::is_some`、拾取后写
//!   move_state `= 0x101`（Walking+force_reach_goal，写入 **Transform.translation 槽**，
//!   非 `Sheep+0x30`——2026-10-17 fresh 反编译修正：`+0x30` 是 goal payload）与
//!   `_UNK_142abcb20` timeout_s）——那些触发条件属 picking/UI 域，待取证，本文件不伪造。
//! - 中央 agent（a8a2e9731c4957436）2026-10-16 仲裁批准落 `systems/debug/*`。

use crate::resources::critters::{
    CritterGoal,
    sheep::{
        Sheep, SheepMoveState,
        control::{SheepControlSnapshot, SheepSnapshot},
    },
};
use crate::utils::components::transform::Transform;
use bevy_ecs::prelude::*;

/// `SheepControl` 调试资源（@0x1416f0ea0）：捕获的 `Option<SheepControlSnapshot>`
/// （无羊 → None，与 save "Sheep" 节同为 None→null 语义）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SheepControl {
    pub snapshot: Option<SheepControlSnapshot>,
}

/// `sheep_controls` @0x1416f11a0 — 真实 query 所有者（capture 生产者）。
///
/// 按 query 实际顺序捕获 `(Transform.translation, CritterGoal, Sheep)` 的
/// **原生绑定**（O，2026-10-17 修正）：`position ← CritterGoal@+0x00`、
/// `goal ← Sheep@+0x30 payload + CritterGoal@+0x0c tag`、
/// `move_state ← utils Transform.translation@+0x10`，存进 [`SheepControl`]。
/// 原生不存 Entity、不排序（O）。拾取后写 move_state 的交互分支
/// （0x1416f11a0 行 547..548）依赖 picking，待取证，此处只做捕获。
pub fn sheep_controls(
    sheep: Query<(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)>,
    mut control: ResMut<SheepControl>,
) {
    control.snapshot = Some(SheepControlSnapshot {
        sheep: sheep
            .iter()
            .map(|(_, transform, sheep, goal)| SheepSnapshot {
                position: goal.position,
                goal: (goal.goal_tag != 0).then_some(sheep.goal),
                move_state: SheepMoveState::from_translation(transform.translation),
            })
            .collect(),
    });
}

/// `apply_sheep_snapshot` @0x1416f24f0 — restore 消费者。
///
/// 按 query 序 zip 快照、短路到较短一侧；只写 `Transform.translation`、
/// `Sheep@+0x30`、`CritterGoal`，不动 rotation/scale/其它字段（O）。
/// 原生 0x1416f11a0 内联调用它；本系统把它暴露为独立 ECS 消费者接口。
pub fn apply_sheep_snapshot(
    mut sheep: Query<(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)>,
    control: Res<SheepControl>,
) {
    let Some(snapshot) = &control.snapshot else {
        return;
    };
    for ((_, mut transform, mut sheep, mut goal), saved) in sheep.iter_mut().zip(&snapshot.sheep) {
        transform.translation = SheepMoveState::to_translation(saved.move_state);
        sheep.goal = saved.goal.unwrap_or_default();
        goal.position = saved.position;
        goal.goal_tag = saved.goal.is_some() as u32;
    }
}
