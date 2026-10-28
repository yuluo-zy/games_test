//! B rectangle edit consumer (0x140819c00): last Wip event, archival and
//! saved-before to authoritative-current decorator remapping.
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::walls::InnerWalls;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator_archivist::{
    DecoratorArchivalResources, LiveDecoratorArchivist,
};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use utils::geometry::Rectangle2d;

use crate::decorator_on_wall_height_changed::{
    get_anchor_ws, get_wall_coord, set_anchor_ws, set_wall_coord, side_from_tag_byte,
};

/// 矩形角附着 id（等价 `WallCornerAttachment::to_rectangle_corner_id`
/// 0x140b256f0：`{present: self!=0, id: self-1}`；0=None，1..=4=角 id+1）。
#[inline]
pub fn rectangle_corner_id(attachment: u8) -> Option<u32> {
    if attachment == 0 {
        None
    } else {
        Some(u32::from(attachment) - 1)
    }
}

fn map_saved_anchor(old: Rectangle2d, rectangle: Rectangle2d, anchor: Vec2) -> Vec2 {
    let inverse = old.orientation_mat2().inverse();
    let old_center = inverse * old.center;
    let delta = inverse * rectangle.center - old_center;
    let q = inverse * anchor - old_center;
    let normalized = q / (old.size * 0.5);
    let clamp = |v: f32| {
        let low = if v > -1.0 { v } else { -1.0 };
        if low < 1.0 { low } else { 1.0 }
    };
    let v = Vec2::new(clamp(normalized.x), clamp(normalized.y));
    let local = if v.x.abs() > v.y.abs() {
        Vec2::new(v.x * rectangle.size.x * 0.5 + delta.x, q.y)
    } else {
        Vec2::new(q.x, v.y * rectangle.size.y * 0.5 + delta.y)
    };
    old.center + rectangle.orientation_mat2() * local
}

