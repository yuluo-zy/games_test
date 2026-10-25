//! Original B critter data. Species owners are added only with recovered fields.
pub mod sheep;

use bevy_ecs::component::Component;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Native 16-byte critter component (O: stride 0x10 in the sheep control
/// query; capture 0x1412f03f0 / apply 0x1416f24f0 copy its full 16 bytes as
/// the snapshot's `position` + the goal `Option` tag).
///
/// Layout: `position: Vec3 @ +0x00` — the sheep's position, **not**
/// `Transform.translation` (which stores `SheepMoveState`, O); and
/// `goal_tag: u32 @ +0x0c` — the goal `Option<Vec3>` tag (0 = `None`,
/// non-zero = `Some`). The goal's Vec3 payload lives in `Sheep @ +0x30`.
#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CritterGoal {
    /// Native `+0x00` (O).
    pub position: Vec3,
    /// Native `+0x0c` (O): goal `Option` tag (0 = `None`, 1 = `Some`).
    pub goal_tag: u32,
}

