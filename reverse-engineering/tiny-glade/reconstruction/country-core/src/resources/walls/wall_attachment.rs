//! wall_attachment.rs — 墙锚类型（WP11 附着族；S09 backlog）。
//!
//! 源骨架 `resources/walls/wall_attachment.rs`（B）；PDB 命名实体：
//! `WallAttachmentAnchor` / `WallAttachmentSide` / `WallCornerAttachment`。
//! 序列化字段名 O（anchor 序列化器 0x1416fa690 + WallAttachment 序列化器 0x140a4b2a0）。

#![allow(dead_code, unused_imports)]

use crate::resources::walls::WallId;

/// 墙侧/壁龛枚举（O：字节 0/1；`try_dst_as_wall` = side/niche <2）。变体名 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum WallAttachmentSide {
    /// 侧 0（O）。
    #[default]
    Side0 = 0,
    /// 侧 1（O）。
    Side1 = 1,
}

/// 角附着（O：字节枚举；`is_attached` 0x140b256b0 = 字节!=0；
/// `from_rectangle_corner_id` 0x140b256c0 = id<4 否则 panic；`to_rectangle_corner_id` 0x140b256f0）。
/// 编码 I：0=None；1..=4 = 矩形角 id（0..3）+1。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum WallCornerAttachment {
    /// 无角附着（字节 0）。
    #[default]
    None = 0,
    /// 矩形角附着（字节 1..=4；内角 id = 字节-1）。I。
    RectangleCorner(u8),
}

impl WallCornerAttachment {
    /// O：0x140b256b0 字节 != 0。
    pub fn is_attached(self) -> bool {
        match self {
            WallCornerAttachment::None => false,
            WallCornerAttachment::RectangleCorner(_) => true,
        }
    }
    /// O：0x140b256c0 角 id <4 否则 panic。编码 I（id+1）。
    pub fn from_rectangle_corner_id(id: u32) -> Self {
        assert!(id < 4, "rectangle corner id out of range");
        WallCornerAttachment::RectangleCorner((id as u8).wrapping_add(1))
    }
    /// O：0x140b256f0 返回角 id。编码 I（id = 字节-1）。
    pub fn to_rectangle_corner_id(self) -> u32 {
        match self {
            WallCornerAttachment::RectangleCorner(b) => (b as u32).wrapping_sub(1),
            WallCornerAttachment::None => 0,
        }
    }
}

/// 墙锚（O；DecoratorInfo +8 起；anchor 序列化器 0x1416fa690 字段名）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallAttachmentAnchor {
    /// 平台墙 ID（u64）。O：`WallId::is_valid` 测 !=0（field 名 `platform`）。
    pub platform: WallId,
    /// 相对墙高的 y。O（field 名 `wall_height_relative_y`）。
    pub wall_height_relative_y: f32,
    /// 世界空间锚点 XZ?（3×f32）。O（field 名 `anchor_ws`）。
    pub anchor_ws: [f32; 3],
    /// 角附着。O（field 名 `corner`）。
    pub corner: WallCornerAttachment,
    /// 底门标记。O（field 名 `is_bottom_door`）。
    pub is_bottom_door: bool,
    /// 侧/壁龛。O（field 名 `side`）。
    pub side: WallAttachmentSide,
}

/// O：行40 `is_default`（逐字段默认比较，内联 14 次于序列化器）。
pub fn is_default(anchor: &WallAttachmentAnchor) -> bool {
    *anchor == WallAttachmentAnchor::default()
}
