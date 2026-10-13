//! Stage 1: plan_duck_trajectories (0x14230ff00, duck.rs 89-366) and the `TrajectoryPlan` slot.
//!
//! The evidence ties plan spawning to developer-hotkey branches (`developer_hotkeys_enabled`,
//! KeyCode 0x1f / 0x27, `ButtonInput<KeyCode>` from bevy_input). UNKNOWN(evidence:
//! critters-behavior-analysis 6): the KeyCode meaning and `_DEV_OVERRIDE_IS_DEMO_MODE` in release
//! builds; the dev gate is NOT modelled (plans execute whenever present).
//! UNKNOWN(evidence: critters-extra-analysis 6): critters-extra says spawned ducks get progress=1.0
//! and state 3 (reverse -> despawn at the start); that is followed literally.

use super::{DuckBundle, DuckConfig, DuckState};
use crate::rng::PerchanceRes;
use crate::sample::{golden_spiral, uniform_sample_circle, x0y};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// `TrajectoryPlan` (0x30-byte slot): points (+0x08/+0x10), random circle offset (+0x18 x2),
/// bookkeeping (+0x20/+0x24, `+0x24 == 0` = not yet executed), tail u32 (+0x28).
#[derive(Clone, Debug, PartialEq)]
pub struct TrajectoryPlan {
    pub points: Vec<Vec3>,
    pub random_offset: [f32; 2],
    pub executed: bool,
    pub tail: u32,
}

impl TrajectoryPlan {
    pub fn new(points: Vec<Vec3>, random_offset: [f32; 2]) -> Self {
        Self { points, random_offset, executed: false, tail: 0 }
    }
}

/// Resource-side `Vec<TrajectoryPlan>` (cap@+0x10, data@+0x18, len@+0x20).
#[derive(Resource, Clone, Debug, Default)]
pub struct TrajectoryPlans(pub Vec<TrajectoryPlan>);

/// Build a 3-point plan around `centre`: `golden_spiral` offsets (x0y) + a `uniform_sample_circle`
/// random offset (hotkey-A path / spawn_ducks path); 3 + 1 `perchance` uniforms.
/// UNKNOWN(evidence: critters-behavior-analysis 1.1): golden_spiral arguments and offset radius.
pub fn plan_around(centre: Vec3, spread: f32, perchance: &mut PerchanceRes) -> TrajectoryPlan {
    let phase = perchance.uniform_f32();
    let points = (0..3).map(|i| centre + x0y(golden_spiral(i, 3, phase)) * spread).collect();
    let o = uniform_sample_circle(perchance.uniform_f32());
    TrajectoryPlan::new(points, [o.x, o.y])
}

/// Execute pending plans: spawn a duck per plan (needs >= 2 points) at the END of its trajectory with
/// progress 1.0 and state 3, mark executed, then `retain` unexecuted plans.
pub fn plan_duck_trajectories(mut commands: Commands, mut plans: ResMut<TrajectoryPlans>, cfg: Res<DuckConfig>) {
    for plan in plans.0.iter_mut().filter(|p| !p.executed) {
        let Some(&end) = plan.points.last().filter(|_| plan.points.len() >= 2) else {
            continue;
        };
        let mut bundle = DuckBundle::new(end, DuckState::TRAJ_REVERSE, &cfg);
        bundle.duck.trajectory = plan.points.clone();
        bundle.duck.progress = 1.0;
        commands.spawn(bundle);
        plan.executed = true;
    }
    plans.0.retain(|p| !p.executed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::components::Transform;
    use crate::critters::duck::Duck;
    use crate::testing::base_app;
    use bevy_app::Update;

    #[test]
    fn plan_has_three_points_near_centre() {
        let mut r = PerchanceRes::seeded(5);
        let p = plan_around(Vec3::new(10.0, 0.0, 10.0), 3.0, &mut r);
        assert_eq!(p.points.len(), 3);
        assert!(p.points.iter().all(|q| q.distance(Vec3::new(10.0, 0.0, 10.0)) <= 3.0 + 1e-4));
        assert!(!p.executed);
        assert!((Vec3::new(p.random_offset[0], p.random_offset[1], 0.0).length() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn plan_consumes_exactly_four_perchance_uniforms() {
        let mut a = PerchanceRes::seeded(5);
        plan_around(Vec3::ZERO, 1.0, &mut a);
        let mut b = PerchanceRes::seeded(5);
        for _ in 0..2 {
            b.uniform_f32();
        }
        assert_eq!(a.uniform_f32(), b.uniform_f32());
    }

    #[test]
    fn plans_spawn_reverse_ducks_and_are_retained_out() {
        let mut app = base_app(0.016);
        app.init_resource::<DuckConfig>().init_resource::<TrajectoryPlans>().add_systems(Update, plan_duck_trajectories);
        let good = plan_around(Vec3::ZERO, 2.0, &mut PerchanceRes::seeded(1));
        app.world_mut().resource_mut::<TrajectoryPlans>().0 = vec![good, TrajectoryPlan::new(vec![Vec3::ZERO], [0.0; 2])];
        app.update();
        assert_eq!(app.world().resource::<TrajectoryPlans>().0.len(), 1, "degenerate plan stays, executed plan removed");
        let mut q = app.world_mut().query::<(&Duck, &DuckState, &Transform)>();
        let ducks: Vec<_> = q.iter(app.world()).collect();
        assert_eq!(ducks.len(), 1);
        let (duck, state, tf) = ducks[0];
        assert_eq!(*state, DuckState::TRAJ_REVERSE);
        assert_eq!(duck.progress, 1.0);
        assert_eq!(tf.translation, *duck.trajectory.last().unwrap());
        app.update();
        assert_eq!(app.world_mut().query::<&Duck>().iter(app.world()).count(), 1, "no second spawn");
    }
}
