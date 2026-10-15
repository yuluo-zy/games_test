//! B 0x140a03ba0 coalesces all unread stroke changes into one notification.
use super::{
    delayed_terrain_changed_msg::ScheduledTerrainReadback, PlayTerrainSound,
    RecalculateHeightMapCmd,
};
use crate::resources::terrain_editing::StrokeId;
use bevy_ecs::prelude::*;

/// B TerrainEdit::reapply 0x1408592d0 writes the source stroke ID as payload.
#[derive(Event, Clone, Copy, Debug)]
pub struct EvTerrainStrokeChanged(pub StrokeId);

pub fn terrain_stroke_changed(
    mut changed: EventReader<EvTerrainStrokeChanged>,
    readback: Res<ScheduledTerrainReadback>,
    mut recalculate: EventWriter<RecalculateHeightMapCmd>,
    mut audio: EventWriter<PlayTerrainSound>,
) {
    if changed.read().last().is_some() {
        recalculate.write(RecalculateHeightMapCmd);
        readback.on_terrain_modified();
        audio.write(PlayTerrainSound::StrokeChanged);
    }
}
