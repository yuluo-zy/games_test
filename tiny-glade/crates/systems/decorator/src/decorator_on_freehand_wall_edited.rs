//! `decorator_on_freehand_wall_edited.rs` — 自由手绘墙/圆墙编辑 → 装饰重定位
//! （decorator_on_freehand_wall_edited 0x14081bac0 + decorator_on_circle_edited
//! 0x14081c060）。
//!
//! 证据：
//! - `toolchain/decor-trunk/decompiled/14081bac0_decorator_on_freehand_wall_edited.c`
//!   （O 全文）：事件步长 0x28；+0x08 元素数组、+0x10 元素数，元素步长 12B
//!   `{id: u32, segment: u32, value: f32 bits}`；逐 id `modified_ids.insert` →
//!   查 map → side 公式；side 0：
//!   * `PublicWalls::get(wall)`（缺失 unwrap panic）；
//!   * 曲线段 `segment`/`segment+1` 的 `panic_bounds_check`；
//!   * `xvy(...)` → 写 `Info+0x14..0x20`（anchor_ws Vec3）；
//!   * `WallSpaceCoord::from_curve_internal_coord` → 写 `Info+0x00`
//!     （coord.u）与 `Info+0x04`（元素第三个字段的 f32 bits）。
//!   side 1 → 无操作；其余 → panic（tag 3 `panic()`、tag 4 panic_fmt）。
//! - `toolchain/decor-trunk/decompiled/14081c060_decorator_on_circle_edited.c`
//!   （O 全文）：事件步长 0x10；遍历整桶；side 0：
//!   `PublicWalls::get` + `WallSpaceCoord::to_world_space` +
//!   `dst_as_wall_mut` → 写 12B 到 `Info+0x14`（anchor_ws）；side 1 无操作；
//!   其余 panic。
//!
//! 原RNE14081bd72读取元素+8为t；14081bda4/14081bddb从Info+4读取y。
//! 先前额外y字段及kind字段不属于原生业务，已统一真实生产者事件。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::roofs::geometry::surface::Curve2 as AuthorityCurve2;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use utils::wall_space::WallSpaceCoord;

use crate::decorator_on_wall_height_changed::{
    get_wall_coord, set_anchor_ws, set_wall_coord, side_from_tag_byte,
};

// 原生产者12B元素已核实；墙空间y来自Info+4，不属于事件。
pub use country_core::systems::ui_edit_freehand_walls::{
    FreehandWallEdit, OnFreehandWallEdited as FreehandWallEditedEvent,
};

pub use country_core::resources::events::OnCircleRotated as CircleEditedEvent;

/// 自由墙单装饰重定位（O：from_curve_internal_coord + xvy 写面）。
pub fn apply_freehand_edit(
    info: &mut country_core::resources::walls::decorator::DecoratorInfo,
    curve: &AuthorityCurve2,
    edit: &FreehandWallEdit,
) {
    let segment = edit.segment as usize;
    assert!(
        segment + 1 < curve.points.len(),
        "panic_bounds_check: freehand curve segment {segment} out of range"
    );
    // 原14081bda4/14081bddb读取目的地墙空间y，锚点旧高度不是这个源。
    let y = get_wall_coord(info).y;
    let coord = WallSpaceCoord::from_curve_internal_coord(curve, segment, edit.t, y);
    let position = Vec2::from_array(curve.points[segment + 1]) * edit.t
        + Vec2::from_array(curve.points[segment]) * (1.0 - edit.t);
    set_anchor_ws(info, Vec3::new(position.x, coord.y, position.y));
    set_wall_coord(info, coord);
}

/// `decorator_on_freehand_wall_edited` @0x14081bac0（O 结构）。
pub fn decorator_on_freehand_wall_edited(
    walls: Res<PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<FreehandWallEditedEvent>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        for edit in &event.affected {
            bucket.modified_ids.insert(edit.id);
            let Some(info) = bucket.get_mut(DecoratorId(edit.id)) else {
                continue;
            };
            match side_from_tag_byte(info.dst_tag()) {
                0 => {
                    // 原只在实际命中墙装饰时查公开墙；空事件不触发unwrap。
                    let state = walls.get(event.wall).expect("自由墙装饰目标公开墙不存在");
                    // 直接借用权威缓存；每个装饰不分配或重新参数化曲线。
                    apply_freehand_edit(info, state.simplified_curve_2d(), edit);
                }
                1 => {}
                2 => panic!("自由墙装饰不能附着楼梯"),
                _ => unreachable!("自由墙装饰目的地无效"),
            }
        }
    }
}

