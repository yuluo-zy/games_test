//! `CrittersPlugin`: registers the critters events, resources and systems.
//!
//! Order (UNKNOWN(evidence: critters-behavior-analysis 1/2): the real schedule; the document
//! numbering is followed, which is also the order the earlier standalone frame used):
//! `Plan` (spawn_ducks, plan_duck_trajectories, move_ducks_along_trajectories) ->
//! `Goals` (duck / sheep / butterfly / firefly goals, butterfly states) -> `Velocities` ->
//! `Collision` (critters_avoid_collisions, DespawnDuckCmd consumer) ->
//! `Motion` (smooth_critter_velocities, sheep_walk_on_terrain, move_critters) -> `Gpu` (ripples).
//! The boid flocks (wren, fish) are independent of the goal chain.

use super::boids::RandomTurn;
use super::butterfly::*;
use super::duck::goals::update_duck_goals;
use super::duck::movement::move_ducks_along_trajectories;
use super::duck::plan::{plan_duck_trajectories, TrajectoryPlans};
use super::duck::ripples::{update_duck_ripples, DuckRipples};
use super::duck::spawn::{despawn_ducks, spawn_ducks, DuckSpawnLocations};
use super::duck::velocities::set_duck_velocities_towards_goals;
use super::duck::DuckConfig;
use super::firefly::*;
use super::fish::{sim_fish, FishFlock, FishSettings, FishSpawnLocations};
use super::sheep::{update_sheep_goals, SheepGoalConfig};
use super::sheep_terrain::sheep_walk_on_terrain;
use super::shared::{
    critters_avoid_collisions, despawn_duck_on_cmd, move_critters, smooth_critter_velocities, DespawnDuckCmd,
};
use super::wren::{sim_wren_boids, TreeShaken, WrenFlock, WrenSettings};
use crate::env::EnvPlugin;
use crate::rng::{FastRng, PerchanceRes};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CrittersSet {
    Plan,
    Goals,
    Velocities,
    Collision,
    Motion,
    Gpu,
}

/// Seeds: `perchance_seed` for the goal systems, `fastrand_seed` for boids / spawn positions.
pub struct CrittersPlugin {
    pub perchance_seed: u128,
    pub fastrand_seed: u64,
    /// Optional boid random-turn parameters (UNKNOWN(evidence: critters-extra-analysis 1.4)); inserted when set.
    pub random_turn: Option<RandomTurn>,
    /// Register `despawn_ducks`. Off by default: its departure plans are consumed by plan_duck, which
    /// spawns a *new* duck per plan (critters-extra 5.3 vs behavior 1.1), so a duck that stays out of
    /// range would breed one plan and one duck per frame. UNKNOWN(evidence: critters-extra-analysis 5.3).
    pub duck_departures: bool,
}

impl CrittersPlugin {
    pub fn new(perchance_seed: u128, fastrand_seed: u64) -> Self {
        Self { perchance_seed, fastrand_seed, random_turn: None, duck_departures: false }
    }

    pub fn with_random_turn(mut self, turn: RandomTurn) -> Self {
        self.random_turn = Some(turn);
        self
    }
}

impl CrittersPlugin {
    pub fn with_duck_departures(mut self) -> Self {
        self.duck_departures = true;
        self
    }
}

