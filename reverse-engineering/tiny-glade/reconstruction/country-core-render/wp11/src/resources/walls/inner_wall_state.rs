//! inner_wall_state.rs — 内部墙状态与材质枚举族（WP11）。
//!
//! 源骨架 `resources/walls/inner_wall_state.rs`（B）；PDB 命名实体：
//! `ArchType` / `FreehandHeightType` / `PlasterPeelAmount` / `StructureType` /
//! `WallState` / `WallStyle` / `deserialize_wall_style::WallStyleVisitor`。
//! WallState 为 `InnerWalls: HashMap<WallId, WallState>` 的权威编辑状态。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::walls::WallId;
use crate::typed_gap::Gap;

/// 结构类型（O：字节枚举；`is_wall` RVA 0x833230 机器码 `cmp byte,0; sete` → Wall=0；
/// `is_fence` RVA 0x8c5680 机器码 `movzx byte[rcx]; ret` = 判别读取，比较内联进调用方）。
/// 变体值除 Wall=0 外 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StructureType {
    /// O：判别 0（is_wall ==0 机器码）。
    Wall = 0,
    /// I：栅栏（is_fence 判别；具体值 U）。
    Fence = 1,
    /// U：其它（arch?）。
    Other = 2,
}

impl StructureType {
    /// O：0x140833230 `cmp byte [self], 0; sete`。
    pub fn is_wall(self) -> bool {
        matches!(self, StructureType::Wall)
    }
    /// O：0x1408c5680 读取判别字节（调用方内联比较）；Fence 具体值 U。
    pub fn is_fence(self) -> bool {
        // 判别读取 O；比较语义 U。
        self.discriminant() != StructureType::Wall.discriminant()
    }
    #[inline]
    pub fn discriminant(self) -> u8 {
        self as u8
    }
}

/// 墙材质（O：字节枚举；变体 Stone=0 / StoneWithSkirts=1 / HalfTimbered=2 由
/// deserialize_wall_style 0x14085ebc0 字符串比对 "Stone"/"StoneWithSkirts"/"HalfTimbered" 证实；
/// 其余变体 U）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WallStyle {
    /// O：deserialize_wall_style 字符串 "Stone"（行 280 visit_str）。
    Stone = 0,
    /// O：字符串 "StoneWithSkirts"（15 字符）。
    StoneWithSkirts = 1,
    /// O：字符串 "HalfTimbered"（12 字符）。
    HalfTimbered = 2,
    /// U：其余材质。
    Unknown = 3,
}

impl WallStyle {
    /// O：0x1408c5670 机器码 `cmp byte [self], 2; setae` → >=2。
    pub fn is_halftimbered(self) -> bool {
        (self as u8) >= 2
    }
    /// O：源码行62 方法；把样式转换为石材版本。转换表 U。
    pub fn convert_to_stone_version(self) -> WallStyle {
        // typed gap：行62 映射未恢复。
        WallStyle::Stone
    }
}

/// 拱类型（O：字节枚举；deserialize/serialize 行27）。变体 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ArchType {
    #[default]
    Unknown = 0,
}

/// 徒手高度类型（O：字节枚举；`toggle` RVA 0x8c5660 机器码 `*self ^= 1`）。变体 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum FreehandHeightType {
    #[default]
    V0 = 0,
    V1 = 1,
}

impl FreehandHeightType {
    /// O：0x1408c5660 `*self ^= 1`（位翻转）。
    pub fn toggle(&mut self) {
        *self = if *self as u8 & 1 != 0 {
            FreehandHeightType::V0
        } else {
            FreehandHeightType::V1
        };
    }
}

/// 石膏剥离量（O：命名实体；clone 行84）。变体 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PlasterPeelAmount {
    #[default]
    Unknown = 0,
}

/// O：行169 `is_default_arch_height` RVA 0x1408c5690 机器码：
/// `movss xmm0,[全局默认]` + `cmpeqss([arch_height], xmm0)` → arch_height == 默认常量。
/// 默认常量值 U（全局 .rdata）。
pub fn is_default_arch_height(arch_height: f32) -> bool {
    // typed gap：默认拱高常量未读取；比较语义 O。
    let _ = arch_height;
    false
}

/// 内部墙状态（权威编辑状态）。
///
/// 布局 U；观察字段（先验 `wall_height_authority.rs`）：源行 +0x54 = 可编辑高度，
/// +0x58 = terrain floor。方法面按 PDB 骨架登记。
#[derive(Resource, Clone, Debug)]
pub struct WallState {
    /// 存储布局未恢复；保留资源外壳供 InnerWalls 值类型引用。
    pub gap: Gap,
}

impl Default for WallState {
    fn default() -> Self {
        Self { gap: Gap }
    }
}

impl WallState {
    /// O：行174 `new`。构造参数 U。
    pub fn new() -> Self {
        Self::default()
    }
    /// O：行195 `height`。返回可编辑高度（源 +0x54，先验 wall_height_authority）。
    pub fn height(&self) -> f32 {
        // typed gap：+0x54 高度值未接线。
        0.0
    }
    /// O：行202。形状判别 <2。
    pub fn is_premade_shape(&self) -> bool {
        // typed gap：与 PublicWallState 形状记录同语义，未接线。
        false
    }
    /// O：行211，RVA 0x1408c56b0。封闭/围合参数。返回类型 U。
    pub fn get_enclosure_params(&self) -> Gap {
        Gap
    }
    /// O：行219。valid_enclosure 判定。
    pub fn is_enclosed(&self) -> bool {
        // typed gap（WP12 valid_enclosure）。
        false
    }
    /// O：行239 已废弃空高度访问器。
    pub fn _deprecated_empty_w_height(&self) -> Gap {
        Gap
    }
}
