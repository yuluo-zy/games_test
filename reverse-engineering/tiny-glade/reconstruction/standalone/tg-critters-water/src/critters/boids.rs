//! `Boid::frame_update` (boids.rs 329-387), `Boid::random_vel_change` (389-403, 0x141dc2170),
//! `BoidFlock::remove` (114-124). Evidence: critters-extra-analysis 1.2-1.5 (fish / wren instances
//! decompile identically; only the call-site constants differ).

use crate::rng::FastRng;
use crate::sample::uniform_sample_cone;
use bevy_ecs::prelude::{Entity, Resource};
use glam::Vec3;
use lerp::Lerp;

/// Neighbour radius 4.0 (dist^2 < 16.0, 0x41800000).
pub const NEIGHBOR_RADIUS_SQ: f32 = 16.0;
/// Cohesion factor 0.0056 (0x3ba3d70a), scaled by |vel|.
pub const COHESION: f32 = 0.0056;
/// Separation weight 1.7 (0x3fd9999a).
pub const SEPARATION_WEIGHT: f32 = 1.7;
/// Velocity-change clamp max (1.0; 1.5 with a goal); lower bound is `-max`, upper `max / 2`.
pub const ACCEL_CLAMP: f32 = 1.0;
pub const ACCEL_CLAMP_WITH_GOAL: f32 = 1.5;

/// Boid (0x18): pos + vel; `turn_timer` is the random_vel_change timer (+0x30 of the entity record).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boid {
    pub pos: Vec3,
    pub vel: Vec3,
    pub turn_timer: f32,
}

impl Boid {
    pub fn new(pos: Vec3, vel: Vec3) -> Self {
        Self { pos, vel, turn_timer: 0.0 }
    }
}

/// Call-site constants (critters-extra-analysis 1.3).
#[derive(Clone, Copy, Debug)]
pub struct BoidParams {
    pub sep_radius: f32,
    pub align_weight: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub has_goal: bool,
}

/// `(4 - 2t) t^2` — documented as the separation "smoothstep" term.
pub fn separation_curve(t: f32) -> f32 {
    (4.0 - 2.0 * t) * t * t
}