impl Default for CrittersPlugin {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Plugin for CrittersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnvPlugin)
            .add_event::<DespawnDuckCmd>()
            .add_event::<TreeShaken>()
            .insert_resource(PerchanceRes::seeded(self.perchance_seed))
            .insert_resource(FastRng::seeded(self.fastrand_seed))
            .init_resource::<DuckConfig>()
            .init_resource::<TrajectoryPlans>()
            .init_resource::<DuckSpawnLocations>()
            .init_resource::<DuckRipples>()
            .init_resource::<SheepGoalConfig>()
            .init_resource::<WrenFlock>()
            .init_resource::<WrenSettings>()
            .init_resource::<FishFlock>()
            .init_resource::<FishSettings>()
            .init_resource::<FishSpawnLocations>()
            .init_resource::<FireflyControl>();
        if let Some(turn) = self.random_turn {
            app.insert_resource(turn);
        }
        if self.duck_departures {
            app.add_systems(
                Update,
                despawn_ducks.in_set(CrittersSet::Plan).after(spawn_ducks).before(plan_duck_trajectories),
            );
        }
        app.configure_sets(
            Update,
            (
                CrittersSet::Plan,
                CrittersSet::Goals,
                CrittersSet::Velocities,
                CrittersSet::Collision,
                CrittersSet::Motion,
                CrittersSet::Gpu,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (spawn_ducks, plan_duck_trajectories, move_ducks_along_trajectories)
                    .chain()
                    .in_set(CrittersSet::Plan),
                (
                    update_duck_goals,
                    update_sheep_goals,
                    update_butterfly_goals,
                    update_butterfly_states,
                    update_firefly_goals,
                )
                    .chain()
                    .in_set(CrittersSet::Goals),
                (
                    set_duck_velocities_towards_goals,
                    set_butterfly_velocities_towards_goals,
                    set_firefly_velocities_towards_goals,
                )
                    .chain()
                    .in_set(CrittersSet::Velocities),
                (critters_avoid_collisions, despawn_duck_on_cmd).chain().in_set(CrittersSet::Collision),
                (smooth_critter_velocities, sheep_walk_on_terrain, move_critters).chain().in_set(CrittersSet::Motion),
                update_duck_ripples.in_set(CrittersSet::Gpu),
                (sim_wren_boids, sim_fish),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::components::*;
    use crate::critters::duck::{Duck, DuckBundle, DuckState};
    use crate::env::Time;
    use crate::water::WorldRaster;
    use glam::Vec3;

    fn pond_app(seed: u128) -> App {
        let mut app = App::new();
        let mut raster = WorldRaster::new();
        for row in 156..356 {
            for col in 156..356 {
                raster.composite[row * 512 + col] = 255;
            }
        }
        app.insert_resource(raster)
            .insert_resource(Time::from_delta(0.016))
            .add_plugins(CrittersPlugin::new(seed, 7));
        app
    }

    #[test]
    fn plugin_registers_everything_a_frame_needs() {
        let mut app = pond_app(1);
        app.update();
        let w = app.world();
        assert!(w.contains_resource::<PerchanceRes>() && w.contains_resource::<FastRng>());
        assert!(w.contains_resource::<Events<DespawnDuckCmd>>() && w.contains_resource::<Events<TreeShaken>>());
        assert!(w.contains_resource::<DuckSpawnLocations>() && w.contains_resource::<WrenFlock>());
    }

    #[test]
    fn plan_to_free_swim_pipeline_stays_in_water() {
        let mut app = pond_app(42);
        // spawn_ducks is idle for 30 s of timer; hand the chain a plan directly through the resource.
        let plan = super::super::duck::plan::plan_around(Vec3::ZERO, 4.0, &mut PerchanceRes::seeded(3));
        app.world_mut().resource_mut::<TrajectoryPlans>().0.push(plan);
        app.update();
        let mut q = app.world_mut().query::<(Entity, &DuckState)>();
        let ducks: Vec<_> = q.iter(app.world()).map(|(e, s)| (e, *s)).collect();
        assert_eq!(ducks.len(), 1);
        assert_eq!(ducks[0].1, DuckState::TRAJ_REVERSE);
        assert!(app.world().resource::<TrajectoryPlans>().0.is_empty());
        // pipeline for a forward duck: becomes a free swimmer which keeps moving inside the pond
        let e = ducks[0].0;
        *app.world_mut().get_mut::<DuckState>(e).unwrap() = DuckState::TRAJ_FORWARD;
        app.world_mut().get_mut::<Duck>(e).unwrap().progress = 0.0;
        for _ in 0..3000 {
            app.update();
        }
        assert_eq!(*app.world().get::<DuckState>(e).unwrap(), DuckState::FREE);
        let p = app.world().get::<Transform>(e).unwrap().translation;
        assert!(app.world().resource::<WorldRaster>().sample_at_world_xz(p.x, p.z) > 0.5, "duck at {p:?}");
    }

    #[test]
    fn collision_despawn_removes_duck_in_a_full_frame() {
        let mut app = pond_app(1);
        app.world_mut().resource_mut::<crate::env::RaycastWorld>().spheres.push((Vec3::new(5.05, 0.0, 0.0), 5.0));
        let cfg = DuckConfig::default();
        let mut b = DuckBundle::new(Vec3::ZERO, DuckState::FREE, &cfg);
        b.body.goal = CritterGoal::set(Vec3::new(5.0, 0.0, 0.0));
        b.duck.moving = true;
        b.duck.timers = [9.0, 9.0];
        let e = app.world_mut().spawn(b).id();
        app.update();
        assert!(app.world().get_entity(e).is_err());
    }

    #[test]
    fn same_seed_same_world_and_different_seed_differs() {
        let run = |seed: u128| {
            let mut app = pond_app(seed);
            let mut b = DuckBundle::new(Vec3::new(5.0, 0.0, 5.0), DuckState::FREE, &DuckConfig::default());
            b.duck.moving = true;
            b.duck.timers = [9.0, 9.0];
            let e = app.world_mut().spawn(b).id();
            for _ in 0..200 {
                app.update();
            }
            app.world().get::<Transform>(e).unwrap().translation
        };
        assert_eq!(run(5), run(5));
        assert_ne!(run(5), run(6));
    }
}
