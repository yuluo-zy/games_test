//! B control snapshots: native serializer 0x1416f9eb0/0x1416f9fd0,
//! capture iterator 0x1412f03f0, restore consumer 0x1416f24f0.
//! Borrowed rows expose the original query's fields without inventing the
//! still-incomplete whole Sheep owner or copying unrelated animation state.
use super::SheepMoveState;
use crate::resources::critters::CritterGoal;
use crate::utils::components::transform::Transform;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// One saved sheep row. Field names verified by rne-read on the B image:
/// `sheep` at 0x142c91fa9; `position`/`goal`/`move_state` at
/// 0x142c91f49/0x142c91f51/0x142c91f55 (re-analyst, 2026-10-15). The serde
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

/// Caller passes rows in actual query order. Native capture stores no entity ID
/// and does not sort: +0x1c position, +0 goal, +0x10 movement state.
pub fn capture_sheep_snapshot<'a>(
    rows: impl IntoIterator<Item = (&'a Transform, &'a CritterGoal, &'a SheepMoveState)>,
) -> SheepControlSnapshot {
    SheepControlSnapshot {
        sheep: rows
            .into_iter()
            .map(|(transform, goal, move_state)| SheepSnapshot {
                position: transform.translation,
                goal: goal.0,
                move_state: *move_state,
            })
            .collect(),
    }
}

/// Original restore zips query order with saved order, stops at the shorter
/// input and overwrites exactly these three fields. It neither creates nor
/// removes sheep, and preserves transform rotation/scale and other sheep state.
pub fn apply_sheep_snapshot<'a>(
    snapshot: &SheepControlSnapshot,
    rows: impl IntoIterator<
        Item = (
            &'a mut Transform,
            &'a mut CritterGoal,
            &'a mut SheepMoveState,
        ),
    >,
) {
    for ((transform, goal, move_state), saved) in rows.into_iter().zip(&snapshot.sheep) {
        transform.translation = saved.position;
        goal.0 = saved.goal;
        *move_state = saved.move_state;
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
    fn capture_and_apply_use_query_order_and_preserve_other_fields() {        let original = Transform::from_translation_scale(Vec3::new(1., 2., 3.), Vec3::splat(2.));
        let original_goal = CritterGoal(Some(Vec3::new(4., 5., 6.)));
        let original_move = SheepMoveState::Idle { timeout_s: 7. };
        let snapshot = capture_sheep_snapshot([(&original, &original_goal, &original_move)]);
        let mut transforms = [
            Transform::from_scale(Vec3::splat(3.)),
            Transform::identity(),
        ];
        let mut goals = [CritterGoal(None); 2];
        let mut states = [SheepMoveState::Idle { timeout_s: 0. }; 2];
        apply_sheep_snapshot(
            &snapshot,
            transforms
                .iter_mut()
                .zip(goals.iter_mut())
                .zip(states.iter_mut())
                .map(|((a, b), c)| (a, b, c)),
        );
        assert_eq!(transforms[0].translation, original.translation);
        assert_eq!(transforms[0].scale, Vec3::splat(3.));
        assert_eq!(goals[0], original_goal);
        assert_eq!(states[0], original_move);
        assert_eq!(transforms[1], Transform::identity());
        assert_eq!(goals[1], CritterGoal(None));
        assert_eq!(states[1], SheepMoveState::Idle { timeout_s: 0. });
    }

    #[test]
    fn sheep_section_is_option_json_none_to_null() {
        assert_eq!(sheep_section_json(None), "null");
        let snapshot = capture_sheep_snapshot([(
            &Transform::from_translation_scale(Vec3::new(1., 2., 3.), Vec3::ONE),
            &CritterGoal(None),
            &SheepMoveState::Idle { timeout_s: 2. },
        )]);
        let json = sheep_section_json(Some(&snapshot));
        assert_eq!(
            json,
            r#"{"sheep":[{"position":[1.0,2.0,3.0],"goal":null,"move_state":{"Idle":{"timeout_s":2.0}}}]}"#
        );
    }
}
