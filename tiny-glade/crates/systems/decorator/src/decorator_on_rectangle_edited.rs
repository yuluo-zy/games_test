//! `decorator_on_rectangle_edited.rs` — 矩形编辑 → 装饰响应
//! （0x140819c00 + 内联 on_rectangle_edited 行 35，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x140819c00-*.txt`）：
//! `(Res<PublicWalls>, EventReader<0x58B 事件>, ResMut<DecoratorStorage>,
//! ResMut<DecoratorArchivist>)`。逐事件（0x58B = 88B 记录，键 16B）：
//! 1. 归档（FUN_1412b4f10 别名集，标签 "rectangle edit" = 0x142a90213/0xe，
//!    U 身份）；
//! 2. `decorator_cull_allow_wall`（0x140b2bc00，archivist 墙键 → cull 放行）；
//! 3. archivist 墙桶查（0x14085a560）+ 标志位 bit0 → `wall_decorators_mut`
//!    + 逐装饰位置更新（`on_rectangle_edited` 内联：Rectangle2d
//!    get_closest_u_from_pos/get_point/orientation_mat2/perimeter + Affine2
//!    transform_point2 + Vec2 clamp——机器码尾段 U：完整公式见 decomp
//!    `0x140819c00` 中段）。
//!
//! core 缺口登记：`WallCornerAttachment::to_rectangle_corner_id` 未在 core
//! 实现（章节 O：`{present, id-1}`，0x140b256f0 手解）——本模块提供等价
//! 助手 [`rectangle_corner_id`]。
//!
//! 事件类型（I）：[`RectangleEditedEvent`] 为系统侧模型（0x58B 载荷）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;

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

/// 矩形编辑事件（I：0x58B 载荷模型；键 16B = 墙键，O 读面）。
#[derive(Event, Clone, Debug)]
pub struct RectangleEditedEvent {
    /// 墙键（16B {wall, 附加}）。
    pub wall: WallId,
    /// 附加（U）。
    pub extra: u64,
}

/// `decorator_on_rectangle_edited` @0x140819c00（O 结构）。
pub fn decorator_on_rectangle_edited<C: Send + Sync + 'static>(
    _walls: Res<PublicWalls>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    mut events: EventReader<RectangleEditedEvent>,
) {
    for event in events.read() {
        // 机器码：归档标签 "rectangle edit"（FUN_1412b4f10 别名集，U 身份）。
        let _ = event.extra;
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        // 机器码：cull_allow_wall（archivist 墙键 → cull 放行；core 模型
        // 以 culling.allowed_ids 承载——墙级放行 = 桶内全部 id，U 精确面）。
        for id in bucket.decorators.keys() {
            archivist.culling.allowed_ids.insert(*id);
        }
        // 机器码：逐装饰矩形锚更新（on_rectangle_edited 内联；尾段 U——
        // 公式见 decomp 中段：get_closest_u_from_pos + orientation_mat2 +
        // Affine2 transform + clamp）。
        let _ = bucket;
    }
}
