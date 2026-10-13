//! Duck five-stage chain (critters-behavior-analysis 1):
//! plan (0x14230ff00) -> move along trajectories (0x142312200) -> update goals (0x142313b30)
//! -> set velocities towards goals (0x142314470) -> ripples (0x142314b70).
//! Spawn / despawn: critters-extra-analysis 5.2 / 5.3.

pub mod goals;
pub mod movement;
pub mod plan;
pub mod ripples;
pub mod spawn;
pub mod velocities;

use crate::critters::components::*;
pub use crate::critters::shared::DespawnDuckCmd;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// DuckState (1 B enum). critters-extra-analysis 6: 1 = along trajectory forward (at the end ->
/// state 2), 2 = free swimming (goals/velocities), 3 = along trajectory in reverse (at the start ->
/// despawn). UNKNOWN(evidence: critters-extra-analysis 7): meaning of value 0.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DuckState(pub u8);

impl DuckState {
    pub const TRAJ_FORWARD: Self = Self(1);
    pub const FREE: Self = Self(2);
    pub const TRAJ_REVERSE: Self = Self(3);
}

/// `Duck` component fields recovered from offsets (stride 0x100).
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Duck {
    /// +0xe0: 1 = "moving" window active (goal followed), 0 = resting.
    pub moving: bool,
    /// +0xe1 byte (cleared at the end of update_duck_goals; role UNKNOWN(evidence: critters-behavior-analysis 1.3)).
    pub e1: bool,
    /// +0xe4: two f32 timers.
    pub timers: [f32; 2],
    /// +0xe8: set to 0.1 whenever a new goal is chosen (role UNKNOWN).
    pub e8: f32,
    /// +0xfc == 1: freeze (velocity target zeroed).
    pub frozen: bool,
    /// +0x98/+0xa0: trajectory points (stride 0xc).
    pub trajectory: Vec<Vec3>,
    /// +0xec: progress along the trajectory in [0,1].
    pub progress: f32,
}

impl Duck {
    pub fn free() -> Self {
        Self { moving: false, e1: false, timers: [0.0; 2], e8: 0.0, frozen: false, trajectory: Vec::new(), progress: 0.0 }
    }
}

/// Spawn-time constants the evidence leaves open.
/// UNKNOWN(evidence: critters-extra-analysis 6/7): CritterLocomotion value for ducks, the initial
/// `retention_per_second` and the 3-point plan spread radius; placeholders below are demo values.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct DuckConfig {
    pub locomotion: u32,
    pub retention_per_second: f32,
    pub plan_spread: f32,
}

impl Default for DuckConfig {
    fn default() -> Self {
        Self { locomotion: 4, retention_per_second: 0.05, plan_spread: 3.0 }
    }
}

/// Spawn tuple of plan_duck (`Commands::spawn<(Duck, DuckState, DrawableMeshBundle, CritterLocomotion,
/// CritterGoal, CritterVelocityTarget, ObjectGrassPush, Velocity, CritterVelocityMultiplier,
/// PrevTransform)>`; mesh/prev-transform are render glue and omitted).
#[derive(Bundle, Clone, Debug)]
pub struct DuckBundle {
    pub duck: Duck,
    pub state: DuckState,
    pub body: CritterBundle,
    pub grass_push: ObjectGrassPush,
}

impl DuckBundle {
    pub fn new(pos: Vec3, state: DuckState, cfg: &DuckConfig) -> Self {
        Self {
            duck: Duck::free(),
            state,
            body: CritterBundle::new(pos, cfg.locomotion, cfg.retention_per_second),
            grass_push: ObjectGrassPush,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_values_and_bundle() {
        assert_eq!(DuckState::TRAJ_FORWARD.0, 1);
        assert_eq!(DuckState::FREE.0, 2);
        assert_eq!(DuckState::TRAJ_REVERSE.0, 3);
        let mut world = World::new();
        let cfg = DuckConfig { locomotion: 4, retention_per_second: 0.1, plan_spread: 3.0 };
        let e = world.spawn(DuckBundle::new(Vec3::ONE, DuckState::FREE, &cfg)).id();
        assert!(!world.get::<Duck>(e).unwrap().moving);
        assert_eq!(world.get::<CritterLocomotion>(e).unwrap().0, 4);
        assert!(world.get::<ObjectGrassPush>(e).is_some());
        assert_eq!(DuckConfig::default().locomotion, 4);
    }
}
