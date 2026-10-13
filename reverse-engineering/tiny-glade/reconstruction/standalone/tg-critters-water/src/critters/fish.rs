//! fish (0x140f90cd0, fish.rs 92-334) and FishSpawnLocations::new (fish.rs 63-81).
//! Evidence: critters-extra-analysis 2. UNKNOWN(evidence: critters-extra-analysis 7): the wall
//! repulsion (WallSpatialHash, 1.12), cursor/camera escape (settings switch table), the hash-based
//! candidate pick (a uniform `fastrand` index is used), the fish y coordinate (0) and the meaning of
//! the `is_corrupted()` speed tiers (selected explicitly by `SpeedTier`).

use super::boids::*;
use super::components::Transform;
use crate::env::Time;
use crate::rng::FastRng;
use crate::water::WorldRaster;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

pub const MAX_FISH: usize = 200; // `< 200`
pub const SPAWN_ATTEMPTS_PER_FRAME: usize = 20; // 0x14
pub const GRID_SIDE: usize = 25; // 25 x 25 = 625
pub const GRID_EXTENT: f32 = 52.0;
pub const SEP_RADIUS: f32 = 0.8;
pub const ALIGN_WEIGHT: f32 = 0.0584;
/// Spawn requires `water > 0.5`; in-loop water check is `>= 0.5`.
pub const WATER_SPAWN_MIN: f32 = 0.5;
pub const BOUNCE_DAMPING: f32 = 0.9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpeedTier {
    /// (0.9, 4.0)
    Slow,
    /// (3.0, 8.0)
    Fast,
}

/// `Fish` marker of the spawn tuple `(Fish, NaniBundle<SolidVertexColorInstanceData>)`.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Fish;

/// The fish BoidFlock.
#[derive(Resource, Clone, Debug, Default)]
pub struct FishFlock(pub BoidFlock);

/// Which `is_corrupted()` speed tier the call site selects.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FishSettings {
    pub tier: SpeedTier,
}

impl Default for FishSettings {
    fn default() -> Self {
        Self { tier: SpeedTier::Slow }
    }
}

pub fn fish_params(tier: SpeedTier) -> BoidParams {
    let (min_speed, max_speed) = match tier {
        SpeedTier::Slow => (0.9, 4.0),
        SpeedTier::Fast => (3.0, 8.0),
    };
    BoidParams { sep_radius: SEP_RADIUS, align_weight: ALIGN_WEIGHT, min_speed, max_speed, has_goal: false }
}

/// 25 x 25 grid points over [-52, 52]^2 (step 104/24).
pub fn fish_spawn_locations() -> Vec<Vec2> {
    let at = |i: usize| -GRID_EXTENT + 2.0 * GRID_EXTENT * i as f32 / (GRID_SIDE - 1) as f32;
    (0..GRID_SIDE).flat_map(|zi| (0..GRID_SIDE).map(move |xi| Vec2::new(at(xi), at(zi)))).collect()
}

/// `FishSpawnLocations` resource.
#[derive(Resource, Clone, Debug)]
pub struct FishSpawnLocations(pub Vec<Vec2>);

impl Default for FishSpawnLocations {
    fn default() -> Self {
        Self(fish_spawn_locations())
    }
}

/// Water bounce: a boid that left the water is moved back (`pos -= vel*dt`) and reflected `vel = -vel*0.9`.
/// Water is tested at the new position (`>= 0.5` is water).
pub fn water_bounce(water: &WorldRaster, b: &mut Boid, dt: f32) {
    if water.sample_at_world_xz(b.pos.x, b.pos.z) < WATER_SPAWN_MIN {
        b.pos -= b.vel * dt;
        b.vel = -b.vel * BOUNCE_DAMPING;
    }
}

/// Fill up to 200 fish, at most 20 attempts per frame; returns how many spawned.
/// Initial velocity `(u - 0.5, 0, u - 1.0)` follows the decompile literally
/// (UNKNOWN(evidence: critters-extra-analysis 2): the y part, which looks like 2D).
fn spawn_fish(
    commands: &mut Commands,
    water: &WorldRaster,
    locations: &FishSpawnLocations,
    rng: &mut FastRng,
    flock: &mut BoidFlock,
) -> usize {
    let mut spawned = 0;
    for _ in 0..SPAWN_ATTEMPTS_PER_FRAME {
        if flock.len() >= MAX_FISH {
            break;
        }
        let loc = locations.0[(rng.f32() * locations.0.len() as f32) as usize % locations.0.len()];
        if water.sample_at_world_xz(loc.x, loc.y) > WATER_SPAWN_MIN {
            let vel = Vec3::new(rng.f32() - 0.5, 0.0, rng.f32() - 1.0);
            let pos = Vec3::new(loc.x, 0.0, loc.y);
            let entity = commands.spawn((Fish, Transform::at(pos))).id();
            flock.push(Boid::new(pos, vel), entity);
            spawned += 1;
        }
    }
    spawned
}

