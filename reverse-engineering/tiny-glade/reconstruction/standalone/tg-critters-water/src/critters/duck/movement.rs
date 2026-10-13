//! Stage 2: move_ducks_along_trajectories (0x142312200, duck.rs 369-576).
//! `progress += dir * 12.0 * dt / curve_length`; state 1 forward (end -> state 2), state 3 reverse
//! (start -> despawn), state 2 skipped.
//! UNKNOWN(evidence: critters-behavior-analysis 1.2): the curve is `utils::curve::CurveInternalCoord`
//! (smooth); a plain polyline is used. Audio (water-grid dependent loops) is out of scope here.

use super::{Duck, DuckState};
use crate::critters::components::*;
use crate::critters::shared::FORWARD_AXIS;
use crate::env::Time;
use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};

/// `__real_41400000` = 12.0 world units per second along the trajectory.
pub const TRAJECTORY_SPEED: f32 = 12.0;
/// On arrival (state 1 -> 2) `+0xe0 = 1`, `+0xe4 = (2.0, 4.0)` (L422-423).
pub const ARRIVAL_TIMERS: [f32; 2] = [2.0, 4.0];

pub fn polyline_length(points: &[Vec3]) -> f32 {
    points.windows(2).map(|w| w[0].distance(w[1])).sum()
}

/// Position and unit tangent at fraction `t` of the total length.
pub fn point_at(points: &[Vec3], t: f32) -> Option<(Vec3, Vec3)> {
    let total = polyline_length(points);
    if points.len() < 2 || total <= 0.0 {
        return None;
    }
    let mut remaining = t.clamp(0.0, 1.0) * total;
    for w in points.windows(2) {
        let len = w[0].distance(w[1]);
        if remaining <= len || std::ptr::eq(&w[1], points.last().unwrap()) {
            let f = if len > 0.0 { (remaining / len).min(1.0) } else { 0.0 };
            return Some((w[0].lerp(w[1], f), (w[1] - w[0]).normalize_or_zero()));
        }
        remaining -= len;
    }
    None
}

