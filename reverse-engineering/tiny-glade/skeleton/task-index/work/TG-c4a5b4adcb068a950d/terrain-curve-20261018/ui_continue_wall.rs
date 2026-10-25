//! Existing-wall continuation is a separate original producer. Only its
//! suppression flag written by ui_draw_wall is restored in this batch.
use bevy_ecs::prelude::Resource;

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SuppressContinueWall {
    /// Original +8: set on draw completion; its later reset belongs to the
    /// original continuation input consumer, not to a fabricated UI timeout.
    pub suppressed: bool,
}
