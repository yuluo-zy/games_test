//! `decorator_on_freehand_wall_edited.rs` — 自由手绘墙编辑 → 装饰响应
//! （decorator_on_freehand_wall_edited 0x14081bac0 + decorator_on_circle_edited
//! 0x14081c060，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x14081bac0/
//! 0x14081c060-*.txt`）：
//! - freehand：`(Res<PublicWalls>, EventReader<0x28B 事件>, ResMut<Decorator
//!   Storage>)`；事件 `{?@0, ids ptr@8, len@0x10, wall@0x18}` → 逐 id 标
//!   modified → 逐装饰沿新曲线重定位（WallSpaceCoord::from_curve_internal_coord
//!   + Curve::get_pos_at_coord，skeleton O；尾段公式 U）。
//! - circle：`(Res<PublicWalls>, EventReader<0x10B 事件>, ResMut<Decorator
//!   Storage>)`；事件 `{wall@+8, …}` → 逐 modified 装饰：side 公式 →
//!   侧 0 → `PublicWalls` 查 + 位置计算（FUN_140c92090）→ 写
//!   `info+0x14`（8B）+ `info+0x1c`（4B）；侧非 0/1 → unreachable panic
//!   （0x142a904a8 line 0x28，file decorator_on_freehand_wall_edited.rs）。
//!
//! 事件类型（I）：系统侧模型；原生事件族归属 core wall 系统（U 登记）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;

use crate::decorator_on_wall_height_changed::side_from_tag_byte;

/// 自由手绘墙编辑事件（I：0x28B 载荷模型；字段放置 O 读面）。
#[derive(Event, Clone, Debug)]
pub struct FreehandWallEditedEvent {
    /// +0x00：判别/前域（U）。
    pub kind: u64,
    /// +0x08：受影响装饰 id 列表（ptr@+8 / len@+0x10）。
    pub affected: Vec<u32>,
    /// +0x18：墙 id。
    pub wall: WallId,
}

/// 圆形编辑事件（I：0x10B 载荷模型；wall@+8）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CircleEditedEvent {
    /// +0x00：附加（U）。
    pub extra: u64,
    /// +0x08：墙 id。
    pub wall: WallId,
}

/// `decorator_on_freehand_wall_edited` @0x14081bac0（O 结构）。
pub fn decorator_on_freehand_wall_edited(
    _walls: Res<PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<FreehandWallEditedEvent>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        for id in &event.affected {
            bucket.modified_ids.insert(*id);
        }
        // 机器码尾段：沿编辑后曲线重定位（WallSpaceCoord::from_curve_
        // internal_coord + get_pos_at_coord；公式 U——见 decomp 尾段）。
        for (_id, _info) in bucket.iter_mut() {
            // U：曲线重定位写入。
        }
    }
}

/// `decorator_on_circle_edited` @0x14081c060（O 结构）。
pub fn decorator_on_circle_edited(
    _walls: Res<PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<CircleEditedEvent>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        for (id, info) in bucket.iter_mut() {
            let side = side_from_tag_byte(((info.u20 >> 16) & 0xff) as u8);
            match side {
                0 => {
                    // 机器码：位置计算（FUN_140c92090，U 公式）→ 写
                    // info+0x14（8B）+ info+0x1c（4B）。
                    let _ = id;
                }
                1 => {}
                _ => unreachable!(
                    "internal error: entered unreachable code (decorator_on_freehand_wall_edited.rs:40)"
                ),
            }
        }
    }
}

/// 圆编辑位置写入面（机器码偏移 O；值公式 U 登记）。
#[allow(dead_code)]
pub(crate) fn _circle_position_write(
    info: &mut country_core::resources::walls::decorator::DecoratorInfo,
    position: (u32, u32),
    height: u32,
) {
    let bytes = info.dst_payload.to_le_bytes();
    let mut low = [0u8; 8];
    low[0..4].copy_from_slice(&position.0.to_le_bytes());
    low[4..8].copy_from_slice(&position.1.to_le_bytes());
    info.dst_payload = u64::from_le_bytes(low);
    info.u18 = u64::from(height);
    let _ = bytes;
}
