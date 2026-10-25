//! shape_intersections.rs — 形状求交缓存（WP21）。
//!
//! O：PDB 实体 `ShapeIntersections`；`impl$0::default` 内联。U。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `ShapeIntersections`（O 名；形状求交缓存，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ShapeIntersections(pub Gap);
