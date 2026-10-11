//! B firefly business rules, recovered from PDB + original instruction ranges.
//! Owner components live in country-core; these semantic structs are adapters, not ABI layouts.
use glam::Vec3;
use perchance::PerchanceContext;

/// Original plugin passes raw Perchance state 0xc0df (B 0x14067d537).
/// Perchance 0.4.0 `new` expands a seed as `(seed << 1) | 1`.
pub const CRITTER_RNG_SEED_B: u128 = 0x606f;
pub const FIREFLY_COUNT_B: usize = 96;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Firefly {
    pub goal_timer: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityTarget {
    pub direction: Vec3,
    pub retention_per_second: f32,
}

/// CPU projection sent to FireflySsbo; exact field attribution is separately recorded.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FireflyInstance {
    pub position: Vec3,
    pub fourth: f32,
}

/// Definition-owned utils::uniform_sample_sphere at B 0x140c907d0.
/// Public trig remains the standard-library implementation; no math library reimplementation.
pub fn sample_sphere_game(u: f32, v: f32) -> Vec3 {
    let z = 1.0 - (u + u);
    let radial_squared = 1.0 - z * z;
    let radial = if radial_squared <= 0.0 { 0.0 } else { radial_squared }.sqrt();
    let angle = v * std::f32::consts::TAU;
    let sine = angle.sin();
    let cosine = angle.cos();
    Vec3::new(radial * cosine, radial * sine, z)
}

/// B 0x142316d79..0x142316fc2. A selected entity from the actual query.
/// With control disabled, the caller performs no mutation and consumes no random values.
/// `goal` is the semantic Option; ABI preserved payload bytes are covered by the native oracle.
pub fn update_goal(
    firefly: &mut Firefly,
    goal: &mut Option<Vec3>,
    position: Vec3,
    velocity: Vec3,
    dt: f32,
    control_enabled: bool,
    rng: &mut PerchanceContext,
) {
    if !control_enabled { return; }
    firefly.goal_timer -= dt;
    let arrived = if let Some(target) = *goal {
        let d = target - position;
        // Original SSE adds y² + x², then z² (not an approximate distance).
        (d.y * d.y + d.x * d.x) + d.z * d.z < f32::from_bits(0x3c23d70b)
    } else { true };
    if !(firefly.goal_timer <= 0.0 || arrived) { return; }
    let inertia = if goal.is_none() { -0.5 } else { 0.5 };
    // Three calls, strictly in this order; the shared resource is not per-firefly.
    firefly.goal_timer = rng.uniform_f32() * 0.15 + 0.05;
    let u = rng.uniform_f32();
    let v = rng.uniform_f32();
    let sample = sample_sphere_game(u, v);
    let biased = Vec3::new(
        velocity.x * inertia + sample.x,
        sample.y * 0.5,
        velocity.z * inertia + sample.z,
    );
    let direction = biased.normalize_or_zero();
    let proposed = Vec3::new(
        firefly.goal_timer * direction.x + position.x,
        firefly.goal_timer * direction.y + position.y,
        direction.z * firefly.goal_timer + position.z,
    );
    let radius = (proposed.z * proposed.z + proposed.x * proposed.x).sqrt();
    let old_radius = (position.z * position.z + position.x * position.x).sqrt();
    *goal = if radius < 65.0 || radius < old_radius { Some(proposed) } else { None };
}

/// B 0x142317430..0x142317552: no random calls, multiplier is always reset to 1.
/// The altitude floor modifies the normalized direction without renormalizing it.
pub fn set_velocity_target(position: Vec3, goal: Option<Vec3>, target: &mut VelocityTarget, multiplier: &mut f32) {
    let d = goal.unwrap_or(position) - position;
    target.direction = d.normalize_or_zero();
    *multiplier = 1.0;
    if position.y < 1.0 {
        target.direction.y = target.direction.y.max(0.25);
    }
}

/// B shared::smooth_critter_velocities at 0x142318c10.
/// Original order is old * retention + target * (1-retention), not lerp's reordered expression.
pub fn smooth_velocity(velocity: &mut Vec3, target: VelocityTarget, dt: f32) {
    let retention = target.retention_per_second.powf(dt);
    let weight = 1.0 - retention;
    *velocity = Vec3::new(
        velocity.x * retention + target.direction.x * weight,
        velocity.y * retention + target.direction.y * weight,
        velocity.z * retention + target.direction.z * weight,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paused_goal_owner_and_rng_are_untouched() {
        let mut f=Firefly{goal_timer:2.0}; let mut goal=None;
        let mut rng=PerchanceContext::new(CRITTER_RNG_SEED_B); let mut expected=rng.clone();
        update_goal(&mut f,&mut goal,Vec3::ZERO,Vec3::ZERO,1.0,false,&mut rng);
        assert_eq!(f.goal_timer,2.0);assert_eq!(goal,None);assert_eq!(rng.get_u128(),expected.get_u128());
    }
    #[test]
    fn missing_goal_low_altitude_is_upward_without_normalization() {
        let mut t=VelocityTarget{direction:Vec3::ONE,retention_per_second:0.05};let mut m=0.0;
        set_velocity_target(Vec3::ZERO,None,&mut t,&mut m);
        assert_eq!(t.direction,Vec3::new(0.0,0.25,0.0));assert_eq!(m,1.0);
    }
}
