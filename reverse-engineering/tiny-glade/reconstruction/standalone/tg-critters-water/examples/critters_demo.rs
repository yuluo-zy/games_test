//! Run N frames of ducks + wren boids + fish on a real `bevy_app::App` over a water raster and print
//! summary stats.
//! Run: `cargo run -p tg-critters-water --example critters_demo -- [frames]` (default 1200).
//!
//! Demo-only placeholders for values the evidence leaves unknown: duck locomotion/retention, plan
//! spread (the `DuckConfig` default), random-turn cone/interval, fish speed tier (default Slow).

use bevy_app::App;
use bevy_ecs::prelude::Entity;
use glam::Vec3;
use tg_critters_water::critters::boids::{Boid, RandomTurn};
use tg_critters_water::critters::duck::spawn::DuckSpawnLocations;
use tg_critters_water::critters::duck::{Duck, DuckState};
use tg_critters_water::critters::fish::FishFlock;
use tg_critters_water::critters::wren::WrenFlock;
use tg_critters_water::critters::{CrittersPlugin, Transform};
use tg_critters_water::env::Time;
use tg_critters_water::water::tool::{draw_raster, ActiveRasterStroke, StrokePhase};
use tg_critters_water::water::{BufferSel, WorldRaster};

fn spread(b: &[Boid]) -> f32 {
    if b.is_empty() {
        return 0.0;
    }
    let c = b.iter().map(|x| x.pos).sum::<Vec3>() / b.len() as f32;
    b.iter().map(|x| x.pos.distance(c)).sum::<f32>() / b.len() as f32
}

/// Mean distance to the nearest other boid (lower = tighter flock).
fn mean_nn(b: &[Boid]) -> f32 {
    if b.len() < 2 {
        return 0.0;
    }
    b.iter()
        .enumerate()
        .map(|(i, x)| b.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, y)| x.pos.distance(y.pos)).fold(f32::MAX, f32::min))
        .sum::<f32>()
        / b.len() as f32
}

/// Length of the mean unit heading (1 = all aligned).
fn alignment(b: &[Boid]) -> f32 {
    if b.is_empty() {
        return 0.0;
    }
    (b.iter().map(|x| x.vel.normalize_or_zero()).sum::<Vec3>() / b.len() as f32).length()
}

fn pond() -> WorldRaster {
    let mut r = WorldRaster::new();
    let mut s = ActiveRasterStroke { draw_flag: true, ..Default::default() };
    // a thick ring-ish pond: several strokes
    for z in (-30..=30).step_by(4) {
        let z = z as f32;
        draw_raster(&mut r, &mut s, StrokePhase::Start, Vec3::new(-40.0, 0.0, z), 10.0, 2.0);
        draw_raster(&mut r, &mut s, StrokePhase::Update, Vec3::new(40.0, 0.0, z), 10.0, 2.0);
        draw_raster(&mut r, &mut s, StrokePhase::Release, Vec3::new(40.0, 0.0, z), 10.0, 2.0);
    }
    // Demo: scale the pond to level 0.8 so it satisfies spawn_ducks' `water <= 0.99` candidate rule.
    for c in r.composite.iter_mut() {
        *c = (*c as u32 * 4 / 5) as u8;
    }
    r.copy_to_staging(BufferSel::Composite);
    r
}

/// Controlled flock: a tight cluster with headings spread inside +-60 degrees.
fn controlled_flock(rng: &fastrand::Rng) -> (f32, f32, f32, f32) {
    use tg_critters_water::critters::boids::{flock_step, BoidParams};
    let mut b: Vec<Boid> = (0..40)
        .map(|_| {
            let a = rng.f32() * 2.0 - 1.0;
            Boid::new(Vec3::new(rng.f32() * 6.0 - 3.0, 0.0, rng.f32() * 6.0 - 3.0), Vec3::new(a.cos(), 0.0, a.sin()) * 10.0)
        })
        .collect();
    let (a0, n0) = (alignment(&b), mean_nn(&b));
    let p = BoidParams { sep_radius: 1.3, align_weight: 0.1169, min_speed: 7.0, max_speed: 14.0, has_goal: false };
    for _ in 0..300 {
        flock_step(&mut b, &p, 1.0 / 60.0);
    }
    (a0, alignment(&b), n0, mean_nn(&b))
}

