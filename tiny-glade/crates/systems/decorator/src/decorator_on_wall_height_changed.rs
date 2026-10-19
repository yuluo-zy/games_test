//! `decorator_on_wall_height_changed.rs` — 墙高变更 → 装饰锚高响应
//! （0x14081b5e0 + 闭包 0x14081b920）。
//!
//! 证据：
//! - `toolchain/decor-trunk/decompiled/14081b5e0_decorator_on_wall_height_changed.c`：
//!   系统由 **4 个事件 reader** 组成（0x30B / 0x10B 步长，逐个游标推进）；
//!   每个事件调用同一闭包。
//! - `toolchain/decor-trunk/decompiled/14081b920_closure_0.c`（O 全文）：
//!   事件墙 id → `wall_decorators_mut(wall)` → 遍历 modified 装饰 →
//!   side 公式（`(b-2)<=2 ? b-1 : 0`）：
//!   * side 0：`dst_as_wall_mut` + `PublicWalls::get(wall)`（缺失 unwrap panic）
//!     + `get_wall_attachment_height(info+4)`；
//!   * 写 `Info+0x10 = Info+4 - height`（f32 直写）；
//!   * side 1：无操作；
//!   * 其余：panic（原不可达）。
//!
//! B 布局（building-decorator-info/findings.md）：
//! `+0x00/+0x04 WallSpaceCoord`、`+0x10 wall_height_relative_y`、
//! `+0x14..0x20 anchor_ws Vec3`、`+0x20 corner`、`+0x21 is_bottom_door`、
//! `+0x22 side/dst tag`。本文件提供这些原始偏移的读写助手（供同组文件复用）。
//!
//! U：原系统 4 个事件流的各自载荷类型/生产者未取证；本模型以单一
//! [`WallHeightChangedEvent`] 承载（每个事件仍执行同一闭包逻辑）。

use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use utils::wall_space::WallSpaceCoord;

/// 墙高变更事件（I：4 个原始事件流之一的载荷模型；`wall` 为事件墙）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallHeightChangedEvent {
    /// +0x00：墙 id（原始闭包 `**(u32**)(env+0x10) = event.wall`）。
    pub wall: WallId,
    /// +0x08：附加（U）。
    pub extra: u64,
}

/// 侧字节公式（O：`(b-2)<=2 ? b-1 : 0`；tag∈{2,3,4} → 1/2/3，其余 0）。
#[inline]
pub(crate) fn side_from_tag_byte(b: u8) -> u8 {
    if 2 < b.wrapping_sub(2) {
        0
    } else {
        b.wrapping_sub(1)
    }
}

/// `Info+0x00/+0x04`：WallSpaceCoord（O）。
#[inline]
pub fn get_wall_coord(info: &DecoratorInfo) -> WallSpaceCoord {
    WallSpaceCoord::new_unchecked(
        f32::from_bits(info.dst_wall as u32),
        f32::from_bits((info.dst_wall >> 32) as u32),
    )
}

#[inline]
pub fn set_wall_coord(info: &mut DecoratorInfo, coord: WallSpaceCoord) {
    info.dst_wall = u64::from(coord.u.to_bits()) | (u64::from(coord.y.to_bits()) << 32);
}

/// `Info+0x10`：wall_height_relative_y（O）。
#[inline]
pub fn get_wall_height_relative_y(info: &DecoratorInfo) -> f32 {
    f32::from_bits(info.dst_payload as u32)
}

#[inline]
pub fn set_wall_height_relative_y(info: &mut DecoratorInfo, value: f32) {
    info.dst_payload = (info.dst_payload & 0xffff_ffff_0000_0000) | u64::from(value.to_bits());
}

/// `Info+0x14..0x20`：anchor_ws Vec3（O：3 个 f32，跨 dst_payload 高 32 位与 u18）。
#[inline]
pub fn get_anchor_ws(info: &DecoratorInfo) -> Vec3 {
    Vec3::new(
        f32::from_bits((info.dst_payload >> 32) as u32),
        f32::from_bits(info.u18 as u32),
        f32::from_bits((info.u18 >> 32) as u32),
    )
}

#[inline]
pub fn set_anchor_ws(info: &mut DecoratorInfo, value: Vec3) {
    info.dst_payload = (info.dst_payload & 0x0000_0000_ffff_ffff)
        | (u64::from(value.x.to_bits()) << 32);
    info.u18 = u64::from(value.y.to_bits()) | (u64::from(value.z.to_bits()) << 32);
}

