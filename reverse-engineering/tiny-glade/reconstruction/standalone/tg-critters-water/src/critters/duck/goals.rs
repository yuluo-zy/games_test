//! Stage 3: update_duck_goals (0x142313b30, duck.rs 581-695).

use super::Duck;
use crate::critters::components::{CritterGoal, Transform};
use crate::env::{GladeShape, Time};
use crate::rng::PerchanceRes;
use crate::sample::{uniform_sample_disk, x0y};
use crate::water::WorldRaster;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Goal kept while `dist2(pos, goal) >= 0.16` (`__real_3e23d70b`).
pub const GOAL_REACHED_DIST_SQ: f32 = 0.16;
/// New goal = pos + disk sample * 0.8 when a goal was previously active.
pub const GOAL_SAMPLE_SCALE_ACTIVE: f32 = 0.8;
/// UNKNOWN(evidence: critters-behavior-analysis 1.3): the scale when no goal was active.
pub const GOAL_SAMPLE_SCALE_INACTIVE: f32 = 1.0;
/// Water level at the candidate must exceed this (`<= 0.5` -> invalid, @0xc96).
pub const GOAL_MIN_WATER: f32 = 0.5;

/// One frame of the goal state machine for every duck.
/// Timers: resting (`moving == false`): `t0 -= dt`, at <= 0 -> `(u1*3+1, u2*3+1)`, moving = true.
/// Moving: both timers `-= dt`; both <= 0 -> `t0 = u*0.5+0.5`, moving = false; otherwise keep the goal
/// while active and `dist2 >= 0.16`, else pick a new one.
///
/// Parameters follow the documented RCX/RDX/R8/R9 (Query, Res, Time, WaterRaster) plus the glade
/// config and the Perchance state.
/// UNKNOWN(evidence: critters-behavior-analysis 1.3): the second half written with the rest timer
/// (`(u*0.5+0.5, ...)`), the 4-sample clamp>=0.1 check (sample offsets/raster not recovered) and the
/// glade-vs-radius combination; `valid = inside_glade || r(new) <= r(pos)` is an interpretation.
pub fn update_duck_goals(
    time: Res<Time>,
    glade: Res<GladeShape>,
    water: Res<WorldRaster>,
    mut perchance: ResMut<PerchanceRes>,
    mut ducks: Query<(&mut Duck, &Transform, &mut CritterGoal)>,
) {
    let dt = time.delta_secs();
    for (mut duck, transform, mut goal) in &mut ducks {
        if !duck.moving {
            duck.timers[0] -= dt;
            if duck.timers[0] <= 0.0 {
                let (u1, u2) = (perchance.uniform_f32(), perchance.uniform_f32());
                duck.timers = [u1 * 3.0 + 1.0, u2 * 3.0 + 1.0];
                duck.moving = true;
                duck.e1 = false;
            }
        } else {
            duck.timers[0] -= dt;
            duck.timers[1] -= dt;
            if duck.timers[0] <= 0.0 && duck.timers[1] <= 0.0 {
                duck.timers[0] = perchance.uniform_f32() * 0.5 + 0.5;
                duck.moving = false;
            } else {
                let pos = transform.translation;
                if !(goal.active && pos.distance_squared(goal.pos) >= GOAL_REACHED_DIST_SQ) {
                    pick_goal(&glade, &water, &mut perchance, &mut duck, &mut goal, pos);
                }
            }
        }
        duck.e1 = false;
    }
}