/// `decorator_on_rectangle_edited` @0x140819c00（O 结构）。
pub fn decorator_on_rectangle_edited(
    walls: Res<InnerWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    archival: DecoratorArchivalResources,
    mut events: EventReader<
        country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::OnRectangleEdited,
    >,
) {
    if let Some(event) = events.read().last() {
        let tag = u32::from_le_bytes(event.0[..4].try_into().unwrap());
        if tag == 0 {
            return;
        }
        let (wall_at, rect_at) = if tag == 1 { (16, 32) } else { (8, 24) };
        let wall = WallId(u64::from_le_bytes(
            event.0[wall_at..wall_at + 8].try_into().unwrap(),
        ));
        let mut previous = [0u8; 28];
        previous[..4].copy_from_slice(&1u32.to_le_bytes());
        previous[4..].copy_from_slice(&event.0[rect_at..rect_at + 24]);
        let old = ShapeParameters::from_observed(previous).as_rectangle();
        use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
        archivist.archive_current_decorators_in_storages_with_resources(
            &[
                DecoratorStorageAddr::Wall(wall),
                DecoratorStorageAddr::Terrain,
            ],
            &storage,
            &archival,
            "rectangle edit",
        );
        archivist.decorator_cull_allow_wall(wall, &storage);
        let state = walls.get(wall).expect("rectangle edited wall missing");
        let shape = ShapeParameters::from_snapshot(
            state
                .get_enclosure_params()
                .expect("rectangle enclosure missing"),
        );
        let Some(Shape::Rectangle(rectangle)) = shape.get_shape() else {
            panic!("rectangle enclosure expected")
        };
        let Some(bucket) = storage.wall_decorators_mut(wall) else {
            return;
        };
        let ids: Vec<_> = bucket.iter_maybe_mut().map(|(id, _)| id).collect();
        for id in ids {
            let side = side_from_tag_byte(
                bucket
                    .get(country_core::resources::walls::decorator::DecoratorId(id))
                    .unwrap()
                    .dst_tag(),
            );
            match side {
                0 => {
                    bucket.modified_ids.insert(id);
                    let info = bucket
                        .get_mut(country_core::resources::walls::decorator::DecoratorId(id))
                        .unwrap();
                    assert!(
                        info.dst_tag() < 2,
                        "rectangle decorator is not attached to a wall"
                    );
                    let original = archivist
                        .archived_wall_decorator(wall, id)
                        .expect("archived rectangle decorator missing");
                    let anchor = get_anchor_ws(&original);
                    let point = if let Some(corner) = rectangle_corner_id((info.u20 & 0xff) as u8) {
                        rectangle.get_point(corner as usize)
                    } else {
                        map_saved_anchor(old, rectangle, Vec2::new(anchor.x, anchor.z))
                    };
                    set_anchor_ws(info, Vec3::new(point.x, anchor.y, point.y));
                    let mut coord = get_wall_coord(info);
                    coord.u = shape.get_closest_u_from_pos(point) / shape.perimeter();
                    if rectangle_corner_id((info.u20 & 0xff) as u8).is_some() {
                        coord.y = anchor.y;
                    }
                    set_wall_coord(info, coord);
                }
                1 => {}
                2 => panic!(
                    "assertion failed: rectangle decorator has unsupported destination side 2"
                ),
                _ => unreachable!(
                    "internal error: entered unreachable code (decorator_on_rectangle_edited.rs)"
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;

    #[test]
    fn saved_anchor_uses_dominant_axis_ties_and_degenerate_sizes() {
        let old = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(4.0));
        let new = Rectangle2d::new_axis_aligned(Vec2::new(2.0, 3.0), Vec2::new(8.0, 6.0));
        assert_eq!(
            map_saved_anchor(old, new, Vec2::new(1.0, 0.5)),
            Vec2::new(4.0, 0.5)
        );
        assert_eq!(
            map_saved_anchor(old, new, Vec2::new(1.0, 1.0)),
            Vec2::new(1.0, 4.5)
        );
        let mut zero = old;
        zero.size = Vec2::ZERO;
        assert_eq!(map_saved_anchor(zero, new, Vec2::ZERO), Vec2::new(0.0, 0.0));
        assert!(map_saved_anchor(old, new, Vec2::splat(f32::NAN)).is_nan());
    }

    #[test]
    fn saved_anchor_matches_original_instruction_fragment_bits() {
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-rectangle-consumer-fragment-fixture.json"
        ));
        let cases = [
            (
                Vec2::new(2.0, 1.0),
                Vec2::new(4.0, 4.0),
                Vec2::new(1.0, 2.0),
            ),
            (
                Vec2::new(1.0, 2.0),
                Vec2::new(4.0, 4.0),
                Vec2::new(1.0, 2.0),
            ),
            (
                Vec2::new(1.0, 1.0),
                Vec2::new(4.0, 4.0),
                Vec2::new(1.0, 2.0),
            ),
            (
                Vec2::new(0.0, 1.0),
                Vec2::new(0.0, 4.0),
                Vec2::new(1.0, 2.0),
            ),
            (
                Vec2::new(f32::NAN, 1.0),
                Vec2::new(4.0, 4.0),
                Vec2::new(1.0, 2.0),
            ),
            (Vec2::new(-0.0, 0.0), Vec2::new(4.0, 4.0), Vec2::ZERO),
        ];
        let expected = [
            [0x40a00000, 0x3f800000],
            [0x3f800000, 0x40a00000],
            [0x3f800000, 0x40600000],
            [0xc0400000, 0x3f800000],
            [0x7fc00000, 0x7fc00000],
            [0, 0],
        ];
        for ((point, size, center), expected) in cases.into_iter().zip(expected) {
            let old = Rectangle2d {
                axis: Vec2::X,
                center: Vec2::ZERO,
                size,
            };
            let new = Rectangle2d {
                axis: Vec2::X,
                center,
                size: Vec2::new(8.0, 6.0),
            };
            let actual = map_saved_anchor(old, new, point);
            for bits in expected {
                assert!(fixture.contains(&format!("{bits:08x}")));
            }
            assert_eq!([actual.x.to_bits(), actual.y.to_bits()], expected);
        }
    }

    #[test]
    fn real_event_archives_original_and_remaps_from_it_each_frame() {
        use country_core::resources::walls::{
            WallState, decorator::DecoratorId, decorator_storage::DecoratorStorageAddr,
        };
        use country_core::systems::wall::valid_enclosure::Shape;
        let before = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(4.0, 4.0));
        let after = Rectangle2d::new_axis_aligned(Vec2::new(2.0, 0.0), Vec2::new(8.0, 4.0));
        let state = WallState::new(
            country_core::systems::wall::shapes::to_authority_curve(&after.as_curve2()),
            4.0,
            0,
            0,
            false,
        );
        let mut walls = InnerWalls::new();
        let wall = walls.new_wall_from_curve(state);
        walls.update_wall_state_enclosure(
            wall,
            ShapeParameters::from_shape(Shape::Rectangle(after)).to_snapshot(),
        );
        let mut storage = DecoratorStorage::default();
        let mut info = DecoratorInfo::default();
        set_anchor_ws(&mut info, Vec3::new(1.0, 2.0, 0.0));
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(7), info);
        let mut world = World::new();
        world.insert_resource(walls);
        world.insert_resource(storage);
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<country_core::systems::collision::world::RaycastWorld>();
        world.init_resource::<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<Events<country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::OnRectangleEdited>>();
        let mut snapshot = [0u8; 80];
        snapshot[..4].copy_from_slice(&2u32.to_le_bytes());
        snapshot[8..16].copy_from_slice(&wall.0.to_le_bytes());
        snapshot[24..48].copy_from_slice(&Shape::Rectangle(before).to_record()[4..]);
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_on_rectangle_edited);
        for _ in 0..2 {
            world.send_event(country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::OnRectangleEdited(snapshot));
            schedule.run(&mut world);
            let storage = world.resource::<DecoratorStorage>();
            let saved = storage
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .get(DecoratorId(7))
                .unwrap();
            assert_eq!(get_anchor_ws(saved), Vec3::new(4.0, 2.0, 0.0));
        }
        assert_eq!(
            get_anchor_ws(
                &world
                    .resource::<LiveDecoratorArchivist>()
                    .archived_wall_decorator(wall, 7)
                    .unwrap()
            ),
            Vec3::new(1.0, 2.0, 0.0)
        );
    }

    #[test]
    fn corner_id_mapping_matches_accessor() {
        assert_eq!(rectangle_corner_id(0), None);
        assert_eq!(rectangle_corner_id(1), Some(0));
        assert_eq!(rectangle_corner_id(4), Some(3));
    }
}
