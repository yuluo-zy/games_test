//! geometry.rs — 几何原语残族（WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`geometry/{plane,cube,sphere,instanced_wall}.rs`。
//! PDB 实体：`Plane` / `Cube` / `Box` / `Sphere` / `InstancedWall` /
//! `BrickSourceId` / `BrickTransformSsbo` / `BrickDrawListSsbo` /
//! `BrickPrev/NextVisibilityBitmaskSsbo` / `WallBrickInstancedDrawProxy`。
//! 墙体砖 instancing（instanced_wall）由墙构造系统生产砖变换并上传
//! `SparseInstanceBufferWriter<BrickTransformSsbo>`（wp2 复用）→ GPU 绘制列表。

#![allow(dead_code)]

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::boundary::SparseInstanceBufferWriter;
use crate::typed_gap::Gap;

/// 平面（O：`geometry::plane::Plane`；`impl$1::from` 0x140a06370）。
#[derive(Clone, Copy, Debug, Default)]
pub struct Plane(pub Gap);

/// 立方体（O：`geometry::cube::Cube`；`Box::new` 内联于 debug/startup_ivy）。
#[derive(Clone, Copy, Debug, Default)]
pub struct Cube(pub Gap);

/// 立方体盒（O：`geometry::cube::Box`；`impl$5::from` 0x140a05530）。
#[derive(Clone, Copy, Debug, Default)]
pub struct Box(pub Gap);

/// 球（O：`geometry::sphere::Sphere`；`Sphere::new` 0x140b4af60 /
/// `impl$2::try_from` 0x140a06b00 从网格试转）。
#[derive(Clone, Copy, Debug, Default)]
pub struct Sphere(pub Gap);

impl Sphere {
    /// `new`（O 0x140b4af60）。
    pub fn new(_center: Vec3, _radius: f32) -> Self {
        Sphere(Gap)
    }
    /// `try_from`（O 0x140a06b00）：从点集拟合球。
    pub fn try_from_points(_pts: &[Vec3]) -> Option<Self> {
        None
    }
}

// ---------------------------------------------------------------------------
// instanced_wall — 砖实例墙
// ---------------------------------------------------------------------------

/// 砖源 id（O：`BrickSourceId`；`from_wall_id` 0x140858750 /
/// `with_transient_stair_edge_id` 0x140858780 —— 墙体砖 vs 楼梯瞬态砖）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BrickSourceId {
    Wall(u64),
    TransientStairEdge(u64),
}

impl BrickSourceId {
    pub fn from_wall_id(wall_id: u64) -> Self {
        BrickSourceId::Wall(wall_id)
    }
    pub fn with_transient_stair_edge_id(edge_id: u64) -> Self {
        BrickSourceId::TransientStairEdge(edge_id)
    }
}

/// 砖变换（O：`BrickTransformSsbo` 载荷；`upload_to_storage_buffer` push stride
/// 0x60=96B：位置/旋转/缩放/可见位等，字段 U）。
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct BrickTransform(pub [f32; 24]);

/// 砖变换 SSBO 载体（O：`BrickTransformSsbo`；SparseInstanceBuffer 类型参数）。
pub struct BrickTransformSsbo(pub Vec<BrickTransform>);

/// 砖绘制列表 SSBO（O：`BrickDrawListSsbo`；间接绘制参数）。
pub struct BrickDrawListSsbo(pub Vec<u32>);

/// 前一帧可见位掩码（O：`BrickPrevVisibilityBitmaskSsbo`）。
pub struct BrickPrevVisibilityBitmaskSsbo(pub Vec<u32>);

/// 下一帧可见位掩码（O：`BrickNextVisibilityBitmaskSsbo`）。
pub struct BrickNextVisibilityBitmaskSsbo(pub Vec<u32>);

/// 实例墙（O：`InstancedWall`；`new` 0x140858800 / `update` 0x140858830 /
/// `upload_to_storage_buffer` 0x140858880）。
#[derive(Clone, Debug)]
pub struct InstancedWall {
    /// 源 id（O）。
    pub source_id: BrickSourceId,
    /// 砖变换表（O：`update` 整体替换，Vec<BrickTransform>）。
    pub transforms: Vec<BrickTransform>,
    pub _gap: Gap,
}

impl Default for InstancedWall {
    fn default() -> Self {
        InstancedWall {
            source_id: BrickSourceId::Wall(0),
            transforms: Vec::new(),
            _gap: Gap,
        }
    }
}

impl InstancedWall {
    /// `new`（O 0x140858800）。
    pub fn new(source_id: BrickSourceId, transforms: Vec<BrickTransform>) -> Self {
        InstancedWall {
            source_id,
            transforms,
            _gap: Gap,
        }
    }

    /// `update`（O 0x140858830）：替换砖表。
    pub fn update(&mut self, transforms: Vec<BrickTransform>) {
        self.transforms = transforms;
    }

    /// `upload_to_storage_buffer`（O 0x140858880）：锁 writer，`clear` 后逐砖
    /// `push`（0x60B stride），结束 `gpu_flush`。
    pub fn upload_to_storage_buffer(&mut self, writer: &mut SparseInstanceBufferWriter<BrickTransformSsbo>) {
        writer.clear();
        for t in &self.transforms {
            writer.push(crate::Gap);
            let _ = t;
        }
        writer.gpu_flush();
    }
}

/// 砖实例绘制代理默认（O：`impl$3::default` 0x1408587c0 —— 远距默认 AABB，
/// `0xc2c80000`=-100.0 常量）。
#[derive(Clone, Copy, Debug)]
pub struct WallBrickInstancedDrawProxy {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for WallBrickInstancedDrawProxy {
    fn default() -> Self {
        // O：默认 AABB 各轴 -100（0xc2c80000）。
        WallBrickInstancedDrawProxy {
            min: Vec3::splat(-100.0),
            max: Vec3::splat(-100.0),
        }
    }
}

/// 绘制代理位标志（O：`impl$18::fmt::impl$0::*` 常量：
/// DISCARD_ABOVE / FORCE_LOD0 / LESS_SNOWY / NO_ERASER_HIGHLIGHT / ROOF_STITCH /
/// SLIGHT_OVERLAP / STEPS）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BrickDrawProxyFlags(pub u8);

impl BrickDrawProxyFlags {
    pub const DISCARD_ABOVE: u8 = 1;
    pub const FORCE_LOD0: u8 = 2;
    pub const LESS_SNOWY: u8 = 4;
    pub const NO_ERASER_HIGHLIGHT: u8 = 8;
    pub const ROOF_STITCH: u8 = 0x10;
    pub const SLIGHT_OVERLAP: u8 = 0x20;
    pub const STEPS: u8 = 0x40;
}

/// 二维坐标辅助（O：`Vec2` 使用；无独立实体）。
pub type Vec2Pair = (Vec2, Vec2);
