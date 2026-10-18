//! SheepControl keeps a frozen snapshot and restores it each frame.
//! B0x1416f11a0 applies the owned snapshot before its trailing unit-event
//! reader clears that snapshot. Picking and the debug freeze toggle remain
//! with their actual UI owner; this system does not capture every frame.
use crate::{
    resources::critters::{
        CritterGoal,
        sheep::{
            Sheep,
            control::{self, SheepControlSnapshot},
        },
    },
    utils::components::transform::Transform,
    systems::history::finish_replay::CmdUnfreezeSheep,
};
use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Debug, Default)]
pub struct SheepControl {
    pub snapshot: Option<SheepControlSnapshot>,
}

pub fn sheep_controls(
    mut sheep: Query<(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)>,
    mut control: ResMut<SheepControl>,
    mut unfreeze: EventReader<CmdUnfreezeSheep>,
) {
    if let Some(snapshot) = &control.snapshot {
        for ((_, mut transform, mut sheep, mut goal), saved) in sheep.iter_mut().zip(&snapshot.sheep) {
            control::apply_row(saved, &mut transform, &mut sheep, &mut goal);
        }
    }
    // B0x1416f19e8..1a70 consumes all unread unit events. Clearing occurs
    // after the current frame's restore, including a snapshot of zero sheep.
    if unfreeze.read().count() != 0 {
        control.snapshot = None;
    }
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

#[cfg(test)]
mod replay_unfreeze_tests {
    use super::*;
    use crate::resources::critters::sheep::{SheepMoveState, control::SheepSnapshot};
    use glam::Vec3;

    #[test]
    fn frozen_snapshot_restores_before_unfreeze_then_stays_released() {
        let mut world = World::new();
        world.init_resource::<Events<CmdUnfreezeSheep>>();
        let saved = SheepSnapshot {
            position: Vec3::X,
            goal: None,
            move_state: SheepMoveState::Idle { timeout_s: 2.0 },
        };
        world.insert_resource(SheepControl { snapshot: Some(SheepControlSnapshot { sheep: vec![saved] }) });
        let entity = world.spawn((
            Transform::from_translation(Vec3::splat(9.0)),
            Sheep { move_state: SheepMoveState::Idle { timeout_s: 9.0 }, being_petted_timer: 5.0 },
            CritterGoal { goal_tag: 1, position: Vec3::splat(9.0) },
        )).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(sheep_controls);
        schedule.run(&mut world);
        assert_eq!(world.get::<Transform>(entity).unwrap().translation, Vec3::X);
        assert!(world.resource::<SheepControl>().snapshot.is_some());
        world.get_mut::<Transform>(entity).unwrap().translation = Vec3::splat(3.0);
        world.resource_mut::<Events<CmdUnfreezeSheep>>().write(CmdUnfreezeSheep);
        schedule.run(&mut world);
        assert_eq!(world.get::<Transform>(entity).unwrap().translation, Vec3::X);
        assert!(world.resource::<SheepControl>().snapshot.is_none());
        world.get_mut::<Transform>(entity).unwrap().translation = Vec3::splat(4.0);
        schedule.run(&mut world);
        assert_eq!(world.get::<Transform>(entity).unwrap().translation, Vec3::splat(4.0));
        assert!(world.resource::<SheepControl>().snapshot.is_none());
        assert_eq!(world.get::<Sheep>(entity).unwrap().being_petted_timer, 5.0);
    }
}
