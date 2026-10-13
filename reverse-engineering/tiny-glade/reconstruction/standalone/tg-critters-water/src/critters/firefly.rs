//! Firefly: update_firefly_goals (0x142316940, firefly.rs 46-112) and
//! set_firefly_velocities_towards_goals (0x142317040, 115-136). Evidence: critters-extra-analysis 4.
//! UNKNOWN(evidence: critters-extra-analysis 4): the direction formula is applied literally
//! (`sample * (1 + bias)`, then `z += 0.5`); with bias -1.0 (no goal) this degenerates to +z, which
//! suggests the bias really targets a single component. Kept literal and flagged.

use super::components::*;
use crate::env::Time;
use crate::rng::PerchanceRes;
use crate::sample::uniform_sample_sphere;
use bevy_ecs::prelude::*;
use glam::Vec3;

pub const WORLD_RADIUS: f32 = 65.0;
pub const GOAL_REACHED_DIST_SQ: f32 = 0.16;
/// Inertia bias: +0.5 with an active goal, -1.0 without (corrects the earlier draft's -0.5).
pub const BIAS_WITH_GOAL: f32 = 0.5;
pub const BIAS_NO_GOAL: f32 = -1.0;
/// Below this altitude `dir.y` is floored at 0.25 (0x3e800000).
pub const LOW_ALTITUDE: f32 = 1.0;
pub const MIN_CLIMB: f32 = 0.25;

#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Firefly {
    pub goal_timer: f32,
}

#[derive(Bundle, Clone, Debug)]
pub struct FireflyBundle {
    pub firefly: Firefly,
    pub body: CritterBundle,
}

impl FireflyBundle {
    pub fn new(pos: Vec3, locomotion: u32, retention: f32) -> Self {
        Self { firefly: Firefly::default(), body: CritterBundle::new(pos, locomotion, retention) }
    }
}

/// The firefly control switch: `*(res + 4) == 1`; anything else makes the goal system return early.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FireflyControl {
    pub active: bool,
}

impl Default for FireflyControl {
    fn default() -> Self {
        Self { active: true }
    }
}

pub fn update_firefly_goals(
    time: Res<Time>,
    control: Res<FireflyControl>,
    mut perchance: ResMut<PerchanceRes>,
    mut flies: Query<(&Transform, &mut Firefly, &mut CritterGoal)>,
) {
    if !control.active {
        return;
    }
    let dt = time.delta_secs();
    for (transform, mut fly, mut goal) in &mut flies {
        fly.goal_timer -= dt;
        let pos = transform.translation;
        let had_goal = goal.active;
        if !(fly.goal_timer <= 0.0 || !had_goal || pos.distance_squared(goal.pos) < GOAL_REACHED_DIST_SQ) {
            continue;
        }
        fly.goal_timer = perchance.uniform_f32() * 0.15 + 0.05;
        let bias = if had_goal { BIAS_WITH_GOAL } else { BIAS_NO_GOAL };
        let mut d = uniform_sample_sphere(perchance.uniform_f32(), perchance.uniform_f32()) * (1.0 + bias);
        d.z += 0.5;
        let proposed = pos + d.normalize_or_zero() * fly.goal_timer;
        let (r_new, r_old) = (proposed.x.hypot(proposed.z), pos.x.hypot(pos.z));
        *goal = if r_new < WORLD_RADIUS || r_new < r_old {
            CritterGoal::set(proposed)
        } else {
            CritterGoal { active: false, pos: goal.pos }
        };
    }
}

