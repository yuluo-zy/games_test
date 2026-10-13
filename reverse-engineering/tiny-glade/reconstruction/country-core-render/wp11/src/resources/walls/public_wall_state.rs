//! public_wall_state.rs — 公共墙状态（WP11 状态家族）。
//!
//! 源骨架 `resources/walls/public_wall_state.rs`（B）；PDB 命名实体 `PublicWallState`。
//! 记录大小 0x2a0（O，先验 `reconstruction/building/wall_state.rs` + 本轮反编译）。
//! 存储于 `PublicWalls: HashMap<WallId, PublicWallState>`（O，PDB hashbrown 泛型）。
//! 形状记录与 Roof 共享同一 28B 布局（Roof+8..+36，先验 roof_spatial）。

#![allow(dead_code, unused_imports)]

use crate::resources::walls::WallId;
use crate::typed_gap::Gap;

/// 形状载荷：Circle = 4×f32（16B），Rectangle = 6×f32（24B），Freehand = 无固定载荷。
/// 记录：判别 u32 + 24B 联合（+0x258 起 28B，O）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WallShape {
    /// 判别 0。载荷 +0x25c..0x26c（O：is_circle 0x140b2a7d0 复制 16B）。
    Circle(CircleShape),
    /// 判别 1。载荷 +0x25c..0x274（O：is_rectangle 0x140b2a790 复制 24B）。
    Rectangle(RectangleShape),
    /// 判别 2。徒手形状，无固定载荷（O：get_wall_shape 对 2 panic）。
    Freehand,
}

/// Circle 形状载荷（O：4×f32；内字段名 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CircleShape(pub [f32; 4]);

/// Rectangle 形状载荷（O：6×f32；内字段名 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectangleShape(pub [f32; 6]);

impl WallShape {
    /// O：判别 0/1 = premade。
    #[inline]
    pub fn is_premade(self) -> bool {
        matches!(self, WallShape::Circle(_) | WallShape::Rectangle(_))
    }
    /// O：判别字节。
    #[inline]
    pub fn tag(self) -> u32 {
        match self {
            WallShape::Circle(_) => 0,
            WallShape::Rectangle(_) => 1,
            WallShape::Freehand => 2,
        }
    }
}

/// max_y / max_y_gable 缓存缺失错误（O：max_y 0x140b2a620 存在位否则 panic_fmt）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeightCacheError {
    MissingMaximum,
    MissingGableMaximum,
}

/// 墙空间（u 区间）。返回类型 U（typed gap）；语义：装饰/烟囱/窗/旗等消费方
/// 沿墙可用空间放置（is_enclosed 参与，行161/173）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WallSpace(pub Gap);

/// u 参数区间（归一 0..1）。返回类型 U；构造拱/楣/分段消费（行177，is_enclosed 参与）。
#[derive(Clone, Copy, Debug, Default)]
pub struct USpace(pub Gap);

/// `corner_pos_and_normal_xz` 输出（O：0x140b2a800）：{present u32, corner Vec2, normal Vec2}。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CornerPosAndNormalXz {
    /// 仅 Rectangle 形状提供（O：判别==1 才计算）。
    pub present: bool,
    /// corner world-space XZ。
    pub corner: [f32; 2],
    /// corner 法线 XZ。
    pub normal: [f32; 2],
}

/// 公共墙状态记录（O：0x2a0）。
///
/// 观察字段：+0x10 raw Curve2 点计数 u64；+0x30 raw 长度 f32；+0xe8 附加数组长度 u64（U 名）；
/// +0x108/+0x10c max_y 缓存位/值；+0x110/+0x114 gable 缓存位/值；
/// +0x258..0x274 形状记录；+0x274/+0x278/+0x27c min/max position y + min relative height；
/// +0x284 constrained_extra；+0x28c published 长度；+0x299 style。
#[derive(Clone, Debug)]
pub struct PublicWallState([u8; 0x2a0]);

impl Default for PublicWallState {
    fn default() -> Self {
        Self([0; 0x2a0])
    }
}

impl PublicWallState {
    /// 从观察字节构造（O，先验 wall_state.rs 同投影）。
    pub fn from_observed(bytes: [u8; 0x2a0]) -> Self {
        Self(bytes)
    }
    pub fn raw_bytes(&self) -> &[u8; 0x2a0] {
        &self.0
    }

