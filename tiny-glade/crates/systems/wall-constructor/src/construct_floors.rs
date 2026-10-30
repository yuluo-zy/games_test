//! 墙内地板构造（construct_floors.rs）。
//!
//! `construct_floors`（0x1407661e0）。**权威签名**（Ghidra
//! `get_xrefs_to(0x1407661e0)` 的 `run_unsafe` 泛型名逐字，2026-10-17 fresh）：
//!
//! ```text
//! construct_floors(
//!     Res<PublicWalls>,
//!     EventReader<OnWallChanged>,
//!     EventReader<OnWallDeleted>,
//!     Local<FloorStorage>,
//!     Commands,
//! )
//! ```
//!
//! （此前 skeleton 的参数顺序 `(changed, deleted, walls, storage, commands)`
//! 与机器码不符，已按 xref 证据更正。）
//!
//! fresh 反编译（`work/restorer-remaining-wallconstructor/evidence/0x1407661e0.c`）
//! 已解出的控制流：
//! - 先消费 **OnWallDeleted**：逐事件 `wall_id`（`EventInstance` @+8）→
//!   `FloorStorage.entities`（`HashMap<WallId, Entity>`）`remove_entry`；
//!   命中且 `Entities::contains` → `commands.entity(e).despawn()`。
//! - 再消费 **OnWallChanged**：逐事件 `wall_id` → 同样先从 `FloorStorage`
//!   `remove_entry` + despawn 旧实体；随后 `PublicWalls` 哈希表探测
//!   （墙记录 stride 0x2a8）：
//!   * 命中且 `0.0 < wall.elevation（记录 -0x20 处 f32，即 state +0x280）` 且
//!     `*(u32 *)(wall - 0x48) < 2` 才生成；
//!   * `PublicWallState::get_wall_shape()`；shape 首 qword bit0 == 0 时
//!     `log2f(max(shape+0xc, 1.0) * 3.0)`（floor 高度缩放）；
//!   * `DecoratorRank::new` / `NaniInstanceMeshDesc::from` /
//!     `ShapeParameters::center` / `xvy` / `Transform::default` 装配
//!     `NaniBundle<StoneFloorInstanceData>` + `Floor` 的 insert 命令
//!     （命令载荷 0xb8B，bundle 0x48B @payload+8，Floor 载荷其后）；
//!   * `FloorStorage` 插入 `(wall_id, new_entity)`。
//!
use bevy_ecs::prelude::*;
use country_core::components::drawable::NaniBundle;
use country_core::render::nani::{gpu_world::{GpuWorldSubsetKind, InstanceDataMarker}, instance_data::StoneFloorInstanceData, systems::{NaniInstanceDesc, NaniMeshSelector}};
use country_core::resources::walls::{Floor, public_walls::{OnWallChanged, OnWallDeleted, PublicWalls}};
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use country_core::utils::components::transform::Transform;

#[derive(Default, Debug)]
pub struct FloorStorage {
    pub entities: hashbrown::HashMap<u64, Entity>,
}

/// 原圆形半径到图集层级的指令顺序。
fn floor_circle_rank(radius: f32) -> u8 {
    let radius = if radius > 1.0 { radius } else { 1.0 };
    ((radius * 3.0).log2().max(1.0).min(5.0) as u32) as u8
}

