//! 羊群调试快照的ECS所有者；字段绑定复用资源层的同一实现。
//! B原apply按查询序zip，不生成或删除实体。完整拾取/UI分支仍独立待恢复。
use crate::{
    resources::critters::{
        CritterGoal,
        sheep::{
            Sheep,
            control::{self, SheepControlSnapshot},
        },
    },
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Debug, Default)]
pub struct SheepControl {
    pub snapshot: Option<SheepControlSnapshot>,
}

pub fn sheep_controls(
    sheep: Query<(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)>,
    mut control: ResMut<SheepControl>,
) {
    control.snapshot = Some(control::capture_sheep_snapshot(
        sheep
            .iter()
            .map(|(_, transform, sheep, goal)| (transform, sheep, goal)),
    ));
}

pub fn apply_sheep_snapshot(
    mut sheep: Query<(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)>,
    control: Res<SheepControl>,
) {
    let Some(snapshot) = &control.snapshot else {
        return;
    };
    for ((_, mut transform, mut sheep, mut goal), saved) in sheep.iter_mut().zip(&snapshot.sheep) {
        control::apply_row(saved, &mut transform, &mut sheep, &mut goal);
    }
}
