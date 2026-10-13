//! shared.rs systems: `smooth_critter_velocities` (0x142318c10), `move_critters` (0x142318fa0),
//! `critters_avoid_collisions` (0x1423179a0). Evidence: critters-behavior-analysis 2.

use super::components::*;
use super::duck::Duck;
use super::sheep::Sheep;
use crate::env::{RaycastWorld, Time, WallSpatialHash};
use crate::water::WorldRaster;
use bevy_ecs::prelude::*;
use glam::{Quat, Vec2, Vec3};

/// Rotation decay base: `powf(0.01, dt)` (move_critters L223).
pub const ROTATION_DECAY_BASE: f32 = 0.01;
/// Spherecast hit distance below which movement is skipped (`< 1.0`).
pub const SPHERECAST_BLOCK_DIST: f32 = 1.0;
/// Hit distance below which a duck is despawned (`< 0.1`, avoid_collisions L179).
pub const DUCK_DESPAWN_DIST: f32 = 0.1;
/// UNKNOWN(evidence: critters-behavior-analysis 2.1): the model forward axis used by
/// `Quat::from_rotation_arc` (0x140d4ae30). +Z is an assumption.
pub const FORWARD_AXIS: Vec3 = Vec3::Z;

/// `DespawnDuckCmd` event written by critters_avoid_collisions (L179). The consumer chain is
/// UNKNOWN(evidence: critters-behavior-analysis 6); see `despawn_duck_on_cmd`.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DespawnDuckCmd {
    pub entity: Entity,
}

/// `velocity = velocity * r + dir * (1 - r)`, `r = retention_per_second.powf(dt)`.
/// The multiplier write (`query env u32`) is UNKNOWN(evidence: critters-behavior-analysis 6).
pub fn smooth_critter_velocities(
    time: Res<Time>,
    mut critters: Query<(&CritterVelocityTarget, &mut Velocity, &mut CritterVelocityMultiplier)>,
) {
    let dt = time.delta_secs();
    for (target, mut velocity, _multiplier) in &mut critters {
        let r = target.retention_per_second.powf(dt);
        velocity.0 = velocity.0 * r + target.dir * (1.0 - r);
    }
}

