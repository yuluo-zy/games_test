//! 半木实体的原事件消费顺序及两张持久实体表。
use bevy_ecs::prelude::*;
use country_core::{
    components::drawable::NaniBundle,
    render::nani::{gpu_world::{GpuWorldSubsetKind, InstanceDataMarker},
        instance_data::CurvedHalfTimberInstanceData, systems::{NaniInstanceDesc, NaniMeshSelector}},
    resources::{roofs::{commands::DeleteRoofCmd, roof_shape::Roof}, walls::{
        WallId, curved_halftimber::{CurvedHalfTimber, CurvedHalfTimberDrawables},
        plaster::{RecalculateHalfTimberMeshCmd, RecalculateWoodenTurretCmd},
        public_walls::{OnWallDeleted, PublicWalls}, wall_holes::WallHoles,
    }},
    systems::ui::signifiers::signifier_create::UiSignifierStream,
    utils::components::transform::Transform,
};

pub mod wooden_floor_borders;

fn despawn_list(entities: Option<Vec<Entity>>, commands: &mut Commands) {
    if let Some(entities) = entities {
        for entity in entities {
            // 原命令无提前存活过滤；Bevy despawn自带延迟warn错误处理。
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_instances(wall: WallId, direct: u8, instances: &[CurvedHalfTimberInstanceData],
    commands: &mut Commands, output: &mut Vec<Entity>) {
    for data in instances {
        spawn_instance(wall, direct, *data, data.w30, commands, output);
    }
}

fn spawn_instance(wall: WallId, direct: u8, data: CurvedHalfTimberInstanceData, mesh: u32,
    commands: &mut Commands, output: &mut Vec<Entity>) {
    let entity = commands.spawn((NaniBundle::<CurvedHalfTimberInstanceData> {
        desc: NaniInstanceDesc { mesh: NaniMeshSelector::AtlasPart(mesh),
            subset_kind: GpuWorldSubsetKind::CurvedHalfTimber },
        transform: Transform::identity(), marker: InstanceDataMarker::default(),
    }, data, CurvedHalfTimber { key: wall.0, direct })).id();
    output.push(entity);
}

/// B 0x1407706d0：先删除两表，再按半木、木塔事件顺序重建对应表。
/// 木边条目标孔经过原有曲线与实体生成路径。
pub fn curved_halftimber_maintain_entities(
    walls: Res<PublicWalls>,
    mut deleted: EventReader<OnWallDeleted>,
    mut delete_roof: EventReader<DeleteRoofCmd>,
    mut drawables: ResMut<CurvedHalfTimberDrawables>,
    mut commands: Commands,
    mut half: EventReader<RecalculateHalfTimberMeshCmd>,
    mut turret: EventReader<RecalculateWoodenTurretCmd>,
    holes: Res<WallHoles>,
    signifiers: Res<UiSignifierStream>,
    roofs: Query<&Roof>,
) {
    for wall in deleted.read().map(|event| event.0)
        .chain(delete_roof.read().map(|event| event.0)) {
        despawn_list(drawables.primary.remove(&wall), &mut commands);
        despawn_list(drawables.secondary.remove(&wall), &mut commands);
    }
    for event in half.read() {
        despawn_list(drawables.primary.remove(&event.wall), &mut commands);
        let Some(wall) = walls.get_unchecked(event.wall) else { continue };
        let mut entities = Vec::new();
        wooden_floor_borders::wooden_floor_borders(&walls, event.wall, wall, &holes,
            &signifiers, &roofs, &mut commands, &mut entities);
        spawn_instances(event.wall, 1, &event.instances, &mut commands, &mut entities);
        drawables.primary.insert(event.wall, entities);
    }
    for event in turret.read() {
        despawn_list(drawables.secondary.remove(&event.wall), &mut commands);
        let mut entities = Vec::new();
        spawn_instances(event.wall, 2, &event.instances, &mut commands, &mut entities);
        drawables.secondary.insert(event.wall, entities);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_rebuilds_for_existing_wall_without_a_roof() {
        use country_core::resources::walls::{InnerWalls, WallState, publication_source::TerrainHeightGrid512,
            public_walls::{WallPublicationDirty, WallSpatialHash}};
        use country_core::resources::roofs::geometry::surface::Curve2;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(Curve2::try_new(vec![[0.0, 0.0], [3.0, 0.0]], false).unwrap(), 2.0, 0, 0, false));
        let grid = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: true };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        assert!(walls.get_unchecked(wall).is_some());
        let mut world = World::new();
        world.insert_resource(walls);
        world.init_resource::<WallHoles>(); world.init_resource::<UiSignifierStream>();
        world.init_resource::<CurvedHalfTimberDrawables>();
        world.init_resource::<Events<OnWallDeleted>>(); world.init_resource::<Events<DeleteRoofCmd>>();
        world.init_resource::<Events<RecalculateHalfTimberMeshCmd>>(); world.init_resource::<Events<RecalculateWoodenTurretCmd>>();
        let mut schedule = Schedule::default(); schedule.add_systems(curved_halftimber_maintain_entities);
        let data = CurvedHalfTimberInstanceData { w30: 3, ..Default::default() };
        world.send_event(RecalculateHalfTimberMeshCmd { wall, instances: vec![data] });
        schedule.run(&mut world);
        let entity = world.resource::<CurvedHalfTimberDrawables>().primary[&wall][0];
        assert_eq!(world.get::<CurvedHalfTimber>(entity).unwrap(), &CurvedHalfTimber { key: wall.0, direct: 1 });
        assert_eq!(world.get::<NaniInstanceDesc>(entity).unwrap().mesh, NaniMeshSelector::AtlasPart(3));
        world.send_event(DeleteRoofCmd(wall)); schedule.run(&mut world);
        assert!(world.get_entity(entity).is_err());
        assert!(!world.resource::<CurvedHalfTimberDrawables>().primary.contains_key(&wall));
    }

    #[test]
    fn turret_replacement_and_wall_deletion_use_the_real_entity_owners() {
        let mut world = World::new();
        world.init_resource::<PublicWalls>();
        world.init_resource::<WallHoles>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<CurvedHalfTimberDrawables>();
        world.init_resource::<Events<OnWallDeleted>>();
        world.init_resource::<Events<DeleteRoofCmd>>();
        world.init_resource::<Events<RecalculateHalfTimberMeshCmd>>();
        world.init_resource::<Events<RecalculateWoodenTurretCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(curved_halftimber_maintain_entities);
        let wall = WallId(7);
        let instance = CurvedHalfTimberInstanceData { w30: 4, ..Default::default() };
        world.send_event(RecalculateWoodenTurretCmd { wall, instances: vec![instance] });
        schedule.run(&mut world);
        let old = world.resource::<CurvedHalfTimberDrawables>().secondary[&wall][0];
        assert_eq!(world.get::<CurvedHalfTimber>(old).unwrap(), &CurvedHalfTimber { key: 7, direct: 2 });
        assert_eq!(world.get::<NaniInstanceDesc>(old).unwrap().mesh, NaniMeshSelector::AtlasPart(4));
        assert_eq!(*world.get::<CurvedHalfTimberInstanceData>(old).unwrap(), instance);
        // 其他生产者已删除实体时，原删除命令仍入队并由warn处理陈旧代数。
        world.despawn(old);
        world.send_event(RecalculateWoodenTurretCmd { wall, instances: vec![] });
        schedule.run(&mut world);
        assert!(world.get_entity(old).is_err());
        assert!(world.resource::<CurvedHalfTimberDrawables>().secondary[&wall].is_empty());
        world.send_event(RecalculateHalfTimberMeshCmd { wall, instances: vec![instance] });
        schedule.run(&mut world);
        assert!(!world.resource::<CurvedHalfTimberDrawables>().primary.contains_key(&wall));
        world.send_event(OnWallDeleted(wall));
        schedule.run(&mut world);
        assert!(!world.resource::<CurvedHalfTimberDrawables>().secondary.contains_key(&wall));
        // 同帧先删后重建，新的木塔事件仍应创建实体。
        world.send_event(OnWallDeleted(wall));
        world.send_event(RecalculateWoodenTurretCmd { wall, instances: vec![instance] });
        schedule.run(&mut world);
        let rebuilt = world.resource::<CurvedHalfTimberDrawables>().secondary[&wall][0];
        assert!(world.get::<CurvedHalfTimberInstanceData>(rebuilt).is_some());
    }
    #[test]
    fn circle_floor_border_spawns_real_instance_for_finalized_hole() {
        use country_core::resources::walls::{InnerWalls, WallState,
            publication_source::TerrainHeightGrid512,
            public_walls::{WallPublicationDirty, WallSpatialHash},
            wall_holes::RawWallHole};
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        inner.update_wall_state_enclosure(wall, ShapeParameters::from_shape(Shape::Circle(
            utils::geometry::Circle2d { center: glam::Vec2::new(1.0, 2.0), radius: 3.0, rotation: 0.25 }
        )).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let length = walls.get_unchecked(wall).unwrap().wall_space_length();
        let mut world = World::new();
        let mut roof = [0u8; 88]; roof[0x40] = 2; roof[0x41] = 1;
        let roof_entity = world.spawn(Roof::from_observed(roof)).id();
        walls.associate_roof(wall, roof_entity);
        world.insert_resource(walls);
        let mut holes = WallHoles::default();
        holes.add(wall, RawWallHole { center: [1.0, 2.0], dimensions: [0.5, 1.0], kind: 1 }).unwrap();
        holes.finalize(wall, length, true).unwrap();
        world.insert_resource(holes); world.init_resource::<UiSignifierStream>();
        world.init_resource::<CurvedHalfTimberDrawables>();
        world.init_resource::<Events<OnWallDeleted>>(); world.init_resource::<Events<DeleteRoofCmd>>();
        world.init_resource::<Events<RecalculateHalfTimberMeshCmd>>(); world.init_resource::<Events<RecalculateWoodenTurretCmd>>();
        world.send_event(RecalculateHalfTimberMeshCmd { wall, instances: vec![] });
        let mut schedule = Schedule::default(); schedule.add_systems(curved_halftimber_maintain_entities);
        schedule.run(&mut world);
        let entities = &world.resource::<CurvedHalfTimberDrawables>().primary[&wall];
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        assert_eq!(*world.get::<CurvedHalfTimber>(entity).unwrap(), CurvedHalfTimber { key: wall.0, direct: 0 });
        let data = world.get::<CurvedHalfTimberInstanceData>(entity).unwrap();
        let scale = std::f32::consts::TAU / length;
        let middle = scale - 0.25; let half = 0.25 * scale;
        let expected = CurvedHalfTimberInstanceData::new_curved_xz(glam::Vec3::new(1.0, 1.5, 2.0), 3.0, 0.54, 0.095, middle - half, middle + half, 0);
        assert_eq!(*data, expected);
        assert_eq!(world.get::<NaniInstanceDesc>(entity).unwrap().mesh, NaniMeshSelector::AtlasPart(expected.w30));
    }

    #[test]
    fn rectangle_floor_border_uses_straight_mesh_and_floor_height() {
        use country_core::resources::walls::{InnerWalls, WallState,
            publication_source::TerrainHeightGrid512,
            public_walls::{WallPublicationDirty, WallSpatialHash}, wall_holes::RawWallHole};
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        inner.update_wall_state_enclosure(wall, ShapeParameters::from_shape(Shape::Rectangle(
            utils::geometry::Rectangle2d { center: glam::Vec2::ZERO, axis: glam::Vec2::X, size: glam::Vec2::new(4.0, 2.0) }
        )).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut world = World::new();
        let mut roof = [0u8; 88]; roof[0x40] = 2; roof[0x41] = 1;
        let roof_entity = world.spawn(Roof::from_observed(roof)).id();
        walls.associate_roof(wall, roof_entity);
        world.insert_resource(walls);
        let mut holes = WallHoles::default();
        holes.add(wall, RawWallHole { center: [0.75, 2.0], dimensions: [0.5, 1.0], kind: 4 }).unwrap();
        holes.finalize(wall, 12.0, false).unwrap();
        world.insert_resource(holes); world.init_resource::<UiSignifierStream>();
        world.init_resource::<CurvedHalfTimberDrawables>();
        world.init_resource::<Events<OnWallDeleted>>(); world.init_resource::<Events<DeleteRoofCmd>>();
        world.init_resource::<Events<RecalculateHalfTimberMeshCmd>>(); world.init_resource::<Events<RecalculateWoodenTurretCmd>>();
        world.send_event(RecalculateHalfTimberMeshCmd { wall, instances: vec![] });
        let mut schedule = Schedule::default(); schedule.add_systems(curved_halftimber_maintain_entities);
        schedule.run(&mut world);
        let entities = &world.resource::<CurvedHalfTimberDrawables>().primary[&wall];
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        assert_eq!(world.get::<NaniInstanceDesc>(entity).unwrap().mesh, NaniMeshSelector::AtlasPart(4));
        assert_eq!(world.get::<CurvedHalfTimberInstanceData>(entity).unwrap().position.y, 1.5);
        assert_eq!(world.get::<CurvedHalfTimberInstanceData>(entity).unwrap().w20, 1);
        assert_eq!(world.get::<CurvedHalfTimberInstanceData>(entity).unwrap().w30, 1);
        assert_eq!(*world.get::<CurvedHalfTimber>(entity).unwrap(), CurvedHalfTimber { key: wall.0, direct: 0 });
    }

}
