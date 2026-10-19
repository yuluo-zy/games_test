//! `decorator_on_move_shape.rs` — 形状移动 → 装饰位置偏移（0x140811fc0）。
//!
//! 证据：`toolchain/decor-trunk/decompiled/140811fc0_decorator_on_move_shape.c`
//! （O 全文）。
//!
//! 事件（O 布局）：
//! - 事件步长 0x30；+0x08 = 元素数组指针，+0x10 = 元素数，元素步长 12B
//!   `{id: u32, offset: Vec2}`；+0x20 = 事件级 `Vec2` 偏移。
//! - 每个元素：`modified_ids.insert(id)` → 查内嵌 map → side 公式；
//!   side 0（墙）→ 写 `Info+0x14..0x20 = xvy(elem.offset + event.offset)`；
//!   side 1（屋顶）→ 无操作；side 2/3 → panic（tag 3 panic()、tag 4
//!   panic_fmt unreachable）。
//!
//! 写面（O）：`*(u64*)(value+0x14) = vec.x/vec.y` + `*(u32*)(value+0x1c) = vec.z`
//! = 3 个 f32（anchor_ws Vec3）。
//!
//! U：`xvy` 的中间（y）实参在反编译中不可见。本实现保留既有 `anchor_ws.y`
//! （水平移动不改变锚高），并显式登记该选择；不是机器码直证。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::WallId;

use crate::decorator_on_wall_height_changed::{
    get_anchor_ws, set_anchor_ws, side_from_tag_byte,
};

/// 单元素偏移（O：12B `{id: u32, offset: Vec2}`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveShapeOffset {
    pub id: u32,
    pub offset: Vec2,
}

/// 形状移动事件（O 读面；生产者属 core wall 系统，U）。
#[derive(Event, Clone, Debug)]
pub struct MoveShapeEvent {
    /// +0x00：判别/前域（U）。
    pub kind: u64,
    /// +0x08/+0x10：受影响装饰 + 各自偏移。
    pub affected: Vec<MoveShapeOffset>,
    /// +0x20：事件级偏移。
    pub offset: Vec2,
    /// +0x18/推得：墙 id（闭包 `**(u32**)(env+0x10) = event.wall`）。
    pub wall: WallId,
}

/// 位置写入（O：anchor_ws = xvy(elem.offset + event.offset)，y 保留，见 U）。
#[inline]
pub fn apply_move_shape_offset(
    info: &mut DecoratorInfo,
    element_offset: Vec2,
    event_offset: Vec2,
) {
    let delta = element_offset + event_offset;
    let y = get_anchor_ws(info).y;
    set_anchor_ws(info, glam::Vec3::new(delta.x, y, delta.y));
}

/// `decorator_on_move_shape` @0x140811fc0（O 结构）。
pub fn decorator_on_move_shape(
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<MoveShapeEvent>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        // O：元素 id → modified_ids（`HashSet::insert`）。
        for entry in &event.affected {
            bucket.modified_ids.insert(entry.id);
        }
        // O：逐元素查 map + side 分派 + 位置写。
        for entry in &event.affected {
            let Some(info) = bucket.get_mut(DecoratorId(entry.id)) else {
                continue;
            };
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => apply_move_shape_offset(info, entry.offset, event.offset),
                1 => {}
                2 => panic!(
                    "assertion failed: move_shape decorator {} has unsupported destination side 2",
                    entry.id
                ),
                _ => unreachable!(
                    "internal error: entered unreachable code (decorator_on_move_shape.rs)"
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decorator_on_wall_height_changed::{get_wall_coord, set_wall_coord};
    use utils::wall_space::WallSpaceCoord;

    #[test]
    fn offset_adds_element_and_event_and_preserves_height() {
        let mut info = DecoratorInfo::default();
        set_anchor_ws(&mut info, glam::Vec3::new(10.0, 3.0, 10.0));
        apply_move_shape_offset(&mut info, Vec2::new(1.0, 2.0), Vec2::new(-0.5, 0.5));
        let anchor = get_anchor_ws(&info);
        // O（0x140811fc0）：写面 = xvy(elem.offset + event.offset)，**覆盖**为
        // 偏移和（不叠加旧锚）；y 保留原锚高（U：xvy 中间实参不可见）。
        assert_eq!(anchor, glam::Vec3::new(0.5, 3.0, 2.5));
    }

    #[test]
    fn move_shape_does_not_touch_wall_coord() {
        let mut info = DecoratorInfo::default();
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(1.0, 2.0));
        apply_move_shape_offset(&mut info, Vec2::new(1.0, 1.0), Vec2::ZERO);
        assert_eq!(get_wall_coord(&info), WallSpaceCoord::new_unchecked(1.0, 2.0));
    }

    #[test]
    fn side_classification_for_event_targets() {
        assert_eq!(side_from_tag_byte(0), 0);
        assert_eq!(side_from_tag_byte(2), 1);
        assert_eq!(side_from_tag_byte(3), 2);
        assert_eq!(side_from_tag_byte(4), 3);
    }
}