/// New state of boid `i` given the whole flock snapshot. Neighbours: dist^2 < 16 and
/// `self.vel . n.vel >= 0`. (The second documented test `cos >= -1` is always true: omitted.)
///
/// Interpretation: the "acceleration clamp" is applied per component to `new_vel - old_vel`
/// (UNKNOWN(evidence: critters-extra-analysis 1.2): the exact scope of that clamp).
pub fn frame_update(i: usize, flock: &[Boid], p: &BoidParams, dt: f32) -> Boid {
    let me = flock[i];
    let (mut count, mut sum_vel, mut sum_pos, mut sep) = (0u32, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
    for (j, n) in flock.iter().enumerate() {
        if j == i {
            continue;
        }
        let d = n.pos - me.pos;
        let d2 = d.length_squared();
        if d2 < NEIGHBOR_RADIUS_SQ && me.vel.dot(n.vel) >= 0.0 {
            count += 1;
            sum_vel += n.vel;
            sum_pos += n.pos;
            if d2 < p.sep_radius * p.sep_radius {
                let t = 1.0 - d2.sqrt() / p.sep_radius;
                sep -= d.normalize_or_zero() * separation_curve(t);
            }
        }
    }
    let mut vel = me.vel;
    if count > 0 {
        let c = count as f32;
        vel = (sum_vel / c) * p.align_weight + vel * (1.0 - p.align_weight);
        vel += ((sum_pos / c) - me.pos).normalize_or_zero() * vel.length() * COHESION;
    }
    vel += sep * SEPARATION_WEIGHT;
    let m = if p.has_goal { ACCEL_CLAMP_WITH_GOAL } else { ACCEL_CLAMP };
    let delta = (vel - me.vel).clamp(Vec3::splat(-m), Vec3::splat(m * 0.5));
    vel = me.vel + delta;
    vel = vel.normalize_or_zero() * vel.length().clamp(p.min_speed, p.max_speed);
    if !vel.is_finite() {
        vel = Vec3::ZERO; // NaN guard (0x7f7fffff compare)
    }
    Boid { pos: me.pos + vel * dt, vel, turn_timer: me.turn_timer }
}

/// One flocking step for every boid (all read the same snapshot).
pub fn flock_step(boids: &mut [Boid], p: &BoidParams, dt: f32) {
    let snap = boids.to_vec();
    for (i, b) in boids.iter_mut().enumerate() {
        *b = frame_update(i, &snap, p, dt);
    }
}

/// UNKNOWN(evidence: critters-extra-analysis 1.4): cone half-angle and the timer `lerp` range.
#[derive(Resource, Clone, Copy, Debug)]
pub struct RandomTurn {
    pub cone_half_angle: f32,
    pub interval: (f32, f32),
}

/// `timer -= dt`; at <= 0: 2 `fastrand` draws -> cone direction (around the current heading,
/// interpretation), then 1 draw -> new timer via `lerp`; velocity is rotated onto the new direction
/// keeping its length.
pub fn random_vel_change(b: &mut Boid, dt: f32, cfg: RandomTurn, rng: &mut FastRng) {
    b.turn_timer -= dt;
    if b.turn_timer > 0.0 {
        return;
    }
    let (u1, u2) = (rng.f32(), rng.f32());
    let dir = uniform_sample_cone(u1, u2, b.vel, cfg.cone_half_angle);
    b.vel = dir * b.vel.length();
    b.turn_timer = cfg.interval.0.lerp(cfg.interval.1, rng.f32());
}

/// `BoidFlock`: boids plus the parallel entity records (the 0x38 slots).
#[derive(Clone, Debug, Default)]
pub struct BoidFlock {
    pub boids: Vec<Boid>,
    pub entities: Vec<Entity>,
}

impl BoidFlock {
    pub fn push(&mut self, boid: Boid, entity: Entity) {
        self.boids.push(boid);
        self.entities.push(entity);
    }

    pub fn len(&self) -> usize {
        self.boids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.boids.is_empty()
    }

    /// `BoidFlock::remove`: indices processed in descending order with `swap_remove` on both arrays;
    /// out-of-range and duplicate indices are ignored. Returns the removed entities (to despawn).
    pub fn remove(&mut self, indices: &[u32]) -> Vec<Entity> {
        let mut idx: Vec<usize> = indices.iter().map(|&i| i as usize).collect();
        idx.sort_unstable_by(|a, b| b.cmp(a));
        idx.dedup();
        let mut removed = Vec::new();
        let len = self.boids.len();
        for i in idx.into_iter().filter(|&i| i < len) {
            self.boids.swap_remove(i);
            removed.push(self.entities.swap_remove(i));
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample::uniform_sample_cone as cone;

    fn range(rng: &fastrand::Rng, lo: f32, hi: f32) -> f32 {
        lo.lerp(hi, rng.f32())
    }

    const P: BoidParams = BoidParams { sep_radius: 1.3, align_weight: 0.1169, min_speed: 7.0, max_speed: 14.0, has_goal: false };

    #[test]
    fn separation_curve_values() {
        assert_eq!(separation_curve(0.0), 0.0);
        assert_eq!(separation_curve(1.0), 2.0);
        assert!((separation_curve(0.5) - 0.75).abs() < 1e-6);
    }

    #[test]
    fn lone_boid_keeps_heading_and_clamps_speed() {
        let flock = [Boid::new(Vec3::ZERO, Vec3::new(3.0, 0.0, 0.0))];
        let b = frame_update(0, &flock, &P, 0.1);
        assert!((b.vel.length() - 7.0).abs() < 1e-5, "raised to min speed");
        assert!(b.vel.x > 0.0 && b.vel.y == 0.0);
        assert!((b.pos - b.vel * 0.1).length() < 1e-6);
        let fast = [Boid::new(Vec3::ZERO, Vec3::new(100.0, 0.0, 0.0))];
        assert!((frame_update(0, &fast, &P, 0.1).vel.length() - 14.0).abs() < 1e-4);
    }

    #[test]
    fn accel_delta_clamp_bounds_change() {
        let flock = [Boid::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0))];
        let b = frame_update(0, &flock, &P, 0.1);
        assert!((b.vel - flock[0].vel).abs().max_element() <= 1.0 + 1e-5);
        let slow = [Boid::new(Vec3::ZERO, Vec3::new(0.5, 0.0, 0.0))];
        let b = frame_update(0, &slow, &P, 0.1);
        // clamp upper bound is max/2 = 0.5 per component, then speed floor is applied
        assert!(b.vel.length() >= 7.0 - 1e-4);
    }

    #[test]
    fn opposite_heading_is_not_a_neighbour() {
        let a = Boid::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        let same = Boid::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0));
        let opp = Boid::new(Vec3::new(0.0, 0.0, 2.0), Vec3::new(-10.0, 0.0, 0.0));
        let with_opp = frame_update(0, &[a, opp], &P, 0.1);
        let alone = frame_update(0, &[a], &P, 0.1);
        assert_eq!(with_opp, alone);
        let with_same = frame_update(0, &[a, same], &P, 0.1);
        assert_ne!(with_same.vel, alone.vel);
    }

    #[test]
    fn close_neighbour_pushes_away_far_does_not() {
        let a = Boid::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0));
        let near = Boid::new(Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 10.0));
        let far = Boid::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 10.0));
        let n = frame_update(0, &[a, near], &P, 0.1);
        let f = frame_update(0, &[a, far], &P, 0.1);
        assert!(n.vel.x < 0.0, "pushed to -x away from the neighbour at +x");
        assert!(f.vel.x > 0.0, "only cohesion toward +x beyond sep radius");
    }

    #[test]
    fn flock_cohesion_metric_improves() {
        let rng = fastrand::Rng::with_seed(11);
        let mut boids: Vec<Boid> = (0..30)
            .map(|_| Boid::new(Vec3::new(range(&rng, -3.0, 3.0), 0.0, range(&rng, -3.0, 3.0)), Vec3::new(8.0, 0.0, 0.0)))
            .collect();
        let spread = |b: &[Boid]| {
            let c = b.iter().map(|x| x.pos).sum::<Vec3>() / b.len() as f32;
            b.iter().map(|x| x.pos.distance(c)).sum::<f32>() / b.len() as f32
        };
        let before = spread(&boids);
        for _ in 0..100 {
            flock_step(&mut boids, &P, 0.02);
        }
        assert!(boids.iter().all(|b| b.vel.is_finite()));
        assert!(spread(&boids) < before * 2.0, "flock does not explode");
    }

    #[test]
    fn alignment_increases_in_a_cluster() {
        let rng = fastrand::Rng::with_seed(7);
        let mut b: Vec<Boid> = (0..40)
            .map(|_| {
                let a = range(&rng, -1.0, 1.0);
                Boid::new(Vec3::new(range(&rng, -3.0, 3.0), 0.0, range(&rng, -3.0, 3.0)), Vec3::new(a.cos(), 0.0, a.sin()) * 10.0)
            })
            .collect();
        let align = |b: &[Boid]| (b.iter().map(|x| x.vel.normalize_or_zero()).sum::<Vec3>() / b.len() as f32).length();
        let before = align(&b);
        for _ in 0..300 {
            flock_step(&mut b, &P, 1.0 / 60.0);
        }
        assert!(align(&b) > before && align(&b) > 0.95);
    }

    #[test]
    fn nan_velocity_zeroed() {
        let flock = [Boid::new(Vec3::ZERO, Vec3::new(f32::NAN, 0.0, 0.0))];
        assert_eq!(frame_update(0, &flock, &P, 0.1).vel, Vec3::ZERO);
    }

    #[test]
    fn random_turn_waits_then_redirects_with_the_fastrand_stream() {
        let cfg = RandomTurn { cone_half_angle: 0.4, interval: (1.0, 3.0) };
        let mut b = Boid::new(Vec3::ZERO, Vec3::new(8.0, 0.0, 0.0));
        b.turn_timer = 0.5;
        let mut rng = FastRng::seeded(21);
        random_vel_change(&mut b, 0.1, cfg, &mut rng);
        assert_eq!(b.vel, Vec3::new(8.0, 0.0, 0.0));
        random_vel_change(&mut b, 0.5, cfg, &mut rng);
        let mirror = fastrand::Rng::with_seed(21);
        let (u1, u2, u3) = (mirror.f32(), mirror.f32(), mirror.f32());
        let expect = cone(u1, u2, Vec3::new(8.0, 0.0, 0.0), 0.4) * 8.0;
        assert!((b.vel - expect).length() < 1e-4);
        assert!((b.vel.length() - 8.0).abs() < 1e-4);
        assert!(b.vel.normalize().dot(Vec3::X) >= 0.4_f32.cos() - 1e-4);
        assert_eq!(b.turn_timer, 1.0f32.lerp(3.0, u3));
        assert!((1.0..=3.0).contains(&b.turn_timer));
    }

    #[test]
    fn flock_remove_swap_semantics() {
        let id = |n: u32| Entity::from_raw(100 + n);
        let mut f = BoidFlock::default();
        for e in 0..5u32 {
            f.push(Boid::new(Vec3::splat(e as f32), Vec3::ZERO), id(e));
        }
        let removed = f.remove(&[1, 3, 3, 99]);
        assert_eq!(removed, vec![id(3), id(1)]);
        assert_eq!(f.len(), 3);
        assert_eq!(f.entities.len(), f.boids.len());
        assert!(f.entities.iter().all(|e| ![id(1), id(3)].contains(e)));
        assert!(!f.is_empty());
    }
}
