//! `decorator_on_wall_height_changed.rs` — 墙高变更 → 装饰锚高响应
//! （0x14081b5e0 + 闭包 0x14081b920，O）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x14081b5e0/
//! 0x14081b920-*.txt`）：
//! `decorator_on_wall_height_changed`（6 参）：四个事件流（0x10B/0x30B 元素，
//! 游标推进）→ 墙高事件 → `FUN_14081b920` 闭包（每装饰）。
//!
//! 闭包（O）：`wall_decorators_mut(wall)` → 逐 modified 装饰（0x140a1b200
//! 迭代 + 0x1400885d0 取值）：目的侧字节 `+0x22` 经 side 公式
//! `(b-2)<=2 ? b-1 : 0`；侧 0 → `dst_as_wall_mut` + `PublicWalls::get(wall)`
//! （未中 panic）→ `get_wall_attachment_height`（0x140b2a890）→ 写
//! `info+0x10`（f32）= `(f32)(info+4 u32) - height`；侧非 0/1 → panic
//! "internal error: entered unreachable code"（0x142a903f0 line 0x28，
//! file decorator_on_wall_height_changed.rs）。
//!
//! 事件类型（I）：本文件定义 [`WallHeightChangedEvent`]（0x10B 载荷）；
//! 原生事件族归属 core wall 系统（U 登记）。
//!
//! 字段写语义登记：`info+0x10`（dst_payload 低 f32）写 `(f32)(dst_wall 高
//! u32) - attachment_height` 为机器码直写（U：锚语义待装饰消费端取证）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;

/// 墙高变更事件（I：0x10B 载荷 {wall, ?}；生产者 U）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallHeightChangedEvent {
    /// +0x00：墙 id。
    pub wall: WallId,
    /// +0x08：附加（U）。
    pub extra: u64,
}

/// 侧字节公式（O：`(b-2)<=2 ? b-1 : 0`，同 DecoratorStorage::on_wall_split）。
#[inline]
pub(crate) fn side_from_tag_byte(b: u8) -> u8 {
    if 2 < b.wrapping_sub(2) {
        0
    } else {
        b.wrapping_sub(1)
    }
}

/// 闭包 `FUN_14081b920`（O）：逐 modified 装饰调整锚高。
pub fn adjust_wall_height_decorators(
    storage: &mut DecoratorStorage,
    walls: &PublicWalls,
    wall: WallId,
) {
    let Some(bucket) = storage.wall_decorators_mut(wall) else {
        return;
    };
    // 机器码：0x140a1b200 迭代内嵌 map；逐条按 side 公式处理。
    bucket.iter_mut().for_each(|(_id, info)| {
        let side = side_from_tag_byte(((info.u20 >> 16) & 0xff) as u8);
        match side {
            0 => {
                let Some(wall_state) = walls.get(wall) else {
                    panic!(
                        "assertion failed: walls.get(wall).is_some() (decorator_on_wall_height_changed.rs)"
                    );
                };
                // 机器码直写（U 锚语义）：dst_payload 低 f32 = (f32)(dst_wall
                // 高 u32) - attachment_height。
                let raw_u = (info.dst_wall >> 32) as u32;
                let as_f32 = f32::from_bits(raw_u);
                let height = wall_state
                    .get_wall_attachment_height(as_f32)
                    .expect("get_wall_attachment_height failed (decorator_on_wall_height_changed.rs)");
                let value = (as_f32 - height).to_bits();
                info.dst_payload =
                    (info.dst_payload & 0xffff_ffff_0000_0000) | u64::from(value);
            }
            1 => {}
            _ => unreachable!(
                "internal error: entered unreachable code (decorator_on_wall_height_changed.rs:40)"
            ),
        }
    });
}

/// `decorator_on_wall_height_changed` @0x14081b5e0（O 结构）。
pub fn decorator_on_wall_height_changed(
    mut storage: ResMut<DecoratorStorage>,
    walls: Res<PublicWalls>,
    mut events: EventReader<WallHeightChangedEvent>,
) {
    for &WallHeightChangedEvent { wall, .. } in events.read() {
        adjust_wall_height_decorators(&mut storage, &walls, wall);
    }
}
