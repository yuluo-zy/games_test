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
//!     （coord.u）并保留 `Info+0x04` 的原墙空间高度。
//!   side 1 → 无操作；其余 → panic（tag 3 `panic()`、tag 4 panic_fmt）。
//! - `toolchain/decor-trunk/decompiled/14081c060_decorator_on_circle_edited.c`
//!   （O 全文）：事件步长 0x10；遍历整桶；side 0：
//!   `PublicWalls::get` + `WallSpaceCoord::to_world_space` +
//!   `dst_as_wall_mut` → 写 12B 到 `Info+0x14`（anchor_ws）；side 1 无操作；
//!   其余 panic。
//!
//! 原事件第三字段为曲线段 fraction，y 从现有 decorator coordinate 读取。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use utils::curve::Curve2 as SamplingCurve2;
use utils::wall_space::WallSpaceCoord;

use crate::decorator_on_wall_height_changed::{
    get_wall_coord, set_anchor_ws, set_wall_coord, side_from_tag_byte,
};

/// 圆形编辑事件（O：0x10B 步长；wall@+0x1c）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CircleEditedEvent {
    /// +0x00：附加（U）。
    pub extra: u64,
    /// 墙 id。
    pub wall: WallId,
}

/// 由 PublicWallState 构造 utils Curve2（源曲线；O：0x140c... Curve 采样面）。
pub fn sampling_curve(state: &PublicWallState) -> Option<SamplingCurve2> {
    let raw = state.simplified_curve_2d();
    let points: Vec<Vec2> = raw.points.iter().map(|p| Vec2::new(p[0], p[1])).collect();
    SamplingCurve2::try_new_from_points(points, false).ok()
}

/// `decorator_on_freehand_wall_edited` @0x14081bac0（O 结构）。
pub fn decorator_on_freehand_wall_edited(
    walls: Res<PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<country_core::resources::events::OnFreehandWallEdited>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        for &(id, segment, t) in &event.offsets {
            bucket.modified_ids.insert(id);
            let Some(info) = bucket.get_mut(DecoratorId(id)) else {
                continue;
            };
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => {
                    assert!(info.dst_tag() < 2, "freehand decorator is not wall-attached");
                    let state = walls.get(event.wall).expect("freehand decorator wall missing");
                    let curve = state.simplified_curve_2d();
                    let index = segment as usize;
                    let a = Vec2::from_array(curve.points[index]);
                    let b = Vec2::from_array(curve.points[index + 1]);
                    let p = b * t + a * (1.0 - t);
                    let y = get_wall_coord(info).y;
                    set_anchor_ws(info, Vec3::new(p.x, y, p.y));
                    let coord = WallSpaceCoord::from_curve_internal_coord(curve, index, t, y);
                    set_wall_coord(info, coord);
                }
                1 => {}
                2 => panic!(
                    "assertion failed: freehand decorator {} has unsupported destination side 2",
                    id
                ),
                _ => unreachable!(
                    "internal error: entered unreachable code (decorator_on_freehand_wall_edited.rs)"
                ),
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
        let Some(state) = walls.get(event.wall) else {
            panic!(
                "option unwrap failed: PublicWalls::get({:?}) (decorator_on_freehand_wall_edited.rs)",
                event.wall
            );
        };
        let Some(curve) = sampling_curve(state) else {
            log::warn!(
                "decorator_on_circle_edited: invalid source curve for wall {:?}",
                event.wall
            );
            continue;
        };
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        for (_id, info) in bucket.iter_mut() {
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => {
                    // O：WallSpaceCoord::to_world_space → 写 Info+0x14（12B）。
                    let coord = get_wall_coord(info);
                    let world = coord.to_world_space(&curve);
                    set_anchor_ws(info, world);
                }
                1 => {}
                2 => panic!(
                    "assertion failed: circle decorator has unsupported destination side 2"
                ),
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

    #[test]
    fn real_event_uses_published_curve_and_preserves_wall_height() {
        use country_core::resources::walls::{InnerWalls, WallState, decorator_storage::DecoratorStorageAddr, publication_source::TerrainHeightGrid512, public_walls::{WallPublicationDirty, WallSpatialHash}};
        let curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(vec![[0.0,0.0],[4.0,2.0]], false).unwrap();
        let mut inner = InnerWalls::new(); let wall = inner.new_wall_from_curve(WallState::new(curve,4.0,0,0,false));
        let grid = TerrainHeightGrid512 { encoded:vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:true };
        let mut public = PublicWalls::default(); public.consume_with_paths(&mut inner,&grid,None,&WallPublicationDirty::default(),&WallSpatialHash::default());
        let mut storage = DecoratorStorage::default(); let mut info = DecoratorInfo::default();
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(0.0,3.0));
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall)).insert(DecoratorId(9),info);
        let mut world = World::new(); world.insert_resource(public); world.insert_resource(storage); world.init_resource::<Events<country_core::resources::events::OnFreehandWallEdited>>();
        world.send_event(country_core::resources::events::OnFreehandWallEdited { wall, offsets:vec![(9,0,0.5)] });
        let mut schedule = Schedule::default(); schedule.add_systems(decorator_on_freehand_wall_edited); schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>(); let bucket = storage.get(DecoratorStorageAddr::Wall(wall)).unwrap();
        let info = bucket.get(DecoratorId(9)).unwrap();
        assert_eq!(crate::decorator_on_wall_height_changed::get_anchor_ws(info), Vec3::new(2.0,3.0,1.0));
        assert_eq!(get_wall_coord(info).y,3.0); assert!(bucket.modified_ids.contains(&9));
    }

    fn line_curve() -> SamplingCurve2 {
        SamplingCurve2::try_new_from_points(
            vec![Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0)],
            false,
        )
        .unwrap()
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
}