/// One frame: spawn, flock step, water bounce, entity transforms follow the boids.
#[allow(clippy::too_many_arguments)]
pub fn sim_fish(
    mut commands: Commands,
    time: Res<Time>,
    water: Res<WorldRaster>,
    settings: Res<FishSettings>,
    locations: Res<FishSpawnLocations>,
    mut flock: ResMut<FishFlock>,
    mut rng: ResMut<FastRng>,
    mut transforms: Query<&mut Transform, With<Fish>>,
) {
    let dt = time.delta_secs();
    let flock = &mut flock.0;
    spawn_fish(&mut commands, &water, &locations, &mut rng, flock);
    flock_step(&mut flock.boids, &fish_params(settings.tier), dt);
    for b in flock.boids.iter_mut() {
        water_bounce(&water, b, dt);
    }
    for (b, e) in flock.boids.iter().zip(&flock.entities) {
        if let Ok(mut t) = transforms.get_mut(*e) {
            t.translation = b.pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::base_app;
    use bevy_app::{App, Update};

    fn pond_raster() -> WorldRaster {
        let mut r = WorldRaster::new();
        for row in 150..362 {
            for col in 150..362 {
                r.composite[row * 512 + col] = 255;
            }
        }
        r
    }

    fn setup(tier: SpeedTier, wet: bool, seed: u64) -> App {
        let mut app = base_app(0.02);
        app.init_resource::<FishFlock>()
            .init_resource::<FishSpawnLocations>()
            .insert_resource(FishSettings { tier })
            .insert_resource(FastRng::seeded(seed))
            .add_systems(Update, sim_fish);
        if wet {
            app.insert_resource(pond_raster());
        }
        app
    }

    fn boids(app: &App) -> &Vec<Boid> {
        &app.world().resource::<FishFlock>().0.boids
    }

    #[test]
    fn grid_625_in_extent() {
        let l = fish_spawn_locations();
        assert_eq!(l.len(), 625);
        assert_eq!(l[0], Vec2::new(-52.0, -52.0));
        assert_eq!(l[624], Vec2::new(52.0, 52.0));
        assert!(l.iter().all(|p| p.abs().max_element() <= 52.0 + 1e-4));
    }

    #[test]
    fn tiers() {
        let s = fish_params(SpeedTier::Slow);
        let f = fish_params(SpeedTier::Fast);
        assert_eq!((s.min_speed, s.max_speed, s.sep_radius, s.align_weight), (0.9, 4.0, 0.8, 0.0584));
        assert_eq!((f.min_speed, f.max_speed), (3.0, 8.0));
        assert_eq!(FishSettings::default().tier, SpeedTier::Slow);
    }

    #[test]
    fn spawns_only_in_water_and_caps_attempts() {
        let mut app = setup(SpeedTier::Slow, true, 3);
        app.update();
        let n = boids(&app).len();
        assert!(n > 0 && n <= SPAWN_ATTEMPTS_PER_FRAME);
        let water = pond_raster();
        assert!(boids(&app).iter().all(|b| water.sample_at_world_xz(b.pos.x, b.pos.z) > 0.0));
        let mut dry = setup(SpeedTier::Slow, false, 3);
        dry.update();
        assert_eq!(boids(&dry).len(), 0);
    }

    #[test]
    fn population_capped_at_200_with_matching_entities() {
        let mut app = setup(SpeedTier::Slow, true, 3);
        for _ in 0..200 {
            app.update();
        }
        assert_eq!(boids(&app).len(), MAX_FISH);
        assert_eq!(app.world_mut().query::<&Fish>().iter(app.world()).count(), MAX_FISH);
    }

    #[test]
    fn bounce_reflects_with_damping() {
        let dry = WorldRaster::new();
        let mut b = Boid::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0));
        water_bounce(&dry, &mut b, 0.5);
        assert_eq!(b.pos.x, 9.0);
        assert!((b.vel.x + 1.8).abs() < 1e-6);
        let wet = pond_raster();
        let mut ok = Boid::new(Vec3::ZERO, Vec3::X);
        water_bounce(&wet, &mut ok, 0.5);
        assert_eq!(ok.vel, Vec3::X);
    }

    #[test]
    fn fish_mostly_stay_in_pond_and_transforms_follow() {
        let mut app = setup(SpeedTier::Slow, true, 9);
        for _ in 0..300 {
            app.update();
        }
        let w = pond_raster();
        let b = boids(&app);
        let inside = b.iter().filter(|b| w.sample_at_world_xz(b.pos.x, b.pos.z) >= 0.5).count();
        assert!(inside as f32 / b.len() as f32 > 0.9);
        let flock = &app.world().resource::<FishFlock>().0;
        let t = app.world().get::<Transform>(flock.entities[0]).unwrap();
        assert_eq!(t.translation, flock.boids[0].pos);
    }
}
