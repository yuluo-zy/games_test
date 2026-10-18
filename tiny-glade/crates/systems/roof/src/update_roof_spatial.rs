//! 屋顶空间同步（update_roof_spatial.rs）。
//!
//! - `update_roof_spatial`（0x1421a7710）
//! - `update_wall_gable_heights`（gable 高度缓存维护；原
//!   `RecalculateGableRoofWallHeights` 通知的消费面）
use country_core::resources::roofs::spatial::synchronize_resolved_roof;
use country_core::resources::walls::publication_fields::WallId as ObservedWallId;
use country_core::resources::{
    roofs::{
        Roof, RoofMarkedForDeletion,
        commands::{RoofGeometryChanged, WallGableChanged, WallRoofPresenceChanged},
    },
    walls::{WallId, public_walls::{OnWallChanged, PublicWalls}},
};
use country_core::utils::components::transform::Transform;
use bevy_ecs::prelude::*;

/// B 0x1421a7710。每个查询到的屋顶只更新一次，即使事件批重复其墙；
/// 风格与形状各发自己的 kind-2 通知。
pub fn update_roof_spatial(
    mut changed: EventReader<OnWallChanged>,
    walls: Res<PublicWalls>,
    mut roofs: Query<(Entity, &mut Roof, &mut Transform), Without<RoofMarkedForDeletion>>,
    mut geometry: EventWriter<RoofGeometryChanged>,
) {
    let ids: Vec<_> = changed.read().map(|event| event.0).collect();
    for (entity, mut roof, mut transform) in &mut roofs {
        let id = roof.wall_id();
        if !ids.contains(&id) {
            continue;
        }
        let Some(wall) = walls.get(id) else {
            continue;
        };
        let mut translation_projection = [0; 48];
        let (mut roof_tick, mut transform_tick) = (0, 0);
        let outcome = synchronize_resolved_roof(
            roof.observed_mut(),
            &mut translation_projection,
            &wall.building_projection(),
            ObservedWallId(id.0),
            entity.to_bits(),
            &mut roof_tick,
            &mut transform_tick,
            1,
        )
        .expect("update_roof_spatial requires a valid published wall");
        transform.translation = glam::Vec3::from_array(core::array::from_fn(|index| {
            let offset = 16 + index * 4;
            f32::from_le_bytes(
                translation_projection[offset..offset + 4]
                    .try_into()
                    .unwrap(),
            )
        }));
        for change in outcome.geometry_changes {
            geometry.write(RoofGeometryChanged {
                entity,
                wall: id,
                kind: change.observed_kind,
            });
        }
    }
}

/// 屋顶编辑与关联存在性接入公共墙拥有的高度缓存；移除最后一个屋顶恢复普通墙缓存。
pub fn update_wall_gable_heights(
    mut gable: EventReader<WallGableChanged>,
    mut spatial: EventReader<OnWallChanged>,
    mut presence: EventReader<WallRoofPresenceChanged>,
    roofs: Query<&Roof>,
    mut walls: ResMut<PublicWalls>,
) {
    let mut ids: Vec<_> = gable
        .read()
        .map(|event| event.0)
        .chain(spatial.read().map(|event| event.0))
        .chain(presence.read().map(|event| event.0))
        .collect();
    ids.sort_unstable_by_key(|id| id.0);
    ids.dedup();
    for id in ids {
        let roof = walls
            .roof_entity(id)
            .and_then(|entity| roofs.get(entity).ok());
        if let Some(wall) = walls.get_mut(id) {
            wall.update_gable_heights(roof.map(Roof::observed))
                .expect("published roof requires compatible wall gable geometry");
        }
    }
}
