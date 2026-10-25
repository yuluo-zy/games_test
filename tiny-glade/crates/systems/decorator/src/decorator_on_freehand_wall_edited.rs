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
//! U：自由墙元素 12B 中 `t` 与 `y` 的拆分（`from_curve_internal_coord` 的
//! t 实参在反编译中不可见）。本模型用具名 `t`/`y` 字段承载，并在测试中标定。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use utils::curve::{Curve2 as SamplingCurve2, CurveInternalCoord};
use utils::wall_space::WallSpaceCoord;

use crate::decorator_on_wall_height_changed::{
    get_wall_coord, set_anchor_ws, set_wall_coord, side_from_tag_byte,
};

/// 自由墙编辑元素（I：原 12B `{id, segment, f32}`；t/y 拆分 U）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreehandWallEdit {
    pub id: u32,
    /// 曲线段索引（O：`panic_bounds_check` 的索引）。
    pub segment: u32,
    /// 段内参数（`from_curve_internal_coord` t；断言 0..=1）。
    pub t: f32,
    /// 墙空间 y（O：写回 Info+0x04）。
    pub y: f32,
}

/// 自由手绘墙编辑事件（O 读面；生产者属 core wall 系统，U）。
#[derive(Event, Clone, Debug)]
pub struct FreehandWallEditedEvent {
    /// +0x00：判别/前域（U）。
    pub kind: u64,
    /// +0x08/+0x10：受影响装饰。
    pub affected: Vec<FreehandWallEdit>,
    /// 墙 id（闭包 `**(u32**)(env+0x10) = event.wall`）。
    pub wall: WallId,
}

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

/// 自由墙单装饰重定位（O：from_curve_internal_coord + xvy 写面）。
pub fn apply_freehand_edit(
    info: &mut country_core::resources::walls::decorator::DecoratorInfo,
    curve: &SamplingCurve2,
    edit: &FreehandWallEdit,
) {
    let segment = edit.segment as usize;
    assert!(
        segment + 1 < curve.points.len(),
        "panic_bounds_check: freehand curve segment {segment} out of range"
    );
    let coord = WallSpaceCoord::from_curve_internal_coord(curve, segment, edit.t, edit.y);
    let position = curve.get_pos_at_coord(CurveInternalCoord {
        segment,
        t: edit.t,
    });
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
        let Some(state) = walls.get(event.wall) else {
            panic!(
                "option unwrap failed: PublicWalls::get({:?}) (decorator_on_freehand_wall_edited.rs)",
                event.wall
            );
        };
        let Some(curve) = sampling_curve(state) else {
            log::warn!(
                "decorator_on_freehand_wall_edited: invalid source curve for wall {:?}",
                event.wall
            );
            continue;
        };
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        // O：元素 id → modified_ids。
        for edit in &event.affected {
            bucket.modified_ids.insert(edit.id);
        }
        for edit in &event.affected {
            let Some(info) = bucket.get_mut(DecoratorId(edit.id)) else {
                continue;
            };
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => apply_freehand_edit(info, &curve, edit),
                1 => {}
                2 => panic!(
                    "assertion failed: freehand decorator {} has unsupported destination side 2",
                    edit.id
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

    fn line_curve() -> SamplingCurve2 {
        SamplingCurve2::try_new_from_points(
            vec![Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0)],
            false,
        )
        .unwrap()
    }

    #[test]
    fn freehand_midpoint_updates_coord_and_anchor() {
        let curve = line_curve();
        let mut info = DecoratorInfo::default();
        let edit = FreehandWallEdit {
            id: 1,
            segment: 0,
            t: 0.5,
            y: 2.5,
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
