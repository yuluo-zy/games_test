//! decorator.rs — 装饰附着/信息类型（WP11 附着族；S09 backlog）。
//!
//! 源骨架 `resources/walls/decorator.rs`（B）；PDB 命名实体 27 个（见 evidence.json）。
//! 布局先验：`reconstruction/building/decorator_info.rs` + `evidence/building-decorator-info/findings.md`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::resources::walls::wall_attachment::WallAttachmentAnchor;
use crate::typed_gap::Gap;

/// 装饰 ID（O：u32 newtype；`DecoratorId::new`/`get_inner`；先验 NativeDecoratorId u32）。
/// 作为 DecoratorInfo 插入键（Info +0x2c）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct DecoratorId(pub u32);

impl DecoratorId {
    /// O：构造。
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }
    /// O：返回内值。
    pub fn get_inner(self) -> u32 {
        self.0
    }
}

/// 装饰 ID 生成器（O：`generate`/`reserve`；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct DecoratorIdGen(pub Gap);

/// 装饰信息 72B 记录（O 布局；先验 decorator-info findings）。
#[derive(Clone, Debug, Default)]
pub struct DecoratorInfo([u8; 72]);

impl DecoratorInfo {
    /// 从观察字节构造（O）。
    pub fn from_observed(bytes: [u8; 72]) -> Self {
        Self(bytes)
    }
    /// O：+0x22 目的地侧/壁龛字节；<2 = 墙目的地。
    pub fn destination_tag(&self) -> u8 {
        self.0[0x22]
    }
    /// O：+0x2c DecoratorId（插入键）。
    pub fn id(&self) -> DecoratorId {
        DecoratorId(u32::from_le_bytes(self.0[0x2c..0x30].try_into().unwrap()))
    }
    /// O：+0x44 rank。
    pub fn rank(&self) -> u8 {
        self.0[0x44]
    }
    /// O：+0x46 DecoratorType。
    pub fn decorator_type(&self) -> u8 {
        self.0[0x46]
    }
    /// O：`try_dst_as_wall`（destination_tag <2 返回墙锚）。
    pub fn try_dst_as_wall(&self) -> Option<WallAttachment> {
        (self.destination_tag() < 2).then(|| self.wall_fields())
    }
    /// O：墙锚读取（Info +0..0x24）。
    pub fn dst_as_wall(&self) -> WallAttachment {
        self.wall_fields()
    }
    /// O：`dst_as_wall_mut`（可变版）。本包以拷贝代替（typed gap）。
    pub fn dst_as_wall_mut(&mut self) -> WallAttachment {
        self.wall_fields()
    }
    /// O：`dst_as_roof`（destination_tag ==2 屋顶目的地）。
    pub fn dst_as_roof(&self) -> Option<Gap> {
        None
    }
    /// O：`dst_as_stair`（目的地 ==4 楼梯）。
    pub fn dst_as_stair(&self) -> Option<Gap> {
        None
    }
    /// O：`dst_as_terrain`（目的地 ==3 地形）。
    pub fn dst_as_terrain(&self) -> Option<Gap> {
        None
    }
    fn wall_fields(&self) -> WallAttachment {
        let f = |at| f32::from_le_bytes(self.0[at..at + 4].try_into().unwrap());
        WallAttachment {
            coord: [f(0), f(4)],
            anchor: WallAttachmentAnchor {
                platform: crate::resources::walls::WallId(u64::from_le_bytes(
                    self.0[8..16].try_into().unwrap(),
                )),
                wall_height_relative_y: f(0x10),
                anchor_ws: [f(0x14), f(0x18), f(0x1c)],
                corner: match self.0[0x20] {
                    0 => crate::resources::walls::wall_attachment::WallCornerAttachment::None,
                    b => crate::resources::walls::wall_attachment::WallCornerAttachment::RectangleCorner(b),
                },
                is_bottom_door: self.0[0x21] != 0,
                side: match self.0[0x22] & 1 {
                    0 => crate::resources::walls::wall_attachment::WallAttachmentSide::Side0,
                    _ => crate::resources::walls::wall_attachment::WallAttachmentSide::Side1,
                },
            },
        }
    }
}

/// 顶层墙附着（O：WallAttachment 序列化器 0x140a4b2a0 字段 `coord`（首 8B）+ `anchor`（+8 起））。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallAttachment {
    /// 墙空间坐标（2×f32，field `coord`）。
    pub coord: [f32; 2],
    /// 墙锚（field `anchor`）。
    pub anchor: WallAttachmentAnchor,
}

/// 目的地（O：`DecoratorDst::as_wall/as_wall_mut/as_roof/is_wall`）。布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DecoratorDst(pub Gap);

/// 附加信息联合（O：`DecoratorExtraInfo::as_chimney/as_flag/as_lantern/as_stair/as_window`
/// + `_mut` + `try_`）。布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DecoratorExtraInfo(pub Gap);

/// 装饰 rank（O：`DecoratorRank::new/x/is_atomic/is_compound`；Info +0x44；有效 1..=3）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum DecoratorRank {
    #[default]
    Unknown = 0,
}

impl DecoratorRank {
    /// O：构造。
    pub fn new(x: u8) -> Self {
        let _ = x;
        DecoratorRank::Unknown
    }
    /// O：rank 值。
    pub fn x(self) -> u8 {
        self as u8
    }
    /// O：原子 rank。
    pub fn is_atomic(self) -> bool {
        false
    }
    /// O：复合 rank。
    pub fn is_compound(self) -> bool {
        false
    }
}

/// 装饰旋转 XZ（O：`DecoratorRotationXZ::from_quat/from_xz_forward_vector/offset/to_projective_xz_mat2`）。
/// 布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DecoratorRotationXZ(pub Gap);

/// 装饰定位（O：`DecoratorLocator::new`）。布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DecoratorLocator(pub Gap);

/// 装饰类型（O：字节；window cottage 0x38 / gothic 0x39）。变体 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum DecoratorType {
    #[default]
    Unknown = 0,
}

/// Clutter 版本（O：`is_initial` RVA 0x140833230 机器码 `cmp byte,0; sete` → ==0；
/// `latest`）。字节枚举。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ClutterVersion {
    /// 判别 0（初始）。
    #[default]
    Initial = 0,
    /// 其它版本（U）。
    Other = 1,
}

impl ClutterVersion {
    /// O：0x140833230 ==0。
    pub fn is_initial(self) -> bool {
        matches!(self, ClutterVersion::Initial)
    }
    /// O：`latest` 最新版本（值 U）。
    pub fn latest() -> Self {
        ClutterVersion::Other
    }
}

/// 屋顶附着（O 命名实体；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RoofAttachment(pub Gap);
/// 地形附着（O 命名实体；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct TerrainAttachment(pub Gap);
/// 楼梯附着（O 命名实体；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct StairAttachment(pub Gap);

/// 装饰组件（O：ECS 查询消费）。布局 U。
#[derive(Component, Clone, Debug, Default)]
pub struct DecoratorInfoComponent(pub Gap);