/// B 0x1407661e0：先处理删除，再替换每个变更墙的地板。
pub fn construct_floors(
    walls: Res<PublicWalls>,
    mut changed: EventReader<OnWallChanged>,
    mut deleted: EventReader<OnWallDeleted>,
    mut storage: Local<FloorStorage>,
    mut commands: Commands,
) {
    for id in deleted.read().map(|event| event.0) {
        if let Some(entity) = storage.entities.remove(&id.0) {
            if let Ok(mut entity) = commands.get_entity(entity) { entity.despawn(); }
        }
    }
    for id in changed.read().map(|event| event.0) {
        if let Some(entity) = storage.entities.remove(&id.0) {
            if let Ok(mut entity) = commands.get_entity(entity) { entity.despawn(); }
        }
        let Some(wall) = walls.get_unchecked(id) else { continue };
        let elevation = wall.authority.elevation();
        if !(elevation > 0.0) || !wall.is_premade_shape() { continue; }
        let shape = wall.get_wall_shape().clone();
        let parameters = ShapeParameters::from_snapshot(&shape);
        let center = parameters.center();
        let rank = if shape.tag() & 1 == 0 {
            let radius = f32::from_le_bytes(shape.bytes()[12..16].try_into().unwrap());
            floor_circle_rank(radius)
        } else { 0 };
        let entity = commands.spawn((Floor { wall_id: id, shape }, NaniBundle::<StoneFloorInstanceData> {
            desc: NaniInstanceDesc { mesh: NaniMeshSelector::AtlasPart(rank as u32), subset_kind: GpuWorldSubsetKind::RoofFloor },
            transform: Transform::from_translation(glam::Vec3::new(center.x, elevation, center.y)),
            marker: InstanceDataMarker::default(),
        })).id();
        storage.entities.insert(id.0, entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::event::Events;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::{InnerWalls, WallState, public_walls::{WallPublicationDirty, WallSpatialHash}};
    use country_core::resources::walls::publication_source::TerrainHeightGrid512;
    use country_core::systems::wall::valid_enclosure::Shape;
    use utils::geometry::Circle2d;

    #[test]
    fn rank_matches_original_instruction_samples() {
        // 原 RNE 指令执行夹具，log2f 桥接 macOS；不是 Windows UCRT 全域证明。
        for (bits, expected) in [
            (0x00000000, 1),
            (0x80000000, 1),
            (0x7f800000, 5),
            (0xff800000, 1),
            (0x7fc12345, 1),
            (0x7f812345, 1),
            (0x00000001, 1),
            (0x3f800000, 1),
            (0x3eaaaaab, 1),
            (0x402aaaab, 3),
            (0x412aaaaa, 5),
            (0x422aaaab, 5),
            (0x7f7fffff, 5),
            (0x3eaaaaaa, 1),
            (0x3eaaaaab, 1),
            (0x3eaaaaac, 1),
            (0x3f2aaaaa, 1),
            (0x3f2aaaab, 1),
            (0x3f2aaaac, 1),
            (0x3faaaaaa, 1),
            (0x3faaaaab, 2),
            (0x3faaaaac, 2),
            (0x402aaaaa, 3),
            (0x402aaaab, 3),
            (0x402aaaac, 3),
            (0x40aaaaaa, 4),
            (0x40aaaaab, 4),
            (0x40aaaaac, 4),
            (0x412aaaaa, 5),
            (0x412aaaab, 5),
            (0x412aaaac, 5)
        ] {
            assert_eq!(floor_circle_rank(f32::from_bits(bits)), expected, "radius={bits:08x}");
        }
    }

    #[test]
    fn floor_is_replaced_and_removed_with_wall_events() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let state = WallState::new(curve, 7.0, 0, 0, false);
        let shape = ShapeParameters::from_shape(Shape::Circle(Circle2d {
            center: glam::Vec2::new(1.0, 2.0), radius: 3.0, rotation: 0.0,
        })).to_snapshot();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(state);
        inner.update_wall_state_enclosure(id, shape);
        inner.update_elevation(id, 4.0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0u16; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut world = World::new();
        world.insert_resource(public);
        world.insert_resource(Events::<OnWallChanged>::default());
        world.insert_resource(Events::<OnWallDeleted>::default());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        let mut schedule = Schedule::default();
        schedule.add_systems(construct_floors);
        schedule.run(&mut world);
        let first = world.query::<(Entity, &Floor, &Transform)>().iter(&world).map(|(e, f, t)| {
            assert_eq!(f.wall_id, id);
            assert_eq!(t.translation, glam::Vec3::new(1.0, 4.0, 2.0));
            e
        }).collect::<Vec<_>>();
        assert_eq!(first.len(), 1);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        schedule.run(&mut world);
        let second = world.query::<(Entity, &Floor)>().iter(&world).map(|(e, _)| e).collect::<Vec<_>>();
        assert_eq!(second.len(), 1);
        assert_ne!(first[0], second[0]);
        world.resource_mut::<Events<OnWallDeleted>>().send(OnWallDeleted(id));
        schedule.run(&mut world);
        assert_eq!(world.query::<&Floor>().iter(&world).count(), 0);
    }
    #[test]
    fn combined_events_and_ineligible_wall_states() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(curve, 7.0, 0, 0, false));
        let shape = ShapeParameters::from_shape(Shape::Circle(Circle2d {
            center: glam::Vec2::new(1.0, 2.0), radius: 3.0, rotation: 0.0,
        })).to_snapshot();
        inner.update_wall_state_enclosure(id, shape.clone());
        inner.update_elevation(id, 4.0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0u16; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut world = World::new();
        world.insert_resource(public);
        world.insert_resource(Events::<OnWallChanged>::default());
        world.insert_resource(Events::<OnWallDeleted>::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(construct_floors);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        schedule.run(&mut world);
        let old = world.query::<(Entity, &Floor)>().single(&world).unwrap().0;
        // 两类事件在同帧到达时，删除先执行，之后仍按当前墙记录重建。
        world.resource_mut::<Events<OnWallDeleted>>().send(OnWallDeleted(id));
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        schedule.run(&mut world);
        let new = world.query::<(Entity, &Floor)>().single(&world).unwrap().0;
        assert_ne!(old, new);
        for height in [0.0, -1.0, f32::NAN] {
            inner.update_elevation(id, height);
            world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
            world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
            schedule.run(&mut world);
            assert_eq!(world.query::<&Floor>().iter(&world).count(), 0, "height={height:?}");
        }
        inner.update_elevation(id, 4.0);
        // 源墙高度不是 +0x280；零高度不阻止正高程楼板。
        inner.update_wall_state_height(id, 0.0);
        inner.mark_as_non_enclosed(id);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        schedule.run(&mut world);
        assert_eq!(world.query::<&Floor>().iter(&world).count(), 0);
        inner.update_wall_state_enclosure(id, shape);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        schedule.run(&mut world);
        assert_eq!(world.query::<&Floor>().iter(&world).count(), 1);
        inner.remove(id);
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        schedule.run(&mut world);
        assert_eq!(world.query::<&Floor>().iter(&world).count(), 0);
    }

}
