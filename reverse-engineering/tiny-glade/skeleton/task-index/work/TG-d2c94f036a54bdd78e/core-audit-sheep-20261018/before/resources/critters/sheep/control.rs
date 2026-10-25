//! B control snapshots: native serializer 0x1416f9eb0/0x1416f9fd0,
//! capture iterator 0x1412f03f0, restore consumer 0x1416f24f0.
//!
//! 2026-10-17 component-binding correction (fresh decompilation): the original
//! query is `(Entity, &mut Transform, &mut Sheep, &mut CritterGoal)` and the
//! native field binding is:
//! - `position` ← `CritterGoal @ +0x00` (NOT `Transform.translation`);
//! - `goal` ← `Sheep @ +0x30` payload + `CritterGoal @ +0x0c` tag;
//! - `move_state` ← `utils Transform` translation slot @ +0x10 (12 B).
//! The old Rust triplet `(&Transform, &CritterGoal, &SheepMoveState)` bound
//! position←transform.translation / goal←CritterGoal / move_state←SheepMoveState,
//! a shifted binding (move_state actually lives in the translation slot, and
//! position in CritterGoal) — corrected here.
use super::{Sheep, SheepMoveState};
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

/// Caller passes rows in actual query order: `(Transform, Sheep, CritterGoal)`.
/// Native capture stores no entity ID and does not sort; per-field binding
/// (O): `position` ← `CritterGoal @ +0x00`, `goal` ← `Sheep @ +0x30` payload +
/// `CritterGoal @ +0x0c` tag, `move_state` ← `Transform.translation` @ +0x10.
pub fn capture_sheep_snapshot<'a>(
    rows: impl IntoIterator<Item = (&'a Transform, &'a Sheep, &'a CritterGoal)>,
) -> SheepControlSnapshot {
    SheepControlSnapshot {
        sheep: rows
            .into_iter()
            .map(|(transform, sheep, critter_goal)| SheepSnapshot {
                position: critter_goal.position,
                goal: (critter_goal.goal_tag != 0).then_some(sheep.goal),
                move_state: SheepMoveState::from_translation(transform.translation),
            })
            .collect(),
    }
}

/// Original restore zips query order with saved order, stops at the shorter
/// input and overwrites exactly these three fields. It neither creates nor
/// removes sheep, and preserves transform rotation/scale and other sheep state.
/// Per-field binding (O): `Transform.translation` ← `move_state`,
/// `Sheep @ +0x30` ← `goal` payload, `CritterGoal` ← `{position, goal_tag}`.
pub fn apply_sheep_snapshot<'a>(
    snapshot: &SheepControlSnapshot,
    rows: impl IntoIterator<
        Item = (
            &'a mut Transform,
            &'a mut Sheep,
            &'a mut CritterGoal,
        ),
    >,
) {
    for ((transform, sheep, critter_goal), saved) in rows.into_iter().zip(&snapshot.sheep) {
        transform.translation = SheepMoveState::to_translation(saved.move_state);
        sheep.goal = saved.goal.unwrap_or_default();
        critter_goal.position = saved.position;
        critter_goal.goal_tag = saved.goal.is_some() as u32;
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
    fn capture_and_apply_use_query_order_and_preserve_other_fields() {
        let original_sheep = Sheep {
            goal: Vec3::new(4., 5., 6.),
            being_petted_timer: 9.0,
        };
        let original_goal = CritterGoal {
            position: Vec3::new(7., 8., 9.),
            goal_tag: 1,
        };
        let original_move = SheepMoveState::Walking {
            timeout_s: 4.,
            goal_change_timeout: 5.,
            force_reach_goal: true,
        };
        // Capture: position←CritterGoal@0, goal←Sheep@+0x30(+tag),
        // move_state←Transform.translation (O binding).
        let snapshot = capture_sheep_snapshot([(
            &Transform::from_translation(SheepMoveState::to_translation(original_move)),
            &original_sheep,
            &original_goal,
        )]);
        let row = &snapshot.sheep[0];
        assert_eq!(row.position, original_goal.position);
        assert_eq!(row.goal, Some(original_sheep.goal));
        assert_eq!(row.move_state, original_move);

        let mut transforms = [
            Transform::from_scale(Vec3::splat(3.)),
            Transform::identity(),
        ];
        let mut sheeps = [
            Sheep {
                goal: Vec3::ZERO,
                being_petted_timer: 0.,
            },
            Sheep {
                goal: Vec3::ZERO,
                being_petted_timer: 0.,
            },
        ];
        let mut goals = [
            CritterGoal {
                position: Vec3::ZERO,
                goal_tag: 0,
            },
            CritterGoal {
                position: Vec3::ZERO,
                goal_tag: 0,
            },
        ];
        apply_sheep_snapshot(
            &snapshot,
            transforms
                .iter_mut()
                .zip(sheeps.iter_mut())
                .zip(goals.iter_mut())
                .map(|((a, b), c)| (a, b, c)),
        );
        // move_state goes to Transform.translation (not position).
        assert_eq!(
            transforms[0].translation,
            SheepMoveState::to_translation(original_move)
        );
        assert_eq!(transforms[0].scale, Vec3::splat(3.), "rotation/scale preserved");
        assert_eq!(sheeps[0].goal, original_sheep.goal);
        assert_eq!(sheeps[0].being_petted_timer, 0., "other sheep state preserved");
        assert_eq!(goals[0].position, original_goal.position);
        assert_eq!(goals[0].goal_tag, 1);
        // Second row (beyond the snapshot) is untouched.
        assert_eq!(transforms[1], Transform::identity());
        assert_eq!(sheeps[1].goal, Vec3::ZERO);
        assert_eq!(goals[1].goal_tag, 0);
    }

    #[test]
    fn sheep_section_is_option_json_none_to_null() {
        assert_eq!(sheep_section_json(None), "null");
        let snapshot = capture_sheep_snapshot([(
            &Transform::from_translation(SheepMoveState::to_translation(
                SheepMoveState::Idle { timeout_s: 2. },
            )),
            &Sheep {
                goal: Vec3::ZERO,
                being_petted_timer: 0.,
            },
            &CritterGoal {
                position: Vec3::new(1., 2., 3.),
                goal_tag: 0,
            },
        )]);
        let json = sheep_section_json(Some(&snapshot));
        assert_eq!(
            json,
            r#"{"sheep":[{"position":[1.0,2.0,3.0],"goal":null,"move_state":{"Idle":{"timeout_s":2.0}}}]}"#
        );
    }
}
