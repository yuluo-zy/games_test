//! B 140757800：复用core唯一逐墙循环，在主砖数组提交前执行原附加几何链。
//! 半木尚未恢复的分支继续显式失败，不将缺少输出当作成功。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources;
use country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd;
use country_core::resources::walls::wall_holes::WallHoleStorage;
use country_core::systems::backend::input::translate_input::DemoModeEnabled;
use country_core::systems::wall::construct_wall_bricks::WallBrickConstruction;

/// 原外层拥有附加算法资源，core不反向依赖wall-constructor，也不另扫已完成事件。
pub fn construct_walls(
    walls: WallBrickConstruction,
    mut library: ResMut<AssetSsboLibrary>,
    terrain: Res<TerrainHeightsData>,
    decorators: Res<DecoratorStorage>,
    derived: DerivedDecoratorInfoResources,
    demo: Res<DemoModeEnabled>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
    mut half_changed: EventWriter<RecalculateHalfTimberMeshCmd>,
) {
    walls.run_with(Some(&mut |id, wall, roof, prepared, holes, instances| {
        // Public+296的special原分支直接跳到实例提交，不生成附加几何或半木事件。
        let Some(prepared) = prepared else { return None; };
        let arrays = &prepared.arrays;
        let global = holes.storage.entry(id).or_insert_with(WallHoleStorage::new_finalized);
        let mut local = if wall.authority.style() >= 2 { None } else {
            // 原按style决定克隆；新角洞只进入本次局部缓存，不污染全局装饰洞口。
            Some(global.clone_and_reopen())
        };
        let finalized = if let Some(local) = local.as_mut() {
            crate::utils::wall_corners::add_wall_corners_new(wall, id,
                &arrays.second_heights, &arrays.arch_heights, instances,
                local, prepared.kind, prepared.top);
            local.finalize(wall.wall_space_length(), wall.shape().tag() != 2)
                .expect("原墙角局部洞口最终化失败");
            local.finalized_holes().expect("原局部洞口尚未最终化")
        } else { global.finalized_holes().expect("原半木墙洞口尚未最终化") };
        let mut curved = Vec::new();
        if let Some(roof) = roof.filter(|_| wall.authority.style() >= 2) {
            crate::utils::half_timber::half_timbering(crate::utils::half_timber::HalfTimberInput {
                wall, wall_id: id, decorators: &decorators, derived: derived.as_view(),
                terrain: &terrain, segment_heights: &wall.segment_heights,
                path_heights: &arrays.second_heights, roof, holes: finalized, demo_mode: demo.0,
            }, &mut library, &output.gpu, &mut curved);
            if roof.is_gable() {
                crate::utils::half_timber::roof_outline(wall, id, roof, &mut library, &output.gpu);
            }
        }
        // 原无半木输出也发布空列表，用于维护消费者退役旧曲面实体。
        half_changed.write(RecalculateHalfTimberMeshCmd { instances: curved, wall: id });
        if wall.authority.style() == 1 {
            crate::utils::wall_skirts::add_skirts_new(wall, id, &arrays.curve,
                &arrays.second_heights, &wall.segment_heights, finalized, instances);
        }
        if matches!(prepared.kind, 1 | 3) {
            crate::utils::stairs_signifiers::stairs_signifiers_new(wall, id, instances, &terrain, finalized);
        }
        local
    }));
}

