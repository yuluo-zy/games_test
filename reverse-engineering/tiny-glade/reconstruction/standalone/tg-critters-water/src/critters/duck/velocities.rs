//! Stage 4: set_duck_velocities_towards_goals (0x142314470, duck.rs 698-728).
//! Only `DuckState == 2`. `dir = normalize(goal - pos)` when the goal is active and `Duck+0xe0 == 1`
//! (moving), else zero; frozen ducks (`+0xfc == 1`) get zero.
//! UNKNOWN(evidence: critters-behavior-analysis 1.4): the CritterVelocityMultiplier write and whether
//! the normalisation is 3D or planar (3D used).

use super::{Duck, DuckState};
use crate::critters::components::*;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Query `(Transform, &Duck, &DuckState, &CritterGoal, &mut CritterVelocityTarget,
/// &mut CritterVelocityMultiplier)` (init@0x14230ecf0).
#[allow(clippy::type_complexity)]
pub fn set_duck_velocities_towards_goals(
    mut ducks: Query<(
        &Transform,
        &Duck,
        &DuckState,
        &CritterGoal,
        &mut CritterVelocityTarget,
        &mut CritterVelocityMultiplier,
    )>,
) {
    for (transform, duck, state, goal, mut target, _multiplier) in &mut ducks {
        if *state != DuckState::FREE {
            continue;
        }
        let dir = if goal.active && duck.moving {
            (goal.pos - transform.translation).normalize_or_zero()
        } else {
            Vec3::ZERO
        };
        target.dir = if duck.frozen { Vec3::ZERO } else { dir };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::duck::{DuckBundle, DuckConfig};
    use crate::testing::base_app;
    use bevy_app::{App, Update};

    fn duck(app: &mut App, state: DuckState) -> Entity {
        let mut b = DuckBundle::new(Vec3::ZERO, state, &DuckConfig::default());
        b.body.goal = CritterGoal::set(Vec3::new(3.0, 0.0, 4.0));
        b.duck.moving = true;
        app.world_mut().spawn(b).id()
    }

    fn dir(app: &App, e: Entity) -> Vec3 {
        app.world().get::<CritterVelocityTarget>(e).unwrap().dir
    }

    #[test]
    fn free_moving_duck_heads_to_goal() {
        let mut app = base_app(0.016);
        app.add_systems(Update, set_duck_velocities_towards_goals);
        let e = duck(&mut app, DuckState::FREE);
        app.update();
        assert!((dir(&app, e) - Vec3::new(0.6, 0.0, 0.8)).length() < 1e-6);
    }

    #[test]
    fn resting_inactive_frozen_and_wrong_state_get_zero_or_untouched() {
        let mut app = base_app(0.016);
        app.add_systems(Update, set_duck_velocities_towards_goals);
        let rest = duck(&mut app, DuckState::FREE);
        app.world_mut().get_mut::<Duck>(rest).unwrap().moving = false;
        let inactive = duck(&mut app, DuckState::FREE);
        app.world_mut().get_mut::<CritterGoal>(inactive).unwrap().active = false;
        let frozen = duck(&mut app, DuckState::FREE);
        app.world_mut().get_mut::<Duck>(frozen).unwrap().frozen = true;
        let other = duck(&mut app, DuckState::TRAJ_FORWARD);
        for e in [rest, inactive, frozen, other] {
            app.world_mut().get_mut::<CritterVelocityTarget>(e).unwrap().dir = Vec3::Y;
        }
        app.update();
        for e in [rest, inactive, frozen] {
            assert_eq!(dir(&app, e), Vec3::ZERO);
        }
        assert_eq!(dir(&app, other), Vec3::Y, "state != 2 untouched");
    }
}
