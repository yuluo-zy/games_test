//! update_sheep_goals (0x142315050, sheep_behavior.rs 26-221). Evidence: critters-behavior-analysis 4.
//! set_sheep_velocities / update_sheep_audio / sheep_unstuck / sheep_floatie were not read:
//! UNKNOWN(evidence: critters-behavior-analysis 6).

use super::components::*;
use crate::env::{CursorTerrainRaycast, MainCamera, Time};
use crate::rng::PerchanceRes;
use crate::sample::{uniform_sample_disk, x0y};
use crate::water::WorldRaster;
use bevy_ecs::prelude::*;
use glam::Vec3;

pub const GOAL_REACHED_DIST_SQ: f32 = 0.16;
/// follow target with camera mode 3: `eye + forward * 2`.
pub const CAMERA_FOLLOW_DISTANCE: f32 = 2.0;
/// While close to the target the follow timer drains 3 extra dt.
pub const FOLLOW_CLOSE_DRAIN: f32 = 3.0;
pub const SHEEP_FOLLOW: u32 = 1;

/// `Sheep` (0x68): +8 behaviour state, +0xc timer, +0x30 byte, +0x34 two f32 timers.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Sheep {
    pub state: u32,
    pub follow_timer: f32,
    pub wander_enabled: bool,
    pub timers: [f32; 2],
}

impl Default for Sheep {
    fn default() -> Self {
        Self { state: 0, follow_timer: 0.0, wander_enabled: true, timers: [1.0, 1.0] }
    }
}

/// Sheep terrain state (8 B: state u32 @+0, f32 @+4). See sheep_terrain.rs.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepTerrain {
    pub state: u32,
    pub aux: f32,
}

#[derive(Bundle, Clone, Debug)]
pub struct SheepBundle {
    pub sheep: Sheep,
    pub terrain: SheepTerrain,
    pub body: CritterBundle,
}

impl SheepBundle {
    pub fn new(pos: Vec3, locomotion: u32, retention: f32) -> Self {
        Self { sheep: Sheep::default(), terrain: SheepTerrain::default(), body: CritterBundle::new(pos, locomotion, retention) }
    }
}

/// UNKNOWN(evidence: critters-behavior-analysis 4): `fVar13` (the "close to target" distance) and the
/// wander disk scale are not recovered; both are configuration.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct SheepGoalConfig {
    pub follow_close_distance: f32,
    pub wander_radius: f32,
}

impl Default for SheepGoalConfig {
    fn default() -> Self {
        Self { follow_close_distance: 1.0, wander_radius: 3.0 }
    }
}