fn ccw(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

/// 2D segment-segment intersection (`LineInterval::line_segment` + `relate`).
pub fn segments_intersect(p1: Vec2, p2: Vec2, q1: Vec2, q2: Vec2) -> bool {
    let (d1, d2) = (ccw(p1, p2, q1), ccw(p1, p2, q2));
    let (d3, d4) = (ccw(q1, q2, p1), ccw(q1, q2, p2));
    d1 * d2 < 0.0 && d3 * d4 < 0.0
}

/// A wall (taller than the critter: `max_y_gable >= pos.y`) crosses `from -> to` (xz).
pub fn wall_blocks(walls: &WallSpatialHash, from: Vec3, to: Vec3) -> bool {
    let (a, b) = (Vec2::new(from.x, from.z), Vec2::new(to.x, to.z));
    let radius = a.distance(b) * 0.5 + 0.5;
    walls
        .query((a + b) * 0.5, radius)
        .iter()
        .any(|w| w.max_y_gable >= from.y && segments_intersect(a, b, w.a, w.b))
}

/// move_critters. Per critter: clamp y >= 0, `disp = velocity * dt * multiplier`, skip when a wall or
/// (non wall-hash-only locomotion) a spherecast within 1.0 blocks, else apply and blend rotation.
///
/// UNKNOWN(evidence: critters-behavior-analysis 2.1): what the rock-mask (`locomotion == 2`) and
/// the water double sample (`!= 4`) decide, and BirdSpots handling; none is built on here (the
/// raster, Duck, Sheep and BirdSpots parameters are part of the documented query but unused).
#[allow(clippy::type_complexity)]
pub fn move_critters(
    time: Res<Time>,
    _water: Res<WorldRaster>,
    raycast: Res<RaycastWorld>,
    walls: Res<WallSpatialHash>,
    mut critters: Query<(
        &mut Transform,
        &CritterLocomotion,
        &Velocity,
        Option<&CritterVelocityMultiplier>,
        Option<&Duck>,
        Option<&Sheep>,
        Option<&mut BirdSpots>,
    )>,
) {
    let dt = time.delta_secs();
    let rot_k = ROTATION_DECAY_BASE.powf(dt);
    for (mut transform, locomotion, velocity, multiplier, _duck, _sheep, _bird_spots) in &mut critters {
        let pos = &mut transform.translation;
        pos.y = pos.y.max(0.0);
        let disp = velocity.0 * dt * multiplier.map_or(1.0, |m| m.0);
        let to = *pos + disp;
        let blocked = wall_blocks(&walls, *pos, to)
            || (!locomotion.wall_hash_only()
                && raycast.spherecast(*pos, to).is_some_and(|d| d < SPHERECAST_BLOCK_DIST));
        if blocked {
            continue;
        }
        *pos = to;
        let flat = Vec3::new(velocity.0.x, 0.0, velocity.0.z);
        if flat.length_squared() > 1e-12 {
            let target = Quat::from_rotation_arc(FORWARD_AXIS, flat.normalize());
            let rot = transform.rotation * rot_k + target * (1.0 - rot_k);
            transform.rotation = rot.normalize();
        }
    }
}

/// critters_avoid_collisions: only critters with an active goal; a wall between pos and goal clears
/// the goal; a duck's spherecast hit within 0.1 emits `DespawnDuckCmd`.
/// UNKNOWN(evidence: critters-behavior-analysis 2.2): the 8x water-sample sum threshold and the
/// meaning of the final `*goal.active = 0` for the non-wall path; only the wall path clears here.
pub fn critters_avoid_collisions(
    _water: Res<WorldRaster>,
    raycast: Res<RaycastWorld>,
    walls: Res<WallSpatialHash>,
    mut critters: Query<(Entity, &Transform, &mut CritterGoal, Option<&Duck>)>,
    mut despawns: EventWriter<DespawnDuckCmd>,
) {
    for (entity, transform, mut goal, duck) in &mut critters {
        if !goal.active {
            continue;
        }
        let pos = transform.translation;
        if wall_blocks(&walls, pos, goal.pos) {
            goal.active = false;
            continue;
        }
        if duck.is_some() && raycast.spherecast(pos, goal.pos).is_some_and(|d| d < DUCK_DESPAWN_DIST) {
            despawns.write(DespawnDuckCmd { entity });
        }
    }
}

/// Assumed consumer of `DespawnDuckCmd`.
/// UNKNOWN(evidence: critters-behavior-analysis 6): the original consumer chain is not read;
/// despawning the named entity is the minimal reading of "撤销鸭子".
pub fn despawn_duck_on_cmd(mut commands: Commands, mut cmds: EventReader<DespawnDuckCmd>) {
    for cmd in cmds.read() {
        if let Ok(mut e) = commands.get_entity(cmd.entity) {
            e.despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::WallSegment;
    use crate::testing::base_app;
    use bevy_app::{App, Update};

    fn critter(app: &mut App, pos: Vec3, loco: u32, retention: f32) -> Entity {
        app.world_mut().spawn(CritterBundle::new(pos, loco, retention)).id()
    }

    fn wall(x: f32, gable: f32) -> WallSegment {
        WallSegment { a: Vec2::new(x, -5.0), b: Vec2::new(x, 5.0), max_y_gable: gable }
    }

    #[test]
    fn smoothing_converges_to_target() {
        let mut app = base_app(0.05);
        app.add_systems(Update, smooth_critter_velocities);
        let e = critter(&mut app, Vec3::ZERO, 4, 0.1);
        app.world_mut().get_mut::<CritterVelocityTarget>(e).unwrap().dir = Vec3::X;
        for _ in 0..200 {
            app.update();
        }
        assert!((app.world().get::<Velocity>(e).unwrap().0 - Vec3::X).length() < 1e-3);
    }

    #[test]
    fn smoothing_exact_one_step() {
        let mut app = base_app(1.0);
        app.add_systems(Update, smooth_critter_velocities);
        let e = critter(&mut app, Vec3::ZERO, 4, 0.25);
        app.world_mut().get_mut::<Velocity>(e).unwrap().0 = Vec3::new(2.0, 0.0, 0.0);
        app.world_mut().get_mut::<CritterVelocityTarget>(e).unwrap().dir = Vec3::new(0.0, 0.0, 4.0);
        app.update();
        assert!((app.world().get::<Velocity>(e).unwrap().0 - Vec3::new(0.5, 0.0, 3.0)).length() < 1e-6);
    }

    #[test]
    fn move_applies_velocity_dt_multiplier() {
        let mut app = base_app(0.5);
        app.add_systems(Update, move_critters);
        let e = critter(&mut app, Vec3::new(0.0, -2.0, 0.0), 4, 0.5);
        app.world_mut().get_mut::<Velocity>(e).unwrap().0 = Vec3::new(2.0, 0.0, 0.0);
        app.world_mut().get_mut::<CritterVelocityMultiplier>(e).unwrap().0 = 0.5;
        app.update();
        let p = app.world().get::<Transform>(e).unwrap().translation;
        assert!((p - Vec3::new(0.5, 0.0, 0.0)).length() < 1e-6, "y clamped to 0, got {p:?}");
    }

    #[test]
    fn missing_multiplier_counts_as_one() {
        let mut app = base_app(1.0);
        app.add_systems(Update, move_critters);
        let e = critter(&mut app, Vec3::ZERO, 4, 0.5);
        app.world_mut().get_mut::<Velocity>(e).unwrap().0 = Vec3::X;
        app.world_mut().entity_mut(e).remove::<CritterVelocityMultiplier>();
        app.update();
        assert_eq!(app.world().get::<Transform>(e).unwrap().translation.x, 1.0);
    }

    #[test]
    fn rotation_blends_toward_heading() {
        let mut app = base_app(0.1);
        app.add_systems(Update, move_critters);
        let e = critter(&mut app, Vec3::ZERO, 4, 0.5);
        app.world_mut().get_mut::<Velocity>(e).unwrap().0 = Vec3::X;
        for _ in 0..200 {
            app.update();
        }
        let rot = app.world().get::<Transform>(e).unwrap().rotation;
        assert!((rot * FORWARD_AXIS).dot(Vec3::X) > 0.99);
        assert!((rot.length() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn spherecast_blocks_unless_wall_hash_only() {
        let mut app = base_app(1.0);
        app.add_systems(Update, move_critters);
        app.world_mut().resource_mut::<RaycastWorld>().spheres.push((Vec3::new(1.5, 0.0, 0.0), 1.0));
        let blocked = critter(&mut app, Vec3::ZERO, 4, 0.5);
        let free = critter(&mut app, Vec3::ZERO, 0, 0.5);
        for e in [blocked, free] {
            app.world_mut().get_mut::<Velocity>(e).unwrap().0 = Vec3::X;
        }
        app.update();
        assert_eq!(app.world().get::<Transform>(blocked).unwrap().translation.x, 0.0);
        assert_eq!(app.world().get::<Transform>(free).unwrap().translation.x, 1.0);
    }

    #[test]
    fn wall_blocks_by_gable_height() {
        let walls = WallSpatialHash { walls: vec![wall(0.5, 3.0)] };
        assert!(wall_blocks(&walls, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)));
        assert!(!wall_blocks(&walls, Vec3::new(0.0, 4.0, 0.0), Vec3::new(1.0, 4.0, 0.0)), "critter above gable");
        assert!(!wall_blocks(&walls, Vec3::ZERO, Vec3::new(0.2, 0.0, 0.0)));

        let mut app = base_app(1.0);
        app.add_systems(Update, move_critters);
        app.world_mut().resource_mut::<WallSpatialHash>().walls.push(wall(0.5, 3.0));
        let e = critter(&mut app, Vec3::ZERO, 0, 0.5);
        app.world_mut().get_mut::<Velocity>(e).unwrap().0 = Vec3::X;
        app.update();
        assert_eq!(app.world().get::<Transform>(e).unwrap().translation.x, 0.0);
    }

    #[test]
    fn avoid_clears_goal_behind_wall() {
        let mut app = base_app(0.016);
        app.add_systems(Update, critters_avoid_collisions);
        app.world_mut().resource_mut::<WallSpatialHash>().walls.push(wall(2.0, 3.0));
        let e = critter(&mut app, Vec3::ZERO, 0, 0.5);
        *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(5.0, 0.0, 0.0));
        app.update();
        assert!(!app.world().get::<CritterGoal>(e).unwrap().active);
        let events = app.world().resource::<Events<DespawnDuckCmd>>();
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn avoid_sends_despawn_event_for_ducks_only_and_consumer_despawns() {
        let mut app = base_app(0.016);
        app.add_systems(Update, (critters_avoid_collisions, despawn_duck_on_cmd).chain());
        app.world_mut().resource_mut::<RaycastWorld>().spheres.push((Vec3::new(5.05, 0.0, 0.0), 5.0));
        let duck = critter(&mut app, Vec3::ZERO, 0, 0.5);
        app.world_mut().entity_mut(duck).insert(Duck::free());
        let sheep = critter(&mut app, Vec3::ZERO, 0, 0.5);
        let idle = critter(&mut app, Vec3::ZERO, 0, 0.5);
        app.world_mut().entity_mut(idle).insert(Duck::free());
        for e in [duck, sheep] {
            *app.world_mut().get_mut::<CritterGoal>(e).unwrap() = CritterGoal::set(Vec3::new(5.0, 0.0, 0.0));
        }
        app.update();
        assert!(app.world().get_entity(duck).is_err(), "duck with a close hit despawned");
        assert!(app.world().get_entity(sheep).is_ok(), "non-duck untouched");
        assert!(app.world().get_entity(idle).is_ok(), "inactive goal ignored");
    }

    #[test]
    fn segment_intersection() {
        let (a, b) = (Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        assert!(segments_intersect(a, b, Vec2::new(0.0, 2.0), Vec2::new(2.0, 0.0)));
        assert!(!segments_intersect(a, b, Vec2::new(3.0, 0.0), Vec2::new(3.0, 5.0)));
    }
}
