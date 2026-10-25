//! 羊群控制快照。B capture 0x1412f03f0 与 serializer 0x1416f9eb0
//! 共同确认：position=Transform.translation，goal=CritterGoal，
//! move_state=Sheep+0x30。声明顺序不代表原生内存顺序。
use super::{Sheep, SheepMoveState};
use crate::resources::critters::CritterGoal;
use crate::utils::components::transform::Transform;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// One saved sheep row. Field names verified by rne-read on the B image:
/// `sheep` at 0x142c91fa9; `position`/`goal`/`move_state` at
/// 0x142c91f4c/0x142c91f54/0x142c91f58 (re-analyst, 2026-10-15). The serde
/// serialize (0x1416f9eb0) writes exactly these three entries in this order.
/// The save file's Sheep section is `Option<SheepControlSnapshot>` (None →
/// "null").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SheepSnapshot {
    pub position: Vec3,
    pub goal: Option<Vec3>,
    pub move_state: SheepMoveState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SheepControlSnapshot {
    pub sheep: Vec<SheepSnapshot>,
}

/// 按实际查询顺序捕获三个权威字段，不排序或保存实体ID。
pub fn capture_sheep_snapshot<'a>(
    rows: impl IntoIterator<Item = (&'a Transform, &'a Sheep, &'a CritterGoal)>,
) -> SheepControlSnapshot {
    SheepControlSnapshot {
        sheep: rows
            .into_iter()
            .map(|(transform, sheep, critter_goal)| SheepSnapshot {
                position: transform.translation,
                goal: (critter_goal.goal_tag == 1).then_some(critter_goal.position),
                move_state: sheep.move_state,
            })
            .collect(),
    }
}

/// 单行恢复供纯函数与ECS消费者共用；不修改旋转、缩放或抚摸状态。
pub(crate) fn apply_row(
    saved: &SheepSnapshot,
    transform: &mut Transform,
    sheep: &mut Sheep,
    goal: &mut CritterGoal,
) {
    transform.translation = saved.position;
    sheep.move_state = saved.move_state;
    goal.position = saved.goal.unwrap_or_default();
    goal.goal_tag = saved.goal.is_some() as u32;
}

/// 原apply按查询顺序zip到较短一侧，只恢复位置、目标和移动状态。
/// rotation/scale、抚摸计时和额外实体保持原值。
pub fn apply_sheep_snapshot<'a>(
    snapshot: &SheepControlSnapshot,
    rows: impl IntoIterator<Item = (&'a mut Transform, &'a mut Sheep, &'a mut CritterGoal)>,
) {
    for ((transform, sheep, critter_goal), saved) in rows.into_iter().zip(&snapshot.sheep) {
        apply_row(saved, transform, sheep, critter_goal);
    }
}

