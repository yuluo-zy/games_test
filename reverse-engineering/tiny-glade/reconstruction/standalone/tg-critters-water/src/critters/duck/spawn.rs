//! DuckSpawnLocations::new (0x140f8ee90), spawn_ducks (0x140f8f210, duck 80-197),
//! despawn_ducks (0x140f8f8b0, 206-278). Evidence: critters-extra-analysis 5.

use super::plan::{plan_around, TrajectoryPlan, TrajectoryPlans};
use super::{Duck, DuckConfig, DuckState};
use crate::critters::components::Transform;
use crate::env::{BackupSaveFileKind, DemoMode, TerrainHeights, Time};
use crate::rng::PerchanceRes;
use crate::water::WorldRaster;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use itertools::Itertools;

/// Second-axis values of the 10x10 candidate grid (0xc2500000 ... 0x42500000).
pub const SPAWN_AXIS_Z: [f32; 10] = [-52.0, -40.22, -28.56, -17.33, -5.78, 5.78, 17.33, 28.56, 40.22, 52.0];
/// Live-duck count below which the spawn timer runs 3x faster.
pub const FAST_SPAWN_BELOW: usize = 15;
pub const SPAWN_TIMER_THRESHOLD: f32 = 30.0;
/// `DuckSpawnLocations` tail `+0x18 = 25.0` (0x41c80000), which is also the resource field spawn_ducks
/// accumulates dt into (both docs say `+0x18`): used as the initial timer.
/// UNKNOWN(evidence: critters-extra-analysis 5.1/5.2): that the two `+0x18` mentions are one field.
pub const INITIAL_SPAWN_TIMER: f32 = 25.0;
/// `bilinear water > 0.99` candidates are skipped (@ 0x3f7d70a4).
pub const SPAWN_MAX_WATER: f32 = 0.99;
/// despawn: leave when `0.86 < water || sample < 0.1` (state < 2 branch).
pub const DEPART_HIGH: f32 = 0.86;
pub const DEPART_LOW: f32 = 0.1;

/// 100 candidates `(xz, used)`: x = 10 uniform steps over [-52, 52], z from `SPAWN_AXIS_Z`.
/// UNKNOWN(evidence: critters-extra-analysis 5.1): whether the first axis is x (assumed; doc:
/// "x in [-52,52], second dimension z ten steps").
pub fn duck_spawn_locations() -> Vec<(Vec2, bool)> {
    (0..10)
        .flat_map(|i| {
            let x = -52.0 + 104.0 * i as f32 / 9.0;
            SPAWN_AXIS_Z.iter().map(move |&z| (Vec2::new(x, z), false))
        })
        .collect()
}

/// `DuckSpawnLocations` resource: candidate list plus the spawn timer (`+0x18`).
#[derive(Resource, Clone, Debug)]
pub struct DuckSpawnLocations {
    pub candidates: Vec<(Vec2, bool)>,
    pub timer: f32,
}

impl Default for DuckSpawnLocations {
    fn default() -> Self {
        Self { candidates: duck_spawn_locations(), timer: INITIAL_SPAWN_TIMER }
    }
}

/// spawn_ducks: gate (demo mode / `BackupSaveFileKind::is_corrupted` -> nothing), timer `+= dt`
/// (x3 when < 15 live ducks), at >= 30 pick an unused candidate on water (<= 0.99) with no terrain
/// above water level (height <= 0), build a 3-point plan, mark used, reset timer.
/// UNKNOWN(evidence: critters-extra-analysis 5.2): the `sorted_by` key (unused-first is used; stable,
/// so list order among unused) and when `used` flags reset.
#[allow(clippy::too_many_arguments)]
pub fn spawn_ducks(
    time: Res<Time>,
    demo: Res<DemoMode>,
    save: Res<BackupSaveFileKind>,
    cfg: Res<DuckConfig>,
    water: Res<WorldRaster>,
    terrain: Res<TerrainHeights>,
    ducks: Query<&Duck>,
    mut locations: ResMut<DuckSpawnLocations>,
    mut plans: ResMut<TrajectoryPlans>,
    mut perchance: ResMut<PerchanceRes>,
) {
    if demo.0 || save.corrupted {
        return;
    }
    let dt = time.delta_secs();
    locations.timer += if ducks.iter().count() < FAST_SPAWN_BELOW { 3.0 * dt } else { dt };
    if locations.timer < SPAWN_TIMER_THRESHOLD {
        return;
    }
    let picked = locations
        .candidates
        .iter_mut()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .find(|c| {
            !c.1 && water.sample_at_world_xz(c.0.x, c.0.y) <= SPAWN_MAX_WATER
                && terrain.sample_at_world_xz(c.0.x, c.0.y) <= 0.0
        })
        .map(|c| {
            c.1 = true;
            c.0
        });
    let Some(p) = picked else { return };
    locations.timer = 0.0;
    plans.0.push(plan_around(Vec3::new(p.x, 0.0, p.y), cfg.plan_spread, &mut perchance));
}

/// `0.86 < water || sample < 0.1`.
pub fn needs_departure(water: f32) -> bool {
    water > DEPART_HIGH || water < DEPART_LOW
}

