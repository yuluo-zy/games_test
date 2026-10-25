//! Butterfly: update_butterfly_goals (0x141d86060, 256-331), set_butterfly_velocities_towards_goals
//! (0x141d867b0, 334-378), the distance gate / dwell reset of update_butterfly_states (0x141d84d40).
//! Evidence: critters-extra-analysis 3.
//! UNKNOWN(evidence: critters-extra-analysis 3/7): the Butterfly state enum and both jump tables
//! (0x142f68008 / 0x142f68018), `multiplier += 1.0` semantics, audio. State transitions are
//! therefore NOT modelled; `state` is an opaque u32 except `== 1` selecting the vertical bias pair.

use super::components::*;
use crate::env::{CursorTerrainRaycast, MainCamera, Time};
use crate::rng::PerchanceRes;
use crate::sample::uniform_sample_sphere;
use bevy_ecs::prelude::*;
use glam::Vec3;

pub const SPEED_SCALE: f32 = 1.77; // 0x3fe147ae
pub const MAX_ALTITUDE: f32 = 8.0;
pub const WORLD_RADIUS: f32 = 65.0;
pub const GOAL_REACHED_DIST_SQ: f32 = 0.16;
/// Vertical bias `(bias, scale)`: state == 1 -> (-0.2835, 0.5), otherwise (0.8529, 1.0).
/// UNKNOWN(evidence: critters-extra-analysis 3): which component the pair modifies; applied as
/// `y = sample.y * scale + bias`.
pub const BIAS_STATE1: (f32, f32) = (-0.2835, 0.5);
pub const BIAS_OTHER: (f32, f32) = (0.8529, 1.0);

#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Butterfly {
    pub state: u32,
    /// f32 [2]
    pub goal_timer: f32,
    /// f32 [3]
    pub dwell: f32,
    /// f32 [4]
    pub phase: f32,
    /// f32 [5]
    pub phase_ref: f32,
}

#[derive(Bundle, Clone, Debug)]
pub struct ButterflyBundle {
    pub butterfly: Butterfly,
    pub body: CritterBundle,
}

impl ButterflyBundle {
    pub fn new(pos: Vec3, locomotion: u32, retention: f32) -> Self {
        Self { butterfly: Butterfly::default(), body: CritterBundle::new(pos, locomotion, retention) }
    }
}

/// New goal selection (goal timer expiry / inactive goal / goal reached).
/// Query `(Transform, &mut Butterfly, &mut CritterGoal)`; perchance draws per refresh: timer, then the
/// two sphere uniforms.
pub fn update_butterfly_goals(
    time: Res<Time>,
    mut perchance: ResMut<PerchanceRes>,
    mut flies: Query<(&Transform, &mut Butterfly, &mut CritterGoal)>,
) {
    let dt = time.delta_secs();
    for (transform, mut fly, mut goal) in &mut flies {
        fly.goal_timer -= dt;
        let pos = transform.translation;
        let refresh = fly.goal_timer <= 0.0 || !goal.active || pos.distance_squared(goal.pos) < GOAL_REACHED_DIST_SQ;
        if !refresh {
            continue;
        }
        fly.goal_timer = perchance.uniform_f32() * 0.5 + 0.1;
        let s = uniform_sample_sphere(perchance.uniform_f32(), perchance.uniform_f32());
        let (bias, scale) = if fly.state == 1 { BIAS_STATE1 } else { BIAS_OTHER };
        let dir = Vec3::new(s.x, s.y * scale + bias, s.z).normalize_or_zero();
        let mut proposed = pos + dir * (fly.goal_timer * SPEED_SCALE);
        proposed.y = proposed.y.min(MAX_ALTITUDE);
        let r_new = proposed.x.hypot(proposed.z);
        let r_old = pos.x.hypot(pos.z);
        *goal = if r_new < WORLD_RADIUS || r_new < r_old {
            CritterGoal::set(proposed)
        } else {
            CritterGoal { active: false, pos: goal.pos }
        };
    }
}

