//! B140819c00：矩形编辑快照到装饰锚点、墙空间距离、碰撞归档的真实链路。
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use utils::geometry::Rectangle2d;

use crate::decorator_on_wall_height_changed::{
    get_wall_coord, set_anchor_ws, set_wall_coord, side_from_tag_byte,
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

/// 原非角锚点重映射：以before坐标系找主轴，仅主轴随尺寸/中心变化。
/// B140819ef6..f56、14081a174..228；不能把两轴一同缩放或投到当前矩形。
pub fn remap_rectangle_anchor(before: Rectangle2d, after: Rectangle2d, position: Vec2) -> Vec2 {
    let inverse = before.orientation_mat2().inverse();
    let old_center = inverse * before.center;
    let new_center = inverse * after.center;
    let shift = new_center - old_center;
    let mut local = inverse * position - old_center;
    let ratio = local / (before.size * 0.5);
    // 原MAXPS/MINPS后显式unordered blend将NaN取下限-1。
    let bound = |value: f32| {
        if value.is_nan() {
            -1.0
        } else {
            value.clamp(-1.0, 1.0)
        }
    };
    let ratio = Vec2::new(bound(ratio.x), bound(ratio.y));
    if ratio.x.abs() > ratio.y.abs() {
        local.x = (ratio.x * after.size.x) * 0.5 + shift.x;
    } else {
        local.y = (ratio.y * after.size.y) * 0.5 + shift.y;
    }
    after.orientation_mat2() * local + before.center
}

/// 静止矩形的同一原算法；保留既有调用接口，不提供退化尺寸早退。
pub fn project_onto_rectangle(rectangle: Rectangle2d, position: Vec2) -> Vec2 {
    remap_rectangle_anchor(rectangle, rectangle, position)
}

/// 使用首次归档的锚点，防止跨帧累加重映射。原角分支还把coord.y改为anchor.y。
pub fn remap_decorator_on_rectangle(
    info: &mut country_core::resources::walls::decorator::DecoratorInfo,
    before: Rectangle2d,
    after: Rectangle2d,
    archived_anchor: Vec3,
    attachment: u8,
) {
    let (position, vertical) = match rectangle_corner_id(attachment) {
        Some(corner) => (after.get_point(corner as usize), archived_anchor.y),
        None => (
            remap_rectangle_anchor(
                before,
                after,
                Vec2::new(archived_anchor.x, archived_anchor.z),
            ),
            get_wall_coord(info).y,
        ),
    };
    set_anchor_ws(info, Vec3::new(position.x, archived_anchor.y, position.y));
    // 原14081a165/277均是MULSS，产出墙空间距离，而不是归一化u。
    set_wall_coord(
        info,
        utils::wall_space::WallSpaceCoord::new_unchecked(
            after.get_closest_u_from_pos(position) * after.perimeter(),
            vertical,
        ),
    );
}

/// 原140819c00五参数，取最后一帧快照；读取编辑后的InnerWalls而非延迟发布表。
pub fn decorator_on_rectangle_edited(
    walls: Res<country_core::resources::walls::InnerWalls>,
    mut events: EventReader<country_core::systems::wall::shapes::rectangle::OnRectangleEdited>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<
        country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    >,
    resources: country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
) {
    use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
    use country_core::resources::walls::publication_fields::WallId as ArchivedWallId;
    use country_core::systems::wall::shapes::rectangle::{
        WipRectangle, edit_decision::RectangleEditState,
    };
    let Some(event) = events.read().last() else {
        return;
    };
    let state = RectangleEditState(event.0);
    if state.tag() == 0 {
        return;
    }
    let wall = WallId(state.wall().expect("矩形快照标签无效").0);
    let before = WipRectangle(event.0).before();
    archivist.archive_current_decorators_in_storages_with_resources(
        &[
            DecoratorStorageAddr::Wall(wall),
            DecoratorStorageAddr::Terrain,
        ],
        &storage,
        &resources,
        "rectangle edit",
    );
    archivist.decorator_cull_allow_wall(wall, &storage);
    let source = walls.get(wall).expect("矩形编辑墙不存在于InnerWalls");
    let shape = source
        .get_enclosure_params()
        .expect("矩形编辑墙缺少enclosure");
    let after = ShapeParameters::from_snapshot(shape).as_rectangle();
    let Some(bucket) = storage.wall_decorators_mut(wall) else {
        return;
    };
    // 原MaybeMut::get_mut@140a1b1e0仅side0登记modified；拆借字段避免逐帧复制ID。
    let modified = &mut bucket.modified_ids;
    for (&id, info) in &mut bucket.decorators {
        match side_from_tag_byte(info.dst_tag()) {
            0 => {
                modified.insert(id);
                let archived = archivist
                    .walls
                    .get(&ArchivedWallId(wall.0))
                    .and_then(|bucket| bucket.items.get(&id))
                    .expect("矩形编辑装饰缺少首次归档");
                let f = |at| f32::from_le_bytes(archived.0[at..at + 4].try_into().unwrap());
                let anchor = Vec3::new(f(0x14), f(0x18), f(0x1c));
                let attachment = archived.0[0x20];
                remap_decorator_on_rectangle(info, before, after, anchor, attachment);
            }
            1 => {}
            _ => panic!("矩形编辑装饰包含原版不支持的目的地side"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn saved_anchor_uses_dominant_axis_ties_and_degenerate_sizes() {
        let old = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::splat(4.0));
        let new = Rectangle2d::new_axis_aligned(Vec2::new(2.0, 3.0), Vec2::new(8.0, 6.0));
        assert_eq!(
            remap_rectangle_anchor(old, new, Vec2::new(1.0, 0.5)),
            Vec2::new(4.0, 0.5)
        );
        assert_eq!(
            remap_rectangle_anchor(old, new, Vec2::new(1.0, 1.0)),
            Vec2::new(1.0, 4.5)
        );
        let mut zero = old;
        zero.size = Vec2::ZERO;
        assert_eq!(
            remap_rectangle_anchor(zero, new, Vec2::ZERO),
            Vec2::new(0.0, 0.0)
        );
        assert!(remap_rectangle_anchor(old, new, Vec2::splat(f32::NAN)).is_nan());
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
            let actual = remap_rectangle_anchor(old, new, point);
            for bits in expected {
                assert!(fixture.contains(&format!("{bits:08x}")));
            }
            assert_eq!([actual.x.to_bits(), actual.y.to_bits()], expected);
        }
    }

    use crate::decorator_on_wall_height_changed::get_anchor_ws;
    use country_core::resources::walls::decorator::DecoratorInfo;

    fn rect() -> Rectangle2d {
        Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(10.0, 4.0))
    }

    #[test]
    fn actual_world_archive_survives_two_frames_and_restores_without_drift() {
        use country_core::{
            resources::walls::{
                InnerWalls,
                decorator::{DecoratorId, DecoratorInfo},
                decorator_archivist::LiveDecoratorArchivist,
                decorator_collision::DecoratorPhysicsColliders,
                decorator_storage::DecoratorStorageAddr,
            },
            systems::{
                collision::world::RaycastWorld,
                wall::{
                    shapes::rectangle::{OnRectangleEdited, WipRectangle},
                    valid_enclosure::Shape,
                },
            },
        };
        let before = rect();
        let mut inner = InnerWalls::default();
        let curve = before.as_curve2();
        let authority_curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(
            curve.points.iter().map(|point| point.to_array()).collect(),
            false,
        )
        .unwrap();
        let wall = inner.new_wall_w_params(authority_curve, 3.0, 0, 0, false);
        inner.update_wall_state_enclosure(
            wall,
            ShapeParameters::from_shape(Shape::Rectangle(before)).to_snapshot(),
        );
        let mut original = DecoratorInfo::default();
        set_anchor_ws(&mut original, Vec3::new(4., 7., 1.));
        set_wall_coord(
            &mut original,
            utils::wall_space::WallSpaceCoord::new_unchecked(12., 9.),
        );
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(1), original);
        storage
            .decorator_to_storage
            .insert(DecoratorId(1), DecoratorStorageAddr::Wall(wall));
        let mut roof = DecoratorInfo::default();
        roof.u20 = 2 << 16;
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(2), roof);
        storage
            .decorator_to_storage
            .insert(DecoratorId(2), DecoratorStorageAddr::Wall(wall));
        let mut world = World::new();
        world.insert_resource(inner);
        world.insert_resource(storage);
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<RaycastWorld>();
        world.init_resource::<DecoratorPhysicsColliders>();
        world.init_resource::<Events<OnRectangleEdited>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_on_rectangle_edited);
        let mut wip = WipRectangle::default();
        wip.begin_dimensions(wall, 2, before, 7., Vec2::ZERO);
        let mut expected = None;
        for (width, center) in [(12., 1.), (14., 2.)] {
            world
                .resource_mut::<DecoratorStorage>()
                .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
                .modified_ids
                .clear();
            let after = Rectangle2d::new_axis_aligned(Vec2::new(center, 0.), Vec2::new(width, 4.));
            world
                .resource_mut::<InnerWalls>()
                .update_wall_state_enclosure(
                    wall,
                    ShapeParameters::from_shape(Shape::Rectangle(after)).to_snapshot(),
                );
            wip.set_current(after);
            world
                .resource_mut::<Events<OnRectangleEdited>>()
                .send(OnRectangleEdited(wip.0));
            schedule.run(&mut world);
            let actual = world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap()
                .get(DecoratorId(1))
                .unwrap();
            let bucket = world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Wall(wall))
                .unwrap();
            assert!(
                bucket.modified_ids.contains(&1),
                "原MaybeMut必须触发后续装饰重建"
            );
            assert!(
                !bucket.modified_ids.contains(&2),
                "原roof侧不申请可变记录也不标dirty"
            );
            let point = remap_rectangle_anchor(before, after, Vec2::new(4., 1.));
            assert_eq!(
                get_anchor_ws(actual).to_array().map(f32::to_bits),
                [point.x, 7., point.y].map(f32::to_bits)
            );
            assert_eq!(get_wall_coord(actual).y, 9.);
            expected = Some(*actual);
        }
        schedule.run(&mut world);
        let unchanged = world
            .resource::<DecoratorStorage>()
            .get(DecoratorStorageAddr::Wall(wall))
            .unwrap()
            .get(DecoratorId(1))
            .unwrap();
        assert_eq!(
            get_anchor_ws(unchanged),
            get_anchor_ws(&expected.unwrap()),
            "空闲帧不累计重映射"
        );
        world.resource_scope(|world, mut archivist: Mut<LiveDecoratorArchivist>| {
            archivist.extract_and_restore_to_storage(&mut world.resource_mut::<DecoratorStorage>());
            assert!(archivist.walls.is_empty());
            assert!(archivist.collision_a.is_empty());
            assert!(archivist.collision_b.is_empty());
            // 原140b2c4b0只drain归档/碰撞；culling的清理由extract_diff负责。
            assert!(archivist.culling.allowed_ids.contains(&1));
        });
        let storage = world.resource::<DecoratorStorage>();
        let restored = storage
            .get(DecoratorStorageAddr::Wall(wall))
            .unwrap()
            .get(DecoratorId(1))
            .unwrap();
        assert_eq!(get_anchor_ws(restored), get_anchor_ws(&original));
        assert_eq!(get_wall_coord(restored), get_wall_coord(&original));
        assert_eq!(
            storage.decorator_to_storage.get(DecoratorId(1)),
            Some(DecoratorStorageAddr::Wall(wall))
        );
    }

    #[test]
    fn native_rectangle_archive_anchor_bits() {
        type Case = ([u32; 6], [u32; 6], [u32; 2], [u32; 2]);
        let cases: &[Case] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-rectangle-anchor-native-cases.rs"
        ));
        let rectangle = |v: [u32; 6]| {
            let v = v.map(f32::from_bits);
            Rectangle2d {
                axis: Vec2::new(v[0], v[1]),
                center: Vec2::new(v[2], v[3]),
                size: Vec2::new(v[4], v[5]),
            }
        };
        for (index, (before, after, point, expected)) in cases.iter().copied().enumerate() {
            let actual = remap_rectangle_anchor(
                rectangle(before),
                rectangle(after),
                Vec2::from_array(point.map(f32::from_bits)),
            );
            assert_eq!(
                actual.to_array().map(f32::to_bits),
                expected,
                "原指令锚点差异case {index}"
            );
        }
    }

    #[test]
    fn corner_id_mapping_matches_accessor() {
        assert_eq!(rectangle_corner_id(0), None);
        assert_eq!(rectangle_corner_id(1), Some(0));
        assert_eq!(rectangle_corner_id(4), Some(3));
    }

    #[test]
    fn outside_point_clamps_to_rectangle_edge() {
        let projected = project_onto_rectangle(rect(), Vec2::new(8.0, 0.0));
        assert_eq!(projected, Vec2::new(5.0, 0.0));
        let projected = project_onto_rectangle(rect(), Vec2::new(0.0, -9.0));
        assert_eq!(projected, Vec2::new(0.0, -2.0));
    }

    #[test]
    fn inside_point_is_unchanged() {
        let p = Vec2::new(3.0, 1.0);
        assert_eq!(project_onto_rectangle(rect(), p), p);
    }

    #[test]
    fn corner_remap_writes_anchor_and_normalized_u() {
        let mut info = DecoratorInfo::default();
        info.u20 = 2; // corner tag 2 → corner id 1
        let anchor = Vec3::new(100.0, 7.0, 100.0);
        set_anchor_ws(&mut info, anchor);
        remap_decorator_on_rectangle(&mut info, rect(), rect(), anchor, 2);
        // 角 1 = (-w/2, +l/2) = (-5, 2)。
        let a = crate::decorator_on_wall_height_changed::get_anchor_ws(&info);
        assert_eq!(a, Vec3::new(-5.0, 7.0, 2.0));
        let coord = get_wall_coord(&info);
        assert_eq!(coord.u, 4.0);
        assert_eq!(coord.y, 7.0);
    }
}
