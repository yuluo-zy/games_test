//! Component structs and layouts (critters-behavior-analysis 5 "状态结构字段布局汇总").

use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};

/// bevy `Transform` subset: rotation Quat @+0, translation @+0x10 (stride 0x30; scale unused here).
/// See env.rs for why this is a local type.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub rotation: Quat,
    pub translation: Vec3,
}

impl Transform {
    pub fn at(translation: Vec3) -> Self {
        Self { rotation: Quat::IDENTITY, translation }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::at(Vec3::ZERO)
    }
}

/// CritterGoal (0x10): `active u32 @+0`, `pos Vec3 @+4`.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct CritterGoal {
    pub active: bool,
    pub pos: Vec3,
}

impl CritterGoal {
    pub fn set(pos: Vec3) -> Self {
        Self { active: true, pos }
    }
}

/// CritterVelocityTarget (0x10): `dir Vec3 @+0`, `retention_per_second f32 @+0xc`.
/// UNKNOWN(evidence: critters-behavior-analysis 6): where `retention_per_second` is initialised
/// (no recovered system writes it); it is supplied at spawn.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CritterVelocityTarget {
    pub dir: Vec3,
    pub retention_per_second: f32,
}

/// Velocity (0xc): x@+0, y@+4, z@+8.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Velocity(pub Vec3);

/// CritterVelocityMultiplier (4 B). `Option<&CritterVelocityMultiplier>` absent counts as 1.0.
/// UNKNOWN(evidence: critters-behavior-analysis 6): the "query env u32" writes of duck/shared systems.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CritterVelocityMultiplier(pub f32);

impl Default for CritterVelocityMultiplier {
    fn default() -> Self {
        Self(1.0)
    }
}

/// CritterLocomotion (8 B enum, u32 @+0). The raw value is kept; the per-value behaviours that the
/// disassembly proves are exposed as predicates.
/// UNKNOWN(evidence: critters-extra-analysis 6): value -> species mapping (duck/sheep/butterfly/...).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CritterLocomotion(pub u32);

impl CritterLocomotion {
    /// `== 2` -> terrain_rock_mask sample branch.
    pub fn samples_rock_mask(self) -> bool {
        self.0 == 2
    }
    /// `== 4` -> skip the water double-sample.
    pub fn skips_water_sample(self) -> bool {
        self.0 == 4
    }
    /// `== 3` -> skip PublicWalls::get.
    pub fn skips_public_walls(self) -> bool {
        self.0 == 3
    }
    /// `99u32 >> (v & 31) & 1` selects {0,1,5,6}: pure WallSpatialHash wall avoidance, no spherecast.
    pub fn wall_hash_only(self) -> bool {
        (99u32 >> (self.0 & 31)) & 1 == 1
    }
}

/// `Critter` marker of the shared spawn tuple.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Critter;

/// `ObjectGrassPush` (part of the duck spawn tuple; only appears in the move_ducks query).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ObjectGrassPush;

/// `BirdSpots` (wren perch points), optional in `move_critters`.
/// UNKNOWN(evidence: critters-behavior-analysis 2.1): stride 0x18 record contents / update rule.
#[derive(Component, Clone, Debug, Default)]
pub struct BirdSpots {
    pub spots: Vec<Vec3>,
}

/// Shared bundle every critter carries
/// (Critter, Transform, CritterLocomotion, CritterGoal, CritterVelocityTarget, Velocity, CritterVelocityMultiplier).
#[derive(Bundle, Clone, Debug)]
pub struct CritterBundle {
    pub critter: Critter,
    pub transform: Transform,
    pub locomotion: CritterLocomotion,
    pub goal: CritterGoal,
    pub vel_target: CritterVelocityTarget,
    pub velocity: Velocity,
    pub multiplier: CritterVelocityMultiplier,
}

impl CritterBundle {
    pub fn new(pos: Vec3, locomotion: u32, retention: f32) -> Self {
        Self {
            critter: Critter,
            transform: Transform::at(pos),
            locomotion: CritterLocomotion(locomotion),
            goal: CritterGoal::default(),
            vel_target: CritterVelocityTarget { dir: Vec3::ZERO, retention_per_second: retention },
            velocity: Velocity(Vec3::ZERO),
            multiplier: CritterVelocityMultiplier::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locomotion_mask_selects_0_1_5_6() {
        let set: Vec<u32> = (0..32).filter(|&v| CritterLocomotion(v).wall_hash_only()).collect();
        assert_eq!(set, vec![0, 1, 5, 6]);
    }

    #[test]
    fn locomotion_predicates() {
        assert!(CritterLocomotion(2).samples_rock_mask());
        assert!(CritterLocomotion(4).skips_water_sample());
        assert!(CritterLocomotion(3).skips_public_walls());
        assert!(!CritterLocomotion(7).wall_hash_only());
    }

    #[test]
    fn goal_and_bundle_spawn_into_a_world() {
        assert!(CritterGoal::set(Vec3::X).active);
        assert!(!CritterGoal::default().active);
        let mut world = World::new();
        let e = world.spawn(CritterBundle::new(Vec3::ONE, 0, 0.5)).id();
        assert_eq!(world.get::<Transform>(e).unwrap().translation, Vec3::ONE);
        assert_eq!(world.get::<CritterVelocityMultiplier>(e).unwrap().0, 1.0);
        assert_eq!(world.get::<CritterVelocityTarget>(e).unwrap().retention_per_second, 0.5);
        assert_eq!(Transform::default().translation, Vec3::ZERO);
    }
}
