use bevy_ecs::prelude::Resource;
use glam::Vec2;

/// Screen-space f32 position read by B 0x140b17a80. This is distinct from the
/// world-space CursorTerrainRaycast, and from integer WindowSize dimensions.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct CursorPositionSS(pub Vec2);
