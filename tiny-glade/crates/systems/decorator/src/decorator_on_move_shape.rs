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
use country_core::resources::events::OnShapeMoved;

use crate::decorator_on_wall_height_changed::{
    get_anchor_ws, set_anchor_ws, side_from_tag_byte,
};

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
    mut events: EventReader<OnShapeMoved>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        // O：元素 id → modified_ids（`HashSet::insert`）。
        for &(id, _, _) in &event.offsets {
            bucket.modified_ids.insert(id);
        }
        // O：逐元素查 map + side 分派 + 位置写。
        for &(id, x, z) in &event.offsets {
            let Some(info) = bucket.get_mut(DecoratorId(id)) else {
                continue;
            };
            let side = side_from_tag_byte(info.dst_tag());
            match side {
                0 => apply_move_shape_offset(info, Vec2::new(x, z), event.center),
                1 => {}
                2 => panic!(
                    "assertion failed: move_shape decorator {} has unsupported destination side 2",
                    id
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
    fn real_event_updates_wall_decorators_and_preserves_y() {
        use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
        use country_core::resources::walls::WallId;
        let wall = WallId(7);
        let mut world = World::new();
        let mut storage = DecoratorStorage::default();
        let mut info = DecoratorInfo::default();
        set_anchor_ws(&mut info, glam::Vec3::new(5.0, 3.0, 9.0));
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall)).insert(DecoratorId(11), info);
        world.insert_resource(storage);
        world.init_resource::<Events<OnShapeMoved>>();
        world.resource_mut::<Events<OnShapeMoved>>().send(OnShapeMoved { offsets: vec![(11, 1.0, 2.0)], wall, center: Vec2::new(4.0, 5.0) });
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_on_move_shape);
        schedule.run(&mut world);
        let info = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Wall(wall)).unwrap().get(DecoratorId(11)).unwrap();
        assert_eq!(get_anchor_ws(info), glam::Vec3::new(5.0, 3.0, 7.0));
    }

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