fn main() {
    let frames: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(1200);
    let dt = 1.0 / 60.0;
    let raster = pond();

    let mut app = App::new();
    app.insert_resource(raster.clone())
        .insert_resource(Time::from_delta(dt))
        .add_plugins(
            CrittersPlugin::new(2024, 2024).with_random_turn(RandomTurn { cone_half_angle: 0.6, interval: (1.0, 3.0) }),
        );
    // Demo: only use spawn candidates that lie in the pond (flat demo terrain has no shore logic).
    app.world_mut().resource_mut::<DuckSpawnLocations>().candidates.retain(|(p, _)| raster.sample_at_world_xz(p.x, p.y) > 0.5);

    let (mut wren_spread0, mut fish_spread0) = (None, None);
    let mut max_ducks = 0;
    for f in 0..frames {
        // spawn_ducks needs a 30 s timer; the demo advances it by one extra 29 s every second.
        if f % 60 == 0 {
            app.world_mut().resource_mut::<DuckSpawnLocations>().timer += 29.0;
        }
        app.update();
        // trajectory ducks spawned in reverse mode would despawn immediately; demo flips them to arrive
        let reversed: Vec<Entity> = app
            .world_mut()
            .query::<(Entity, &DuckState)>()
            .iter(app.world())
            .filter(|(_, s)| **s == DuckState::TRAJ_REVERSE)
            .map(|(e, _)| e)
            .collect();
        for e in reversed {
            *app.world_mut().get_mut::<DuckState>(e).unwrap() = DuckState::TRAJ_FORWARD;
            app.world_mut().get_mut::<Duck>(e).unwrap().progress = 0.0;
        }
        max_ducks = max_ducks.max(app.world_mut().query::<&Duck>().iter(app.world()).count());
        if f == 300 {
            wren_spread0 = Some(spread(&app.world().resource::<WrenFlock>().0.boids));
            fish_spread0 = Some(spread(&app.world().resource::<FishFlock>().0.boids));
        }
    }

    let ducks: Vec<(Vec3, DuckState)> =
        app.world_mut().query::<(&Transform, &DuckState)>().iter(app.world()).map(|(t, s)| (t.translation, *s)).collect();
    let in_water = ducks.iter().filter(|(p, _)| raster.sample_at_world_xz(p.x, p.z) > 0.5).count();
    let wrens = &app.world().resource::<WrenFlock>().0;
    let fish = &app.world().resource::<FishFlock>().0;
    let fish_in = fish.boids.iter().filter(|b| raster.sample_at_world_xz(b.pos.x, b.pos.z) >= 0.5).count();
    println!("frames: {frames}");
    println!("live ducks: {} (max {max_ducks})  in water: {in_water}", ducks.len());
    println!("  ducks free-swimming: {}", ducks.iter().filter(|(_, s)| *s == DuckState::FREE).count());
    println!(
        "wrens: {}  spread (mean dist to centroid): @300 {:.1} -> end {:.1}  nn-dist {:.2}  alignment {:.2}",
        wrens.len(), wren_spread0.unwrap_or(0.0), spread(&wrens.boids), mean_nn(&wrens.boids), alignment(&wrens.boids)
    );
    println!(
        "fish: {}  in water: {fish_in}  spread: @300 {:.1} -> end {:.1}  nn-dist {:.2}  alignment {:.2}",
        fish.len(), fish_spread0.unwrap_or(0.0), spread(&fish.boids), mean_nn(&fish.boids), alignment(&fish.boids)
    );
    let (a0, a1, n0, n1) = controlled_flock(&fastrand::Rng::with_seed(7));
    println!("controlled wren flock (40 boids, 300 frames): alignment {a0:.2} -> {a1:.2}, nearest-neighbour dist {n0:.2} -> {n1:.2}");
}
