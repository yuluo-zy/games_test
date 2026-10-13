//! Same seeds => identical simulation on a real `bevy_app::App` (reproducibility requirement);
//! different seeds => different simulation.

use bevy_app::App;
use bevy_ecs::prelude::{Entity, With};
use glam::Vec3;
use tg_critters_water::critters::boids::RandomTurn;
use tg_critters_water::critters::duck::plan::{plan_around, TrajectoryPlans};
use tg_critters_water::critters::duck::{Duck, DuckState};
use tg_critters_water::critters::fish::FishFlock;
use tg_critters_water::critters::wren::WrenFlock;
use tg_critters_water::critters::{CrittersPlugin, Transform};
use tg_critters_water::env::Time;
use tg_critters_water::rng::PerchanceRes;
use tg_critters_water::water::WorldRaster;

type Snapshot = (Vec<Vec3>, Vec<Vec3>, Vec<Vec3>);

fn run(perchance_seed: u128, fastrand_seed: u64) -> Snapshot {
    let mut raster = WorldRaster::new();
    raster.composite.fill(200);
    let mut app = App::new();
    app.insert_resource(raster)
        .insert_resource(Time::from_delta(0.016))
        .add_plugins(CrittersPlugin::new(perchance_seed, fastrand_seed).with_random_turn(RandomTurn {
            cone_half_angle: 0.5,
            interval: (1.0, 2.0),
        }));
    let plan = plan_around(Vec3::ZERO, 3.0, &mut PerchanceRes::seeded(3));
    app.world_mut().resource_mut::<TrajectoryPlans>().0.push(plan);
    app.update(); // plan_duck spawns the duck reversed (state 3); send it forward so it swims
    let duck = app.world_mut().query_filtered::<Entity, With<Duck>>().single(app.world()).unwrap();
    *app.world_mut().get_mut::<DuckState>(duck).unwrap() = DuckState::TRAJ_FORWARD;
    app.world_mut().get_mut::<Duck>(duck).unwrap().progress = 0.0;
    for _ in 0..300 {
        app.update();
    }
    let mut ducks: Vec<Vec3> = app
        .world_mut()
        .query::<(&Duck, &DuckState, &Transform)>()
        .iter(app.world())
        .map(|(_, _, t)| t.translation)
        .collect();
    ducks.sort_by(|a, b| a.to_array().partial_cmp(&b.to_array()).unwrap());
    (
        ducks,
        app.world().resource::<WrenFlock>().0.boids.iter().map(|b| b.pos).collect(),
        app.world().resource::<FishFlock>().0.boids.iter().map(|b| b.pos).collect(),
    )
}

#[test]
fn same_seed_same_world() {
    assert_eq!(run(5, 5), run(5, 5));
}

#[test]
fn different_fastrand_seed_changes_boids_and_perchance_seed_changes_ducks() {
    let base = run(5, 5);
    let other_fast = run(5, 6);
    assert_ne!(base.1, other_fast.1);
    assert_ne!(base.2, other_fast.2);
    assert_eq!(base.0, other_fast.0, "ducks use perchance only");
    assert_ne!(base.0, run(6, 5).0);
}
