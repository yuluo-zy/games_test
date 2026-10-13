//! sim_wren_boids (0x142326a80, wren_boids.rs 56-533). Evidence: critters-extra-analysis 1.
//! UNKNOWN(evidence: critters-extra-analysis 7): BirdSpots/FreeSpot perching, BirdHaven tree spheres,
//! the RAY_DIRS (32 x Vec3) steering of `closure_3` (no ray steering is modelled), the exact
//! per-frame spawn count (1 assumed), the initial velocity (random direction at min speed), the
//! tree-shake trigger quantities (the BirdHaven timers/threshold are replaced by a `TreeShaken`
//! event; only the burst size rule is modelled).

use super::boids::*;
use super::components::Transform;
use crate::env::Time;
use crate::rng::FastRng;
use crate::sample::uniform_sample_sphere;
use bevy_ecs::prelude::*;
use glam::Vec3;

pub const MAX_WRENS: usize = 80; // `0x4f < n -> break`
pub const KEEP_RADIUS: f32 = 91.9; // 0x42b7d907
pub const KEEP_MAX_Y: f32 = 30.0; // 0x41f00000
pub const SEP_RADIUS: f32 = 1.3;
pub const ALIGN_WEIGHT: f32 = 0.1169;
pub const MIN_SPEED: f32 = 7.0;
pub const MAX_SPEED: f32 = 14.0;

/// `Wren` marker of the spawn tuple `(Wren, NaniBundle<WrenInstanceData>, PrevTransform)`.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Wren;

/// The BoidFlock (boids + 0x38 slot entities) held by sim_wren_boids.
#[derive(Resource, Clone, Debug, Default)]
pub struct WrenFlock(pub BoidFlock);

/// The wren switch (`*(char *)(*in_R8 + 5)`): off despawns every wren.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrenSettings {
    pub enabled: bool,
}

impl Default for WrenSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// "Tree has been shaken!" (L1231+): the BirdHaven tree shake that spawns a burst of birds at `pos`.
/// UNKNOWN(evidence: critters-extra-analysis 1.1): the quantities that trigger it (`> 10.0` and the
/// vertical timer), so it is an input event here.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct TreeShaken {
    pub pos: Vec3,
}

pub fn wren_params() -> BoidParams {
    BoidParams { sep_radius: SEP_RADIUS, align_weight: ALIGN_WEIGHT, min_speed: MIN_SPEED, max_speed: MAX_SPEED, has_goal: false }
}

/// Spawn position with `fastrand::f32`: `x = u*130-65`, `y = u*9+1`, `z = u*130-65` (z "same as x").
pub fn spawn_position(rng: &mut FastRng) -> Vec3 {
    let (x, y, z) = (rng.f32() * 130.0 - 65.0, rng.f32() * 9.0 + 1.0, rng.f32() * 130.0 - 65.0);
    Vec3::new(x, y, z)
}

/// Kept while `hypot(x, z) <= 91.9 && y <= 30`. (The doc prints `sqrt(x^2+y^2)`; xz is the sensible
/// reading given the separate y test: UNKNOWN(evidence: critters-extra-analysis 1.1).)
pub fn within_keep_volume(p: Vec3) -> bool {
    p.x.hypot(p.z) <= KEEP_RADIUS && p.y <= KEEP_MAX_Y
}

fn spawn_wren(commands: &mut Commands, flock: &mut BoidFlock, pos: Vec3, rng: &mut FastRng) {
    let dir = uniform_sample_sphere(rng.f32(), rng.f32());
    let entity = commands.spawn((Wren, Transform::at(pos))).id();
    flock.push(Boid::new(pos, dir * MIN_SPEED), entity);
}

/// Tree-shake burst: `floor(rng * 5 + 1)` (1..=5) birds at `pos`, capped at 80 total.
fn shake_tree(commands: &mut Commands, flock: &mut BoidFlock, pos: Vec3, rng: &mut FastRng) {
    let burst = (rng.f32() * 5.0 + 1.0) as usize;
    log::info!("Tree has been shaken! Spawning {burst} birds");
    for _ in 0..burst {
        if flock.len() >= MAX_WRENS {
            break;
        }
        spawn_wren(commands, flock, pos, rng);
    }
}

