//! `decorator_on_rectangle_edited.rs` — 矩形墙编辑 → 装饰沿周长重映射
//! （decorator_on_rectangle_edited 0x140819c00 + 内联 `on_rectangle_edited` 行 35）。
//!
//! 证据：`toolchain/decor-trunk/decompiled/140819c00_decorator_on_rectangle_edited.c`
//! （O 全文）。
//!
//! 流程（O）：
//! 1. 事件（步长 0x58）→ `DecoratorArchivist::archive_current_decorators_in_storages`
//!    （标签 "rectangle edit"）+ `decorator_cull_allow_wall`；
//! 2. 墙的 premade 形状（`get_enclosure_params`）→ Rectangle2d；
//! 3. 逐装饰（side 0）：
//!    * `WallCornerAttachment::to_rectangle_corner_id` 命中 → `Rectangle2d::get_point`
//!      → `xvy` → 写 `Info+0x14`（anchor_ws）；
//!    * 未命中 → `orientation_mat2` 逆变换 + `world_to_local` → 除以半尺寸 →
//!      clamp `[-1,1]`（minps/maxps + NaN 传播）→ 变换回世界 → `xvy` → 写 anchor_ws；
//!    * `get_closest_u_from_pos(position) / perimeter()` → 写 `Info+0x00`（coord.u）；
//!    * `Info+0x04` 写第二个输出（精确来源 U；本实现保留原 coord.y）。
//! 4. side 1 → 无操作；其余 → panic。
//!
//! U：
//! - `Info+0x04` 的原写入量（decomp 中 `uVar30 = extraout_RAX_03[6]` 或 `[1]`）
//!   未闭合；矩形编辑不改变竖直墙空间坐标，保留原值并登记。
//! - `WallCornerAttachment` 类型在 core 未实现（章节 O）；本文件提供
//!   [`rectangle_corner_id`] 等价助手（0x140b256f0：`{present!=0, id-1}`）。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
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

/// 矩形编辑事件（O：0x58B 记录；本模型只消费墙键）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RectangleEditedEvent {
    /// 墙 id。
    pub wall: WallId,
    /// 附加（U）。
    pub extra: u64,
}

/// 非角分支：局部投影 → 每轴除以半尺寸 → clamp[-1,1] → 反变换（O）。
pub fn project_onto_rectangle(rectangle: Rectangle2d, position: Vec2) -> Vec2 {
    let half = rectangle.size * 0.5;
    if half.x == 0.0 || half.y == 0.0 {
        // 退化矩形：原实现除法会产生 inf/nan；显式跳过，避免伪造坐标。
        return position;
    }
    let local = rectangle.world_to_local(position) / half;
    let clamped = Vec2::new(clamped_or_nan(local.x), clamped_or_nan(local.y));
    rectangle.local_to_world(clamped * half)
}

#[inline]
fn clamped_or_nan(value: f32) -> f32 {
    // 原 minps/maxps + NAN 处理：NaN 传播，其余 clamp 到 [-1,1]。
    if value.is_nan() {
        value
    } else {
        value.clamp(-1.0, 1.0)
    }
}

/// 单个装饰重映射（O）。
pub fn remap_decorator_on_rectangle(
    info: &mut country_core::resources::walls::decorator::DecoratorInfo,
    shape: &ShapeParameters,
    rectangle: Rectangle2d,
) {
    let anchor = get_anchor_ws(info);
    let position = Vec2::new(anchor.x, anchor.z);
    let new_position = match rectangle_corner_id((info.u20 & 0xff) as u8) {
        Some(corner) => {
            let point = rectangle.get_point(corner as usize);
            // `xvy`：保留原 y。
            set_anchor_ws(info, Vec3::new(point.x, anchor.y, point.y));
            point
        }
        None => {
            let projected = project_onto_rectangle(rectangle, position);
            set_anchor_ws(info, Vec3::new(projected.x, anchor.y, projected.y));
            projected
        }
    };
    // O：`get_closest_u_from_pos(..) / perimeter()` 写入 Info+0x00。
    let perimeter = shape.perimeter();
    if perimeter > 0.0 {
        let u = shape.get_closest_u_from_pos(new_position) / perimeter;
        let mut coord = get_wall_coord(info);
        coord.u = u;
        set_wall_coord(info, coord);
    }
}

/// `decorator_on_rectangle_edited` @0x140819c00（O 结构）。
pub fn decorator_on_rectangle_edited<C: Send + Sync + 'static>(
    walls: Res<PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    mut events: EventReader<RectangleEditedEvent>,
) {
    for event in events.read() {
        // O：归档当前装饰（标签 "rectangle edit"）。
        archivist.archive_current_decorators_in_storages(std::slice::from_ref(&*storage));
        // O：cull 放行查询（原返回值未参与后续写入；保留调用）。
        let cull_allowed = archivist.decorator_cull_allow_wall(event.wall);
        log::trace!(
            "decorator_on_rectangle_edited: wall {:?} cull_allowed={cull_allowed}",
            event.wall
        );
        let Some(state) = walls.get(event.wall) else {
            log::warn!(
                "decorator_on_rectangle_edited: wall {:?} missing from PublicWalls",
                event.wall
            );
            continue;
        };
        // O：get_enclosure_params → 矩形；circle/non-premade 不属于本系统语义。
        let shape = ShapeParameters::from_snapshot(state.shape());
        let Some(Shape::Rectangle(rectangle)) = shape.get_shape() else {
            continue;
        };
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        for (_id, info) in bucket.iter_mut() {
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => remap_decorator_on_rectangle(info, &shape, rectangle),
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

    fn rect() -> Rectangle2d {
        Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(10.0, 4.0))
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
        let shape = ShapeParameters::from_shape(Shape::Rectangle(rect()));
        let mut info = DecoratorInfo::default();
        info.u20 = 2; // corner tag 2 → corner id 1
        let anchor = Vec3::new(100.0, 7.0, 100.0);
        set_anchor_ws(&mut info, anchor);
        remap_decorator_on_rectangle(&mut info, &shape, rect());
        // 角 1 = (-w/2, +l/2) = (-5, 2)。
        let a = crate::decorator_on_wall_height_changed::get_anchor_ws(&info);
        assert_eq!(a, Vec3::new(-5.0, 7.0, 2.0));
        let coord = get_wall_coord(&info);
        assert!(coord.u > 0.0 && coord.u < 1.0);
    }
}