#[cfg(test)]
mod real_world_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::geometry::instanced_wall::{BrickTransformSsbo, InstancedWall};
    use country_core::resources::roofs::{Roof, geometry::surface::Curve2};
    use country_core::resources::walls::{InnerWalls, WallId, WallState,
        decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorSubtypes},
        public_walls::{OnWallChanged, OnWallDeleted, PublicWalls, WallEntities, WallPublicationDirty, WallSpatialHash},
        publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512,
        wall_holes::{WallHoleStorage, WallHoles}};
    use country_core::systems::wall::construction::production::WallTopBounds;
    use country_core::systems::wall::wall_state::path_preparation::PathRasters512;

    fn fixture(style: u8, height: f32, rectangle: bool, special: bool) -> (World, WallId) {
        let points = if rectangle { vec![[0.,0.],[0.,3.],[3.,3.],[3.,0.],[0.,0.]] }
            else { vec![[0.,0.],[3.,0.],[3.,3.]] };
        let curve = Curve2::try_new(points, false).unwrap();
        let base = WallState::new(curve, height, style, u8::from(special), false);
        let mut snapshot = base.snapshot().clone();
        if rectangle {
            let mut shape = [0u8; 28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
            for (i, v) in [1f32,0.,1.5,1.5,3.,3.].iter().enumerate() {
                shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes());
            }
            snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
            snapshot.source.curve_kind_tag = 1;
        }
        let mut inner = InnerWalls::default();
        let id = inner.new_wall(WallState::from_snapshot(snapshot, 0));
        let grid = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0., maximum_y:0., sampling_fix:false };
        let paths = PathRasters512::from_encoded(std::array::from_fn(|_| vec![0u8;512*512].into()), false).unwrap();
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &grid, Some(&paths), &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut world = World::new();
        country_core::systems::wall::registration::register_wall_cpu_resources(&mut world);
        world.insert_resource(public);
        world.insert_resource(AssetSsboLibrary::new());
        let mut terrain = TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new();
        terrain.set_height_range(0.,0.);
        world.insert_resource(terrain);
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<DecoratorMeshLibrary>();
        world.init_resource::<DemoModeEnabled>();
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        (world, id)
    }

    fn output(world: &World, id: WallId) -> &[BrickTransformSsbo] {
        let entity = world.resource::<WallEntities>().wall_entity(id).unwrap();
        world.get::<InstancedWall<BrickTransformSsbo>>(entity).unwrap().pending_instances()
    }
    fn bytes(values: &[BrickTransformSsbo]) -> Vec<u8> {
        values.iter().flat_map(|v| v.observed_bytes()).collect()
    }

    #[test]
    fn walls_append_corner_then_skirt_before_body_and_rebuild_without_global_mutation() {
        for style in [0, 1] {
            let (mut world,id) = fixture(style,6.,false,false);
            let wall = world.resource::<PublicWalls>().get(id).unwrap();
            let path = wall.prepared_path.as_ref().unwrap();
            let path_y: Vec<_> = path.positions.iter().map(|p|p[1]).collect();
            let top = wall.arch_segment_heights().unwrap();
            let mut local = WallHoleStorage::new_in_progress();
            let mut extras = Vec::new();
            crate::utils::wall_corners::add_wall_corners_new(wall,id,&path_y,top,&mut extras,&mut local,2,WallTopBounds::PLAIN);
            let corners = extras.len();
            assert!(corners > 0);
            local.finalize(wall.wall_space_length(),false).unwrap();
            if style == 1 {
                let curve = Curve2::try_new(path.positions.iter().map(|p|[p[0],p[2]]).collect(),false).unwrap();
                crate::utils::wall_skirts::add_skirts_new(wall,id,&curve,&path_y,&wall.segment_heights,local.finalized_holes().unwrap(),&mut extras);
                assert!(extras.len()>corners, "style 1 必须生成裙砖");
            }
            let empty_body = country_core::systems::wall::construct_wall_bricks::build_wall_bricks(id,wall,None,&WallHoles::default()).unwrap();
            let expected_prefix = bytes(&extras);
            world.send_event(OnWallChanged(id));
            world.run_system_once(construct_walls).unwrap();
            let actual = output(&world,id);
            assert!(actual.len()>extras.len(), "应继续追加主体砖");
            assert_eq!(bytes(&actual[..extras.len()]),expected_prefix);
            assert_ne!(bytes(&actual[extras.len()..]),bytes(&empty_body),"主体应使用角生成的局部洞口");
            assert!(world.resource::<WallHoles>().storage[&id].finalized_holes().unwrap().is_empty(),"局部角洞不得污染全局");
            let half = world.resource::<Events<RecalculateHalfTimberMeshCmd>>();
            let mut cursor = half.get_cursor();
            let notices: Vec<_> = cursor.read(half).filter(|v|v.wall==id).collect();
            assert_eq!(notices.len(),1); assert!(notices[0].instances.is_empty());
            let first = bytes(actual);
            world.resource_mut::<Events<OnWallChanged>>().clear();
            world.resource_mut::<Events<RecalculateHalfTimberMeshCmd>>().clear();
            world.send_event(OnWallChanged(id));
            world.run_system_once(construct_walls).unwrap();
            assert_eq!(bytes(output(&world,id)),first,"重建不应叠加旧实例");
            let entity = world.resource::<WallEntities>().wall_entity(id).unwrap();
            world.send_event(OnWallDeleted(id));
            world.run_system_once(country_core::systems::wall::delete_wall::delete_wall).unwrap();
            assert!(world.resource::<WallEntities>().wall_entity(id).is_none());
            assert!(world.get_entity(entity).is_err());
        }
    }

    #[test]
    fn special_bypasses_extra_geometry_and_half_timber_event() {
        let (mut world,id) = fixture(0,3.,false,true);
        world.send_event(OnWallChanged(id));
        world.run_system_once(construct_walls).unwrap();
        assert!(output(&world,id).is_empty());
        assert!(world.resource::<Events<RecalculateHalfTimberMeshCmd>>().is_empty());
        assert!(!world.resource::<WallHoles>().storage.contains_key(&id));
    }

    #[test]
    fn roof_kind_one_and_three_emit_stairs_into_main_owner() {
        for style in [0,2] {
            let (mut world,id) = fixture(style,3.,true,false);
            world.resource_mut::<DemoModeEnabled>().0 = true;
            let mut roof = [0u8;88]; roof[..8].copy_from_slice(&id.0.to_le_bytes());
            let roof_entity = world.spawn(Roof::from_observed(roof)).id();
            world.resource_mut::<PublicWalls>().associate_roof(id,roof_entity);
            let wall = world.resource::<PublicWalls>().get(id).unwrap();
            let mut local = WallHoleStorage::new_in_progress();
            let mut extras = Vec::new();
            if style==0 {
                let path = wall.prepared_path.as_ref().unwrap();
                let y:Vec<_> = path.positions.iter().map(|p|p[1]).collect();
                crate::utils::wall_corners::add_wall_corners_new(wall,id,&y,wall.arch_segment_heights().unwrap(),&mut extras,&mut local,1,WallTopBounds::PLAIN);
                local.finalize(wall.wall_space_length(),true).unwrap();
            } else { local = WallHoleStorage::new_finalized(); }
            let before = extras.len();
            crate::utils::stairs_signifiers::stairs_signifiers_new(wall,id,&mut extras,world.resource::<TerrainHeightsData>(),local.finalized_holes().unwrap());
            assert!(extras.len()>before,"kind {} 必须生成阶梯指示砖",if style==0{1}else{3});
            let prefix = bytes(&extras);
            world.send_event(OnWallChanged(id));
            world.run_system_once(construct_walls).unwrap();
            let actual = output(&world,id);
            assert!(actual.len()>=extras.len());
            assert_eq!(bytes(&actual[..extras.len()]),prefix);
        }
    }
}
