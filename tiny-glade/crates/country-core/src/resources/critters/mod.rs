//! Original B critter data. Species owners are added only with recovered fields.
pub mod sheep;

use bevy_ecs::component::Component;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Native 16-byte component; the snapshot serializer proves Option<Vec3>.
/// An absent goal has no invented position payload.
#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CritterGoal(pub Option<Vec3>);
