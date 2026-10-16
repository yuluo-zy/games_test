//! 屋顶编辑消费（edit_roof.rs）。
//!
//! - `edit_roof`（0x1421ab7c0）：候选参数面（skeleton 富化）含
//!   `EventWriter<SwitchRoofTypeCmd>` —— 类型穿越通知走 SwitchRoofTypeCmd，
//!   由 `visual::switch_roof_type::switch_roof_type` 消费（原 `switch_roof_type`
//!   0x1421ac4f0）。
//! - 方向/平顶风格命令：`switch_roof_direction`、`set_flat_roof_style`、
//!   `set_flat_roof_rim_style`。
use country_core::recovered_building::{
    building_history_transactions::RoofHistoryEdit,
    roof_customization::{apply_flat_rim_resolved, apply_flat_style_resolved},
    roof_direction::{RoofDirectionRequest, apply_resolved_direction},
    wall_state::WallId as ObservedWallId,
};
use country_core::resources::{
    roofs::{Roof, commands::*},
    walls::{WallId, public_walls::PublicWalls},
};
use bevy_ecs::prelude::*;

pub fn edit_roof(
    mut requests: EventReader<EditRoofCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableChanged>,
    mut feedback: EventWriter<RoofEditFeedback>,
    mut crossing: EventWriter<SwitchRoofTypeCmd>,
) {
    for request in requests.read() {
        let id = WallId(request.0.wall_id);
        let entity = walls
            .roof_entity(id)
            .expect("edit_roof requires a roof association");
        let mut roof = roofs
            .get_mut(entity)
            .expect("edit_roof associated entity has no Roof");
        let mut changed = 0;
        let result = country_core::resources::roofs::geometry::edit_rules::apply_edit(
            roof.bypass_change_detection().observed_mut(),
            &request.0,
            &mut changed,
            1,
        );
        if changed != 0 {
            roof.set_changed();
        }
        if result.rebuild_gable {
            gable.write(WallGableChanged(id));
        }
        for value in result.feedback {
            feedback.write(RoofEditFeedback {
                entity,
                wall: id,
                feedback: value,
            });
        }
        // M1（reviewer-C）：16B 变更通知只在 resolved_update（gable 进出/
        // 变化，或 profile/height/eave 变化且 gable-after）时推送；
        // 仅 ridge/tip 变化不推。Scalar 反馈的 |ridge| 分量与 16B 通知
        // 的推送条件分开实现（feedback 已按自身条件写入）。
        if result.resolved_update {
            geometry.write(RoofGeometryChanged {
                entity,
                wall: id,
                kind: 1,
            });
        }
        if let Some(value) = result.height_crossing {
            crossing.write(SwitchRoofTypeCmd {
                entity,
                wall: id,
                crossing: value,
                source_flag: request.0.source_flag,
            });
        }
    }
}

pub fn switch_roof_direction(
    mut requests: EventReader<SwitchRoofDirectionCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableChanged>,
    mut sound: EventWriter<RoofSound>,
) {
    for request in requests.read() {
        let entity = walls
            .roof_entity(request.wall)
            .expect("roof direction requires a roof association");
        let mut roof = roofs
            .get_mut(entity)
            .expect("roof direction associated entity has no Roof");
        let mut changed = 0;
        let result = apply_resolved_direction(
            roof.observed_mut(),
            RoofDirectionRequest {
                wall: ObservedWallId(request.wall.0),
                direction: request.direction,
            },
            entity.to_bits(),
            &mut changed,
            1,
        );
        geometry.write(RoofGeometryChanged {
            entity,
            wall: request.wall,
            kind: result.geometry_kind,
        });
        gable.write(WallGableChanged(request.wall));
        sound.write(RoofSound {
            entity,
            wall: request.wall,
            pivot: result.sound_pivot,
            kind: result.sound_kind,
            flat_surface: None,
        });
    }
}

pub fn set_flat_roof_style(
    mut requests: EventReader<SetFlatRoofStyleCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut holes: EventWriter<RecalculateWallHoles>,
    mut sounds: EventWriter<RoofSound>,
) {
    for request in requests.read() {
        let wall = WallId(request.0.wall_id.0);
        // B 0x142299dd0 skips absent associations/query rows, unlike edit_roof.
        let Some(entity) = walls.roof_entity(wall) else {
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            continue;
        };
        let mut changed = 0;
        let result = apply_flat_style_resolved(roof.observed_mut(), request.0, &mut changed, 1);
        if let Some(id) = result.recalculate_wall_holes {
            holes.write(RecalculateWallHoles(WallId(id.0)));
        }
        if let Some(sound) = result.sound {
            sounds.write(RoofSound {
                entity,
                wall,
                pivot: sound.pivot,
                kind: sound.observed_kind,
                flat_surface: None,
            });
        }
    }
}

pub fn set_flat_roof_rim_style(
    mut requests: EventReader<SetFlatRoofRimStyleCmd>,
    walls: Res<PublicWalls>,
    mut roofs: Query<&mut Roof>,
    mut holes: EventWriter<RecalculateWallHoles>,
    mut history: EventWriter<RecordRoofHistory>,
) {
    for request in requests.read() {
        let wall = WallId(request.0.wall_id.0);
        let Some(entity) = walls.roof_entity(wall) else {
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            continue;
        };
        let mut changed = 0;
        let result = apply_flat_rim_resolved(roof.observed_mut(), request.0, &mut changed, 1);
        if let Some(id) = result.recalculate_wall_holes {
            holes.write(RecalculateWallHoles(WallId(id.0)));
        }
        if let Some(mutation) = result.history_mutation {
            history.write(RecordRoofHistory(RoofHistoryEdit::rim(mutation)));
        }
    }
}