/// save "Sheep" 节载荷（O `to_json_string<Option<SheepControlSnapshot>>`
/// @0x140a51d40，save_game.rs 第 5 节）：`None` → JSON `null`，`Some` →
/// `{"sheep":[...]}`。**纯函数接口，供 C 组 save agent 在 `systems/save/save_game.rs`
/// 接线**（中央 agent 2026-10-16 仲裁：B 组提供接口，C 组写盘，避免并行写同文件）。
pub fn sheep_section_json(snapshot: Option<&SheepControlSnapshot>) -> String {
    serde_json::to_string(&snapshot).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_wire_fields_and_variants() {
        let snapshot = SheepControlSnapshot {
            sheep: vec![SheepSnapshot {
                position: Vec3::new(1., 2., 3.),
                goal: None,
                move_state: SheepMoveState::Walking {
                    timeout_s: 4.,
                    goal_change_timeout: 5.,
                    force_reach_goal: true,
                },
            }],
        };
        let json = serde_json::to_string(&snapshot).unwrap();
        assert_eq!(
            json,
            r#"{"sheep":[{"position":[1.0,2.0,3.0],"goal":null,"move_state":{"Walking":{"timeout_s":4.0,"goal_change_timeout":5.0,"force_reach_goal":true}}}]}"#
        );
        assert_eq!(
            serde_json::from_str::<SheepControlSnapshot>(&json).unwrap(),
            snapshot
        );
        assert_eq!(
            serde_json::to_string(&SheepMoveState::Idle { timeout_s: 2. }).unwrap(),
            r#"{"Idle":{"timeout_s":2.0}}"#
        );
    }

    #[test]
    fn capture_and_apply_bind_real_positions_goals_and_movement() {
        let position = Vec3::new(1., 2., 3.);
        let target = Vec3::new(7., 8., 9.);
        let movement = SheepMoveState::Walking {
            timeout_s: 4.,
            goal_change_timeout: 5.,
            force_reach_goal: true,
        };
        let snapshot = capture_sheep_snapshot([(
            &Transform::from_translation(position),
            &Sheep {
                behavior_state: 0,
                goal_timer: 0.0,
                pet_phase: 0,
                pet_progress: 0.0,
                pet_target: 0,
                control_flag: 0,
                move_state: movement,
                amplitude: 9.,
            },
            &CritterGoal {
                position: target,
                goal_tag: 1,
            },
        )]);
        assert_eq!(snapshot.sheep[0].position, position);
        assert_eq!(snapshot.sheep[0].goal, Some(target));
        assert_eq!(snapshot.sheep[0].move_state, movement);
        let mut transforms = [
            Transform::from_scale(Vec3::splat(3.)),
            Transform::identity(),
        ];
        let initial_sheep = Sheep {
            behavior_state: 1,
            goal_timer: 2.25,
            pet_phase: 3,
            pet_progress: 0.625,
            pet_target: 17,
            control_flag: 1,
            move_state: SheepMoveState::Idle { timeout_s: 17. },
            amplitude: 19.,
        };
        let mut sheeps = [initial_sheep; 2];
        let initial_goal = CritterGoal {
            position: Vec3::ZERO,
            goal_tag: 0,
        };
        let mut goals = [initial_goal; 2];
        apply_sheep_snapshot(
            &snapshot,
            transforms
                .iter_mut()
                .zip(sheeps.iter_mut())
                .zip(goals.iter_mut())
                .map(|((t, s), g)| (t, s, g)),
        );
        assert_eq!(transforms[0].translation, position);
        assert_eq!(transforms[0].scale, Vec3::splat(3.));
        let mut expected_sheep = initial_sheep;
        expected_sheep.move_state = movement;
        assert_eq!(sheeps[0], expected_sheep);
        assert_eq!(goals[0].position, target);
        assert_eq!(goals[0].goal_tag, 1);
        assert_eq!(transforms[1], Transform::identity());
        assert_eq!(sheeps[1], initial_sheep);
        assert_eq!(goals[1], initial_goal);
    }

    #[test]
    fn sheep_section_is_option_json_none_to_null() {
        assert_eq!(sheep_section_json(None), "null");
        let snapshot = capture_sheep_snapshot([(
            &Transform::from_translation(Vec3::new(1., 2., 3.)),
            &Sheep {
                behavior_state: 0,
                goal_timer: 0.0,
                pet_phase: 0,
                pet_progress: 0.0,
                pet_target: 0,
                control_flag: 0,
                move_state: SheepMoveState::Idle { timeout_s: 2. },
                amplitude: 0.,
            },
            &CritterGoal {
                position: Vec3::new(7., 8., 9.),
                goal_tag: 0,
            },
        )]);
        assert_eq!(
            sheep_section_json(Some(&snapshot)),
            r#"{"sheep":[{"position":[1.0,2.0,3.0],"goal":null,"move_state":{"Idle":{"timeout_s":2.0}}}]}"#
        );
    }
}

/// Loading before OnEnter(Game) can precede the original control component's
/// birth. Resource presence means one pending transfer, including a None clear.
/// This DTO is not a live SheepControl and never owns an applied snapshot.
#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct PendingSheepSnapshot(pub Option<SheepControlSnapshot>);

/// Query the unique real owner, or retain a pre-birth DTO until startup consumes
/// it. A duplicate owner is an error rather than selecting an arbitrary entity.
pub fn install_loaded_sheep_snapshot(
    world: &mut bevy_ecs::world::World,
    snapshot: Option<SheepControlSnapshot>,
) -> Result<(), &'static str> {
    use crate::systems::debug::sheep_controls::SheepControl;
    use bevy_ecs::{entity::Entity, query::With};
    let owners: Vec<_> = world
        .query_filtered::<Entity, With<SheepControl>>()
        .iter(world)
        .collect();
    match owners.as_slice() {
        [] => {
            world.insert_resource(PendingSheepSnapshot(snapshot));
        }
        [entity] => {
            world.remove_resource::<PendingSheepSnapshot>();
            world
                .get_mut::<SheepControl>(*entity)
                .expect("validated live sheep control")
                .snapshot = snapshot;
        }
        _ => return Err("multiple live SheepControl components"),
    }
    Ok(())
}
