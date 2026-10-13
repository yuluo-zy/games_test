//! stairs_assembly_params.rs — 楼梯装配参数（WP21）。
//!
//! O：`PlayermadeStairsAssemblyParams::length_ws` 0x959ca0（points Vec<Vec2> @+0x8/
//! +0x10、heights Vec<f32> @+0x40/+0x48，zip xvy 收集 Vec3 求墙空间长）、
//! `StairWallAnchor::wall_normal` 0x959e90（(sin,cos)）、`AlignKind::outgoing_dir`
//! 0x959f50（判别 0/1 否则 panic）、`StairNodeAnchor::try_as_wall` 0x959ef0。

use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

/// `AlignKind`（O：对齐种类；判别 0/1）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignKind {
    /// 沿墙。
    Wall,
    /// 自由。
    Free,
    /// 其他（U）。
    Other(u8),
}

impl AlignKind {
    /// `position_offset`（O 0x959f00）：位置偏移（布局 U）。
    pub fn position_offset(&self) -> Vec3 {
        Vec3::ZERO
    }
    /// `outgoing_dir`（O 0x959f50：判别 0/1 否则 panic）。
    pub fn outgoing_dir(&self) -> Vec2 {
        match self {
            AlignKind::Wall => Vec2::new(1.0, 0.0),
            AlignKind::Free => Vec2::new(0.0, 1.0),
            AlignKind::Other(_) => Vec2::ZERO,
        }
    }
}

/// `StairWallAnchor`（O：墙锚点；angle @+0x8）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StairWallAnchor {
    pub anchor: Vec2,
    /// 墙角（弧度，O：`wall_normal` 读 +8）。
    pub angle: f32,
}

impl StairWallAnchor {
    /// `new`（O 0x959e80）。
    pub fn new(anchor: Vec2, angle: f32) -> Self {
        Self { anchor, angle }
    }
    /// `wall_normal`（O 0x959e90）：`(sin(angle), cos(angle))`。
    pub fn wall_normal(&self) -> Vec2 {
        Vec2::new(self.angle.sin(), self.angle.cos())
    }
}

/// `StairNodeAnchor`（O：节点锚点）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StairNodeAnchor {
    Wall(StairWallAnchor),
    Free,
}

impl StairNodeAnchor {
    /// `try_as_wall`（O 0x959ef0）。
    pub fn try_as_wall(&self) -> Option<&StairWallAnchor> {
        match self {
            StairNodeAnchor::Wall(w) => Some(w),
            StairNodeAnchor::Free => None,
        }
    }
}

/// `PlayermadeStairsAssemblyParams`（O：一段手工楼梯装配参数）。
#[derive(Clone, Debug, Default)]
pub struct PlayermadeStairsAssemblyParams {
    /// 墙空间点列（O：+0x8 ptr / +0x10 len，8B 步长 Vec2）。
    pub points: Vec<Vec2>,
    /// 高列表（O：+0x40/+0x48，4B 步长 f32）。
    pub heights: Vec<f32>,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl PlayermadeStairsAssemblyParams {
    /// `length_ws`（O 0x959ca0）：点/高 zip → Vec3 链求墙空间总长。
    pub fn length_ws(&self) -> f32 {
        let n = self.points.len().min(self.heights.len());
        let mut len = 0.0;
        let mut prev: Option<Vec3> = None;
        for i in 0..n {
            let p = Vec3::new(self.points[i].x, self.heights[i], self.points[i].y);
            if let Some(prev) = prev {
                len += (p - prev).length();
            }
            prev = Some(p);
        }
        len
    }
}

/// `PlayermadeStairsAssemblyRequests`（O：装配请求列表）。
#[derive(Clone, Debug, Default)]
pub struct PlayermadeStairsAssemblyRequests {
    pub params: Vec<PlayermadeStairsAssemblyParams>,
}

impl PlayermadeStairsAssemblyRequests {
    /// `prev_requests`（O 0x274480）。
    pub fn prev_requests(&self) -> &[PlayermadeStairsAssemblyParams] {
        &self.params
    }
}

/// `StairGraphEdgeIdentity`（O：32B 边标识；装配/支撑 map 键）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StairGraphEdgeIdentity(pub [u8; 32]);

/// `StairSubSegment`（O：smallvec<[StairSubSegment;1]> 元素；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StairSubSegment(pub Gap);

/// `StairArchWallSupportType`（O：拱墙支撑类型判别；map 值）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StairArchWallSupportType(pub u8);

/// `AdjustedStairEdge`（O：装配后边；IndexMap 值，U）。
#[derive(Clone, Debug, Default)]
pub struct AdjustedStairEdge(pub Gap);

/// `SampledStairsPoints`（O：采样楼梯点；U）。
#[derive(Clone, Debug, Default)]
pub struct SampledStairsPoints(pub Gap);