/// `dir = normalize(goal - pos) * 1.77` written to the velocity target (zero without a goal).
/// Query `(Transform, &Butterfly, &CritterGoal, &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)`
/// (the doc lists a `Sheep` column in this query; it is read as the `Butterfly` component, see NOTES.md).
/// `multiplier += 1.0` is UNKNOWN(evidence: critters-extra-analysis 3) and not applied.
pub fn set_butterfly_velocities_towards_goals(
    mut flies: Query<(&Transform, &Butterfly, &CritterGoal, &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)>,
) {
    for (transform, _fly, goal, mut target, _multiplier) in &mut flies {
        target.dir = if goal.active {
            (goal.pos - transform.translation).normalize_or_zero() * SPEED_SCALE
        } else {
            Vec3::ZERO
        };
    }
}

/// Distance gate of update_butterfly_states: camera mode 3 -> distance to eye > 4.0 (0x40400000),
/// otherwise distance to cursor > 8.0 (0x40800000) before the state machine may advance.
pub fn state_gate_distance(camera_mode3: bool) -> f32 {
    if camera_mode3 { 4.0 } else { 8.0 }
}

/// dwell reset `u * 0.3 + 0.2`.
pub fn reset_dwell(perchance: &mut PerchanceRes) -> f32 {
    perchance.uniform_f32() * 0.3 + 0.2
}

