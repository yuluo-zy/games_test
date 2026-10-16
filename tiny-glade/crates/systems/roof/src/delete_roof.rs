//! 屋顶删除三系统（delete_roof.rs）。
//!
//! - `process_invalid_roof_notifications`（0x14062eb80）
//! - `mark_roof_for_deletion`（0x14062f3d0）
//! - `delete_roofs`（0x14062f8c0）
use country_core::recovered_building::{
    roof_deletion::{match_deletion_request, should_despawn},
    roof_invalid_notifications::{
        InvalidRoofRow, process_invalid_roof_notifications as invalid_roofs,
    },
    wall_state::WallId as ObservedWallId,
};
use country_core::resources::{
    roofs::{Roof, RoofAnimation, RoofMarkedForDeletion, commands::*},
    walls::{WallId, public_walls::PublicWalls},
};
use bevy_ecs::prelude::*;

/// B 0x14062eb80。非法屋顶策略保留重置优先级与历史载荷；RecordRoofHistory 是
/// 显式待提交变更，不是历史提交。
pub fn process_invalid_roof_notifications(
    roofs: Query<(&Roof, Has<RoofMarkedForDeletion>)>,
    mut invalid: EventReader<RoofIsInvalid>,
    mut resets: EventReader<ResetRoofs>,
    mut deletion: EventWriter<DeleteRoofCmd>,
    mut history: EventWriter<RecordRoofHistory>,
) {
    let ids: Vec<_> = invalid
        .read()
        .map(|event| ObservedWallId(event.0.0))
        .collect();
    let reset = resets.read().count() != 0;
    let rows: Vec<_> = roofs
        .iter()
        .map(|(roof, marked)| InvalidRoofRow {
            roof: roof.observed(),
            marked_for_deletion: marked,
        })
        .collect();
    for request in invalid_roofs(&rows, &ids, reset) {
        deletion.write(DeleteRoofCmd(WallId(request.wall_id.0)));
        if let Some(mutation) = request.history_mutation {
            history.write(RecordRoofHistory(mutation));
        }
    }
}

/// B 0x14062f3d0。
pub fn mark_roof_for_deletion(
    mut commands: Commands,
    mut requests: EventReader<DeleteRoofCmd>,
    roofs: Query<(Entity, &Roof), Without<RoofMarkedForDeletion>>,
    mut sound: EventWriter<RoofSound>,
) {
    let ids: Vec<_> = requests
        .read()
        .map(|event| ObservedWallId(event.0.0))
        .collect();
    for (entity, roof) in &roofs {
        if let Some(mark) = match_deletion_request(&ids, entity.to_bits(), roof.observed()) {
            commands.entity(entity).insert(RoofMarkedForDeletion);
            sound.write(RoofSound {
                entity,
                wall: roof.wall_id(),
                pivot: mark.pivot,
                kind: mark.sound_event_kind,
                flat_surface: None,
            });
        }
    }
}

/// B 0x14062f8c0。动画播放完毕的标记屋顶出队。
pub fn delete_roofs(
    mut commands: Commands,
    roofs: Query<(Entity, &RoofAnimation), With<RoofMarkedForDeletion>>,
) {
    for (entity, animation) in &roofs {
        if should_despawn(true, animation.inner()) {
            commands.entity(entity).despawn();
        }
    }
}