    fn float_at(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
    fn tag(&self) -> u32 {
        u32::from_le_bytes(self.0[0x258..0x25c].try_into().unwrap())
    }

    /// O：+0x10 u64 raw Curve2 点计数。
    pub fn raw_point_count(&self) -> u64 {
        u64::from_le_bytes(self.0[0x10..0x18].try_into().unwrap())
    }
    /// O：+0x30 raw Curve2 长度。
    pub fn raw_curve_length(&self) -> f32 {
        self.float_at(0x30)
    }
    /// O：+0x28c published raw 长度（wall_space 周期/末端）。
    pub fn published_curve_length(&self) -> f32 {
        self.float_at(0x28c)
    }
    /// O：+0x299 style 字节。
    pub fn style(&self) -> u8 {
        self.0[0x299]
    }
    /// O：style >= 2 = halftimbered（WallStyle::is_halftimbered 0x1408c5670 机器码 `cmp byte,2;setae`）。
    pub fn is_halftimbered(&self) -> bool {
        self.style() >= 2
    }

    /// O：形状判别 <2（行113，is_premade_shape 内联）。
    pub fn is_premade_shape(&self) -> bool {
        self.tag() < 2
    }
    /// O：0x140b2a790。判别==1 → Rectangle 载荷。
    pub fn is_rectangle(&self) -> Option<RectangleShape> {
        (self.tag() == 1).then(|| {
            let mut v = [0f32; 6];
            for i in 0..6 {
                v[i] = self.float_at(0x25c + i * 4);
            }
            RectangleShape(v)
        })
    }
    /// O：0x140b2a7d0。判别==0 → Circle 载荷。
    pub fn is_circle(&self) -> Option<CircleShape> {
        (self.tag() == 0).then(|| {
            let mut v = [0f32; 4];
            for i in 0..4 {
                v[i] = self.float_at(0x25c + i * 4);
            }
            CircleShape(v)
        })
    }
    /// O：形状记录。返回 &WallShape（本包以 Copy 值代替引用；Freehand → unwrap panic）。
    pub fn get_wall_shape(&self) -> WallShape {
        self.try_get_wall_shape().expect("premade wall shape")
    }
    /// O：0x140b2a600。判别>=2（Freehand）→ None。
    pub fn try_get_wall_shape(&self) -> Option<WallShape> {
        match self.tag() {
            0 => self.is_circle().map(WallShape::Circle),
            1 => self.is_rectangle().map(WallShape::Rectangle),
            _ => None,
        }
    }
    /// O：0x140b2a620。+0x108 位0 存在 → +0x10c；否则 panic_fmt（本包返回 Result）。
    pub fn max_y(&self) -> Result<f32, HeightCacheError> {
        if self.0[0x108] & 1 != 0 {
            Ok(self.float_at(0x10c))
        } else {
            Err(HeightCacheError::MissingMaximum)
        }
    }
    /// O：0x140b2a680。+0x110 位0 存在 → +0x114。
    pub fn max_y_gable(&self) -> Result<f32, HeightCacheError> {
        if self.0[0x110] & 1 != 0 {
            Ok(self.float_at(0x114))
        } else {
            Err(HeightCacheError::MissingGableMaximum)
        }
    }
    /// O：0x140b2a6e0。`max_y − 0.79`（bits 0xbf4a3d71，先验 wall_height_rules::flat_roof_y）。
    pub fn flat_roof_y(&self) -> Result<f32, HeightCacheError> {
        self.max_y().map(|y| y + f32::from_bits(0xbf4a3d71))
    }
    /// O：is_enclosed（行118，valid_enclosure）。返回类型 U。
    pub fn is_enclosed(&self) -> bool {
        // 行118 内联判定；随 WP12 valid_enclosure 恢复。此处 typed gap。
        false
    }
    /// O：0x140b2a740（调 is_enclosed）。返回类型 U（typed gap）。
    pub fn wall_space(&self) -> WallSpace {
        WallSpace(Gap)
    }
    /// O：0x140b2a760。wall_space 长度。
    pub fn wall_space_length(&self) -> f32 {
        // 长度 = published 周期/空间；本包 typed gap（未恢复）。
        0.0
    }
    /// O：0x140b2a770（调 is_enclosed）。u 参数区间。返回类型 U。
    pub fn u_space(&self) -> USpace {
        USpace(Gap)
    }
    /// O：0x140b2a800。仅 Rectangle：Rectangle2d::get_point / get_corner_normal。
    pub fn corner_pos_and_normal_xz(&self) -> CornerPosAndNormalXz {
        let mut out = CornerPosAndNormalXz::default();
        if self.tag() == 1 {
            // 矩形几何从 6×f32 载荷经 utils::geometry::rectangle::Rectangle2d
            // 计算角点/法线（WP12 形状系统接线）；本包 typed gap。
            out.present = true;
        }
        out
    }
    /// O：0x140b2a890。锚高：premade → max_y；Freehand → `u × raw length`→clamp 0..1→Curve 投影。
    /// 先验 `reconstruction/building/decorator_info.rs::wall_attachment_height`。
    pub fn get_wall_attachment_height(&self, old_x: f32) -> Result<f32, HeightCacheError> {
        self.max_y()?;
        if self.tag() >= 2 {
            // Freehand：old_x × raw length → clamp 0..1 → Curve 投影（typed gap，随 WP12）。
            let _ = old_x * self.raw_curve_length();
            return Ok(0.0);
        }
        self.max_y()
    }
    /// O：常春藤生长上限（命名实体 `max_ivy_grow_height`，源码行~140）。返回类型 U。
    pub fn max_ivy_grow_height(&self) -> f32 {
        // typed gap（ivy 域 WP16 消费）。
        0.0
    }
    /// O：角法线（`corner_normal_xz`，行~215；corner_pos_and_normal_xz 前段）。返回类型 U。
    pub fn corner_normal_xz(&self) -> [f32; 2] {
        [0.0; 2]
    }
}

// ---- 便于集成引用的发布投影（先验 wall_publication.rs，不重推导）----
/// 发布度量（O：先验 `reconstruction/building/wall_publication.rs`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WallPublicationMetrics {
    pub minimum_relative_height: f32,
    pub minimum_position_y: f32,
    pub maximum_position_y: f32,
    pub constrained_extra: f32,
}