/// Query `(Transform, &Firefly, &CritterGoal, &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)`
/// (the doc prints a `Sheep` column; read as the `Firefly` component, see NOTES.md).
pub fn set_firefly_velocities_towards_goals(
    mut flies: Query<(&Transform, &Firefly, &CritterGoal, &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)>,
) {
    for (transform, _fly, goal, mut target, mut multiplier) in &mut flies {
        let pos = transform.translation;
        let mut dir = if goal.active { (goal.pos - pos).normalize_or_zero() } else { Vec3::ZERO };
        if pos.y <= LOW_ALTITUDE {
            dir.y = dir.y.max(MIN_CLIMB);
        }
        target.dir = dir;
        multiplier.0 = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{base_app, PERCHANCE_SEED};
    use bevy_app::{App, Update};
    use perchance::PerchanceContext;

    fn setup(dt: f32, seed: u128) -> App {
        let mut app = base_app(dt);
        app.init_resource::<FireflyControl>()
            .insert_resource(PerchanceRes::seeded(seed))
            .add_systems(Update, update_firefly_goals);
        app
    }

    fn fly(app: &mut App, pos: Vec3) -> Entity {
        app.world_mut().spawn(FireflyBundle::new(pos, 0, 0.5)).id()
    }

    fn goal(app: &App, e: Entity) -> CritterGoal {
        *app.world().get::<CritterGoal>(e).unwrap()
    }

    #[test]
    fn control_gate() {
        let mut app = setup(0.1, 1);
        app.insert_resource(FireflyControl { active: false });
        let e = fly(&mut app, Vec3::ZERO);
        app.update();
        assert!(!goal(&app, e).active);
        app.insert_resource(FireflyControl { active: true });
        app.update();
        assert!(goal(&app, e).active);
        let t = app.world().get::<Firefly>(e).unwrap().goal_timer;
        assert!((0.05..0.2).contains(&t));
    }

    #[test]
    fn timer_and_direction_follow_perchance_stream() {
        let mut app = setup(0.0, PERCHANCE_SEED);
        let e = fly(&mut app, Vec3::ZERO);
        app.update();
        let mut m = PerchanceContext::new(PERCHANCE_SEED);
        let timer = m.uniform_f32() * 0.15 + 0.05;
        let (u1, u2) = (m.uniform_f32(), m.uniform_f32());
        // no goal: bias -1.0 -> sample * 0, then z += 0.5 -> +z
        let _ = uniform_sample_sphere(u1, u2);
        assert_eq!(app.world().get::<Firefly>(e).unwrap().goal_timer, timer);
        let g = goal(&app, e);
        assert!((g.pos - Vec3::new(0.0, 0.0, timer)).length() < 1e-6, "degenerates to +z with bias -1.0: {:?}", g.pos);
    }

    #[test]
    fn with_goal_bias_adds_the_sample_component() {
        let mut xs = 0.0f32;
        for seed in 0..30u128 {
            let mut app = setup(0.0, seed);
            let e = fly(&mut app, Vec3::ZERO);
            *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(0.0, 0.0, 9.0));
            app.world_mut().get_mut::<Firefly>(e).unwrap().goal_timer = -1.0;
            app.update();
            let g = goal(&app, e);
            assert!(g.active);
            xs += g.pos.x.abs();
        }
        assert!(xs > 0.0, "x component present only with the +0.5 bias");
    }

    #[test]
    fn kept_goal_and_radius_rejection() {
        let mut app = setup(0.1, 1);
        let e = fly(&mut app, Vec3::ZERO);
        app.world_mut().get_mut::<Firefly>(e).unwrap().goal_timer = 5.0;
        *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(4.0, 0.0, 0.0));
        app.update();
        assert_eq!(goal(&app, e).pos, Vec3::new(4.0, 0.0, 0.0));
        // no goal at the rim: the +z candidate lies outside the radius and farther out -> rejected
        let mut far = setup(0.0, 1);
        let f = fly(&mut far, Vec3::new(0.0, 0.0, 65.0));
        far.update();
        assert!(!goal(&far, f).active);
    }

    #[test]
    fn velocities_floor_dir_y_and_multiplier() {
        let mut app = base_app(0.016);
        app.add_systems(Update, set_firefly_velocities_towards_goals);
        let low = fly(&mut app, Vec3::new(0.0, 0.5, 0.0));
        *app.world_mut().get_mut::<CritterGoal>(low).unwrap() = CritterGoal::set(Vec3::new(0.0, -10.0, 10.0));
        let high = fly(&mut app, Vec3::new(0.0, 5.0, 0.0));
        *app.world_mut().get_mut::<CritterGoal>(high).unwrap() = CritterGoal::set(Vec3::new(0.0, -10.0, 0.0));
        app.world_mut().get_mut::<CritterVelocityMultiplier>(low).unwrap().0 = 7.0;
        app.update();
        assert_eq!(app.world().get::<CritterVelocityTarget>(low).unwrap().dir.y, 0.25);
        assert_eq!(app.world().get::<CritterVelocityMultiplier>(low).unwrap().0, 1.0);
        assert_eq!(app.world().get::<CritterVelocityTarget>(high).unwrap().dir.y, -1.0);
    }
}