/// `decorator_on_circle_edited` @0x14081c060（O 结构）。
pub fn decorator_on_circle_edited(
    walls: Res<PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<CircleEditedEvent>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.0) else {
            continue;
        };
        for (&id, info) in &mut bucket.decorators {
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => {
                    // O：WallSpaceCoord::to_world_space → 写 Info+0x14（12B）。
                    let coord = get_wall_coord(info);
                    // 原只在真正处理墙附着时查PublicWalls；缺桶、空桶及Roof不会unwrap。
                    let state = walls.get(event.0).expect("圆旋转装饰目标公开墙不存在");
                    let world = coord.to_world_space(state.simplified_curve_2d());
                    set_anchor_ws(info, world);
                    bucket.modified_ids.insert(id);
                }
                1 => {}
                2 => {
                    panic!("assertion failed: circle decorator has unsupported destination side 2")
                }
                _ => unreachable!(
                    "internal error: entered unreachable code (decorator_on_freehand_wall_edited.rs)"
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;

    fn line_curve() -> AuthorityCurve2 {
        AuthorityCurve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap()
    }

    #[test]
    fn freehand_midpoint_updates_coord_and_anchor() {
        let curve = line_curve();
        let mut info = DecoratorInfo::default();
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(0.0, 2.5));
        let edit = FreehandWallEdit {
            id: 1,
            segment: 0,
            t: 0.5,
        };
        apply_freehand_edit(&mut info, &curve, &edit);
        let coord = get_wall_coord(&info);
        assert!((coord.u - 2.0).abs() < 1e-5);
        assert_eq!(coord.y, 2.5);
        assert_eq!(
            crate::decorator_on_wall_height_changed::get_anchor_ws(&info),
            Vec3::new(2.0, 2.5, 0.0)
        );
    }

    #[test]
    fn missing_bucket_and_empty_shared_event_do_not_require_public_wall() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
        let mut world = World::new();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<Events<FreehandWallEditedEvent>>();
        world.send_event(
            country_core::systems::ui_edit_freehand_walls::OnFreehandWallEdited {
                wall: WallId(99),
                affected: vec![FreehandWallEdit {
                    id: 7,
                    segment: 0,
                    t: 0.5,
                }],
            },
        );
        world
            .run_system_once(decorator_on_freehand_wall_edited)
            .unwrap();
        world
            .resource_mut::<Events<FreehandWallEditedEvent>>()
            .clear();
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(100)));
        world.send_event(
            country_core::systems::ui_edit_freehand_walls::OnFreehandWallEdited {
                wall: WallId(100),
                affected: vec![],
            },
        );
        world
            .run_system_once(decorator_on_freehand_wall_edited)
            .unwrap();
    }

    #[test]
    fn circle_projection_round_trips_world_position() {
        let curve = line_curve();
        let mut info = DecoratorInfo::default();
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(1.0, 3.0));
        let world = get_wall_coord(&info).to_world_space(&curve);
        set_anchor_ws(&mut info, world);
        let anchor = crate::decorator_on_wall_height_changed::get_anchor_ws(&info);
        assert!((anchor.x - 1.0).abs() < 1e-5);
        assert_eq!(anchor.y, 3.0);
        assert!(anchor.z.abs() < 1e-5);
    }

    #[test]
    fn circle_shared_rotation_event_reanchors_wall_and_compound_but_keeps_roof_and_unknown_bucket() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::walls::{InnerWalls, WallState};
        use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
        use country_core::resources::walls::public_walls::WallPublicationDirty;
        use country_core::resources::walls::publication_source::TerrainHeightGrid512;
        use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(line_curve(), 3.0, 0, 0, false));
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0.0,
            maximum_y: 0.0, sampling_fix: false,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None,
            &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut storage = DecoratorStorage::default();
        let bucket = storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall));
        for (id, tag) in [(1, 0), (2, 1), (3, 2)] {
            let mut info = DecoratorInfo { u20: tag << 16, ..Default::default() };
            set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(1.0, 3.0));
            set_anchor_ws(&mut info, Vec3::splat(-50.0));
            bucket.insert(DecoratorId(id), info);
        }
        bucket.modified_ids.clear();
        // 原先查桶再查实际墙装饰，缺桶和只有屋顶的孤桶不要求公共墙存在。
        let roof_only = WallId(999);
        let mut roof = DecoratorInfo { u20: 2 << 16, ..Default::default() };
        set_anchor_ws(&mut roof, Vec3::splat(77.0));
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(roof_only)).insert(DecoratorId(9), roof);
        let mut world = World::new();
        world.insert_resource(storage);
        world.insert_resource(public);
        world.init_resource::<Events<CircleEditedEvent>>();
        for id in [WallId(888), roof_only, wall] {
            world.send_event(country_core::resources::events::OnCircleRotated(id));
        }
        world.run_system_once(decorator_on_circle_edited).unwrap();
        let storage = world.resource::<DecoratorStorage>();
        let bucket = storage.wall_decorators(wall).unwrap();
        for id in [1, 2] {
            assert_eq!(crate::decorator_on_wall_height_changed::get_anchor_ws(bucket.get(DecoratorId(id)).unwrap()), Vec3::new(1.0, 3.0, 0.0));
            assert!(bucket.modified_ids.contains(&id));
        }
        assert_eq!(crate::decorator_on_wall_height_changed::get_anchor_ws(bucket.get(DecoratorId(3)).unwrap()), Vec3::splat(-50.0));
        assert!(!bucket.modified_ids.contains(&3));
        assert_eq!(crate::decorator_on_wall_height_changed::get_anchor_ws(storage.wall_decorators(roof_only).unwrap().get(DecoratorId(9)).unwrap()), Vec3::splat(77.0));
    }
}