fn pick_goal(
    glade: &GladeShape,
    water: &WorldRaster,
    perchance: &mut PerchanceRes,
    duck: &mut Duck,
    goal: &mut CritterGoal,
    pos: Vec3,
) {
    let was_active = goal.active;
    duck.e8 = 0.1;
    let (u1, u2) = (perchance.uniform_f32(), perchance.uniform_f32());
    let scale = if was_active { GOAL_SAMPLE_SCALE_ACTIVE } else { GOAL_SAMPLE_SCALE_INACTIVE };
    let new = pos + x0y(uniform_sample_disk(u1, u2)) * scale;
    let (r_new, r_old) = (new.x.hypot(new.z), pos.x.hypot(pos.z));
    let in_bounds = glade.is_within_glade(new.x, new.z) || r_new <= r_old;
    *goal = if in_bounds && water.sample_at_world_xz(new.x, new.z) > GOAL_MIN_WATER {
        CritterGoal::set(new)
    } else {
        CritterGoal { active: false, pos: goal.pos }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::duck::{DuckBundle, DuckConfig, DuckState};
    use crate::testing::{base_app, PERCHANCE_SEED};
    use bevy_app::{App, Update};
    use perchance::PerchanceContext;

    fn setup(dt: f32, wet: bool) -> App {
        let mut app = base_app(dt);
        app.add_systems(Update, update_duck_goals);
        if wet {
            app.world_mut().resource_mut::<WorldRaster>().composite.fill(255);
        }
        app
    }

    fn duck(app: &mut App, moving: bool, timers: [f32; 2]) -> Entity {
        let mut b = DuckBundle::new(Vec3::new(5.0, 0.0, 5.0), DuckState::FREE, &DuckConfig::default());
        b.duck.moving = moving;
        b.duck.timers = timers;
        app.world_mut().spawn(b).id()
    }

    fn duck_of(app: &App, e: Entity) -> Duck {
        app.world().get::<Duck>(e).unwrap().clone()
    }

    fn goal_of(app: &App, e: Entity) -> CritterGoal {
        *app.world().get::<CritterGoal>(e).unwrap()
    }

    #[test]
    fn rest_timer_expiry_starts_moving_with_1_to_4_timers_from_perchance() {
        let mut app = setup(0.1, true);
        let e = duck(&mut app, false, [0.05, 0.0]);
        app.update();
        let mut mirror = PerchanceContext::new(PERCHANCE_SEED);
        let (u1, u2) = (mirror.uniform_f32(), mirror.uniform_f32());
        let d = duck_of(&app, e);
        assert!(d.moving);
        assert_eq!(d.timers, [u1 * 3.0 + 1.0, u2 * 3.0 + 1.0]);
        assert!(d.timers.iter().all(|t| (1.0..4.0).contains(t)));
    }

    #[test]
    fn rest_timer_counts_down_without_rng() {
        let mut app = setup(0.25, true);
        let e = duck(&mut app, false, [1.0, 0.0]);
        app.update();
        let d = duck_of(&app, e);
        assert!(!d.moving);
        assert_eq!(d.timers[0], 0.75);
        let mut fresh = PerchanceContext::new(PERCHANCE_SEED);
        assert_eq!(app.world_mut().resource_mut::<PerchanceRes>().uniform_f32(), fresh.uniform_f32(), "no draw consumed");
    }

    #[test]
    fn moving_both_timers_expired_goes_to_rest_with_half_to_one_timer() {
        let mut app = setup(0.1, true);
        let e = duck(&mut app, true, [0.05, 0.05]);
        app.update();
        let u = PerchanceContext::new(PERCHANCE_SEED).uniform_f32();
        let d = duck_of(&app, e);
        assert!(!d.moving);
        assert_eq!(d.timers[0], u * 0.5 + 0.5);
    }

    #[test]
    fn moving_picks_goal_on_water_within_disk() {
        let mut app = setup(0.1, true);
        let e = duck(&mut app, true, [5.0, 5.0]);
        app.update();
        let g = goal_of(&app, e);
        assert!(g.active);
        assert!(g.pos.distance(Vec3::new(5.0, 0.0, 5.0)) <= GOAL_SAMPLE_SCALE_INACTIVE + 1e-5);
        assert_eq!(duck_of(&app, e).e8, 0.1);
    }

    #[test]
    fn active_far_goal_is_kept() {
        let mut app = setup(0.1, true);
        let e = duck(&mut app, true, [5.0, 5.0]);
        *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(9.0, 0.0, 9.0));
        app.update();
        assert_eq!(goal_of(&app, e).pos, Vec3::new(9.0, 0.0, 9.0));
    }

    #[test]
    fn reached_goal_is_replaced_with_scale_08() {
        let mut app = setup(0.1, true);
        let e = duck(&mut app, true, [5.0, 5.0]);
        *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(5.0, 0.0, 5.1));
        app.update();
        let g = goal_of(&app, e);
        assert!(g.active && g.pos != Vec3::new(5.0, 0.0, 5.1));
        assert!(g.pos.distance(Vec3::new(5.0, 0.0, 5.0)) <= 0.8 + 1e-5);
    }

    #[test]
    fn dry_candidate_makes_goal_inactive() {
        let mut app = setup(0.1, false);
        let e = duck(&mut app, true, [5.0, 5.0]);
        app.update();
        assert!(!goal_of(&app, e).active);
    }

    #[test]
    fn outside_glade_outward_candidates_are_rejected_inward_accepted() {
        let (mut accepted, mut rejected) = (0, 0);
        for seed in 0..40u128 {
            let mut app = setup(0.1, true);
            app.insert_resource(GladeShape { radius: 1.0 }).insert_resource(PerchanceRes::seeded(seed));
            let e = duck(&mut app, true, [5.0, 5.0]); // r(pos) = 7.07 > glade radius
            app.update();
            let g = goal_of(&app, e);
            if g.active {
                accepted += 1;
                assert!(g.pos.x.hypot(g.pos.z) <= 5.0f32.hypot(5.0) + 1e-5, "accepted only if not outward");
            } else {
                rejected += 1;
            }
        }
        assert!(accepted > 5 && rejected > 5, "accepted {accepted} rejected {rejected}");
    }
}