fn remove_and_despawn(commands: &mut Commands, flock: &mut BoidFlock, indices: &[u32]) {
    for e in flock.remove(indices) {
        if let Ok(mut ec) = commands.get_entity(e) {
            ec.despawn();
        }
    }
}

/// One frame. `enabled == false` despawns every wren. Otherwise: spawn one (dt != 0, below the
/// cap), tree-shake bursts, flock step, random turns, then remove boids that left the keep volume.
#[allow(clippy::too_many_arguments)]
pub fn sim_wren_boids(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<WrenSettings>,
    turn: Option<Res<RandomTurn>>,
    mut flock: ResMut<WrenFlock>,
    mut rng: ResMut<FastRng>,
    mut shakes: EventReader<TreeShaken>,
    mut transforms: Query<&mut Transform, With<Wren>>,
) {
    let dt = time.delta_secs();
    let flock = &mut flock.0;
    if !settings.enabled {
        let all: Vec<u32> = (0..flock.len() as u32).collect();
        remove_and_despawn(&mut commands, flock, &all);
        shakes.clear();
        return;
    }
    if dt != 0.0 && flock.len() < MAX_WRENS {
        let p = spawn_position(&mut rng);
        spawn_wren(&mut commands, flock, p, &mut rng);
    }
    for shake in shakes.read() {
        shake_tree(&mut commands, flock, shake.pos, &mut rng);
    }
    flock_step(&mut flock.boids, &wren_params(), dt);
    if let Some(cfg) = turn {
        for b in flock.boids.iter_mut() {
            random_vel_change(b, dt, *cfg, &mut rng);
        }
    }
    for (b, e) in flock.boids.iter().zip(&flock.entities) {
        if let Ok(mut t) = transforms.get_mut(*e) {
            t.translation = b.pos;
        }
    }
    let gone: Vec<u32> =
        flock.boids.iter().enumerate().filter(|(_, b)| !within_keep_volume(b.pos)).map(|(i, _)| i as u32).collect();
    remove_and_despawn(&mut commands, flock, &gone);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{base_app, FASTRAND_SEED};
    use bevy_app::{App, Update};

    fn setup(dt: f32) -> App {
        let mut app = base_app(dt);
        app.init_resource::<WrenFlock>().init_resource::<WrenSettings>().add_systems(Update, sim_wren_boids);
        app
    }

    fn flock_len(app: &App) -> usize {
        app.world().resource::<WrenFlock>().0.len()
    }

    fn wren_entities(app: &mut App) -> usize {
        app.world_mut().query::<&Wren>().iter(app.world()).count()
    }

    #[test]
    fn spawn_position_ranges_and_stream() {
        let mut r = FastRng::seeded(4);
        for _ in 0..500 {
            let p = spawn_position(&mut r);
            assert!((-65.0..65.0).contains(&p.x) && (-65.0..65.0).contains(&p.z) && (1.0..10.0).contains(&p.y));
        }
        let mirror = fastrand::Rng::with_seed(4);
        let p = spawn_position(&mut FastRng::seeded(4));
        assert_eq!(p, Vec3::new(mirror.f32() * 130.0 - 65.0, mirror.f32() * 9.0 + 1.0, mirror.f32() * 130.0 - 65.0));
    }

    #[test]
    fn keep_volume() {
        assert!(within_keep_volume(Vec3::new(91.0, 29.0, 0.0)));
        assert!(!within_keep_volume(Vec3::new(92.0, 1.0, 0.0)));
        assert!(!within_keep_volume(Vec3::new(0.0, 31.0, 0.0)));
    }

    #[test]
    fn spawns_one_per_frame_up_to_cap_and_never_over() {
        let mut app = setup(0.016);
        for _ in 0..200 {
            app.update();
            assert!(flock_len(&app) <= MAX_WRENS);
        }
        assert!(flock_len(&app) > 40);
        assert_eq!(wren_entities(&mut app), flock_len(&app), "every boid has its entity");
        let before = flock_len(&app);
        app.insert_resource(Time::from_delta(0.0));
        app.update();
        assert_eq!(flock_len(&app), before, "dt == 0 does not spawn");
    }

    #[test]
    fn disabled_switch_clears_all() {
        let mut app = setup(0.016);
        for _ in 0..10 {
            app.update();
        }
        assert!(flock_len(&app) > 0);
        app.insert_resource(WrenSettings { enabled: false });
        app.update();
        assert_eq!(flock_len(&app), 0);
        assert_eq!(wren_entities(&mut app), 0);
    }

    #[test]
    fn escapees_are_removed() {
        let mut app = setup(0.0);
        let e = app.world_mut().spawn((Wren, Transform::at(Vec3::new(95.0, 5.0, 0.0)))).id();
        let kept = app.world_mut().spawn((Wren, Transform::default())).id();
        {
            let mut f = app.world_mut().resource_mut::<WrenFlock>();
            f.0.push(Boid::new(Vec3::new(95.0, 5.0, 0.0), Vec3::new(14.0, 0.0, 0.0)), e);
            f.0.push(Boid::new(Vec3::new(1.0, 5.0, 0.0), Vec3::new(14.0, 0.0, 0.0)), kept);
        }
        app.update();
        assert_eq!(flock_len(&app), 1);
        assert!(app.world().get_entity(e).is_err());
        assert!(app.world().get_entity(kept).is_ok());
    }

    #[test]
    fn boid_positions_are_written_to_transforms() {
        let mut app = setup(0.0);
        let e = app.world_mut().spawn((Wren, Transform::default())).id();
        app.world_mut().resource_mut::<WrenFlock>().0.push(Boid::new(Vec3::new(3.0, 4.0, 5.0), Vec3::new(14.0, 0.0, 0.0)), e);
        app.update();
        assert_eq!(app.world().get::<Transform>(e).unwrap().translation, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn shake_burst_size_follows_fastrand_and_cap() {
        let mut app = setup(0.0);
        app.world_mut().send_event(TreeShaken { pos: Vec3::new(1.0, 2.0, 3.0) });
        app.update();
        let burst = (fastrand::Rng::with_seed(FASTRAND_SEED).f32() * 5.0 + 1.0) as usize;
        assert!((1..=5).contains(&burst));
        assert_eq!(flock_len(&app), burst);
        assert!(app.world().resource::<WrenFlock>().0.boids.iter().all(|b| b.pos == Vec3::new(1.0, 2.0, 3.0)));

        let mut full = setup(0.0);
        for i in 0..MAX_WRENS {
            let e = full.world_mut().spawn((Wren, Transform::default())).id();
            full.world_mut().resource_mut::<WrenFlock>().0.push(Boid::new(Vec3::splat(i as f32 * 0.0), Vec3::X * 10.0), e);
        }
        full.world_mut().send_event(TreeShaken { pos: Vec3::ZERO });
        full.update();
        assert_eq!(flock_len(&full), MAX_WRENS);
    }

    #[test]
    fn random_turn_hook_runs() {
        let mut app = setup(0.016);
        app.insert_resource(RandomTurn { cone_half_angle: 0.5, interval: (1.0, 2.0) });
        for _ in 0..5 {
            app.update();
        }
        let f = &app.world().resource::<WrenFlock>().0;
        assert!(!f.is_empty() && f.boids.iter().all(|b| b.turn_timer > 0.0));
    }

    #[test]
    fn same_seed_same_flock() {
        let run = || {
            let mut app = setup(0.016);
            for _ in 0..60 {
                app.update();
            }
            app.world().resource::<WrenFlock>().0.boids.iter().map(|b| b.pos).collect::<Vec<_>>()
        };
        assert_eq!(run(), run());
    }
}