/// despawn_ducks (state < 2 branch with a trajectory): ducks whose water sample is out of range get a
/// departure plan (their own trajectory).
/// UNKNOWN(evidence: critters-extra-analysis 5.3): the `DuckState == 2` thresholds and the 4+1 sample
/// offsets; state 2 never departs here and a single sample at the duck position is used.
pub fn despawn_ducks(
    water: Res<WorldRaster>,
    ducks: Query<(&Duck, &DuckState, &Transform)>,
    mut plans: ResMut<TrajectoryPlans>,
) {
    for (duck, state, transform) in &ducks {
        if state.0 >= DuckState::FREE.0 || duck.trajectory.is_empty() {
            continue;
        }
        let p = transform.translation;
        if needs_departure(water.sample_at_world_xz(p.x, p.z)) {
            plans.0.push(TrajectoryPlan::new(duck.trajectory.clone(), [0.0, 0.0]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::duck::DuckBundle;
    use crate::testing::base_app;
    use bevy_app::{App, Update};

    fn setup(dt: f32, level: u8) -> App {
        let mut app = base_app(dt);
        app.init_resource::<DuckConfig>()
            .init_resource::<DuckSpawnLocations>()
            .init_resource::<TrajectoryPlans>()
            .add_systems(Update, spawn_ducks);
        app.world_mut().resource_mut::<WorldRaster>().composite.fill(level);
        app
    }

    fn plan_count(app: &App) -> usize {
        app.world().resource::<TrajectoryPlans>().0.len()
    }

    #[test]
    fn locations_are_100_in_range_and_timer_starts_at_25() {
        let l = duck_spawn_locations();
        assert_eq!(l.len(), 100);
        assert!(l.iter().all(|&(p, u)| !u && p.x.abs() <= 52.001 && p.y.abs() <= 52.001));
        assert_eq!(l[0].0, Vec2::new(-52.0, -52.0));
        assert_eq!(l[99].0, Vec2::new(52.0, 52.0));
        assert_eq!(DuckSpawnLocations::default().timer, 25.0);
    }

    #[test]
    fn timer_gates_with_normal_rate_when_15_ducks_alive() {
        let mut app = setup(2.0, 200);
        for _ in 0..FAST_SPAWN_BELOW {
            app.world_mut().spawn(Duck::free());
        }
        app.update(); // 27
        app.update(); // 29
        assert_eq!(plan_count(&app), 0);
        app.update(); // 31 >= 30
        assert_eq!(plan_count(&app), 1);
        assert_eq!(app.world().resource::<DuckSpawnLocations>().timer, 0.0);
    }

    #[test]
    fn few_ducks_triple_rate() {
        let mut app = setup(2.0, 200);
        app.update(); // 25 + 3*2 = 31
        assert_eq!(plan_count(&app), 1);
        let loc = app.world().resource::<DuckSpawnLocations>();
        assert!(loc.candidates[0].1, "first candidate marked used");
        let plan = &app.world().resource::<TrajectoryPlans>().0[0];
        assert_eq!(plan.points.len(), 3);
        assert!(plan.points.iter().all(|p| p.distance(Vec3::new(-52.0, 0.0, -52.0)) <= 3.0 + 1e-3));
    }

    #[test]
    fn gated_off_and_unsuitable_water_block() {
        let mut demo = setup(100.0, 200);
        demo.insert_resource(DemoMode(true));
        demo.update();
        assert_eq!(plan_count(&demo), 0);
        assert_eq!(demo.world().resource::<DuckSpawnLocations>().timer, 25.0, "gate returns before the timer");
        let mut corrupted = setup(100.0, 200);
        corrupted.insert_resource(BackupSaveFileKind { corrupted: true });
        corrupted.update();
        assert_eq!(plan_count(&corrupted), 0);
        // water 255/255 = 1.0 > 0.99 -> every candidate skipped, timer keeps running
        let mut full = setup(100.0, 255);
        full.update();
        assert_eq!(plan_count(&full), 0);
        assert!(full.world().resource::<DuckSpawnLocations>().timer >= 30.0);
    }

    #[test]
    fn terrain_above_water_level_skips_candidates() {
        let mut app = setup(100.0, 200);
        app.insert_resource(TerrainHeights { ground_y: 1.0 });
        app.update();
        assert_eq!(plan_count(&app), 0);
    }

    #[test]
    fn used_candidates_not_reused() {
        let mut app = setup(10.0, 200);
        for _ in 0..100 {
            app.update();
        }
        assert_eq!(plan_count(&app), 100);
        app.update();
        assert_eq!(plan_count(&app), 100);
        assert!(app.world().resource::<DuckSpawnLocations>().candidates.iter().all(|c| c.1));
    }

    #[test]
    fn departure_thresholds() {
        assert!(needs_departure(0.87));
        assert!(needs_departure(0.05));
        assert!(!needs_departure(0.5));
    }

    #[test]
    fn despawn_ducks_only_trajectory_ducks_out_of_range() {
        let run = |level: u8| {
            let mut app = base_app(0.016);
            app.init_resource::<TrajectoryPlans>().add_systems(Update, despawn_ducks);
            app.world_mut().resource_mut::<WorldRaster>().composite.fill(level);
            let cfg = DuckConfig::default();
            let mk = |state| {
                let mut d = DuckBundle::new(Vec3::ZERO, state, &cfg);
                d.duck.trajectory = vec![Vec3::ZERO, Vec3::X];
                d
            };
            app.world_mut().spawn(mk(DuckState::TRAJ_FORWARD));
            app.world_mut().spawn(mk(DuckState::FREE));
            app.world_mut().spawn(DuckBundle::new(Vec3::ZERO, DuckState::TRAJ_REVERSE, &cfg));
            app.update();
            app.world().resource::<TrajectoryPlans>().0.clone()
        };
        assert_eq!(run(0).len(), 1);
        assert_eq!(run(0)[0].points, vec![Vec3::ZERO, Vec3::X]);
        assert_eq!(run(128).len(), 0);
        assert_eq!(run(250).len(), 1);
    }
}
