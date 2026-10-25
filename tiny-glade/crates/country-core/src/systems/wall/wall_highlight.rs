//! B 0x140981390. Reset the kind, consume only the final unread command, then
//! resolve the wall entity. Missing targets leave the previous IDs/color intact.
use bevy_ecs::{
    event::{Event, EventReader},
    system::{Res, ResMut},
};
use glam::Vec3;

use crate::resources::{
    editing_highlight::EditingHighlight,
    walls::{WallHighlight, WallId, public_walls::WallEntities},
};

#[derive(Event, Clone, Copy, Debug)]
pub struct HighlightWallCmd {
    pub wall: WallId,
    pub ty: u8,
}

pub fn wall_highlight(
    mut highlight: ResMut<WallHighlight>,
    mut editing: ResMut<EditingHighlight>,
    mut reader: EventReader<HighlightWallCmd>,
    entities: Res<WallEntities>,
) {
    highlight.ty = 0;
    let Some(command) = reader.read().last() else {
        return;
    };
    let Some(entity) = entities.wall_entity(command.wall) else {
        return;
    };
    highlight.wall_entity = Some(entity);
    highlight.wall_id = Some(command.wall);
    highlight.ty = command.ty;
    editing.color = if command.ty == 1 {
        Vec3::new(
            f32::from_bits(0x3dcc_cccd),
            f32::from_bits(0x3cf5_c290),
            0.0,
        )
    } else {
        Vec3::new(
            f32::from_bits(0x3d8f_5c29),
            f32::from_bits(0x3da3_d70b),
            f32::from_bits(0x3d8f_5c29),
        )
    };
}