/// `Info+0x20`：WallCornerAttachment 原始字节；`Info+0x21`：is_bottom_door（O）。
#[inline]
pub fn get_attachment_flags(info: &DecoratorInfo) -> (u8, bool) {
    ((info.u20 & 0xff) as u8, ((info.u20 >> 8) & 1) != 0)
}

/// 闭包 `FUN_14081b920`（O）：逐 modified 装饰调整相对锚高。
pub fn adjust_wall_height_decorators(
    storage: &mut DecoratorStorage,
    walls: &PublicWalls,
    wall: WallId,
) {
    let Some(bucket) = storage.wall_decorators_mut(wall) else {
        // O：`wall_decorators_mut` 返回 None（桶不存在）→ 无操作。
        return;
    };
    // O：impl$3::next + DecoratorMaybeMut::get_mut → 逐 modified 记录。
    let ids: Vec<u32> = bucket.iter_modified_ids().collect();
    if ids.is_empty() {
        // O：原迭代器为空时同样不查 PublicWalls（避免无谓 unwrap）。
        return;
    }
    for id in ids {
        let Some(info) = bucket.get_mut(country_core::resources::walls::decorator::DecoratorId(id))
        else {
            continue;
        };
        let side = side_from_tag_byte(info.dst_tag());
        match side {
            0 => {
                let wall_state = walls.get(wall).unwrap_or_else(|| {
                    panic!(
                        "assertion failed: walls.get(wall).is_some() \
                         (decorator_on_wall_height_changed.rs)"
                    )
                });
                let u = f32::from_bits((info.dst_wall >> 32) as u32);
                let height = wall_state.get_wall_attachment_height(u).unwrap_or_else(|error| {
                    panic!("get_wall_attachment_height failed: {error:?}")
                });
                // O：`*(f32*)(info+0x10) = *(f32*)(info+4) - height`。
                let value = u - height;
                info.dst_payload =
                    (info.dst_payload & 0xffff_ffff_0000_0000) | u64::from(value.to_bits());
            }
            1 => {}
            _ => unreachable!(
                "internal error: entered unreachable code (decorator_on_wall_height_changed.rs)"
            ),
        }
    }
}

/// `decorator_on_wall_height_changed` @0x14081b5e0（O 结构；4 reader 见模块文档）。
pub fn decorator_on_wall_height_changed(
    mut storage: ResMut<DecoratorStorage>,
    walls: Res<PublicWalls>,
    mut events: EventReader<WallHeightChangedEvent>,
) {
    for &WallHeightChangedEvent { wall, .. } in events.read() {
        adjust_wall_height_decorators(&mut storage, &walls, wall);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_formula_matches_closure() {
        assert_eq!(side_from_tag_byte(0), 0);
        assert_eq!(side_from_tag_byte(1), 0);
        assert_eq!(side_from_tag_byte(2), 1);
        assert_eq!(side_from_tag_byte(3), 2);
        assert_eq!(side_from_tag_byte(4), 3);
        assert_eq!(side_from_tag_byte(5), 0);
        assert_eq!(side_from_tag_byte(0xff), 0);
    }

    #[test]
    fn raw_offset_helpers_round_trip() {
        let mut info = DecoratorInfo::default();
        set_wall_coord(&mut info, WallSpaceCoord::new_unchecked(1.5, -2.5));
        assert_eq!(get_wall_coord(&info), WallSpaceCoord::new_unchecked(1.5, -2.5));
        set_wall_height_relative_y(&mut info, 0.75);
        assert_eq!(get_wall_height_relative_y(&info), 0.75);
        set_anchor_ws(&mut info, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(get_anchor_ws(&info), Vec3::new(1.0, 2.0, 3.0));
        // 坐标与相对高互不覆盖。
        assert_eq!(get_wall_coord(&info), WallSpaceCoord::new_unchecked(1.5, -2.5));
    }

    #[test]
    fn attachment_flags_read_plus_0x20_and_0x21() {
        let info = DecoratorInfo {
            u20: 0x0000_0103,
            ..DecoratorInfo::default()
        };
        assert_eq!(get_attachment_flags(&info), (3, true));
    }
}
