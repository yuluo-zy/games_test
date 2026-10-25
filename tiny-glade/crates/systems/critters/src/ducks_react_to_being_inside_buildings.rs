//! B 0x14231b140：完成建筑编辑后，检查建筑内的鸭子并触发叫声。
//!
//! 原函数及两个直接 Bevy 包装器确认输入为 `CmdCheckForSheepInsideBuilding`
//! 事件、`(&Duck, &Transform)`、`PublicWalls` 与 `&Roof`。事件名称沿用原版，
//! 鸭子也消费同一事件；不是每帧扫描全部墙，也没有 DuckState 或谷地边界门。
//! 证据：TG-ec39e67771c9fa33c3/core-duck-building-20261021/native-evidence-v3。

use bevy_ecs::prelude::*;
use country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use glam::Vec2;

use crate::components::{Duck, Transform};

/// 逐编辑事件检查该建筑内的鸭子，播放 Duck 已持有的 Quack 实例。
///
/// 原版 Duck 的音频实例可通过共享引用调用；当前音频缝需要可变引用。
/// 这里只修改音频后端状态，通过 `bypass_change_detection` 保持 Duck 业务
/// 字段的变更语义，不给后续 `Changed<Duck>` 消费者制造行为变更。
pub fn ducks_react_to_being_inside_buildings(
    mut checks: EventReader<CmdCheckForSheepInsideBuilding>,
    mut ducks: Query<(ComponentWrite<Duck>, &Transform)>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
) {
    for check in checks.read() {
        // B 0x14231b2c6：不存在的墙会 unwrap panic，即使场景中没有鸭子。
        let wall = walls.get(check.wall).unwrap();
        let elevation = wall.authority.elevation();
        // +0x258 是形状 tag，+0x280 是标高；NaN 不在此处被提前过滤。
        if !wall.is_premade_shape() || elevation > 1.5 {
            continue;
        }
        let shape = ShapeParameters::from_snapshot(wall.get_wall_shape());
        for (mut duck, transform) in &mut ducks {
            // 保持原失败顺序：max_y/屋顶读取在二维包含判断之前；没有鸭子时不读。
            let maximum_y = wall.max_y();
            let roof_height = walls
                .roof_entity(check.wall)
                .and_then(|entity| roofs.get(entity).ok())
                .map_or(0.0, Roof::height);
            let position = transform.translation;
            if shape.contains_point(Vec2::new(position.x, position.z))
                && (elevation..=maximum_y + roof_height).contains(&position.y)
            {
                // 原调用只 start，不重新实例化，也不写位置或停止其它音频。
                duck.bypass_change_detection()
                    .audio
                    .quack
                    .start_if_not_playing();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::{AudioHandle, CritterAudioSink};
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;
    use country_core::resources::walls::publication_source::TerrainHeightGrid512;
    use country_core::resources::walls::{InnerWalls, WallId, WallState};
    use country_core::systems::wall::valid_enclosure::Shape;
    use glam::Vec3;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use utils::geometry::{Circle2d, Rectangle2d};

    struct QuackCounter(Arc<AtomicUsize>);

    impl CritterAudioSink for QuackCounter {
        fn start_if_not_playing(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
        fn stop(&mut self) {
            panic!("建筑反应不应停止音频");
        }
        fn set_position(&mut self, _: Vec3) {
            panic!("建筑反应不应改音频位置");
        }
        fn set_volume(&mut self, _: f32) {
            panic!("建筑反应不应改音量");
        }
        fn set_parameter(&mut self, _: &str, _: f32) {
            panic!("建筑反应不应改参数");
        }
    }

    fn circle() -> ShapeParameters {
        ShapeParameters::from_shape(Shape::Circle(Circle2d {
            center: Vec2::ZERO,
            radius: 3.0,
            rotation: 0.0,
        }))
    }

    fn world_with_wall(shape: ShapeParameters, elevation: f32) -> (World, WallId, Schedule) {
        // 使用真实权威→发布链；曲线刻意远离形状，防止退回曲线射线近似也通过测试。
        let curve = Curve2::try_new(vec![[20.0, 20.0], [24.0, 20.0]], false).unwrap();
        let mut snapshot = WallState::new(curve, 4.0, 0, 0, false).snapshot().clone();
        snapshot.source.shape = shape.to_snapshot();
        snapshot.source.terrain_floor = elevation;
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::from_snapshot(snapshot, 0));
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
        assert!(public.get(wall).is_some());
        let mut world = World::new();
        world.insert_resource(inner);
        world.insert_resource(public);
        world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(ducks_react_to_being_inside_buildings);
        (world, wall, schedule)
    }

    fn add_duck(world: &mut World, position: Vec3) -> (Entity, Arc<AtomicUsize>) {
        let calls = Arc::new(AtomicUsize::new(0));
        let mut duck = Duck::default();
        duck.audio.quack = AudioHandle::Active(Box::new(QuackCounter(calls.clone())));
        let entity = world
            .spawn((
                duck,
                Transform {
                    translation: position,
                    ..Default::default()
                },
            ))
            .id();
        (entity, calls)
    }

    fn check(world: &mut World, wall: WallId) {
        world
            .resource_mut::<Events<CmdCheckForSheepInsideBuilding>>()
            .send(CmdCheckForSheepInsideBuilding { wall });
    }

    fn set_height(world: &mut World, wall: WallId, height: f32) {
        world
            .resource_mut::<PublicWalls>()
            .get_mut(wall)
            .unwrap()
            .regular_heights
            .as_mut()
            .unwrap()
            .maximum_y = height;
    }

    fn remove_height(world: &mut World, wall: WallId) {
        let mut public = world.resource_mut::<PublicWalls>();
        let wall = public.get_mut(wall).unwrap();
        wall.regular_heights = None;
        wall.freeform_heights = None;
    }

    #[test]
    fn real_event_is_consumed_once_and_uses_the_existing_duck_audio() {
        let (mut world, wall, mut schedule) = world_with_wall(circle(), 0.0);
        let (entity, calls) = add_duck(&mut world, Vec3::new(0.0, 1.0, 0.0));
        world.clear_trackers();
        schedule.run(&mut world);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        check(&mut world, wall);
        schedule.run(&mut world);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(!world.entity(entity).get_ref::<Duck>().unwrap().is_changed());
        // 鸭子不必持有 DuckState 或旧的额外音频组件；重复运行不能重读同一事件。
        schedule.run(&mut world);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        check(&mut world, wall);
        check(&mut world, wall);
        schedule.run(&mut world);
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn circle_boundary_and_inclusive_vertical_bounds_use_the_authoritative_shape() {
        let (mut world, wall, mut schedule) = world_with_wall(circle(), 0.5);
        set_height(&mut world, wall, 4.0);
        let positions = [
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            Vec3::new(0.0, 0.49, 0.0),
            Vec3::new(0.0, 4.01, 0.0),
            Vec3::new(3.0, 1.0, 0.0),
            Vec3::new(2.99, 1.0, 0.0),
            Vec3::new(f32::NAN, 1.0, 0.0),
            Vec3::new(0.0, f32::NAN, 0.0),
        ];
        let calls: Vec<_> = positions
            .into_iter()
            .map(|p| add_duck(&mut world, p).1)
            .collect();
        check(&mut world, wall);
        schedule.run(&mut world);
        let actual: Vec<_> = calls.iter().map(|c| c.load(Ordering::SeqCst)).collect();
        assert_eq!(actual, [1, 1, 0, 0, 0, 1, 0, 0]);
    }

    #[test]
    fn rotated_rectangle_uses_its_axes_and_includes_touching_edges() {
        let shape = ShapeParameters::from_shape(Shape::Rectangle(Rectangle2d {
            axis: Vec2::Y,
            center: Vec2::new(5.0, 6.0),
            size: Vec2::new(2.0, 8.0),
        }));
        let (mut world, wall, mut schedule) = world_with_wall(shape, 0.0);
        let calls: Vec<_> = [Vec3::new(9.0, 1.0, 6.0), Vec3::new(5.0, 1.0, 7.01)]
            .into_iter()
            .map(|p| add_duck(&mut world, p).1)
            .collect();
        check(&mut world, wall);
        schedule.run(&mut world);
        assert_eq!(calls[0].load(Ordering::SeqCst), 1);
        assert_eq!(calls[1].load(Ordering::SeqCst), 0);
    }

    #[test]
    fn elevation_threshold_and_nan_are_not_replaced_with_default_zero() {
        for (elevation, expected) in [
            (1.5, 1),
            (f32::from_bits(1.5f32.to_bits() + 1), 0),
            (f32::NAN, 0),
        ] {
            let (mut world, wall, mut schedule) = world_with_wall(circle(), elevation);
            let (_, calls) = add_duck(&mut world, Vec3::new(0.0, 2.0, 0.0));
            check(&mut world, wall);
            schedule.run(&mut world);
            assert_eq!(calls.load(Ordering::SeqCst), expected);
        }
        let (mut world, wall, mut schedule) = world_with_wall(circle(), 0.0);
        set_height(&mut world, wall, f32::NAN);
        let (_, calls) = add_duck(&mut world, Vec3::new(0.0, 1.0, 0.0));
        check(&mut world, wall);
        schedule.run(&mut world);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn real_roof_extends_the_upper_bound_but_missing_stale_and_nonroof_entities_do_not() {
        for mode in 0..4 {
            let shape = circle();
            let (mut world, wall, mut schedule) = world_with_wall(shape, 0.0);
            set_height(&mut world, wall, 4.0);
            let mut bytes = [0; 88];
            bytes[8..36].copy_from_slice(&shape.to_record());
            bytes[0x30..0x34].copy_from_slice(&0.5f32.to_le_bytes());
            let roof = Roof::from_observed(bytes);
            assert!(roof.height() > 0.0);
            let roof_y = 4.0 + roof.height();
            if mode != 0 {
                let entity = if mode == 3 {
                    world.spawn_empty().id()
                } else {
                    world.spawn(roof).id()
                };
                world
                    .resource_mut::<PublicWalls>()
                    .associate_roof(wall, entity);
                if mode == 2 {
                    world.despawn(entity);
                }
            }
            let (_, calls) = add_duck(&mut world, Vec3::new(0.0, roof_y, 0.0));
            check(&mut world, wall);
            schedule.run(&mut world);
            assert_eq!(calls.load(Ordering::SeqCst), usize::from(mode == 1));
        }
    }

    #[test]
    fn freehand_and_elevated_buildings_skip_height_reads() {
        for freehand in [false, true] {
            let (mut world, wall, mut schedule) =
                world_with_wall(circle(), if freehand { 0.0 } else { 1.6 });
            if freehand {
                let mut public = world.resource_mut::<PublicWalls>();
                let wall = public.get_mut(wall).unwrap();
                let mut snapshot = wall.authority.snapshot().clone();
                let mut bytes = [0; 28];
                bytes[..4].copy_from_slice(&2u32.to_le_bytes());
                snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
                wall.authority = WallState::from_snapshot(snapshot, 0);
            }
            remove_height(&mut world, wall);
            let (_, calls) = add_duck(&mut world, Vec3::new(0.0, 2.0, 0.0));
            check(&mut world, wall);
            schedule.run(&mut world);
            assert_eq!(calls.load(Ordering::SeqCst), 0);
        }
    }

    #[test]
    fn no_ducks_do_not_read_the_height_cache() {
        let (mut world, wall, mut schedule) = world_with_wall(circle(), 0.0);
        remove_height(&mut world, wall);
        check(&mut world, wall);
        schedule.run(&mut world);
    }

    #[test]
    #[should_panic]
    fn missing_wall_panics_even_without_ducks() {
        let (mut world, _, mut schedule) = world_with_wall(circle(), 0.0);
        check(&mut world, WallId(999));
        schedule.run(&mut world);
    }

    #[test]
    #[should_panic(expected = "wall heights have not been calculated")]
    fn missing_height_panics_before_rejecting_a_duck_outside_the_shape() {
        let (mut world, wall, mut schedule) = world_with_wall(circle(), 0.0);
        remove_height(&mut world, wall);
        add_duck(&mut world, Vec3::new(99.0, 1.0, 99.0));
        check(&mut world, wall);
        schedule.run(&mut world);
    }
}

use country_core::render::extract::ComponentWrite;
