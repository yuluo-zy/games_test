//! Shared wall construction algorithms and CPU input contracts.

pub mod arches;
pub mod bricks;
pub mod fences;
pub mod freehand_arches;
pub mod gates;
pub mod parameters;
pub mod platform_collision;

#[cfg(feature = "native-physics")]
pub mod platform_rapier_backend;
pub mod platform_segmentation;
pub mod platform_stair_rows;
pub mod platform_stairs;
pub mod production;
pub mod segmentation;
pub mod structures;

pub mod arch_stats;
