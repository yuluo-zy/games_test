//! critters module (critters-behavior-analysis.md, critters-extra-analysis.md).
//!
//! Every business function is a bevy_ecs 0.16 system over real components/resources;
//! [`CrittersPlugin`] registers them. Per-frame `dt` is `Res<Time>` (`*Res<Time> + 0x28`).

pub mod boids;
pub mod butterfly;
pub mod components;
pub mod duck;
pub mod firefly;
pub mod fish;
pub mod plugin;
pub mod sheep;
pub mod sheep_terrain;
pub mod shared;
pub mod wren;

pub use components::*;
pub use plugin::{CrittersPlugin, CrittersSet};
