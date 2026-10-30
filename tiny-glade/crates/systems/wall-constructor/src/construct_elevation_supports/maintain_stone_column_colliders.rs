//! 石柱碰撞维护（construct_elevation_supports/maintain_stone_column_colliders.rs）。
//!
//! `maintain_stone_column_colliders`（0x14078a6e0）。候选参数面（skeleton
//! 富化）：`(ResMut<StoneColumnCollisionStorage>, ResMut<RaycastWorld>,
//!  EventReader<OnWallDeleted>, EventWriter<StoneColumnsChanged>)`。
//!
//! 原版按墙删除和脏队列维护碰撞，变更事件每次系统运行最多发送一次。
use bevy_ecs::prelude::*;
use country_core::resources::walls::{
    public_walls::OnWallDeleted, wall_stone_columns::StoneColumnCollisionStorage,
};
use country_core::systems::collision::world::RaycastWorld;

/// 原 0x1412007da–0x14120089d：将每个柱的有向包围盒展开为原箱体三角形。
fn column_triangles(column: &country_core::resources::walls::wall_stone_columns::StoneColumn) -> Vec<utils::geometry::intersect::Triangle> {
    let bounds = column.bounds;
    let center_y = (bounds.top + bounds.bottom) * 0.5;
    let direction = glam::Vec2::new(-bounds.footprint.axis.y, bounds.footprint.axis.x);
    let transform = country_core::utils::components::transform::Transform {
        translation: glam::Vec3::new(bounds.footprint.center.x, center_y, bounds.footprint.center.y),
        rotation: utils::xz_rotation_bad(direction),
        scale: glam::Vec3::ONE,
    };
    country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume(
        bounds.footprint.size.x, bounds.top - bounds.bottom, bounds.footprint.size.y, &transform,
    )
}

/// B 0x14078a6e0：删除墙先移除碰撞，再处理本帧被清理/重建的柱。
pub fn maintain_stone_column_colliders(
    mut storage: ResMut<StoneColumnCollisionStorage>,
    mut world: ResMut<RaycastWorld>,
    mut deleted: EventReader<OnWallDeleted>,
    mut changed: EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>,
) {
    use country_core::systems::collision::world::{remove_collider, upsert_collider, RaycastMeshData};
    let storage = &mut *storage;
    let mut touched = false;
    for id in deleted.read().map(|event| event.0) {
        storage.columns.insert(id, Vec::new());
        remove_collider(&mut world, &mut storage.colliders, &id);
        touched = true;
    }
    for id in storage.cleared.drain(..) {
        let columns = storage.columns.get(&id).expect("石柱更新必须具有对应墙记录");
        if columns.is_empty() {
            remove_collider(&mut world, &mut storage.colliders, &id);
        } else {
            let vertices: Vec<_> = columns.iter().flat_map(column_triangles)
                .flat_map(|triangle| triangle.points).collect();
            let indices: Vec<[u32; 3]> = (0..vertices.len() / 3).map(|index| {
                let base = (index * 3) as u32;
                [base, base + 1, base + 2]
            }).collect();
            let mesh = std::sync::Arc::new(RaycastMeshData::new_indexed(&indices, &vertices, None, None));
            let mut source = [0u8; 24];
            source[0] = 6;
            source[8..16].copy_from_slice(&id.0.to_le_bytes());
            upsert_collider(&mut world, &mut storage.colliders, id, country_core::systems::collision::query_policy::RaycastGeoSourceProjection(source), mesh, glam::Vec3::ZERO, glam::Quat::IDENTITY);
        }
        touched = true;
    }
    if touched { changed.write(country_core::resources::walls::wall_stone_columns::StoneColumnsChanged); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stone_column_expands_to_original_box_triangles() {
        let column = country_core::resources::walls::wall_stone_columns::StoneColumn {
            bounds: utils::geometry::rectangle::Rectangle3d::new(
                glam::Vec3::new(3.0, 5.0, 7.0), glam::Vec3::new(2.0, 4.0, 6.0), glam::Vec2::X),
        };
        let triangles = column_triangles(&column);
        assert_eq!(triangles.len(), 12);
        for point in triangles.iter().flat_map(|triangle| triangle.points) {
            assert!((point.y - 3.0).abs() < 1e-5 || (point.y - 7.0).abs() < 1e-5);
            assert!((point.x - 2.0).abs() < 1e-5 || (point.x - 4.0).abs() < 1e-5);
            assert!((point.z - 4.0).abs() < 1e-5 || (point.z - 10.0).abs() < 1e-5);
        }
    }
    #[test]
    fn collider_tracks_creation_replacement_deletion_and_quiet_frame() {
        use bevy_ecs::event::Events;
        use country_core::resources::walls::{WallId, wall_stone_columns::{StoneColumn, StoneColumnsChanged}};
        let id = WallId(77);
        let mut world = World::new();
        world.insert_resource(StoneColumnCollisionStorage::default());
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(Events::<OnWallDeleted>::default());
        world.insert_resource(Events::<StoneColumnsChanged>::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(maintain_stone_column_colliders);
        let make = |x| StoneColumn { bounds: utils::geometry::rectangle::Rectangle3d::new(
            glam::Vec3::new(x, 5.0, 7.0), glam::Vec3::new(2.0,4.0,6.0), glam::Vec2::X) };
        world.resource_mut::<StoneColumnCollisionStorage>().clear_and_get_mut(id).push(make(9.0));
        world.resource_mut::<StoneColumnCollisionStorage>().clear_and_get_mut(id).push(make(3.0));
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().cleared, vec![id, id]);
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().columns[&id], vec![make(3.0)]);
        schedule.run(&mut world);
        let first = world.resource::<StoneColumnCollisionStorage>().colliders[&id];
        assert_eq!(world.resource::<Events<StoneColumnsChanged>>().len(), 1);
        schedule.run(&mut world);
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().colliders[&id], first);
        assert_eq!(world.resource::<Events<StoneColumnsChanged>>().len(), 1);
        world.resource_mut::<StoneColumnCollisionStorage>().clear_and_get_mut(id).push(make(3.0));
        schedule.run(&mut world);
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().colliders[&id], first);
        assert_eq!(world.resource::<Events<StoneColumnsChanged>>().len(), 2);
        world.resource_mut::<StoneColumnCollisionStorage>().clear_and_get_mut(id).push(make(9.0));
        schedule.run(&mut world);
        let next = world.resource::<StoneColumnCollisionStorage>().colliders[&id];
        assert_ne!(first, next);
        assert_eq!(world.resource::<Events<StoneColumnsChanged>>().len(), 3);
        world.resource_mut::<Events<OnWallDeleted>>().send(OnWallDeleted(id));
        schedule.run(&mut world);
        let storage = world.resource::<StoneColumnCollisionStorage>();
        assert!(!storage.colliders.contains_key(&id));
        assert!(storage.columns[&id].is_empty());
        assert_eq!(world.resource::<Events<StoneColumnsChanged>>().len(), 4);
    }

}