/// Interpretation notes: follow state pins the goal to the target and skips wandering; when
/// `timer < 0` the state returns to 0; the shared refresh only runs when all wander timers are > 0
/// (expiry branch UNKNOWN), and a water candidate (`level > 0.5`) is rejected keeping the old goal.
/// Camera mode 3 -> `eye + forward * 2`, otherwise the cursor terrain raycast.
pub fn update_sheep_goals(
    time: Res<Time>,
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    water: Res<WorldRaster>,
    cfg: Res<SheepGoalConfig>,
    mut perchance: ResMut<PerchanceRes>,
    mut sheep: Query<(&Transform, &mut Sheep, &mut CritterGoal)>,
) {
    let dt = time.delta_secs();
    let follow_target =
        if camera.is_mode3() { Some(camera.eye + camera.forward * CAMERA_FOLLOW_DISTANCE) } else { cursor.0 };
    for (transform, mut s, mut goal) in &mut sheep {
        let pos = transform.translation;
        if s.state == SHEEP_FOLLOW {
            if let Some(t) = follow_target {
                *goal = CritterGoal::set(t);
                s.follow_timer -= dt;
                if pos.distance(t) <= cfg.follow_close_distance {
                    s.follow_timer -= FOLLOW_CLOSE_DRAIN * dt;
                }
                if s.follow_timer < 0.0 {
                    s.state = 0;
                    let _ = perchance.uniform_f32(); // @0x1874: purpose UNKNOWN; draw kept for stream alignment
                }
            }
            continue;
        }
        if !s.wander_enabled {
            continue;
        }
        s.timers[0] -= dt;
        s.timers[1] -= dt;
        if !(s.timers[0] > 0.0 && s.timers[1] > 0.0) {
            continue; // UNKNOWN(evidence: critters-behavior-analysis 4): timer-expiry branch
        }
        if goal.active && pos.distance_squared(goal.pos) >= GOAL_REACHED_DIST_SQ {
            continue;
        }
        let (u1, u2) = (perchance.uniform_f32(), perchance.uniform_f32());
        let cand = pos + x0y(uniform_sample_disk(u1, u2)) * cfg.wander_radius;
        if water.sample_at_world_xz(cand.x, cand.z) <= 0.5 {
            *goal = CritterGoal::set(cand);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{base_app, PERCHANCE_SEED};
    use bevy_app::{App, Update};
    use perchance::PerchanceContext;

    fn setup() -> App {
        let mut app = base_app(0.1);
        app.init_resource::<SheepGoalConfig>().add_systems(Update, update_sheep_goals);
        app
    }

    fn sheep(app: &mut App) -> Entity {
        app.world_mut().spawn(SheepBundle::new(Vec3::ZERO, 0, 0.5)).id()
    }

    fn goal(app: &App, e: Entity) -> CritterGoal {
        *app.world().get::<CritterGoal>(e).unwrap()
    }

    fn follow(app: &mut App, e: Entity, timer: f32) {
        let mut s = app.world_mut().get_mut::<Sheep>(e).unwrap();
        s.state = SHEEP_FOLLOW;
        s.follow_timer = timer;
    }

    #[test]
    fn follow_targets_cursor_and_eye_forward() {
        let mut app = setup();
        let e = sheep(&mut app);
        follow(&mut app, e, 10.0);
        app.insert_resource(CursorTerrainRaycast(Some(Vec3::new(9.0, 0.0, 0.0))));
        app.update();
        assert_eq!(goal(&app, e), CritterGoal::set(Vec3::new(9.0, 0.0, 0.0)));
        app.insert_resource(MainCamera { mode: 3, eye: Vec3::new(0.0, 5.0, 0.0), forward: Vec3::Z });
        app.update();
        assert_eq!(goal(&app, e).pos, Vec3::new(0.0, 5.0, 2.0));
    }

    #[test]
    fn follow_timer_drains_faster_when_close_and_ends_follow_with_one_draw() {
        let mut app = setup();
        let e = sheep(&mut app);
        follow(&mut app, e, 0.3);
        app.insert_resource(CursorTerrainRaycast(Some(Vec3::new(0.5, 0.0, 0.0))));
        app.update();
        let s = app.world().get::<Sheep>(e).unwrap();
        assert!((s.follow_timer - (0.3 - 0.1 - 0.3)).abs() < 1e-6);
        assert_eq!(s.state, 0);
        let mut mirror = PerchanceContext::new(PERCHANCE_SEED);
        mirror.uniform_f32();
        assert_eq!(app.world_mut().resource_mut::<PerchanceRes>().uniform_f32(), mirror.uniform_f32());
    }

    #[test]
    fn follow_without_target_does_nothing() {
        let mut app = setup();
        let e = sheep(&mut app);
        follow(&mut app, e, 5.0);
        app.update();
        assert!(!goal(&app, e).active);
        assert_eq!(app.world().get::<Sheep>(e).unwrap().follow_timer, 5.0);
    }

    #[test]
    fn wander_picks_dry_goal_within_radius() {
        let mut app = setup();
        let e = sheep(&mut app);
        app.update();
        let g = goal(&app, e);
        assert!(g.active);
        assert!(g.pos.length() <= 3.0 + 1e-5);
        let mut mirror = PerchanceContext::new(PERCHANCE_SEED);
        let (u1, u2) = (mirror.uniform_f32(), mirror.uniform_f32());
        assert_eq!(g.pos, x0y(uniform_sample_disk(u1, u2)) * 3.0);
    }

    #[test]
    fn wander_rejects_water_keeps_goal_and_far_goal_kept() {
        let mut wet = setup();
        wet.world_mut().resource_mut::<WorldRaster>().composite.fill(255);
        let e = sheep(&mut wet);
        wet.update();
        assert!(!goal(&wet, e).active);
        let mut dry = setup();
        let e = sheep(&mut dry);
        *dry.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(5.0, 0.0, 0.0));
        dry.update();
        assert_eq!(goal(&dry, e).pos, Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn expired_timers_or_disabled_wander_skip() {
        let mut app = setup();
        let e = sheep(&mut app);
        app.world_mut().get_mut::<Sheep>(e).unwrap().timers = [0.05, 5.0];
        app.update();
        assert!(!goal(&app, e).active);
        {
            let mut s = app.world_mut().get_mut::<Sheep>(e).unwrap();
            s.timers = [5.0, 5.0];
            s.wander_enabled = false;
        }
        app.update();
        assert!(!goal(&app, e).active);
    }
}
