use bevy_ecs::prelude::Resource;
use glam::Vec2;

/// Native Option<Vec2>; the translator tests its discriminant, and keeps the
/// capture anchor owned by mouse capture rather than moving the window cursor.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct CursorCapturedPosition(pub Option<Vec2>);
