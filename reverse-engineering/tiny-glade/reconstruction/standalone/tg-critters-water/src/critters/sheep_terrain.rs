//! sheep_terrain::sheep_walk_on_terrain (0x14231bd80, sheep_terrain.rs 15-80).
//! Evidence: critters-behavior-analysis 3. The vertical-velocity 12 B record is modelled as
//! `Velocity.y` (the move_critters system applies it); this identification is an interpretation.

use super::components::{Transform, Velocity};
use super::sheep::SheepTerrain;
use crate::env::{TerrainHeights, Time};
use crate::water::WorldRaster;
use bevy_ecs::prelude::*;
use lerp::Lerp;

pub const SMOOTH_RATE: f32 = -3.0;
/// Body offset above the terrain in the walking state.
pub const WALK_HEIGHT_OFFSET: f32 = 4.0;

/// Terrain states from the switch @0x1030: 2 ground, 3 panic, 4 transition, 6 floating; others walk.
pub const STATE_GROUND: u32 = 2;
pub const STATE_UNREACHABLE: u32 = 3;
pub const STATE_TRANSITION: u32 = 4;
pub const STATE_FLOAT: u32 = 6;

/// Query `(Transform, state 8 B, vertical velocity 12 B)`; `case 3` panics like the original.
pub fn sheep_walk_on_terrain(
    time: Res<Time>,
    terrain: Res<TerrainHeights>,
    water: Res<WorldRaster>,
    mut sheep: Query<(&mut Transform, &mut SheepTerrain, &mut Velocity)>,
) {
    let dt = time.delta_secs();
    let smooth = 1.0 - (SMOOTH_RATE * dt).exp();
    for (mut transform, mut state, mut velocity) in &mut sheep {
        let pos = &mut transform.translation;
        let h = terrain.sample_at_world_xz(pos.x, pos.z);
        let v = &mut velocity.0.y;
        match state.state {
            STATE_GROUND => pos.y = h,
            STATE_UNREACHABLE => panic!("sheep_walk_on_terrain: terrain state 3 is unreachable"),
            STATE_TRANSITION => {
                // `utils::time_continuous_lerp`; UNKNOWN(evidence: critters-behavior-analysis 3): rate/args.
                pos.y = pos.y.lerp(h, smooth);
                if water.sample_at_world_xz(pos.x, pos.z) <= 0.0 {
                    state.state = STATE_GROUND;
                }
            }
            STATE_FLOAT => {
                let dy = (h - pos.y).clamp(-0.3, 0.3);
                *v += (dy * 20.0 - *v) * smooth;
                if dy > -0.05 {
                    state.state = STATE_GROUND;
                    *v = 0.0;
                }
            }
            _ => {
                let dy = (h + WALK_HEIGHT_OFFSET) - pos.y;
                if h <= pos.y {
                    *v += (dy.clamp(-2.0, 2.0) * 10.0 - *v) * smooth;
                } else {
                    pos.y = h;
                    *v += (dy.clamp(-5.0, 5.0) * 10.0 - *v) * smooth * 3.0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::sheep::SheepBundle;
    use crate::testing::base_app;
    use bevy_app::{App, Update};
    use glam::Vec3;

    fn setup(dt: f32, ground: f32) -> App {
        let mut app = base_app(dt);
        app.insert_resource(TerrainHeights { ground_y: ground }).add_systems(Update, sheep_walk_on_terrain);
        app
    }

    fn sheep(app: &mut App, state: u32, y: f32) -> Entity {
        let mut b = SheepBundle::new(Vec3::new(0.0, y, 0.0), 0, 0.5);
        b.terrain.state = state;
        app.world_mut().spawn(b).id()
    }

    fn y(app: &App, e: Entity) -> f32 {
        app.world().get::<Transform>(e).unwrap().translation.y
    }

    fn vy(app: &App, e: Entity) -> f32 {
        app.world().get::<Velocity>(e).unwrap().0.y
    }

    #[test]
    fn ground_snaps() {
        let mut app = setup(0.1, 2.5);
        let e = sheep(&mut app, STATE_GROUND, 9.0);
        app.update();
        assert_eq!(y(&app, e), 2.5);
    }

    #[test]
    #[should_panic]
    fn state3_panics_like_the_original() {
        let mut app = setup(0.1, 0.0);
        sheep(&mut app, STATE_UNREACHABLE, 0.0);
        app.update();
    }

    #[test]
    fn walk_below_terrain_snaps_up_and_pushes_velocity_x3() {
        let mut app = setup(1.0, 0.0);
        let e = sheep(&mut app, 0, -1.0);
        app.update();
        let sm = 1.0 - (-3.0f32).exp();
        assert_eq!(y(&app, e), 0.0);
        // dy = (0+4)-(-1) = 5 -> clamp 5 -> target 50; v = 50 * sm * 3
        assert!((vy(&app, e) - 50.0 * sm * 3.0).abs() < 1e-4);
    }

    #[test]
    fn walk_above_terrain_clamps_to_two_and_gain_ten() {
        let mut app = setup(1.0, 0.0);
        let e = sheep(&mut app, 0, 10.0);
        app.update();
        let sm = 1.0 - (-3.0f32).exp();
        assert_eq!(y(&app, e), 10.0, "not snapped");
        // dy = 4 - 10 = -6 -> clamp -2 -> target -20
        assert!((vy(&app, e) + 20.0 * sm).abs() < 1e-4);
    }

    #[test]
    fn float_state_exit_condition() {
        let mut app = setup(0.1, 0.0);
        let a = sheep(&mut app, STATE_FLOAT, 0.0);
        app.world_mut().get_mut::<Velocity>(a).unwrap().0.y = 3.0;
        let b = sheep(&mut app, STATE_FLOAT, 5.0); // dy = -5 -> clamp -0.3 -> stays floating
        app.update();
        assert_eq!(app.world().get::<SheepTerrain>(a).unwrap().state, STATE_GROUND);
        assert_eq!(vy(&app, a), 0.0);
        assert_eq!(app.world().get::<SheepTerrain>(b).unwrap().state, STATE_FLOAT);
        assert!(vy(&app, b) < 0.0);
    }

    #[test]
    fn transition_lands_when_no_water() {
        let mut dry = setup(0.5, 0.0);
        let e = sheep(&mut dry, STATE_TRANSITION, 3.0);
        dry.update();
        assert_eq!(dry.world().get::<SheepTerrain>(e).unwrap().state, STATE_GROUND);
        assert!(y(&dry, e) < 3.0);
        let mut wet = setup(0.5, 0.0);
        wet.world_mut().resource_mut::<WorldRaster>().composite.fill(255);
        let t = sheep(&mut wet, STATE_TRANSITION, 3.0);
        wet.update();
        assert_eq!(wet.world().get::<SheepTerrain>(t).unwrap().state, STATE_TRANSITION);
    }
}
