//! 创建屋顶与墙→屋顶关联维护。
//!
//! - `create_roof`（0x1421aaea0）：B create_roof 系统。
//!   候选参数面（skeleton 富化）：`(EventReader<CreateRoofCmd>, Res<PublicWalls>,
//!   EventWriter<RecalculateRoofCmd>, EventWriter<RecalculateGableRoofWallHeights>,
//!   EventWriter<PlayRoofSound>, Commands)`；重建映射：
//!   RecalculateRoofCmd ≡ RoofGeometryChanged、RecalculateGableRoofWallHeights ≡
//!   WallGableChanged、PlayRoofSound ≡ RoofSound。
//! - `wall_roof_hashmap`：原 `country_core::systems::wall::wall_state::wall_roof_hashmap`
//!   的关联哈希表系统（RoofIsInvalid 事件的原始归属模块）。
use country_core::resources::roofs::associations::AssociationAction;
use country_core::resources::roofs::associations::association_action;
use country_core::resources::roofs::components::prepare_resolved_roof_components;
use country_core::resources::{
    roofs::{
        Roof, RoofAnimation, RoofMarkedForDeletion, RoofTileSimulation, RoofTriggerVolume,
        commands::*,
    },
    walls::{WallId, public_walls::PublicWalls},
};
use country_core::utils::components::transform::Transform;
use bevy_ecs::prelude::*;
use hashbrown::HashMap;

/// B 0x1421aaea0. 缺失墙与不支持形状按原不变式失败。命令拥有实体与延迟组件插入。
pub fn create_roof(
    mut commands: Commands,
    mut requests: EventReader<CreateRoofCmd>,
    walls: Res<PublicWalls>,
    mut gable: EventWriter<WallGableChanged>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut sounds: EventWriter<RoofSound>,
) {
    for request in requests.read() {
        let id = WallId(request.0.wall_id().0);
        let wall = walls.get(id).expect("create_roof requires a public wall");
        let prepared = prepare_resolved_roof_components(
            [0; 88],
            [0; 24],
            &request.0,
            &wall.building_projection(),
        )
        .expect("create_roof requires a supported wall with a valid height cache");
        // Only ABI padding survives the initializer: Roof[0x55..0x58] and
        // TileSimulation[21..24]. Zeroing those bytes defines no game fields.
        let roof = Roof::from_observed(prepared.roof);
        let pivot = roof.pivot_3d().to_array();
        let pitched = roof.is_pitched();
        let style = roof.observed()[0x54];
        let entity = commands
            .spawn((
                roof,
                RoofAnimation::from_inner(prepared.animation),
                RoofTileSimulation::from_inner(prepared.tile_simulation),
                RoofTriggerVolume::default(),
                // 1421ab2bf..2f2: zero translation, identity rotation, unit scale.
                Transform::identity(),
            ))
            .id();
        sounds.write(RoofSound {
            entity,
            wall: id,
            pivot,
            kind: if pitched { 0 } else { 6 },
            flat_surface: (!pitched).then_some(style),
        });
        geometry.write(RoofGeometryChanged {
            entity,
            wall: id,
            kind: 0,
        });
        if let Some(id) = prepared.creation.gable_notification {
            gable.write(WallGableChanged(WallId(id.0)));
        }
    }
}

/// 关联系统：与 mode manager 消费同一 AppMode 资源；13/14 模式禁止存活非法关联。
pub fn wall_roof_hashmap(
    mode: Res<country_core::systems::mode_manager::AppMode>,
    roofs: Query<(Entity, &Roof, Has<RoofMarkedForDeletion>)>,
    mut walls: ResMut<PublicWalls>,
    mut invalid: EventWriter<RoofIsInvalid>,
    mut presence: EventWriter<WallRoofPresenceChanged>,
) {
    let mut entities = HashMap::new();
    for (entity, roof, marked) in &roofs {
        let id = roof.wall_id();
        let projection = walls.get(id).map(|wall| wall.building_projection());
        let action = association_action(
            projection.as_ref(),
            marked,
            mode.native_tag(),
            entities.contains_key(&id),
        )
        .expect("invalid wall/roof association");
        match action {
            AssociationAction::Insert => {
                entities.insert(id, entity);
            }
            AssociationAction::KeepDeleting => {}
            AssociationAction::RequestDeletion => {
                invalid.write(RoofIsInvalid(id));
            }
        }
    }
    for wall in walls.replace_roof_entities(entities) {
        presence.write(WallRoofPresenceChanged(wall));
    }
}