/// Query `(Entity, &mut Duck, &mut DuckState, &mut Transform, &mut Velocity,
/// &mut CritterVelocityMultiplier, &mut CritterGoal, &mut ObjectGrassPush)` (init@0x14230eb00).
/// Velocity / multiplier / goal / grass-push writes are UNKNOWN(evidence: critters-behavior-analysis
/// 1.2) and are not performed.
#[allow(clippy::type_complexity)]
pub fn move_ducks_along_trajectories(
    mut commands: Commands,
    time: Res<Time>,
    mut ducks: Query<(
        Entity,
        &mut Duck,
        &mut DuckState,
        &mut Transform,
        &mut Velocity,
        &mut CritterVelocityMultiplier,
        &mut CritterGoal,
        &mut ObjectGrassPush,
    )>,
) {
    let dt = time.delta_secs();
    for (entity, mut duck, mut state, mut transform, _vel, _mult, _goal, _push) in &mut ducks {
        let dir = match *state {
            DuckState::FREE => continue,
            DuckState::TRAJ_FORWARD => 1.0,
            DuckState::TRAJ_REVERSE => -1.0,
            _ => continue, // UNKNOWN(evidence: critters-extra-analysis 7): DuckState 0
        };
        let len = polyline_length(&duck.trajectory);
        duck.progress = if len > 0.0 {
            (duck.progress + dir * TRAJECTORY_SPEED * dt / len).clamp(0.0, 1.0)
        } else if dir > 0.0 {
            1.0
        } else {
            0.0
        };
        if let Some((pos, tangent)) = point_at(&duck.trajectory, duck.progress) {
            transform.translation = pos;
            let flat = Vec3::new(tangent.x, 0.0, tangent.z) * dir;
            if flat.length_squared() > 1e-12 {
                transform.rotation = Quat::from_rotation_arc(FORWARD_AXIS, flat.normalize());
            }
        }
        if dir > 0.0 && duck.progress >= 1.0 {
            *state = DuckState::FREE;
            duck.moving = true;
            duck.e1 = false;
            duck.timers = ARRIVAL_TIMERS;
        }
        if dir < 0.0 && duck.progress <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::duck::{DuckBundle, DuckConfig};
    use crate::testing::base_app;
    use bevy_app::{App, Update};

    fn setup(dt: f32) -> App {
        let mut app = base_app(dt);
        app.add_systems(Update, move_ducks_along_trajectories);
        app
    }

    fn duck(app: &mut App, state: DuckState, progress: f32) -> Entity {
        let mut b = DuckBundle::new(Vec3::ZERO, state, &DuckConfig::default());
        b.duck.trajectory = vec![Vec3::ZERO, Vec3::new(24.0, 0.0, 0.0)];
        b.duck.progress = progress;
        app.world_mut().spawn(b).id()
    }

    #[test]
    fn polyline_helpers() {
        let pts = [Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 10.0)];
        assert_eq!(polyline_length(&pts), 20.0);
        let (p, t) = point_at(&pts, 0.75).unwrap();
        assert!((p - Vec3::new(10.0, 0.0, 5.0)).length() < 1e-5);
        assert!((t - Vec3::Z).length() < 1e-5);
        assert!(point_at(&pts[..1], 0.5).is_none());
        let (p, _) = point_at(&pts, 2.0).unwrap();
        assert!((p - pts[2]).length() < 1e-5);
    }

    #[test]
    fn forward_advances_at_12_per_second_and_arrives() {
        let mut app = setup(1.0);
        let e = duck(&mut app, DuckState::TRAJ_FORWARD, 0.0);
        app.update();
        assert!((app.world().get::<Duck>(e).unwrap().progress - 0.5).abs() < 1e-6);
        assert!((app.world().get::<Transform>(e).unwrap().translation.x - 12.0).abs() < 1e-4);
        assert_eq!(*app.world().get::<DuckState>(e).unwrap(), DuckState::TRAJ_FORWARD);
        app.update();
        let d = app.world().get::<Duck>(e).unwrap();
        assert_eq!(*app.world().get::<DuckState>(e).unwrap(), DuckState::FREE);
        assert!(d.moving);
        assert_eq!(d.timers, ARRIVAL_TIMERS);
        assert!((app.world().get::<Transform>(e).unwrap().translation.x - 24.0).abs() < 1e-4);
    }

    #[test]
    fn reverse_despawns_at_start() {
        let mut app = setup(1.0);
        let e = duck(&mut app, DuckState::TRAJ_REVERSE, 1.0);
        app.update();
        assert!((app.world().get::<Duck>(e).unwrap().progress - 0.5).abs() < 1e-6);
        app.update();
        assert!(app.world().get_entity(e).is_err());
    }

    #[test]
    fn free_and_unknown_states_skipped_and_degenerate_trajectory() {
        let mut app = setup(5.0);
        let a = duck(&mut app, DuckState::FREE, 0.3);
        let b = duck(&mut app, DuckState(0), 0.3);
        let mut z = DuckBundle::new(Vec3::ZERO, DuckState::TRAJ_REVERSE, &DuckConfig::default());
        z.duck.progress = 1.0;
        let z = app.world_mut().spawn(z).id();
        app.update();
        assert_eq!(app.world().get::<Duck>(a).unwrap().progress, 0.3);
        assert_eq!(app.world().get::<Duck>(b).unwrap().progress, 0.3);
        assert!(app.world().get_entity(z).is_err(), "empty reverse trajectory despawns");
    }

    #[test]
    fn rotation_follows_tangent() {
        let mut app = setup(0.1);
        let e = duck(&mut app, DuckState::TRAJ_FORWARD, 0.0);
        app.update();
        let rot = app.world().get::<Transform>(e).unwrap().rotation;
        assert!((rot * FORWARD_AXIS).dot(Vec3::X) > 0.999);
    }
}