/// update_butterfly_states, only the documented parts: the distance gate and the dwell reset
/// (`dwell <= 0 -> u * 0.3 + 0.2`). Query `(Transform, &Velocity, &mut Butterfly, &CritterGoal)`
/// (ButterflyAudio and the Sheep-avoidance list are not modelled).
/// UNKNOWN(evidence: critters-extra-analysis 3/7): the state machine jump table, so `state`,
/// `phase` and `phase_ref` are never written, and nothing decrements `dwell`.
pub fn update_butterfly_states(
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    mut perchance: ResMut<PerchanceRes>,
    mut flies: Query<(&Transform, &Velocity, &mut Butterfly, &CritterGoal)>,
) {
    let gate = state_gate_distance(camera.is_mode3());
    let reference = if camera.is_mode3() { Some(camera.eye) } else { cursor.0 };
    for (transform, _velocity, mut fly, _goal) in &mut flies {
        let far = reference.is_none_or(|r| transform.translation.distance(r) > gate);
        if far && fly.dwell <= 0.0 {
            fly.dwell = reset_dwell(&mut perchance);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{base_app, PERCHANCE_SEED};
    use bevy_app::{App, Update};
    use perchance::PerchanceContext;

    fn setup(dt: f32) -> App {
        let mut app = base_app(dt);
        app.add_systems(Update, update_butterfly_goals);
        app
    }

    fn fly(app: &mut App, pos: Vec3) -> Entity {
        app.world_mut().spawn(ButterflyBundle::new(pos, 0, 0.5)).id()
    }

    fn goal(app: &App, e: Entity) -> CritterGoal {
        *app.world().get::<CritterGoal>(e).unwrap()
    }

    #[test]
    fn inactive_goal_gets_new_goal_with_timer_and_speed_scaled_step() {
        let mut app = setup(0.016);
        let e = fly(&mut app, Vec3::new(1.0, 2.0, 1.0));
        app.update();
        let mut m = PerchanceContext::new(PERCHANCE_SEED);
        let timer = m.uniform_f32() * 0.5 + 0.1;
        let b = app.world().get::<Butterfly>(e).unwrap();
        assert_eq!(b.goal_timer, timer);
        assert!((0.1..0.6).contains(&b.goal_timer));
        let g = goal(&app, e);
        let step = g.pos.distance(Vec3::new(1.0, 2.0, 1.0));
        if g.active && g.pos.y < MAX_ALTITUDE {
            assert!((step - timer * 1.77).abs() < 1e-4, "unclamped step is timer * 1.77");
        }
        assert!(g.pos.y <= 8.0 + 1e-5);
    }

    #[test]
    fn altitude_clamped_for_many_seeds() {
        for seed in 0..30u128 {
            let mut app = setup(0.0);
            app.insert_resource(PerchanceRes::seeded(seed));
            let e = fly(&mut app, Vec3::new(0.0, 7.9, 0.0));
            app.update();
            assert!(goal(&app, e).pos.y <= 8.0 + 1e-5);
        }
    }

    #[test]
    fn state1_biases_down_other_states_up() {
        let (mut down, mut up) = (0.0f32, 0.0f32);
        for seed in 0..40u128 {
            for (state, acc) in [(1u32, &mut down), (0, &mut up)] {
                let mut app = setup(0.0);
                app.insert_resource(PerchanceRes::seeded(seed));
                let e = fly(&mut app, Vec3::new(0.0, 4.0, 0.0));
                app.world_mut().get_mut::<Butterfly>(e).unwrap().state = state;
                app.update();
                *acc += goal(&app, e).pos.y - 4.0;
            }
        }
        assert!(down < 0.0 && up > 0.0, "mean dy state1 {down}, others {up}");
    }

    #[test]
    fn active_far_goal_and_running_timer_keep_goal() {
        let mut app = setup(0.1);
        let e = fly(&mut app, Vec3::ZERO);
        app.world_mut().get_mut::<Butterfly>(e).unwrap().goal_timer = 5.0;
        *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(3.0, 1.0, 0.0));
        app.update();
        assert_eq!(goal(&app, e).pos, Vec3::new(3.0, 1.0, 0.0));
        assert!((app.world().get::<Butterfly>(e).unwrap().goal_timer - 4.9).abs() < 1e-6);
    }

    #[test]
    fn goal_outside_world_radius_rejected_unless_moving_inward() {
        let (mut out_rejected, mut out_accepted, mut in_accepted) = (0, 0, 0);
        for seed in 0..60u128 {
            let mut a = setup(0.0);
            a.insert_resource(PerchanceRes::seeded(seed));
            let e = fly(&mut a, Vec3::new(64.99, 1.0, 0.0));
            a.update();
            let g = goal(&a, e);
            if g.active {
                out_accepted += 1;
                assert!(g.pos.x.hypot(g.pos.z) < 64.99 || g.pos.x.hypot(g.pos.z) < 65.0);
            } else {
                out_rejected += 1;
            }
            let mut b = setup(0.0);
            b.insert_resource(PerchanceRes::seeded(seed));
            let e = fly(&mut b, Vec3::new(70.0, 1.0, 0.0));
            b.update();
            let g = goal(&b, e);
            if g.active {
                in_accepted += 1;
                assert!(g.pos.x.hypot(g.pos.z) < 70.0, "outside the radius only inward goals pass");
            }
        }
        assert!(out_rejected > 0 && out_accepted > 0, "{out_rejected}/{out_accepted}");
        assert!(in_accepted > 0 && in_accepted < 60);
    }

    #[test]
    fn velocity_towards_goal_scaled() {
        let mut app = base_app(0.016);
        app.add_systems(Update, set_butterfly_velocities_towards_goals);
        let e = fly(&mut app, Vec3::ZERO);
        *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(0.0, 0.0, 10.0));
        app.update();
        let dir = |app: &App| app.world().get::<CritterVelocityTarget>(e).unwrap().dir;
        assert!((dir(&app) - Vec3::new(0.0, 0.0, 1.77)).length() < 1e-6);
        app.world_mut().get_mut::<CritterGoal>(e).unwrap().active = false;
        app.update();
        assert_eq!(dir(&app), Vec3::ZERO);
    }

    #[test]
    fn gate_distance_and_dwell_reset_system() {
        assert_eq!(state_gate_distance(true), 4.0);
        assert_eq!(state_gate_distance(false), 8.0);
        let mut m = PerchanceContext::new(PERCHANCE_SEED);
        let expect = m.uniform_f32() * 0.3 + 0.2;
        assert_eq!(reset_dwell(&mut PerchanceRes::seeded(PERCHANCE_SEED)), expect);

        let mut app = base_app(0.016);
        app.add_systems(Update, update_butterfly_states);
        app.insert_resource(CursorTerrainRaycast(Some(Vec3::ZERO)));
        let far = fly(&mut app, Vec3::new(9.0, 0.0, 0.0));
        let near = fly(&mut app, Vec3::new(7.0, 0.0, 0.0));
        app.update();
        assert_eq!(app.world().get::<Butterfly>(far).unwrap().dwell, expect);
        assert_eq!(app.world().get::<Butterfly>(near).unwrap().dwell, 0.0, "inside the 8.0 cursor gate");
        app.insert_resource(MainCamera { mode: 3, eye: Vec3::ZERO, forward: Vec3::Z });
        app.world_mut().get_mut::<Butterfly>(near).unwrap().dwell = 0.0;
        app.update();
        assert!(app.world().get::<Butterfly>(near).unwrap().dwell > 0.0, "7.0 > 4.0 eye gate");
        assert!((0.2..0.5).contains(&app.world().get::<Butterfly>(near).unwrap().dwell));
    }
}
